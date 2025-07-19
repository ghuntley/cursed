yeet "testz"

test_start("PAL Performance Benchmark")

sus iterations drip = 1000000

// Memory allocation benchmark
sus start_time drip = current_time_nanos()
periodt i := 0; i < iterations; i++ {
    sus ptr drip = allocate_memory(64)
    deallocate_memory(ptr, 64)
}
sus memory_time drip = current_time_nanos() - start_time

// Goroutine spawning benchmark
start_time = current_time_nanos()
periodt i := 0; i < 1000; i++ {
    stan {
        sus dummy drip = i * i
    }
}
wait_for_all_goroutines()
sus goroutine_time drip = current_time_nanos() - start_time

vibez.spill("Memory operations: " + str(memory_time) + " ns")
vibez.spill("Goroutine operations: " + str(goroutine_time) + " ns")

print_test_summary()
