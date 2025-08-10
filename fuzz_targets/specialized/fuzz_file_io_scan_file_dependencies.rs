// CURSED File I/O Fuzz Target Template
// Targets: scan_file_dependencies in src/documentation/build_integration.rs:427

#include <stdint.h>\n#include <stddef.h>

#define MAX_PATH_SIZE 4096
#define MAX_FILE_SIZE (1024 * 1024)  // 1MB limit

// C-specific setup

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size == 0 || size > MAX_FILE_SIZE) return 0;
    
    // C uses malloc/free directly
    
    // Create temporary file with fuzzed content
    char temp_path[] = "/tmp/fuzz_file_XXXXXX";
    int fd = mkstemp(temp_path);
    if (fd == -1) return 0;
    
    // Write fuzzed data to temp file
    if (write(fd, data, size) != (ssize_t)size) {
        close(fd);
        unlink(temp_path);
        return 0;
    }
    close(fd);
    
    // Test file operations
    // Test scan_file_dependencies with temp file
    // Example: scan_file_dependencies(temp_path);
    // Example: scan_file_dependencies(temp_path, data, size);
    
    // Cleanup
    unlink(temp_path);
    // C cleanup handled manually
    
    return 0;
}

// Test with various path manipulations
void test_path_operations(const uint8_t *data, size_t size) {
    if (size == 0 || size > MAX_PATH_SIZE) return;
    
    char path[MAX_PATH_SIZE];
    size_t copy_size = size < MAX_PATH_SIZE - 1 ? size : MAX_PATH_SIZE - 1;
    memcpy(path, data, copy_size);
    path[copy_size] = '\0';
    
    // Test path validation and manipulation
    // scan_file_dependencies(path);
}


// Additional test functions for scan_file_dependencies
void test_edge_cases(const uint8_t *data, size_t size) {
    // Test with edge cases specific to file_io
}

void test_error_conditions(const uint8_t *data, size_t size) {
    // Test error handling paths
}

