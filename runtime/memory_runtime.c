/*
 * Memory Runtime Bridge for CURSED
 * 
 * This C library provides the bridge between CURSED stdlib memory functions
 * and the Rust runtime memory management system with GC integration.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <assert.h>

// Forward declarations for Rust FFI functions
extern void* rust_heap_allocate(size_t size, int32_t tag);
extern void rust_heap_deallocate(void* ptr);
extern void* rust_heap_reallocate(void* ptr, size_t new_size);
extern int32_t rust_gc_collect(void);
extern char* rust_gc_stats(void);
extern char* rust_memory_stats(void);
extern bool rust_track_allocation(void* ptr, size_t size, const char* tag);
extern double rust_memory_pressure(void);
extern int32_t rust_stack_size(void);
extern bool rust_check_stack_overflow(void);
extern void* rust_create_memory_pool(int32_t block_size, int32_t block_count);
extern void* rust_pool_alloc(void* pool_id, int32_t size);
extern bool rust_pool_free(void* pool_id, void* ptr);
extern bool rust_zero_memory(void* ptr, int32_t size);
extern bool rust_copy_memory(void* dest, void* src, int32_t size);
extern int32_t rust_compare_memory(void* ptr1, void* ptr2, int32_t size);
extern int32_t rust_align_size(int32_t size, int32_t alignment);
extern bool rust_is_aligned(void* ptr, int32_t alignment);
extern bool rust_set_memory_limit(size_t limit);
extern size_t rust_get_memory_usage(void);
extern int32_t rust_memory_compact(void);
extern bool rust_reset_memory_stats(void);

// Memory allocation tags
#define OBJECT_TAG 1
#define ARRAY_TAG 2
#define STRING_TAG 3
#define FUNCTION_TAG 4
#define CHANNEL_TAG 5
#define GOROUTINE_TAG 6

/**
 * Allocate memory with GC tracking
 */
void* cursed_runtime_malloc(int32_t size, int32_t tag) {
    if (size <= 0) {
        return NULL;
    }
    
    void* ptr = rust_heap_allocate((size_t)size, tag);
    if (ptr) {
        // Track allocation for debugging
        const char* tag_name = "unknown";
        switch (tag) {
            case OBJECT_TAG: tag_name = "object"; break;
            case ARRAY_TAG: tag_name = "array"; break;
            case STRING_TAG: tag_name = "string"; break;
            case FUNCTION_TAG: tag_name = "function"; break;
            case CHANNEL_TAG: tag_name = "channel"; break;
            case GOROUTINE_TAG: tag_name = "goroutine"; break;
        }
        rust_track_allocation(ptr, (size_t)size, tag_name);
    }
    
    return ptr;
}

/**
 * Free memory through GC system
 */
bool cursed_runtime_free(void* ptr) {
    if (!ptr) {
        return true;
    }
    
    rust_heap_deallocate(ptr);
    return true;
}

/**
 * Reallocate memory with GC tracking
 */
void* cursed_runtime_realloc(void* ptr, int32_t new_size) {
    if (new_size <= 0) {
        if (ptr) {
            cursed_runtime_free(ptr);
        }
        return NULL;
    }
    
    if (!ptr) {
        return cursed_runtime_malloc(new_size, OBJECT_TAG);
    }
    
    void* new_ptr = rust_heap_reallocate(ptr, (size_t)new_size);
    if (new_ptr) {
        rust_track_allocation(new_ptr, (size_t)new_size, "realloc");
    }
    
    return new_ptr;
}

/**
 * Trigger garbage collection
 */
int32_t cursed_runtime_gc_collect(void) {
    return rust_gc_collect();
}

/**
 * Get GC statistics
 */
const char* cursed_runtime_gc_stats(void) {
    char* stats = rust_gc_stats();
    return stats ? stats : "GC stats unavailable";
}

/**
 * Get memory statistics
 */
const char* cursed_runtime_memory_stats(void) {
    char* stats = rust_memory_stats();
    return stats ? stats : "Memory stats unavailable";
}

/**
 * Track allocation for debugging
 */
bool cursed_runtime_track_allocation(void* ptr, int32_t size, const char* tag) {
    if (!ptr || size <= 0 || !tag) {
        return false;
    }
    
    return rust_track_allocation(ptr, (size_t)size, tag);
}

/**
 * Get memory pressure (0.0-1.0)
 */
double cursed_runtime_memory_pressure(void) {
    return rust_memory_pressure();
}

/**
 * Get current stack size
 */
int32_t cursed_runtime_stack_size(void) {
    return rust_stack_size();
}

/**
 * Check for stack overflow
 */
bool cursed_runtime_check_stack_overflow(void) {
    return rust_check_stack_overflow();
}

/**
 * Create memory pool
 */
void* cursed_runtime_create_memory_pool(int32_t block_size, int32_t block_count) {
    if (block_size <= 0 || block_count <= 0) {
        return NULL;
    }
    
    return rust_create_memory_pool(block_size, block_count);
}

/**
 * Allocate from memory pool
 */
void* cursed_runtime_pool_alloc(void* pool_id, int32_t size) {
    if (!pool_id || size <= 0) {
        return NULL;
    }
    
    return rust_pool_alloc(pool_id, size);
}

/**
 * Free to memory pool
 */
bool cursed_runtime_pool_free(void* pool_id, void* ptr) {
    if (!pool_id || !ptr) {
        return false;
    }
    
    return rust_pool_free(pool_id, ptr);
}

/**
 * Zero memory
 */
bool cursed_runtime_zero_memory(void* ptr, int32_t size) {
    if (!ptr || size <= 0) {
        return false;
    }
    
    return rust_zero_memory(ptr, size);
}

/**
 * Copy memory
 */
bool cursed_runtime_copy_memory(void* dest, void* src, int32_t size) {
    if (!dest || !src || size <= 0) {
        return false;
    }
    
    return rust_copy_memory(dest, src, size);
}

/**
 * Compare memory
 */
int32_t cursed_runtime_compare_memory(void* ptr1, void* ptr2, int32_t size) {
    if (!ptr1 || !ptr2 || size <= 0) {
        return -1; // Error
    }
    
    return rust_compare_memory(ptr1, ptr2, size);
}

/**
 * Align size to boundary
 */
int32_t cursed_runtime_align_size(int32_t size, int32_t alignment) {
    if (size <= 0 || alignment <= 0) {
        return size;
    }
    
    return rust_align_size(size, alignment);
}

/**
 * Check if pointer is aligned
 */
bool cursed_runtime_is_aligned(void* ptr, int32_t alignment) {
    if (!ptr || alignment <= 0) {
        return false;
    }
    
    return rust_is_aligned(ptr, alignment);
}

/**
 * Set memory limit
 */
bool cursed_runtime_set_memory_limit(size_t limit) {
    return rust_set_memory_limit(limit);
}

/**
 * Get current memory usage
 */
size_t cursed_runtime_get_memory_usage(void) {
    return rust_get_memory_usage();
}

/**
 * Compact memory
 */
int32_t cursed_runtime_memory_compact(void) {
    return rust_memory_compact();
}

/**
 * Reset memory statistics
 */
bool cursed_runtime_reset_memory_stats(void) {
    return rust_reset_memory_stats();
}

// Memory validation and debugging functions

/**
 * Validate memory integrity
 */
bool cursed_validate_memory_integrity(void* ptr, size_t size) {
    if (!ptr || size == 0) {
        return false;
    }
    
    // Basic validation - check if we can read/write the memory
    // In a real implementation, this would check magic numbers, checksums, etc.
    volatile char* test_ptr = (volatile char*)ptr;
    char original = *test_ptr;
    *test_ptr = original; // Write back the same value
    
    return *test_ptr == original;
}

/**
 * Debug memory allocation
 */
void cursed_debug_allocation(void* ptr, size_t size, const char* location) {
    if (!ptr) {
        fprintf(stderr, "DEBUG: Failed allocation of %zu bytes at %s\n", size, location ? location : "unknown");
        return;
    }
    
    fprintf(stderr, "DEBUG: Allocated %zu bytes at %p from %s\n", size, ptr, location ? location : "unknown");
}

/**
 * Debug memory deallocation
 */
void cursed_debug_deallocation(void* ptr, const char* location) {
    if (!ptr) {
        fprintf(stderr, "DEBUG: Attempted to free NULL pointer at %s\n", location ? location : "unknown");
        return;
    }
    
    fprintf(stderr, "DEBUG: Freed pointer %p at %s\n", ptr, location ? location : "unknown");
}

/**
 * Memory fence for synchronization
 */
void cursed_memory_fence(void) {
    __sync_synchronize();
}

/**
 * Get memory alignment for platform
 */
size_t cursed_get_memory_alignment(void) {
    return sizeof(void*); // Pointer alignment is usually sufficient
}
