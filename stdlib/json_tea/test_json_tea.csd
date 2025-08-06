yeet "testz"
yeet "json_tea"

fr fr ==========================================
fr fr CURSED JSON Tea Module Test Suite
fr fr Comprehensive Testing for Marshal/Unmarshal
fr fr ==========================================

slay test_marshal_basic_types() {
    test_start("Marshal Basic Types") fr fr Test string marshaling
    sus str_result tea = json_tea.Marshal("hello")
    assert_eq_string(str_result, "\"hello\"") fr fr Test number marshaling
    sus num_result tea = json_tea.Marshal("42")
    assert_eq_string(num_result, "42")
    
    sus float_result tea = json_tea.Marshal("3.14")
    assert_eq_string(float_result, "3.14") fr fr Test boolean marshaling
    sus true_result tea = json_tea.Marshal("based")
    assert_eq_string(true_result, "true")
    
    sus false_result tea = json_tea.Marshal("cap")
    assert_eq_string(false_result, "false") fr fr Test null marshaling
    sus null_result tea = json_tea.Marshal("cringe")
    assert_eq_string(null_result, "null")
    
    sus empty_result tea = json_tea.Marshal("")
    assert_eq_string(empty_result, "null")
}

slay test_unmarshal_basic_types() {
    test_start("Unmarshal Basic Types") fr fr Test string unmarshaling
    sus str_result tea = json_tea.Unmarshal("\"hello\"")
    assert_eq_string(str_result, "hello") fr fr Test number unmarshaling
    sus num_result tea = json_tea.Unmarshal("42")
    assert_eq_string(num_result, "42")
    
    sus float_result tea = json_tea.Unmarshal("3.14")
    assert_eq_string(float_result, "3.14") fr fr Test boolean unmarshaling
    sus true_result tea = json_tea.Unmarshal("true")
    assert_eq_string(true_result, "based")
    
    sus false_result tea = json_tea.Unmarshal("false")
    assert_eq_string(false_result, "cap") fr fr Test null unmarshaling
    sus null_result tea = json_tea.Unmarshal("null")
    assert_eq_string(null_result, "cringe")
}

slay test_marshal_object() {
    test_start("Marshal Object Data") fr fr Test simple object marshaling
    sus obj_data tea = "{\"name\": \"John\", \"age\": 30}"
    sus obj_result tea = json_tea.Marshal(obj_data)
    assert_true(json_tea.string_contains(obj_result, "name"))
    assert_true(json_tea.string_contains(obj_result, "John"))
    assert_true(json_tea.string_contains(obj_result, "age"))
    assert_true(json_tea.string_contains(obj_result, "30")) fr fr Test empty object marshaling
    sus empty_obj tea = json_tea.Marshal("{}")
    assert_eq_string(empty_obj, "{}")
}

slay test_unmarshal_object() {
    test_start("Unmarshal Object Data") fr fr Test simple object unmarshaling
    sus json_obj tea = "{\"name\": \"John\", \"age\": 30}"
    sus obj_result tea = json_tea.Unmarshal(json_obj)
    assert_true(json_tea.string_contains(obj_result, "name"))
    assert_true(json_tea.string_contains(obj_result, "John")) fr fr Test empty object unmarshaling
    sus empty_result tea = json_tea.Unmarshal("{}")
    assert_eq_string(empty_result, "{}")
}

slay test_marshal_array() {
    test_start("Marshal Array Data") fr fr Test simple array marshaling
    sus arr_data tea = "[1, 2, 3, \"hello\"]"
    sus arr_result tea = json_tea.Marshal(arr_data)
    assert_true(json_tea.string_contains(arr_result, "1"))
    assert_true(json_tea.string_contains(arr_result, "2"))
    assert_true(json_tea.string_contains(arr_result, "3"))
    assert_true(json_tea.string_contains(arr_result, "hello")) fr fr Test empty array marshaling
    sus empty_arr tea = json_tea.Marshal("[]")
    assert_eq_string(empty_arr, "[]")
}

slay test_unmarshal_array() {
    test_start("Unmarshal Array Data") fr fr Test simple array unmarshaling
    sus json_arr tea = "[1, 2, 3, \"hello\"]"
    sus arr_result tea = json_tea.Unmarshal(json_arr)
    assert_true(json_tea.string_contains(arr_result, "1"))
    assert_true(json_tea.string_contains(arr_result, "2"))
    assert_true(json_tea.string_contains(arr_result, "3")) fr fr Test empty array unmarshaling
    sus empty_result tea = json_tea.Unmarshal("[]")
    assert_eq_string(empty_result, "[]")
}

slay test_marshal_indent() {
    test_start("Marshal with Indentation") fr fr Test indented marshaling
    sus data tea = "{\"name\": \"John\"}"
    sus indented tea = json_tea.MarshalIndent(data, "", "  ") fr fr Should contain original data
    assert_true(json_tea.string_contains(indented, "name"))
    assert_true(json_tea.string_contains(indented, "John")) fr fr Should be longer due to formatting
    assert_true(string_length(indented) >= string_length(data))
}

slay test_marshal_compact() {
    test_start("Marshal Compact") fr fr Test compact marshaling
    sus data tea = "{ \"name\" : \"John\" , \"age\" : 30 }"
    sus compact tea = json_tea.MarshalCompact(data) fr fr Should contain original data
    assert_true(json_tea.string_contains(compact, "name"))
    assert_true(json_tea.string_contains(compact, "John")) fr fr Should be shorter due to whitespace removal
    assert_true(string_length(compact) <= string_length(data))
}

slay test_unmarshal_to_map() {
    test_start("Unmarshal to Map") fr fr Test object to map conversion
    sus json_obj tea = "{\"key\": \"value\", \"num\": 42}"
    sus map_result tea = json_tea.UnmarshalToMap(json_obj)
    
    assert_true(json_tea.string_starts_with(map_result, "MAP:"))
    assert_true(json_tea.string_contains(map_result, "key"))
    assert_true(json_tea.string_contains(map_result, "value")) fr fr Test error for non-object
    sus error_result tea = json_tea.UnmarshalToMap("[1, 2, 3]")
    assert_true(json_tea.string_starts_with(error_result, "ERROR"))
}

slay test_unmarshal_to_slice() {
    test_start("Unmarshal to Slice") fr fr Test array to slice conversion
    sus json_arr tea = "[1, 2, 3, \"hello\"]"
    sus slice_result tea = json_tea.UnmarshalToSlice(json_arr)
    
    assert_true(json_tea.string_starts_with(slice_result, "SLICE:"))
    assert_true(json_tea.string_contains(slice_result, "1"))
    assert_true(json_tea.string_contains(slice_result, "hello")) fr fr Test error for non-array
    sus error_result tea = json_tea.UnmarshalToSlice("{\"key\": \"value\"}")
    assert_true(json_tea.string_starts_with(error_result, "ERROR"))
}

slay test_json_validation() {
    test_start("JSON Validation") fr fr Test valid JSON validation
    assert_true(json_tea.IsValidJSON("{\"name\": \"John\"}"))
    assert_true(json_tea.IsValidJSON("[1, 2, 3]"))
    assert_true(json_tea.IsValidJSON("\"hello\""))
    assert_true(json_tea.IsValidJSON("42"))
    assert_true(json_tea.IsValidJSON("true"))
    assert_true(json_tea.IsValidJSON("false"))
    assert_true(json_tea.IsValidJSON("null")) fr fr Test invalid JSON validation
    assert_false(json_tea.IsValidJSON("invalid"))
    assert_false(json_tea.IsValidJSON(""))
    assert_false(json_tea.IsValidJSON("{invalid}"))
    assert_false(json_tea.IsValidJSON("[invalid]"))
}

slay test_schema_validation() {
    test_start("Schema Validation") fr fr Test object schema validation
    assert_true(json_tea.ValidateSchema("{\"key\": \"value\"}", "object"))
    assert_false(json_tea.ValidateSchema("\"string\"", "object")) fr fr Test array schema validation
    assert_true(json_tea.ValidateSchema("[1, 2, 3]", "array"))
    assert_false(json_tea.ValidateSchema("{}", "array")) fr fr Test string schema validation
    assert_true(json_tea.ValidateSchema("\"hello\"", "string"))
    assert_false(json_tea.ValidateSchema("42", "string")) fr fr Test number schema validation
    assert_true(json_tea.ValidateSchema("42", "number"))
    assert_true(json_tea.ValidateSchema("3.14", "number"))
    assert_false(json_tea.ValidateSchema("\"42\"", "number")) fr fr Test boolean schema validation
    assert_true(json_tea.ValidateSchema("true", "boolean"))
    assert_true(json_tea.ValidateSchema("false", "boolean"))
    assert_false(json_tea.ValidateSchema("\"true\"", "boolean")) fr fr Test null schema validation
    assert_true(json_tea.ValidateSchema("null", "null"))
    assert_false(json_tea.ValidateSchema("\"null\"", "null"))
}

slay test_string_escaping() {
    test_start("String Escaping/Unescaping") fr fr Test basic escaping
    sus original tea = "Hello\nWorld\t\"Quote\""
    sus escaped tea = json_tea.json_escape_string(original) fr fr Should contain escape sequences
    assert_true(json_tea.string_contains(escaped, "\\n"))
    assert_true(json_tea.string_contains(escaped, "\\t"))
    assert_true(json_tea.string_contains(escaped, "\\\"")) fr fr Test unescaping
    sus unescaped tea = json_tea.json_unescape_string(escaped)
    assert_true(json_tea.string_contains(unescaped, "Hello"))
    assert_true(json_tea.string_contains(unescaped, "World")) fr fr Test round-trip
    sus marshaled tea = json_tea.Marshal(original)
    sus unmarshaled tea = json_tea.Unmarshal(marshaled)
    assert_true(json_tea.string_contains(unmarshaled, "Hello"))
    assert_true(json_tea.string_contains(unmarshaled, "World"))
}

slay test_numeric_validation() {
    test_start("Numeric Validation") fr fr Test valid numbers
    assert_true(json_tea.is_numeric("42"))
    assert_true(json_tea.is_numeric("3.14"))
    assert_true(json_tea.is_numeric("-42"))
    assert_true(json_tea.is_numeric("-3.14"))
    assert_true(json_tea.is_numeric("0"))
    assert_true(json_tea.is_numeric("0.0")) fr fr Test invalid numbers
    assert_false(json_tea.is_numeric("abc"))
    assert_false(json_tea.is_numeric(""))
    assert_false(json_tea.is_numeric("3.14.15")) fr fr Multiple dots
    assert_false(json_tea.is_numeric("3.14abc")) fr fr Mixed
    assert_false(json_tea.is_numeric("--42")) fr fr Double negative
}

slay test_json_number_validation() {
    test_start("JSON Number Validation") fr fr Test valid JSON numbers
    assert_true(json_tea.is_valid_json_number("42"))
    assert_true(json_tea.is_valid_json_number("3.14"))
    assert_true(json_tea.is_valid_json_number("-42"))
    assert_true(json_tea.is_valid_json_number("-3.14"))
    assert_true(json_tea.is_valid_json_number("0"))
    assert_true(json_tea.is_valid_json_number("1e10"))
    assert_true(json_tea.is_valid_json_number("1E-10"))
    assert_true(json_tea.is_valid_json_number("1.5e+10")) fr fr Test invalid JSON numbers
    assert_false(json_tea.is_valid_json_number(""))
    assert_false(json_tea.is_valid_json_number("-"))
    assert_false(json_tea.is_valid_json_number("abc"))
    assert_false(json_tea.is_valid_json_number("3.14.15"))
    assert_false(json_tea.is_valid_json_number("1ee10"))
    assert_false(json_tea.is_valid_json_number("1e"))
}

slay test_type_detection() {
    test_start("Type Detection") fr fr Test object detection
    assert_true(json_tea.is_object("{\"key\": \"value\"}"))
    assert_true(json_tea.is_object("key: value")) fr fr Key-value pair
    assert_false(json_tea.is_object("[1, 2, 3]")) fr fr Test array detection
    assert_true(json_tea.is_array("[1, 2, 3]"))
    assert_true(json_tea.is_array("1, 2, 3")) fr fr Comma-separated
    assert_false(json_tea.is_array("{\"key\": \"value\"}")) fr fr Test string literal detection
    assert_true(json_tea.is_string_literal("\"hello\""))
    assert_false(json_tea.is_string_literal("hello")) fr fr Test boolean detection
    assert_true(json_tea.is_boolean("based"))
    assert_true(json_tea.is_boolean("cap"))
    assert_true(json_tea.is_boolean("true"))
    assert_true(json_tea.is_boolean("false"))
    assert_false(json_tea.is_boolean("maybe"))
}

slay test_json_type_detection() {
    test_start("JSON Type Detection") fr fr Test get_json_type function
    assert_eq_string(json_tea.get_json_type("{\"key\": \"value\"}"), "object")
    assert_eq_string(json_tea.get_json_type("[1, 2, 3]"), "array")
    assert_eq_string(json_tea.get_json_type("\"hello\""), "string")
    assert_eq_string(json_tea.get_json_type("42"), "number")
    assert_eq_string(json_tea.get_json_type("3.14"), "number")
    assert_eq_string(json_tea.get_json_type("true"), "boolean")
    assert_eq_string(json_tea.get_json_type("false"), "boolean")
    assert_eq_string(json_tea.get_json_type("null"), "null")
    assert_eq_string(json_tea.get_json_type("invalid"), "unknown")
}

slay test_formatting_functions() {
    test_start("Formatting Functions") fr fr Test compact formatting
    sus json_with_spaces tea = "{ \"name\" : \"John\" , \"age\" : 30 }"
    sus compacted tea = json_tea.compact_json(json_with_spaces) fr fr Should be shorter and contain no unnecessary spaces
    assert_true(string_length(compacted) < string_length(json_with_spaces))
    assert_true(json_tea.string_contains(compacted, "name"))
    assert_true(json_tea.string_contains(compacted, "John"))
    assert_false(json_tea.string_contains(compacted, " : ")) fr fr Test indented formatting
    sus compact_json tea = "{\"name\":\"John\",\"age\":30}"
    sus formatted tea = json_tea.format_json_with_indent(compact_json, "", "  ") fr fr Should be longer due to formatting
    assert_true(string_length(formatted) > string_length(compact_json))
    assert_true(json_tea.string_contains(formatted, "name"))
    assert_true(json_tea.string_contains(formatted, "John"))
}

slay test_round_trip_processing() {
    test_start("Round-Trip Processing") fr fr Test string round-trip
    sus original_str tea = "hello world"
    sus marshaled_str tea = json_tea.Marshal(original_str)
    sus unmarshaled_str tea = json_tea.Unmarshal(marshaled_str)
    assert_eq_string(unmarshaled_str, original_str) fr fr Test number round-trip
    sus original_num tea = "42"
    sus marshaled_num tea = json_tea.Marshal(original_num)
    sus unmarshaled_num tea = json_tea.Unmarshal(marshaled_num)
    assert_eq_string(unmarshaled_num, original_num) fr fr Test boolean round-trip
    sus marshaled_true tea = json_tea.Marshal("based")
    sus unmarshaled_true tea = json_tea.Unmarshal(marshaled_true)
    assert_eq_string(unmarshaled_true, "based") fr fr Test object round-trip
    sus original_obj tea = "{\"name\": \"John\"}"
    sus marshaled_obj tea = json_tea.Marshal(original_obj)
    sus unmarshaled_obj tea = json_tea.Unmarshal(marshaled_obj)
    assert_true(json_tea.string_contains(unmarshaled_obj, "name"))
    assert_true(json_tea.string_contains(unmarshaled_obj, "John"))
}

slay test_error_handling() {
    test_start("Error Handling") fr fr Test invalid JSON unmarshal errors
    sus invalid_result tea = json_tea.Unmarshal("invalid")
    assert_true(json_tea.string_starts_with(invalid_result, "ERROR"))
    
    sus empty_result tea = json_tea.Unmarshal("")
    assert_true(json_tea.string_starts_with(empty_result, "ERROR")) fr fr Test invalid object errors
    sus invalid_obj tea = json_tea.UnmarshalToMap("not an object")
    assert_true(json_tea.string_starts_with(invalid_obj, "ERROR")) fr fr Test invalid array errors
    sus invalid_arr tea = json_tea.UnmarshalToSlice("not an array")
    assert_true(json_tea.string_starts_with(invalid_arr, "ERROR")) fr fr Test invalid number marshal
    sus invalid_marshal tea = json_tea.marshal_number("not a number")
    assert_true(json_tea.string_starts_with(invalid_marshal, "ERROR"))
}

slay test_edge_cases() {
    test_start("Edge Cases") fr fr Test empty string marshaling
    sus empty_str tea = ""
    sus empty_marshaled tea = json_tea.Marshal(empty_str)
    assert_eq_string(empty_marshaled, "null") fr fr Test whitespace handling
    sus whitespace_json tea = "  \"hello\"  "
    sus whitespace_result tea = json_tea.Unmarshal(whitespace_json)
    assert_eq_string(whitespace_result, "hello") fr fr Test zero number
    sus zero_marshaled tea = json_tea.Marshal("0")
    assert_eq_string(zero_marshaled, "0")
    
    sus zero_unmarshaled tea = json_tea.Unmarshal("0")
    assert_eq_string(zero_unmarshaled, "0") fr fr Test empty object and array
    sus empty_obj_result tea = json_tea.Unmarshal("{}")
    assert_eq_string(empty_obj_result, "{}")
    
    sus empty_arr_result tea = json_tea.Unmarshal("[]")
    assert_eq_string(empty_arr_result, "[]")
}

slay test_complex_json() {
    test_start("Complex JSON Processing") fr fr Test nested object
    sus nested_obj tea = "{\"user\": {\"name\": \"John\", \"age\": 30}, \"active\": true}"
    sus nested_result tea = json_tea.Unmarshal(nested_obj)
    assert_true(json_tea.string_contains(nested_result, "user"))
    assert_true(json_tea.string_contains(nested_result, "name"))
    assert_true(json_tea.string_contains(nested_result, "John")) fr fr Test array of objects
    sus obj_array tea = "[{\"id\": 1}, {\"id\": 2}]"
    sus array_result tea = json_tea.Unmarshal(obj_array)
    assert_true(json_tea.string_contains(array_result, "id"))
    assert_true(json_tea.string_contains(array_result, "1"))
    assert_true(json_tea.string_contains(array_result, "2")) fr fr Test mixed array
    sus mixed_array tea = "[\"hello\", 42, true, null]"
    sus mixed_result tea = json_tea.Unmarshal(mixed_array)
    assert_true(json_tea.string_contains(mixed_result, "hello"))
    assert_true(json_tea.string_contains(mixed_result, "42"))
    assert_true(json_tea.string_contains(mixed_result, "true"))
    assert_true(json_tea.string_contains(mixed_result, "null"))
}

slay test_enhanced_parsing() {
    test_start("Enhanced JSON Parsing") fr fr Test parse_json functions
    sus parsed_string tea = json_tea.parse_json_string("\"hello\"")
    assert_eq_string(parsed_string, "hello")
    
    sus parsed_number tea = json_tea.parse_json("42")
    assert_eq_string(parsed_number, "42") fr fr Test parse_json_file (should return error in demo)
    sus file_result tea = json_tea.parse_json_file("test.json")
    assert_true(json_tea.string_starts_with(file_result, "ERROR"))
}

slay test_json_generation() {
    test_start("JSON Generation") fr fr Test to_json function
    sus to_json_result tea = json_tea.to_json("hello")
    assert_eq_string(to_json_result, "\"hello\"") fr fr Test format_json function
    sus format_result tea = json_tea.format_json("{\"name\": \"John\"}")
    assert_true(json_tea.string_contains(format_result, "name"))
    assert_true(json_tea.string_contains(format_result, "John"))
}

slay test_value_access() {
    test_start("Value Access Functions") fr fr Test get_value function
    sus json_obj tea = "{\"name\": \"John\", \"age\": 30}"
    sus name_value tea = json_tea.get_value(json_obj, "name")
    assert_eq_string(name_value, "John")
    
    sus age_value tea = json_tea.get_value(json_obj, "age")
    assert_eq_string(age_value, "30") fr fr Test get_string function
    sus string_value tea = json_tea.get_string(json_obj, "name")
    assert_eq_string(string_value, "John") fr fr Test get_number function
    sus number_value tea = json_tea.get_number(json_obj, "age")
    assert_eq_string(number_value, "30") fr fr Test error cases
    sus error_value tea = json_tea.get_value(json_obj, "missing")
    assert_true(json_tea.string_starts_with(error_value, "ERROR"))
}

slay test_type_checking_functions() {
    test_start("Type Checking Functions") fr fr Test is_string function
    assert_true(json_tea.is_string("\"hello\""))
    assert_false(json_tea.is_string("42")) fr fr Test is_number function
    assert_true(json_tea.is_number("42"))
    assert_true(json_tea.is_number("3.14"))
    assert_false(json_tea.is_number("\"42\"")) fr fr Test is_boolean_value function
    assert_true(json_tea.is_boolean_value("true"))
    assert_true(json_tea.is_boolean_value("false"))
    assert_false(json_tea.is_boolean_value("\"true\"")) fr fr Test is_array_value function
    assert_true(json_tea.is_array_value("[1, 2, 3]"))
    assert_false(json_tea.is_array_value("{\"key\": \"value\"}")) fr fr Test is_object_value function
    assert_true(json_tea.is_object_value("{\"key\": \"value\"}"))
    assert_false(json_tea.is_object_value("[1, 2, 3]")) fr fr Test is_null_value function
    assert_true(json_tea.is_null_value("null"))
    assert_false(json_tea.is_null_value("\"null\""))
}

slay test_manipulation_functions() {
    test_start("JSON Manipulation Functions") fr fr Test set_value function
    sus original_obj tea = "{\"name\": \"John\", \"age\": 30}"
    sus modified_obj tea = json_tea.set_value(original_obj, "name", "Jane")
    assert_true(json_tea.string_contains(modified_obj, "Jane")) fr fr Test add_to_array function
    sus empty_array tea = "[]"
    sus array_with_item tea = json_tea.add_to_array(empty_array, "hello")
    assert_true(json_tea.string_contains(array_with_item, "hello"))
    
    sus existing_array tea = "[1, 2, 3]"
    sus extended_array tea = json_tea.add_to_array(existing_array, "new")
    assert_true(json_tea.string_contains(extended_array, "new")) fr fr Test merge_objects function
    sus obj1 tea = "{\"name\": \"John\"}"
    sus obj2 tea = "{\"age\": 30}"
    sus merged tea = json_tea.merge_objects(obj1, obj2)
    assert_true(json_tea.string_contains(merged, "name"))
    assert_true(json_tea.string_contains(merged, "age"))
}

slay test_validation_functions() {
    test_start("Enhanced Validation Functions") fr fr Test validate_json function
    assert_true(json_tea.validate_json("{\"name\": \"John\"}"))
    assert_true(json_tea.validate_json("[1, 2, 3]"))
    assert_false(json_tea.validate_json("invalid")) fr fr Test validate_schema function
    assert_true(json_tea.validate_schema("{\"key\": \"value\"}", "object"))
    assert_true(json_tea.validate_schema("[1, 2, 3]", "array"))
    assert_false(json_tea.validate_schema("\"string\"", "number"))
}

slay test_legacy_compatibility() {
    test_start("Legacy Compatibility") fr fr Test legacy marshal function
    sus legacy_marshal tea = json_tea.marshal("hello")
    assert_eq_string(legacy_marshal, "\"hello\"") fr fr Test legacy unmarshal function
    sus legacy_unmarshal tea = json_tea.unmarshal("\"hello\"")
    assert_eq_string(legacy_unmarshal, "hello") fr fr Test legacy parse function
    sus legacy_parse tea = json_tea.parse("42")
    assert_eq_string(legacy_parse, "42") fr fr Test legacy stringify function
    sus legacy_stringify tea = json_tea.stringify("hello")
    assert_eq_string(legacy_stringify, "\"hello\"")
}

slay test_performance_basics() {
    test_start("Performance Basics") fr fr Test large number handling
    sus large_num tea = "1234567890123456789"
    assert_true(json_tea.is_numeric(large_num))
    sus large_marshaled tea = json_tea.Marshal(large_num)
    assert_eq_string(large_marshaled, large_num) fr fr Test long string handling
    sus long_str tea = "This is a very long string to test performance with JSON Tea module"
    sus long_marshaled tea = json_tea.Marshal(long_str)
    sus long_unmarshaled tea = json_tea.Unmarshal(long_marshaled)
    assert_eq_string(long_unmarshaled, long_str) fr fr Test multiple operations
    sus data tea = "{\"count\": 100}"
    sus i normie = 0
    bestie i < 10 {
        sus marshaled tea = json_tea.Marshal(data)
        sus unmarshaled tea = json_tea.Unmarshal(marshaled)
        assert_true(json_tea.string_contains(unmarshaled, "count"))
        i = i + 1
    }
}

slay run_all_json_tea_tests() {
    vibez.spill("🍵 Running CURSED JSON Tea Module Tests")
    vibez.spill("==========================================")
    vibez.spill("Enhanced JSON Processing with Comprehensive Functionality") fr fr Core Marshal/Unmarshal tests
    test_marshal_basic_types()
    test_unmarshal_basic_types()
    test_marshal_object()
    test_unmarshal_object()
    test_marshal_array()
    test_unmarshal_array() fr fr Advanced functionality tests
    test_marshal_indent()
    test_marshal_compact()
    test_unmarshal_to_map()
    test_unmarshal_to_slice() fr fr Enhanced parsing and generation tests
    test_enhanced_parsing()
    test_json_generation() fr fr Value access and manipulation tests
    test_value_access()
    test_manipulation_functions() fr fr Type checking tests
    test_type_checking_functions() fr fr Validation tests
    test_json_validation()
    test_schema_validation()
    test_validation_functions() fr fr String processing tests
    test_string_escaping()
    test_numeric_validation()
    test_json_number_validation() fr fr Type detection tests
    test_type_detection()
    test_json_type_detection() fr fr Formatting tests
    test_formatting_functions() fr fr Integration tests
    test_round_trip_processing()
    test_error_handling()
    test_edge_cases()
    test_complex_json()
    test_legacy_compatibility()
    test_performance_basics()
    
    print_test_summary()
}

fr fr Auto-run tests when this file is executed
run_all_json_tea_tests()
