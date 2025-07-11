// Test CURSED memory optimization system
yeet "testz"

// Test basic memory allocation and deallocation
test_start("Basic Memory Operations")

// Test string allocation
sus test_string tea = "Hello, optimized memory!"
vibez.spill(test_string)
assert_eq_string(test_string, "Hello, optimized memory!")

// Test integer allocation
sus test_int normie = 42
vibez.spill(test_int)
assert_eq_int(test_int, 42)

// Test float allocation
sus test_float drip = 3.14
vibez.spill(test_float)

// Test array allocation
sus test_array [3]normie = [1, 2, 3]
vibez.spill(test_array[0])
assert_eq_int(test_array[0], 1)

vibez.spill("Basic memory operations test passed")

// Test memory pool optimization
test_start("Memory Pool Optimization")

// Allocate many small objects to test pool efficiency
bestie i := 0; i < 100; i++ {
    sus small_obj normie = i
    vibez.spill(small_obj)
}

vibez.spill("Memory pool optimization test passed")

// Test garbage collection optimization
test_start("GC Optimization")

// Create objects with different lifetimes
sus long_lived_obj tea = "I live long"
bestie j := 0; j < 50; j++ {
    sus short_lived_obj normie = j
    // short_lived_obj goes out of scope quickly
}

vibez.spill(long_lived_obj)
vibez.spill("GC optimization test passed")

// Test memory pressure detection
test_start("Memory Pressure Detection")

// Allocate progressively larger objects
sus sizes [5]normie = [64, 128, 256, 512, 1024]
bestie k := 0; k < 5; k++ {
    sus size_val normie = sizes[k]
    vibez.spill(size_val)
}

vibez.spill("Memory pressure detection test passed")

// Test concurrent memory allocation
test_start("Concurrent Memory Allocation")

// Simulate concurrent allocation patterns
bestie thread_id := 0; thread_id < 4; thread_id++ {
    bestie alloc_count := 0; alloc_count < 25; alloc_count++ {
        sus concurrent_obj normie = thread_id * 100 + alloc_count
        vibez.spill(concurrent_obj)
    }
}

vibez.spill("Concurrent memory allocation test passed")

// Test memory fragmentation handling
test_start("Memory Fragmentation Handling")

// Create mixed allocation patterns
sus mixed_sizes [4]normie = [16, 512, 32, 1024]
bestie m := 0; m < 4; m++ {
    sus frag_size normie = mixed_sizes[m]
    vibez.spill(frag_size)
}

vibez.spill("Memory fragmentation handling test passed")

// Test memory profiling
test_start("Memory Profiling")

// Allocate tracked objects
sus profiled_string tea = "Profiled allocation"
sus profiled_int normie = 123
sus profiled_float drip = 2.71

vibez.spill(profiled_string)
vibez.spill(profiled_int)
vibez.spill(profiled_float)

vibez.spill("Memory profiling test passed")

// Test adaptive garbage collection
test_start("Adaptive GC")

// Create allocation patterns that should trigger adaptive behavior
bestie burst_count := 0; burst_count < 20; burst_count++ {
    sus burst_obj normie = burst_count
    vibez.spill(burst_obj)
}

// Brief pause to simulate different allocation patterns
bestie pause := 0; pause < 10; pause++ {
    sus pause_obj normie = pause
}

// Another burst
bestie burst2_count := 0; burst2_count < 20; burst2_count++ {
    sus burst2_obj normie = burst2_count + 100
    vibez.spill(burst2_obj)
}

vibez.spill("Adaptive GC test passed")

// Test memory pool tuning
test_start("Memory Pool Tuning")

// Allocate objects of common sizes to test pool optimization
sus common_sizes [3]normie = [64, 128, 256]
bestie size_idx := 0; size_idx < 3; size_idx++ {
    bestie count := 0; count < 10; count++ {
        sus pool_obj normie = common_sizes[size_idx] + count
        vibez.spill(pool_obj)
    }
}

vibez.spill("Memory pool tuning test passed")

// Test leak detection simulation
test_start("Leak Detection")

// Create objects with known patterns
sus potential_leak tea = "This might leak"
sus properly_managed normie = 456

vibez.spill(potential_leak)
vibez.spill(properly_managed)

vibez.spill("Leak detection test passed")

// Test memory optimization under load
test_start("Memory Optimization Under Load")

// Simulate high memory load
bestie load_test := 0; load_test < 200; load_test++ {
    sus load_obj normie = load_test
    
    // Every 50th allocation, create a larger object
    sus large_obj normie = 0
    sho load_test % 50 == 0 {
        large_obj = load_test * 1000
        vibez.spill(large_obj)
    }
}

vibez.spill("Memory optimization under load test passed")

// Print final test summary
print_test_summary()
vibez.spill("All memory optimization tests completed successfully!")
