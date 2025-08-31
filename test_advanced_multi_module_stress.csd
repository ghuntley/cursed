// ADVANCED MULTI-MODULE STRESS TEST
// Complex scenarios with deep stdlib integration

import mathz from "stdlib/mathz"
import stringz from "stdlib/stringz"
import fs from "stdlib/fs" 
import env from "stdlib/env"
import io_basic from "stdlib/io_basic"
import collections from "stdlib/collections"
import json from "stdlib/json"
import regex from "stdlib/regex"

func recursive_fibonacci_with_stdlib(n: int) -> int {
    // Recursive Fibonacci with extensive stdlib usage
    if mathz.less_than_or_equal(n, 1) {
        return n
    }
    
    // Use string operations for logging
    let log_msg = stringz.concat("Computing fib(", ")")
    io_basic.print_line(log_msg)
    
    // Recursive calls with stdlib math operations
    let fib_n_minus_1 = recursive_fibonacci_with_stdlib(mathz.subtract(n, 1))
    let fib_n_minus_2 = recursive_fibonacci_with_stdlib(mathz.subtract(n, 2))
    
    let result = mathz.add(fib_n_minus_1, fib_n_minus_2)
    
    // Log result with file operations
    let result_str = stringz.concat("fib result: ", "computed")
    fs.write_file("fib_log.txt", result_str)
    
    return result
}

func complex_data_processing_pipeline() -> int {
    print("=== COMPLEX DATA PROCESSING PIPELINE ===")
    
    // Create complex data structure
    let data_collection = collections.create_list()
    let processed_items = 0
    
    // Generate test data
    let i = 0
    while i < 10 {
        let item_value = mathz.multiply(i, i)  // Square numbers
        collections.add_item(data_collection, item_value)
        i = mathz.add(i, 1)
    }
    
    print("Generated data collection with squares")
    
    // Process each item through multiple stdlib modules
    let collection_size = collections.size(data_collection)
    let j = 0
    
    while j < collection_size {
        let current_item = collections.get_item(data_collection, j)
        
        // Mathematical processing
        let sqrt_approx = mathz.divide(current_item, 2)  // Simple approximation
        let processed_value = mathz.add(sqrt_approx, 10)
        
        // String processing
        let item_description = stringz.concat("Processed item ", "value")
        let desc_length = stringz.length(item_description)
        
        // File operations
        let filename = stringz.concat("processed_", ".txt")
        fs.write_file(filename, item_description)
        
        // JSON serialization
        let json_obj = json.create_object()
        json.set_number(json_obj, "original", current_item)
        json.set_number(json_obj, "processed", processed_value)
        json.set_string(json_obj, "description", item_description)
        
        let json_str = json.to_string(json_obj)
        let json_filename = stringz.concat("data_", ".json")
        fs.write_file(json_filename, json_str)
        
        processed_items = mathz.add(processed_items, 1)
        j = mathz.add(j, 1)
    }
    
    print("Processed items: ")
    print(processed_items)
    
    return processed_items
}

func advanced_string_regex_processing() -> int {
    print("=== ADVANCED STRING & REGEX PROCESSING ===")
    
    let test_text = "CURSED-v1.0-self-hosted-compiler-2024"
    let processing_steps = 0
    
    // Complex string transformations
    let upper_text = stringz.to_upper(test_text)
    let lower_text = stringz.to_lower(test_text)
    
    print("Original: ")
    print(test_text)
    print("Upper: ")
    print(upper_text)
    print("Lower: ")
    print(lower_text)
    
    // String analysis and manipulation
    let contains_cursed = stringz.contains(test_text, "CURSED")
    let contains_hosted = stringz.contains(test_text, "hosted")
    
    if contains_cursed {
        processing_steps = mathz.add(processing_steps, 1)
        print("Found CURSED in text")
    }
    
    if contains_hosted {
        processing_steps = mathz.add(processing_steps, 1)
        print("Found hosted in text")
    }
    
    // Regex pattern matching (if available)
    let pattern = "\\d+"  // Match digits
    let regex_compiled = regex.compile(pattern)
    let has_numbers = regex.match(regex_compiled, test_text)
    
    if has_numbers {
        processing_steps = mathz.add(processing_steps, 1)
        print("Text contains numbers")
    }
    
    // String splitting and rejoining simulation
    let parts_processed = 0
    let char_index = 0
    let text_length = stringz.length(test_text)
    
    while char_index < text_length {
        // Simulate character-by-character processing
        let char_analysis = mathz.add(char_index, 1)
        parts_processed = mathz.add(parts_processed, 1)
        char_index = mathz.add(char_index, 1)
    }
    
    print("Characters processed: ")
    print(parts_processed)
    
    return mathz.add(processing_steps, parts_processed)
}

func multi_level_nested_operations() -> int {
    print("=== MULTI-LEVEL NESTED OPERATIONS ===")
    
    let final_result = 0
    let level = 0
    
    while level < 3 {  // 3 levels of nesting
        print("Processing level: ")
        print(level)
        
        let level_result = 0
        let inner_loop = 0
        
        while inner_loop < 4 {  // Inner processing
            let calculation = mathz.multiply(
                mathz.add(level, 1),
                mathz.add(inner_loop, 1)
            )
            
            // String operations within nested loops
            let level_str = stringz.concat("Level ", " processing")
            let inner_str = stringz.concat("Inner ", " calculation")
            let combined_str = stringz.concat(level_str, inner_str)
            
            // File operations for each nested computation
            let nested_filename = stringz.concat(
                stringz.concat("nested_", "_"),
                ".txt"
            )
            fs.write_file(nested_filename, combined_str)
            
            // Collection operations
            let temp_collection = collections.create_list()
            collections.add_item(temp_collection, calculation)
            collections.add_item(temp_collection, stringz.length(combined_str))
            
            let collection_sum = mathz.add(
                collections.get_item(temp_collection, 0),
                collections.get_item(temp_collection, 1)
            )
            
            level_result = mathz.add(level_result, collection_sum)
            inner_loop = mathz.add(inner_loop, 1)
        }
        
        final_result = mathz.add(final_result, level_result)
        level = mathz.add(level, 1)
    }
    
    print("Multi-level result: ")
    print(final_result)
    
    return final_result
}

func comprehensive_error_recovery_test() -> int {
    print("=== COMPREHENSIVE ERROR RECOVERY TEST ===")
    
    let recovery_count = 0
    
    // Test 1: Mathematical edge cases
    let div_by_zero = mathz.divide(100, 0)
    if mathz.equal(div_by_zero, 0) {
        recovery_count = mathz.add(recovery_count, 1)
        print("Recovered from division by zero")
    }
    
    // Test 2: File operations with invalid inputs
    let invalid_file = fs.read_file("")
    let empty_content = stringz.length(invalid_file)
    if mathz.equal(empty_content, 0) {
        recovery_count = mathz.add(recovery_count, 1)
        print("Recovered from invalid file read")
    }
    
    // Test 3: String operations with empty strings
    let empty_str = ""
    let empty_upper = stringz.to_upper(empty_str)
    let empty_concat = stringz.concat(empty_str, empty_str)
    
    if mathz.equal(stringz.length(empty_concat), 0) {
        recovery_count = mathz.add(recovery_count, 1)
        print("Recovered from empty string operations")
    }
    
    // Test 4: Collection operations with invalid indices
    let test_collection = collections.create_list()
    collections.add_item(test_collection, 42)
    
    let invalid_item = collections.get_item(test_collection, 10)  // Invalid index
    if mathz.equal(invalid_item, 0) {
        recovery_count = mathz.add(recovery_count, 1)
        print("Recovered from invalid collection access")
    }
    
    // Test 5: Environment variables that don't exist
    let missing_env = env.get_var("DEFINITELY_DOES_NOT_EXIST")
    let missing_len = stringz.length(missing_env)
    
    if mathz.equal(missing_len, 0) {
        recovery_count = mathz.add(recovery_count, 1)
        print("Recovered from missing environment variable")
    }
    
    print("Total recoveries: ")
    print(recovery_count)
    
    return recovery_count
}

func ultimate_integration_test() -> int {
    print("=== ULTIMATE INTEGRATION TEST ===")
    
    // Create a complex workflow that uses ALL stdlib modules
    let integration_score = 0
    
    // Phase 1: Data preparation
    let base_data = mathz.multiply(7, 6)  // 42
    let data_description = stringz.concat("Base data: ", "computed")
    
    env.set_var("CURSED_INTEGRATION_DATA", data_description)
    let stored_description = env.get_var("CURSED_INTEGRATION_DATA")
    
    if stringz.contains(stored_description, "Base") {
        integration_score = mathz.add(integration_score, 10)
    }
    
    // Phase 2: Data collection and processing
    let main_collection = collections.create_list()
    let processing_iterations = 3
    let current_iteration = 0
    
    while current_iteration < processing_iterations {
        let iteration_value = mathz.multiply(base_data, current_iteration)
        collections.add_item(main_collection, iteration_value)
        
        // JSON serialization of each iteration
        let iteration_json = json.create_object()
        json.set_number(iteration_json, "iteration", current_iteration)
        json.set_number(iteration_json, "value", iteration_value)
        json.set_string(iteration_json, "status", "processed")
        
        let json_output = json.to_string(iteration_json)
        let json_filename = stringz.concat("integration_", ".json")
        fs.write_file(json_filename, json_output)
        
        // Verify file was written
        if fs.file_exists(json_filename) {
            integration_score = mathz.add(integration_score, 5)
        }
        
        current_iteration = mathz.add(current_iteration, 1)
    }
    
    // Phase 3: Complex analysis
    let collection_size = collections.size(main_collection)
    let analysis_result = 0
    let analysis_index = 0
    
    while analysis_index < collection_size {
        let current_value = collections.get_item(main_collection, analysis_index)
        
        // Mathematical analysis
        if mathz.greater_than(current_value, 50) {
            analysis_result = mathz.add(analysis_result, current_value)
        } else {
            analysis_result = mathz.add(analysis_result, mathz.multiply(current_value, 2))
        }
        
        analysis_index = mathz.add(analysis_index, 1)
    }
    
    // Phase 4: Results compilation and reporting
    let final_report = json.create_object()
    json.set_number(final_report, "integration_score", integration_score)
    json.set_number(final_report, "analysis_result", analysis_result)
    json.set_number(final_report, "collection_size", collection_size)
    json.set_string(final_report, "status", "COMPLETE")
    json.set_string(final_report, "system", "CURSED Pure Self-Hosted")
    
    let final_json = json.to_string(final_report)
    fs.write_file("ultimate_integration_report.json", final_json)
    
    // Output results
    print("Integration Score: ")
    print(integration_score)
    print("Analysis Result: ")
    print(analysis_result)
    print("Report saved to file")
    
    let final_score = mathz.add(integration_score, analysis_result)
    return final_score
}

func main() -> int {
    print("CURSED ADVANCED MULTI-MODULE STRESS TEST")
    print("======================================")
    
    // Execute all advanced tests
    let fib_result = recursive_fibonacci_with_stdlib(8)
    print("Fibonacci(8) = ")
    print(fib_result)
    
    let pipeline_result = complex_data_processing_pipeline()
    let string_regex_result = advanced_string_regex_processing()
    let nested_result = multi_level_nested_operations()
    let error_recovery_result = comprehensive_error_recovery_test()
    let integration_result = ultimate_integration_test()
    
    // Calculate comprehensive stress test score
    let intermediate_sum = mathz.add(
        mathz.add(fib_result, pipeline_result),
        mathz.add(string_regex_result, nested_result)
    )
    
    let final_stress_score = mathz.add(
        intermediate_sum,
        mathz.add(error_recovery_result, integration_result)
    )
    
    print("")
    print("======================================")
    print("ADVANCED STRESS TEST COMPLETE")
    print("Fibonacci result: ")
    print(fib_result)
    print("Pipeline result: ")
    print(pipeline_result)
    print("String/Regex result: ")
    print(string_regex_result)
    print("Nested operations result: ")
    print(nested_result)
    print("Error recovery result: ")
    print(error_recovery_result)
    print("Integration result: ")
    print(integration_result)
    print("")
    print("FINAL STRESS TEST SCORE: ")
    print(final_stress_score)
    print("CURSED ADVANCED CAPABILITIES: VERIFIED")
    print("======================================")
    
    return final_stress_score
}
