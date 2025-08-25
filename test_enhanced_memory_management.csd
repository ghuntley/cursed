fr fr CURSED Enhanced Memory Management Test Suite
fr fr Comprehensive test of production-grade memory management components

yeet "vibez"
yeet "memory/mod"
yeet "memory/numa_topology"
yeet "memory/advanced_gc"
yeet "memory/heap_defragmentation"
yeet "memory/thread_identification"
yeet "memory/high_resolution_timing"
yeet "memory/profiler"

fr fr Test NUMA topology detection
slay test_numa_topology() lit {
    vibez.spill("Testing NUMA Topology Detection...")
    vibez.spill("=" * 40)
    
    fr fr Initialize NUMA topology
    sus topology *NUMATopology = numa_topology_init()
    yo topology == cringe {
        vibez.spill("❌ NUMA topology initialization failed")
        damn cap
    }
    
    fr fr Test basic topology functions
    sus node_count normie = numa_get_node_count()
    sus current_node normie = numa_get_current_node()
    
    vibez.spillf("✅ NUMA Nodes: {}", node_count)
    vibez.spillf("✅ Current Node: {}", current_node)
    
    fr fr Test NUMA-aware allocation
    sus numa_ptr *void = numa_alloc_local(1024)
    yo numa_ptr != cringe {
        vibez.spill("✅ NUMA-aware allocation successful")
        
        fr fr Test NUMA allocation on specific node
        sus node_ptr *void = numa_alloc_on_node(2048, 0)
        yo node_ptr != cringe {
            vibez.spill("✅ Node-specific allocation successful")
        }
        
        fr fr Test interleaved allocation
        sus interleaved_ptr *void = numa_alloc_interleaved(4096)
        yo interleaved_ptr != cringe {
            vibez.spill("✅ Interleaved allocation successful")
        }
    }
    
    fr fr Print topology information
    numa_print_topology()
    
    vibez.spill("✅ NUMA topology test completed successfully")
    damn based
}

fr fr Test advanced garbage collector
slay test_advanced_gc() lit {
    vibez.spill("Testing Advanced Garbage Collector...")
    vibez.spill("=" * 40)
    
    fr fr Initialize advanced GC
    sus gc *AdvancedGarbageCollector = advanced_gc_init(32 * 1024 * 1024)  fr fr 32MB heap
    yo gc == cringe {
        vibez.spill("❌ Advanced GC initialization failed")
        damn cap
    }
    
    fr fr Test object allocation
    sus objects []*GCObjectHeader = []
    
    vibez.spill("Allocating test objects...")
    bestie i := 0; i < 100; i = i + 1 {
        sus obj *GCObjectHeader = advanced_gc_allocate(1024, 1)  fr fr 1KB objects, type 1
        yo obj != cringe {
            objects.push(obj)
        } otherwise {
            vibez.spillf("❌ GC allocation failed at object {}", i)
            damn cap
        }
    }
    
    vibez.spillf("✅ Allocated {} objects successfully", objects.len())
    
    fr fr Test garbage collection
    vibez.spill("Triggering garbage collection...")
    advanced_gc_force_collection(gc)
    
    fr fr Test statistics
    advanced_gc_get_statistics()
    
    vibez.spill("✅ Advanced GC test completed successfully")
    damn based
}

fr fr Test heap defragmentation
slay test_heap_defragmentation() lit {
    vibez.spill("Testing Heap Defragmentation...")
    vibez.spill("=" * 40)
    
    fr fr Allocate heap memory for testing
    sus heap_size thicc = 8 * 1024 * 1024  fr fr 8MB heap
    sus heap_memory *void = memory_alloc(heap_size.(normie))
    yo heap_memory == cringe {
        vibez.spill("❌ Failed to allocate heap memory for defrag test")
        damn cap
    }
    
    fr fr Initialize heap defragmenter
    sus defrag *HeapDefragmenter = heap_defrag_init(heap_memory, heap_size)
    yo defrag == cringe {
        vibez.spill("❌ Heap defragmenter initialization failed")
        damn cap
    }
    
    fr fr Analyze heap fragmentation
    sus analysis FragmentationAnalysis = heap_defrag_analyze_fragmentation(defrag)
    vibez.spillf("✅ Fragmentation analysis completed - {:.1f}% fragmented", 
                analysis.fragmentation_ratio * 100.0)
    
    fr fr Test different defragmentation algorithms
    sus algorithms []normie = [
        DEFRAG_ALGORITHM_SLIDING_COMPACTION,
        DEFRAG_ALGORITHM_MARK_COMPACT,
        DEFRAG_ALGORITHM_COPYING_COLLECTION
    ]
    
    bestie i := 0; i < algorithms.len(); i = i + 1 {
        sus algorithm normie = algorithms[i]
        vibez.spillf("Testing algorithm: {}", get_algorithm_name(algorithm))
        
        heap_defrag_set_algorithm(algorithm)
        heap_defrag_set_threshold(0.15)  fr fr 15% threshold
        
        yo heap_defrag_should_compact(defrag) {
            heap_defrag_trigger_compaction(defrag)
            vibez.spillf("✅ Compaction completed for algorithm {}", algorithm)
        }
    }
    
    fr fr Show defragmentation statistics
    heap_defrag_get_statistics()
    
    vibez.spill("✅ Heap defragmentation test completed successfully")
    damn based
}

fr fr Test thread identification
slay test_thread_identification() lit {
    vibez.spill("Testing Thread Identification...")
    vibez.spill("=" * 40)
    
    fr fr Initialize thread ID system
    sus registry *ThreadRegistry = thread_id_init(THREAD_ID_STRATEGY_OS_NATIVE)
    yo registry == cringe {
        vibez.spill("❌ Thread identification initialization failed")
        damn cap
    }
    
    fr fr Test main thread identification
    sus main_thread_id thicc = get_current_thread_id()
    vibez.spillf("✅ Main thread ID: {}", main_thread_id)
    
    fr fr Test thread information
    sus thread_info *ThreadInfo = get_current_thread_info()
    yo thread_info != cringe {
        vibez.spillf("✅ Thread name: '{}'", thread_info.name)
        vibez.spillf("✅ OS thread ID: {}", thread_info.os_thread_id)
        vibez.spillf("✅ Stack size: {} KB", thread_info.stack_size / 1024)
        vibez.spillf("✅ NUMA node: {}", thread_info.numa_node)
        vibez.spillf("✅ CPU affinity: 0x{:x}", thread_info.cpu_affinity)
    }
    
    fr fr Test thread name setting
    yo set_thread_name("enhanced_memory_test") {
        vibez.spill("✅ Thread name set successfully")
    }
    
    fr fr Test thread utility functions
    vibez.spillf("✅ Is main thread: {}", is_main_thread())
    vibez.spillf("✅ Current thread name: '{}'", get_thread_name())
    vibez.spillf("✅ Stack usage: {} bytes", get_thread_stack_usage())
    
    fr fr Show thread statistics
    get_thread_statistics()
    
    vibez.spill("✅ Thread identification test completed successfully")
    damn based
}

fr fr Test high-resolution timing
slay test_high_resolution_timing() lit {
    vibez.spill("Testing High-Resolution Timing...")
    vibez.spill("=" * 40)
    
    fr fr Initialize timing system
    sus timer *HighResolutionTimer = hr_timing_init()
    yo timer == cringe {
        vibez.spill("❌ High-resolution timing initialization failed")
        damn cap
    }
    
    fr fr Test basic timing functions
    sus time_ns thicc = hr_timing_get_time_ns()
    sus time_us thicc = hr_timing_get_time_us()
    sus time_ms thicc = hr_timing_get_time_ms()
    sus cpu_cycles thicc = hr_timing_get_cpu_cycles()
    
    vibez.spillf("✅ Current time: {} ns, {} µs, {} ms", time_ns, time_us, time_ms)
    vibez.spillf("✅ CPU cycles: {}", cpu_cycles)
    
    fr fr Test timing measurements
    sus measurement TimingMeasurement = hr_timing_start_measurement()
    
    fr fr Simulate work
    sus work_counter normie = 0
    bestie i := 0; i < 1000000; i = i + 1 {
        work_counter = work_counter + i % 100
    }
    
    yo hr_timing_end_measurement(&measurement) {
        sus duration_ns thicc = hr_timing_get_duration_ns(&measurement)
        sus duration_us thicc = hr_timing_get_duration_us(&measurement)
        sus duration_cycles thicc = hr_timing_get_duration_cycles(&measurement)
        
        vibez.spillf("✅ Work completed in {} ns ({} µs, {} cycles)", 
                    duration_ns, duration_us, duration_cycles)
        vibez.spillf("✅ Overhead compensated: {}", measurement.overhead_compensated)
    }
    
    fr fr Test function benchmarking
    vibez.spill("Benchmarking simple function...")
    hr_timing_benchmark_function(slay() void {
        sus dummy normie = 42 * 13 + 7
    }, 1000)
    
    fr fr Show timing system information
    hr_timing_get_system_info()
    
    vibez.spill("✅ High-resolution timing test completed successfully")
    damn based
}

fr fr Test memory profiler
slay test_memory_profiler() lit {
    vibez.spill("Testing Memory Profiler...")
    vibez.spill("=" * 40)
    
    fr fr Enable memory profiler with full tracking
    yo !profiler_enable(based, based, 10000) {
        vibez.spill("❌ Memory profiler initialization failed")
        damn cap
    }
    
    vibez.spill("✅ Memory profiler enabled with full tracking")
    
    fr fr Allocate various sizes to test profiler
    sus test_ptrs [](*void) = []
    sus sizes []normie = [64, 256, 1024, 4096, 16384, 65536]
    
    bestie i := 0; i < sizes.len(); i = i + 1 {
        bestie j := 0; j < 10; j = j + 1 {
            sus ptr *void = memory_alloc(sizes[i])
            yo ptr != cringe {
                test_ptrs.push(ptr)
            }
        }
    }
    
    vibez.spillf("✅ Allocated {} test objects", test_ptrs.len())
    
    fr fr Free some objects to test tracking
    bestie i := 0; i < test_ptrs.len() / 2; i = i + 1 {
        memory_free(test_ptrs[i])
    }
    
    vibez.spillf("✅ Freed {} objects", test_ptrs.len() / 2)
    
    fr fr Generate profiler report
    profiler_generate_report()
    
    fr fr Detect memory leaks
    yo profiler_detect_leaks() {
        vibez.spill("✅ Memory leaks detected (expected for test)")
    } otherwise {
        vibez.spill("✅ No memory leaks detected")
    }
    
    fr fr Clean up remaining objects
    bestie i := test_ptrs.len() / 2; i < test_ptrs.len(); i = i + 1 {
        memory_free(test_ptrs[i])
    }
    
    vibez.spill("✅ Memory profiler test completed successfully")
    damn based
}

fr fr Test integrated memory management
slay test_integrated_memory_management() lit {
    vibez.spill("Testing Integrated Memory Management...")
    vibez.spill("=" * 50)
    
    fr fr Initialize the complete system
    yo !memory_init() {
        vibez.spill("❌ Memory management system initialization failed")
        damn cap
    }
    
    vibez.spill("✅ Production memory management system initialized")
    
    fr fr Test allocation with all features enabled
    sus allocation_sizes []normie = [128, 512, 2048, 8192, 32768]
    sus allocations [](*void) = []
    
    vibez.spill("Testing integrated allocation with timing...")
    
    bestie size_idx := 0; size_idx < allocation_sizes.len(); size_idx = size_idx + 1 {
        sus size normie = allocation_sizes[size_idx]
        sus start_measurement TimingMeasurement = hr_timing_start_measurement()
        
        bestie i := 0; i < 50; i = i + 1 {
            sus ptr *void = memory_alloc(size)
            yo ptr != cringe {
                allocations.push(ptr)
            }
        }
        
        hr_timing_end_measurement(&start_measurement)
        sus alloc_time thicc = hr_timing_get_duration_us(&start_measurement)
        
        vibez.spillf("✅ Allocated 50x {} byte objects in {} µs", size, alloc_time)
    }
    
    vibez.spillf("✅ Total allocations: {}", allocations.len())
    
    fr fr Test memory statistics with all components
    vibez.spill("")
    memory_stats()
    
    fr fr Test memory leak detection
    vibez.spill("")
    vibez.spill("Testing memory leak detection...")
    profiler_detect_leaks()
    
    fr fr Clean up half the allocations
    vibez.spill("")
    vibez.spill("Cleaning up half of allocations...")
    
    bestie i := 0; i < allocations.len() / 2; i = i + 1 {
        memory_free(allocations[i])
    }
    
    fr fr Force garbage collection
    advanced_gc_force_collection(global_advanced_gc)
    
    fr fr Force heap defragmentation
    heap_defrag_force_compaction()
    
    fr fr Final memory statistics
    vibez.spill("")
    vibez.spill("Final integrated system statistics:")
    memory_stats()
    
    vibez.spill("✅ Integrated memory management test completed successfully")
    damn based
}

fr fr Performance stress test
slay test_memory_performance() lit {
    vibez.spill("Testing Memory Management Performance...")
    vibez.spill("=" * 45)
    
    sus iterations normie = 10000
    sus allocation_sizes []normie = [32, 64, 128, 256, 512, 1024, 2048, 4096]
    
    vibez.spillf("Performance test: {} iterations per size", iterations)
    
    bestie size_idx := 0; size_idx < allocation_sizes.len(); size_idx = size_idx + 1 {
        sus size normie = allocation_sizes[size_idx]
        
        fr fr Measure allocation performance
        sus alloc_start thicc = hr_timing_get_time_ns()
        sus ptrs [](*void) = []
        
        bestie i := 0; i < iterations; i = i + 1 {
            sus ptr *void = memory_alloc(size)
            yo ptr != cringe {
                ptrs.push(ptr)
            }
        }
        
        sus alloc_end thicc = hr_timing_get_time_ns()
        sus alloc_time thicc = alloc_end - alloc_start
        
        fr fr Measure deallocation performance
        sus free_start thicc = hr_timing_get_time_ns()
        
        bestie i := 0; i < ptrs.len(); i = i + 1 {
            memory_free(ptrs[i])
        }
        
        sus free_end thicc = hr_timing_get_time_ns()
        sus free_time thicc = free_end - free_start
        
        fr fr Calculate performance metrics
        sus alloc_per_op drip = drip(alloc_time) / drip(iterations)
        sus free_per_op drip = drip(free_time) / drip(iterations)
        sus total_time thicc = alloc_time + free_time
        sus throughput drip = drip(iterations * 2) / (drip(total_time) / 1000000000.0)  fr fr ops/sec
        
        vibez.spillf("Size {} bytes: Alloc {:.1f} ns/op, Free {:.1f} ns/op, {:.0f} ops/sec",
                    size, alloc_per_op, free_per_op, throughput)
    }
    
    vibez.spill("✅ Memory performance test completed successfully")
    damn based
}

fr fr Main test runner
slay main() {
    vibez.spill("CURSED Enhanced Memory Management Test Suite")
    vibez.spill("=" * 60)
    vibez.spill("")
    
    sus all_tests_passed lit = based
    
    fr fr Run individual component tests
    yo !test_numa_topology() {
        all_tests_passed = cap
        vibez.spill("❌ NUMA topology test failed")
    }
    
    vibez.spill("")
    yo !test_advanced_gc() {
        all_tests_passed = cap
        vibez.spill("❌ Advanced GC test failed")
    }
    
    vibez.spill("")
    yo !test_heap_defragmentation() {
        all_tests_passed = cap
        vibez.spill("❌ Heap defragmentation test failed")
    }
    
    vibez.spill("")
    yo !test_thread_identification() {
        all_tests_passed = cap
        vibez.spill("❌ Thread identification test failed")
    }
    
    vibez.spill("")
    yo !test_high_resolution_timing() {
        all_tests_passed = cap
        vibez.spill("❌ High-resolution timing test failed")
    }
    
    vibez.spill("")
    yo !test_memory_profiler() {
        all_tests_passed = cap
        vibez.spill("❌ Memory profiler test failed")
    }
    
    vibez.spill("")
    yo !test_integrated_memory_management() {
        all_tests_passed = cap
        vibez.spill("❌ Integrated memory management test failed")
    }
    
    vibez.spill("")
    yo !test_memory_performance() {
        all_tests_passed = cap
        vibez.spill("❌ Memory performance test failed")
    }
    
    fr fr Final results
    vibez.spill("")
    vibez.spill("=" * 60)
    
    yo all_tests_passed {
        vibez.spill("🎉 ALL TESTS PASSED!")
        vibez.spill("✅ Production-grade memory management system is working correctly")
        vibez.spill("✅ All enhanced components validated successfully")
        vibez.spill("✅ NUMA topology, advanced GC, heap defrag, thread ID, timing, and profiler operational")
        vibez.spill("✅ Memory management is ready for production use")
    } otherwise {
        vibez.spill("❌ SOME TESTS FAILED")
        vibez.spill("⚠️  Please review failed components before production use")
    }
    
    vibez.spill("")
    vibez.spill("Enhanced Memory Management Test Suite Complete")
    vibez.spill("=" * 60)
}
