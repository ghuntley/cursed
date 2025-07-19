yeet "testz"

test_start("Apple Silicon Core Scheduling")

// CPU-intensive task should prefer P-cores
stan {
    periodt i := 0; i < 1000000; i++ {
        sus heavy_computation drip = fibonacci(30)
    }
    vibez.spill("CPU-intensive task completed")
}

// I/O task should prefer E-cores  
stan {
    periodt i := 0; i < 1000; i++ {
        read_file("test.txt")
        yield_goroutine()
    }
    vibez.spill("I/O task completed")
}

wait_for_all_goroutines()

print_test_summary()
