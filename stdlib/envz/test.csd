fr fr Environment Variable Module Test Suite
fr fr Testing real environment variable functionality

yeet "envz"
yeet "vibez"
yeet "testz"

fr fr Test suite for environment variable operations
test_start("envz")

fr fr ===== TEST 1: Basic Environment Variable Operations =====

vibez.spill("=== TEST 1: Basic Environment Variable Access ===")

# Test getting existing environment variables
sus user_var tea = get_env("USER")
sus home_var tea = get_env("HOME")
sus path_var tea = get_env("PATH")

vibez.spill("Current USER: " + user_var)
vibez.spill("Current HOME: " + home_var)  
vibez.spill("Current PATH (first 50 chars): " + path_var.substring(0, 50))

assert_not_empty(user_var, "USER environment variable should exist")
assert_not_empty(home_var, "HOME environment variable should exist")
assert_not_empty(path_var, "PATH environment variable should exist")

fr fr ===== TEST 2: Environment Variable Setting and Getting =====

vibez.spill("=== TEST 2: Set and Get Environment Variables ===")

# Test setting a new environment variable
sus test_key tea = "CURSED_TEST_VAR"
sus test_value tea = "cursed_test_value_123"

sus set_result lit = set_env(test_key, test_value)
assert_eq_bool(set_result, based, "Setting environment variable should succeed")

# Test getting the set variable
sus retrieved_value tea = get_env(test_key)
vibez.spill("Set '" + test_key + "' to '" + test_value + "'")
vibez.spill("Retrieved value: '" + retrieved_value + "'")
assert_eq_string(retrieved_value, test_value, "Retrieved value should match set value")

# Test environment variable existence
sus exists_result lit = env_exists(test_key)
assert_eq_bool(exists_result, based, "Environment variable should exist after setting")

fr fr ===== TEST 3: Environment Variable Default Values =====

vibez.spill("=== TEST 3: Default Value Handling ===")

sus nonexistent_key tea = "CURSED_NONEXISTENT_VAR_12345"
sus default_value tea = "default_fallback_value"

# Test getting non-existent variable with default
sus default_result tea = get_env_default(nonexistent_key, default_value)
vibez.spill("Non-existent var with default: '" + default_result + "'")
assert_eq_string(default_result, default_value, "Should return default value for non-existent variable")

# Test getting existing variable (should ignore default)
sus existing_result tea = get_env_default(test_key, "ignored_default")
vibez.spill("Existing var with default: '" + existing_result + "'")
assert_eq_string(existing_result, test_value, "Should return actual value, not default, for existing variable")

fr fr ===== TEST 4: Environment Variable Removal =====

vibez.spill("=== TEST 4: Environment Variable Removal ===")

# Test unsetting an environment variable
sus unset_result lit = unset_env(test_key)
assert_eq_bool(unset_result, based, "Unsetting environment variable should succeed")

# Verify variable is removed
sus after_unset tea = get_env(test_key)
vibez.spill("Value after unset: '" + after_unset + "'")
assert_empty(after_unset, "Variable should be empty after unset")

sus exists_after_unset lit = env_exists(test_key)
assert_eq_bool(exists_after_unset, cap, "Variable should not exist after unset")

fr fr ===== TEST 5: Environment Variable Listing =====

vibez.spill("=== TEST 5: Environment Variable Listing ===")

# Test listing all environment variables
sus all_env map<tea, tea> = list_env()
sus env_count drip = all_env.size()

vibez.spill("Total environment variables: " + env_count.to_string())
assert_greater_than(env_count, 5, "Should have at least 5 environment variables")

# Check that common variables exist in the list
assert_eq_bool(all_env.has_key("HOME"), based, "Environment list should contain HOME")
assert_eq_bool(all_env.has_key("USER"), based, "Environment list should contain USER")
assert_eq_bool(all_env.has_key("PATH"), based, "Environment list should contain PATH")

fr fr ===== TEST 6: Platform Detection =====

vibez.spill("=== TEST 6: Platform Detection ===")

sus platform tea = get_platform()
vibez.spill("Detected platform: " + platform)
assert_not_empty(platform, "Platform detection should return non-empty string")

sus path_separator tea = get_path_separator()
vibez.spill("Path separator: '" + path_separator + "'")
assert_not_empty(path_separator, "Path separator should not be empty")

fr fr ===== TEST 7: PATH Environment Variable Operations =====

vibez.spill("=== TEST 7: PATH Operations ===")

sus original_path [tea] = get_path()
sus original_path_count drip = original_path.length()
vibez.spill("Original PATH entries: " + original_path_count.to_string())
assert_greater_than(original_path_count, 0, "PATH should contain at least one directory")

# Test adding to PATH
sus test_path tea = "/cursed/test/bin"
sus add_result lit = add_to_path(test_path)
assert_eq_bool(add_result, based, "Adding to PATH should succeed")

sus modified_path [tea] = get_path()
sus modified_path_count drip = modified_path.length()
assert_eq_int(modified_path_count, original_path_count + 1, "PATH should have one more entry")

# Test removing from PATH  
sus remove_result lit = remove_from_path(test_path)
assert_eq_bool(remove_result, based, "Removing from PATH should succeed")

sus restored_path [tea] = get_path()
sus restored_path_count drip = restored_path.length()
assert_eq_int(restored_path_count, original_path_count, "PATH should be restored to original count")

fr fr ===== TEST 8: Common Environment Variable Helpers =====

vibez.spill("=== TEST 8: Common Environment Variable Helpers ===")

sus home_dir tea = get_home()
sus current_user tea = get_user()
sus shell_path tea = get_shell()
sus temp_dir tea = get_temp_dir()

vibez.spill("Home directory: " + home_dir)
vibez.spill("Current user: " + current_user)
vibez.spill("Shell: " + shell_path)
vibez.spill("Temp directory: " + temp_dir)

assert_not_empty(home_dir, "Home directory should not be empty")
assert_not_empty(current_user, "Current user should not be empty")
assert_not_empty(shell_path, "Shell path should not be empty")
assert_not_empty(temp_dir, "Temp directory should not be empty")

fr fr ===== TEST 9: Variable Expansion =====

vibez.spill("=== TEST 9: Variable Expansion ===")

# Set a test variable for expansion
set_env("CURSED_EXPAND_TEST", "expanded_value")

# Test ${VAR} format expansion
sus template1 tea = "Path: ${HOME}/documents"
sus expanded1 tea = expand(template1)
vibez.spill("Template: " + template1)
vibez.spill("Expanded: " + expanded1)
assert_contains(expanded1, home_dir, "Expansion should contain actual HOME value")

# Test $VAR format expansion
sus template2 tea = "User $USER works in $HOME"
sus expanded2 tea = expand(template2)
vibez.spill("Template: " + template2)
vibez.spill("Expanded: " + expanded2)
assert_contains(expanded2, current_user, "Expansion should contain actual USER value")

# Clean up test variable
unset_env("CURSED_EXPAND_TEST")

fr fr ===== TEST 10: Error Handling =====

vibez.spill("=== TEST 10: Error Handling ===")

# Test empty key handling
sus empty_key_result tea = get_env("")
assert_empty(empty_key_result, "Getting empty key should return empty string")

sus empty_key_exists lit = env_exists("")
assert_eq_bool(empty_key_exists, cap, "Empty key should not exist")

sus empty_key_set lit = set_env("", "value")
assert_eq_bool(empty_key_set, cap, "Setting empty key should fail")

# Test very long key (should be rejected)
sus long_key tea = "A".repeat(300)  # Exceeds MAX_VAR_NAME_LENGTH
sus long_key_set lit = set_env(long_key, "value")
assert_eq_bool(long_key_set, cap, "Setting overly long key should fail")

vibez.spill("=== Environment Variable Tests Complete ===")
print_test_summary()

fr fr ===== HELPER FUNCTIONS =====

slay assert_not_empty(value tea, message tea) {
    check value == "" {
        vibez.spill("FAIL: " + message + " (got empty string)")
        increment_test_failures()
        damn
    }
    vibez.spill("PASS: " + message)
    increment_test_passes()
}

slay assert_empty(value tea, message tea) {
    check value != "" {
        vibez.spill("FAIL: " + message + " (got: '" + value + "')")
        increment_test_failures()
        damn
    }
    vibez.spill("PASS: " + message)
    increment_test_passes()
}

slay assert_contains(haystack tea, needle tea, message tea) {
    check !stringz.contains(haystack, needle) {
        vibez.spill("FAIL: " + message + " ('" + haystack + "' does not contain '" + needle + "')")
        increment_test_failures()
        damn
    }
    vibez.spill("PASS: " + message)
    increment_test_passes()
}

slay assert_greater_than(actual drip, expected drip, message tea) {
    check actual <= expected {
        vibez.spill("FAIL: " + message + " (got " + actual.to_string() + ", expected > " + expected.to_string() + ")")
        increment_test_failures()
        damn
    }
    vibez.spill("PASS: " + message)
    increment_test_passes()
}

vibez.spill("envz test suite completed!")
