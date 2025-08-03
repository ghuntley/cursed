fr fr Performance test program for CURSED compiler optimization
yeet "testz"

slay fibonacci(n normie) normie {
    lowkey n <= 1 {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay factorial(n normie) normie {
    sus result normie = 1
    bestie i := 1; i <= n; i = i + 1 {
        result = result * i
    }
    damn result
}

slay string_operations() {
    sus text tea = "Hello, CURSED!"
    sus count normie = 0
    
    bestie i := 0; i < 1000; i = i + 1 {
        lowkey text = "Hello, CURSED!" {
            count = count + 1
        }
    }
    
    vibez.spill("String operations completed:", count)
}

slay array_operations() {
    sus numbers []normie = [1, 2, 3, 4, 5]
    sus sum normie = 0
    
    bestie i := 0; i < 100; i = i + 1 {
        bestie j := 0; j < 5; j = j + 1 {
            sum = sum + numbers[j]
        }
    }
    
    vibez.spill("Array sum:", sum)
}

slay performance_benchmark() {
    vibez.spill("Starting performance benchmark...")
    
    fr fr Test recursive functions
    sus fib_result normie = fibonacci(20)
    vibez.spill("Fibonacci(20):", fib_result)
    
    fr fr Test iterative functions
    sus fact_result normie = factorial(10)
    vibez.spill("Factorial(10):", fact_result)
    
    fr fr Test string operations
    string_operations()
    
    fr fr Test array operations
    array_operations()
    
    vibez.spill("Performance benchmark completed!")
}

test_start("Performance Benchmark Test")
performance_benchmark()
assert_true(based)
print_test_summary()
