#include <stdlib.h>

struct BrocStr {
    char* bytes;
    size_t len;
    size_t capacity;
};

extern void broc__mainForHost_1_exposed_generic(const struct BrocStr *data);
