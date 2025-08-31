vibe main
yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "collections"

// Stress test for deeply nested operations and complex expressions
// Tests parser limits, expression evaluation, and memory management
// Expected: Successful evaluation of complex nested operations

slay deeply_nested_calculation(depth) {
    sus if depth <= 0 {
        return 1
    }
    
    sus result normie = mathz.multiply(depth, deeply_nested_calculation(mathz.subtract(depth, 1)))
    return result
}

slay nested_string_operations(base_string, count) {
    sus if count <= 0 {
        return base_string
    }
    
    sus numbered_suffix tea = stringz.from_number(count)
    sus new_string tea = stringz.concat(base_string, numbered_suffix)
    return nested_string_operations(new_string, mathz.subtract(count, 1))
}

slay complex_array_processing(arr, operations) {
    sus length normie = collections.length(arr)
    sus i normie = 0
    
    sus while i < length {
        sus element normie = collections.get(arr, i)
        
        // Nested mathematical operations
        sus processed normie = mathz.add(
            mathz.multiply(
                mathz.power(element, 2),
                3
            ),
            mathz.divide(
                mathz.add(element, 10),
                2
            )
        )
        
        collections.set(arr, i, processed)
        i = mathz.add(i, 1)
    }
    
    return arr
}

slay main_character() {
    vibez.spill("=== DEEP NESTED OPERATIONS STRESS TEST ===")
    
    // Test deeply nested arithmetic expressions
    sus complex_expr normie = mathz.add(
        mathz.multiply(
            mathz.add(
                mathz.power(2, 3),
                mathz.multiply(4, 5)
            ),
            mathz.subtract(
                mathz.divide(100, 5),
                mathz.mod(17, 3)
            )
        ),
        mathz.abs(
            mathz.subtract(
                mathz.multiply(-5, 4),
                mathz.add(10, 15)
            )
        )
    )
    
    vibez.spill("Complex nested expression result:")
    vibez.spill(complex_expr)
    
    // Test recursive factorial calculation
    sus factorial_result normie = deeply_nested_calculation(6)
    vibez.spill("Factorial of 6:")
    vibez.spill(factorial_result)
    
    // Test nested string building
    sus nested_string normie = nested_string_operations("Level", 5)
    vibez.spill("Nested string building:")
    vibez.spill(nested_string)
    
    // Test complex array operations
    sus test_array []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus processed_array normie = complex_array_processing(test_array, 3)
    
    vibez.spill("Processed array sample:")
    sus sample_element normie = collections.get(processed_array, 0)
    vibez.spill(sample_element)
    
    // Test nested conditional expressions
    sus nested_condition normie = (complex_expr > 100) && (factorial_result > 500) && (collections.length(processed_array) == 10)
    vibez.spill("Complex boolean evaluation:")
    vibez.spill(nested_condition)
    
    // Test deeply nested function calls
    sus final_result normie = mathz.power(
        mathz.add(
            deeply_nested_calculation(4),
            collections.length(processed_array)
        ),
        2
    )
    
    vibez.spill("Final nested calculation:")
    vibez.spill(final_result)
    
    vibez.spill("=== STRESS TEST COMPLETE ===")
}
