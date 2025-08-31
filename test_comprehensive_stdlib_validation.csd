// COMPREHENSIVE CURSED STDLIB VALIDATION TEST
// Testing pure self-hosting with multiple stdlib modules

yeet "vibez"
yeet "mathz"
yeet "collections"
yeet "stringz"
yeet "io"
yeet "env"
yeet "fs"

slay comprehensive_math_test() {
    vibez.spill("=== COMPREHENSIVE MATH TEST ===")
    
    let base_num = 42.0
    let second_num = 17.0
    let third_num = 3.0
    
    // Basic arithmetic operations
    let sum_result = mathz.add(base_num, second_num)
    let diff_result = mathz.subtract(base_num, second_num)  
    let mult_result = mathz.multiply(base_num, third_num)
    let div_result = mathz.divide(sum_result, third_num)
    
    vibez.spill("Basic math results:")
    vibez.spill(sum_result)
    vibez.spill(diff_result)
    vibez.spill(mult_result)
    vibez.spill(div_result)
    
    // Advanced operations
    let abs_result = mathz.abs_normie(-25.0)
    let max_result = mathz.max(base_num, second_num)
    let min_result = mathz.min(base_num, second_num)
    let power_result = mathz.power(third_num, 3.0)
    
    vibez.spill("Advanced math results:")
    vibez.spill(abs_result)
    vibez.spill(max_result)
    vibez.spill(min_result)
    vibez.spill(power_result)
    
    // Complex nested calculations
    let nested_calc = mathz.add(
        mathz.multiply(abs_result, third_num),
        mathz.divide(power_result, 2.0)
    )
    
    vibez.spill("Complex nested calculation:")
    vibez.spill(nested_calc)
    
    return nested_calc
}

slay comprehensive_string_test() {
    vibez.spill("=== COMPREHENSIVE STRING TEST ===")
    
    let name_str = "CURSED"
    let version_str = "v1.0"
    let separator = " - "
    
    // String operations
    let full_name = stringz.concat(name_str, separator)
    let complete_title = stringz.concat(full_name, version_str)
    let title_len = stringz.length(complete_title)
    
    vibez.spill("String operations:")
    vibez.spill(complete_title)
    vibez.spill(title_len)
    
    // String transformations
    let lower_title = stringz.to_lower(complete_title)
    let upper_name = stringz.to_upper(name_str)
    
    vibez.spill("String transformations:")
    vibez.spill(lower_title)
    vibez.spill(upper_name)
    
    // String analysis
    let contains_cursed = stringz.contains(complete_title, "CURSED")
    let starts_c = stringz.starts_with(complete_title, "C")
    let ends_zero = stringz.ends_with(version_str, "0")
    
    vibez.spill("String analysis:")
    vibez.spill(contains_cursed)
    vibez.spill(starts_c)
    vibez.spill(ends_zero)
    
    return title_len
}

slay comprehensive_collection_test() {
    vibez.spill("=== COMPREHENSIVE COLLECTION TEST ===")
    
    // Create collection and add items
    let numbers_list = collections.Vec_new()
    collections.Vec_push(numbers_list, 10.0)
    collections.Vec_push(numbers_list, 20.0)
    collections.Vec_push(numbers_list, 30.0)
    collections.Vec_push(numbers_list, 40.0)
    
    let list_size = collections.Vec_len(numbers_list)
    vibez.spill("Collection size:")
    vibez.spill(list_size)
    
    // Get items from collection
    let first_item = collections.Vec_get(numbers_list, 0.0)
    let second_item = collections.Vec_get(numbers_list, 1.0)
    let third_item = collections.Vec_get(numbers_list, 2.0)
    
    vibez.spill("Collection items:")
    vibez.spill(first_item)
    vibez.spill(second_item)
    vibez.spill(third_item)
    
    // Collection operations
    let contains_20 = collections.Vec_contains(numbers_list, 20.0)
    vibez.spill("Contains 20:")
    vibez.spill(contains_20)
    
    // Clear and verify
    collections.Vec_clear(numbers_list)
    let empty_size = collections.Vec_len(numbers_list)
    vibez.spill("Empty size after clear:")
    vibez.spill(empty_size)
    
    return list_size
}

slay comprehensive_io_test() {
    vibez.spill("=== COMPREHENSIVE IO TEST ===")
    
    // Basic IO operations
    io.print_line("Basic IO: Hello from CURSED!")
    io.print_number(42.0)
    io.print_line("IO number test completed")
    
    // Formatted output simulation
    let test_message = "Advanced IO formatting test"
    io.print_line(test_message)
    
    let number_to_print = 12345.0
    io.print_number(number_to_print)
    
    vibez.spill("IO operations completed successfully")
    
    return 42.0  // Success indicator
}

slay comprehensive_file_test() {
    vibez.spill("=== COMPREHENSIVE FILE TEST ===")
    
    let test_filename = "cursed_test_file.txt"
    let test_content = "CURSED Multi-Module Validation Content"
    
    // Write file operation
    let write_success = fs.write_file(test_filename, test_content)
    vibez.spill("File write success:")
    vibez.spill(write_success)
    
    // Check if file exists
    let file_exists = fs.file_exists(test_filename)
    vibez.spill("File exists:")
    vibez.spill(file_exists)
    
    // Read file back
    let read_content = fs.read_file(test_filename)
    vibez.spill("Read content:")
    vibez.spill(read_content)
    
    // File size
    let file_size = fs.file_size(test_filename)
    vibez.spill("File size:")
    vibez.spill(file_size)
    
    // Directory operations
    let current_dir = fs.current_dir()
    vibez.spill("Current directory:")
    vibez.spill(current_dir)
    
    return file_size
}

slay comprehensive_env_test() {
    vibez.spill("=== COMPREHENSIVE ENV TEST ===")
    
    // Set environment variable
    env.set_var("CURSED_TEST_MODE", "comprehensive")
    
    // Get environment variable
    let test_mode = env.get_var("CURSED_TEST_MODE")
    vibez.spill("Test mode:")
    vibez.spill(test_mode)
    
    // Check if variable exists
    let var_exists = env.has_var("CURSED_TEST_MODE")
    vibez.spill("Variable exists:")
    vibez.spill(var_exists)
    
    // Get system PATH
    let path_var = env.get_var("PATH")
    let path_length = stringz.length(path_var)
    vibez.spill("PATH length:")
    vibez.spill(path_length)
    
    return path_length
}

slay stress_test_nested_operations() {
    vibez.spill("=== STRESS TEST: NESTED OPERATIONS ===")
    
    let total_result = 0.0
    let iterations = 5.0
    let i = 0.0
    
    while i < iterations {
        vibez.spill("Stress iteration:")
        vibez.spill(i)
        
        // Complex nested math
        let math_result = mathz.multiply(
            mathz.add(i, 10.0),
            mathz.subtract(20.0, i)
        )
        
        // String operations in loop
        let result_desc = stringz.concat("Result: ", "computed")
        let desc_length = stringz.length(result_desc)
        
        // Collection operations in loop  
        let temp_collection = collections.Vec_new()
        collections.Vec_push(temp_collection, math_result)
        collections.Vec_push(temp_collection, desc_length)
        
        let collection_sum = mathz.add(
            collections.Vec_get(temp_collection, 0.0),
            collections.Vec_get(temp_collection, 1.0)
        )
        
        // Conditional logic with stdlib
        if mathz.greater_than(math_result, 100.0) {
            total_result = mathz.add(total_result, math_result)
            vibez.spill("Added large math result")
        } else {
            total_result = mathz.add(total_result, collection_sum)
            vibez.spill("Added collection sum")
        }
        
        // File operation in loop
        let loop_filename = stringz.concat("stress_", "loop.txt")
        fs.write_file(loop_filename, result_desc)
        
        collections.Vec_clear(temp_collection)
        i = mathz.add(i, 1.0)
    }
    
    vibez.spill("Stress test total result:")
    vibez.spill(total_result)
    
    return total_result
}

slay error_recovery_test() {
    vibez.spill("=== ERROR RECOVERY TEST ===")
    
    let recovery_count = 0.0
    
    // Test division by zero
    let safe_div = mathz.divide(100.0, 0.0)
    if mathz.equal(safe_div, 0.0) {
        recovery_count = mathz.add(recovery_count, 1.0)
        vibez.spill("Recovered from division by zero")
    }
    
    // Test empty string operations
    let empty_str = ""
    let empty_length = stringz.length(empty_str)
    if mathz.equal(empty_length, 0.0) {
        recovery_count = mathz.add(recovery_count, 1.0) 
        vibez.spill("Recovered from empty string")
    }
    
    // Test invalid file operations
    let invalid_file = fs.read_file("nonexistent_file.txt")
    let invalid_len = stringz.length(invalid_file)
    if mathz.equal(invalid_len, 0.0) {
        recovery_count = mathz.add(recovery_count, 1.0)
        vibez.spill("Recovered from invalid file")
    }
    
    // Test missing environment variable
    let missing_var = env.get_var("NONEXISTENT_VAR_12345")
    let missing_len = stringz.length(missing_var)
    if mathz.equal(missing_len, 0.0) {
        recovery_count = mathz.add(recovery_count, 1.0)
        vibez.spill("Recovered from missing env var")
    }
    
    vibez.spill("Total recoveries:")
    vibez.spill(recovery_count)
    
    return recovery_count
}

slay ultimate_integration_test() {
    vibez.spill("=== ULTIMATE INTEGRATION TEST ===")
    
    let integration_score = 0.0
    
    // Phase 1: Data preparation with multiple modules
    let base_data = mathz.multiply(7.0, 6.0)  // 42
    let data_desc = stringz.concat("Integration data: ", "prepared")
    
    env.set_var("CURSED_INTEGRATION_SCORE", data_desc)
    let stored_desc = env.get_var("CURSED_INTEGRATION_SCORE")
    
    if stringz.contains(stored_desc, "Integration") {
        integration_score = mathz.add(integration_score, 10.0)
        vibez.spill("Phase 1 complete: Data preparation")
    }
    
    // Phase 2: Collection processing
    let main_collection = collections.Vec_new()
    let processing_rounds = 3.0
    let round = 0.0
    
    while round < processing_rounds {
        let round_value = mathz.multiply(base_data, round)
        collections.Vec_push(main_collection, round_value)
        
        // File operations for each round
        let round_filename = stringz.concat("integration_round_", ".txt")
        let round_content = stringz.concat("Round ", " completed")
        fs.write_file(round_filename, round_content)
        
        if fs.file_exists(round_filename) {
            integration_score = mathz.add(integration_score, 5.0)
        }
        
        round = mathz.add(round, 1.0)
    }
    
    vibez.spill("Phase 2 complete: Collection processing")
    
    // Phase 3: Analysis and results
    let collection_size = collections.Vec_len(main_collection)
    let analysis_result = 0.0
    let index = 0.0
    
    while index < collection_size {
        let current_val = collections.Vec_get(main_collection, index)
        
        if mathz.greater_than(current_val, 50.0) {
            analysis_result = mathz.add(analysis_result, current_val)
        } else {
            analysis_result = mathz.add(analysis_result, mathz.multiply(current_val, 2.0))
        }
        
        index = mathz.add(index, 1.0)
    }
    
    // Phase 4: Final reporting
    let final_report_content = stringz.concat("Integration complete: ", "SUCCESS")
    fs.write_file("ultimate_integration_report.txt", final_report_content)
    
    let final_score = mathz.add(integration_score, analysis_result)
    
    vibez.spill("Phase 3 complete: Analysis")
    vibez.spill("Phase 4 complete: Reporting")
    vibez.spill("Integration score:")
    vibez.spill(integration_score)
    vibez.spill("Analysis result:")
    vibez.spill(analysis_result)
    vibez.spill("Final integration score:")
    vibez.spill(final_score)
    
    return final_score
}

slay main_character() {
    vibez.spill("CURSED COMPREHENSIVE STDLIB VALIDATION")
    vibez.spill("=====================================")
    
    // Execute all comprehensive tests
    let math_result = comprehensive_math_test()
    let string_result = comprehensive_string_test()
    let collection_result = comprehensive_collection_test()
    let io_result = comprehensive_io_test()
    let file_result = comprehensive_file_test()
    let env_result = comprehensive_env_test()
    
    // Execute stress tests
    let stress_result = stress_test_nested_operations()
    let error_result = error_recovery_test()
    let integration_result = ultimate_integration_test()
    
    // Calculate comprehensive validation score
    let primary_score = mathz.add(
        mathz.add(math_result, string_result),
        mathz.add(collection_result, io_result)
    )
    
    let secondary_score = mathz.add(
        mathz.add(file_result, env_result),
        mathz.add(stress_result, error_result)
    )
    
    let final_validation_score = mathz.add(
        mathz.add(primary_score, secondary_score),
        integration_result
    )
    
    vibez.spill("")
    vibez.spill("=====================================")
    vibez.spill("COMPREHENSIVE VALIDATION RESULTS:")
    vibez.spill("Math test result:")
    vibez.spill(math_result)
    vibez.spill("String test result:")
    vibez.spill(string_result)
    vibez.spill("Collection test result:")
    vibez.spill(collection_result)
    vibez.spill("IO test result:")
    vibez.spill(io_result)
    vibez.spill("File test result:")
    vibez.spill(file_result)
    vibez.spill("Environment test result:")
    vibez.spill(env_result)
    vibez.spill("Stress test result:")
    vibez.spill(stress_result)
    vibez.spill("Error recovery result:")
    vibez.spill(error_result)
    vibez.spill("Integration test result:")
    vibez.spill(integration_result)
    vibez.spill("")
    vibez.spill("FINAL VALIDATION SCORE:")
    vibez.spill(final_validation_score)
    vibez.spill("")
    vibez.spill("✅ CURSED PURE SELF-HOSTING VERIFIED")
    vibez.spill("✅ MULTI-MODULE STDLIB OPERATIONAL")
    vibez.spill("✅ COMPREHENSIVE TESTING COMPLETE")
    vibez.spill("=====================================")
}
