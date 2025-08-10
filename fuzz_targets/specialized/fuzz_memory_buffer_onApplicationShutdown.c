// CURSED Memory Buffer Fuzz Target Template
// Targets: onApplicationShutdown in src-zig/windows_async_integration.zig:347

const std = @import("std");
const testing = std.testing;
const c = @cImport({
    @cInclude("stdint.h");
    @cInclude("stdlib.h");
    @cInclude("string.h");
});

#define MAX_BUFFER_SIZE (1024 * 1024)

// Zig-specific setup
extern fn malloc(size: usize) ?*anyopaque;
extern fn free(ptr: ?*anyopaque) void;

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size == 0 || size > MAX_BUFFER_SIZE) return 0;
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    
    // Test buffer operations with various sizes
    test_buffer_operations(data, size);
    test_string_operations(data, size);
    test_boundary_conditions(data, size);
    
    // Zig GPA cleanup handled by defer
    return 0;
}

void test_buffer_operations(const uint8_t *data, size_t size) {
    // Test buffer copying and manipulation
    uint8_t *buffer = malloc(size + 1);
    if (!buffer) return;
    
    // onApplicationShutdown(buffer, data, size);
    
    free(buffer);
}

void test_string_operations(const uint8_t *data, size_t size) {
    char *str = malloc(size + 1);
    if (!str) return;
    
    memcpy(str, data, size);
    str[size] = '\0';
    
    // Test string functions
    // onApplicationShutdown(str);
    
    free(str);
}

void test_boundary_conditions(const uint8_t *data, size_t size) {
    // Test edge cases
    if (size > 0) {
        // onApplicationShutdown(data, 0);        // Zero size
        // onApplicationShutdown(data, 1);        // Single byte
        // onApplicationShutdown(data, size);     // Full size
        // onApplicationShutdown(NULL, 0);        // NULL pointer
    }
}


// Additional test functions for onApplicationShutdown
void test_edge_cases(const uint8_t *data, size_t size) {
    // Test with edge cases specific to memory_buffer
}

void test_error_conditions(const uint8_t *data, size_t size) {
    // Test error handling paths
}

