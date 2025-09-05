fr fr CURSED JSON Cross-Module Functionality Test
fr fr Testing JSON module integration with other stdlib modules

yeet "json"
yeet "stringz"
yeet "mathz"
yeet "timez"
yeet "fs"

fr fr ==========================================
fr fr Basic JSON functionality tests
fr fr ==========================================

slay test_json_basic() tea {
    sus json_str tea = "{\"name\":\"CURSED\",\"version\":1,\"active\":true}"
    sus parsed tea = parse_json(json_str)
    vibez.spill("Basic JSON parse: " + parsed)
    
    sus simple_obj tea = "{\"count\":42}"
    sus obj_parsed tea = parse_json(simple_obj)
    vibez.spill("Simple object parse: " + obj_parsed)
    
    fr fr Test JSON generation
    sus test_val tea = "Hello JSON"
    sus stringified tea = stringify_json(test_val)
    vibez.spill("JSON stringify: " + stringified)
    
    damn "JSON basic tests completed"
}

fr fr ==========================================
fr fr Cross-module tests: JSON + StringZ
fr fr ==========================================

slay test_json_with_stringz() tea {
    fr fr Use stringz functions with JSON data
    sus json_data tea = "{\"message\":\"Hello World\",\"language\":\"cursed\"}"
    sus parsed tea = parse_json(json_data)
    
    fr fr Use stringz to manipulate JSON string content
    sus cleaned tea = string_trim(json_data)
    sus length normie = string_length(cleaned)
    vibez.spill("JSON string length: " + string_from_int(length))
    
    fr fr Extract a value and use stringz functions
    sus extracted_msg tea = "Hello World"
    sus upper_msg tea = string_to_upper(extracted_msg)
    sus contains_world lit = string_contains(extracted_msg, "World")
    
    vibez.spill("String processing: " + upper_msg)
    vibez.spill("Contains 'World': " + lit_to_string(contains_world))
    
    fr fr Create JSON from stringz operations
    sus concat_result tea = string_concat("JSON", " + StringZ")
    sus json_result tea = stringify_json(concat_result)
    vibez.spill("StringZ to JSON: " + json_result)
    
    damn "JSON + StringZ cross-module test completed"
}

fr fr ==========================================
fr fr Cross-module tests: JSON + MathZ
fr fr ==========================================

slay test_json_with_mathz() tea {
    fr fr JSON numeric data processing with mathz
    sus json_nums tea = "{\"a\":10,\"b\":20,\"result\":0}"
    sus parsed tea = parse_json(json_nums)
    
    fr fr Use mathz for calculations with JSON numeric data
    sus num_a normie = 10
    sus num_b normie = 20
    sus sum normie = math_add(num_a, num_b)
    sus product normie = math_multiply(num_a, num_b)
    sus power normie = math_power(num_a, 2)
    
    vibez.spill("Math operations on JSON numbers:")
    vibez.spill("Sum: " + string_from_int(sum))
    vibez.spill("Product: " + string_from_int(product))
    vibez.spill("Power: " + string_from_int(power))
    
    fr fr Create JSON with math results
    sus math_result tea = "{\"sum\":" + string_from_int(sum) + 
                         ",\"product\":" + string_from_int(product) + 
                         ",\"power\":" + string_from_int(power) + "}"
    
    sus validated lit = is_valid_json(math_result)
    vibez.spill("Math result JSON valid: " + lit_to_string(validated))
    vibez.spill("Math JSON: " + math_result)
    
    damn "JSON + MathZ cross-module test completed"
}

fr fr ==========================================
fr fr Cross-module tests: JSON + TimeZ + FS
fr fr ==========================================

slay test_json_with_time_and_fs() tea {
    fr fr Create JSON metadata with timestamps and file operations
    sus current_time normie = time_now_unix()
    sus time_str tea = string_from_int(current_time)
    
    fr fr Create JSON with timestamp
    sus metadata_json tea = "{\"created\":" + time_str + 
                           ",\"type\":\"test_file\"," +
                           "\"status\":\"active\"}"
    
    vibez.spill("Metadata JSON: " + metadata_json)
    
    fr fr Validate the JSON
    sus is_valid lit = is_valid_json(metadata_json)
    vibez.spill("Metadata JSON valid: " + lit_to_string(is_valid))
    
    fr fr Use FS operations with JSON data
    sus filename tea = "test_metadata.json"
    sus write_result tea = fs_write_file(filename, metadata_json)
    vibez.spill("File write result: " + write_result)
    
    fr fr Read back and parse
    sus read_content tea = fs_read_file(filename)
    sus reparsed tea = parse_json(read_content)
    vibez.spill("Read and reparsed: " + reparsed)
    
    fr fr Clean up
    sus cleanup_result tea = fs_delete_file(filename)
    vibez.spill("Cleanup result: " + cleanup_result)
    
    damn "JSON + TimeZ + FS cross-module test completed"
}

fr fr ==========================================
fr fr Complex cross-module dependency test
fr fr ==========================================

slay test_complex_cross_module() tea {
    fr fr Complex scenario: JSON config with math calculations, 
    fr fr string processing, and file operations
    
    fr fr Create a complex configuration JSON
    sus config_template tea = string_concat("{", 
        string_concat("\"app_name\":", stringify_json("CURSED_APP")) + "," +
        "\"version\":1," +
        "\"math_settings\":{\"precision\":10,\"max_iterations\":1000}," +
        "\"paths\":{\"data\":\"./data\",\"logs\":\"./logs\"}," +
        "\"features\":[\"json\",\"math\",\"strings\",\"files\"]" +
        "}")
    
    vibez.spill("Complex config JSON:")
    vibez.spill(config_template)
    
    fr fr Validate the complex JSON
    sus config_valid lit = is_valid_json(config_template)
    vibez.spill("Config JSON valid: " + lit_to_string(config_valid))
    
    fr fr Parse and extract values
    sus parsed_config tea = parse_json(config_template)
    vibez.spill("Parsed config: " + parsed_config)
    
    fr fr Use extracted values in cross-module operations
    sus precision normie = 10
    sus max_iter normie = 1000
    
    fr fr Math operations based on config
    sus calc_result normie = math_multiply(precision, max_iter)
    sus formatted_result tea = string_from_int(calc_result)
    
    fr fr String operations on paths
    sus data_path tea = "./data"
    sus logs_path tea = "./logs" 
    sus combined_paths tea = string_concat(data_path, " and " + logs_path)
    sus paths_upper tea = string_to_upper(combined_paths)
    
    fr fr Create result JSON with all cross-module data
    sus result_json tea = "{" +
        "\"calculation\":" + formatted_result + "," +
        "\"paths\":\"" + paths_upper + "\"," +
        "\"timestamp\":" + string_from_int(time_now_unix()) +
        "}"
    
    vibez.spill("Cross-module result JSON:")
    vibez.spill(result_json)
    
    fr fr Validate result
    sus result_valid lit = is_valid_json(result_json)
    vibez.spill("Result JSON valid: " + lit_to_string(result_valid))
    
    damn "Complex cross-module test completed"
}

fr fr ==========================================
fr fr Module dependency validation
fr fr ==========================================

slay test_module_dependencies() tea {
    fr fr Test that all required modules are properly loaded
    vibez.spill("Testing module dependencies...")
    
    fr fr Test JSON functions are available
    sus json_test tea = parse_json("{\"test\":true}")
    vibez.spill("JSON module: " + json_test)
    
    fr fr Test StringZ functions are available
    sus string_test tea = string_trim("  test  ")
    vibez.spill("StringZ module: " + string_test)
    
    fr fr Test MathZ functions are available
    sus math_test normie = math_add(5, 3)
    vibez.spill("MathZ module: " + string_from_int(math_test))
    
    fr fr Test TimeZ functions are available
    sus time_test normie = time_now_unix()
    vibez.spill("TimeZ module: " + string_from_int(time_test))
    
    fr fr Test FS functions are available
    sus fs_test tea = fs_current_directory()
    vibez.spill("FS module: " + fs_test)
    
    damn "All module dependencies verified"
}

fr fr ==========================================
fr fr Main test execution
fr fr ==========================================

slay main_character() tea {
    vibez.spill("=== CURSED JSON Cross-Module Tests ===")
    vibez.spill("")
    
    sus test1 tea = test_json_basic()
    vibez.spill(test1)
    vibez.spill("")
    
    sus test2 tea = test_json_with_stringz()
    vibez.spill(test2)
    vibez.spill("")
    
    sus test3 tea = test_json_with_mathz()
    vibez.spill(test3)
    vibez.spill("")
    
    sus test4 tea = test_json_with_time_and_fs()
    vibez.spill(test4)
    vibez.spill("")
    
    sus test5 tea = test_complex_cross_module()
    vibez.spill(test5)
    vibez.spill("")
    
    sus test6 tea = test_module_dependencies()
    vibez.spill(test6)
    
    vibez.spill("")
    vibez.spill("=== All Cross-Module Tests Completed ===")
    
    damn "JSON cross-module testing complete"
}
