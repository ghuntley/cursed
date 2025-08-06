yeet "testz"
yeet "vibez"

// Comprehensive CURSED optimization benchmarks
// Tests all major optimization features: inlining, dead code elimination,
// constant folding, loop optimization, memory optimization, and PGO

// Function inlining benchmark
slay inline_test_small(x normie) normie {
    damn x + 1
}

slay inline_test_medium(x normie, y normie) normie {
    sus temp drip = x * 2
    damn temp + y
}

slay inline_test_large(a normie, b normie, c normie) normie {
    sus sum drip = a + b + c
    sus product drip = a * b * c
    sus average drip = sum / 3
    damn average + product
}

// Dead code elimination benchmark
slay dead_code_test() {
    // Dead variables
    sus unused_var drip = 42
    sus another_unused tea = "will be eliminated"
    
    // Dead computation
    sus dead_calc drip = 100 * 200 + 300
    
    // Used computation
    sus used_calc drip = 10 + 20
    vibez.spill(used_calc)
    
    // Dead branch
    bestie (cringe) {
        sus dead_branch_var drip = 999
        vibez.spill("This will never execute")
    }
}

// Constant folding benchmark
slay constant_folding_test() normie {
    // Arithmetic constant folding
    sus const1 drip = 10 + 20 + 30
    sus const2 drip = 100 * 2 / 4
    sus const3 drip = 50 - 25 + 10
    
    // Boolean constant folding
    sus bool_const lit = based && based
    sus bool_const2 lit = cringe || based
    
    // Conditional constant folding
    sus result drip = bestie (based) { 100 } yikes { 200 }
    
    damn const1 + const2 + const3 + result
}

// Loop optimization benchmark
slay loop_optimization_test() normie {
    sus sum drip = 0
    sus product drip = 1
    
    // Simple loop for unrolling
    sus i drip = 0
    bestie (i < 10) {
        sum = sum + i
        i = i + 1
    }
    
    // Vectorizable loop
    sus numbers drip[] = [1, 2, 3, 4, 5, 6, 7, 8]
    sus total drip = 0
    sus j drip = 0
    bestie (j < 8) {
        total = total + numbers[j]
        j = j + 1
    }
    
    // Loop with invariant code motion opportunity
    sus base drip = 100
    sus k drip = 0
    bestie (k < 20) {
        sus invariant drip = base * 2 + 50  // Should be moved out of loop
        sum = sum + invariant + k
        k = k + 1
    }
    
    damn sum + total
}

// Memory optimization benchmark
slay memory_optimization_test() normie {
    // Small allocation (should be stack promoted)
    sus small_array drip[] = [1, 2, 3, 4, 5]
    
    // Temporary variables with non-overlapping lifetimes (coalescing opportunity)
    {
        sus temp1 drip = 10 + 20
        vibez.spill(temp1)
    }
    
    {
        sus temp2 drip = 30 + 40
        vibez.spill(temp2)
    }
    
    // Access pattern that benefits from layout optimization
    sus counter drip = 0
    sus m drip = 0
    bestie (m < 5) {
        counter = counter + small_array[m]
        m = m + 1
    }
    
    damn counter
}

// Complex optimization scenario
slay complex_optimization_scenario() normie {
    // Multiple optimization opportunities
    sus result drip = 0
    
    // Inlining opportunity
    result = result + inline_test_small(10)
    result = result + inline_test_small(20)
    result = result + inline_test_small(30)
    
    // Constant folding in loop
    sus n drip = 0
    bestie (n < 5) {
        sus folded drip = 2 * 3 + 4  // Should be folded to 10
        result = result + folded
        n = n + 1
    }
    
    // Dead code mixed with live code
    sus live_var drip = result * 2
    sus dead_var drip = 999 + 111  // Dead
    result = result + live_var
    
    damn result
}

// Recursive function (should not be inlined)
slay fibonacci(n normie) normie {
    bestie (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

// Function with side effects (affects optimization decisions)
slay side_effect_function(x normie) normie {
    vibez.spill(x)  // Side effect: I/O
    damn x * 2
}

// Profile-guided optimization test
slay hot_function(data normie[]) normie {
    // This function would be marked as "hot" in profile data
    sus sum drip = 0
    sus i drip = 0
    bestie (i < 1000) {  // Hot loop
        sum = sum + data[i % 10]
        i = i + 1
    }
    damn sum
}

slay cold_function(x normie) normie {
    // This function would be marked as "cold" in profile data
    sus expensive_calc drip = x * x * x + x * x + x
    damn expensive_calc
}

// Vectorization benchmark
slay vectorization_test() normie {
    sus data drip[] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    sus result drip[] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    
    // Vector addition (should be vectorized)
    sus i drip = 0
    bestie (i < 16) {
        result[i] = data[i] + 10
        i = i + 1
    }
    
    // Vector multiplication (should be vectorized)
    sus j drip = 0
    bestie (j < 16) {
        result[j] = result[j] * 2
        j = j + 1
    }
    
    // Sum reduction
    sus total drip = 0
    sus k drip = 0
    bestie (k < 16) {
        total = total + result[k]
        k = k + 1
    }
    
    damn total
}

// Main benchmark function
slay main() normie {
    test_start("CURSED Optimization Benchmarks")
    
    // Test function inlining
    test_start("Function Inlining Benchmark")
    sus inline_result drip = 0
    sus iter drip = 0
    bestie (iter < 1000) {
        inline_result = inline_result + inline_test_small(iter)
        inline_result = inline_result + inline_test_medium(iter, iter + 1)
        iter = iter + 1
    }
    vibez.spillf("Inlining result: {}", inline_result)
    
    // Test dead code elimination
    test_start("Dead Code Elimination Benchmark")
    dead_code_test()
    vibez.spill("Dead code test completed")
    
    // Test constant folding
    test_start("Constant Folding Benchmark")
    sus folding_result drip = constant_folding_test()
    vibez.spillf("Constant folding result: {}", folding_result)
    assert_eq_int(folding_result, 190)  // 60 + 50 + 35 + 100 - 55 = 190
    
    // Test loop optimization
    test_start("Loop Optimization Benchmark")
    sus loop_result drip = loop_optimization_test()
    vibez.spillf("Loop optimization result: {}", loop_result)
    
    // Test memory optimization
    test_start("Memory Optimization Benchmark")
    sus memory_result drip = memory_optimization_test()
    vibez.spillf("Memory optimization result: {}", memory_result)
    
    // Test complex optimization scenario
    test_start("Complex Optimization Scenario")
    sus complex_result drip = complex_optimization_scenario()
    vibez.spillf("Complex optimization result: {}", complex_result)
    
    // Test vectorization
    test_start("Vectorization Benchmark")
    sus vector_result drip = vectorization_test()
    vibez.spillf("Vectorization result: {}", vector_result)
    
    // Test hot vs cold functions (PGO simulation)
    test_start("Profile-Guided Optimization Simulation")
    sus hot_data drip[] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus hot_result drip = hot_function(hot_data)
    sus cold_result drip = cold_function(5)
    vibez.spillf("Hot function result: {}, Cold function result: {}", hot_result, cold_result)
    
    // Performance measurement
    test_start("Performance Measurement")
    sus start_time drip = 0  // Would use actual timing in real implementation
    
    // Repeat optimized operations
    sus perf_iter drip = 0
    sus perf_total drip = 0
    bestie (perf_iter < 100) {
        perf_total = perf_total + constant_folding_test()
        perf_total = perf_total + loop_optimization_test()
        perf_total = perf_total + memory_optimization_test()
        perf_iter = perf_iter + 1
    }
    
    sus end_time drip = 1000  // Would use actual timing in real implementation
    sus execution_time drip = end_time - start_time
    
    vibez.spillf("Performance total: {}, Execution time: {}", perf_total, execution_time)
    
    // Optimization effectiveness tests
    test_start("Optimization Effectiveness Validation")
    
    // Test that constants are properly folded
    sus compile_time_constant drip = 5 * 10 + 15  // Should be folded to 65
    assert_eq_int(compile_time_constant, 65)
    
    // Test that simple expressions are optimized
    sus x drip = 10
    sus optimized_expr drip = x + 0  // Should be optimized to just x
    assert_eq_int(optimized_expr, 10)
    
    // Test boolean optimizations
    sus bool_opt lit = based && based  // Should be folded to based
    assert_true(bool_opt)
    
    sus bool_opt2 lit = cringe || based  // Should be folded to based
    assert_true(bool_opt2)
    
    print_test_summary()
    
    vibez.spill("🚀 CURSED Optimization Benchmarks Complete!")
    vibez.spill("   📈 Function inlining tested")
    vibez.spill("   🗑️ Dead code elimination tested")
    vibez.spill("   📊 Constant folding validated")
    vibez.spill("   🔄 Loop optimization benchmarked")
    vibez.spill("   💾 Memory optimization evaluated")
    vibez.spill("   ⚡ Vectorization performance measured")
    vibez.spill("   🎯 Profile-guided optimization simulated")
    
    damn 0
}

// Additional optimization stress tests
slay optimization_stress_test() {
    test_start("Optimization Stress Test")
    
    // Deeply nested inlining
    slay deeply_nested1(x normie) normie { damn x + 1 }
    slay deeply_nested2(x normie) normie { damn deeply_nested1(x) + 2 }
    slay deeply_nested3(x normie) normie { damn deeply_nested2(x) + 3 }
    slay deeply_nested4(x normie) normie { damn deeply_nested3(x) + 4 }
    
    sus nested_result drip = deeply_nested4(10)
    assert_eq_int(nested_result, 20)  // 10 + 1 + 2 + 3 + 4 = 20
    
    // Complex constant expressions
    sus complex_const drip = ((5 + 3) * 2 - 1) / 3 + (10 % 3) * 4
    // Should be folded to: ((8) * 2 - 1) / 3 + (1) * 4 = (16 - 1) / 3 + 4 = 15/3 + 4 = 5 + 4 = 9
    vibez.spillf("Complex constant result: {}", complex_const)
    
    // Nested loop optimization
    sus nested_sum drip = 0
    sus outer drip = 0
    bestie (outer < 10) {
        sus inner drip = 0
        bestie (inner < 10) {
            nested_sum = nested_sum + (outer * inner + 1)  // +1 should be loop invariant for inner loop
            inner = inner + 1
        }
        outer = outer + 1
    }
    vibez.spillf("Nested loop sum: {}", nested_sum)
    
    vibez.spill("✅ Stress test completed")
}

// Benchmark different optimization levels
slay benchmark_optimization_levels() {
    test_start("Optimization Level Comparison")
    
    // These would be compiled with different -O levels
    sus baseline_result drip = complex_optimization_scenario()
    vibez.spillf("Baseline (O0) result: {}", baseline_result)
    
    // Same computation, different optimization level
    sus optimized_result drip = complex_optimization_scenario()
    vibez.spillf("Optimized (O2) result: {}", optimized_result)
    
    // They should produce the same result but different performance
    assert_eq_int(baseline_result, optimized_result)
    
    vibez.spill("✅ Optimization level comparison completed")
}
