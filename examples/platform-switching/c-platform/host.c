#include <errno.h>
#include <signal.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

#ifdef _WIN32
#else
#include <sys/shm.h> // shm_open
#include <sys/mman.h> // for mmap
#include <signal.h> // for kill
#endif

void* broc_alloc(size_t size, unsigned int alignment) { return malloc(size); }

void* broc_realloc(void* ptr, size_t new_size, size_t old_size, unsigned int alignment) {
  return realloc(ptr, new_size);
}

void broc_dealloc(void* ptr, unsigned int alignment) { free(ptr); }

void broc_panic(void* ptr, unsigned int alignment) {
  char* msg = (char*)ptr;
  fprintf(stderr,
          "Application crashed with message\n\n    %s\n\nShutting down\n", msg);
  exit(0);
}

void* broc_memcpy(void* dest, const void* src, size_t n) {
  return memcpy(dest, src, n);
}

void* broc_memset(void* str, int c, size_t n) { return memset(str, c, n); }

int broc_shm_open(char* name, int oflag, int mode) {
#ifdef _WIN32
    return 0;
#else
    return shm_open(name, oflag, mode);
#endif
}
void* broc_mmap(void* addr, int length, int prot, int flags, int fd, int offset) {
#ifdef _WIN32
    return addr;
#else
    return mmap(addr, length, prot, flags, fd, offset);
#endif
}

int broc_getppid() {
#ifdef _WIN32
    return 0;
#else
    return getppid();
#endif
}

struct BrocStr {
  char* bytes;
  size_t len;
  size_t capacity;
};

bool is_small_str(struct BrocStr str) { return ((ssize_t)str.capacity) < 0; }

// Determine the length of the string, taking into
// account the small string optimization
size_t broc_str_len(struct BrocStr str) {
  char* bytes = (char*)&str;
  char last_byte = bytes[sizeof(str) - 1];
  char last_byte_xored = last_byte ^ 0b10000000;
  size_t small_len = (size_t)(last_byte_xored);
  size_t big_len = str.len;

  // Avoid branch misprediction costs by always
  // determining both small_len and big_len,
  // so this compiles to a cmov instruction.
  if (is_small_str(str)) {
    return small_len;
  } else {
    return big_len;
  }
}

extern void broc__mainForHost_1_exposed_generic(struct BrocStr *string);

int main() {

  struct BrocStr str;
  broc__mainForHost_1_exposed_generic(&str);

  // Determine str_len and the str_bytes pointer,
  // taking into account the small string optimization.
  size_t str_len = broc_str_len(str);
  char* str_bytes;

  if (is_small_str(str)) {
    str_bytes = (char*)&str;
  } else {
    str_bytes = str.bytes;
  }

  // Write to stdout
  if (write(1, str_bytes, str_len) >= 0) {
    // Writing succeeded!

    // NOTE: the string is a static string, read from in the binary
    // if you make it a heap-allocated string, it'll be leaked here
    return 0;
  } else {
    printf("Error writing to stdout: %s\n", strerror(errno));

    // NOTE: the string is a static string, read from in the binary
    // if you make it a heap-allocated string, it'll be leaked here
    return 1;
  }
}
