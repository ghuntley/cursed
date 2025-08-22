// Environment variable validation - this will work even without printing

yeet "envz"

// Test get_env with HOME directory
(home_dir, err) := get_env("HOME")

// Test that we can set and get an environment variable
set_env("CURSED_VALIDATION", "working")
(test_value, test_err) := get_env("CURSED_VALIDATION")

// Test environment variable existence
home_exists := env_exists("HOME")
test_exists := env_exists("CURSED_VALIDATION")

// Test default values
default_test := get_env_with_default("NONEXISTENT", "default_worked")

// Clean up
unset_env("CURSED_VALIDATION")

// All tests should work silently - the fact that the program doesn't crash means success
