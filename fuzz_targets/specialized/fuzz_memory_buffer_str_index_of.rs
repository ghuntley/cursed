// CURSED Memory Buffer Fuzz Target Template
// Targets: str_index_of in stdlib/networkz/simple_mod.csd:278

#include <stdint.h>\n#include <stddef.h>

#define MAX_BUFFER_SIZE (1024 * 1024)

// C-specific setup

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size == 0 || size > MAX_BUFFER_SIZE) return 0;
    
    // C uses malloc/free directly
    
    // Test buffer operations with various sizes
    test_buffer_operations(data, size);
    test_string_operations(data, size);
    test_boundary_conditions(data, size);
    
    // C cleanup handled manually
    return 0;
}

void test_buffer_operations(const uint8_t *data, size_t size) {
    // Test buffer copying and manipulation
    uint8_t *buffer = malloc(size + 1);
    if (!buffer) return;
    
    // str_index_of(buffer, data, size);
    
    free(buffer);
}

void test_string_operations(const uint8_t *data, size_t size) {
    char *str = malloc(size + 1);
    if (!str) return;
    
    memcpy(str, data, size);
    str[size] = '\0';
    
    // Test string functions
    // str_index_of(str);
    
    free(str);
}

void test_boundary_conditions(const uint8_t *data, size_t size) {
    // Test edge cases
    if (size > 0) {
        // str_index_of(data, 0);        // Zero size
        // str_index_of(data, 1);        // Single byte
        // str_index_of(data, size);     // Full size
        // str_index_of(NULL, 0);        // NULL pointer
    }
}


// Additional test functions for str_index_of
void test_edge_cases(const uint8_t *data, size_t size) {
    // Test with edge cases specific to memory_buffer
}

void test_error_conditions(const uint8_t *data, size_t size) {
    // Test error handling paths
}

