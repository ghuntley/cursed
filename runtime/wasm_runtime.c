/**
 * CURSED WebAssembly Runtime - FIXED VERSION
 * Provides proper memory management for CURSED programs compiled to WebAssembly
 * 
 * CRITICAL FIXES:
 * 1. Proper memory allocation and deallocation tracking
 * 2. Free list implementation for memory reuse
 * 3. Memory block header system for size tracking
 * 4. Proper cleanup on module unloading
 * 5. Memory leak detection and prevention
 */

#include <stdint.h>
#include <stdlib.h>
#include <string.h>

// Memory management constants
#define WASM_PAGE_SIZE 65536
#define ALLOCATION_HEADER_SIZE 8
#define MIN_ALLOCATION_SIZE 16
#define MAX_ALLOCATION_SIZE (1024 * 1024)  // 1MB max allocation

// WebAssembly memory layout
extern unsigned char __heap_base;
static uint32_t heap_start = 0;  // Will be initialized
static uint32_t heap_end = 0;    // Will be set by memory initialization
static uint32_t heap_current = 0;

// Memory block header for tracking allocations
typedef struct allocation_header {
    uint32_t size;           // Size of the allocation (excluding header)
    uint32_t magic;          // Magic number for corruption detection
    uint32_t is_free;        // 0 = allocated, 1 = free
    uint32_t next_free;      // Pointer to next free block (if free)
} allocation_header_t;

#define ALLOCATION_MAGIC 0xDEADBEEF

// Free list head - circular linked list of free blocks
static uint32_t free_list_head = 0;

// Statistics for memory leak detection
static uint32_t total_allocations = 0;
static uint32_t total_deallocations = 0;
static uint32_t bytes_allocated = 0;
static uint32_t bytes_freed = 0;
static uint32_t peak_memory_usage = 0;

// Memory initialization
void __wasm_memory_init(uint32_t memory_size_pages) {
    heap_start = (uintptr_t)&__heap_base;
    heap_end = heap_start + (memory_size_pages * WASM_PAGE_SIZE);
    heap_current = heap_start;
    free_list_head = 0;
    
    // Initialize statistics
    total_allocations = 0;
    total_deallocations = 0;
    bytes_allocated = 0;
    bytes_freed = 0;
    peak_memory_usage = 0;
}

// Memory cleanup - called when module is unloaded
void __wasm_memory_cleanup(void) {
    // Report memory leaks if any
    if (total_allocations != total_deallocations) {
        wasm_print_string("MEMORY LEAK DETECTED!");
        // In a real implementation, we'd report details here
    }
    
    // Reset all tracking
    free_list_head = 0;
    heap_current = heap_start;
    total_allocations = 0;
    total_deallocations = 0;
    bytes_allocated = 0;
    bytes_freed = 0;
    peak_memory_usage = 0;
}

// Helper: Find a free block of at least the requested size
static allocation_header_t* find_free_block(uint32_t size) {
    if (free_list_head == 0) {
        return NULL;  // No free blocks
    }
    
    uint32_t current = free_list_head;
    do {
        allocation_header_t* header = (allocation_header_t*)current;
        
        // Validate header
        if (header->magic != ALLOCATION_MAGIC || !header->is_free) {
            return NULL;  // Corruption detected
        }
        
        if (header->size >= size) {
            return header;  // Found suitable block
        }
        
        current = header->next_free;
    } while (current != free_list_head && current != 0);
    
    return NULL;  // No suitable free block found
}

// Helper: Remove block from free list
static void remove_from_free_list(allocation_header_t* block) {
    if (free_list_head == 0) return;
    
    uint32_t block_addr = (uint32_t)block;
    
    // If this is the only block in the free list
    if (block->next_free == block_addr) {
        free_list_head = 0;
        return;
    }
    
    // Find the block that points to this one
    uint32_t current = free_list_head;
    do {
        allocation_header_t* current_block = (allocation_header_t*)current;
        if (current_block->next_free == block_addr) {
            current_block->next_free = block->next_free;
            
            // Update head if necessary
            if (free_list_head == block_addr) {
                free_list_head = block->next_free;
            }
            break;
        }
        current = current_block->next_free;
    } while (current != free_list_head && current != 0);
}

// Helper: Add block to free list
static void add_to_free_list(allocation_header_t* block) {
    uint32_t block_addr = (uint32_t)block;
    
    block->is_free = 1;
    
    if (free_list_head == 0) {
        // First free block
        block->next_free = block_addr;  // Point to self (circular list)
        free_list_head = block_addr;
    } else {
        // Insert into circular list
        allocation_header_t* head = (allocation_header_t*)free_list_head;
        block->next_free = head->next_free;
        head->next_free = block_addr;
    }
}

// Helper: Coalesce adjacent free blocks to reduce fragmentation
static void coalesce_free_blocks(allocation_header_t* block) {
    uint32_t block_addr = (uint32_t)block;
    uint32_t block_end = block_addr + ALLOCATION_HEADER_SIZE + block->size;
    
    // Check if next block in memory is also free and adjacent
    if (block_end < heap_end) {
        allocation_header_t* next_block = (allocation_header_t*)block_end;
        if (next_block->magic == ALLOCATION_MAGIC && next_block->is_free) {
            // Coalesce with next block
            remove_from_free_list(next_block);
            block->size += ALLOCATION_HEADER_SIZE + next_block->size;
            // Clear the coalesced block's header
            memset(next_block, 0, ALLOCATION_HEADER_SIZE);
        }
    }
    
    // TODO: Also check for previous adjacent free blocks
    // This would require a more sophisticated data structure
}

/**
 * WebAssembly memory allocation - FIXED VERSION
 */
void* __wasm_malloc(uint32_t size) {
    // Validate input
    if (size == 0 || size > MAX_ALLOCATION_SIZE) {
        return NULL;
    }
    
    // Align size to 8 bytes
    size = (size + 7) & ~7;
    
    // Ensure minimum allocation size
    if (size < MIN_ALLOCATION_SIZE) {
        size = MIN_ALLOCATION_SIZE;
    }
    
    // Try to find a suitable free block first
    allocation_header_t* free_block = find_free_block(size);
    
    if (free_block != NULL) {
        // Reuse existing free block
        remove_from_free_list(free_block);
        free_block->is_free = 0;
        
        // Split the block if it's significantly larger than needed
        if (free_block->size >= size + ALLOCATION_HEADER_SIZE + MIN_ALLOCATION_SIZE) {
            uint32_t remaining_size = free_block->size - size - ALLOCATION_HEADER_SIZE;
            allocation_header_t* split_block = (allocation_header_t*)((uint32_t)free_block + ALLOCATION_HEADER_SIZE + size);
            
            // Initialize the split block
            split_block->size = remaining_size;
            split_block->magic = ALLOCATION_MAGIC;
            split_block->is_free = 1;
            split_block->next_free = 0;
            
            // Update original block size
            free_block->size = size;
            
            // Add split block to free list
            add_to_free_list(split_block);
        }
        
        // Update statistics
        total_allocations++;
        bytes_allocated += size;
        
        return (void*)((uint32_t)free_block + ALLOCATION_HEADER_SIZE);
    }
    
    // No suitable free block found, allocate from heap end
    uint32_t total_needed = ALLOCATION_HEADER_SIZE + size;
    
    // Check if we have enough memory
    if (heap_current + total_needed > heap_end) {
        return NULL;  // Out of memory
    }
    
    // Create new allocation
    allocation_header_t* header = (allocation_header_t*)heap_current;
    header->size = size;
    header->magic = ALLOCATION_MAGIC;
    header->is_free = 0;
    header->next_free = 0;
    
    void* user_ptr = (void*)(heap_current + ALLOCATION_HEADER_SIZE);
    heap_current += total_needed;
    
    // Align heap_current to 8 bytes
    heap_current = (heap_current + 7) & ~7;
    
    // Update statistics
    total_allocations++;
    bytes_allocated += size;
    uint32_t current_usage = bytes_allocated - bytes_freed;
    if (current_usage > peak_memory_usage) {
        peak_memory_usage = current_usage;
    }
    
    return user_ptr;
}

/**
 * WebAssembly memory deallocation - FIXED VERSION
 */
void __wasm_free(void* ptr) {
    if (ptr == NULL) {
        return;
    }
    
    // Calculate header address
    allocation_header_t* header = (allocation_header_t*)((uint32_t)ptr - ALLOCATION_HEADER_SIZE);
    
    // Validate header
    if (header->magic != ALLOCATION_MAGIC) {
        // Corruption or invalid pointer
        return;
    }
    
    if (header->is_free) {
        // Double free detected
        return;
    }
    
    // Update statistics
    total_deallocations++;
    bytes_freed += header->size;
    
    // Add to free list
    add_to_free_list(header);
    
    // Attempt to coalesce with adjacent free blocks
    coalesce_free_blocks(header);
    
    // Clear the user data to help detect use-after-free
    memset(ptr, 0xDD, header->size);  // Use 0xDD as freed memory marker
}

/**
 * Memory statistics and debugging functions
 */
uint32_t __wasm_get_memory_stats(void) {
    // Returns a packed value with statistics
    // Upper 16 bits: active allocations, Lower 16 bits: free blocks
    uint32_t active_allocations = total_allocations - total_deallocations;
    
    // Count free blocks
    uint32_t free_blocks = 0;
    if (free_list_head != 0) {
        uint32_t current = free_list_head;
        do {
            free_blocks++;
            allocation_header_t* header = (allocation_header_t*)current;
            current = header->next_free;
        } while (current != free_list_head && current != 0 && free_blocks < 1000);
    }
    
    return (active_allocations << 16) | (free_blocks & 0xFFFF);
}

uint32_t __wasm_get_peak_memory_usage(void) {
    return peak_memory_usage;
}

uint32_t __wasm_get_current_memory_usage(void) {
    return bytes_allocated - bytes_freed;
}

/**
 * Memory validation and leak detection
 */
int __wasm_validate_memory(void) {
    // Walk all allocations and verify headers
    uint32_t addr = heap_start;
    uint32_t valid_allocations = 0;
    
    while (addr < heap_current) {
        allocation_header_t* header = (allocation_header_t*)addr;
        
        if (header->magic != ALLOCATION_MAGIC) {
            return -1;  // Corruption detected
        }
        
        if (!header->is_free) {
            valid_allocations++;
        }
        
        addr += ALLOCATION_HEADER_SIZE + header->size;
        
        // Safety check to prevent infinite loop
        if (addr <= (uint32_t)header) {
            return -2;  // Invalid size detected
        }
    }
    
    return valid_allocations;  // Return count of valid allocations
}

// Original function implementations with proper memory management

/**
 * String output for WebAssembly (export to host)
 */
extern void wasm_print_string(const char* str);

/**
 * Print function for CURSED vibez.spill()
 */
void cursed_print(const char* str) {
    wasm_print_string(str);
}

/**
 * Print integer
 */
void cursed_print_int(int32_t value) {
    // Use stack allocation for temporary buffer (no heap allocation)
    char buffer[16];
    int32_t temp = value;
    int pos = 0;
    
    if (temp == 0) {
        buffer[pos++] = '0';
    } else {
        if (temp < 0) {
            buffer[pos++] = '-';
            temp = -temp;
        }
        
        // Convert to string (reverse order)
        char digits[16];
        int digit_count = 0;
        while (temp > 0) {
            digits[digit_count++] = '0' + (temp % 10);
            temp /= 10;
        }
        
        // Reverse digits into buffer
        for (int i = digit_count - 1; i >= 0; i--) {
            buffer[pos++] = digits[i];
        }
    }
    
    buffer[pos] = '\0';
    cursed_print(buffer);
}

/**
 * Print float
 */
void cursed_print_float(float value) {
    // Simplified float printing - using stack allocation only
    if (value == 0.0f) {
        cursed_print("0.0");
        return;
    }
    
    char buffer[32];  // Stack allocation
    // Simple float conversion (basic implementation)
    int32_t integer_part = (int32_t)value;
    cursed_print_int(integer_part);
    cursed_print(".");
    
    float fractional = value - integer_part;
    if (fractional < 0) fractional = -fractional;
    
    // Print first few decimal places
    for (int i = 0; i < 3; i++) {
        fractional *= 10;
        int digit = (int)fractional;
        buffer[0] = '0' + digit;
        buffer[1] = '\0';
        cursed_print(buffer);
        fractional -= digit;
    }
}

/**
 * WebAssembly exports - Fixed versions
 */
__attribute__((export_name("malloc")))
void* exported_malloc(uint32_t size) {
    return __wasm_malloc(size);
}

__attribute__((export_name("free")))
void exported_free(void* ptr) {
    __wasm_free(ptr);
}

__attribute__((export_name("cursed_print")))
void exported_cursed_print(const char* str) {
    cursed_print(str);
}

__attribute__((export_name("cursed_print_int")))
void exported_cursed_print_int(int32_t value) {
    cursed_print_int(value);
}

__attribute__((export_name("cursed_print_float")))
void exported_cursed_print_float(float value) {
    cursed_print_float(value);
}

// Export memory management functions for debugging
__attribute__((export_name("wasm_memory_init")))
void exported_wasm_memory_init(uint32_t memory_size_pages) {
    __wasm_memory_init(memory_size_pages);
}

__attribute__((export_name("wasm_memory_cleanup")))
void exported_wasm_memory_cleanup(void) {
    __wasm_memory_cleanup();
}

__attribute__((export_name("wasm_get_memory_stats")))
uint32_t exported_wasm_get_memory_stats(void) {
    return __wasm_get_memory_stats();
}

__attribute__((export_name("wasm_validate_memory")))
int exported_wasm_validate_memory(void) {
    return __wasm_validate_memory();
}
