yeet "testz"

# Basic Performance Testing Demo

slay simple_load_test(func_name tea, users normie) lit {
    vibez.spill("Load Test: ", func_name, " with ", users, " users")
    
    sus total normie = 0
    sus success normie = 0
    sus i normie = 0
    
    bestie i < users * 10 {
        # Simulate function execution
        lowkey func_name == "fast_function" {
            success = success + 1
        } fam lowkey func_name == "slow_function" {
            success = success + 1
        } fam {
            success = success + 1
        }
        total = total + 1
        i = i + 1
    }
    
    vibez.spill("Results: ", success, "/", total, " successful")
    damn (success > 0)
}

slay simple_benchmark(func_name tea, iterations normie) drip {
    vibez.spill("Benchmark: ", func_name, " for ", iterations, " iterations")
    
    sus total_time drip = 0.0
    sus i normie = 0
    
    bestie i < iterations {
        # Simulate timing
        total_time = total_time + 1.0
        i = i + 1
    }
    
    sus avg_time drip = total_time / iterations
    vibez.spill("Average time: ", avg_time, "ms")
    damn avg_time
}

slay simple_stress_test(func_name tea, max_load normie) lit {
    vibez.spill("Stress Test: ", func_name, " up to ", max_load)
    
    sus current_load normie = 1
    sus max_successful normie = 0
    
    bestie current_load <= max_load {
        sus success lit = simple_load_test(func_name, current_load)
        lowkey success {
            max_successful = current_load
        } fam {
            ghosted
        }
        current_load = current_load + 1
    }
    
    vibez.spill("Max successful load: ", max_successful)
    damn (max_successful > 0)
}

# Run the demo
test_start("Performance Testing Demo")

vibez.spill("=== Performance Testing Framework Demo ===")

# Test 1: Basic Load Test
test_start("simple load test")
sus load_result lit = simple_load_test("fast_function", 5)
assert_true(load_result)
print_test_summary()

# Test 2: Basic Benchmark
test_start("simple benchmark")
sus benchmark_result drip = simple_benchmark("fast_function", 100)
assert_true(benchmark_result > 0.0)
print_test_summary()

# Test 3: Basic Stress Test
test_start("simple stress test")
sus stress_result lit = simple_stress_test("fast_function", 3)
assert_true(stress_result)
print_test_summary()

vibez.spill("=== Demo Complete ===")
vibez.spill("Basic performance testing primitives are working!")

print_test_summary()
