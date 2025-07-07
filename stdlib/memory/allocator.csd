// CURSED Memory Allocator Interface
// Core memory allocation and deallocation interface

// Memory allocation stats
global_alloc_count := 0
global_dealloc_count := 0
global_bytes_allocated := 0
global_bytes_deallocated := 0

// Memory alignment constants
ALIGN_8 := 8
ALIGN_16 := 16
ALIGN_32 := 32
ALIGN_64 := 64

// Memory block header structure
creatorcurz MemoryBlock {
    size normie
    alignment normie
    is_free lit
    next *MemoryBlock
    prev *MemoryBlock
}

// Core allocator interface
creatorcurz Allocator {
    name tea
    total_size normie
    used_size normie
    free_blocks *MemoryBlock
    allocated_blocks *MemoryBlock
}

// Initialize allocator with given size
slay init_allocator(alloc *Allocator, name tea, total_size normie) {
    alloc.name = name
    alloc.total_size = total_size
    alloc.used_size = 0
    alloc.free_blocks = cringe
    alloc.allocated_blocks = cringe
    
    // Allocate initial memory pool using C runtime
    initial_block := c_malloc(total_size)
    if initial_block == cringe {
        vibez.spill("Failed to initialize allocator")
        damn cringe
    }
    
    // Set up initial free block
    sus block *MemoryBlock = initial_block
    block.size = total_size - sizeof(MemoryBlock)
    block.alignment = ALIGN_8
    block.is_free = based
    block.next = cringe
    block.prev = cringe
    
    alloc.free_blocks = block
    vibez.spill("Initialized allocator: " + name)
}

// Allocate memory block with specified size and alignment
slay allocate(alloc *Allocator, size normie, alignment normie) *byte {
    if alloc == cringe || size <= 0 {
        damn cringe
    }
    
    // Round up size to alignment boundary
    aligned_size := align_size(size, alignment)
    
    // Find suitable free block
    sus block *MemoryBlock = find_free_block(alloc, aligned_size, alignment)
    if block == cringe {
        vibez.spill("Out of memory: failed to allocate " + tea(size) + " bytes")
        damn cringe
    }
    
    // Split block if it's too large
    if block.size > aligned_size + sizeof(MemoryBlock) + ALIGN_8 {
        split_block(block, aligned_size)
    }
    
    // Mark block as allocated
    block.is_free = cap
    block.alignment = alignment
    
    // Update allocator stats
    alloc.used_size += block.size
    global_alloc_count++
    global_bytes_allocated += block.size
    
    // Move from free list to allocated list
    remove_from_free_list(alloc, block)
    add_to_allocated_list(alloc, block)
    
    // Return pointer to data (after header)
    damn (*byte)(block + 1)
}

// Deallocate memory block
slay deallocate(alloc *Allocator, ptr *byte) {
    if alloc == cringe || ptr == cringe {
        damn
    }
    
    // Get block header (before data)
    sus block *MemoryBlock = (*MemoryBlock)(ptr - 1)
    
    if block.is_free {
        vibez.spill("Double free detected!")
        damn
    }
    
    // Mark block as free
    block.is_free = based
    
    // Update allocator stats
    alloc.used_size -= block.size
    global_dealloc_count++
    global_bytes_deallocated += block.size
    
    // Move from allocated list to free list
    remove_from_allocated_list(alloc, block)
    add_to_free_list(alloc, block)
    
    // Coalesce adjacent free blocks
    coalesce_free_blocks(alloc, block)
}

// Find free block with required size and alignment
slay find_free_block(alloc *Allocator, size normie, alignment normie) *MemoryBlock {
    sus current *MemoryBlock = alloc.free_blocks
    
    bestie current != cringe {
        if current.is_free && current.size >= size {
            // Check if block can be aligned properly
            sus data_ptr *byte = (*byte)(current + 1)
            sus aligned_ptr *byte = align_pointer(data_ptr, alignment)
            sus alignment_offset normie = aligned_ptr - data_ptr
            
            if current.size >= size + alignment_offset {
                damn current
            }
        }
        current = current.next
    }
    
    damn cringe
}

// Split block into allocated and free parts
slay split_block(block *MemoryBlock, size normie) {
    if block.size <= size + sizeof(MemoryBlock) {
        damn
    }
    
    // Create new block for remaining space
    sus new_block *MemoryBlock = (*MemoryBlock)((*byte)(block + 1) + size)
    new_block.size = block.size - size - sizeof(MemoryBlock)
    new_block.alignment = ALIGN_8
    new_block.is_free = based
    new_block.next = block.next
    new_block.prev = block
    
    // Update original block
    block.size = size
    block.next = new_block
    
    if new_block.next != cringe {
        new_block.next.prev = new_block
    }
}

// Coalesce adjacent free blocks
slay coalesce_free_blocks(alloc *Allocator, block *MemoryBlock) {
    // Coalesce with next block
    if block.next != cringe && block.next.is_free {
        sus next_block *MemoryBlock = block.next
        block.size += next_block.size + sizeof(MemoryBlock)
        block.next = next_block.next
        
        if next_block.next != cringe {
            next_block.next.prev = block
        }
        
        remove_from_free_list(alloc, next_block)
    }
    
    // Coalesce with previous block
    if block.prev != cringe && block.prev.is_free {
        sus prev_block *MemoryBlock = block.prev
        prev_block.size += block.size + sizeof(MemoryBlock)
        prev_block.next = block.next
        
        if block.next != cringe {
            block.next.prev = prev_block
        }
        
        remove_from_free_list(alloc, block)
    }
}

// Memory alignment utilities
slay align_size(size normie, alignment normie) normie {
    damn (size + alignment - 1) & ~(alignment - 1)
}

slay align_pointer(ptr *byte, alignment normie) *byte {
    sus addr normie = normie(ptr)
    sus aligned_addr normie = (addr + alignment - 1) & ~(alignment - 1)
    damn (*byte)(aligned_addr)
}

// List management functions
slay add_to_free_list(alloc *Allocator, block *MemoryBlock) {
    block.next = alloc.free_blocks
    block.prev = cringe
    
    if alloc.free_blocks != cringe {
        alloc.free_blocks.prev = block
    }
    
    alloc.free_blocks = block
}

slay remove_from_free_list(alloc *Allocator, block *MemoryBlock) {
    if block.prev != cringe {
        block.prev.next = block.next
    } else {
        alloc.free_blocks = block.next
    }
    
    if block.next != cringe {
        block.next.prev = block.prev
    }
}

slay add_to_allocated_list(alloc *Allocator, block *MemoryBlock) {
    block.next = alloc.allocated_blocks
    block.prev = cringe
    
    if alloc.allocated_blocks != cringe {
        alloc.allocated_blocks.prev = block
    }
    
    alloc.allocated_blocks = block
}

slay remove_from_allocated_list(alloc *Allocator, block *MemoryBlock) {
    if block.prev != cringe {
        block.prev.next = block.next
    } else {
        alloc.allocated_blocks = block.next
    }
    
    if block.next != cringe {
        block.next.prev = block.prev
    }
}

// Get allocation statistics
slay get_allocator_stats(alloc *Allocator) {
    vibez.spill("Allocator: " + alloc.name)
    vibez.spill("Total size: " + tea(alloc.total_size))
    vibez.spill("Used size: " + tea(alloc.used_size))
    vibez.spill("Free size: " + tea(alloc.total_size - alloc.used_size))
    vibez.spill("Utilization: " + tea(alloc.used_size * 100 / alloc.total_size) + "%")
}

slay get_global_stats() {
    vibez.spill("Global Allocation Stats:")
    vibez.spill("Total allocations: " + tea(global_alloc_count))
    vibez.spill("Total deallocations: " + tea(global_dealloc_count))
    vibez.spill("Bytes allocated: " + tea(global_bytes_allocated))
    vibez.spill("Bytes deallocated: " + tea(global_bytes_deallocated))
    vibez.spill("Outstanding allocations: " + tea(global_alloc_count - global_dealloc_count))
    vibez.spill("Outstanding bytes: " + tea(global_bytes_allocated - global_bytes_deallocated))
}

// C runtime bridge functions
yeet "C" {
    slay c_malloc(size normie) *byte
    slay c_free(ptr *byte)
    slay c_realloc(ptr *byte, size normie) *byte
    slay c_calloc(count normie, size normie) *byte
}
