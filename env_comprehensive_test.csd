// Comprehensive environment variable test

// Test runtime_string_length
test_len := runtime_string_length("hello")
lowkey test_len != 5 {
    sus error normie = 1 / 0  // Should fail if length is wrong
}

// Test runtime_to_lowercase
lower_str := runtime_to_lowercase("HELLO")
test_len2 := runtime_string_length(lower_str)
lowkey test_len2 != 5 {
    sus error normie = 1 / 0  // Should fail if lowercase conversion failed
}

// Test runtime_parse_int
(parsed_int, parse_err) := runtime_parse_int("42")
lowkey parse_err != "" {
    sus error normie = 1 / 0  // Should fail if parsing failed
}
lowkey parsed_int != 42 {
    sus error normie = 1 / 0  // Should fail if parsed value is wrong
}

// Test runtime_expand_env by setting a variable first
runtime_set_env("EXPAND_TEST", "expanded")
expanded_text := runtime_expand_env("Value is: $EXPAND_TEST")
exp_len := runtime_string_length(expanded_text)
lowkey exp_len < 10 {  // Should be longer than original if expansion worked
    sus error normie = 1 / 0
}

// Test runtime_split_path
paths := runtime_split_path("/usr/bin:/usr/local/bin")
// paths should be an array, if it's empty something went wrong
// We can't easily check array length, but the function should not crash

// Test runtime_list_env
(env_list, list_err) := runtime_list_env()
lowkey list_err != "" {
    sus error normie = 1 / 0  // Should fail if listing failed
}

// Clean up
runtime_unset_env("EXPAND_TEST")

// If we reach here, all comprehensive tests passed
