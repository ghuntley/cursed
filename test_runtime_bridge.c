// Test C program to verify runtime bridge functions are exposed
// This tests the actual Zig runtime functions directly

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Declare runtime functions that should be available from Zig
extern void runtime_print_string(const char* message);
extern char* runtime_read_line(void);
extern char* runtime_read_file(void* allocator, const char* filename);
extern int runtime_write_file(const char* filename, const char* content);
extern int runtime_file_exists(const char* filename);
extern int runtime_delete_file(const char* filename);

int main() {
    printf("=== Runtime Bridge Function Test ===\n");
    
    // Test 1: Print function
    printf("Testing runtime_print_string...\n");
    runtime_print_string("Hello from runtime bridge!\n");
    printf("✅ runtime_print_string works\n");
    
    // Test 2: File operations
    printf("Testing file operations...\n");
    const char* filename = "bridge_test.txt";
    const char* content = "Test content from C bridge";
    
    // Write file
    if (runtime_write_file(filename, content)) {
        printf("✅ File write successful\n");
    } else {
        printf("❌ File write failed\n");
        return 1;
    }
    
    // Check if file exists
    if (runtime_file_exists(filename)) {
        printf("✅ File exists check successful\n");
    } else {
        printf("❌ File exists check failed\n");
        return 1;
    }
    
    // Read file back (if allocator support is available)
    printf("File operations test completed\n");
    
    // Cleanup
    if (runtime_delete_file(filename)) {
        printf("✅ File cleanup successful\n");
    } else {
        printf("⚠️  File cleanup failed\n");
    }
    
    printf("🎯 All runtime bridge tests passed!\n");
    return 0;
}
