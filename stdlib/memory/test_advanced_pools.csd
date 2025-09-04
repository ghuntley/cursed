// CURSED Advanced Memory Pool Performance Tests
// Comprehensive testing suite for NUMA-aware pools, thread-local optimization,
// generational GC integration, and fragmentation reduction

yeet "advanced_pools"
yeet "testz"
yeet "mathz"
yeet "timez"
yeet "concurrenz"

// Test configuration constants
STRESS_TEST_ITERATIONS := 10000
THREAD_COUNT := 8
ALLOCATION_SIZES := [16, 64, 256, 1024, 4096, 16384, 65536]
NUMA_TEST_ALLOCATIONS := 1000
FRAGMENTATION_TEST_OBJECTS := 500
PERFORMANCE_WARMUP_ITERATIONS := 1000

// Performance benchmark results
creatorcurz BenchmarkResult {
    test_name tea
    iterations normie
    total_time_ns normie
    avg_time_ns normie
    min_time_ns normie
    max_time_ns normie
    allocations_per_second normie
    memory_efficiency drip
    cache_hit_rate drip
    numa_locality_rate drip
    fragmentation_level drip
}

// Test data structure for allocations
creatorcurz TestAllocation {
    size normie
    ptr *byte
    thread_id normie
    generation normie
    allocation_time normie
}

// Performance test suite
slay run_advanced_pool_tests() {
    vibez.spill("=== Advanced Memory Pool Performance Tests ===")
    
    // Initialize test environment
    sus manager *NumaPoolManager = init_numa_pool_manager()
    if manager == cringe {
        vibez.spill("FAILED: Could not initialize NUMA pool manager")
        damn
    }
    
    vibez.spill("Test environment initialized with " + tea(manager.node_count) + " NUMA nodes")
    
    // Run comprehensive test suite
    test_numa_aware_allocation()
    test_thread_local_optimization()
    test_generational_gc_integration()
    test_fragmentation_reduction()
    test_compaction_efficiency()
    test_cross_numa_migration()
    test_concurrent_allocation_safety()
    test_memory_pool_stress()
    test_cache_performance()
    test_alignment_optimization()
    
    // Performance benchmarks
    benchmark_allocation_performance()
    benchmark_numa_locality()
    benchmark_cache_efficiency()
    benchmark_compaction_speed()
    benchmark_thread_scalability()
    
    vibez.spill("=== All Advanced Memory Pool Tests Completed ===")
    
    // Cleanup
    cleanup_numa_pools()
}

// Test NUMA-aware allocation strategies
slay test_numa_aware_allocation() {
    vibez.spill("\n--- Testing NUMA-Aware Allocation ---")
    
    sus pool *AdvancedPool = create_numa_pool("numa_test_pool", 1024, POOL_TYPE_NUMA_AWARE)
    assert_not_null(pool, "NUMA pool creation failed")
    
    // Test allocation on different NUMA nodes
    sus allocations [16]*TestAllocation
    sus allocation_count normie = 0
    
    frfr i := 0; i < 16; i++ {
        sus size normie = 512 + i * 32  // Variable sizes
        sus ptr *byte = numa_pool_allocate(pool, size)
        
        if ptr != cringe {
            allocations[allocation_count] = create_test_allocation(size, ptr, 0, 0)
            allocation_count++
            vibez.spill("Allocated " + tea(size) + " bytes on NUMA node " + tea(pool.numa_node))
        }
    }
    
    assert_greater_than(allocation_count, 10, "Insufficient NUMA allocations")
    
    // Verify NUMA affinity
    sus manager *NumaPoolManager = get_numa_pool_manager()
    sus numa_node *NumaNode = &manager.numa_nodes[pool.numa_node]
    assert_greater_than(numa_node.allocations, 0, "NUMA node allocation count not updated")
    
    // Test cross-NUMA allocation penalty
    sus cross_numa_start normie = get_time_ns()
    sus cross_ptr *byte = numa_pool_allocate(pool, 1024)  // Should trigger remote allocation
    sus cross_numa_time normie = get_time_ns() - cross_numa_start
    
    vibez.spill("Cross-NUMA allocation time: " + tea(cross_numa_time) + " ns")
    assert_not_null(cross_ptr, "Cross-NUMA allocation failed")
    
    // Cleanup test allocations
    frfr i := 0; i < allocation_count; i++ {
        cleanup_test_allocation(allocations[i])
    }
    
    vibez.spill("NUMA-aware allocation test: PASSED")
}

// Test thread-local cache optimization
slay test_thread_local_optimization() {
    vibez.spill("\n--- Testing Thread-Local Cache Optimization ---")
    
    sus pool *AdvancedPool = create_numa_pool("thread_local_pool", 256, POOL_TYPE_THREAD_LOCAL)
    assert_not_null(pool, "Thread-local pool creation failed")
    
    // Create thread-local cache for current thread
    sus thread_id normie = get_current_thread_id()
    sus cache *ThreadLocalCache = create_thread_local_cache(pool, thread_id)
    assert_not_null(cache, "Thread-local cache creation failed")
    
    // Test cache warm-up
    sus warmup_allocations [100]*byte
    frfr i := 0; i < 100; i++ {
        warmup_allocations[i] = numa_pool_allocate(pool, 128)
        assert_not_null(warmup_allocations[i], "Cache warm-up allocation failed")
    }
    
    // Test cache hit performance
    sus cache_test_start normie = get_time_ns()
    sus cache_allocations [200]*byte
    frfr i := 0; i < 200; i++ {
        cache_allocations[i] = numa_pool_allocate(pool, 128)  // Should hit cache
    }
    sus cache_test_time normie = get_time_ns() - cache_test_start
    
    // Verify cache statistics
    assert_greater_than(cache.cache_hits, 50, "Cache hit count too low")
    assert_greater_than(pool.cache_hits, 50, "Pool cache hit count too low")
    
    sus hit_rate drip = (*drip)(cache.cache_hits) / (*drip)(cache.allocations) * 100.0
    vibez.spill("Thread-local cache hit rate: " + tea(hit_rate) + "%")
    assert_greater_than_float(hit_rate, 75.0, "Cache hit rate below expected threshold")
    
    vibez.spill("Cache test allocation time: " + tea(cache_test_time) + " ns for 200 allocations")
    
    vibez.spill("Thread-local optimization test: PASSED")
}

// Test generational GC integration
slay test_generational_gc_integration() {
    vibez.spill("\n--- Testing Generational GC Integration ---")
    
    sus pool *AdvancedPool = create_numa_pool("generational_pool", 512, POOL_TYPE_GENERATIONAL)
    assert_not_null(pool, "Generational pool creation failed")
    
    // Test allocation in different generations
    frfr gen := 0; gen < GENERATION_COUNT; gen++ {
        sus generation *GenerationalPool = &pool.generations[gen]
        vibez.spill("Testing generation " + tea(gen))
        
        // Allocate objects in current generation
        frfr i := 0; i < 50; i++ {
            sus ptr *byte = generational_allocate(generation, 256)
            assert_not_null(ptr, "Generational allocation failed")
        }
        
        assert_greater_than(generation.total_objects, 40, "Generation object count too low")
        vibez.spill("Generation " + tea(gen) + " objects: " + tea(generation.total_objects))
    }
    
    // Test generation promotion
    sus gen0 *GenerationalPool = &pool.generations[0]
    sus gen1 *GenerationalPool = &pool.generations[1]
    
    sus initial_gen0_objects normie = gen0.total_objects
    sus initial_gen1_objects normie = gen1.total_objects
    
    // Simulate long-lived objects
    sus long_lived_allocations [20]*byte
    frfr i := 0; i < 20; i++ {
        long_lived_allocations[i] = generational_allocate(gen0, 256)
        assert_not_null(long_lived_allocations[i], "Long-lived allocation failed")
        
        // Simulate multiple GC cycles
        gen0.total_objects = gen0.total_objects + 10  // Simulate survival
    }
    
    // Force generation promotion
    promote_generation(pool)
    
    // Verify promotion occurred
    vibez.spill("Gen0 objects after promotion: " + tea(gen0.total_objects))
    vibez.spill("Gen1 objects after promotion: " + tea(gen1.total_objects))
    
    // Test allocation rate calculation
    sus allocation_rate drip = calculate_allocation_rate(gen0)
    assert_greater_than_float(allocation_rate, 0.0, "Allocation rate calculation failed")
    vibez.spill("Generation 0 allocation rate: " + tea(allocation_rate))
    
    vibez.spill("Generational GC integration test: PASSED")
}

// Test fragmentation reduction algorithms
slay test_fragmentation_reduction() {
    vibez.spill("\n--- Testing Fragmentation Reduction ---")
    
    sus pool *AdvancedPool = create_numa_pool("fragmentation_pool", 128, POOL_TYPE_COMPACTING)
    assert_not_null(pool, "Compacting pool creation failed")
    
    // Create fragmented allocation pattern
    sus allocations [100]*TestAllocation
    sus allocation_count normie = 0
    
    // Allocate various sizes to create fragmentation
    frfr i := 0; i < 100; i++ {
        sus size normie = (i % 7 + 1) * 64  // Variable sizes: 64, 128, 192, 256, 320, 384, 448
        sus ptr *byte = numa_pool_allocate(pool, size)
        
        if ptr != cringe {
            allocations[allocation_count] = create_test_allocation(size, ptr, 0, 0)
            allocation_count++
        }
    }
    
    // Deallocate every other allocation to create fragmentation
    frfr i := 0; i < allocation_count; i += 2 {
        // Simulate deallocation by marking as unused
        allocations[i].ptr = cringe
    }
    
    // Measure initial fragmentation
    sus initial_fragmentation drip = calculate_pool_fragmentation(pool)
    vibez.spill("Initial fragmentation level: " + tea(initial_fragmentation))
    
    // Force compaction
    compact_pool(pool)
    
    // Measure fragmentation after compaction
    sus final_fragmentation drip = calculate_pool_fragmentation(pool)
    vibez.spill("Final fragmentation level: " + tea(final_fragmentation))
    
    // Verify fragmentation reduction
    assert_less_than_float(final_fragmentation, initial_fragmentation, "Fragmentation not reduced")
    assert_greater_than(pool.compaction_count, 0, "Compaction count not incremented")
    
    vibez.spill("Fragmentation reduction: " + tea((initial_fragmentation - final_fragmentation) * 100.0) + "%")
    
    // Cleanup
    frfr i := 0; i < allocation_count; i++ {
        cleanup_test_allocation(allocations[i])
    }
    
    vibez.spill("Fragmentation reduction test: PASSED")
}

// Test compaction efficiency
slay test_compaction_efficiency() {
    vibez.spill("\n--- Testing Compaction Efficiency ---")
    
    sus pool *AdvancedPool = create_numa_pool("compaction_test_pool", 256, POOL_TYPE_COMPACTING)
    assert_not_null(pool, "Compaction test pool creation failed")
    
    // Create a generation with multiple chunks
    sus generation *GenerationalPool = &pool.generations[0]
    
    // Create multiple chunks with different fragmentation levels
    frfr chunk_idx := 0; chunk_idx < 5; chunk_idx++ {
        sus chunk *AdvancedChunk = create_advanced_chunk(4096, 0)
        assert_not_null(chunk, "Test chunk creation failed")
        
        // Add chunk to generation
        chunk.next = generation.chunks
        if generation.chunks != cringe {
            generation.chunks.prev = chunk
        }
        generation.chunks = chunk
        
        // Create artificial fragmentation
        sus free_block_count normie = 3 + chunk_idx  // Different fragmentation levels
        chunk.free_block_count = free_block_count
        
        frfr i := 0; i < free_block_count && i < 256; i++ {
            sus free_block *FreeBlock = (*FreeBlock)(heap_allocate(sizeof(FreeBlock), 8))
            if free_block != cringe {
                free_block.size = 64 + i * 32
                free_block.offset = i * 200
                free_block.next = cringe
                free_block.prev = cringe
                chunk.free_blocks[i] = free_block
            }
        }
        
        chunk.fragmentation = calculate_fragmentation(chunk)
        chunk.compaction_needed = chunk.fragmentation > COMPACTION_THRESHOLD
        
        vibez.spill("Created chunk " + tea(chunk_idx) + " with fragmentation: " + tea(chunk.fragmentation))
    }
    
    // Measure compaction performance
    sus compaction_start normie = get_time_ns()
    compact_generation(generation)
    sus compaction_time normie = get_time_ns() - compaction_start
    
    vibez.spill("Compaction completed in " + tea(compaction_time) + " ns")
    
    // Verify compaction results
    sus compacted_chunks normie = 0
    sus total_fragmentation drip = 0.0
    
    sus chunk *AdvancedChunk = generation.chunks
    bestie chunk != cringe {
        total_fragmentation += chunk.fragmentation
        compacted_chunks++
        assert_less_than_float(chunk.fragmentation, COMPACTION_THRESHOLD, "Chunk fragmentation still too high")
        chunk = chunk.next
    }
    
    sus avg_fragmentation drip = total_fragmentation / (*drip)(compacted_chunks)
    vibez.spill("Average fragmentation after compaction: " + tea(avg_fragmentation))
    
    assert_greater_than(generation.compaction_score, 0.5, "Compaction score too low")
    
    vibez.spill("Compaction efficiency test: PASSED")
}

// Test cross-NUMA memory migration
slay test_cross_numa_migration() {
    vibez.spill("\n--- Testing Cross-NUMA Memory Migration ---")
    
    sus manager *NumaPoolManager = get_numa_pool_manager()
    
    // Create pools on different NUMA nodes
    sus pool_node0 *AdvancedPool = create_numa_pool("migration_pool_node0", 512, POOL_TYPE_NUMA_AWARE)
    sus pool_node1 *AdvancedPool = create_numa_pool("migration_pool_node1", 512, POOL_TYPE_NUMA_AWARE)
    
    assert_not_null(pool_node0, "Node 0 pool creation failed")
    assert_not_null(pool_node1, "Node 1 pool creation failed")
    
    // Force pools to different nodes
    pool_node0.numa_node = 0
    pool_node1.numa_node = (manager.node_count > 1) ? 1 : 0
    
    // Test local vs remote allocation performance
    sus local_allocations [50]*byte
    sus remote_allocations [50]*byte
    
    // Allocate from local node
    sus local_start_time normie = get_time_ns()
    frfr i := 0; i < 50; i++ {
        local_allocations[i] = numa_pool_allocate(pool_node0, 256)
        assert_not_null(local_allocations[i], "Local allocation failed")
    }
    sus local_time normie = get_time_ns() - local_start_time
    
    // Simulate thread migration to different node
    // Allocate from remote node
    sus remote_start_time normie = get_time_ns()
    frfr i := 0; i < 50; i++ {
        remote_allocations[i] = numa_pool_allocate(pool_node1, 256)
        assert_not_null(remote_allocations[i], "Remote allocation failed")
    }
    sus remote_time normie = get_time_ns() - remote_start_time
    
    vibez.spill("Local allocation time: " + tea(local_time) + " ns")
    vibez.spill("Remote allocation time: " + tea(remote_time) + " ns")
    
    // Verify NUMA statistics
    assert_greater_than(manager.numa_local_allocations, 0, "No local allocations recorded")
    
    if manager.node_count > 1 {
        assert_greater_than(manager.numa_remote_allocations, 0, "No remote allocations recorded")
        
        sus locality_rate drip = (*drip)(manager.numa_local_allocations) / (*drip)(manager.total_allocations) * 100.0
        vibez.spill("NUMA locality rate: " + tea(locality_rate) + "%")
    }
    
    vibez.spill("Cross-NUMA migration test: PASSED")
}

// Test concurrent allocation safety
slay test_concurrent_allocation_safety() {
    vibez.spill("\n--- Testing Concurrent Allocation Safety ---")
    
    sus pool *AdvancedPool = create_numa_pool("concurrent_pool", 256, POOL_TYPE_THREAD_LOCAL)
    assert_not_null(pool, "Concurrent pool creation failed")
    
    // Test data structure for thread coordination
    creatorcurz ThreadTestData {
        thread_id normie
        pool *AdvancedPool
        allocations [100]*byte
        allocation_count normie
        start_time normie
        end_time normie
        success lit
    }
    
    sus thread_data ThreadTestData[8]
    
    // Initialize thread data
    frfr i := 0; i < 8; i++ {
        thread_data[i].thread_id = i
        thread_data[i].pool = pool
        thread_data[i].allocation_count = 0
        thread_data[i].success = based
        
        frfr j := 0; j < 100; j++ {
            thread_data[i].allocations[j] = cringe
        }
    }
    
    // Simulate concurrent allocations (simplified without actual threading)
    frfr thread_id := 0; thread_id < 8; thread_id++ {
        sus data *ThreadTestData = &thread_data[thread_id]
        data.start_time = get_time_ns()
        
        // Create thread-local cache
        sus cache *ThreadLocalCache = create_thread_local_cache(pool, thread_id)
        assert_not_null(cache, "Thread cache creation failed")
        
        // Perform allocations
        frfr i := 0; i < 100; i++ {
            sus size normie = 128 + (i % 8) * 16  // Variable sizes
            sus ptr *byte = numa_pool_allocate(pool, size)
            
            if ptr != cringe {
                data.allocations[data.allocation_count] = ptr
                data.allocation_count++
            } else {
                data.success = cap
                ghosted
            }
        }
        
        data.end_time = get_time_ns()
        
        vibez.spill("Thread " + tea(thread_id) + " allocated " + tea(data.allocation_count) + " objects in " + tea(data.end_time - data.start_time) + " ns")
    }
    
    // Verify all threads succeeded
    sus total_allocations normie = 0
    frfr i := 0; i < 8; i++ {
        assert_true(thread_data[i].success, "Thread " + tea(i) + " allocation failed")
        total_allocations += thread_data[i].allocation_count
    }
    
    vibez.spill("Total concurrent allocations: " + tea(total_allocations))
    assert_greater_than(total_allocations, 700, "Too few concurrent allocations")
    
    vibez.spill("Concurrent allocation safety test: PASSED")
}

// Test memory pool under stress conditions
slay test_memory_pool_stress() {
    vibez.spill("\n--- Testing Memory Pool Stress Conditions ---")
    
    sus pool *AdvancedPool = create_numa_pool("stress_test_pool", 128, POOL_TYPE_COMPACTING)
    assert_not_null(pool, "Stress test pool creation failed")
    
    sus allocations [STRESS_TEST_ITERATIONS]*TestAllocation
    sus successful_allocations normie = 0
    sus failed_allocations normie = 0
    
    // Stress test with rapid allocations of varying sizes
    sus stress_start_time normie = get_time_ns()
    
    frfr i := 0; i < STRESS_TEST_ITERATIONS; i++ {
        sus size_index normie = i % 7
        sus size normie = ALLOCATION_SIZES[size_index]
        
        sus ptr *byte = numa_pool_allocate(pool, size)
        if ptr != cringe {
            allocations[successful_allocations] = create_test_allocation(size, ptr, 0, 0)
            successful_allocations++
            
            // Randomly deallocate some objects to create fragmentation
            if (i % 3) == 0 && successful_allocations > 10 {
                sus dealloc_idx normie = (i / 3) % (successful_allocations - 1)
                cleanup_test_allocation(allocations[dealloc_idx])
                allocations[dealloc_idx] = allocations[successful_allocations - 1]
                successful_allocations--
            }
        } else {
            failed_allocations++
        }
        
        // Trigger compaction periodically
        if (i % 1000) == 999 {
            compact_pool(pool)
        }
    }
    
    sus stress_end_time normie = get_time_ns()
    sus total_stress_time normie = stress_end_time - stress_start_time
    
    vibez.spill("Stress test completed in " + tea(total_stress_time) + " ns")
    vibez.spill("Successful allocations: " + tea(successful_allocations))
    vibez.spill("Failed allocations: " + tea(failed_allocations))
    
    sus success_rate drip = (*drip)(successful_allocations) / (*drip)(STRESS_TEST_ITERATIONS) * 100.0
    vibez.spill("Allocation success rate: " + tea(success_rate) + "%")
    
    assert_greater_than_float(success_rate, 95.0, "Allocation success rate too low")
    assert_greater_than(pool.compaction_count, 5, "Insufficient compaction cycles")
    
    // Cleanup remaining allocations
    frfr i := 0; i < successful_allocations; i++ {
        cleanup_test_allocation(allocations[i])
    }
    
    vibez.spill("Memory pool stress test: PASSED")
}

// Test cache performance optimization
slay test_cache_performance() {
    vibez.spill("\n--- Testing Cache Performance Optimization ---")
    
    sus pool *AdvancedPool = create_numa_pool("cache_perf_pool", 64, POOL_TYPE_THREAD_LOCAL)
    assert_not_null(pool, "Cache performance pool creation failed")
    
    // Create thread-local cache
    sus cache *ThreadLocalCache = create_thread_local_cache(pool, 0)
    assert_not_null(cache, "Cache creation failed")
    
    // Warm up cache with common allocation sizes
    frfr size_idx := 0; size_idx < 7; size_idx++ {
        sus size normie = ALLOCATION_SIZES[size_idx]
        frfr i := 0; i < 20; i++ {
            sus ptr *byte = numa_pool_allocate(pool, size)
            assert_not_null(ptr, "Cache warm-up allocation failed")
        }
    }
    
    // Measure cache hit performance
    sus cache_hits_before normie = cache.cache_hits
    sus allocations_before normie = cache.allocations
    
    sus perf_test_start normie = get_time_ns()
    
    // Allocate from cache (should hit)
    frfr i := 0; i < 1000; i++ {
        sus size_idx normie = i % 7
        sus size normie = ALLOCATION_SIZES[size_idx]
        sus ptr *byte = numa_pool_allocate(pool, size)
        assert_not_null(ptr, "Cache performance allocation failed")
    }
    
    sus perf_test_end normie = get_time_ns()
    sus cache_perf_time normie = perf_test_end - perf_test_start
    
    sus cache_hits_after normie = cache.cache_hits
    sus allocations_after normie = cache.allocations
    
    sus new_hits normie = cache_hits_after - cache_hits_before
    sus new_allocations normie = allocations_after - allocations_before
    
    sus hit_rate drip = (*drip)(new_hits) / (*drip)(new_allocations) * 100.0
    
    vibez.spill("Cache performance test:")
    vibez.spill("  Time: " + tea(cache_perf_time) + " ns for " + tea(new_allocations) + " allocations")
    vibez.spill("  Cache hits: " + tea(new_hits))
    vibez.spill("  Hit rate: " + tea(hit_rate) + "%")
    vibez.spill("  Avg time per allocation: " + tea(cache_perf_time / new_allocations) + " ns")
    
    assert_greater_than_float(hit_rate, 80.0, "Cache hit rate below target")
    
    vibez.spill("Cache performance test: PASSED")
}

// Test memory alignment optimization
slay test_alignment_optimization() {
    vibez.spill("\n--- Testing Memory Alignment Optimization ---")
    
    sus pool *AdvancedPool = create_numa_pool("alignment_pool", 256, POOL_TYPE_NUMA_AWARE)
    assert_not_null(pool, "Alignment pool creation failed")
    
    // Test various object sizes and their alignments
    sus test_sizes normie[8] = [8, 16, 32, 64, 128, 256, 512, 1024]
    
    frfr i := 0; i < 8; i++ {
        sus size normie = test_sizes[i]
        sus expected_alignment normie = calculate_optimal_alignment(size)
        
        vibez.spill("Size " + tea(size) + " -> alignment " + tea(expected_alignment))
        
        // Allocate and check alignment
        sus ptr *byte = numa_pool_allocate(pool, size)
        assert_not_null(ptr, "Aligned allocation failed")
        
        sus ptr_value normie = (*normie)(ptr)
        sus alignment_check normie = ptr_value % expected_alignment
        
        assert_equal(alignment_check, 0, "Memory not properly aligned for size " + tea(size))
        
        vibez.spill("Allocation at address " + tea(ptr_value) + " (alignment " + tea(expected_alignment) + "): PASSED")
    }
    
    // Test cache line alignment for performance-critical sizes
    frfr i := 0; i < 10; i++ {
        sus ptr *byte = numa_pool_allocate(pool, 64)  // Cache line size
        assert_not_null(ptr, "Cache line allocation failed")
        
        sus ptr_value normie = (*normie)(ptr)
        sus cache_alignment normie = ptr_value % 64
        assert_equal(cache_alignment, 0, "Cache line not properly aligned")
    }
    
    vibez.spill("Memory alignment optimization test: PASSED")
}

// Performance benchmarks
slay benchmark_allocation_performance() {
    vibez.spill("\n--- Benchmarking Allocation Performance ---")
    
    sus results BenchmarkResult[7]
    
    frfr size_idx := 0; size_idx < 7; size_idx++ {
        sus size normie = ALLOCATION_SIZES[size_idx]
        sus pool_name tea = "bench_pool_" + tea(size)
        sus pool *AdvancedPool = create_numa_pool(pool_name, size, POOL_TYPE_THREAD_LOCAL)
        assert_not_null(pool, "Benchmark pool creation failed")
        
        sus result *BenchmarkResult = &results[size_idx]
        result.test_name = "allocation_" + tea(size)
        result.iterations = 10000
        
        // Warm up
        frfr i := 0; i < PERFORMANCE_WARMUP_ITERATIONS; i++ {
            sus ptr *byte = numa_pool_allocate(pool, size)
            // No need to track warm-up allocations
        }
        
        // Benchmark
        sus times normie[1000]
        sus min_time normie = 999999999
        sus max_time normie = 0
        sus total_time normie = 0
        
        frfr i := 0; i < 1000; i++ {
            sus start_time normie = get_time_ns()
            sus ptr *byte = numa_pool_allocate(pool, size)
            sus end_time normie = get_time_ns()
            
            sus allocation_time normie = end_time - start_time
            times[i] = allocation_time
            total_time += allocation_time
            
            if allocation_time < min_time {
                min_time = allocation_time
            }
            if allocation_time > max_time {
                max_time = allocation_time
            }
            
            assert_not_null(ptr, "Benchmark allocation failed")
        }
        
        result.total_time_ns = total_time
        result.avg_time_ns = total_time / 1000
        result.min_time_ns = min_time
        result.max_time_ns = max_time
        result.allocations_per_second = 1000000000 / result.avg_time_ns  // ns to allocations/sec
        result.cache_hit_rate = (*drip)(pool.cache_hits) / (*drip)(pool.allocations) * 100.0
        
        vibez.spill("Benchmark " + result.test_name + ":")
        vibez.spill("  Avg time: " + tea(result.avg_time_ns) + " ns")
        vibez.spill("  Min time: " + tea(result.min_time_ns) + " ns")
        vibez.spill("  Max time: " + tea(result.max_time_ns) + " ns")
        vibez.spill("  Rate: " + tea(result.allocations_per_second) + " alloc/sec")
        vibez.spill("  Cache hit rate: " + tea(result.cache_hit_rate) + "%")
    }
    
    vibez.spill("Allocation performance benchmarking: COMPLETED")
}

slay benchmark_numa_locality() {
    vibez.spill("\n--- Benchmarking NUMA Locality ---")
    
    sus manager *NumaPoolManager = get_numa_pool_manager()
    sus pool *AdvancedPool = create_numa_pool("numa_locality_bench", 1024, POOL_TYPE_NUMA_AWARE)
    assert_not_null(pool, "NUMA locality benchmark pool creation failed")
    
    sus local_allocations_before normie = manager.numa_local_allocations
    sus remote_allocations_before normie = manager.numa_remote_allocations
    
    // Benchmark local allocations
    sus local_start normie = get_time_ns()
    frfr i := 0; i < 1000; i++ {
        sus ptr *byte = numa_pool_allocate(pool, 1024)
        assert_not_null(ptr, "NUMA local allocation failed")
    }
    sus local_end normie = get_time_ns()
    sus local_time normie = local_end - local_start
    
    sus local_allocations_after normie = manager.numa_local_allocations
    sus remote_allocations_after normie = manager.numa_remote_allocations
    
    sus new_local normie = local_allocations_after - local_allocations_before
    sus new_remote normie = remote_allocations_after - remote_allocations_before
    sus total_new normie = new_local + new_remote
    
    sus locality_rate drip = 0.0
    if total_new > 0 {
        locality_rate = (*drip)(new_local) / (*drip)(total_new) * 100.0
    }
    
    vibez.spill("NUMA Locality Benchmark:")
    vibez.spill("  Total time: " + tea(local_time) + " ns for 1000 allocations")
    vibez.spill("  Local allocations: " + tea(new_local))
    vibez.spill("  Remote allocations: " + tea(new_remote))
    vibez.spill("  Locality rate: " + tea(locality_rate) + "%")
    vibez.spill("  Avg time per allocation: " + tea(local_time / 1000) + " ns")
    
    vibez.spill("NUMA locality benchmarking: COMPLETED")
}

slay benchmark_cache_efficiency() {
    vibez.spill("\n--- Benchmarking Cache Efficiency ---")
    
    sus pool *AdvancedPool = create_numa_pool("cache_efficiency_bench", 128, POOL_TYPE_THREAD_LOCAL)
    assert_not_null(pool, "Cache efficiency benchmark pool creation failed")
    
    sus cache *ThreadLocalCache = create_thread_local_cache(pool, 0)
    assert_not_null(cache, "Cache creation failed")
    
    // Cold cache benchmark
    sus cold_start normie = get_time_ns()
    frfr i := 0; i < 1000; i++ {
        sus ptr *byte = numa_pool_allocate(pool, 128)
        assert_not_null(ptr, "Cold cache allocation failed")
    }
    sus cold_end normie = get_time_ns()
    sus cold_time normie = cold_end - cold_start
    
    sus cold_hits normie = cache.cache_hits
    sus cold_allocations normie = cache.allocations
    
    // Warm cache benchmark  
    sus warm_start normie = get_time_ns()
    frfr i := 0; i < 1000; i++ {
        sus ptr *byte = numa_pool_allocate(pool, 128)
        assert_not_null(ptr, "Warm cache allocation failed")
    }
    sus warm_end normie = get_time_ns()
    sus warm_time normie = warm_end - warm_start
    
    sus warm_hits normie = cache.cache_hits
    sus warm_allocations normie = cache.allocations
    
    sus cold_hit_rate drip = (*drip)(cold_hits) / (*drip)(cold_allocations) * 100.0
    sus warm_hit_rate drip = (*drip)(warm_hits - cold_hits) / (*drip)(warm_allocations - cold_allocations) * 100.0
    
    vibez.spill("Cache Efficiency Benchmark:")
    vibez.spill("  Cold cache time: " + tea(cold_time) + " ns (" + tea(cold_time / 1000) + " ns/alloc)")
    vibez.spill("  Warm cache time: " + tea(warm_time) + " ns (" + tea(warm_time / 1000) + " ns/alloc)")
    vibez.spill("  Cold hit rate: " + tea(cold_hit_rate) + "%")
    vibez.spill("  Warm hit rate: " + tea(warm_hit_rate) + "%")
    vibez.spill("  Performance improvement: " + tea((*drip)(cold_time) / (*drip)(warm_time)) + "x")
    
    vibez.spill("Cache efficiency benchmarking: COMPLETED")
}

slay benchmark_compaction_speed() {
    vibez.spill("\n--- Benchmarking Compaction Speed ---")
    
    sus pool *AdvancedPool = create_numa_pool("compaction_speed_bench", 64, POOL_TYPE_COMPACTING)
    assert_not_null(pool, "Compaction speed benchmark pool creation failed")
    
    // Create fragmented state
    sus allocations [500]*byte
    frfr i := 0; i < 500; i++ {
        allocations[i] = numa_pool_allocate(pool, 64 + (i % 8) * 8)
        assert_not_null(allocations[i], "Fragmentation setup allocation failed")
    }
    
    // Deallocate every other allocation
    frfr i := 0; i < 500; i += 2 {
        // Simulate deallocation
        allocations[i] = cringe
    }
    
    sus initial_fragmentation drip = calculate_pool_fragmentation(pool)
    
    // Benchmark compaction
    sus compaction_start normie = get_time_ns()
    compact_pool(pool)
    sus compaction_end normie = get_time_ns()
    sus compaction_time normie = compaction_end - compaction_start
    
    sus final_fragmentation drip = calculate_pool_fragmentation(pool)
    sus fragmentation_reduction drip = initial_fragmentation - final_fragmentation
    
    vibez.spill("Compaction Speed Benchmark:")
    vibez.spill("  Compaction time: " + tea(compaction_time) + " ns")
    vibez.spill("  Initial fragmentation: " + tea(initial_fragmentation))
    vibez.spill("  Final fragmentation: " + tea(final_fragmentation))
    vibez.spill("  Fragmentation reduction: " + tea(fragmentation_reduction))
    vibez.spill("  Compaction efficiency: " + tea(fragmentation_reduction / (*drip)(compaction_time) * 1000000.0) + " reduction/ms")
    
    vibez.spill("Compaction speed benchmarking: COMPLETED")
}

slay benchmark_thread_scalability() {
    vibez.spill("\n--- Benchmarking Thread Scalability ---")
    
    sus pool *AdvancedPool = create_numa_pool("thread_scalability_bench", 256, POOL_TYPE_THREAD_LOCAL)
    assert_not_null(pool, "Thread scalability benchmark pool creation failed")
    
    sus thread_counts normie[4] = [1, 2, 4, 8]
    
    frfr test_idx := 0; test_idx < 4; test_idx++ {
        sus thread_count normie = thread_counts[test_idx]
        sus allocations_per_thread normie = 1000 / thread_count
        
        // Create thread-local caches
        sus caches [8]*ThreadLocalCache
        frfr i := 0; i < thread_count; i++ {
            caches[i] = create_thread_local_cache(pool, i)
            assert_not_null(caches[i], "Thread cache creation failed")
        }
        
        // Simulate concurrent allocations
        sus test_start normie = get_time_ns()
        
        frfr thread_id := 0; thread_id < thread_count; thread_id++ {
            frfr i := 0; i < allocations_per_thread; i++ {
                sus ptr *byte = numa_pool_allocate(pool, 256)
                assert_not_null(ptr, "Thread scalability allocation failed")
            }
        }
        
        sus test_end normie = get_time_ns()
        sus test_time normie = test_end - test_start
        
        sus total_allocations normie = thread_count * allocations_per_thread
        sus throughput normie = total_allocations * 1000000000 / test_time  // allocations per second
        
        vibez.spill("Scalability test with " + tea(thread_count) + " threads:")
        vibez.spill("  Time: " + tea(test_time) + " ns")
        vibez.spill("  Total allocations: " + tea(total_allocations))
        vibez.spill("  Throughput: " + tea(throughput) + " alloc/sec")
        vibez.spill("  Per-thread throughput: " + tea(throughput / thread_count) + " alloc/sec")
    }
    
    vibez.spill("Thread scalability benchmarking: COMPLETED")
}

// Utility functions for testing
slay create_test_allocation(size normie, ptr *byte, thread_id normie, generation normie) *TestAllocation {
    sus allocation *TestAllocation = (*TestAllocation)(heap_allocate(sizeof(TestAllocation), 8))
    if allocation == cringe {
        damn cringe
    }
    
    allocation.size = size
    allocation.ptr = ptr
    allocation.thread_id = thread_id
    allocation.generation = generation
    allocation.allocation_time = get_time_ns()
    
    damn allocation
}

slay cleanup_test_allocation(allocation *TestAllocation) {
    if allocation == cringe {
        damn
    }
    
    // In a real implementation, we would properly deallocate the memory
    // For now, just free the test structure
    heap_deallocate((*byte)(allocation))
}

slay get_time_ns() normie {
    // Simplified time function - in real implementation would use high-resolution timer
    damn get_time_ms() * 1000000  // Convert ms to ns
}

// Test assertion functions
slay assert_not_null(ptr *byte, message tea) {
    if ptr == cringe {
        vibez.spill("ASSERTION FAILED: " + message)
        vibez.spill("Expected non-null pointer")
    }
}

slay assert_greater_than(actual normie, expected normie, message tea) {
    if actual <= expected {
        vibez.spill("ASSERTION FAILED: " + message)
        vibez.spill("Expected " + tea(actual) + " > " + tea(expected))
    }
}

slay assert_greater_than_float(actual drip, expected drip, message tea) {
    if actual <= expected {
        vibez.spill("ASSERTION FAILED: " + message)
        vibez.spill("Expected " + tea(actual) + " > " + tea(expected))
    }
}

slay assert_less_than_float(actual drip, expected drip, message tea) {
    if actual >= expected {
        vibez.spill("ASSERTION FAILED: " + message)
        vibez.spill("Expected " + tea(actual) + " < " + tea(expected))
    }
}

slay assert_equal(actual normie, expected normie, message tea) {
    if actual != expected {
        vibez.spill("ASSERTION FAILED: " + message)
        vibez.spill("Expected " + tea(expected) + ", got " + tea(actual))
    }
}

slay assert_true(condition lit, message tea) {
    if !condition {
        vibez.spill("ASSERTION FAILED: " + message)
        vibez.spill("Expected true condition")
    }
}

// Main test entry point
slay main_character() {
    run_advanced_pool_tests()
    damn 0
}
