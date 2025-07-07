yeet "testz"
yeet "json"
yeet "string"
yeet "collections"

fr fr ========================================
fr fr CURSED JSON Library Test Suite
fr fr ========================================

slay test_json_parse_primitives() {
    testz.test_start("JSON Parse Primitive Values")
    
    fr fr Test parsing strings
    sus str_result extra = json.parse_value("\"hello world\"")
    testz.assert_eq_string(json.to_string(str_result), "hello world")
    
    fr fr Test parsing numbers
    sus num_result extra = json.parse_value("42")
    testz.assert_eq_int(json.to_integer(num_result), 42)
    
    sus float_result extra = json.parse_value("3.14")
    testz.assert_eq_string(tea(json.to_number(float_result)), "3.14")
    
    fr fr Test parsing booleans
    sus true_result extra = json.parse_value("true")
    testz.assert_true(json.to_boolean(true_result))
    
    sus false_result extra = json.parse_value("false")
    testz.assert_false(json.to_boolean(false_result))
    
    fr fr Test parsing null
    sus null_result extra = json.parse_value("null")
    testz.assert_true(json.is_null(null_result))
}

slay test_json_parse_objects() {
    testz.test_start("JSON Parse Objects")
    
    fr fr Test simple object
    sus simple_json tea = "{\"name\": \"John\", \"age\": 30}"
    sus obj map = json.parse(simple_json)
    
    testz.assert_eq_string(json.get_value(obj, "name"), "John")
    testz.assert_eq_string(json.get_value(obj, "age"), "30")
    testz.assert_true(json.has_key(obj, "name"))
    testz.assert_true(json.has_key(obj, "age"))
    testz.assert_false(json.has_key(obj, "email"))
    
    fr fr Test nested object
    sus nested_json tea = "{\"user\": {\"name\": \"Jane\", \"details\": {\"age\": 25}}}"
    sus nested_obj map = json.parse(nested_json)
    
    testz.assert_true(json.has_key(nested_obj, "user"))
    testz.assert_eq_string(json.get_path(nested_obj, "user.name"), "Jane")
    testz.assert_eq_string(json.get_path(nested_obj, "user.details.age"), "25")
}

slay test_json_parse_arrays() {
    testz.test_start("JSON Parse Arrays")
    
    fr fr Test simple array
    sus simple_array_json tea = "[1, 2, 3, 4, 5]"
    sus arr [extra] = json.parse_array(simple_array_json)
    
    testz.assert_eq_int(json.array_length(arr), 5)
    testz.assert_eq_string(json.get_array_value(arr, 0), "1")
    testz.assert_eq_string(json.get_array_value(arr, 4), "5")
    
    fr fr Test mixed array
    sus mixed_array_json tea = "[\"hello\", 42, true, null]"
    sus mixed_arr [extra] = json.parse_array(mixed_array_json)
    
    testz.assert_eq_int(json.array_length(mixed_arr), 4)
    testz.assert_eq_string(json.get_array_value(mixed_arr, 0), "hello")
    testz.assert_eq_string(json.get_array_value(mixed_arr, 1), "42")
    testz.assert_eq_string(json.get_array_value(mixed_arr, 2), "true")
    testz.assert_true(json.is_null(json.parse_value(json.get_array_value(mixed_arr, 3))))
    
    fr fr Test nested array
    sus nested_array_json tea = "[[1, 2], [3, 4], [5, 6]]"
    sus nested_arr [extra] = json.parse_array(nested_array_json)
    
    testz.assert_eq_int(json.array_length(nested_arr), 3)
}

slay test_json_stringify_primitives() {
    testz.test_start("JSON Stringify Primitive Values")
    
    fr fr Test stringify primitives by creating objects
    sus obj map = map_new()
    obj = map_set(obj, "string", "hello")
    obj = map_set(obj, "number", "42")
    obj = map_set(obj, "boolean", "true")
    obj = map_set(obj, "null", "null")
    
    sus json_str tea = json.stringify(obj)
    testz.assert_true(string_contains(json_str, "\"string\""))
    testz.assert_true(string_contains(json_str, "\"hello\""))
    testz.assert_true(string_contains(json_str, "\"number\""))
    testz.assert_true(string_contains(json_str, "42"))
}

slay test_json_round_trip() {
    testz.test_start("JSON Round-Trip Parse/Stringify")
    
    fr fr Test object round-trip
    sus original_json tea = "{\"name\":\"Alice\",\"age\":25,\"active\":true}"
    sus parsed_obj map = json.parse(original_json)
    sus stringified_json tea = json.stringify(parsed_obj)
    
    fr fr Parse both versions to compare
    sus original_parsed map = json.parse(original_json)
    sus round_trip_parsed map = json.parse(stringified_json)
    
    testz.assert_eq_string(json.get_value(original_parsed, "name"), json.get_value(round_trip_parsed, "name"))
    testz.assert_eq_string(json.get_value(original_parsed, "age"), json.get_value(round_trip_parsed, "age"))
    testz.assert_eq_string(json.get_value(original_parsed, "active"), json.get_value(round_trip_parsed, "active"))
    
    fr fr Test array round-trip
    sus original_array_json tea = "[1,2,3,\"test\",true]"
    sus parsed_array [extra] = json.parse_array(original_array_json)
    sus stringified_array_json tea = json.stringify_array(parsed_array)
    
    testz.assert_true(string_contains(stringified_array_json, "1"))
    testz.assert_true(string_contains(stringified_array_json, "2"))
    testz.assert_true(string_contains(stringified_array_json, "3"))
    testz.assert_true(string_contains(stringified_array_json, "test"))
    testz.assert_true(string_contains(stringified_array_json, "true"))
}

slay test_json_validation() {
    testz.test_start("JSON Validation")
    
    fr fr Test valid JSON
    testz.assert_true(json.validate("{\"name\": \"John\", \"age\": 30}"))
    testz.assert_true(json.validate("[1, 2, 3]"))
    testz.assert_true(json.validate("\"hello\""))
    testz.assert_true(json.validate("42"))
    testz.assert_true(json.validate("true"))
    testz.assert_true(json.validate("null"))
    
    fr fr Test invalid JSON
    testz.assert_false(json.validate("{name: \"John\"}"))  // Missing quotes
    testz.assert_false(json.validate("{\"name\": \"John\",}"))  // Trailing comma
    testz.assert_false(json.validate("[1, 2, 3,]"))  // Trailing comma
    testz.assert_false(json.validate("{\"name\": }"))  // Missing value
    testz.assert_false(json.validate("undefined"))  // Invalid literal
    testz.assert_false(json.validate("'single quotes'"))  // Single quotes
}

slay test_json_empty_structures() {
    testz.test_start("JSON Empty Structures")
    
    fr fr Test empty object
    sus empty_obj_json tea = "{}"
    sus empty_obj map = json.parse(empty_obj_json)
    
    testz.assert_true(json.is_empty(empty_obj))
    testz.assert_eq_int(json.size(empty_obj), 0)
    testz.assert_false(json.has_key(empty_obj, "any_key"))
    
    fr fr Test empty array
    sus empty_arr_json tea = "[]"
    sus empty_arr [extra] = json.parse_array(empty_arr_json)
    
    testz.assert_eq_int(json.array_length(empty_arr), 0)
    testz.assert_true(json.is_empty(empty_arr))
}

slay test_json_escape_sequences() {
    testz.test_start("JSON Escape Sequences")
    
    fr fr Test string with escape sequences
    sus escaped_json tea = "{\"text\": \"Hello\\nWorld\\t\\\"Quote\\\"\"}"
    sus obj map = json.parse(escaped_json)
    
    sus text_value tea = json.get_value(obj, "text")
    testz.assert_true(string_contains(text_value, "Hello"))
    testz.assert_true(string_contains(text_value, "World"))
    
    fr fr Test escape and unescape functions
    sus original tea = "Hello\nWorld\t\"Quote\""
    sus escaped tea = json.escape_string(original)
    sus unescaped tea = json.unescape_string(escaped)
    
    testz.assert_true(string_contains(escaped, "\\n"))
    testz.assert_true(string_contains(escaped, "\\t"))
    testz.assert_true(string_contains(escaped, "\\\""))
    testz.assert_eq_string(unescaped, original)
}

slay test_json_nested_structures() {
    testz.test_start("JSON Nested Structures")
    
    fr fr Test deeply nested object
    sus nested_json tea = "{\"level1\": {\"level2\": {\"level3\": {\"value\": 42}}}}"
    sus nested_obj map = json.parse(nested_json)
    
    testz.assert_true(json.has_path(nested_obj, "level1"))
    testz.assert_true(json.has_path(nested_obj, "level1.level2"))
    testz.assert_true(json.has_path(nested_obj, "level1.level2.level3"))
    testz.assert_true(json.has_path(nested_obj, "level1.level2.level3.value"))
    testz.assert_eq_string(json.get_path(nested_obj, "level1.level2.level3.value"), "42")
    
    fr fr Test array of objects
    sus array_objects_json tea = "[{\"id\": 1, \"name\": \"A\"}, {\"id\": 2, \"name\": \"B\"}]"
    sus array_objects [extra] = json.parse_array(array_objects_json)
    
    testz.assert_eq_int(json.array_length(array_objects), 2)
}

slay test_json_pretty_print() {
    testz.test_start("JSON Pretty Print")
    
    fr fr Test pretty printing
    sus compact_json tea = "{\"name\":\"John\",\"age\":30,\"city\":\"New York\"}"
    sus pretty_json tea = json.pretty_print(compact_json)
    
    testz.assert_true(string_contains(pretty_json, "\"name\""))
    testz.assert_true(string_contains(pretty_json, "\"John\""))
    testz.assert_true(string_contains(pretty_json, "\"age\""))
    testz.assert_true(string_contains(pretty_json, "30"))
    
    fr fr Test minification
    sus minified_json tea = json.minify(pretty_json)
    testz.assert_true(string_len(minified_json) < string_len(pretty_json))
    testz.assert_false(string_contains(minified_json, "  "))  // No double spaces
    testz.assert_false(string_contains(minified_json, "\n"))  // No newlines
}

slay test_json_utility_functions() {
    testz.test_start("JSON Utility Functions")
    
    fr fr Test get_value with default
    sus obj map = map_new()
    obj = map_set(obj, "existing", "value")
    
    testz.assert_eq_string(json.get_value_or_default(obj, "existing", "default"), "value")
    testz.assert_eq_string(json.get_value_or_default(obj, "missing", "default"), "default")
    
    fr fr Test set_value and has_key
    obj = json.set_value(obj, "new_key", "new_value")
    testz.assert_true(json.has_key(obj, "new_key"))
    testz.assert_eq_string(json.get_value(obj, "new_key"), "new_value")
    
    fr fr Test remove_key
    obj = json.remove_key(obj, "existing")
    testz.assert_false(json.has_key(obj, "existing"))
    testz.assert_true(json.has_key(obj, "new_key"))
    
    fr fr Test get_keys
    sus keys [tea] = json.get_keys(obj)
    testz.assert_eq_int(len(keys), 1)
    testz.assert_eq_string(keys[0], "new_key")
}

slay test_json_array_operations() {
    testz.test_start("JSON Array Operations")
    
    fr fr Test array manipulation
    sus arr [extra] = json.parse_array("[1, 2, 3]")
    
    testz.assert_eq_int(json.array_length(arr), 3)
    testz.assert_eq_string(json.get_array_value(arr, 1), "2")
    
    fr fr Test array modification
    arr = json.set_array_value(arr, 1, "42")
    testz.assert_eq_string(json.get_array_value(arr, 1), "42")
    
    fr fr Test array push and pop
    arr = json.push_array_value(arr, "4")
    testz.assert_eq_int(json.array_length(arr), 4)
    testz.assert_eq_string(json.get_array_value(arr, 3), "4")
    
    sus popped_value tea = json.pop_array_value(arr)
    testz.assert_eq_string(popped_value, "4")
    testz.assert_eq_int(json.array_length(arr), 3)
}

slay test_json_type_checking() {
    testz.test_start("JSON Type Checking")
    
    fr fr Test type identification
    sus obj_val extra = json.parse_value("{\"key\": \"value\"}")
    sus arr_val extra = json.parse_value("[1, 2, 3]")
    sus str_val extra = json.parse_value("\"hello\"")
    sus num_val extra = json.parse_value("42")
    sus bool_val extra = json.parse_value("true")
    sus null_val extra = json.parse_value("null")
    
    testz.assert_true(json.is_object(obj_val))
    testz.assert_true(json.is_array(arr_val))
    testz.assert_true(json.is_string(str_val))
    testz.assert_true(json.is_number(num_val))
    testz.assert_true(json.is_boolean(bool_val))
    testz.assert_true(json.is_null(null_val))
    
    fr fr Test type names
    testz.assert_eq_string(json.get_type(obj_val), "object")
    testz.assert_eq_string(json.get_type(arr_val), "array")
    testz.assert_eq_string(json.get_type(str_val), "string")
    testz.assert_eq_string(json.get_type(num_val), "number")
    testz.assert_eq_string(json.get_type(bool_val), "boolean")
    testz.assert_eq_string(json.get_type(null_val), "null")
}

slay test_json_conversion_functions() {
    testz.test_start("JSON Conversion Functions")
    
    fr fr Test conversions
    sus str_val extra = json.parse_value("\"hello\"")
    sus num_val extra = json.parse_value("42")
    sus float_val extra = json.parse_value("3.14")
    sus bool_val extra = json.parse_value("true")
    
    testz.assert_eq_string(json.to_string(str_val), "hello")
    testz.assert_eq_int(json.to_integer(num_val), 42)
    testz.assert_eq_string(tea(json.to_number(float_val)), "3.14")
    testz.assert_true(json.to_boolean(bool_val))
}

slay test_json_merge_operations() {
    testz.test_start("JSON Merge Operations")
    
    fr fr Test simple merge
    sus obj1 map = json.parse("{\"a\": 1, \"b\": 2}")
    sus obj2 map = json.parse("{\"c\": 3, \"d\": 4}")
    sus merged map = json.merge(obj1, obj2)
    
    testz.assert_true(json.has_key(merged, "a"))
    testz.assert_true(json.has_key(merged, "b"))
    testz.assert_true(json.has_key(merged, "c"))
    testz.assert_true(json.has_key(merged, "d"))
    testz.assert_eq_int(json.size(merged), 4)
    
    fr fr Test array merge
    sus arr1 [extra] = json.parse_array("[1, 2, 3]")
    sus arr2 [extra] = json.parse_array("[4, 5, 6]")
    sus merged_arr [extra] = json.merge_arrays(arr1, arr2)
    
    testz.assert_eq_int(json.array_length(merged_arr), 6)
}

slay test_json_comparison_functions() {
    testz.test_start("JSON Comparison Functions")
    
    fr fr Test equality
    sus obj1 map = json.parse("{\"name\": \"John\", \"age\": 30}")
    sus obj2 map = json.parse("{\"name\": \"John\", \"age\": 30}")
    sus obj3 map = json.parse("{\"name\": \"Jane\", \"age\": 25}")
    
    testz.assert_true(json.deep_equals(obj1, obj2))
    testz.assert_false(json.deep_equals(obj1, obj3))
    
    fr fr Test array equality
    sus arr1 [extra] = json.parse_array("[1, 2, 3]")
    sus arr2 [extra] = json.parse_array("[1, 2, 3]")
    sus arr3 [extra] = json.parse_array("[1, 2, 4]")
    
    testz.assert_true(json.deep_equals(arr1, arr2))
    testz.assert_false(json.deep_equals(arr1, arr3))
}

slay test_json_copy_functions() {
    testz.test_start("JSON Copy Functions")
    
    fr fr Test copy
    sus original map = json.parse("{\"name\": \"John\", \"age\": 30}")
    sus copied map = json.copy(original)
    
    testz.assert_eq_string(json.get_value(copied, "name"), json.get_value(original, "name"))
    testz.assert_eq_string(json.get_value(copied, "age"), json.get_value(original, "age"))
    
    fr fr Test deep copy
    sus deep_copied map = json.deep_copy(original)
    testz.assert_eq_string(json.get_value(deep_copied, "name"), json.get_value(original, "name"))
    testz.assert_eq_string(json.get_value(deep_copied, "age"), json.get_value(original, "age"))
}

slay test_json_edge_cases() {
    testz.test_start("JSON Edge Cases")
    
    fr fr Test whitespace handling
    sus whitespace_json tea = "  {  \"name\"  :  \"John\"  }  "
    sus obj map = json.parse(whitespace_json)
    testz.assert_eq_string(json.get_value(obj, "name"), "John")
    
    fr fr Test Unicode handling
    sus unicode_json tea = "{\"message\": \"Hello 🌍\"}"
    sus unicode_obj map = json.parse(unicode_json)
    testz.assert_eq_string(json.get_value(unicode_obj, "message"), "Hello 🌍")
    
    fr fr Test large numbers
    sus large_num_json tea = "{\"big\": 1234567890123456789}"
    sus large_obj map = json.parse(large_num_json)
    testz.assert_eq_string(json.get_value(large_obj, "big"), "1234567890123456789")
    
    fr fr Test empty strings
    sus empty_str_json tea = "{\"empty\": \"\"}"
    sus empty_obj map = json.parse(empty_str_json)
    testz.assert_eq_string(json.get_value(empty_obj, "empty"), "")
}

slay run_all_json_tests() {
    vibez.spill("🔧 Running CURSED JSON Library Tests")
    vibez.spill("====================================")
    
    test_json_parse_primitives()
    test_json_parse_objects()
    test_json_parse_arrays()
    test_json_stringify_primitives()
    test_json_round_trip()
    test_json_validation()
    test_json_empty_structures()
    test_json_escape_sequences()
    test_json_nested_structures()
    test_json_pretty_print()
    test_json_utility_functions()
    test_json_array_operations()
    test_json_type_checking()
    test_json_conversion_functions()
    test_json_merge_operations()
    test_json_comparison_functions()
    test_json_copy_functions()
    test_json_edge_cases()
    
    testz.print_test_summary()
    damn testz.run_all_tests()
}

fr fr Auto-run tests when this file is executed
run_all_json_tests()
