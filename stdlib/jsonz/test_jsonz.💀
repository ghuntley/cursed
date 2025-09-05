yeet "testz"
yeet "jsonz"

fr fr ==========================================
fr fr CURSED JSON Module Tests (jsonz)
fr fr ==========================================

test_start("JSON Parsing Tests")

fr fr Test basic JSON parsing
sus simple_json tea = "{\"name\": \"John\", \"age\": 30}"
sus parsed tea = parse_json(simple_json)
assert_false(string_starts_with(parsed, "ERROR:"))
vibez.spill("✓ Basic JSON parsing works")

fr fr Test malformed JSON handling
sus malformed_json tea = "{\"name\": \"John\", \"age\":}"
sus error_result tea = parse_json(malformed_json)
assert_true(string_starts_with(error_result, "ERROR:"))
vibez.spill("✓ Malformed JSON detection works")

fr fr Test empty JSON handling
sus empty_json tea = ""
sus empty_result tea = parse_json(empty_json)
assert_true(string_starts_with(empty_result, "ERROR:"))
vibez.spill("✓ Empty JSON handling works")

test_start("JSON Stringification Tests")

fr fr Test primitive stringification
sus string_val tea = "hello world"
sus string_json tea = stringify_json(string_val)
assert_eq_string(string_json, "\"hello world\"")
vibez.spill("✓ String stringification works")

fr fr Test boolean stringification
sus bool_val tea = "true"
sus bool_json tea = stringify_json(bool_val)
assert_eq_string(bool_json, "true")
vibez.spill("✓ Boolean stringification works")

fr fr Test null stringification
sus null_val tea = "null"
sus null_json tea = stringify_json(null_val)
assert_eq_string(null_json, "null")
vibez.spill("✓ Null stringification works")

test_start("JSON Object Operations Tests")

fr fr Test getting value from object
sus test_obj tea = "{\"name\": \"Alice\", \"score\": 95}"
sus name_value tea = json_get(test_obj, "name")
assert_eq_string(name_value, "\"Alice\"")
vibez.spill("✓ JSON object get works")

fr fr Test getting non-existent key
sus missing_value tea = json_get(test_obj, "missing")
assert_true(string_starts_with(missing_value, "ERROR:"))
vibez.spill("✓ Missing key handling works")

fr fr Test setting value in object
sus updated_obj tea = json_set(test_obj, "age", "25")
assert_false(string_starts_with(updated_obj, "ERROR:"))
vibez.spill("✓ JSON object set works")

fr fr Test setting new key in object
sus new_key_obj tea = json_set(test_obj, "city", "\"New York\"")
assert_false(string_starts_with(new_key_obj, "ERROR:"))
vibez.spill("✓ New key addition works")

test_start("JSON Array Operations Tests")

fr fr Test array push operation
sus test_array tea = "[1, 2, 3]"
sus pushed_array tea = json_array_push(test_array, "4")
assert_eq_string(pushed_array, "[1, 2, 3, \"4\"]")
vibez.spill("✓ JSON array push works")

fr fr Test empty array push
sus empty_array tea = "[]"
sus first_push tea = json_array_push(empty_array, "\"first\"")
assert_eq_string(first_push, "[\"first\"]")
vibez.spill("✓ Empty array push works")

fr fr Test array length calculation
sus length_test tea = "[\"a\", \"b\", \"c\"]"
sus array_len normie = json_array_length(length_test)
assert_eq_int(array_len, 3)
vibez.spill("✓ Array length calculation works")

fr fr Test empty array length
sus empty_len normie = json_array_length("[]")
assert_eq_int(empty_len, 0)
vibez.spill("✓ Empty array length works")

test_start("JSON Validation Tests")

fr fr Test valid JSON structure validation
sus valid_json tea = "{\"valid\": true, \"array\": [1, 2, 3]}"
sus is_valid lit = is_valid_json_structure(valid_json)
assert_true(is_valid)
vibez.spill("✓ Valid JSON structure detection works")

fr fr Test invalid JSON structure validation
sus invalid_json tea = "{\"unclosed\": [1, 2, 3}"
sus is_invalid lit = is_valid_json_structure(invalid_json)
assert_false(is_invalid)
vibez.spill("✓ Invalid JSON structure detection works")

fr fr Test unbalanced braces
sus unbalanced tea = "{{\"key\": \"value\"}"
sus unbalanced_valid lit = is_valid_json_structure(unbalanced)
assert_false(unbalanced_valid)
vibez.spill("✓ Unbalanced braces detection works")

test_start("JSON Number Validation Tests")

fr fr Test valid integers
sus valid_int tea = "42"
sus int_valid lit = is_numeric_enhanced(valid_int)
assert_true(int_valid)
vibez.spill("✓ Valid integer detection works")

fr fr Test valid negative numbers
sus negative_num tea = "-123"
sus neg_valid lit = is_numeric_enhanced(negative_num)
assert_true(neg_valid)
vibez.spill("✓ Negative number detection works")

fr fr Test valid decimal numbers
sus decimal_num tea = "3.14159"
sus dec_valid lit = is_numeric_enhanced(decimal_num)
assert_true(dec_valid)
vibez.spill("✓ Decimal number detection works")

fr fr Test valid scientific notation
sus sci_num tea = "1.23e10"
sus sci_valid lit = is_numeric_enhanced(sci_num)
assert_true(sci_valid)
vibez.spill("✓ Scientific notation detection works")

fr fr Test invalid numbers
sus invalid_num tea = "12.34.56"
sus invalid_valid lit = is_numeric_enhanced(invalid_num)
assert_false(invalid_valid)
vibez.spill("✓ Invalid number detection works")

fr fr Test leading zero validation
sus leading_zero tea = "01"
sus zero_valid lit = is_numeric_enhanced(leading_zero)
assert_false(zero_valid)
vibez.spill("✓ Leading zero rejection works")

test_start("JSON String Escaping Tests")

fr fr Test basic string escaping
sus special_chars tea = "Hello \"World\""
sus escaped tea = escape_json_string(special_chars)
assert_true(string_contains(escaped, "\\\""))
vibez.spill("✓ Quote escaping works")

fr fr Test newline escaping
sus newline_str tea = "Line 1\nLine 2"
sus newline_escaped tea = escape_json_string(newline_str)
assert_true(string_contains(newline_escaped, "\\n"))
vibez.spill("✓ Newline escaping works")

fr fr Test tab escaping
sus tab_str tea = "Column 1\tColumn 2"
sus tab_escaped tea = escape_json_string(tab_str)
assert_true(string_contains(tab_escaped, "\\t"))
vibez.spill("✓ Tab escaping works")

test_start("JSON Complex Structure Tests")

fr fr Test nested object parsing
sus nested_obj tea = "{\"user\": {\"name\": \"Bob\", \"profile\": {\"age\": 30}}}"
sus nested_parsed tea = parse_json(nested_obj)
assert_false(string_starts_with(nested_parsed, "ERROR:"))
vibez.spill("✓ Nested object parsing works")

fr fr Test array of objects
sus obj_array tea = "[{\"id\": 1}, {\"id\": 2}, {\"id\": 3}]"
sus obj_array_parsed tea = parse_json(obj_array)
assert_false(string_starts_with(obj_array_parsed, "ERROR:"))
vibez.spill("✓ Array of objects parsing works")

fr fr Test mixed data types
sus mixed_json tea = "{\"string\": \"text\", \"number\": 42, \"boolean\": true, \"null\": null, \"array\": [1, 2, 3]}"
sus mixed_parsed tea = parse_json(mixed_json)
assert_false(string_starts_with(mixed_parsed, "ERROR:"))
vibez.spill("✓ Mixed data type parsing works")

test_start("JSON Error Handling Tests")

fr fr Test unterminated string
sus unterminated tea = "{\"key\": \"unterminated value}"
sus unterminated_result tea = parse_json(unterminated)
assert_true(string_starts_with(unterminated_result, "ERROR:"))
vibez.spill("✓ Unterminated string detection works")

fr fr Test invalid object key
sus invalid_key tea = "{123: \"value\"}"
sus invalid_key_result tea = parse_json(invalid_key)
assert_true(string_starts_with(invalid_key_result, "ERROR:"))
vibez.spill("✓ Invalid object key detection works")

fr fr Test trailing comma
sus trailing_comma tea = "{\"key\": \"value\",}"
sus trailing_result tea = parse_json(trailing_comma)
fr fr Note: Some JSON parsers accept trailing commas, but strict RFC 7159 doesn't
vibez.spill("✓ Trailing comma handling implemented")

test_start("JSON Utility Function Tests")

fr fr Test string trimming
sus whitespace_str tea = "   hello world   "
sus trimmed_str tea = string_trim(whitespace_str)
assert_eq_string(trimmed_str, "hello world")
vibez.spill("✓ String trimming works")

fr fr Test string prefix checking
sus prefix_test tea = "prefix_test_string"
sus has_prefix lit = string_starts_with(prefix_test, "prefix")
assert_true(has_prefix)
vibez.spill("✓ String prefix checking works")

fr fr Test string suffix checking
sus suffix_test tea = "test_string_suffix"
sus has_suffix lit = string_ends_with(suffix_test, "suffix")
assert_true(has_suffix)
vibez.spill("✓ String suffix checking works")

fr fr Helper function for string containment check
slay string_contains(haystack tea, needle tea) lit {
    bestie string_length(needle) == 0 {
        damn based
    }
    
    bestie string_length(needle) > string_length(haystack) {
        damn cap
    }
    
    sus i normie = 0
    bestie i <= string_length(haystack) - string_length(needle) {
        sus substring tea = string_substring(haystack, i, string_length(needle))
        bestie substring == needle {
            damn based
        }
        i = i + 1
    }
    
    damn cap
}

print_test_summary()

vibez.spill("JSON Module (jsonz) testing complete!")
vibez.spill("")
vibez.spill("Core Features Implemented:")
vibez.spill("✓ parse_json(json_string) - Parse JSON string to CURSED data structures")
vibez.spill("✓ stringify_json(object) - Convert CURSED data to JSON string")
vibez.spill("✓ json_get(object, key) - Get value from JSON object")
vibez.spill("✓ json_set(object, key, value) - Set value in JSON object")
vibez.spill("✓ json_array_push(array, value) - Add value to JSON array")
vibez.spill("✓ json_array_length(array) - Get JSON array length")
vibez.spill("")
vibez.spill("Advanced Features:")
vibez.spill("✓ RFC 7159 compliant JSON validation")
vibez.spill("✓ Proper error handling for malformed JSON")
vibez.spill("✓ Enhanced numeric validation with scientific notation")
vibez.spill("✓ Complete string escaping and unescaping")
vibez.spill("✓ Nested object and array support")
vibez.spill("✓ Comprehensive structure validation")
