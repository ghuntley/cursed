fr fr CURSED Cross-Module Functionality Test
fr fr Testing JSON integration with StringZ, MathZ, TimeZ, and FS modules

yeet "json"
yeet "stringz"  
yeet "mathz"
yeet "timez"
yeet "fs"

fr fr ==========================================
fr fr Test JSON module basic functionality
fr fr ==========================================

slay test_json_basic_functions() {
    vibez.spill("=== Testing JSON Basic Functions ===")
    
    fr fr Test parse_json function
    sus simple_string tea = "\"hello\""
    sus parsed_string tea = parse_json(simple_string)
    vibez.spill("Parsed string: " + parsed_string)
    
    sus simple_number tea = "42"
    sus parsed_number tea = parse_json(simple_number)
    vibez.spill("Parsed number: " + parsed_number)
    
    fr fr Test stringify function  
    sus test_value tea = "world"
    sus stringified tea = stringify(test_value)
    vibez.spill("Stringified: " + stringified)
    
    fr fr Test validation
    sus valid_json tea = "\"test\""
    sus is_valid lit = is_valid_json(valid_json)
    vibez.spill("JSON validation result: " + lit_to_string(is_valid))
    
    damn "JSON basic functions tested"
}

fr fr ==========================================
fr fr Test Cross-Module: JSON + StringZ
fr fr ==========================================

slay test_json_stringz_integration() {
    vibez.spill("=== Testing JSON + StringZ Integration ===")
    
    fr fr Create JSON with stringz operations
    sus base_text tea = "  Hello World  "
    sus trimmed tea = string_trim(base_text)  
    sus length normie = string_length(trimmed)
    sus upper tea = string_to_upper(trimmed)
    
    fr fr Create JSON from string operations
    sus json_data tea = "{\"original\":\"" + base_text + 
                       "\",\"trimmed\":\"" + trimmed + 
                       "\",\"length\":" + string_from_int(length) + 
                       ",\"upper\":\"" + upper + "\"}"
    
    vibez.spill("Generated JSON: " + json_data)
    
    fr fr Validate the generated JSON
    sus json_valid lit = is_valid_json(json_data)
    vibez.spill("JSON validity: " + lit_to_string(json_valid))
    
    fr fr Parse and re-process
    sus parsed tea = parse_json(json_data)
    vibez.spill("Parsed result: " + parsed)
    
    damn "JSON + StringZ integration tested"
}

fr fr ==========================================
fr fr Test Cross-Module: JSON + MathZ  
fr fr ==========================================

slay test_json_mathz_integration() {
    vibez.spill("=== Testing JSON + MathZ Integration ===")
    
    fr fr Use mathz for calculations
    sus num_a normie = 15
    sus num_b normie = 25
    sus sum normie = math_add(num_a, num_b)
    sus product normie = math_multiply(num_a, num_b)
    sus difference normie = math_subtract(num_b, num_a)
    
    fr fr Create JSON with math results
    sus math_json tea = "{" +
                       "\"operand_a\":" + string_from_int(num_a) + "," +
                       "\"operand_b\":" + string_from_int(num_b) + "," +
                       "\"sum\":" + string_from_int(sum) + "," +
                       "\"product\":" + string_from_int(product) + "," +
                       "\"difference\":" + string_from_int(difference) +
                       "}"
    
    vibez.spill("Math JSON: " + math_json)
    
    fr fr Validate math JSON
    sus math_json_valid lit = is_valid_json(math_json)
    vibez.spill("Math JSON valid: " + lit_to_string(math_json_valid))
    
    fr fr Parse the math JSON
    sus parsed_math tea = parse_json(math_json)
    vibez.spill("Parsed math JSON: " + parsed_math)
    
    damn "JSON + MathZ integration tested"
}

fr fr ==========================================
fr fr Test Cross-Module: JSON + TimeZ + FS
fr fr ==========================================

slay test_json_time_fs_integration() {
    vibez.spill("=== Testing JSON + TimeZ + FS Integration ===")
    
    fr fr Get current timestamp
    sus current_time normie = time_now_unix()
    sus time_string tea = string_from_int(current_time)
    
    fr fr Create metadata JSON with timestamp
    sus metadata tea = "{" +
                      "\"created_at\":" + time_string + "," +
                      "\"type\":\"test_metadata\"," +
                      "\"version\":1," +
                      "\"status\":\"active\"" +
                      "}"
    
    vibez.spill("Metadata JSON: " + metadata)
    
    fr fr Validate metadata JSON
    sus metadata_valid lit = is_valid_json(metadata)
    vibez.spill("Metadata valid: " + lit_to_string(metadata_valid))
    
    fr fr Try to write to file (if FS module works)
    sus filename tea = "test_metadata.json"
    sus write_result tea = fs_write_file(filename, metadata)
    vibez.spill("File write attempt: " + write_result)
    
    fr fr Try to read back
    sus read_result tea = fs_read_file(filename)
    vibez.spill("File read attempt: " + read_result)
    
    fr fr Clean up attempt
    sus cleanup tea = fs_delete_file(filename)
    vibez.spill("Cleanup attempt: " + cleanup)
    
    damn "JSON + TimeZ + FS integration tested"
}

fr fr ==========================================
fr fr Test Complex Multi-Module Scenario
fr fr ==========================================

slay test_complex_multi_module() {
    vibez.spill("=== Complex Multi-Module Test ===")
    
    fr fr Create a complex scenario involving all modules
    sus app_name tea = "CURSED_JSON_TEST"
    sus app_version normie = 1
    sus current_timestamp normie = time_now_unix()
    
    fr fr String processing
    sus processed_name tea = string_to_lower(app_name)
    sus name_length normie = string_length(processed_name)
    
    fr fr Math calculations
    sus calculation_base normie = 100
    sus multiplied normie = math_multiply(app_version, calculation_base)
    sus calculated_value normie = math_add(multiplied, name_length)
    
    fr fr Build comprehensive JSON
    sus complex_json tea = "{" +
        "\"application\":{" +
            "\"name\":\"" + app_name + "\"," +
            "\"processed_name\":\"" + processed_name + "\"," +
            "\"version\":" + string_from_int(app_version) + "," +
            "\"name_length\":" + string_from_int(name_length) +
        "}," +
        "\"timestamp\":" + string_from_int(current_timestamp) + "," +
        "\"calculations\":{" +
            "\"base\":" + string_from_int(calculation_base) + "," +
            "\"multiplied\":" + string_from_int(multiplied) + "," +
            "\"final_value\":" + string_from_int(calculated_value) +
        "}" +
        "}"
    
    vibez.spill("Complex JSON:")
    vibez.spill(complex_json)
    
    fr fr Validate complex JSON
    sus complex_valid lit = is_valid_json(complex_json)
    vibez.spill("Complex JSON valid: " + lit_to_string(complex_valid))
    
    fr fr Parse complex JSON
    sus parsed_complex tea = parse_json(complex_json)
    vibez.spill("Parsed complex JSON:")
    vibez.spill(parsed_complex)
    
    damn "Complex multi-module test completed"
}

fr fr ==========================================
fr fr Module Dependency Verification
fr fr ==========================================

slay test_module_dependencies() {
    vibez.spill("=== Module Dependencies Test ===")
    
    fr fr Test each module is loadable and basic functions work
    
    fr fr JSON module test
    sus json_test tea = parse_json("\"json_works\"")
    vibez.spill("JSON module: " + json_test)
    
    fr fr StringZ module test
    sus string_test tea = string_concat("string", "_works")
    vibez.spill("StringZ module: " + string_test)
    
    fr fr MathZ module test
    sus math_test normie = math_add(2, 3)
    vibez.spill("MathZ module: " + string_from_int(math_test))
    
    fr fr TimeZ module test
    sus time_test normie = time_now_unix()
    vibez.spill("TimeZ module timestamp: " + string_from_int(time_test))
    
    fr fr FS module test (basic attempt)
    sus fs_test tea = fs_current_directory()
    vibez.spill("FS module: " + fs_test)
    
    damn "All module dependencies verified"
}

fr fr ==========================================
fr fr Main Test Runner
fr fr ==========================================

slay main_character() {
    vibez.spill("🔥 CURSED Cross-Module Comprehensive Test Suite 🔥")
    vibez.spill("=" * 50)
    vibez.spill("")
    
    sus test1 tea = test_json_basic_functions()
    vibez.spill(test1)
    vibez.spill("")
    
    sus test2 tea = test_json_stringz_integration()
    vibez.spill(test2)
    vibez.spill("")
    
    sus test3 tea = test_json_mathz_integration()
    vibez.spill(test3)
    vibez.spill("")
    
    sus test4 tea = test_json_time_fs_integration()
    vibez.spill(test4)
    vibez.spill("")
    
    sus test5 tea = test_complex_multi_module()
    vibez.spill(test5)
    vibez.spill("")
    
    sus test6 tea = test_module_dependencies()
    vibez.spill(test6)
    
    vibez.spill("")
    vibez.spill("🎉 Cross-Module Test Suite Complete 🎉")
    vibez.spill("=" * 50)
}
