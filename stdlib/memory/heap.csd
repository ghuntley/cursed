// CURSED Heap Manager
// Heap initialization and management with free list optimization

yeet "bootstrap"
yeet "allocator"

// Heap configuration constants
DEFAULT_HEAP_SIZE := 1024 * 1024 * 16  // 16MB default heap
MIN_BLOCK_SIZE := 32
MAX_BLOCK_SIZE := 1024 * 1024 * 4      // 4MB max block

// Free list bins for different size classes
NUM_BINS := 32
BIN_SIZE_MULTIPLIER := 2

// Heap structure
creatorcurz Heap {
    allocator Allocator
    bin_heads [NUM_BINS]*MemoryBlock
    large_blocks *MemoryBlock
    heap_start *byte
    heap_end *byte
    total_allocations normie
    total_deallocations normie
    fragmentation_ratio drip
}

// Global heap instance
sus global_heap *Heap = cringe

// Initialize heap with specified size
slay init_heap(size normie) *Heap {
    if size <= 0 {
        size = DEFAULT_HEAP_SIZE
    }
    
    // Allocate heap structure using bootstrap allocator
    sus heap *Heap = (*Heap)(cursed_malloc(sizeof(Heap)))
    if heap == cringe {
        vibez.spill("Failed to allocate heap structure")
        damn cringe
    }
    
    // Initialize allocator
    init_allocator(&heap.allocator, "Main Heap", size)
    
    // Initialize free list bins
    frfr i := 0; i < NUM_BINS; i++ {
        heap.bin_heads[i] = cringe
    }
    
    heap.large_blocks = cringe
    heap.heap_start = (*byte)(heap.allocator.free_blocks)
    heap.heap_end = heap.heap_start + size
    heap.total_allocations = 0
    heap.total_deallocations = 0
    heap.fragmentation_ratio = 0.0
    
    vibez.spill("Heap initialized with " + tea(size) + " bytes")
    damn heap
}

// Get or create global heap
slay get_global_heap() *Heap {
    if global_heap == cringe {
        global_heap = init_heap(DEFAULT_HEAP_SIZE)
    }
    damn global_heap
}

// Heap allocation with bin-based free list
slay heap_allocate(size normie, alignment normie) *byte {
    sus heap *Heap = get_global_heap()
    if heap == cringe {
        damn cringe
    }
    
    // Determine size class and bin
    sus bin_index normie = get_bin_index(size)
    sus aligned_size normie = align_size(size, alignment)
    
    // Try to find block in appropriate bin
    sus block *MemoryBlock = cringe
    
    if bin_index < NUM_BINS {
        block = find_block_in_bin(heap, bin_index, aligned_size, alignment)
    }
    
    // If not found in bin, try larger bins
    if block == cringe {
        frfr i := bin_index + 1; i < NUM_BINS; i++ {
            block = find_block_in_bin(heap, i, aligned_size, alignment)
            if block != cringe {
                ghosted
            }
        }
    }
    
    // If still not found, try large blocks
    if block == cringe {
        block = find_large_block(heap, aligned_size, alignment)
    }
    
    // If still not found, expand heap
    if block == cringe {
        block = expand_heap(heap, aligned_size, alignment)
    }
    
    if block == cringe {
        vibez.spill("Heap allocation failed: " + tea(size) + " bytes")
        damn cringe
    }
    
    // Remove from free list
    remove_from_bin(heap, block, bin_index)
    
    // Split block if necessary
    if block.size > aligned_size + MIN_BLOCK_SIZE {
        split_heap_block(heap, block, aligned_size)
    }
    
    // Mark as allocated
    block.is_free = cap
    heap.total_allocations++
    
    damn (*byte)(block + 1)
}

// Heap deallocation with bin insertion
slay heap_deallocate(ptr *byte) {
    if ptr == cringe {
        damn
    }
    
    sus heap *Heap = get_global_heap()
    if heap == cringe {
        damn
    }
    
    // Get block header
    sus block *MemoryBlock = (*MemoryBlock)(ptr - 1)
    
    if block.is_free {
        vibez.spill("Double free detected in heap!")
        damn
    }
    
    // Mark as free
    block.is_free = based
    heap.total_deallocations++
    
    // Coalesce with adjacent blocks
    coalesce_heap_blocks(heap, block)
    
    // Insert into appropriate bin
    sus bin_index normie = get_bin_index(block.size)
    insert_into_bin(heap, block, bin_index)
    
    // Update fragmentation ratio
    update_fragmentation_ratio(heap)
}

// Find block in specific bin
slay find_block_in_bin(heap *Heap, bin_index normie, size normie, alignment normie) *MemoryBlock {
    if bin_index >= NUM_BINS {
        damn cringe
    }
    
    sus current *MemoryBlock = heap.bin_heads[bin_index]
    
    bestie current != cringe {
        if current.is_free && current.size >= size {
            // Check alignment
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

// Find large block (outside bins)
slay find_large_block(heap *Heap, size normie, alignment normie) *MemoryBlock {
    sus current *MemoryBlock = heap.large_blocks
    
    bestie current != cringe {
        if current.is_free && current.size >= size {
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

// Expand heap when out of memory
slay expand_heap(heap *Heap, size normie, alignment normie) *MemoryBlock {
    sus expansion_size normie = size * 2
    if expansion_size < DEFAULT_HEAP_SIZE / 4 {
        expansion_size = DEFAULT_HEAP_SIZE / 4
    }
    
    // Allocate new memory from bootstrap allocator
    sus new_memory *byte = cursed_malloc(expansion_size)
    if new_memory == cringe {
        vibez.spill("Failed to expand heap")
        damn cringe
    }
    
    // Create new block
    sus new_block *MemoryBlock = (*MemoryBlock)(new_memory)
    new_block.size = expansion_size - sizeof(MemoryBlock)
    new_block.alignment = alignment
    new_block.is_free = based
    new_block.next = cringe
    new_block.prev = cringe
    
    // Update heap bounds
    heap.heap_end += expansion_size
    
    vibez.spill("Heap expanded by " + tea(expansion_size) + " bytes")
    damn new_block
}

// Split heap block
slay split_heap_block(heap *Heap, block *MemoryBlock, size normie) {
    if block.size <= size + sizeof(MemoryBlock) + MIN_BLOCK_SIZE {
        damn
    }
    
    // Create remainder block
    sus remainder_block *MemoryBlock = (*MemoryBlock)((*byte)(block + 1) + size)
    remainder_block.size = block.size - size - sizeof(MemoryBlock)
    remainder_block.alignment = ALIGN_8
    remainder_block.is_free = based
    remainder_block.next = cringe
    remainder_block.prev = cringe
    
    // Update original block
    block.size = size
    
    // Insert remainder into appropriate bin
    sus bin_index normie = get_bin_index(remainder_block.size)
    insert_into_bin(heap, remainder_block, bin_index)
}

// Coalesce adjacent heap blocks
slay coalesce_heap_blocks(heap *Heap, block *MemoryBlock) {
    // Check if we can coalesce with next block
    sus next_ptr *byte = (*byte)(block + 1) + block.size
    sus next_block *MemoryBlock = (*MemoryBlock)(next_ptr)
    
    if next_ptr < heap.heap_end && next_block.is_free {
        // Remove next block from its bin
        sus next_bin_index normie = get_bin_index(next_block.size)
        remove_from_bin(heap, next_block, next_bin_index)
        
        // Merge blocks
        block.size += next_block.size + sizeof(MemoryBlock)
    }
    
    // Check if we can coalesce with previous block
    // This requires scanning backwards, which is expensive
    // For now, we'll skip backward coalescing
}

// Bin management functions
slay get_bin_index(size normie) normie {
    if size < MIN_BLOCK_SIZE {
        damn 0
    }
    
    sus bin_index normie = 0
    sus bin_size normie = MIN_BLOCK_SIZE
    
    bestie bin_index < NUM_BINS - 1 && bin_size < size {
        bin_size *= BIN_SIZE_MULTIPLIER
        bin_index++
    }
    
    damn bin_index
}

slay insert_into_bin(heap *Heap, block *MemoryBlock, bin_index normie) {
    if bin_index >= NUM_BINS {
        // Insert into large blocks list
        block.next = heap.large_blocks
        if heap.large_blocks != cringe {
            heap.large_blocks.prev = block
        }
        heap.large_blocks = block
        block.prev = cringe
        damn
    }
    
    // Insert at head of bin
    block.next = heap.bin_heads[bin_index]
    block.prev = cringe
    
    if heap.bin_heads[bin_index] != cringe {
        heap.bin_heads[bin_index].prev = block
    }
    
    heap.bin_heads[bin_index] = block
}

slay remove_from_bin(heap *Heap, block *MemoryBlock, bin_index normie) {
    if bin_index >= NUM_BINS {
        // Remove from large blocks list
        if block.prev != cringe {
            block.prev.next = block.next
        } else {
            heap.large_blocks = block.next
        }
        
        if block.next != cringe {
            block.next.prev = block.prev
        }
        damn
    }
    
    // Remove from bin
    if block.prev != cringe {
        block.prev.next = block.next
    } else {
        heap.bin_heads[bin_index] = block.next
    }
    
    if block.next != cringe {
        block.next.prev = block.prev
    }
}

// Update fragmentation ratio
slay update_fragmentation_ratio(heap *Heap) {
    sus free_blocks normie = 0
    sus total_free_size normie = 0
    
    // Count free blocks in bins
    frfr i := 0; i < NUM_BINS; i++ {
        sus current *MemoryBlock = heap.bin_heads[i]
        bestie current != cringe {
            if current.is_free {
                free_blocks++
                total_free_size += current.size
            }
            current = current.next
        }
    }
    
    // Count large free blocks
    sus current *MemoryBlock = heap.large_blocks
    bestie current != cringe {
        if current.is_free {
            free_blocks++
            total_free_size += current.size
        }
        current = current.next
    }
    
    if total_free_size > 0 {
        heap.fragmentation_ratio = drip(free_blocks) / drip(total_free_size / MIN_BLOCK_SIZE)
    } else {
        heap.fragmentation_ratio = 0.0
    }
}

// Get heap statistics
slay get_heap_stats(heap *Heap) {
    vibez.spill("Heap Statistics:")
    vibez.spill("Total allocations: " + tea(heap.total_allocations))
    vibez.spill("Total deallocations: " + tea(heap.total_deallocations))
    vibez.spill("Outstanding allocations: " + tea(heap.total_allocations - heap.total_deallocations))
    vibez.spill("Fragmentation ratio: " + tea(heap.fragmentation_ratio))
    vibez.spill("Heap start: " + tea(normie(heap.heap_start)))
    vibez.spill("Heap end: " + tea(normie(heap.heap_end)))
    vibez.spill("Heap size: " + tea(normie(heap.heap_end - heap.heap_start)))
}

// Heap defragmentation
slay defragment_heap(heap *Heap) {
    vibez.spill("Starting heap defragmentation...")
    
    // Compact free blocks by moving allocated blocks
    // This is a simplified defragmentation - real implementation would be more complex
    
    frfr i := 0; i < NUM_BINS; i++ {
        sus current *MemoryBlock = heap.bin_heads[i]
        bestie current != cringe {
            sus next *MemoryBlock = current.next
            if current.is_free {
                coalesce_heap_blocks(heap, current)
            }
            current = next
        }
    }
    
    update_fragmentation_ratio(heap)
    vibez.spill("Heap defragmentation completed")
}

// Cleanup heap
slay cleanup_heap(heap *Heap) {
    if heap == cringe {
        damn
    }
    
    vibez.spill("Cleaning up heap...")
    
    // Free all remaining blocks
    frfr i := 0; i < NUM_BINS; i++ {
        heap.bin_heads[i] = cringe
    }
    
    heap.large_blocks = cringe
    
    // Note: In a real implementation, we'd free the underlying memory
    // For now, we rely on the C runtime to clean up
    
    vibez.spill("Heap cleanup completed")
}
