fr fr Comprehensive memory leak stress test
yeet "testz"

slay memory_intensive_function() {
    fr fr Create many variables and complex expressions
    sus counter drip = 0
    
    bestie i := 0; i < 1000; i = i + 1 {
        sus nested_counter drip = i * 2
        sus string_value tea = "test_string_" + i
        sus float_value meal = i * 3.14159
        counter = counter + nested_counter
        
        fr fr Nested loops with more allocations
        bestie j := 0; j < 100; j = j + 1 {
            sus temp_value drip = i + j
            counter = counter + temp_value
        }
    }
    
    damn counter
}

slay complex_data_structures() {
    test_start("Complex Data Structure Test")
    
    fr fr Create arrays of arrays
    sus nested_arrays [][]normie = []
    
    bestie i := 0; i < 50; i = i + 1 {
        sus inner_array []normie = []
        bestie j := 0; j < 100; j = j + 1 {
            inner_array.push(i * j)
        }
        nested_arrays.push(inner_array)
    }
    
    fr fr String concatenation stress test
    sus large_concatenated_string tea = ""
    bestie i := 0; i < 500; i = i + 1 {
        large_concatenated_string = large_concatenated_string + "segment_" + i + "_data_"
    }
    
    assert_true(large_concatenated_string.len() > 5000)
    
    fr fr Function calls with return values
    sus result drip = memory_intensive_function()
    assert_true(result > 0)
    
    print_test_summary()
}

fr fr Run multiple iterations to stress memory
complex_data_structures()
complex_data_structures()
complex_data_structures()
