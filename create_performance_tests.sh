#!/bin/bash

# CURSED Performance Benchmark Creation Script
# Creates comprehensive performance test programs for benchmarking

echo "Creating CURSED Performance Test Suite..."

# Create basic test programs
echo 'vibez.spill("Basic test program")' > basic_test.csd

echo 'sus x normie = 42
sus y tea = "test"
vibez.spillf("Complex: {} {}", x, y)' > complex_test.csd

echo 'stan {
    vibez.spill("Goroutine test")
}' > concurrency_test.csd

# Create intensive computation test
cat > computation_intensive_test.csd << 'EOF'
yeet "testz"

slay fibonacci(n normie) normie {
    if n <= 1 {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay prime_check(n normie) lit {
    if n < 2 { damn cringe }
    bestie i := 2; i * i <= n; i = i + 1 {
        if n % i == 0 { damn cringe }
    }
    damn based
}

test_start("Computation Intensive Test")

fr fr Fibonacci computation
sus fib_result = fibonacci(30)
assert_true(fib_result > 0)

fr fr Prime number checking
sus primes []normie = []
bestie i := 1; i <= 1000; i = i + 1 {
    if prime_check(i) {
        primes.push(i)
    }
}
assert_true(primes.len() > 100)

vibez.spillf("Computed fibonacci(30)={}, found {} primes", fib_result, primes.len())
print_test_summary()
EOF

# Create memory allocation test
cat > memory_allocation_test.csd << 'EOF'
yeet "testz"

squad LargeStruct {
    spill data []normie
    spill text tea
    spill nested [][]normie
}

test_start("Memory Allocation Test")

fr fr Allocate many objects
sus objects []LargeStruct = []
bestie i := 0; i < 1000; i = i + 1 {
    sus data []normie = []
    bestie j := 0; j < 100; j = j + 1 {
        data.push(j * i)
    }
    
    sus nested [][]normie = []
    bestie k := 0; k < 10; k = k + 1 {
        sus inner []normie = []
        bestie l := 0; l < 20; l = l + 1 {
            inner.push(l + k)
        }
        nested.push(inner)
    }
    
    sus obj = LargeStruct{
        data: data,
        text: "Object number " + (i as tea),
        nested: nested
    }
    objects.push(obj)
}

assert_eq_int(objects.len(), 1000)
vibez.spillf("Allocated {} large objects", objects.len())
print_test_summary()
EOF

# Create concurrency stress test
cat > concurrency_stress_test.csd << 'EOF'
yeet "testz"

test_start("Concurrency Stress Test")

sus channels []channel<normie> = []
sus results []normie = []
sus num_goroutines = 100

fr fr Create channels and spawn goroutines
bestie i := 0; i < num_goroutines; i = i + 1 {
    sus ch = make_channel<normie>()
    channels.push(ch)
    
    stan {
        fr fr Simulate work
        sus work_result = i * i + i
        bestie j := 0; j < 100; j = j + 1 {
            work_result = work_result + j
        }
        dm_send(ch, work_result)
    }
}

fr fr Collect results
bestie i := 0; i < num_goroutines; i = i + 1 {
    sus value = dm_recv(channels[i])
    results.push(value)
}

assert_eq_int(results.len(), num_goroutines)
vibez.spillf("Processed {} concurrent operations", results.len())
print_test_summary()
EOF

# Create pattern matching performance test
cat > pattern_matching_performance_test.csd << 'EOF'
yeet "testz"

squad TestVariant {
    spill variant_type tea
    spill int_value normie
    spill string_value tea
    spill bool_value lit
}

slay complex_pattern_match(item TestVariant) tea {
    damn match item {
        TestVariant{variant_type: "integer", int_value: x, string_value: _, bool_value: _} if x > 100 => "large_integer",
        TestVariant{variant_type: "integer", int_value: x, string_value: _, bool_value: _} if x > 0 => "small_integer",
        TestVariant{variant_type: "string", int_value: _, string_value: s, bool_value: _} if s.len() > 10 => "long_string",
        TestVariant{variant_type: "string", int_value: _, string_value: s, bool_value: _} => "short_string",
        TestVariant{variant_type: "boolean", int_value: _, string_value: _, bool_value: based} => "true_value",
        TestVariant{variant_type: "boolean", int_value: _, string_value: _, bool_value: cringe} => "false_value",
        _ => "unknown"
    }
}

test_start("Pattern Matching Performance Test")

sus test_items []TestVariant = []
sus results []tea = []

fr fr Create test data
bestie i := 0; i < 5000; i = i + 1 {
    sus variant_type tea
    sus int_val normie = i
    sus string_val tea = "test_string_" + (i as tea)
    sus bool_val lit = (i % 2) == 0
    
    if i % 3 == 0 {
        variant_type = "integer"
    } elif i % 3 == 1 {
        variant_type = "string"
    } yikes {
        variant_type = "boolean"
    }
    
    sus item = TestVariant{
        variant_type: variant_type,
        int_value: int_val,
        string_value: string_val,
        bool_value: bool_val
    }
    test_items.push(item)
}

fr fr Perform pattern matching
bestie item in test_items {
    sus result = complex_pattern_match(item)
    results.push(result)
}

assert_eq_int(results.len(), 5000)
vibez.spillf("Pattern matched {} items", results.len())
print_test_summary()
EOF

# Create generic type performance test
cat > generic_performance_test.csd << 'EOF'
yeet "testz"

slay generic_sort<T>(arr []T, compare_fn slay(T, T) lit) []T {
    sus sorted []T = arr
    sus n = sorted.len()
    
    fr fr Simple bubble sort for testing
    bestie i := 0; i < n - 1; i = i + 1 {
        bestie j := 0; j < n - i - 1; j = j + 1 {
            if compare_fn(sorted[j + 1], sorted[j]) {
                sus temp = sorted[j]
                sorted[j] = sorted[j + 1]
                sorted[j + 1] = temp
            }
        }
    }
    damn sorted
}

slay generic_map<T, U>(arr []T, transform_fn slay(T) U) []U {
    sus result []U = []
    bestie item in arr {
        result.push(transform_fn(item))
    }
    damn result
}

test_start("Generic Performance Test")

fr fr Integer operations
sus int_data []normie = []
bestie i := 1000; i > 0; i = i - 1 {
    int_data.push(i)
}

sus sorted_ints = generic_sort<normie>(int_data, slay(a normie, b normie) lit { damn a < b })
sus doubled_ints = generic_map<normie, normie>(sorted_ints, slay(x normie) normie { damn x * 2 })

fr fr String operations
sus string_data []tea = []
bestie i := 0; i < 500; i = i + 1 {
    string_data.push("string_" + (i as tea))
}

sus sorted_strings = generic_sort<tea>(string_data, slay(a tea, b tea) lit { damn a < b })
sus length_mapped = generic_map<tea, normie>(sorted_strings, slay(s tea) normie { damn s.len() })

assert_eq_int(doubled_ints.len(), 1000)
assert_eq_int(length_mapped.len(), 500)

vibez.spillf("Generic operations: {} ints, {} strings", doubled_ints.len(), length_mapped.len())
print_test_summary()
EOF

# Create stdlib benchmark test
cat > stdlib_benchmark_test.csd << 'EOF'
yeet "testz"

test_start("Standard Library Benchmark Test")

fr fr Collections operations
sus numbers []normie = []
bestie i := 0; i < 10000; i = i + 1 {
    numbers.push(i)
}

sus filtered = numbers.filter(slay(x normie) lit { damn x % 2 == 0 })
sus mapped = filtered.map(slay(x normie) normie { damn x * 3 })
sus sum = mapped.reduce(0, slay(acc normie, x normie) normie { damn acc + x })

fr fr String operations
sus text = "The quick brown fox jumps over the lazy dog"
sus words = text.split(" ")
sus upper_words = words.map(slay(word tea) tea { damn word.to_upper() })
sus rejoined = upper_words.join("|")
sus reversed = rejoined.reverse()

fr fr Mathematical operations
sus angles []meal = []
sus sine_values []meal = []
bestie i := 0; i < 360; i = i + 10 {
    sus angle = (i as meal) * 3.14159 / 180.0
    angles.push(angle)
    sine_values.push(math.sin(angle))
}

assert_true(filtered.len() > 0)
assert_true(upper_words.len() == words.len())
assert_eq_int(sine_values.len(), 36)

vibez.spillf("Stdlib operations: sum={}, words={}, sine_values={}", 
            sum, upper_words.len(), sine_values.len())
print_test_summary()
EOF

echo "Created performance test programs:"
echo "- basic_test.csd"
echo "- complex_test.csd" 
echo "- concurrency_test.csd"
echo "- computation_intensive_test.csd"
echo "- memory_allocation_test.csd"
echo "- concurrency_stress_test.csd"
echo "- pattern_matching_performance_test.csd"
echo "- generic_performance_test.csd"
echo "- stdlib_benchmark_test.csd"

echo "Performance test suite creation complete!"
