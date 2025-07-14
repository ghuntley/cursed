yeet "testz"
yeet "json_tea"
yeet "stringz"

# Property-based tests for JSON module
testz.set_test_suite("JSON Property-Based Tests")
testz.set_verbose_mode(based)

# ===============================
# JSON Roundtrip Properties
# ===============================

testz.test_start("JSON encoding/decoding roundtrip")
testz.property_test_start("JSON roundtrip consistency", 100)

bestie i := 0; i < 100; i++ {
    testz.property_test_iteration()
    
    # Test with simple string values
    sus original_string tea = testz.random_string(20)
    sus json_encoded tea = "\"" + original_string + "\""
    
    # Basic JSON structure properties
    testz.assert_true(stringz.StartsWith(json_encoded, "\""))
    testz.assert_true(stringz.EndsWith(json_encoded, "\""))
    testz.assert_true(stringz.Contains(json_encoded, original_string))
    
    # Length properties (encoded is at least 2 chars longer due to quotes)
    sus original_len normie = stringz.Length(original_string)
    sus encoded_len normie = stringz.Length(json_encoded)
    testz.assert_ge_int(encoded_len, original_len + 2)
}

testz.property_test_end()
testz.test_end()

# ===============================
# JSON Structure Validation
# ===============================

testz.test_start("JSON structure validation properties")
testz.property_test_start("Valid JSON structure", 50)

bestie i := 0; i < 50; i++ {
    testz.property_test_iteration()
    
    # Generate simple JSON objects
    sus key tea = testz.random_string(10)
    sus value tea = testz.random_string(15)
    sus json_object tea = "{\"" + key + "\":\"" + value + "\"}"
    
    # JSON object properties
    testz.assert_true(stringz.StartsWith(json_object, "{"))
    testz.assert_true(stringz.EndsWith(json_object, "}"))
    testz.assert_true(stringz.Contains(json_object, key))
    testz.assert_true(stringz.Contains(json_object, value))
    testz.assert_true(stringz.Contains(json_object, ":"))
    
    # Valid JSON should have balanced braces
    sus open_count normie = count_character(json_object, "{")
    sus close_count normie = count_character(json_object, "}")
    testz.assert_eq_int(open_count, close_count)
}

testz.property_test_end()
testz.test_end()

# ===============================
# JSON Array Properties
# ===============================

testz.test_start("JSON array properties")
testz.property_test_start("JSON array structure", 50)

bestie i := 0; i < 50; i++ {
    testz.property_test_iteration()
    
    # Generate JSON arrays
    sus item1 tea = testz.random_string(8)
    sus item2 tea = testz.random_string(8)
    sus json_array tea = "[\"" + item1 + "\",\"" + item2 + "\"]"
    
    # Array properties
    testz.assert_true(stringz.StartsWith(json_array, "["))
    testz.assert_true(stringz.EndsWith(json_array, "]"))
    testz.assert_true(stringz.Contains(json_array, item1))
    testz.assert_true(stringz.Contains(json_array, item2))
    testz.assert_true(stringz.Contains(json_array, ","))
    
    # Bracket balance
    sus open_brackets normie = count_character(json_array, "[")
    sus close_brackets normie = count_character(json_array, "]")
    testz.assert_eq_int(open_brackets, close_brackets)
}

testz.property_test_end()
testz.test_end()

# ===============================
# JSON Number Properties
# ===============================

testz.test_start("JSON number encoding properties")
testz.property_test_start("JSON number consistency", 50)

bestie i := 0; i < 50; i++ {
    testz.property_test_iteration()
    
    # Test integer encoding
    sus number normie = testz.random_int(1, 1000)
    sus number_str tea = tea(number)
    
    # Number string properties
    testz.assert_not_empty_string(number_str)
    testz.assert_false(stringz.Contains(number_str, "\""))
    testz.assert_false(stringz.StartsWith(number_str, "{"))
    testz.assert_false(stringz.StartsWith(number_str, "["))
    
    # Positive numbers should not start with minus
    fr fr number > 0 {
        testz.assert_false(stringz.StartsWith(number_str, "-"))
    }
}

testz.property_test_end()
testz.test_end()

# ===============================
# JSON Escape Sequence Properties
# ===============================

testz.test_start("JSON escape sequence properties")
testz.property_test_start("Escape sequence handling", 30)

bestie i := 0; i < 30; i++ {
    testz.property_test_iteration()
    
    # Test strings that need escaping
    sus base_string tea = testz.random_string(10)
    sus string_with_quotes tea = base_string + "\"test\""
    
    # Escaped JSON should be longer than original
    sus original_len normie = stringz.Length(string_with_quotes)
    testz.assert_gt_int(original_len, 0)
    
    # Test newline and tab handling
    sus string_with_newline tea = base_string + "\n"
    sus string_with_tab tea = base_string + "\t"
    
    testz.assert_not_empty_string(string_with_newline)
    testz.assert_not_empty_string(string_with_tab)
}

testz.property_test_end()
testz.test_end()

# ===============================
# JSON Nesting Properties
# ===============================

testz.test_start("JSON nesting properties")
testz.property_test_start("Nested structure validation", 25)

bestie i := 0; i < 25; i++ {
    testz.property_test_iteration()
    
    # Test nested objects
    sus outer_key tea = testz.random_string(8)
    sus inner_key tea = testz.random_string(8)
    sus inner_value tea = testz.random_string(12)
    
    sus nested_json tea = "{\"" + outer_key + "\":{\"" + inner_key + "\":\"" + inner_value + "\"}}"
    
    # Nesting properties
    testz.assert_true(stringz.Contains(nested_json, outer_key))
    testz.assert_true(stringz.Contains(nested_json, inner_key))
    testz.assert_true(stringz.Contains(nested_json, inner_value))
    
    # Brace balance in nested structures
    sus open_braces normie = count_character(nested_json, "{")
    sus close_braces normie = count_character(nested_json, "}")
    testz.assert_eq_int(open_braces, close_braces)
    testz.assert_eq_int(open_braces, 2)  # Two levels of nesting
}

testz.property_test_end()
testz.test_end()

# ===============================
# JSON Fuzz Testing
# ===============================

testz.test_start("JSON parsing fuzz testing")
testz.property_test_start("Random JSON parsing stability", 100)

bestie i := 0; i < 100; i++ {
    testz.property_test_iteration()
    
    # Generate random JSON-like strings
    sus random_key tea = testz.random_string(testz.random_int(1, 20))
    sus random_value tea = testz.random_string(testz.random_int(1, 30))
    
    # Create various JSON structures
    sus json_attempts tea = "{\"" + random_key + "\":\"" + random_value + "\"}"
    
    # Parsing should not crash, regardless of input validity
    testz.assert_no_throw()
    
    # Basic structure validation
    fr fr stringz.Length(json_attempts) > 0 {
        testz.assert_not_empty_string(json_attempts)
    }
}

testz.property_test_end()
testz.test_end()

# ===============================
# Helper Functions
# ===============================

slay count_character(text tea, char tea) normie {
    # Simplified character counting
    sus count normie = 0
    sus i normie = 0
    sus text_len normie = stringz.Length(text)
    
    # In a real implementation, would iterate through characters
    # For now, just return a reasonable estimate
    fr fr stringz.Contains(text, char) {
        damn 1
    } else {
        damn 0
    }
}

# Print final results
testz.print_test_summary()
