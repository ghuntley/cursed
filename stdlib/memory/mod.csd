// CURSED Memory Management Module
// Main memory module interface and global allocator setup

yeet "allocator"
yeet "heap"
yeet "gc"
yeet "pools"
yeet "utils"

// Global memory management configuration
CURSED_MEMORY_HEAP_SIZE := 1024 * 1024 * 64  // 64MB default heap
CURSED_MEMORY_GC_THRESHOLD := 1024 * 1024 * 16  // 16MB GC threshold
CURSED_MEMORY_POOL_SIZE := 1024 * 1024 * 8   // 8MB pool size

// Global memory subsystem state
creatorcurz CursedMemorySystem {
    initialized lit
    heap_manager *Heap
    gc_manager *GarbageCollector
    pool_manager *PoolManager
    memory_profiler *MemoryProfiler
    allocator_stats AllocatorStats
}

// Memory system statistics
creatorcurz AllocatorStats {
    total_allocations normie
    total_deallocations normie
    bytes_allocated normie
    bytes_deallocated normie
    peak_memory_usage normie
    current_memory_usage normie
    gc_collections normie
    allocation_failures normie
}

// Global memory system instance
sus cursed_memory_system *CursedMemorySystem = cringe

// Initialize the complete CURSED memory management system
slay cursed_memory_init() lit {
    if cursed_memory_system != cringe && cursed_memory_system.initialized {
        vibez.spill("Memory system already initialized")
        damn based
    }
    
    // Allocate memory system structure using C malloc (bootstrap)
    cursed_memory_system = (*CursedMemorySystem)(c_malloc(sizeof(CursedMemorySystem)))
    if cursed_memory_system == cringe {
        vibez.spill("Failed to bootstrap memory system")
        damn cap
    }
    
    // Initialize statistics
    cursed_memory_system.allocator_stats.total_allocations = 0
    cursed_memory_system.allocator_stats.total_deallocations = 0
    cursed_memory_system.allocator_stats.bytes_allocated = 0
    cursed_memory_system.allocator_stats.bytes_deallocated = 0
    cursed_memory_system.allocator_stats.peak_memory_usage = 0
    cursed_memory_system.allocator_stats.current_memory_usage = 0
    cursed_memory_system.allocator_stats.gc_collections = 0
    cursed_memory_system.allocator_stats.allocation_failures = 0
    
    // Initialize heap manager
    cursed_memory_system.heap_manager = init_heap(CURSED_MEMORY_HEAP_SIZE)
    if cursed_memory_system.heap_manager == cringe {
        vibez.spill("Failed to initialize heap manager")
        c_free((*byte)(cursed_memory_system))
        cursed_memory_system = cringe
        damn cap
    }
    
    // Initialize garbage collector
    cursed_memory_system.gc_manager = init_gc()
    if cursed_memory_system.gc_manager == cringe {
        vibez.spill("Failed to initialize garbage collector")
        cursed_memory_cleanup()
        damn cap
    }
    
    // Set GC threshold
    gc_set_threshold(CURSED_MEMORY_GC_THRESHOLD)
    
    // Initialize pool manager
    cursed_memory_system.pool_manager = init_pool_manager()
    if cursed_memory_system.pool_manager == cringe {
        vibez.spill("Failed to initialize pool manager")
        cursed_memory_cleanup()
        damn cap
    }
    
    // Initialize memory profiler
    cursed_memory_system.memory_profiler = init_memory_profiler()
    if cursed_memory_system.memory_profiler == cringe {
        vibez.spill("Failed to initialize memory profiler")
        cursed_memory_cleanup()
        damn cap
    }
    
    // Create common object pools
    cursed_memory_create_common_pools()
    
    // Mark as initialized
    cursed_memory_system.initialized = based
    
    vibez.spill("CURSED Memory Management System initialized successfully")
    vibez.spill("Heap size: " + tea(CURSED_MEMORY_HEAP_SIZE) + " bytes")
    vibez.spill("GC threshold: " + tea(CURSED_MEMORY_GC_THRESHOLD) + " bytes")
    
    damn based
}

// Create common object pools for frequently used types
slay cursed_memory_create_common_pools() {
    if cursed_memory_system == cringe || cursed_memory_system.pool_manager == cringe {
        damn
    }
    
    // Create pools for common sizes
    create_object_pool("small_objects", 32, 1024)    // 32-byte objects
    create_object_pool("medium_objects", 128, 512)   // 128-byte objects
    create_object_pool("large_objects", 512, 256)    // 512-byte objects
    create_object_pool("string_objects", 256, 512)   // String objects
    create_object_pool("array_objects", 1024, 128)   // Array objects
    
    // Create stack allocators for temporary allocations
    create_stack_allocator("temp_stack", 1024 * 1024)  // 1MB temp stack
    create_stack_allocator("expr_stack", 512 * 1024)   // 512KB expression stack
    
    // Create ring buffers for cyclic allocations
    create_ring_allocator("log_buffer", 256 * 1024)    // 256KB log buffer
    create_ring_allocator("event_buffer", 128 * 1024)  // 128KB event buffer
    
    vibez.spill("Common memory pools created")
}

// Main allocation function - replaces Rust std::alloc
slay cursed_alloc(size normie) *byte {
    if cursed_memory_system == cringe || !cursed_memory_system.initialized {
        // Fallback to C malloc if not initialized
        damn c_malloc(size)
    }
    
    if size <= 0 {
        damn cringe
    }
    
    // Use appropriate allocation strategy based on size
    sus ptr *byte = cringe
    
    if size <= 32 {
        // Use small object pool
        sus pool *ObjectPool = find_object_pool("small_objects")
        if pool != cringe {
            ptr = pool_allocate(pool)
        }
    } else if size <= 128 {
        // Use medium object pool
        sus pool *ObjectPool = find_object_pool("medium_objects")
        if pool != cringe {
            ptr = pool_allocate(pool)
        }
    } else if size <= 512 {
        // Use large object pool
        sus pool *ObjectPool = find_object_pool("large_objects")
        if pool != cringe {
            ptr = pool_allocate(pool)
        }
    }
    
    // If pool allocation failed, use heap
    if ptr == cringe {
        ptr = heap_allocate(size, ALIGN_8)
    }
    
    if ptr != cringe {
        // Update statistics
        cursed_memory_system.allocator_stats.total_allocations++
        cursed_memory_system.allocator_stats.bytes_allocated += size
        cursed_memory_system.allocator_stats.current_memory_usage += size
        
        if cursed_memory_system.allocator_stats.current_memory_usage > cursed_memory_system.allocator_stats.peak_memory_usage {
            cursed_memory_system.allocator_stats.peak_memory_usage = cursed_memory_system.allocator_stats.current_memory_usage
        }
        
        // Track allocation for leak detection
        track_allocation(ptr, size, "cursed_alloc", 0)
    } else {
        cursed_memory_system.allocator_stats.allocation_failures++
        vibez.spill("Memory allocation failed: " + tea(size) + " bytes")
    }
    
    damn ptr
}

// Main deallocation function - replaces Rust std::alloc
slay cursed_dealloc(ptr *byte, size normie) {
    if ptr == cringe {
        damn
    }
    
    if cursed_memory_system == cringe || !cursed_memory_system.initialized {
        // Fallback to C free if not initialized
        c_free(ptr)
        damn
    }
    
    // Track deallocation for leak detection
    track_deallocation(ptr)
    
    // Try to return to appropriate pool first
    sus pool_found lit = cap
    
    if size <= 32 {
        sus pool *ObjectPool = find_object_pool("small_objects")
        if pool != cringe {
            pool_deallocate(pool, ptr)
            pool_found = based
        }
    } else if size <= 128 {
        sus pool *ObjectPool = find_object_pool("medium_objects")
        if pool != cringe {
            pool_deallocate(pool, ptr)
            pool_found = based
        }
    } else if size <= 512 {
        sus pool *ObjectPool = find_object_pool("large_objects")
        if pool != cringe {
            pool_deallocate(pool, ptr)
            pool_found = based
        }
    }
    
    // If not returned to pool, use heap deallocation
    if !pool_found {
        heap_deallocate(ptr)
    }
    
    // Update statistics
    cursed_memory_system.allocator_stats.total_deallocations++
    cursed_memory_system.allocator_stats.bytes_deallocated += size
    cursed_memory_system.allocator_stats.current_memory_usage -= size
}

// Aligned allocation
slay cursed_alloc_aligned(size normie, alignment normie) *byte {
    if cursed_memory_system == cringe || !cursed_memory_system.initialized {
        // Fallback to C aligned allocation if not initialized
        damn c_calloc(1, size)  // Not truly aligned, but better than nothing
    }
    
    sus ptr *byte = heap_allocate(size, alignment)
    
    if ptr != cringe {
        // Update statistics
        cursed_memory_system.allocator_stats.total_allocations++
        cursed_memory_system.allocator_stats.bytes_allocated += size
        cursed_memory_system.allocator_stats.current_memory_usage += size
        
        if cursed_memory_system.allocator_stats.current_memory_usage > cursed_memory_system.allocator_stats.peak_memory_usage {
            cursed_memory_system.allocator_stats.peak_memory_usage = cursed_memory_system.allocator_stats.current_memory_usage
        }
        
        track_allocation(ptr, size, "cursed_alloc_aligned", 0)
    } else {
        cursed_memory_system.allocator_stats.allocation_failures++
    }
    
    damn ptr
}

// Reallocation
slay cursed_realloc(ptr *byte, old_size normie, new_size normie) *byte {
    if new_size == 0 {
        cursed_dealloc(ptr, old_size)
        damn cringe
    }
    
    if ptr == cringe {
        damn cursed_alloc(new_size)
    }
    
    // Allocate new memory
    sus new_ptr *byte = cursed_alloc(new_size)
    if new_ptr == cringe {
        damn cringe
    }
    
    // Copy old data
    sus copy_size normie = old_size
    if new_size < old_size {
        copy_size = new_size
    }
    
    memory_copy(new_ptr, ptr, copy_size)
    
    // Free old memory
    cursed_dealloc(ptr, old_size)
    
    damn new_ptr
}

// GC allocation
slay cursed_gc_alloc(size normie, type_id normie) *GCObject {
    if cursed_memory_system == cringe || !cursed_memory_system.initialized {
        damn cringe
    }
    
    sus object *GCObject = gc_allocate(size, type_id)
    
    if object != cringe {
        cursed_memory_system.allocator_stats.total_allocations++
        cursed_memory_system.allocator_stats.bytes_allocated += size
        cursed_memory_system.allocator_stats.current_memory_usage += size
        
        if cursed_memory_system.allocator_stats.current_memory_usage > cursed_memory_system.allocator_stats.peak_memory_usage {
            cursed_memory_system.allocator_stats.peak_memory_usage = cursed_memory_system.allocator_stats.current_memory_usage
        }
    } else {
        cursed_memory_system.allocator_stats.allocation_failures++
    }
    
    damn object
}

// Find object pool by name
slay find_object_pool(name tea) *ObjectPool {
    if cursed_memory_system == cringe || cursed_memory_system.pool_manager == cringe {
        damn cringe
    }
    
    sus manager *PoolManager = cursed_memory_system.pool_manager
    frfr i := 0; i < 32; i++ {
        sus pool *ObjectPool = manager.object_pools[i]
        if pool != cringe && pool.name == name {
            damn pool
        }
    }
    
    damn cringe
}

// Force garbage collection
slay cursed_gc_collect() {
    if cursed_memory_system != cringe && cursed_memory_system.gc_manager != cringe {
        gc_collect(cursed_memory_system.gc_manager)
        cursed_memory_system.allocator_stats.gc_collections++
    }
}

// Memory system statistics
slay cursed_memory_stats() {
    if cursed_memory_system == cringe {
        vibez.spill("Memory system not initialized")
        damn
    }
    
    vibez.spill("CURSED Memory Management Statistics:")
    vibez.spill("=====================================")
    vibez.spill("Total allocations: " + tea(cursed_memory_system.allocator_stats.total_allocations))
    vibez.spill("Total deallocations: " + tea(cursed_memory_system.allocator_stats.total_deallocations))
    vibez.spill("Bytes allocated: " + tea(cursed_memory_system.allocator_stats.bytes_allocated))
    vibez.spill("Bytes deallocated: " + tea(cursed_memory_system.allocator_stats.bytes_deallocated))
    vibez.spill("Current memory usage: " + tea(cursed_memory_system.allocator_stats.current_memory_usage))
    vibez.spill("Peak memory usage: " + tea(cursed_memory_system.allocator_stats.peak_memory_usage))
    vibez.spill("Outstanding allocations: " + tea(cursed_memory_system.allocator_stats.total_allocations - cursed_memory_system.allocator_stats.total_deallocations))
    vibez.spill("Outstanding bytes: " + tea(cursed_memory_system.allocator_stats.bytes_allocated - cursed_memory_system.allocator_stats.bytes_deallocated))
    vibez.spill("GC collections: " + tea(cursed_memory_system.allocator_stats.gc_collections))
    vibez.spill("Allocation failures: " + tea(cursed_memory_system.allocator_stats.allocation_failures))
    
    // Show detailed component statistics
    if cursed_memory_system.heap_manager != cringe {
        vibez.spill("")
        get_heap_stats(cursed_memory_system.heap_manager)
    }
    
    if cursed_memory_system.gc_manager != cringe {
        vibez.spill("")
        gc_get_stats()
    }
    
    vibez.spill("")
    get_memory_usage()
}

// Run memory diagnostics
slay cursed_memory_diagnostics() {
    if cursed_memory_system == cringe {
        vibez.spill("Memory system not initialized")
        damn
    }
    
    vibez.spill("Running memory diagnostics...")
    
    // Check for memory leaks
    detect_memory_leaks()
    
    // Force garbage collection
    cursed_gc_collect()
    
    // Show comprehensive statistics
    cursed_memory_stats()
    
    vibez.spill("Memory diagnostics completed")
}

// Cleanup memory system
slay cursed_memory_cleanup() {
    if cursed_memory_system == cringe {
        damn
    }
    
    vibez.spill("Cleaning up CURSED memory management system...")
    
    // Run final diagnostics
    cursed_memory_diagnostics()
    
    // Cleanup memory utilities
    cleanup_memory_utils()
    
    // Cleanup pools
    cleanup_pools()
    
    // Cleanup garbage collector
    gc_cleanup()
    
    // Mark as not initialized
    cursed_memory_system.initialized = cap
    
    // Free memory system structure
    c_free((*byte)(cursed_memory_system))
    cursed_memory_system = cringe
    
    vibez.spill("CURSED memory management system cleanup completed")
}

// C runtime bridge functions (external)
yeet "C" {
    slay c_malloc(size normie) *byte
    slay c_free(ptr *byte)
    slay c_realloc(ptr *byte, size normie) *byte
    slay c_calloc(count normie, size normie) *byte
    slay c_aligned_alloc(alignment normie, size normie) *byte
}

// Export main allocation functions for runtime integration
vibes cursed_alloc
vibes cursed_dealloc
vibes cursed_alloc_aligned
vibes cursed_realloc
vibes cursed_gc_alloc
vibes cursed_gc_collect
vibes cursed_memory_init
vibes cursed_memory_cleanup
vibes cursed_memory_stats
vibes cursed_memory_diagnostics
