# Baseline Benchmark 2: Functions and Recursion
# Tests: Function calls, recursion, stack management

yeet "timez"

slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay iterative_sum(n drip) drip {
    sus result drip = 0
    bestie (sus i drip = 1; i <= n; i++) {
        result = result + i
    }
    damn result
}

slay benchmark_functions() drip {
    sus iterations drip = 30
    sus total drip = 0
    
    bestie (sus i drip = 0; i < iterations; i++) {
        total = total + fibonacci(i)
        total = total + factorial(i % 10)  # Keep factorial manageable
        total = total + iterative_sum(i * 100)
    }
    
    damn total
}

slay main() drip {
    sus start drip = timez.now_microseconds()
    sus result drip = benchmark_functions()
    sus end drip = timez.now_microseconds()
    
    vibez.spill("Functions benchmark result:", result)
    vibez.spill("Execution time (μs):", end - start)
    
    damn 0
}
