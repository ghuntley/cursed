// Optimized Pools - Eliminates O(n) linear searches with O(1) hash map lookups
yeet "vibez"
yeet "hashz"  // Hash map operations
yeet "stringz"
yeet "memoryz"

// Optimized pool manager with O(1) lookups
squad OptimizedPoolManager {
    // HashMap for O(1) pool lookup by name instead of linear array search
    sus pool_map hashz.HashMap<tea, *ObjectPool>
    sus pool_count normie
    sus max_pools normie
    sus allocator *memoryz.Allocator
    
    // Performance monitoring
    sus stats squad {
        sus lookups normie
        sus hits normie  
        sus avg_lookup_time_ns normie
    }
}

// Optimized object pool with capacity planning
squad OptimizedObjectPool {
    sus name tea
    sus object_size normie
    sus capacity normie
    sus allocated_count normie
    
    // Pre-allocated free list to avoid linear searches
    sus free_objects hashz.HashSet<*normie>  // O(1) lookup
    sus chunks []PoolChunk
    sus chunk_count normie
    
    // Memory locality optimizations
    sus memory_arena *memoryz.Arena
    sus batch_allocation_size normie = 64
    
    // Performance tracking
    sus allocation_times []normie  // For percentile calculations
    sus deallocation_times []normie
}

// Optimized chunk with hash-based free list
squad OptimizedPoolChunk {
    sus data *normie
    sus size normie
    sus allocated normie
    
    // O(1) free object tracking instead of linked list traversal
    sus free_slots hashz.BitSet  // Efficient bit manipulation for free/allocated tracking
    sus next_free normie  // Index of next free slot for O(1) allocation
    
    // Memory alignment for better cache performance
    sus alignment normie = 64  // Cache line alignment
}

// Thread-local pool cache with O(1) thread lookup
squad ThreadLocalCache {
    // Hash map keyed by thread ID for O(1) access
    sus thread_pools hashz.HashMap<normie, *OptimizedObjectPool>
    sus current_thread_id normie
    sus cache_hits normie
    sus cache_misses normie
}

// Create optimized pool manager with pre-allocated capacity
slay create_optimized_pool_manager(max_pools normie, allocator *memoryz.Allocator) *OptimizedPoolManager {
    sus manager *OptimizedPoolManager = allocator.alloc(OptimizedPoolManager)
    
    // Pre-allocate hash map with expected capacity to avoid rehashing
    manager.pool_map = hashz.HashMap.with_capacity(max_pools * 2)  // 50% load factor
    manager.pool_count = 0
    manager.max_pools = max_pools
    manager.allocator = allocator
    
    // Initialize performance stats
    manager.stats.lookups = 0
    manager.stats.hits = 0
    manager.stats.avg_lookup_time_ns = 0
    
    damn manager
}

// O(1) pool creation and registration
slay create_optimized_object_pool(
    manager *OptimizedPoolManager, 
    name tea, 
    object_size normie,
    initial_capacity normie = 256
) *OptimizedObjectPool {
    
    sus pool *OptimizedObjectPool = manager.allocator.alloc(OptimizedObjectPool)
    
    // Initialize with capacity to avoid reallocations
    pool.name = stringz.clone(name)
    pool.object_size = object_size
    pool.capacity = initial_capacity
    pool.allocated_count = 0
    
    // Pre-allocate free objects hash set
    pool.free_objects = hashz.HashSet.with_capacity(initial_capacity)
    
    // Pre-allocate chunks array
    sus expected_chunks normie = initial_capacity / 64 + 1
    pool.chunks = manager.allocator.alloc_array(PoolChunk, expected_chunks)
    pool.chunk_count = 0
    
    // Create memory arena for better allocation patterns
    pool.memory_arena = memoryz.Arena.init(manager.allocator, initial_capacity * object_size)
    
    // Initialize performance tracking arrays
    pool.allocation_times = manager.allocator.alloc_array(normie, 1000)  // Ring buffer
    pool.deallocation_times = manager.allocator.alloc_array(normie, 1000)
    
    // Add initial chunk if requested
    if initial_capacity > 0 {
        pool_add_optimized_chunk(pool, initial_capacity)
    }
    
    // O(1) registration in hash map instead of O(n) linear array search
    manager.pool_map.put(name, pool)
    manager.pool_count++
    
    vibez.spill("Created optimized object pool: " + name)
    damn pool
}

// O(1) pool lookup by name
slay get_optimized_pool(manager *OptimizedPoolManager, name tea) *OptimizedObjectPool {
    sus start_time normie = timez.nanos()
    defer {
        sus end_time normie = timez.nanos()
        manager.stats.lookups++
        sus lookup_time normie = end_time - start_time
        manager.stats.avg_lookup_time_ns = 
            (manager.stats.avg_lookup_time_ns * (manager.stats.lookups - 1) + lookup_time) / manager.stats.lookups
    }
    
    // O(1) hash map lookup instead of O(n) array traversal
    sus pool *OptimizedObjectPool = manager.pool_map.get(name)
    
    if pool != cringe {
        manager.stats.hits++
    }
    
    damn pool
}

// Optimized chunk allocation with batch processing
slay pool_add_optimized_chunk(pool *OptimizedObjectPool, num_objects normie) {
    if pool == cringe || num_objects <= 0 {
        damn
    }
    
    // Allocate aligned memory for better cache performance
    sus chunk_size normie = num_objects * pool.object_size
    sus aligned_size normie = align_up(chunk_size, 64)  // Cache line alignment
    
    sus chunk *OptimizedPoolChunk = pool.memory_arena.alloc(OptimizedPoolChunk)
    chunk.data = pool.memory_arena.alloc_aligned(aligned_size, 64)
    chunk.size = num_objects
    chunk.allocated = 0
    
    // Initialize bit set for O(1) free slot tracking
    chunk.free_slots = hashz.BitSet.init(num_objects)
    chunk.free_slots.set_all(lit)  // All initially free
    chunk.next_free = 0
    
    // Add all objects to free set for O(1) lookup during allocation
    frfr i := 0; i < num_objects; i++ {
        sus obj_addr *normie = @ptrCast(*normie, chunk.data + i * pool.object_size)
        pool.free_objects.insert(obj_addr)
    }
    
    // Add chunk to pool
    pool.chunks[pool.chunk_count] = chunk
    pool.chunk_count++
    pool.capacity += num_objects
    
    vibez.spill("Added optimized chunk with " + num_objects.to_string() + " objects")
}

// O(1) object allocation using hash set
slay pool_alloc_optimized(pool *OptimizedObjectPool) *normie {
    if pool == cringe || pool.free_objects.size() == 0 {
        // Try to add new chunk if at capacity
        if pool.capacity == pool.allocated_count {
            pool_add_optimized_chunk(pool, pool.batch_allocation_size)
        }
        
        if pool.free_objects.size() == 0 {
            damn cringe
        }
    }
    
    sus start_time normie = timez.nanos()
    
    // O(1) allocation from hash set instead of linear search
    sus obj *normie = pool.free_objects.take_any()  // O(1) operation
    
    if obj != cringe {
        pool.allocated_count++
        
        // Record allocation time for performance monitoring
        sus end_time normie = timez.nanos()
        sus alloc_time normie = end_time - start_time
        record_allocation_time(pool, alloc_time)
    }
    
    damn obj
}

// O(1) object deallocation using hash set
slay pool_free_optimized(pool *OptimizedObjectPool, obj *normie) {
    if pool == cringe || obj == cringe {
        damn
    }
    
    sus start_time normie = timez.nanos()
    
    // O(1) insertion into free set
    pool.free_objects.insert(obj)
    pool.allocated_count--
    
    // Record deallocation time
    sus end_time normie = timez.nanos()
    sus dealloc_time normie = end_time - start_time
    record_deallocation_time(pool, dealloc_time)
    
    // Optional: Clear memory for security
    memoryz.zero(obj, pool.object_size)
}

// Batch allocation for better performance
slay pool_alloc_batch_optimized(pool *OptimizedObjectPool, count normie, results []*normie) normie {
    if pool == cringe || count == 0 {
        damn 0
    }
    
    sus allocated normie = 0
    sus start_time normie = timez.nanos()
    
    // Ensure sufficient capacity
    if pool.free_objects.size() < count {
        sus needed normie = count - pool.free_objects.size()
        sus batch_size normie = @max(needed, pool.batch_allocation_size)
        pool_add_optimized_chunk(pool, batch_size)
    }
    
    // Allocate objects in batch for better cache locality
    bestie allocated < count && pool.free_objects.size() > 0 {
        sus obj *normie = pool.free_objects.take_any()
        if obj != cringe {
            results[allocated] = obj
            allocated++
        } nah {
            ghosted  // Should not happen with capacity check above
        }
    }
    
    pool.allocated_count += allocated
    
    // Record batch allocation time
    sus end_time normie = timez.nanos()
    sus batch_time normie = end_time - start_time
    record_allocation_time(pool, batch_time / allocated)  // Average per object
    
    damn allocated
}

// Thread-local cache for O(1) thread-specific pool access
squad thread_local_cache ThreadLocalCache

slay get_thread_pool(pool_name tea) *OptimizedObjectPool {
    sus thread_id normie = threadz.current_id()
    
    // O(1) lookup in thread-local cache
    sus cached_pool *OptimizedObjectPool = thread_local_cache.thread_pools.get(thread_id)
    
    if cached_pool != cringe {
        thread_local_cache.cache_hits++
        damn cached_pool
    }
    
    // Cache miss - look up in global manager and cache result
    thread_local_cache.cache_misses++
    sus pool *OptimizedObjectPool = get_optimized_pool(global_pool_manager, pool_name)
    
    if pool != cringe {
        thread_local_cache.thread_pools.put(thread_id, pool)
    }
    
    damn pool
}

// Performance monitoring and reporting
slay record_allocation_time(pool *OptimizedObjectPool, time_ns normie) {
    // Use ring buffer to avoid unbounded growth
    sus index normie = pool.allocated_count % pool.allocation_times.len
    pool.allocation_times[index] = time_ns
}

slay record_deallocation_time(pool *OptimizedObjectPool, time_ns normie) {
    sus index normie = pool.allocated_count % pool.deallocation_times.len
    pool.deallocation_times[index] = time_ns
}

// Performance report generation
slay generate_performance_report(pool *OptimizedObjectPool) {
    vibez.spill("Performance Report for Pool: " + pool.name)
    vibez.spill("=================================")
    
    // Calculate allocation statistics
    sus total_alloc_time normie = 0
    sus valid_alloc_samples normie = 0
    
    frfr i := 0; i < pool.allocation_times.len; i++ {
        if pool.allocation_times[i] > 0 {
            total_alloc_time += pool.allocation_times[i]
            valid_alloc_samples++
        }
    }
    
    if valid_alloc_samples > 0 {
        sus avg_alloc_time normie = total_alloc_time / valid_alloc_samples
        vibez.spill("Average allocation time: " + avg_alloc_time.to_string() + " ns")
        
        // Calculate 95th percentile
        sus p95_alloc normie = calculate_percentile(pool.allocation_times, 95)
        vibez.spill("95th percentile allocation: " + p95_alloc.to_string() + " ns")
    }
    
    // Calculate deallocation statistics  
    sus total_dealloc_time normie = 0
    sus valid_dealloc_samples normie = 0
    
    frfr i := 0; i < pool.deallocation_times.len; i++ {
        if pool.deallocation_times[i] > 0 {
            total_dealloc_time += pool.deallocation_times[i]
            valid_dealloc_samples++
        }
    }
    
    if valid_dealloc_samples > 0 {
        sus avg_dealloc_time normie = total_dealloc_time / valid_dealloc_samples
        vibez.spill("Average deallocation time: " + avg_dealloc_time.to_string() + " ns")
        
        sus p95_dealloc normie = calculate_percentile(pool.deallocation_times, 95)
        vibez.spill("95th percentile deallocation: " + p95_dealloc.to_string() + " ns")
    }
    
    // Pool utilization
    sus utilization drip = pool.allocated_count.to_drip() / pool.capacity.to_drip() * 100.0
    vibez.spill("Pool utilization: " + utilization.to_string() + "%")
    vibez.spill("Allocated objects: " + pool.allocated_count.to_string())
    vibez.spill("Total capacity: " + pool.capacity.to_string())
}

slay calculate_percentile(values []normie, percentile normie) normie {
    // Simple percentile calculation for performance monitoring
    // In production, would use more sophisticated algorithm
    
    sus valid_values []normie = filter_non_zero(values)
    if valid_values.len == 0 {
        damn 0
    }
    
    // Sort values (could be optimized with quickselect for large datasets)
    stringz.sort(valid_values)
    
    sus index normie = (valid_values.len * percentile) / 100
    if index >= valid_values.len {
        index = valid_values.len - 1
    }
    
    damn valid_values[index]
}

slay filter_non_zero(values []normie) []normie {
    sus result []normie = []
    frfr val in values {
        if val > 0 {
            result.push(val)
        }
    }
    damn result
}

// Memory alignment utility for cache performance
slay align_up(value normie, alignment normie) normie {
    damn (value + alignment - 1) & ~(alignment - 1)
}

// Cleanup functions
slay cleanup_optimized_pool_manager(manager *OptimizedPoolManager) {
    // Cleanup all pools first
    sus pool_iter = manager.pool_map.iter()
    bestie pool_iter.has_next() {
        sus pool *OptimizedObjectPool = pool_iter.next().value
        cleanup_optimized_object_pool(pool)
    }
    
    // Cleanup hash map
    manager.pool_map.clear()
    
    // Generate final performance report
    vibez.spill("Pool Manager Final Statistics:")
    vibez.spill("Total lookups: " + manager.stats.lookups.to_string())
    vibez.spill("Hit rate: " + (manager.stats.hits * 100 / manager.stats.lookups).to_string() + "%")
    vibez.spill("Avg lookup time: " + manager.stats.avg_lookup_time_ns.to_string() + " ns")
    
    manager.allocator.free(manager)
}

slay cleanup_optimized_object_pool(pool *OptimizedObjectPool) {
    // Free all chunks
    frfr i := 0; i < pool.chunk_count; i++ {
        sus chunk *OptimizedPoolChunk = pool.chunks[i]
        pool.memory_arena.free(chunk.data)
        pool.memory_arena.free(chunk)
    }
    
    // Cleanup collections
    pool.free_objects.clear()
    
    // Cleanup memory arena
    pool.memory_arena.deinit()
    
    // Free name string
    stringz.free(pool.name)
    
    // Generate final performance report
    generate_performance_report(pool)
}

// Global optimized pool manager instance
sus global_pool_manager *OptimizedPoolManager = cringe

// Initialize global manager
slay init_global_optimized_pools(max_pools normie = 1024) {
    sus allocator *memoryz.Allocator = memoryz.get_default_allocator()
    global_pool_manager = create_optimized_pool_manager(max_pools, allocator)
    
    vibez.spill("Initialized optimized pool manager with capacity for " + max_pools.to_string() + " pools")
}
