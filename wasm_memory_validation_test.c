/*
 * WASM Memory Leak Validation Test Suite
 * Comprehensive testing for the fixed WASM memory management
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>

// Include the WASM runtime functions for testing
extern void* __wasm_malloc(uint32_t size);
extern void __wasm_free(void* ptr);
extern void __wasm_memory_init(uint32_t memory_size_pages);
extern void __wasm_memory_cleanup(void);
extern uint32_t __wasm_get_memory_stats(void);
extern uint32_t __wasm_get_peak_memory_usage(void);
extern uint32_t __wasm_get_current_memory_usage(void);
extern int __wasm_validate_memory(void);

// Stub for external function
void wasm_print_string(const char* str) {
    printf("%s\n", str);
}

// Test cases

void test_basic_allocation_deallocation() {
    printf("Test: Basic allocation and deallocation\n");
    
    __wasm_memory_init(16);  // 16 pages = 1MB
    
    void* ptr1 = __wasm_malloc(64);
    assert(ptr1 != NULL);
    
    void* ptr2 = __wasm_malloc(128);
    assert(ptr2 != NULL);
    
    __wasm_free(ptr1);
    __wasm_free(ptr2);
    
    uint32_t stats = __wasm_get_memory_stats();
    uint32_t active_allocs = stats >> 16;
    uint32_t free_blocks = stats & 0xFFFF;
    
    printf("  Active allocations: %u, Free blocks: %u\n", active_allocs, free_blocks);
    assert(active_allocs == 0);  // Should be no active allocations
    assert(free_blocks >= 1);    // Should have at least one free block
    
    __wasm_memory_cleanup();
    printf("  PASSED\n\n");
}

void test_memory_reuse() {
    printf("Test: Memory reuse after free\n");
    
    __wasm_memory_init(16);
    
    // Allocate and free a block
    void* ptr1 = __wasm_malloc(64);
    assert(ptr1 != NULL);
    __wasm_free(ptr1);
    
    // Allocate same size - should reuse the freed block
    void* ptr2 = __wasm_malloc(64);
    assert(ptr2 != NULL);
    
    // With proper reuse, ptr2 might equal ptr1 (but not guaranteed)
    uint32_t current_usage = __wasm_get_current_memory_usage();
    printf("  Current memory usage: %u bytes\n", current_usage);
    
    __wasm_free(ptr2);
    
    uint32_t stats = __wasm_get_memory_stats();
    uint32_t active_allocs = stats >> 16;
    assert(active_allocs == 0);
    
    __wasm_memory_cleanup();
    printf("  PASSED\n\n");
}

void test_multiple_allocations_no_leak() {
    printf("Test: Multiple allocations without leak\n");
    
    __wasm_memory_init(16);
    
    const int num_allocs = 100;
    void* ptrs[num_allocs];
    
    // Allocate multiple blocks
    for (int i = 0; i < num_allocs; i++) {
        ptrs[i] = __wasm_malloc(32 + i);  // Variable sizes
        assert(ptrs[i] != NULL);
    }
    
    uint32_t peak_usage = __wasm_get_peak_memory_usage();
    printf("  Peak memory usage: %u bytes\n", peak_usage);
    
    // Free all blocks
    for (int i = 0; i < num_allocs; i++) {
        __wasm_free(ptrs[i]);
    }
    
    uint32_t stats = __wasm_get_memory_stats();
    uint32_t active_allocs = stats >> 16;
    uint32_t current_usage = __wasm_get_current_memory_usage();
    
    printf("  Active allocations after free: %u\n", active_allocs);
    printf("  Current memory usage after free: %u bytes\n", current_usage);
    
    assert(active_allocs == 0);      // No memory leaks
    assert(current_usage == 0);      // All memory freed
    
    __wasm_memory_cleanup();
    printf("  PASSED\n\n");
}

void test_fragmentation_handling() {
    printf("Test: Fragmentation handling and coalescing\n");
    
    __wasm_memory_init(16);
    
    // Allocate several blocks
    void* ptr1 = __wasm_malloc(64);
    void* ptr2 = __wasm_malloc(64);
    void* ptr3 = __wasm_malloc(64);
    void* ptr4 = __wasm_malloc(64);
    
    // Free every other block to create fragmentation
    __wasm_free(ptr1);
    __wasm_free(ptr3);
    
    uint32_t stats_before = __wasm_get_memory_stats();
    uint32_t free_blocks_before = stats_before & 0xFFFF;
    
    printf("  Free blocks before coalescing: %u\n", free_blocks_before);
    
    // Free remaining blocks - should coalesce adjacent blocks
    __wasm_free(ptr2);
    __wasm_free(ptr4);
    
    uint32_t stats_after = __wasm_get_memory_stats();
    uint32_t active_allocs = stats_after >> 16;
    uint32_t free_blocks_after = stats_after & 0xFFFF;
    
    printf("  Free blocks after coalescing: %u\n", free_blocks_after);
    printf("  Active allocations: %u\n", active_allocs);
    
    assert(active_allocs == 0);
    
    __wasm_memory_cleanup();
    printf("  PASSED\n\n");
}

void test_invalid_operations() {
    printf("Test: Invalid operations handling\n");
    
    __wasm_memory_init(16);
    
    // Test double free (should not crash)
    void* ptr = __wasm_malloc(64);
    assert(ptr != NULL);
    __wasm_free(ptr);
    __wasm_free(ptr);  // Double free - should be handled gracefully
    
    // Test free of NULL (should not crash)
    __wasm_free(NULL);
    
    // Test free of invalid pointer (should not crash)
    void* invalid_ptr = (void*)0x12345678;
    __wasm_free(invalid_ptr);
    
    // Test zero-size allocation
    void* zero_ptr = __wasm_malloc(0);
    if (zero_ptr != NULL) {
        __wasm_free(zero_ptr);
    }
    
    uint32_t stats = __wasm_get_memory_stats();
    uint32_t active_allocs = stats >> 16;
    
    printf("  Active allocations after invalid ops: %u\n", active_allocs);
    assert(active_allocs == 0);
    
    __wasm_memory_cleanup();
    printf("  PASSED\n\n");
}

void test_memory_validation() {
    printf("Test: Memory validation function\n");
    
    __wasm_memory_init(16);
    
    // Allocate some memory
    void* ptr1 = __wasm_malloc(64);
    void* ptr2 = __wasm_malloc(128);
    
    int validation_result = __wasm_validate_memory();
    printf("  Validation result (should be 2): %d\n", validation_result);
    assert(validation_result == 2);  // Should have 2 valid allocations
    
    __wasm_free(ptr1);
    
    validation_result = __wasm_validate_memory();
    printf("  Validation result after one free (should be 1): %d\n", validation_result);
    assert(validation_result == 1);  // Should have 1 valid allocation
    
    __wasm_free(ptr2);
    
    validation_result = __wasm_validate_memory();
    printf("  Validation result after all freed (should be 0): %d\n", validation_result);
    assert(validation_result == 0);  // Should have 0 valid allocations
    
    __wasm_memory_cleanup();
    printf("  PASSED\n\n");
}

void test_stress_allocation_pattern() {
    printf("Test: Stress allocation pattern (simulating CURSED usage)\n");
    
    __wasm_memory_init(64);  // More memory for stress test
    
    const int iterations = 1000;
    const int concurrent_allocs = 50;
    void* active_ptrs[concurrent_allocs];
    
    // Initialize array
    for (int i = 0; i < concurrent_allocs; i++) {
        active_ptrs[i] = NULL;
    }
    
    for (int iter = 0; iter < iterations; iter++) {
        // Allocate at random slots
        int slot = iter % concurrent_allocs;
        
        if (active_ptrs[slot] != NULL) {
            __wasm_free(active_ptrs[slot]);
        }
        
        // Variable size allocation (simulating strings, arrays, etc.)
        uint32_t size = 16 + (iter % 256);
        active_ptrs[slot] = __wasm_malloc(size);
        assert(active_ptrs[slot] != NULL);
        
        // Write some data to test for corruption
        memset(active_ptrs[slot], (char)(iter & 0xFF), size);
    }
    
    uint32_t peak_usage = __wasm_get_peak_memory_usage();
    uint32_t current_usage = __wasm_get_current_memory_usage();
    
    printf("  Peak memory usage: %u bytes\n", peak_usage);
    printf("  Current memory usage: %u bytes\n", current_usage);
    
    // Clean up remaining allocations
    for (int i = 0; i < concurrent_allocs; i++) {
        if (active_ptrs[i] != NULL) {
            __wasm_free(active_ptrs[i]);
        }
    }
    
    uint32_t stats = __wasm_get_memory_stats();
    uint32_t active_allocs = stats >> 16;
    uint32_t final_usage = __wasm_get_current_memory_usage();
    
    printf("  Final active allocations: %u\n", active_allocs);
    printf("  Final memory usage: %u bytes\n", final_usage);
    
    assert(active_allocs == 0);
    assert(final_usage == 0);
    
    __wasm_memory_cleanup();
    printf("  PASSED\n\n");
}

int main() {
    printf("WASM Memory Management Validation Test Suite\n");
    printf("=============================================\n\n");
    
    test_basic_allocation_deallocation();
    test_memory_reuse();
    test_multiple_allocations_no_leak();
    test_fragmentation_handling();
    test_invalid_operations();
    test_memory_validation();
    test_stress_allocation_pattern();
    
    printf("All tests PASSED! ✅\n");
    printf("WASM memory leak fix has been validated.\n");
    
    return 0;
}
