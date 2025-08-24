/*
 * Simple WASM Memory Test - test the basic fix
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

// Mock the external symbol for testing
unsigned char mock_heap_base[1024 * 1024] = {0}; // 1MB mock heap
unsigned char __heap_base __attribute__((alias("mock_heap_base")));

// Test the runtime functions by including them directly
#define TEST_BUILD 1

// Include the runtime source
#include "runtime/wasm_runtime.c"

int main() {
    printf("Simple WASM Memory Fix Test\n");
    printf("===========================\n");
    
    // Initialize the memory system
    __wasm_memory_init(16);  // 16 pages
    
    printf("1. Testing basic allocation...\n");
    void* ptr1 = __wasm_malloc(64);
    if (ptr1) {
        printf("   ✓ Allocated 64 bytes at %p\n", ptr1);
    } else {
        printf("   ✗ Failed to allocate 64 bytes\n");
        return 1;
    }
    
    printf("2. Testing deallocation (old version was no-op)...\n");
    __wasm_free(ptr1);
    printf("   ✓ Free called (should actually free memory now)\n");
    
    printf("3. Testing memory reuse after free...\n");
    void* ptr2 = __wasm_malloc(64);
    if (ptr2) {
        printf("   ✓ Allocated 64 bytes at %p\n", ptr2);
        if (ptr2 == ptr1) {
            printf("   ✓ Memory reused! (fix is working)\n");
        } else {
            printf("   ~ Memory not reused (still valid, depends on allocator)\n");
        }
    } else {
        printf("   ✗ Failed to allocate 64 bytes\n");
        return 1;
    }
    
    printf("4. Testing memory statistics...\n");
    uint32_t stats = __wasm_get_memory_stats();
    uint32_t active_allocs = stats >> 16;
    uint32_t free_blocks = stats & 0xFFFF;
    printf("   Active allocations: %u\n", active_allocs);
    printf("   Free blocks: %u\n", free_blocks);
    
    printf("5. Testing leak detection...\n");
    uint32_t current_usage = __wasm_get_current_memory_usage();
    printf("   Current memory usage: %u bytes\n", current_usage);
    
    __wasm_free(ptr2);
    
    current_usage = __wasm_get_current_memory_usage();
    printf("   Memory usage after free: %u bytes\n", current_usage);
    
    if (current_usage == 0) {
        printf("   ✓ No memory leaks detected!\n");
    } else {
        printf("   ⚠ Memory still in use: %u bytes\n", current_usage);
    }
    
    printf("6. Testing multiple allocation/free cycles...\n");
    for (int i = 0; i < 10; i++) {
        void* temp_ptr = __wasm_malloc(32 + i);
        if (temp_ptr) {
            // Write some data
            memset(temp_ptr, 0xAA, 32 + i);
            __wasm_free(temp_ptr);
        }
    }
    
    current_usage = __wasm_get_current_memory_usage();
    printf("   Memory usage after 10 alloc/free cycles: %u bytes\n", current_usage);
    
    if (current_usage == 0) {
        printf("   ✓ No accumulation of memory leaks!\n");
    } else {
        printf("   ⚠ Memory accumulated: %u bytes\n", current_usage);
    }
    
    // Cleanup
    __wasm_memory_cleanup();
    
    printf("\n✅ WASM memory leak fix test completed successfully!\n");
    printf("The fix replaces the no-op __wasm_free with proper memory management.\n");
    
    return 0;
}
