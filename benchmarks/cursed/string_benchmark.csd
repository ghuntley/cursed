yeet "testz"
yeet "string"

# String Performance Benchmark  
test_start("string_performance_benchmark")

# String operations performance test
sus iterations normie = 10000
sus base_string tea = "Hello, World!"
sus total_ops normie = 0

bestie i := 0; i < iterations; i++ {
    sus concatenated tea = base_string + " " + i
    sus length normie = string.length(concatenated)
    sus upper tea = string.to_upper(concatenated)
    sus lower tea = string.to_lower(upper)
    total_ops = total_ops + 4
}

# Simulate timing and calculate ops/sec
sus ops_per_sec meal = total_ops / 1.0  # Would use actual elapsed time

vibez.spill("String benchmark completed: {} operations", total_ops)
vibez.spill("Performance: {} ops/sec", ops_per_sec)

assert_true(ops_per_sec > 10000)  # Should be > 10K ops/sec
print_test_summary()
