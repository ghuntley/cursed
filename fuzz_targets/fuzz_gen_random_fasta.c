// Fuzz target for gen_random_fasta in benchmarks/c/fasta.c:52
// Risk Level: HIGH
// Input Types: parsing

#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <stdlib.h>

// Include the header containing gen_random_fasta
// #include "benchmarks/c/fasta.h"

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size == 0) return 0;
    
    // Null-terminate data for string functions
    char *input = malloc(size + 1);
    if (!input) return 0;
    memcpy(input, data, size);
    input[size] = '\0';
    
    // TODO: Call gen_random_fasta with fuzzed input
    // Example: gen_random_fasta(input);
    
    free(input);
    return 0;
}
