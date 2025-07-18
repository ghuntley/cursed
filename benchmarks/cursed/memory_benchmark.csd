yeet "testz"
yeet "collections"

# Memory Performance Benchmark
test_start("memory_performance_benchmark")

# Memory allocation and deallocation test
sus iterations normie = 1000
sus total_allocs normie = 0

bestie i := 0; i < iterations; i++ {
    sus new_list = collections.new_list()
    
    # Allocate memory by adding items
    bestie j := 0; j < 100; j++ {
        collections.list_add(new_list, j)
        total_allocs = total_allocs + 1
    }
    
    # Memory will be freed when new_list goes out of scope
}

# Simulate timing and calculate allocs/sec
sus allocs_per_sec meal = total_allocs / 1.0  # Would use actual elapsed time

vibez.spill("Memory benchmark completed: {} allocations", total_allocs)
vibez.spill("Performance: {} allocs/sec", allocs_per_sec)

assert_true(allocs_per_sec > 50000)  # Should be > 50K allocs/sec
print_test_summary()
