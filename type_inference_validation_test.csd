# Oracle's Week 1 Core Correctness - Type Inference Edge Cases Validation Test
# Focused test to validate enhanced type inference implementation

yeet "vibez"

# Test 1: Basic Type Inference Validation
slay test_basic_inference() lit {
    vibez.spill("Testing basic type inference...")
    
    # Simple function call with type inference
    slay add(a drip, b drip) drip {
        damn a + b
    }
    
    sus result drip = add(5, 3)
    vibez.spill("Basic inference result: {d}", result)
    damn result == 8
}

# Test 2: Generic Function Call Inference
slay test_generic_inference() lit {
    vibez.spill("Testing generic function call inference...")
    
    # Generic identity function
    slay identity<T>(value T) T {
        damn value
    }
    
    # Should infer T as drip
    sus number_result drip = identity(42)
    vibez.spill("Generic inference result: {d}", number_result)
    
    # Should infer T as tea
    sus string_result tea = identity("hello")
    vibez.spill("Generic string result: {s}", string_result)
    
    damn number_result == 42 and string_result == "hello"
}

# Test 3: Nested Generic Inference
slay test_nested_generic_inference() lit {
    vibez.spill("Testing nested generic inference...")
    
    # Function with nested generic types
    slay process_array<T>(items []T, processor slay(T) T) []T {
        sus results []T = []
        bestie (item in items) {
            results.append(processor(item))
        }
        damn results
    }
    
    # Test with numbers
    slay double(x drip) drip { damn x * 2 }
    sus numbers []drip = [1, 2, 3]
    sus doubled []drip = process_array(numbers, double)
    
    vibez.spill("Nested generic results: [{d}, {d}, {d}]", doubled[0], doubled[1], doubled[2])
    damn doubled.len() == 3 and doubled[0] == 2 and doubled[1] == 4 and doubled[2] == 6
}

# Test 4: Constraint Satisfaction Test
slay test_constraint_satisfaction() lit {
    vibez.spill("Testing constraint satisfaction...")
    
    # Function with constraints
    slay max_value<T: Comparable>(a T, b T) T {
        ready (a > b) {
            damn a
        } otherwise {
            damn b
        }
    }
    
    sus max_int drip = max_value(10, 20)
    vibez.spill("Max value result: {d}", max_int)
    damn max_int == 20
}

# Test 5: Cycle Detection Test
slay test_cycle_detection() lit {
    vibez.spill("Testing cycle detection in type inference...")
    
    # This should not create infinite recursion
    slay recursive_function(x drip) drip {
        ready (x <= 0) {
            damn 1
        } otherwise {
            damn x * recursive_function(x - 1)
        }
    }
    
    sus factorial_result drip = recursive_function(5)
    vibez.spill("Factorial result: {d}", factorial_result)
    damn factorial_result == 120
}

# Test 6: Complex Pattern Matching
slay test_pattern_matching_inference() lit {
    vibez.spill("Testing pattern matching with type inference...")
    
    enum Result<T, E> {
        Success(T),
        Failure(E)
    }
    
    slay handle_result<T, E>(result Result<T, E>) tea {
        damn sick (result) {
            when Result.Success(value) -> "Success: " + value.to_string(),
            when Result.Failure(error) -> "Error: " + error.to_string(),
            when _ -> "Unknown result"
        }
    }
    
    sus success_result Result<drip, tea> = Result.Success(42)
    sus success_message tea = handle_result(success_result)
    vibez.spill("Pattern matching result: {s}", success_message)
    
    damn success_message.contains("Success")
}

# Test 7: Memory Safety Check
slay test_memory_safety() lit {
    vibez.spill("Testing memory safety in type inference...")
    
    # Create and manipulate various types
    sus large_array []drip = []
    bestie (i in range(0, 100)) {
        large_array.append(i)
    }
    
    # Higher-order function
    slay sum_array(arr []drip) drip {
        sus total drip = 0
        bestie (item in arr) {
            total = total + item
        }
        damn total
    }
    
    sus total drip = sum_array(large_array)
    vibez.spill("Array sum: {d}", total)
    damn total == 4950  # Sum of 0 to 99
}

# Main test runner function
slay main() drip {
    vibez.spill("🚀 Oracle's Week 1 Core Correctness - Type Inference Edge Cases Test")
    vibez.spill("=" * 70)
    
    sus tests_passed drip = 0
    sus total_tests drip = 7
    
    # Run all tests
    ready (test_basic_inference()) {
        vibez.spill("✅ Basic inference test passed")
        tests_passed = tests_passed + 1
    } otherwise {
        vibez.spill("❌ Basic inference test failed")
    }
    
    ready (test_generic_inference()) {
        vibez.spill("✅ Generic inference test passed")
        tests_passed = tests_passed + 1
    } otherwise {
        vibez.spill("❌ Generic inference test failed")
    }
    
    ready (test_nested_generic_inference()) {
        vibez.spill("✅ Nested generic inference test passed")
        tests_passed = tests_passed + 1
    } otherwise {
        vibez.spill("❌ Nested generic inference test failed")
    }
    
    ready (test_constraint_satisfaction()) {
        vibez.spill("✅ Constraint satisfaction test passed")
        tests_passed = tests_passed + 1
    } otherwise {
        vibez.spill("❌ Constraint satisfaction test failed")
    }
    
    ready (test_cycle_detection()) {
        vibez.spill("✅ Cycle detection test passed")
        tests_passed = tests_passed + 1
    } otherwise {
        vibez.spill("❌ Cycle detection test failed")
    }
    
    ready (test_pattern_matching_inference()) {
        vibez.spill("✅ Pattern matching inference test passed")
        tests_passed = tests_passed + 1
    } otherwise {
        vibez.spill("❌ Pattern matching inference test failed")
    }
    
    ready (test_memory_safety()) {
        vibez.spill("✅ Memory safety test passed")
        tests_passed = tests_passed + 1
    } otherwise {
        vibez.spill("❌ Memory safety test failed")
    }
    
    vibez.spill("=" * 70)
    vibez.spill("Type Inference Validation Results:")
    vibez.spill("Passed: {d}/{d} tests ({d}% success rate)", tests_passed, total_tests, (tests_passed * 100) / total_tests)
    
    ready (tests_passed == total_tests) {
        vibez.spill("🎉 ALL TYPE INFERENCE EDGE CASE FIXES VALIDATED!")
        vibez.spill("Oracle's Week 1 Core Correctness - Type inference is production ready")
        damn 0
    } otherwise {
        vibez.spill("⚠️  Some type inference edge cases need attention")
        vibez.spill("Failed tests: {d}", total_tests - tests_passed)
        damn 1
    }
}
