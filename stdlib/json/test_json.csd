yeet "testz"
yeet "json"

fr fr ==========================================
fr fr CURSED JSON Library Test Suite
fr fr RFC 7159 Compliant JSON Processing Tests
fr fr ==========================================

slay test_json_parse_main_api() {
    test_start("JSON Main API - parse_json() and from_string()") fr fr Test main parse_json function
    sus result1 tea = json.parse_json("\"hello\"")
    assert_eq_string(result1, "hello") fr fr Test from_string alias
    sus result2 tea = json.from_string("42")
    assert_eq_string(result2, "42") fr fr Test empty string error handling
    sus empty_result tea = json.parse_json("")
    assert_true(json.string_contains(empty_result, "ERROR"))
}

slay test_json_parse_primitives() {
    test_start("JSON Parse Primitive Values") fr fr Test parsing strings
    sus str_result tea = json.parse_value("\"hello world\"")
    assert_eq_string(str_result, "hello world") fr fr Test parsing numbers
    sus num_result tea = json.parse_value("42")
    assert_eq_string(num_result, "42")
    
    sus float_result tea = json.parse_value("3.14")
    assert_eq_string(float_result, "3.14") fr fr Test parsing booleans
    sus true_result tea = json.parse_value("true")
    assert_eq_string(true_result, "true")
    
    sus false_result tea = json.parse_value("false")
    assert_eq_string(false_result, "false") fr fr Test parsing null
    sus null_result tea = json.parse_value("null")
    assert_eq_string(null_result, "null")
}

slay test_json_parse_objects() {
    test_start("JSON Parse Objects") fr fr Test empty object
    sus empty_obj tea = json.parse_object("{}")
    assert_eq_string(empty_obj, "{}") fr fr Test simple object
    sus simple_obj tea = json.parse_object("{\"name\": \"John\"}")
    assert_true(json.string_contains(simple_obj, "name"))
    assert_true(json.string_contains(simple_obj, "John")) fr fr Test invalid object
    sus invalid_obj tea = json.parse_object("invalid")
    assert_true(json.string_contains(invalid_obj, "ERROR"))
}

slay test_json_parse_arrays() {
    test_start("JSON Parse Arrays") fr fr Test empty array
    sus empty_arr tea = json.parse_array("[]")
    assert_eq_string(empty_arr, "[]") fr fr Test simple array
    sus simple_arr tea = json.parse_array("[1, 2, 3]")
    assert_true(json.string_contains(simple_arr, "1"))
    assert_true(json.string_contains(simple_arr, "2"))
    assert_true(json.string_contains(simple_arr, "3")) fr fr Test invalid array
    sus invalid_arr tea = json.parse_array("invalid")
    assert_true(json.string_contains(invalid_arr, "ERROR"))
}

slay test_json_validation() {
    test_start("JSON Validation - is_valid_json()") fr fr Test valid JSON with is_valid_json
    assert_true(json.is_valid_json("{\"name\": \"John\"}"))
    assert_true(json.is_valid_json("[1, 2, 3]"))
    assert_true(json.is_valid_json("\"hello\""))
    assert_true(json.is_valid_json("42"))
    assert_true(json.is_valid_json("true"))
    assert_true(json.is_valid_json("false"))
    assert_true(json.is_valid_json("null")) fr fr Test invalid JSON
    assert_false(json.is_valid_json("invalid"))
    assert_false(json.is_valid_json(""))
    assert_false(json.is_valid_json("undefined")) fr fr Test legacy validate function
    assert_true(json.validate("\"hello\""))
    assert_false(json.validate("invalid"))
}

slay test_json_schema_validation() {
    test_start("JSON Schema Validation") fr fr Test object schema validation
    assert_true(json.validate_schema("{\"key\": \"value\"}", "object"))
    assert_false(json.validate_schema("\"string\"", "object")) fr fr Test array schema validation
    assert_true(json.validate_schema("[1, 2, 3]", "array"))
    assert_false(json.validate_schema("{}", "array")) fr fr Test string schema validation
    assert_true(json.validate_schema("\"hello\"", "string"))
    assert_false(json.validate_schema("42", "string")) fr fr Test number schema validation
    assert_true(json.validate_schema("42", "number"))
    assert_true(json.validate_schema("3.14", "number"))
    assert_false(json.validate_schema("\"42\"", "number")) fr fr Test boolean schema validation
    assert_true(json.validate_schema("true", "boolean"))
    assert_true(json.validate_schema("false", "boolean"))
    assert_false(json.validate_schema("\"true\"", "boolean")) fr fr Test null schema validation
    assert_true(json.validate_schema("null", "null"))
    assert_false(json.validate_schema("\"null\"", "null"))
}

slay test_json_stringify() {
    test_start("JSON Stringify - to_json() and stringify()") fr fr Test to_json function
    assert_eq_string(json.to_json("hello"), "\"hello\"")
    assert_eq_string(json.to_json("42"), "42")
    assert_eq_string(json.to_json("true"), "true") fr fr Test stringify alias
    assert_eq_string(json.stringify("hello"), "\"hello\"")
    assert_eq_string(json.stringify("42"), "42")
    assert_eq_string(json.stringify("true"), "true")
    assert_eq_string(json.stringify("false"), "false")
    assert_eq_string(json.stringify("null"), "null") fr fr Test object/array passthrough
    assert_eq_string(json.stringify("{}"), "{}")
    assert_eq_string(json.stringify("[]"), "[]")
}

slay test_json_type_conversion() {
    test_start("JSON Type Conversion") fr fr Test object_to_map conversion
    sus obj_map tea = json.object_to_map("{\"key\": \"value\"}")
    assert_true(json.string_contains(obj_map, "MAP:"))
    assert_true(json.string_contains(obj_map, "key")) fr fr Test array_to_slice conversion
    sus arr_slice tea = json.array_to_slice("[1, 2, 3]")
    assert_true(json.string_contains(arr_slice, "SLICE:"))
    assert_true(json.string_contains(arr_slice, "1")) fr fr Test error handling for invalid inputs
    sus invalid_obj tea = json.object_to_map("invalid")
    assert_true(json.string_contains(invalid_obj, "ERROR"))
    
    sus invalid_arr tea = json.array_to_slice("invalid")
    assert_true(json.string_contains(invalid_arr, "ERROR"))
}

slay test_json_minify() {
    test_start("JSON Minify") fr fr Test minification
    sus spaced_json tea = "{ \"name\" : \"John\" , \"age\" : 30 }"
    sus minified tea = json.minify(spaced_json) fr fr Should remove spaces around punctuation
    assert_true(string_length(minified) < string_length(spaced_json))
    assert_false(json.string_contains(minified, " : "))
    assert_false(json.string_contains(minified, " , "))
}

slay test_json_pretty_print() {
    test_start("JSON Pretty Print") fr fr Test pretty printing
    sus compact_json tea = "{\"name\":\"John\",\"age\":30}"
    sus pretty_json tea = json.pretty_print(compact_json) fr fr Should contain original data
    assert_true(json.string_contains(pretty_json, "name"))
    assert_true(json.string_contains(pretty_json, "John"))
    assert_true(json.string_contains(pretty_json, "age"))
    assert_true(json.string_contains(pretty_json, "30")) fr fr Should be longer due to formatting
    assert_true(string_length(pretty_json) > string_length(compact_json))
}

slay test_json_escape_unescape() {
    test_start("JSON Escape/Unescape") fr fr Test escape sequences
    sus original tea = "Hello\nWorld\t\"Quote\""
    sus escaped tea = json.escape_string(original) fr fr Should contain escape sequences
    assert_true(json.string_contains(escaped, "\\n"))
    assert_true(json.string_contains(escaped, "\\t"))
    assert_true(json.string_contains(escaped, "\\\"")) fr fr Test unescape
    sus unescaped tea = json.unescape_string(escaped)
    assert_true(json.string_contains(unescaped, "Hello"))
    assert_true(json.string_contains(unescaped, "World"))
}

slay test_json_numeric_validation() {
    test_start("JSON Numeric Validation") fr fr Test valid numbers
    assert_true(json.is_numeric("42"))
    assert_true(json.is_numeric("3.14"))
    assert_true(json.is_numeric("-42"))
    assert_true(json.is_numeric("-3.14"))
    assert_true(json.is_numeric("0")) fr fr Test invalid numbers
    assert_false(json.is_numeric("abc"))
    assert_false(json.is_numeric(""))
    assert_false(json.is_numeric("3.14.15")) fr fr Multiple dots
    assert_false(json.is_numeric("3.14abc")) fr fr Mixed
}

slay test_json_string_utilities() {
    test_start("JSON String Utilities") fr fr Test trim
    sus trimmed tea = json.string_trim("  hello  ")
    assert_eq_string(trimmed, "hello") fr fr Test starts_with
    assert_true(json.string_starts_with("hello world", "hello"))
    assert_false(json.string_starts_with("hello world", "world")) fr fr Test ends_with
    assert_true(json.string_ends_with("hello world", "world"))
    assert_false(json.string_ends_with("hello world", "hello"))
}

slay test_json_round_trip() {
    test_start("JSON Round-Trip Processing") fr fr Test string round-trip
    sus original_str tea = "hello"
    sus stringified tea = json.stringify(original_str)
    sus parsed tea = json.parse_value(stringified)
    assert_eq_string(parsed, original_str) fr fr Test number round-trip
    sus original_num tea = "42"
    sus stringified_num tea = json.stringify(original_num)
    sus parsed_num tea = json.parse_value(stringified_num)
    assert_eq_string(parsed_num, original_num)
}

slay test_json_edge_cases() {
    test_start("JSON Edge Cases") fr fr Test empty string handling
    sus empty_str_json tea = "\"\""
    sus empty_result tea = json.parse_value(empty_str_json)
    assert_eq_string(empty_result, "") fr fr Test whitespace handling
    sus whitespace_json tea = "  \"hello\"  "
    sus whitespace_result tea = json.parse_value(whitespace_json)
    assert_eq_string(whitespace_result, "hello") fr fr Test zero number
    sus zero_json tea = "0"
    sus zero_result tea = json.parse_value(zero_json)
    assert_eq_string(zero_result, "0")
}

slay test_json_complex_strings() {
    test_start("JSON Complex String Handling") fr fr Test strings with special characters
    sus special_json tea = "\"Hello\\nWorld\\t!\""
    sus special_result tea = json.parse_value(special_json)
    assert_true(json.string_contains(special_result, "Hello"))
    assert_true(json.string_contains(special_result, "World")) fr fr Test unicode handling (basic)
    sus unicode_json tea = "\"Hello 🌍\""
    sus unicode_result tea = json.parse_value(unicode_json)
    assert_true(json.string_contains(unicode_result, "Hello"))
}

slay test_json_performance_basics() {
    test_start("JSON Performance Basics") fr fr Test large number handling
    sus large_num tea = "1234567890123456789"
    assert_true(json.is_numeric(large_num))
    assert_true(json.validate(large_num)) fr fr Test long string handling
    sus long_str tea = "\"This is a very long string to test performance\""
    sus long_result tea = json.parse_value(long_str)
    assert_true(string_length(long_result) > 20)
}

slay test_json_api_consistency() {
    test_start("JSON API Consistency") fr fr Test that main API functions work
    sus test_data tea = "\"test\"" fr fr All these should work without errors
    sus parsed tea = json.parse(test_data)
    assert_eq_string(parsed, "test")
    
    sus stringified tea = json.stringify("test")
    assert_eq_string(stringified, "\"test\"")
    
    assert_true(json.validate(test_data))
    
    sus minified tea = json.minify(test_data)
    assert_true(string_length(minified) > 0)
    
    sus pretty tea = json.pretty_print(test_data)
    assert_true(string_length(pretty) > 0)
}

slay run_all_json_tests() {
    vibez.spill("🔧 Running Enhanced CURSED JSON Library Tests")
    vibez.spill("=============================================")
    vibez.spill("RFC 7159 Compliant JSON Processing") fr fr Test new RFC 7159 features
    test_json_parse_main_api()
    test_json_parse_objects()
    test_json_parse_arrays()
    test_json_schema_validation()
    test_json_type_conversion() fr fr Test enhanced API
    test_json_parse_primitives()
    test_json_validation()
    test_json_stringify()
    test_json_minify()
    test_json_pretty_print()
    test_json_escape_unescape()
    test_json_numeric_validation()
    test_json_string_utilities()
    test_json_round_trip()
    test_json_edge_cases()
    test_json_complex_strings()
    test_json_performance_basics()
    test_json_api_consistency()
    
    print_test_summary()
}

fr fr Auto-run tests when this file is executed
run_all_json_tests()
