// Environment variable validation with error handling

// Test setting and getting environment variable
set_err := runtime_set_env("CURSED_TEST", "hello")
lowkey set_err != "" {
    // Should cause interpreter error if set failed
    sus invalid_operation normie = 1 / 0
}

(value, get_err) := runtime_get_env("CURSED_TEST")
lowkey get_err != "" {
    // Should cause interpreter error if get failed
    sus invalid_operation normie = 1 / 0
}

lowkey value != "hello" {
    // Should cause interpreter error if value is wrong
    sus invalid_operation normie = 1 / 0
}

// Test unset
unset_err := runtime_unset_env("CURSED_TEST")
lowkey unset_err != "" {
    // Should cause interpreter error if unset failed
    sus invalid_operation normie = 1 / 0
}

// If we get here, all tests passed
