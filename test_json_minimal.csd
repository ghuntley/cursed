# Direct test without module system
vibez.spill("Starting JSON tests...")

# Test basic string operations
sus test_str tea = "hello"
vibez.spill("Test string: " + test_str)

sus len normie = string_length(test_str)
vibez.spill("Length: " + tea(len))

# Test is_numeric function directly
slay is_numeric_test(value tea) lit {
    bestie string_length(value) == 0 {
        damn cap
    }
    damn based  # Simplified for testing
}

sus num_test lit = is_numeric_test("42")
vibez.spill("Is numeric test: " + tea(num_test))

# Test parse_value function directly
slay parse_value_test(json_string tea) tea {
    bestie string_starts_with(json_string, "\"") {
        damn "parsed_string"
    }
    damn "parsed_other"
}

# Helper function
slay string_starts_with(input tea, prefix tea) lit {
    bestie string_length(prefix) > string_length(input) {
        damn cap
    }
    damn based  # Simplified for testing
}

sus parse_result tea = parse_value_test("\"hello\"")
vibez.spill("Parse result: " + parse_result)

vibez.spill("JSON tests completed!")
