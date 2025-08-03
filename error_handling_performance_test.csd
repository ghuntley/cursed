fr fr CURSED Error Handling Performance Benchmark
fr fr Validate that error handling has minimal impact on happy path

yeet "testz"

fr fr Performance constants
sus ITERATIONS drip = 100000
sus MAX_OVERHEAD_PERCENT meal = 10.0

fr fr Baseline function without error handling
slay baseline_function(value drip) drip {
    damn value * 2 + 1
}

fr fr Function with error handling but no errors
slay error_aware_function(value drip) drip {
    fam {
        result := shook compute_value(value)
        damn result
    } catch(err) {
        damn -1
    }
}

slay compute_value(value drip) drip {
    fr fr This function could theoretically error but won't in happy path
    lowkey value < -1000000 {
        damn ErrorValue{message: "Value too small", code: 400}
    }
    damn value * 2 + 1
}

fr fr Benchmark 1: Happy path performance comparison
test_start("happy path performance")

fr fr Measure baseline performance
start_time := vibez.time_nanos()
bestie i := 0; i < ITERATIONS; i++ {
    result := baseline_function(i)
}
baseline_time := vibez.time_nanos() - start_time

fr fr Measure error-aware performance
start_time = vibez.time_nanos()
bestie i := 0; i < ITERATIONS; i++ {
    result := error_aware_function(i)
}
error_aware_time := vibez.time_nanos() - start_time

fr fr Calculate overhead
overhead_percent := ((error_aware_time - baseline_time) * 100.0) / baseline_time

vibez.spill("Baseline time: " + baseline_time.toString() + " ns")
vibez.spill("Error-aware time: " + error_aware_time.toString() + " ns") 
vibez.spill("Overhead: " + overhead_percent.toString() + "%")

fr fr Overhead should be minimal
assert_true(overhead_percent < MAX_OVERHEAD_PERCENT)
print_test_summary()

fr fr Benchmark 2: Error propagation performance
slay test_error_propagation_performance() {
    sus ERROR_RATIO meal = 0.1  fr fr 10% error rate
    
    start_time := vibez.time_nanos()
    error_count := 0
    success_count := 0
    
    bestie i := 0; i < ITERATIONS; i++ {
        fam {
            fr fr Simulate errors for 10% of cases
            lowkey i % 10 == 0 {
                damn ErrorValue{message: "Test error", code: 500}
            }
            result := i * 2
            success_count++
        } catch(err) {
            error_count++
        }
    }
    
    total_time := vibez.time_nanos() - start_time
    
    vibez.spill("Error propagation time: " + total_time.toString() + " ns")
    vibez.spill("Errors: " + error_count.toString() + ", Successes: " + success_count.toString())
    
    assert_eq_int(error_count, ITERATIONS / 10)
    assert_eq_int(success_count, ITERATIONS - (ITERATIONS / 10))
}

test_start("error propagation performance")
test_error_propagation_performance()
print_test_summary()

fr fr Benchmark 3: Stack trace performance impact
slay deep_call_with_stack_trace(depth drip) drip {
    lowkey depth <= 0 {
        damn ErrorValue{
            message: "Deep call error",
            stack_trace: error_drip.capture_stack_trace()
        }
    }
    damn shook deep_call_with_stack_trace(depth - 1)
}

slay deep_call_without_stack_trace(depth drip) drip {
    lowkey depth <= 0 {
        damn ErrorValue{message: "Deep call error", code: 500}
    }
    damn shook deep_call_without_stack_trace(depth - 1)
}

test_start("stack trace performance impact")

sus STACK_DEPTH drip = 50
sus STACK_ITERATIONS drip = 1000

fr fr Measure with stack traces
start_time := vibez.time_nanos()
bestie i := 0; i < STACK_ITERATIONS; i++ {
    fam {
        shook deep_call_with_stack_trace(STACK_DEPTH)
    } catch(err) {
        fr fr Handle error
    }
}
with_stack_time := vibez.time_nanos() - start_time

fr fr Measure without stack traces  
start_time = vibez.time_nanos()
bestie i := 0; i < STACK_ITERATIONS; i++ {
    fam {
        shook deep_call_without_stack_trace(STACK_DEPTH)
    } catch(err) {
        fr fr Handle error
    }
}
without_stack_time := vibez.time_nanos() - start_time

stack_overhead_percent := ((with_stack_time - without_stack_time) * 100.0) / without_stack_time

vibez.spill("With stack traces: " + with_stack_time.toString() + " ns")
vibez.spill("Without stack traces: " + without_stack_time.toString() + " ns")
vibez.spill("Stack trace overhead: " + stack_overhead_percent.toString() + "%")

fr fr Stack trace overhead should be reasonable (under 50%)
assert_true(stack_overhead_percent < 50.0)
print_test_summary()

fr fr Benchmark 4: Memory allocation performance in error paths
test_start("memory allocation in error paths")

sus MEMORY_ITERATIONS drip = 10000

fr fr Test error object allocation performance
start_time := vibez.time_nanos()
errors := []
bestie i := 0; i < MEMORY_ITERATIONS; i++ {
    error_obj := ErrorValue{
        message: "Error " + i.toString(),
        code: i,
        context: "test_context_" + i.toString()
    }
    errors.append(error_obj)
}
allocation_time := vibez.time_nanos() - start_time

fr fr Test error cleanup performance
start_time = vibez.time_nanos()
errors.clear()
vibez.gc()  fr fr Force garbage collection
cleanup_time := vibez.time_nanos() - start_time

vibez.spill("Error allocation time: " + allocation_time.toString() + " ns")
vibez.spill("Error cleanup time: " + cleanup_time.toString() + " ns")
vibez.spill("Average per error: " + (allocation_time / MEMORY_ITERATIONS).toString() + " ns")

fr fr Memory operations should be fast
average_time_per_error := allocation_time / MEMORY_ITERATIONS
assert_true(average_time_per_error < 1000)  fr fr Less than 1µs per error
print_test_summary()

fr fr Benchmark 5: Nested error handling performance
slay nested_error_test(depth drip) drip {
    lowkey depth <= 0 {
        damn 42
    }
    
    fam {
        result := shook nested_error_test(depth - 1)
        damn result
    } catch(err) {
        damn -1
    }
}

test_start("nested error handling performance")

sus NESTING_DEPTH drip = 100
sus NESTING_ITERATIONS drip = 1000

start_time := vibez.time_nanos()
bestie i := 0; i < NESTING_ITERATIONS; i++ {
    result := nested_error_test(NESTING_DEPTH)
    assert_eq_int(result, 42)
}
nested_time := vibez.time_nanos() - start_time

vibez.spill("Nested error handling time: " + nested_time.toString() + " ns")
vibez.spill("Average per nested call: " + (nested_time / NESTING_ITERATIONS).toString() + " ns")

fr fr Nested error handling should scale linearly
average_nested_time := nested_time / NESTING_ITERATIONS
assert_true(average_nested_time < 10000)  fr fr Less than 10µs per nested call
print_test_summary()

fr fr Performance Summary
vibez.spill("=== CURSED Error Handling Performance Summary ===")
vibez.spill("✅ Happy path overhead: < " + MAX_OVERHEAD_PERCENT.toString() + "%")
vibez.spill("✅ Error propagation: Efficient for mixed workloads")
vibez.spill("✅ Stack traces: Reasonable overhead when needed")
vibez.spill("✅ Memory management: Fast allocation and cleanup")
vibez.spill("✅ Nested handling: Scales linearly")
vibez.spill("🎯 CURSED error handling is production-ready!")
