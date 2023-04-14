#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <string.h>
#include <unistd.h>
#include <ruby.h>
#include "extconf.h"

void *broc_alloc(size_t size, unsigned int alignment) { return malloc(size); }

void *broc_realloc(void *ptr, size_t new_size, size_t old_size,
                  unsigned int alignment)
{
    return realloc(ptr, new_size);
}

void broc_dealloc(void *ptr, unsigned int alignment) { free(ptr); }

__attribute__((noreturn)) void broc_panic(void *ptr, unsigned int alignment)
{
    rb_raise(rb_eException, "%s", (char *)ptr);
}

void *broc_memcpy(void *dest, const void *src, size_t n)
{
    return memcpy(dest, src, n);
}

void *broc_memset(void *str, int c, size_t n) { return memset(str, c, n); }

// Reference counting

// If the refcount is set to this, that means the allocation is
// stored in readonly memory in the binary, and we must not
// attempt to increment or decrement it; if we do, we'll segfault!
const ssize_t REFCOUNT_READONLY = 0;
const ssize_t REFCOUNT_ONE = (ssize_t)PTRDIFF_MIN;
const size_t MASK = (size_t)PTRDIFF_MIN;

// Increment reference count, given a pointer to the first element in a collection.
// We don't need to check for overflow because in order to overflow a usize worth of refcounts,
// you'd need to somehow have more pointers in memory than the OS's virtual address space can hold.
void incref(uint8_t* bytes, uint32_t alignment)
{
    ssize_t *refcount_ptr = ((ssize_t *)bytes) - 1;
    ssize_t refcount = *refcount_ptr;

    if (refcount != REFCOUNT_READONLY) {
        *refcount_ptr = refcount + 1;
    }
}

// Decrement reference count, given a pointer to the first element in a collection.
// Then call broc_dealloc if nothing is referencing this collection anymore.
void decref(uint8_t* bytes, uint32_t alignment)
{
    if (bytes == NULL) {
        return;
    }

    size_t extra_bytes = (sizeof(size_t) >= (size_t)alignment) ? sizeof(size_t) : (size_t)alignment;
    ssize_t *refcount_ptr = ((ssize_t *)bytes) - 1;
    ssize_t refcount = *refcount_ptr;

    if (refcount != REFCOUNT_READONLY) {
        *refcount_ptr = refcount - 1;

        if (refcount == REFCOUNT_ONE) {
            void *original_allocation = (void *)(refcount_ptr - (extra_bytes - sizeof(size_t)));

            broc_dealloc(original_allocation, alignment);
        }
    }
}

// BrocBytes (List U8)

struct BrocBytes
{
    uint8_t *bytes;
    size_t len;
    size_t capacity;
};

struct BrocBytes init_brocbytes(uint8_t *bytes, size_t len)
{
    if (len == 0)
    {
        struct BrocBytes ret = {
            .len = 0,
            .bytes = NULL,
            .capacity = 0,
        };

        return ret;
    }
    else
    {
        struct BrocBytes ret;
        size_t refcount_size = sizeof(size_t);
        uint8_t *new_content = (uint8_t *)broc_alloc(len + refcount_size, alignof(size_t)) + refcount_size;

        memcpy(new_content, bytes, len);

        ret.bytes = new_content;
        ret.len = len;
        ret.capacity = len;

        return ret;
    }
}

// BrocStr

struct BrocStr
{
    uint8_t *bytes;
    size_t len;
    size_t capacity;
};

struct BrocStr init_brocstr(uint8_t *bytes, size_t len)
{
    if (len == 0)
    {
        struct BrocStr ret = {
            .len = 0,
            .bytes = NULL,
            .capacity = MASK,
        };

        return ret;
    }
    else if (len < sizeof(struct BrocStr))
    {
        // Start out with zeroed memory, so that
        // if we end up comparing two small BrocStr values
        // for equality, we won't risk memory garbage resulting
        // in two equal strings appearing unequal.
        struct BrocStr ret = {
            .len = 0,
            .bytes = NULL,
            .capacity = MASK,
        };

        // Copy the bytes into the stack allocation
        memcpy(&ret, bytes, len);

        // Record the string's length in the last byte of the stack allocation
        ((uint8_t *)&ret)[sizeof(struct BrocStr) - 1] = (uint8_t)len | 0b10000000;

        return ret;
    }
    else
    {
        // A large BrocStr is the same as a List U8 (aka BrocBytes) in memory.
        struct BrocBytes broc_bytes = init_brocbytes(bytes, len);

        struct BrocStr ret = {
            .len = broc_bytes.len,
            .bytes = broc_bytes.bytes,
            .capacity = broc_bytes.capacity,
        };

        return ret;
    }
}

bool is_small_str(struct BrocStr str) { return ((ssize_t)str.capacity) < 0; }

// Determine the length of the string, taking into
// account the small string optimization
size_t broc_str_len(struct BrocStr str)
{
    uint8_t *bytes = (uint8_t *)&str;
    uint8_t last_byte = bytes[sizeof(str) - 1];
    uint8_t last_byte_xored = last_byte ^ 0b10000000;
    size_t small_len = (size_t)(last_byte_xored);
    size_t big_len = str.len;

    // Avoid branch misprediction costs by always
    // determining both small_len and big_len,
    // so this compiles to a cmov instruction.
    if (is_small_str(str))
    {
        return small_len;
    }
    else
    {
        return big_len;
    }
}

extern void broc__mainForHost_1_exposed_generic(struct BrocBytes *ret, struct BrocBytes *arg);

// Receive a value from Ruby, JSON serialized it and pass it to Broc as a List U8
// (at which point the Broc platform will decode it and crash if it's invalid,
// which broc_panic will translate into a Ruby exception), then get some JSON back from Broc
// - also as a List U8 - and have Ruby JSON.parse it into a plain Ruby value to return.
VALUE call_broc(VALUE self, VALUE rb_arg)
{
    // This must be required before the to_json method will exist on String.
    rb_require("json");

    // Turn the given Ruby value into a JSON string.
    // TODO should we defensively encode it as UTF-8 first?
    VALUE json_arg = rb_funcall(rb_arg, rb_intern("to_json"), 0);

    struct BrocBytes arg = init_brocbytes((uint8_t *)RSTRING_PTR(json_arg), RSTRING_LEN(json_arg));
    struct BrocBytes ret;

    // Call the Broc function to populate `ret`'s bytes.
    broc__mainForHost_1_exposed_generic(&ret, &arg);

    // Create a rb_utf8_str from the heap-allocated JSON bytes the Broc function returned.
    VALUE returned_json = rb_utf8_str_new((char *)ret.bytes, ret.len);

    // Now that we've created our Ruby JSON string, we're no longer referencing the BrocBytes.
    decref((void *)&ret, alignof(uint8_t *));

    return rb_funcall(rb_define_module("JSON"), rb_intern("parse"), 1, returned_json);
}

void Init_demo()
{
    VALUE broc_app = rb_define_module("BrocApp");
    rb_define_module_function(broc_app, "call_broc", &call_broc, 1);
}
