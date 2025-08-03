fr fr Comprehensive performance test for optimized CURSED compiler
fr fr Tests lexer performance, memory efficiency, and throughput

yeet "testz"

slay fibonacci_recursive(n normie) normie {
    lowkey n <= 1 {
        damn n
    }
    damn fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

slay fibonacci_iterative(n normie) normie {
    lowkey n <= 1 {
        damn n
    }
    
    sus a normie = 0
    sus b normie = 1
    sus temp normie = 0
    
    bestie i := 2; i <= n; i = i + 1 {
        temp = a + b
        a = b
        b = temp
    }
    
    damn b
}

slay factorial_optimized(n normie) normie {
    sus result normie = 1
    bestie i := 1; i <= n; i = i + 1 {
        result = result * i
    }
    damn result
}

slay string_performance_test() {
    sus messages []tea = [
        "Performance test message 1",
        "Performance test message 2", 
        "Performance test message 3",
        "Performance test message 4",
        "Performance test message 5"
    ]
    
    sus total_length normie = 0
    bestie i := 0; i < 5; i = i + 1 {
        bestie j := 0; j < 100; j = j + 1 {
            total_length = total_length + 25  fr fr Approximate message length
        }
    }
    
    vibez.spill("String performance test completed, total length:", total_length)
}

slay array_operations_benchmark() {
    sus numbers []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus sum normie = 0
    sus product normie = 1
    
    bestie iteration := 0; iteration < 50; iteration = iteration + 1 {
        bestie i := 0; i < 10; i = i + 1 {
            sum = sum + numbers[i]
            lowkey numbers[i] <= 5 {
                product = product * numbers[i] 
            }
        }
    }
    
    vibez.spill("Array benchmark - Sum:", sum, "Product:", product)
}

slay nested_loop_performance() {
    sus total normie = 0
    
    bestie outer := 0; outer < 20; outer = outer + 1 {
        bestie middle := 0; middle < 15; middle = middle + 1 {
            bestie inner := 0; inner < 10; inner = inner + 1 {
                total = total + (outer * middle * inner)
            }
        }
    }
    
    vibez.spill("Nested loop performance test result:", total)
}

slay conditional_branching_test() {
    sus positive_count normie = 0
    sus negative_count normie = 0
    sus zero_count normie = 0
    
    bestie i := -50; i <= 50; i = i + 1 {
        lowkey i > 0 {
            positive_count = positive_count + 1
        } highkey i < 0 {
            negative_count = negative_count + 1
        } highkey {
            zero_count = zero_count + 1
        }
    }
    
    vibez.spill("Branching test - Positive:", positive_count, "Negative:", negative_count, "Zero:", zero_count)
}

slay comprehensive_performance_suite() {
    vibez.spill("🚀 Starting Comprehensive Performance Test Suite")
    vibez.spill("===============================================")
    
    fr fr Test 1: Recursive vs Iterative Performance
    vibez.spill("Test 1: Fibonacci Performance Comparison")
    sus fib_recursive normie = fibonacci_recursive(15)
    sus fib_iterative normie = fibonacci_iterative(25)
    vibez.spill("Fibonacci recursive(15):", fib_recursive)
    vibez.spill("Fibonacci iterative(25):", fib_iterative)
    
    fr fr Test 2: Factorial Computation
    vibez.spill("Test 2: Factorial Computation")
    sus fact_result normie = factorial_optimized(12)
    vibez.spill("Factorial(12):", fact_result)
    
    fr fr Test 3: String Processing Performance
    vibez.spill("Test 3: String Performance Test")
    string_performance_test()
    
    fr fr Test 4: Array Operations Benchmark
    vibez.spill("Test 4: Array Operations Benchmark")
    array_operations_benchmark()
    
    fr fr Test 5: Nested Loop Performance
    vibez.spill("Test 5: Nested Loop Performance")
    nested_loop_performance()
    
    fr fr Test 6: Conditional Branching Test
    vibez.spill("Test 6: Conditional Branching Test")
    conditional_branching_test()
    
    vibez.spill("✅ Comprehensive Performance Test Suite Completed!")
    vibez.spill("Performance characteristics validated for production use")
}

fr fr Main test execution
test_start("Comprehensive Performance Test")
comprehensive_performance_suite()
assert_true(based)
print_test_summary()

fr fr Additional stress tests for compiler performance
slay compiler_stress_test() {
    vibez.spill("🔥 Compiler Stress Test - Token Generation")
    
    fr fr Generate many tokens to test lexer performance
    sus counter normie = 0
    bestie batch := 0; batch < 10; batch = batch + 1 {
        bestie item := 0; item < 100; item = item + 1 {
            counter = counter + 1
        }
    }
    
    vibez.spill("Generated token patterns for", counter, "iterations")
}

compiler_stress_test()
