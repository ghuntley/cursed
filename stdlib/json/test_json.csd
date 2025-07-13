yeet "testz"
yeet "json"

# ==========================================
# CURSED JSON Library Test Suite
# RFC 7159 Compliant JSON Processing Tests
# ==========================================

slay test_json_parse_primitives() {
    test_start("JSON Parse Primitive Values")
    
    # Test parsing strings
    sus str_result tea = json.parse_value("\"hello world\"")
    assert_eq_string(str_result, "hello world")
    
    # Test parsing numbers
    sus num_result tea = json.parse_value("42")
    assert_eq_string(num_result, "42")
    
    sus float_result tea = json.parse_value("3.14")
    assert_eq_string(float_result, "3.14")
    
    # Test parsing booleans
    sus true_result tea = json.parse_value("true")
    assert_eq_string(true_result, "true")
    
    sus false_result tea = json.parse_value("false")
    assert_eq_string(false_result, "false")
    
    # Test parsing null
    sus null_result tea = json.parse_value("null")
    assert_eq_string(null_result, "null")
}

slay test_json_validation() {
    test_start("JSON Validation")
    
    # Test valid JSON
    assert_true(json.validate("{\"name\": \"John\"}"))
    assert_true(json.validate("[1, 2, 3]"))
    assert_true(json.validate("\"hello\""))
    assert_true(json.validate("42"))
    assert_true(json.validate("true"))
    assert_true(json.validate("false"))
    assert_true(json.validate("null"))
    
    # Test invalid JSON (basic cases)
    assert_false(json.validate("invalid"))
    assert_false(json.validate(""))
    assert_false(json.validate("undefined"))
}

slay test_json_stringify() {
    test_start("JSON Stringify")
    
    # Test stringify primitives
    assert_eq_string(json.stringify("hello"), "\"hello\"")
    assert_eq_string(json.stringify("42"), "42")
    assert_eq_string(json.stringify("true"), "true")
    assert_eq_string(json.stringify("false"), "false")
    assert_eq_string(json.stringify("null"), "null")
}

slay test_json_minify() {
    test_start("JSON Minify")
    
    # Test minification
    sus spaced_json tea = "{ \"name\" : \"John\" , \"age\" : 30 }"
    sus minified tea = json.minify(spaced_json)
    
    # Should remove spaces around punctuation
    assert_true(string_length(minified) < string_length(spaced_json))
    assert_false(string_contains(minified, " : "))
    assert_false(string_contains(minified, " , "))
}

slay test_json_pretty_print() {
    test_start("JSON Pretty Print")
    
    # Test pretty printing
    sus compact_json tea = "{\"name\":\"John\",\"age\":30}"
    sus pretty_json tea = json.pretty_print(compact_json)
    
    # Should contain original data
    assert_true(string_contains(pretty_json, "name"))
    assert_true(string_contains(pretty_json, "John"))
    assert_true(string_contains(pretty_json, "age"))
    assert_true(string_contains(pretty_json, "30"))
    
    # Should be longer due to formatting
    assert_true(string_length(pretty_json) > string_length(compact_json))
}

slay test_json_escape_unescape() {
    test_start("JSON Escape/Unescape")
    
    # Test escape sequences
    sus original tea = "Hello\nWorld\t\"Quote\""
    sus escaped tea = json.escape_string(original)
    
    # Should contain escape sequences
    assert_true(string_contains(escaped, "\\n"))
    assert_true(string_contains(escaped, "\\t"))
    assert_true(string_contains(escaped, "\\\""))
    
    # Test unescape
    sus unescaped tea = json.unescape_string(escaped)
    assert_true(string_contains(unescaped, "Hello"))
    assert_true(string_contains(unescaped, "World"))
}

slay test_json_numeric_validation() {
    test_start("JSON Numeric Validation")
    
    # Test valid numbers
    assert_true(json.is_numeric("42"))
    assert_true(json.is_numeric("3.14"))
    assert_true(json.is_numeric("-42"))
    assert_true(json.is_numeric("-3.14"))
    assert_true(json.is_numeric("0"))
    
    # Test invalid numbers
    assert_false(json.is_numeric("abc"))
    assert_false(json.is_numeric(""))
    assert_false(json.is_numeric("3.14.15"))  # Multiple dots
    assert_false(json.is_numeric("3.14abc"))  # Mixed
}

slay test_json_string_utilities() {
    test_start("JSON String Utilities")
    
    # Test trim
    sus trimmed tea = json.string_trim("  hello  ")
    assert_eq_string(trimmed, "hello")
    
    # Test starts_with
    assert_true(json.string_starts_with("hello world", "hello"))
    assert_false(json.string_starts_with("hello world", "world"))
    
    # Test ends_with
    assert_true(json.string_ends_with("hello world", "world"))
    assert_false(json.string_ends_with("hello world", "hello"))
}

slay test_json_round_trip() {
    test_start("JSON Round-Trip Processing")
    
    # Test string round-trip
    sus original_str tea = "hello"
    sus stringified tea = json.stringify(original_str)
    sus parsed tea = json.parse_value(stringified)
    assert_eq_string(parsed, original_str)
    
    # Test number round-trip
    sus original_num tea = "42"
    sus stringified_num tea = json.stringify(original_num)
    sus parsed_num tea = json.parse_value(stringified_num)
    assert_eq_string(parsed_num, original_num)
}

slay test_json_edge_cases() {
    test_start("JSON Edge Cases")
    
    # Test empty string handling
    sus empty_str_json tea = "\"\""
    sus empty_result tea = json.parse_value(empty_str_json)
    assert_eq_string(empty_result, "")
    
    # Test whitespace handling
    sus whitespace_json tea = "  \"hello\"  "
    sus whitespace_result tea = json.parse_value(whitespace_json)
    assert_eq_string(whitespace_result, "hello")
    
    # Test zero number
    sus zero_json tea = "0"
    sus zero_result tea = json.parse_value(zero_json)
    assert_eq_string(zero_result, "0")
}

slay test_json_complex_strings() {
    test_start("JSON Complex String Handling")
    
    # Test strings with special characters
    sus special_json tea = "\"Hello\\nWorld\\t!\""
    sus special_result tea = json.parse_value(special_json)
    assert_true(string_contains(special_result, "Hello"))
    assert_true(string_contains(special_result, "World"))
    
    # Test unicode handling (basic)
    sus unicode_json tea = "\"Hello 🌍\""
    sus unicode_result tea = json.parse_value(unicode_json)
    assert_true(string_contains(unicode_result, "Hello"))
}

slay test_json_performance_basics() {
    test_start("JSON Performance Basics")
    
    # Test large number handling
    sus large_num tea = "1234567890123456789"
    assert_true(json.is_numeric(large_num))
    assert_true(json.validate(large_num))
    
    # Test long string handling
    sus long_str tea = "\"This is a very long string to test performance\""
    sus long_result tea = json.parse_value(long_str)
    assert_true(string_length(long_result) > 20)
}

slay test_json_api_consistency() {
    test_start("JSON API Consistency")
    
    # Test that main API functions work
    sus test_data tea = "\"test\""
    
    # All these should work without errors
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
    vibez.spill("🔧 Running CURSED JSON Library Tests")
    vibez.spill("====================================")
    
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

# Auto-run tests when this file is executed
run_all_json_tests()
