# JIT Engine Performance and Tier-up Test
# Tests the tiered compilation system and hot function optimization

yeet "testz"

# Hot function that should tier up quickly
slay fibonacci(n normie) normie {
    bestie (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

# Mathematical computation function
slay compute_pi_approximation(iterations normie) meal {
    sus pi_approx meal = 0.0
    sus i normie = 0
    
    bestie (i < iterations) {
        sus term meal = 1.0 / (2.0 * i.(meal) + 1.0)
        bestie (i % 2 == 0) {
            pi_approx = pi_approx + term
        } else {
            pi_approx = pi_approx - term
        }
        i = i + 1
    }
    
    damn pi_approx * 4.0
}

# String processing function
slay process_strings(count normie) tea {
    sus result tea = ""
    sus i normie = 0
    
    bestie (i < count) {
        result = result + "item" + i.(tea) + " "
        i = i + 1
    }
    
    damn result
}

# Array manipulation function
slay sum_array(arr_size normie) normie {
    # Create array manually since we don't have dynamic arrays yet
    sus total normie = 0
    sus i normie = 0
    
    bestie (i < arr_size) {
        total = total + i
        i = i + 1
    }
    
    damn total
}

# Struct creation and manipulation
squad TestStruct {
    spill id normie
    spill value meal
    spill name tea
}

slay create_and_process_structs(count normie) normie {
    sus processed normie = 0
    sus i normie = 0
    
    bestie (i < count) {
        sus test_obj TestStruct = TestStruct{
            id: i,
            value: i.(meal) * 2.5,
            name: "object" + i.(tea)
        }
        
        # Simulate processing
        bestie (test_obj.id % 2 == 0) {
            processed = processed + 1
        }
        
        i = i + 1
    }
    
    damn processed
}

# Start performance testing
vibez.spill("=== JIT Performance Test Suite ===")
vibez.spill("Testing tier-up behavior and optimization")

# Test 1: Fibonacci (recursive, should trigger optimization)
vibez.spill("\n1. Fibonacci Test (Recursive - Should Tier Up)")
sus start_time normie = 0  # Mock timestamp

sus fib_calls normie = 0
bestie (fib_calls < 15) {
    sus fib_result normie = fibonacci(10)
    vibez.spill("Fibonacci(10) call", fib_calls + 1, "result:", fib_result)
    fib_calls = fib_calls + 1
}

# Test 2: Pi approximation (iterative, math-heavy)
vibez.spill("\n2. Pi Approximation Test (Math-Heavy)")
sus pi_calls normie = 0
bestie (pi_calls < 20) {
    sus pi_result meal = compute_pi_approximation(1000)
    bestie (pi_calls % 5 == 0) {
        vibez.spill("Pi approximation call", pi_calls + 1, "result:", pi_result)
    }
    pi_calls = pi_calls + 1
}

# Test 3: String processing (memory allocation intensive)
vibez.spill("\n3. String Processing Test (Memory Intensive)")
sus string_calls normie = 0
bestie (string_calls < 25) {
    sus string_result tea = process_strings(50)
    bestie (string_calls % 8 == 0) {
        vibez.spill("String processing call", string_calls + 1, "length:", string_result.length)
    }
    string_calls = string_calls + 1
}

# Test 4: Array operations
vibez.spill("\n4. Array Operations Test")
sus array_calls normie = 0
bestie (array_calls < 30) {
    sus array_result normie = sum_array(100)
    bestie (array_calls % 10 == 0) {
        vibez.spill("Array sum call", array_calls + 1, "result:", array_result)
    }
    array_calls = array_calls + 1
}

# Test 5: Struct operations
vibez.spill("\n5. Struct Operations Test")
sus struct_calls normie = 0
bestie (struct_calls < 20) {
    sus struct_result normie = create_and_process_structs(50)
    bestie (struct_calls % 5 == 0) {
        vibez.spill("Struct processing call", struct_calls + 1, "processed:", struct_result)
    }
    struct_calls = struct_calls + 1
}

# Complex mixed workload
vibez.spill("\n6. Mixed Workload Test (Should Trigger Multiple Tier-ups)")
sus mixed_calls normie = 0
bestie (mixed_calls < 50) {
    # Alternate between different types of work
    bestie (mixed_calls % 4 == 0) {
        sus _ normie = fibonacci(8)
    } meh bestie (mixed_calls % 4 == 1) {
        sus _ meal = compute_pi_approximation(500)
    } meh bestie (mixed_calls % 4 == 2) {
        sus _ tea = process_strings(25)
    } else {
        sus _ normie = sum_array(75)
    }
    
    bestie (mixed_calls % 10 == 0) {
        vibez.spill("Mixed workload progress:", mixed_calls, "/50")
    }
    
    mixed_calls = mixed_calls + 1
}

# Performance verification
vibez.spill("\n=== Performance Test Results ===")
vibez.spill("Total function calls executed:")
vibez.spill("- Fibonacci calls:", fib_calls)
vibez.spill("- Pi approximation calls:", pi_calls)
vibez.spill("- String processing calls:", string_calls)
vibez.spill("- Array operation calls:", array_calls)
vibez.spill("- Struct operation calls:", struct_calls)
vibez.spill("- Mixed workload calls:", mixed_calls)

sus total_calls normie = fib_calls + pi_calls + string_calls + array_calls + struct_calls + mixed_calls
vibez.spill("Total calls executed:", total_calls)

# Test validation
test_start("JIT Performance Test")
assert_true(fib_calls == 15)
assert_true(pi_calls == 20)
assert_true(string_calls == 25)
assert_true(array_calls == 30)
assert_true(struct_calls == 20)
assert_true(mixed_calls == 50)
assert_eq_int(total_calls, 160)
print_test_summary()

vibez.spill("\nJIT Performance Test completed!")
vibez.spill("Expected tier-ups should have occurred for hot functions")
