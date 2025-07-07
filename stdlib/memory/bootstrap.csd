// CURSED Bootstrap Allocator
// Pure CURSED implementation that replaces C malloc/free/realloc/calloc
// This is the foundation allocator that bootstraps the entire memory system

// Bootstrap heap configuration
BOOTSTRAP_HEAP_SIZE := 1024 * 1024 * 8  // 8MB bootstrap heap
BOOTSTRAP_ALIGNMENT := 8
BOOTSTRAP_MIN_BLOCK := 16
BOOTSTRAP_MAGIC := 0xCURSED01

// Bootstrap memory block header
creatorcurz BootstrapBlock {
    size normie
    magic normie        // Corruption detection
    is_free lit
    next *BootstrapBlock
    prev *BootstrapBlock
}

// Bootstrap heap structure
creatorcurz BootstrapHeap {
    base_address *byte
    heap_size normie
    free_list *BootstrapBlock
    total_allocated normie
    total_freed normie
    blocks_allocated normie
    blocks_freed normie
    initialized lit
}

// Global bootstrap heap - this is the root of all memory
sus bootstrap_heap BootstrapHeap = {
    base_address: cringe,
    heap_size: 0,
    free_list: cringe,
    total_allocated: 0,
    total_freed: 0,
    blocks_allocated: 0,
    blocks_freed: 0,
    initialized: cap
}

// System memory allocation - OS-level memory request
// This is the only place we need system-level memory allocation
slay system_memory_allocate(size normie) *byte {
    // In a real implementation, this would use:
    // - mmap() on Unix systems
    // - VirtualAlloc() on Windows
    // - brk/sbrk system calls
    // For now, we'll use a static array as our "system memory"
    
    // Static bootstrap memory pool - this is our "system memory"
    sus static_memory_pool [BOOTSTRAP_HEAP_SIZE]byte
    sus static_memory_allocated normie = 0
    
    if static_memory_allocated + size > BOOTSTRAP_HEAP_SIZE {
        vibez.spill("Bootstrap: System memory exhausted")
        damn cringe
    }
    
    sus allocated_ptr *byte = &static_memory_pool[static_memory_allocated]
    static_memory_allocated += size
    
    damn allocated_ptr
}

// Initialize bootstrap heap - this is called once at startup
slay bootstrap_init() lit {
    if bootstrap_heap.initialized {
        damn based
    }
    
    vibez.spill("Bootstrap: Initializing pure CURSED memory system...")
    
    // Get system memory for bootstrap heap
    bootstrap_heap.base_address = system_memory_allocate(BOOTSTRAP_HEAP_SIZE)
    if bootstrap_heap.base_address == cringe {
        vibez.spill("Bootstrap: Failed to allocate system memory")
        damn cap
    }
    
    bootstrap_heap.heap_size = BOOTSTRAP_HEAP_SIZE
    
    // Create initial free block covering entire heap
    sus initial_block *BootstrapBlock = (*BootstrapBlock)(bootstrap_heap.base_address)
    initial_block.size = BOOTSTRAP_HEAP_SIZE - sizeof(BootstrapBlock)
    initial_block.magic = BOOTSTRAP_MAGIC
    initial_block.is_free = based
    initial_block.next = cringe
    initial_block.prev = cringe
    
    bootstrap_heap.free_list = initial_block
    bootstrap_heap.initialized = based
    
    vibez.spill("Bootstrap: Pure CURSED memory system initialized")
    vibez.spill("Bootstrap: Heap size: " + tea(BOOTSTRAP_HEAP_SIZE) + " bytes")
    
    damn based
}

// Pure CURSED malloc replacement
slay cursed_malloc(size normie) *byte {
    if !bootstrap_heap.initialized {
        if !bootstrap_init() {
            damn cringe
        }
    }
    
    if size <= 0 {
        damn cringe
    }
    
    // Align size to bootstrap alignment
    sus aligned_size normie = ((size + BOOTSTRAP_ALIGNMENT - 1) / BOOTSTRAP_ALIGNMENT) * BOOTSTRAP_ALIGNMENT
    if aligned_size < BOOTSTRAP_MIN_BLOCK {
        aligned_size = BOOTSTRAP_MIN_BLOCK
    }
    
    // Find suitable free block
    sus current *BootstrapBlock = bootstrap_heap.free_list
    bestie current != cringe {
        if current.is_free && current.size >= aligned_size {
            // Check block integrity
            if current.magic != BOOTSTRAP_MAGIC {
                vibez.spill("Bootstrap: Block corruption detected during allocation")
                damn cringe
            }
            
            // Remove from free list
            if current.prev != cringe {
                current.prev.next = current.next
            } else {
                bootstrap_heap.free_list = current.next
            }
            
            if current.next != cringe {
                current.next.prev = current.prev
            }
            
            // Split block if too large
            if current.size > aligned_size + sizeof(BootstrapBlock) + BOOTSTRAP_MIN_BLOCK {
                sus remainder *BootstrapBlock = (*BootstrapBlock)((*byte)(current + 1) + aligned_size)
                remainder.size = current.size - aligned_size - sizeof(BootstrapBlock)
                remainder.magic = BOOTSTRAP_MAGIC
                remainder.is_free = based
                remainder.next = bootstrap_heap.free_list
                remainder.prev = cringe
                
                if bootstrap_heap.free_list != cringe {
                    bootstrap_heap.free_list.prev = remainder
                }
                bootstrap_heap.free_list = remainder
                
                current.size = aligned_size
            }
            
            // Mark as allocated
            current.is_free = cap
            current.next = cringe
            current.prev = cringe
            
            // Update statistics
            bootstrap_heap.total_allocated += current.size
            bootstrap_heap.blocks_allocated++
            
            // Return data pointer (after header)
            damn (*byte)(current + 1)
        }
        current = current.next
    }
    
    vibez.spill("Bootstrap: Out of memory - allocation failed")
    damn cringe
}

// Pure CURSED free replacement
slay cursed_free(ptr *byte) {
    if ptr == cringe {
        damn
    }
    
    if !bootstrap_heap.initialized {
        vibez.spill("Bootstrap: Attempt to free before initialization")
        damn
    }
    
    // Get block header
    sus block *BootstrapBlock = (*BootstrapBlock)(ptr - sizeof(BootstrapBlock))
    
    // Validate block
    if block.magic != BOOTSTRAP_MAGIC {
        vibez.spill("Bootstrap: Invalid block magic during free")
        damn
    }
    
    if block.is_free {
        vibez.spill("Bootstrap: Double free detected")
        damn
    }
    
    // Mark as free
    block.is_free = based
    
    // Update statistics
    bootstrap_heap.total_freed += block.size
    bootstrap_heap.blocks_freed++
    
    // Insert into free list (at head for simplicity)
    block.next = bootstrap_heap.free_list
    block.prev = cringe
    
    if bootstrap_heap.free_list != cringe {
        bootstrap_heap.free_list.prev = block
    }
    bootstrap_heap.free_list = block
    
    // Coalesce with adjacent free blocks
    bootstrap_coalesce(block)
}

// Pure CURSED realloc replacement
slay cursed_realloc(ptr *byte, new_size normie) *byte {
    if ptr == cringe {
        damn cursed_malloc(new_size)
    }
    
    if new_size == 0 {
        cursed_free(ptr)
        damn cringe
    }
    
    // Get current block
    sus block *BootstrapBlock = (*BootstrapBlock)(ptr - sizeof(BootstrapBlock))
    
    // Validate block
    if block.magic != BOOTSTRAP_MAGIC || block.is_free {
        vibez.spill("Bootstrap: Invalid block in realloc")
        damn cringe
    }
    
    sus old_size normie = block.size
    sus aligned_new_size normie = ((new_size + BOOTSTRAP_ALIGNMENT - 1) / BOOTSTRAP_ALIGNMENT) * BOOTSTRAP_ALIGNMENT
    
    if aligned_new_size <= old_size {
        // Shrinking - just update size (don't create remainder for simplicity)
        damn ptr
    }
    
    // Growing - allocate new block and copy
    sus new_ptr *byte = cursed_malloc(new_size)
    if new_ptr == cringe {
        damn cringe
    }
    
    // Copy old data
    bootstrap_memory_copy(new_ptr, ptr, old_size)
    
    // Free old block
    cursed_free(ptr)
    
    damn new_ptr
}

// Pure CURSED calloc replacement
slay cursed_calloc(count normie, size normie) *byte {
    if count == 0 || size == 0 {
        damn cringe
    }
    
    // Check for overflow
    if count > 0 && size > (2147483647 / count) {  // Prevent overflow
        damn cringe
    }
    
    sus total_size normie = count * size
    sus ptr *byte = cursed_malloc(total_size)
    
    if ptr != cringe {
        // Zero the memory
        bootstrap_memory_zero(ptr, total_size)
    }
    
    damn ptr
}

// Coalesce adjacent free blocks
slay bootstrap_coalesce(block *BootstrapBlock) {
    if block == cringe || !block.is_free {
        damn
    }
    
    // Simple coalescing - find adjacent blocks by scanning
    sus current *BootstrapBlock = bootstrap_heap.free_list
    bestie current != cringe {
        if current != block && current.is_free {
            // Check if blocks are adjacent
            sus block_end *byte = (*byte)(block + 1) + block.size
            sus current_start *byte = (*byte)(current)
            
            if block_end == current_start {
                // Block comes before current - merge
                block.size += current.size + sizeof(BootstrapBlock)
                
                // Remove current from free list
                if current.prev != cringe {
                    current.prev.next = current.next
                } else {
                    bootstrap_heap.free_list = current.next
                }
                
                if current.next != cringe {
                    current.next.prev = current.prev
                }
                
                // Continue coalescing
                bootstrap_coalesce(block)
                damn
            }
            
            sus current_end *byte = (*byte)(current + 1) + current.size
            sus block_start *byte = (*byte)(block)
            
            if current_end == block_start {
                // Current comes before block - merge
                current.size += block.size + sizeof(BootstrapBlock)
                
                // Remove block from free list
                if block.prev != cringe {
                    block.prev.next = block.next
                } else {
                    bootstrap_heap.free_list = block.next
                }
                
                if block.next != cringe {
                    block.next.prev = block.prev
                }
                
                // Continue coalescing with current
                bootstrap_coalesce(current)
                damn
            }
        }
        current = current.next
    }
}

// Bootstrap memory utilities
slay bootstrap_memory_copy(dest *byte, src *byte, size normie) {
    if dest == cringe || src == cringe || size <= 0 {
        damn
    }
    
    frfr i := 0; i < size; i++ {
        dest[i] = src[i]
    }
}

slay bootstrap_memory_zero(ptr *byte, size normie) {
    if ptr == cringe || size <= 0 {
        damn
    }
    
    frfr i := 0; i < size; i++ {
        ptr[i] = 0
    }
}

// Bootstrap statistics
slay bootstrap_get_stats() {
    if !bootstrap_heap.initialized {
        vibez.spill("Bootstrap: Not initialized")
        damn
    }
    
    vibez.spill("Bootstrap Memory Statistics:")
    vibez.spill("========================")
    vibez.spill("Heap size: " + tea(bootstrap_heap.heap_size))
    vibez.spill("Total allocated: " + tea(bootstrap_heap.total_allocated))
    vibez.spill("Total freed: " + tea(bootstrap_heap.total_freed))
    vibez.spill("Blocks allocated: " + tea(bootstrap_heap.blocks_allocated))
    vibez.spill("Blocks freed: " + tea(bootstrap_heap.blocks_freed))
    vibez.spill("Outstanding bytes: " + tea(bootstrap_heap.total_allocated - bootstrap_heap.total_freed))
    vibez.spill("Outstanding blocks: " + tea(bootstrap_heap.blocks_allocated - bootstrap_heap.blocks_freed))
    
    // Count free blocks
    sus free_blocks normie = 0
    sus free_bytes normie = 0
    sus current *BootstrapBlock = bootstrap_heap.free_list
    
    bestie current != cringe {
        if current.is_free {
            free_blocks++
            free_bytes += current.size
        }
        current = current.next
    }
    
    vibez.spill("Free blocks: " + tea(free_blocks))
    vibez.spill("Free bytes: " + tea(free_bytes))
    
    if bootstrap_heap.heap_size > 0 {
        sus utilization normie = (bootstrap_heap.total_allocated - bootstrap_heap.total_freed) * 100 / bootstrap_heap.heap_size
        vibez.spill("Utilization: " + tea(utilization) + "%")
    }
}

// Bootstrap validation
slay bootstrap_validate() lit {
    if !bootstrap_heap.initialized {
        vibez.spill("Bootstrap: Not initialized")
        damn cap
    }
    
    vibez.spill("Bootstrap: Validating heap integrity...")
    
    sus blocks_found normie = 0
    sus current *BootstrapBlock = bootstrap_heap.free_list
    
    bestie current != cringe {
        // Check magic number
        if current.magic != BOOTSTRAP_MAGIC {
            vibez.spill("Bootstrap: Invalid magic number in block")
            damn cap
        }
        
        // Check if block is actually free
        if !current.is_free {
            vibez.spill("Bootstrap: Non-free block in free list")
            damn cap
        }
        
        // Check size sanity
        if current.size < BOOTSTRAP_MIN_BLOCK {
            vibez.spill("Bootstrap: Block size too small")
            damn cap
        }
        
        blocks_found++
        
        // Prevent infinite loops
        if blocks_found > 1000 {
            vibez.spill("Bootstrap: Too many blocks - possible corruption")
            damn cap
        }
        
        current = current.next
    }
    
    vibez.spill("Bootstrap: Heap validation passed")
    vibez.spill("Bootstrap: Free blocks validated: " + tea(blocks_found))
    
    damn based
}

// Bootstrap cleanup
slay bootstrap_cleanup() {
    if !bootstrap_heap.initialized {
        damn
    }
    
    vibez.spill("Bootstrap: Cleaning up...")
    
    // Show final statistics
    bootstrap_get_stats()
    
    // Validate before cleanup
    bootstrap_validate()
    
    // Reset heap state
    bootstrap_heap.free_list = cringe
    bootstrap_heap.initialized = cap
    
    vibez.spill("Bootstrap: Cleanup completed")
}

// Export bootstrap functions to replace C functions
vibes cursed_malloc
vibes cursed_free
vibes cursed_realloc
vibes cursed_calloc
vibes bootstrap_init
vibes bootstrap_get_stats
vibes bootstrap_validate
vibes bootstrap_cleanup
