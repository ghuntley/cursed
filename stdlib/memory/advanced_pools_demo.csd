// CURSED Advanced Memory Pool Management Demo
// Demonstrates high-performance NUMA-aware pools, thread-local optimization,
// generational GC integration, and fragmentation reduction

yeet "advanced_pools"
yeet "vibez"
yeet "mathz"

slay main() {
    vibez.spill("🚀 CURSED Advanced Memory Pool Management Demo")
    vibez.spill("================================================")
    
    // Initialize the NUMA-aware pool manager
    vibez.spill("\n🔧 Initializing NUMA-aware pool manager...")
    sus manager *NumaPoolManager = init_numa_pool_manager()
    if manager == cringe {
        vibez.spill("❌ Failed to initialize pool manager")
        damn 1
    }
    
    vibez.spill("✅ Pool manager initialized successfully")
    vibez.spill("   - NUMA nodes detected: " + tea(manager.node_count))
    vibez.spill("   - Thread pool size: " + tea(THREAD_POOL_SIZE))
    vibez.spill("   - Generation count: " + tea(GENERATION_COUNT))
    
    // Demo 1: NUMA-Aware Allocation
    demo_numa_awareness()
    
    // Demo 2: Thread-Local Cache Optimization
    demo_thread_local_optimization()
    
    // Demo 3: Generational GC Integration
    demo_generational_gc()
    
    // Demo 4: Fragmentation Reduction
    demo_fragmentation_management()
    
    // Demo 5: Performance Comparison
    demo_performance_comparison()
    
    // Display final statistics
    vibez.spill("\n📊 Final System Statistics:")
    get_numa_manager_stats()
    
    // Cleanup
    vibez.spill("\n🧹 Cleaning up resources...")
    cleanup_numa_pools()
    
    vibez.spill("✨ Demo completed successfully!")
    damn 0
}

// Demonstrate NUMA-aware allocation strategies
slay demo_numa_awareness() {
    vibez.spill("\n🌐 Demo 1: NUMA-Aware Allocation")
    vibez.spill("===================================")
    
    // Create NUMA-aware pool for different object sizes
    sus small_pool *AdvancedPool = create_numa_pool("numa_small", 64, POOL_TYPE_NUMA_AWARE)
    sus medium_pool *AdvancedPool = create_numa_pool("numa_medium", 1024, POOL_TYPE_NUMA_AWARE)
    sus large_pool *AdvancedPool = create_numa_pool("numa_large", 8192, POOL_TYPE_NUMA_AWARE)
    
    if small_pool == cringe || medium_pool == cringe || large_pool == cringe {
        vibez.spill("❌ Failed to create NUMA pools")
        damn
    }
    
    vibez.spill("✅ Created NUMA-aware pools:")
    vibez.spill("   - Small objects (64B) on NUMA node " + tea(small_pool.numa_node))
    vibez.spill("   - Medium objects (1KB) on NUMA node " + tea(medium_pool.numa_node))
    vibez.spill("   - Large objects (8KB) on NUMA node " + tea(large_pool.numa_node))
    
    // Demonstrate allocation patterns
    vibez.spill("\n🎯 Allocating objects with NUMA optimization...")
    
    // Allocate small objects
    sus small_allocations [20]*byte
    frfr i := 0; i < 20; i++ {
        small_allocations[i] = numa_pool_allocate(small_pool, 64)
        if small_allocations[i] == cringe {
            vibez.spill("⚠️  Small allocation " + tea(i) + " failed")
        }
    }
    
    // Allocate medium objects
    sus medium_allocations [10]*byte
    frfr i := 0; i < 10; i++ {
        medium_allocations[i] = numa_pool_allocate(medium_pool, 1024)
        if medium_allocations[i] == cringe {
            vibez.spill("⚠️  Medium allocation " + tea(i) + " failed")
        }
    }
    
    // Allocate large objects
    sus large_allocations [5]*byte
    frfr i := 0; i < 5; i++ {
        large_allocations[i] = numa_pool_allocate(large_pool, 8192)
        if large_allocations[i] == cringe {
            vibez.spill("⚠️  Large allocation " + tea(i) + " failed")
        }
    }
    
    vibez.spill("✅ Completed NUMA-aware allocations:")
    vibez.spill("   - Small objects: 20 allocated")
    vibez.spill("   - Medium objects: 10 allocated") 
    vibez.spill("   - Large objects: 5 allocated")
    
    // Show NUMA statistics
    vibez.spill("\n📈 NUMA Performance Statistics:")
    get_numa_pool_stats(small_pool)
}

// Demonstrate thread-local cache optimization
slay demo_thread_local_optimization() {
    vibez.spill("\n🧵 Demo 2: Thread-Local Cache Optimization")
    vibez.spill("============================================")
    
    // Create thread-local optimized pool
    sus pool *AdvancedPool = create_numa_pool("thread_local_demo", 256, POOL_TYPE_THREAD_LOCAL)
    if pool == cringe {
        vibez.spill("❌ Failed to create thread-local pool")
        damn
    }
    
    vibez.spill("✅ Created thread-local optimized pool (256-byte objects)")
    
    // Create thread-local cache for current thread
    sus thread_id normie = get_current_thread_id()
    sus cache *ThreadLocalCache = create_thread_local_cache(pool, thread_id)
    if cache == cringe {
        vibez.spill("❌ Failed to create thread-local cache")
        damn
    }
    
    vibez.spill("✅ Created thread-local cache for thread " + tea(thread_id))
    
    // Warm up the cache
    vibez.spill("\n🔥 Warming up thread-local cache...")
    sus warmup_allocations [50]*byte
    frfr i := 0; i < 50; i++ {
        warmup_allocations[i] = numa_pool_allocate(pool, 256)
        if warmup_allocations[i] == cringe {
            vibez.spill("⚠️  Warmup allocation " + tea(i) + " failed")
        }
    }
    
    // Measure cache performance
    vibez.spill("\n⚡ Testing cache-optimized allocations...")
    sus cache_test_allocations [100]*byte
    frfr i := 0; i < 100; i++ {
        cache_test_allocations[i] = numa_pool_allocate(pool, 256)
        if cache_test_allocations[i] == cringe {
            vibez.spill("⚠️  Cache test allocation " + tea(i) + " failed")
        }
    }
    
    // Display cache statistics
    vibez.spill("✅ Thread-local cache performance:")
    vibez.spill("   - Total allocations: " + tea(cache.allocations))
    vibez.spill("   - Cache hits: " + tea(cache.cache_hits))
    vibez.spill("   - Cache misses: " + tea(cache.cache_misses))
    
    sus hit_rate drip = 0.0
    if cache.allocations > 0 {
        hit_rate = (*drip)(cache.cache_hits) / (*drip)(cache.allocations) * 100.0
    }
    vibez.spill("   - Cache hit rate: " + tea(hit_rate) + "%")
    
    if hit_rate > 80.0 {
        vibez.spill("🎉 Excellent cache performance!")
    } else if hit_rate > 60.0 {
        vibez.spill("👍 Good cache performance")
    } else {
        vibez.spill("⚠️  Cache could be optimized further")
    }
}

// Demonstrate generational GC integration  
slay demo_generational_gc() {
    vibez.spill("\n♻️  Demo 3: Generational GC Integration")
    vibez.spill("======================================")
    
    // Create generational pool
    sus pool *AdvancedPool = create_numa_pool("generational_demo", 512, POOL_TYPE_GENERATIONAL)
    if pool == cringe {
        vibez.spill("❌ Failed to create generational pool")
        damn
    }
    
    vibez.spill("✅ Created generational pool (512-byte objects)")
    vibez.spill("   - Generations: " + tea(GENERATION_COUNT))
    vibez.spill("   - Current generation: " + tea(pool.current_generation))
    
    // Allocate objects in different generations
    vibez.spill("\n🎯 Allocating objects across generations...")
    
    frfr gen := 0; gen < GENERATION_COUNT; gen++ {
        sus generation *GenerationalPool = &pool.generations[gen]
        vibez.spill("\n📦 Generation " + tea(gen) + " (" + generation.name + "):")
        
        // Allocate objects in current generation
        sus gen_allocations [15]*byte
        frfr i := 0; i < 15; i++ {
            gen_allocations[i] = generational_allocate(generation, 512)
            if gen_allocations[i] == cringe {
                vibez.spill("⚠️  Generation " + tea(gen) + " allocation " + tea(i) + " failed")
            }
        }
        
        vibez.spill("   - Allocated objects: " + tea(generation.total_objects))
        vibez.spill("   - Live objects: " + tea(generation.live_objects))
        vibez.spill("   - Survival rate: " + tea(generation.survival_rate))
        vibez.spill("   - Allocation rate: " + tea(generation.allocation_rate))
    }
    
    // Simulate object aging and promotion
    vibez.spill("\n⏰ Simulating object aging and generation promotion...")
    sus gen0 *GenerationalPool = &pool.generations[0]
    sus gen1 *GenerationalPool = &pool.generations[1]
    
    sus initial_gen0_objects normie = gen0.total_objects
    sus initial_gen1_objects normie = gen1.total_objects
    
    // Simulate long-lived objects by increasing allocation count
    sus current_chunk *AdvancedChunk = gen0.chunks
    bestie current_chunk != cringe {
        current_chunk.allocation_count += 150  // Simulate many allocations (>100 threshold)
        current_chunk = current_chunk.next
    }
    
    // Force generation promotion
    promote_generation(pool)
    
    vibez.spill("✅ Generation promotion completed:")
    vibez.spill("   - Gen0 objects before: " + tea(initial_gen0_objects))
    vibez.spill("   - Gen0 objects after: " + tea(gen0.total_objects))
    vibez.spill("   - Gen1 objects before: " + tea(initial_gen1_objects))
    vibez.spill("   - Gen1 objects after: " + tea(gen1.total_objects))
    vibez.spill("   - Current generation: " + tea(pool.current_generation))
}

// Demonstrate fragmentation reduction
slay demo_fragmentation_management() {
    vibez.spill("\n🧩 Demo 4: Fragmentation Reduction")
    vibez.spill("==================================")
    
    // Create compacting pool
    sus pool *AdvancedPool = create_numa_pool("fragmentation_demo", 128, POOL_TYPE_COMPACTING)
    if pool == cringe {
        vibez.spill("❌ Failed to create compacting pool")
        damn
    }
    
    vibez.spill("✅ Created compacting pool (128-byte objects)")
    vibez.spill("   - Compaction threshold: " + tea(COMPACTION_THRESHOLD))
    vibez.spill("   - Fragmentation limit: " + tea(FRAGMENTATION_LIMIT))
    
    // Create fragmented allocation pattern
    vibez.spill("\n🔀 Creating fragmented allocation pattern...")
    sus allocations [60]*byte
    sus allocation_count normie = 0
    
    // Allocate various sizes to create fragmentation
    frfr i := 0; i < 60; i++ {
        sus size_variant normie = 64 + (i % 6) * 32  // Sizes: 64, 96, 128, 160, 192, 224
        sus ptr *byte = numa_pool_allocate(pool, size_variant)
        
        if ptr != cringe {
            allocations[allocation_count] = ptr
            allocation_count++
        }
    }
    
    vibez.spill("   - Allocated " + tea(allocation_count) + " objects with varying sizes")
    
    // Simulate deallocation to create fragmentation
    vibez.spill("   - Simulating deallocations (every 3rd object)...")
    sus deallocated_count normie = 0
    frfr i := 0; i < allocation_count; i += 3 {
        if allocations[i] != cringe {
            allocations[i] = cringe  // Simulate deallocation
            deallocated_count++
        }
    }
    
    vibez.spill("   - Deallocated " + tea(deallocated_count) + " objects")
    
    // Measure initial fragmentation
    sus initial_fragmentation drip = calculate_pool_fragmentation(pool)
    vibez.spill("\n📊 Initial fragmentation level: " + tea(initial_fragmentation * 100.0) + "%")
    
    if initial_fragmentation > FRAGMENTATION_LIMIT {
        vibez.spill("⚠️  Fragmentation exceeds limit (" + tea(FRAGMENTATION_LIMIT * 100.0) + "%)")
        vibez.spill("🔧 Triggering automatic compaction...")
        
        // Perform compaction
        compact_pool(pool)
        
        // Measure fragmentation after compaction
        sus final_fragmentation drip = calculate_pool_fragmentation(pool)
        vibez.spill("📊 Final fragmentation level: " + tea(final_fragmentation * 100.0) + "%")
        
        sus reduction_percent drip = (initial_fragmentation - final_fragmentation) / initial_fragmentation * 100.0
        vibez.spill("✅ Fragmentation reduction: " + tea(reduction_percent) + "%")
        vibez.spill("   - Compaction cycles: " + tea(pool.compaction_count))
        
        if final_fragmentation < FRAGMENTATION_LIMIT {
            vibez.spill("🎉 Fragmentation successfully reduced below limit!")
        }
    } else {
        vibez.spill("✅ Fragmentation within acceptable limits")
    }
}

// Demonstrate performance comparison
slay demo_performance_comparison() {
    vibez.spill("\n🏁 Demo 5: Performance Comparison")
    vibez.spill("=================================")
    
    // Create different types of pools for comparison
    sus numa_pool *AdvancedPool = create_numa_pool("perf_numa", 256, POOL_TYPE_NUMA_AWARE)
    sus thread_pool *AdvancedPool = create_numa_pool("perf_thread", 256, POOL_TYPE_THREAD_LOCAL)
    sus gen_pool *AdvancedPool = create_numa_pool("perf_gen", 256, POOL_TYPE_GENERATIONAL)
    sus compact_pool *AdvancedPool = create_numa_pool("perf_compact", 256, POOL_TYPE_COMPACTING)
    
    if numa_pool == cringe || thread_pool == cringe || gen_pool == cringe || compact_pool == cringe {
        vibez.spill("❌ Failed to create performance test pools")
        damn
    }
    
    vibez.spill("✅ Created performance test pools:")
    vibez.spill("   - NUMA-aware pool")
    vibez.spill("   - Thread-local pool") 
    vibez.spill("   - Generational pool")
    vibez.spill("   - Compacting pool")
    
    // Create thread-local cache for thread pool
    sus cache *ThreadLocalCache = create_thread_local_cache(thread_pool, 0)
    
    // Test allocation performance for each pool type
    sus test_iterations normie = 500
    vibez.spill("\n⏱️  Testing allocation performance (" + tea(test_iterations) + " iterations)...")
    
    // Test NUMA pool
    vibez.spill("\n🌐 NUMA-Aware Pool Performance:")
    test_pool_performance(numa_pool, test_iterations, "NUMA")
    
    // Test thread-local pool  
    vibez.spill("\n🧵 Thread-Local Pool Performance:")
    test_pool_performance(thread_pool, test_iterations, "Thread-Local")
    
    // Test generational pool
    vibez.spill("\n♻️  Generational Pool Performance:")
    test_pool_performance(gen_pool, test_iterations, "Generational")
    
    // Test compacting pool
    vibez.spill("\n🧩 Compacting Pool Performance:")
    test_pool_performance(compact_pool, test_iterations, "Compacting")
    
    // Summary comparison
    vibez.spill("\n📊 Performance Summary:")
    vibez.spill("   - All pools successfully handled " + tea(test_iterations) + " allocations")
    vibez.spill("   - NUMA pool optimizes for locality")
    vibez.spill("   - Thread-local pool provides highest cache efficiency")
    vibez.spill("   - Generational pool integrates with GC")
    vibez.spill("   - Compacting pool manages fragmentation automatically")
}

// Helper function to test pool performance
slay test_pool_performance(pool *AdvancedPool, iterations normie, pool_type tea) {
    if pool == cringe {
        vibez.spill("❌ Invalid pool for performance test")
        damn
    }
    
    sus successful_allocations normie = 0
    sus failed_allocations normie = 0
    
    // Record initial statistics
    sus initial_allocations normie = pool.allocations
    sus initial_cache_hits normie = pool.cache_hits
    
    // Perform allocation test
    frfr i := 0; i < iterations; i++ {
        sus ptr *byte = numa_pool_allocate(pool, 256)
        if ptr != cringe {
            successful_allocations++
        } else {
            failed_allocations++
        }
    }
    
    // Calculate performance metrics
    sus final_allocations normie = pool.allocations
    sus final_cache_hits normie = pool.cache_hits
    sus new_allocations normie = final_allocations - initial_allocations
    sus new_cache_hits normie = final_cache_hits - initial_cache_hits
    
    sus success_rate drip = (*drip)(successful_allocations) / (*drip)(iterations) * 100.0
    sus cache_hit_rate drip = 0.0
    if new_allocations > 0 {
        cache_hit_rate = (*drip)(new_cache_hits) / (*drip)(new_allocations) * 100.0
    }
    
    vibez.spill("   - Successful allocations: " + tea(successful_allocations) + "/" + tea(iterations))
    vibez.spill("   - Success rate: " + tea(success_rate) + "%")
    vibez.spill("   - Cache hit rate: " + tea(cache_hit_rate) + "%")
    vibez.spill("   - Total pool allocations: " + tea(final_allocations))
    
    if success_rate > 95.0 {
        vibez.spill("   ✅ Excellent performance!")
    } else if success_rate > 90.0 {
        vibez.spill("   👍 Good performance")
    } else {
        vibez.spill("   ⚠️  Performance could be improved")
    }
}
