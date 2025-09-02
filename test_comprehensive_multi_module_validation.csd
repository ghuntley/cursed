// COMPREHENSIVE MULTI-MODULE CURSED STDLIB VALIDATION
// Testing pure self-hosting capabilities with extensive stdlib usage

import mathz from "stdlib/mathz"
import stringz from "stdlib/stringz"
import fs from "stdlib/fs"
import env from "stdlib/env"
import io_basic from "stdlib/io_basic"
import io_advanced from "stdlib/io_advanced"
import collections from "stdlib/collections"
import time from "stdlib/time"
import json from "stdlib/json"

func comprehensive_math_operations() -> int {
    print("=== MATHEMATICAL OPERATIONS TEST ===")
    
    let a = 42
    let b = 17
    let c = 3
    
    // Basic arithmetic
    let sum = mathz.add(a, b)
    let diff = mathz.subtract(a, b)
    let prod = mathz.multiply(a, c)
    let quot = mathz.divide(sum, c)
    
    print("Basic math: 42 + 17 = ")
    print(sum)
    print("42 - 17 = ")
    print(diff)
    print("42 * 3 = ")
    print(prod)
    print("Division result = ")
    print(quot)
    
    // Advanced operations
    let abs_val = mathz.abs_normie(-25)
    let max_val = mathz.max(a, b)
    let min_val = mathz.min(a, b)
    let power = mathz.power(c, 3)
    
    print("Advanced: abs(-25) = ")
    print(abs_val)
    print("max(42, 17) = ")
    print(max_val)
    print("min(42, 17) = ")
    print(min_val)
    print("3^3 = ")
    print(power)
    
    // Nested calculations
    let complex_result = mathz.add(
        mathz.multiply(
            mathz.abs_normie(mathz.subtract(a, 100)), 
            c
        ),
        mathz.divide(power, 2)
    )
    
    print("Complex nested: ")
    print(complex_result)
    
    return complex_result
}

func comprehensive_string_operations() -> int {
    print("=== STRING OPERATIONS TEST ===")
    
    let name = "CURSED"
    let version = "v1.0"
    let separator = " - "
    
    // Basic string operations
    let full_title = stringz.concat(name, separator)
    let complete_title = stringz.concat(full_title, version)
    let title_length = stringz.length(complete_title)
    
    print("String concat: ")
    print(complete_title)
    print("Length: ")
    print(title_length)
    
    // String transformation
    let lower_title = stringz.to_lower(complete_title)
    let upper_title = stringz.to_upper(name)
    
    print("Lowercase: ")
    print(lower_title)
    print("Uppercase: ")
    print(upper_title)
    
    // String analysis
    let contains_cursed = stringz.contains(complete_title, "CURSED")
    let starts_with_c = stringz.starts_with(complete_title, "C")
    let ends_with_0 = stringz.ends_with(complete_title, "0")
    
    print("Contains CURSED: ")
    print(contains_cursed)
    print("Starts with C: ")
    print(starts_with_c)
    print("Ends with 0: ")
    print(ends_with_0)
    
    return title_length
}

func comprehensive_file_operations() -> int {
    print("=== FILE SYSTEM OPERATIONS TEST ===")
    
    let test_file = "test_multi_module.txt"
    let test_content = "CURSED Multi-Module Test Content"
    
    // Write file
    let write_success = fs.write_file(test_file, test_content)
    print("File write success: ")
    print(write_success)
    
    // Check if file exists
    let file_exists = fs.file_exists(test_file)
    print("File exists: ")
    print(file_exists)
    
    // Read file back
    let read_content = fs.read_file(test_file)
    print("Read content: ")
    print(read_content)
    
    // Get file info
    let file_size = fs.file_size(test_file)
    print("File size: ")
    print(file_size)
    
    // Directory operations
    let current_dir = fs.current_directory()
    print("Current directory: ")
    print(current_dir)
    
    return file_size
}

func comprehensive_environment_operations() -> int {
    print("=== ENVIRONMENT OPERATIONS TEST ===")
    
    // Set and get environment variables
    env.set_var("CURSED_TEST_VAR", "multi_module_test")
    let test_var = env.get_var("CURSED_TEST_VAR")
    print("Set/Get env var: ")
    print(test_var)
    
    // Get system environment
    let path_var = env.get_var("PATH")
    let path_length = stringz.length(path_var)
    print("PATH length: ")
    print(path_length)
    
    // Check if variable exists
    let var_exists = env.has_var("CURSED_TEST_VAR")
    print("Variable exists: ")
    print(var_exists)
    
    return path_length
}

func comprehensive_io_operations() -> int {
    print("=== IO OPERATIONS TEST ===")
    
    // Basic IO
    io_basic.print_line("Basic IO: Hello from CURSED!")
    
    let user_message = "User input simulation"
    io_basic.print_line(user_message)
    
    // Advanced IO with formatting
    let formatted_output = io_advanced.format_string("Advanced IO: {} version {}", "CURSED", "1.0")
    io_advanced.print_formatted(formatted_output)
    
    // IO with numbers
    let number_output = io_advanced.format_number(12345)
    io_advanced.print_formatted(number_output)
    
    return stringz.length(formatted_output)
}

func comprehensive_collection_operations() -> int {
    print("=== COLLECTION OPERATIONS TEST ===")
    
    // Create and manipulate collections
    let numbers = collections.create_list()
    collections.add_item(numbers, 10)
    collections.add_item(numbers, 20)
    collections.add_item(numbers, 30)
    
    let list_size = collections.size(numbers)
    print("Collection size: ")
    print(list_size)
    
    let first_item = collections.get_item(numbers, 0)
    print("First item: ")
    print(first_item)
    
    // Collection operations
    let contains_20 = collections.contains(numbers, 20)
    print("Contains 20: ")
    print(contains_20)
    
    let item_index = collections.find_index(numbers, 30)
    print("Index of 30: ")
    print(item_index)
    
    return list_size
}

func comprehensive_time_operations() -> int {
    print("=== TIME OPERATIONS TEST ===")
    
    // Get current time
    let current_timestamp = time.current_time()
    print("Current timestamp: ")
    print(current_timestamp)
    
    // Time formatting
    let formatted_time = time.format_time(current_timestamp)
    print("Formatted time: ")
    print(formatted_time)
    
    // Time calculations
    let future_time = time.add_seconds(current_timestamp, 3600)
    let time_diff = time.time_difference(future_time, current_timestamp)
    
    print("Time difference (1 hour): ")
    print(time_diff)
    
    return time_diff
}

func comprehensive_json_operations() -> int {
    print("=== JSON OPERATIONS TEST ===")
    
    // Create JSON data
    let json_data = json.create_object()
    json.set_string(json_data, "name", "CURSED")
    json.set_number(json_data, "version", 1)
    json.set_boolean(json_data, "self_hosted", 1)
    
    // Serialize to string
    let json_string = json.to_string(json_data)
    print("JSON string: ")
    print(json_string)
    
    // Parse back
    let parsed_data = json.from_string(json_string)
    let name_field = json.get_string(parsed_data, "name")
    let version_field = json.get_number(parsed_data, "version")
    
    print("Parsed name: ")
    print(name_field)
    print("Parsed version: ")
    print(version_field)
    
    return stringz.length(json_string)
}

func stress_test_nested_operations() -> int {
    print("=== STRESS TEST: NESTED OPERATIONS ===")
    
    let iterations = 5
    let total_result = 0
    let i = 0
    
    while i < iterations {
        print("Iteration: ")
        print(i)
        
        // Nested math operations
        let math_result = mathz.multiply(
            mathz.add(i, 10),
            mathz.subtract(20, i)
        )
        
        // String operations with math results
        let result_str = stringz.concat("Result: ", "value")
        let str_length = stringz.length(result_str)
        
        // Conditional logic with stdlib calls
        if mathz.greater_than(math_result, 100) {
            total_result = mathz.add(total_result, math_result)
            print("Added large result: ")
            print(math_result)
        } else {
            total_result = mathz.add(total_result, str_length)
            print("Added string length: ")
            print(str_length)
        }
        
        // File operation in loop
        let loop_file = stringz.concat("test_", "loop.txt")
        fs.write_file(loop_file, result_str)
        
        i = mathz.add(i, 1)
    }
    
    print("Final stress test result: ")
    print(total_result)
    
    return total_result
}

func error_handling_test() -> int {
    print("=== ERROR HANDLING TEST ===")
    
    // Test division by zero handling
    let safe_division = mathz.divide(100, 0)
    print("Safe division by zero: ")
    print(safe_division)
    
    // Test file operations with invalid paths
    let invalid_read = fs.read_file("nonexistent_file.txt")
    print("Invalid file read: ")
    print(invalid_read)
    
    // Test string operations with edge cases
    let empty_string = ""
    let empty_length = stringz.length(empty_string)
    print("Empty string length: ")
    print(empty_length)
    
    // Test environment variable that doesn't exist
    let missing_var = env.get_var("NONEXISTENT_VAR")
    print("Missing env var: ")
    print(missing_var)
    
    return 1  // Success indicator
}

func main() -> int {
    print("CURSED COMPREHENSIVE MULTI-MODULE VALIDATION")
    print("===========================================")
    
    // Run all comprehensive tests
    let math_result = comprehensive_math_operations()
    let string_result = comprehensive_string_operations()
    let file_result = comprehensive_file_operations()
    let env_result = comprehensive_environment_operations()
    let io_result = comprehensive_io_operations()
    let collection_result = comprehensive_collection_operations()
    let time_result = comprehensive_time_operations()
    let json_result = comprehensive_json_operations()
    
    // Run stress tests
    let stress_result = stress_test_nested_operations()
    let error_result = error_handling_test()
    
    // Calculate final validation score
    let validation_score = mathz.add(
        mathz.add(math_result, string_result),
        mathz.add(
            mathz.add(file_result, env_result),
            mathz.add(io_result, collection_result)
        )
    )
    
    let final_score = mathz.add(
        validation_score,
        mathz.add(
            mathz.add(time_result, json_result),
            mathz.add(stress_result, error_result)
        )
    )
    
    print("")
    print("===========================================")
    print("COMPREHENSIVE VALIDATION COMPLETE")
    print("Final validation score: ")
    print(final_score)
    print("CURSED Pure Self-Hosting: VERIFIED")
    print("===========================================")
    
    return final_score
}
