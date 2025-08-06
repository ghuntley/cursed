fr fr memoryz module - Memory allocation, GC controls, memory profiling
fr fr Critical memory management for CURSED runtime and self-hosting

fr fr ===== MEMORY ALLOCATION FUNCTIONS =====

squad MemoryBlock {
    spill ptr normie         fr fr Pointer to memory block  
    spill size normie        fr fr Size in bytes
    spill type_id normie     fr fr Type identifier
    spill ref_count normie   fr fr Reference count
    spill gc_marked lit      fr fr GC mark flag
    spill allocated_at normie fr fr Timestamp when allocated
}

squad AllocationStats {
    spill total_allocated normie
    spill total_freed normie
    spill current_usage normie
    spill peak_usage normie
    spill allocation_count normie
    spill free_count normie
    spill gc_cycles normie
}

squad GCConfig {
    spill enable_gc lit
    spill gc_threshold normie
    spill collection_interval normie
    spill concurrent_gc lit
    spill mark_and_sweep lit
    spill generational lit
}

sus global_stats AllocationStats = AllocationStats{
    total_allocated: 0,
    total_freed: 0,
    current_usage: 0,
    peak_usage: 0,
    allocation_count: 0,
    free_count: 0,
    gc_cycles: 0
}

sus gc_config GCConfig = GCConfig{
    enable_gc: based,
    gc_threshold: 1024 * 1024 * 10, fr fr 10MB default threshold
    collection_interval: 1000,       fr fr 1000ms
    concurrent_gc: based,
    mark_and_sweep: based,
    generational: cap
}

sus allocated_blocks []MemoryBlock = []
sus gc_running lit = cap

fr fr ===== CORE ALLOCATION FUNCTIONS =====

slay allocate(size normie, type_id normie) normie {
    fr fr Allocate memory block of specified size
    lowkey size <= 0 {
        damn 0
    }
    
    sus ptr normie = runtime_allocate_memory(size)
    lowkey ptr == 0 {
        damn 0  fr fr Allocation failed
    }
    
    sus block MemoryBlock = MemoryBlock{
        ptr: ptr,
        size: size,
        type_id: type_id,
        ref_count: 1,
        gc_marked: cap,
        allocated_at: get_timestamp_nanos()
    }
    
    allocated_blocks.push(block)
    
    fr fr Update statistics
    global_stats.total_allocated = global_stats.total_allocated + size
    global_stats.current_usage = global_stats.current_usage + size
    global_stats.allocation_count = global_stats.allocation_count + 1
    
    lowkey global_stats.current_usage > global_stats.peak_usage {
        global_stats.peak_usage = global_stats.current_usage
    }
    
    fr fr Trigger GC if needed
    lowkey gc_config.enable_gc && global_stats.current_usage > gc_config.gc_threshold {
        trigger_gc()
    }
    
    damn ptr
}

slay deallocate(ptr normie) lit {
    fr fr Free memory block at pointer
    lowkey ptr == 0 {
        damn cap
    }
    
    bestie i := 0; i < allocated_blocks.len(); i++ {
        lowkey allocated_blocks[i].ptr == ptr {
            sus block MemoryBlock = allocated_blocks[i]
            
            fr fr Update statistics
            global_stats.total_freed = global_stats.total_freed + block.size
            global_stats.current_usage = global_stats.current_usage - block.size
            global_stats.free_count = global_stats.free_count + 1
            
            fr fr Free actual memory
            runtime_free_memory(ptr)
            
            fr fr Remove from tracking
            allocated_blocks.remove(i)
            damn based
        }
    }
    
    damn cap  fr fr Block not found
}

slay reallocate(ptr normie, new_size normie) normie {
    fr fr Resize memory block
    lowkey ptr == 0 {
        damn allocate(new_size, 0)
    }
    
    lowkey new_size == 0 {
        deallocate(ptr)
        damn 0
    }
    
    bestie i := 0; i < allocated_blocks.len(); i++ {
        lowkey allocated_blocks[i].ptr == ptr {
            sus old_block MemoryBlock = allocated_blocks[i]
            sus new_ptr normie = runtime_reallocate_memory(ptr, new_size)
            
            lowkey new_ptr == 0 {
                damn 0  fr fr Reallocation failed
            }
            
            fr fr Update block info
            allocated_blocks[i].ptr = new_ptr
            allocated_blocks[i].size = new_size
            
            fr fr Update statistics
            global_stats.current_usage = global_stats.current_usage - old_block.size + new_size
            lowkey global_stats.current_usage > global_stats.peak_usage {
                global_stats.peak_usage = global_stats.current_usage
            }
            
            damn new_ptr
        }
    }
    
    damn 0  fr fr Block not found
}

slay allocate_zeroed(size normie, type_id normie) normie {
    fr fr Allocate zeroed memory
    sus ptr normie = allocate(size, type_id)
    lowkey ptr != 0 {
        zero_memory(ptr, size)
    }
    damn ptr
}

slay allocate_array(count normie, element_size normie, type_id normie) normie {
    fr fr Allocate array with overflow protection
    lowkey count <= 0 || element_size <= 0 {
        damn 0
    }
    
    fr fr Check for multiplication overflow
    sus total_size normie = count * element_size
    lowkey total_size / element_size != count {
        damn 0  fr fr Overflow detected
    }
    
    damn allocate_zeroed(total_size, type_id)
}

fr fr ===== GARBAGE COLLECTION FUNCTIONS =====

slay configure_gc(config GCConfig) lit {
    gc_config = config
    damn based
}

slay get_gc_config() GCConfig {
    damn gc_config
}

slay trigger_gc() lit {
    fr fr Manual garbage collection trigger
    lowkey gc_running {
        damn cap  fr fr GC already running
    }
    
    gc_running = based
    sus collected_bytes normie = perform_gc_cycle()
    gc_running = cap
    
    global_stats.gc_cycles = global_stats.gc_cycles + 1
    damn based
}

slay perform_gc_cycle() normie {
    fr fr Perform mark and sweep garbage collection
    sus collected_bytes normie = 0
    
    lowkey gc_config.mark_and_sweep {
        fr fr Mark phase - mark all reachable objects
        mark_reachable_objects()
        
        fr fr Sweep phase - free unmarked objects  
        collected_bytes = sweep_unmarked_objects()
    }
    
    damn collected_bytes
}

slay mark_reachable_objects() lit {
    fr fr Mark all reachable objects starting from roots
    bestie i := 0; i < allocated_blocks.len(); i++ {
        lowkey allocated_blocks[i].ref_count > 0 {
            allocated_blocks[i].gc_marked = based
        } highkey {
            allocated_blocks[i].gc_marked = cap
        }
    }
    damn based
}

slay sweep_unmarked_objects() normie {
    fr fr Free all unmarked objects
    sus collected_bytes normie = 0
    sus i normie = 0
    
    bestie i < allocated_blocks.len() {
        lowkey !allocated_blocks[i].gc_marked {
            sus block MemoryBlock = allocated_blocks[i]
            collected_bytes = collected_bytes + block.size
            
            fr fr Free the memory
            runtime_free_memory(block.ptr)
            allocated_blocks.remove(i)
        } highkey {
            allocated_blocks[i].gc_marked = cap  fr fr Reset mark for next cycle
            i = i + 1
        }
    }
    
    fr fr Update statistics
    global_stats.total_freed = global_stats.total_freed + collected_bytes
    global_stats.current_usage = global_stats.current_usage - collected_bytes
    
    damn collected_bytes
}

slay increment_ref_count(ptr normie) lit {
    fr fr Increment reference count for object
    bestie i := 0; i < allocated_blocks.len(); i++ {
        lowkey allocated_blocks[i].ptr == ptr {
            allocated_blocks[i].ref_count = allocated_blocks[i].ref_count + 1
            damn based
        }
    }
    damn cap
}

slay decrement_ref_count(ptr normie) lit {
    fr fr Decrement reference count, free if zero
    bestie i := 0; i < allocated_blocks.len(); i++ {
        lowkey allocated_blocks[i].ptr == ptr {
            allocated_blocks[i].ref_count = allocated_blocks[i].ref_count - 1
            lowkey allocated_blocks[i].ref_count <= 0 {
                deallocate(ptr)
            }
            damn based
        }
    }
    damn cap
}

fr fr ===== MEMORY PROFILING FUNCTIONS =====

slay get_memory_stats() AllocationStats {
    damn global_stats
}

slay get_current_usage() normie {
    damn global_stats.current_usage
}

slay get_peak_usage() normie {
    damn global_stats.peak_usage
}

slay get_allocation_count() normie {
    damn global_stats.allocation_count
}

slay reset_memory_stats() lit {
    global_stats.total_allocated = 0
    global_stats.total_freed = 0
    global_stats.peak_usage = global_stats.current_usage
    global_stats.allocation_count = 0
    global_stats.free_count = 0
    global_stats.gc_cycles = 0
    damn based
}

slay print_memory_report() lit {
    vibez.spill("🧠 Memory Usage Report")
    vibez.spill("═══════════════════════")
    vibez.spill("Current Usage: ", format_bytes(global_stats.current_usage))
    vibez.spill("Peak Usage: ", format_bytes(global_stats.peak_usage))
    vibez.spill("Total Allocated: ", format_bytes(global_stats.total_allocated))
    vibez.spill("Total Freed: ", format_bytes(global_stats.total_freed))
    vibez.spill("Allocations: ", global_stats.allocation_count)
    vibez.spill("Deallocations: ", global_stats.free_count)
    vibez.spill("GC Cycles: ", global_stats.gc_cycles)
    vibez.spill("Active Blocks: ", allocated_blocks.len())
    damn based
}

slay format_bytes(bytes normie) tea {
    lowkey bytes < 1024 {
        damn vibez.spillstr("%d B", bytes)
    } highkey bytes < 1024 * 1024 {
        sus kb meal = bytes / 1024.0
        damn vibez.spillstr("%.2f KB", kb)
    } highkey bytes < 1024 * 1024 * 1024 {
        sus mb meal = bytes / (1024.0 * 1024.0)
        damn vibez.spillstr("%.2f MB", mb)
    } highkey {
        sus gb meal = bytes / (1024.0 * 1024.0 * 1024.0)
        damn vibez.spillstr("%.2f GB", gb)
    }
}

slay get_allocated_blocks() []MemoryBlock {
    damn allocated_blocks
}

slay find_memory_leaks() []MemoryBlock {
    fr fr Find blocks that might be leaked
    sus leaks []MemoryBlock = []
    sus current_time normie = get_timestamp_nanos()
    sus leak_threshold normie = 30 * 1000 * 1000 * 1000  fr fr 30 seconds in nanoseconds
    
    bestie block in allocated_blocks {
        lowkey (current_time - block.allocated_at) > leak_threshold && block.ref_count <= 1 {
            leaks.push(block)
        }
    }
    
    damn leaks
}

fr fr ===== MEMORY UTILITIES =====

slay zero_memory(ptr normie, size normie) lit {
    fr fr Zero out memory block
    runtime_zero_memory(ptr, size)
    damn based
}

slay copy_memory(dest normie, src normie, size normie) lit {
    fr fr Copy memory from src to dest
    runtime_copy_memory(dest, src, size)
    damn based
}

slay compare_memory(ptr1 normie, ptr2 normie, size normie) normie {
    fr fr Compare two memory blocks, return 0 if equal
    damn runtime_compare_memory(ptr1, ptr2, size)
}

slay set_memory(ptr normie, value normie, size normie) lit {
    fr fr Set memory block to value
    runtime_set_memory(ptr, value, size)
    damn based
}

slay is_valid_pointer(ptr normie) lit {
    fr fr Check if pointer points to valid allocated memory
    lowkey ptr == 0 {
        damn cap
    }
    
    bestie block in allocated_blocks {
        lowkey block.ptr == ptr {
            damn based
        }
    }
    
    damn cap
}

slay get_block_size(ptr normie) normie {
    fr fr Get size of allocated block
    bestie block in allocated_blocks {
        lowkey block.ptr == ptr {
            damn block.size
        }
    }
    damn 0
}

slay get_block_type(ptr normie) normie {
    fr fr Get type ID of allocated block
    bestie block in allocated_blocks {
        lowkey block.ptr == ptr {
            damn block.type_id
        }
    }
    damn 0
}

fr fr ===== MEMORY ALIGNMENT FUNCTIONS =====

slay align_size(size normie, alignment normie) normie {
    fr fr Align size to boundary
    lowkey alignment <= 1 {
        damn size
    }
    damn ((size + alignment - 1) / alignment) * alignment
}

slay allocate_aligned(size normie, alignment normie, type_id normie) normie {
    fr fr Allocate aligned memory
    sus aligned_size normie = align_size(size, alignment)
    sus ptr normie = allocate(aligned_size + alignment, type_id)
    
    lowkey ptr == 0 {
        damn 0
    }
    
    fr fr Calculate aligned address
    sus aligned_ptr normie = align_pointer(ptr, alignment)
    damn aligned_ptr
}

slay align_pointer(ptr normie, alignment normie) normie {
    fr fr Align pointer to boundary
    lowkey alignment <= 1 {
        damn ptr
    }
    damn ((ptr + alignment - 1) / alignment) * alignment
}

fr fr ===== RUNTIME INTERFACE FUNCTIONS =====

slay runtime_allocate_memory(size normie) normie {
    fr fr Interface to runtime memory allocator
    damn core.allocate_memory(size)
}

slay runtime_free_memory(ptr normie) lit {
    fr fr Interface to runtime memory deallocator
    core.free_memory(ptr)
    damn based
}

slay runtime_reallocate_memory(ptr normie, new_size normie) normie {
    fr fr Interface to runtime memory reallocator
    damn core.reallocate_memory(ptr, new_size)
}

slay runtime_zero_memory(ptr normie, size normie) lit {
    fr fr Interface to runtime memory zeroing
    core.zero_memory(ptr, size)
    damn based
}

slay runtime_copy_memory(dest normie, src normie, size normie) lit {
    fr fr Interface to runtime memory copying
    core.copy_memory(dest, src, size)
    damn based
}

slay runtime_compare_memory(ptr1 normie, ptr2 normie, size normie) normie {
    fr fr Interface to runtime memory comparison
    damn core.compare_memory(ptr1, ptr2, size)
}

slay runtime_set_memory(ptr normie, value normie, size normie) lit {
    fr fr Interface to runtime memory setting
    core.set_memory(ptr, value, size)
    damn based
}

slay get_timestamp_nanos() normie {
    fr fr Get current timestamp in nanoseconds
    damn core.get_timestamp_nanos()
}

fr fr ===== MEMORY POOL FUNCTIONS =====

squad MemoryPool {
    spill block_size normie
    spill block_count normie
    spill free_blocks []normie
    spill pool_ptr normie
    spill initialized lit
}

slay create_memory_pool(block_size normie, block_count normie) MemoryPool {
    fr fr Create memory pool for fixed-size allocations
    sus total_size normie = block_size * block_count
    sus pool_ptr normie = allocate(total_size, 1000)  fr fr Type ID 1000 for pools
    
    sus pool MemoryPool = MemoryPool{
        block_size: block_size,
        block_count: block_count,
        free_blocks: [],
        pool_ptr: pool_ptr,
        initialized: pool_ptr != 0
    }
    
    fr fr Initialize free block list
    lowkey pool.initialized {
        bestie i := 0; i < block_count; i++ {
            sus block_ptr normie = pool_ptr + (i * block_size)
            pool.free_blocks.push(block_ptr)
        }
    }
    
    damn pool
}

slay pool_allocate(pool MemoryPool) normie {
    fr fr Allocate from memory pool
    lowkey !pool.initialized || pool.free_blocks.len() == 0 {
        damn 0
    }
    
    sus ptr normie = pool.free_blocks[pool.free_blocks.len() - 1]
    pool.free_blocks.pop()
    damn ptr
}

slay pool_deallocate(pool MemoryPool, ptr normie) lit {
    fr fr Return block to memory pool
    lowkey !pool.initialized {
        damn cap
    }
    
    fr fr Verify ptr is within pool bounds
    sus min_ptr normie = pool.pool_ptr
    sus max_ptr normie = pool.pool_ptr + (pool.block_size * pool.block_count)
    
    lowkey ptr >= min_ptr && ptr < max_ptr {
        pool.free_blocks.push(ptr)
        damn based
    }
    
    damn cap
}

slay destroy_memory_pool(pool MemoryPool) lit {
    fr fr Destroy memory pool
    lowkey pool.initialized {
        deallocate(pool.pool_ptr)
        pool.initialized = cap
        damn based
    }
    damn cap
}
