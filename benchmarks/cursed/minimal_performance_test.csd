fr fr Minimal Performance Test (without complex dependencies)

sus benchmark_iterations normie = 1000

fr fr Simple string operations benchmark
slay benchmark_strings() {
    sus start_time thicc = 0  fr fr Placeholder for timing
    sus result tea = ""
    
    bestie i := 0; i < benchmark_iterations; i++ {
        sus test_string tea = "Hello World Test String " + string.from_int(i)
        result = result + test_string
        lowkey i % 100 == 0 {
            vibez.spill("String operations progress: " + string.from_int(i))
        }
    }
    
    vibez.spill("String benchmark complete. Result length: " + string.from_int(string.length(result)))
}

fr fr Simple math operations benchmark
slay benchmark_math() {
    sus total drip = 0.0
    
    bestie i := 0; i < benchmark_iterations; i++ {
        sus x drip = drip.(i)
        total = total + x
        total = total + (x * 2.0)
        total = total + (x / 2.0)
        lowkey i % 100 == 0 {
            vibez.spill("Math operations progress: " + string.from_int(i))
        }
    }
    
    vibez.spill("Math benchmark complete. Total: " + string.from_float(total))
}

fr fr Simple array operations benchmark
slay benchmark_arrays() {
    sus numbers [1000]normie
    sus sum normie = 0
    
    fr fr Fill array
    bestie i := 0; i < 1000; i++ {
        numbers[i] = i * 2
    }
    
    fr fr Sum array multiple times
    bestie iteration := 0; iteration < 100; iteration++ {
        bestie i := 0; i < 1000; i++ {
            sum = sum + numbers[i]
        }
        lowkey iteration % 10 == 0 {
            vibez.spill("Array operations progress: " + string.from_int(iteration))
        }
    }
    
    vibez.spill("Array benchmark complete. Sum: " + string.from_int(sum))
}

slay main() {
    vibez.spill("=== CURSED Minimal Performance Test ===")
    vibez.spill("Testing " + string.from_int(benchmark_iterations) + " iterations")
    vibez.spill("")
    
    vibez.spill("Starting String Benchmark...")
    benchmark_strings()
    vibez.spill("")
    
    vibez.spill("Starting Math Benchmark...")
    benchmark_math()
    vibez.spill("")
    
    vibez.spill("Starting Array Benchmark...")
    benchmark_arrays()
    vibez.spill("")
    
    vibez.spill("=== Performance Test Complete ===")
}
