// Fuzz target for print in runtime/interface_runtime.c:336
// Risk Level: HIGH
// Input Types: memory_buffer

#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdlib.h>

// Include the header containing print
// #include "runtime/interface_runtime.h"

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size == 0) return 0;
    
    // Null-terminate data for string functions
    char *input = malloc(size + 1);
    if (!input) return 0;
    memcpy(input, data, size);
    input[size] = '\0';
    
    // TODO: Call print with fuzzed input
    // Example: print(input);
    
    free(input);
    return 0;
}
