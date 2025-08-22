// Test stdlib envz functions

yeet "envz"

// Test get_env
(home_val, home_err) := get_env("HOME")
lowkey home_err != "" {
    sus error normie = 1 / 0  // Should fail if HOME not found
}

// Test set_env
set_error := set_env("STDLIB_TEST", "working")
lowkey set_error != "" {
    sus error normie = 1 / 0  // Should fail if set failed
}

// Test env_exists
lowkey !env_exists("STDLIB_TEST") {
    sus error normie = 1 / 0  // Should fail if variable doesn't exist after setting
}

// Test get_env_with_default
default_val := get_env_with_default("NONEXISTENT_VAR", "default")
test_len := len_str(default_val)
lowkey test_len != 7 {  // "default" has 7 characters
    sus error normie = 1 / 0
}

// Test get_home_dir
(home_dir, home_dir_err) := get_home_dir()
lowkey home_dir_err != "" {
    sus error normie = 1 / 0  // Should be able to get home directory
}

// Test unset_env
unset_error := unset_env("STDLIB_TEST")
lowkey unset_error != "" {
    sus error normie = 1 / 0  // Should succeed in unsetting
}

// Verify it's gone
lowkey env_exists("STDLIB_TEST") {
    sus error normie = 1 / 0  // Should fail if variable still exists
}

// If we reach here, all stdlib tests passed
