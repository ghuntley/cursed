// Fuzz target for cursed_is_string_type in runtime/type_checking.c:133
// Risk Level: HIGH
// Input Types: memory_buffer

#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdlib.h>

// Include the header containing cursed_is_string_type
// #include "runtime/type_checking.h"

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size == 0) return 0;
    
    // Null-terminate data for string functions
    char *input = malloc(size + 1);
    if (!input) return 0;
    memcpy(input, data, size);
    input[size] = '\0';
    
    // TODO: Call cursed_is_string_type with fuzzed input
    // Example: cursed_is_string_type(input);
    
    free(input);
    return 0;
}
