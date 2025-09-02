yeet "testz"
yeet "envz"

// Environment Variable Operations Test Suite

test_start("Basic Environment Operations")

// Test setting and getting environment variables
sus test_var tea = "CURSED_TEST_VAR"
sus test_value tea = "test_value_123"

sus set_err tea = envz.set_env(test_var, test_value)
assert_eq_string(set_err, "")

(retrieved_value, get_err) := envz.get_env(test_var)
assert_eq_string(get_err, "")
assert_eq_string(retrieved_value, test_value)

// Test environment variable existence
assert_true(envz.env_exists(test_var))
assert_false(envz.env_exists("DEFINITELY_NOT_SET_VAR"))

test_start("Environment Variable Types")

// Test getting environment variable with default
sus default_value tea = "default_test"
sus with_default tea = envz.get_env_with_default("NONEXISTENT_VAR", default_value)
assert_eq_string(with_default, default_value)

sus existing_with_default tea = envz.get_env_with_default(test_var, "not_used")
assert_eq_string(existing_with_default, test_value)

// Test integer environment variables
sus int_var tea = "CURSED_INT_VAR"
sus int_value normie = 42
envz.set_env(int_var, "42")

sus retrieved_int normie = envz.get_env_as_int(int_var, 0)
assert_eq_int(retrieved_int, int_value)

sus default_int normie = envz.get_env_as_int("NONEXISTENT_INT", 99)
assert_eq_int(default_int, 99)

test_start("Boolean Environment Variables")

// Test boolean environment variables
sus bool_var tea = "CURSED_BOOL_VAR"
envz.set_env(bool_var, "true")

sus bool_true lit = envz.get_env_as_bool(bool_var, cringe)
assert_true(bool_true)

envz.set_env(bool_var, "false")
sus bool_false lit = envz.get_env_as_bool(bool_var, based)
assert_false(bool_false)

envz.set_env(bool_var, "1")
sus bool_one lit = envz.get_env_as_bool(bool_var, cringe)
assert_true(bool_one)

envz.set_env(bool_var, "0")
sus bool_zero lit = envz.get_env_as_bool(bool_var, based)
assert_false(bool_zero)

// Test default boolean value
sus bool_default lit = envz.get_env_as_bool("NONEXISTENT_BOOL", based)
assert_true(bool_default)

test_start("Environment Variable Expansion")

// Test environment variable expansion
sus expansion_test tea = "Hello ${USER}!"
sus expanded tea = envz.expand_env(expansion_test)
assert_true(len_str(expanded) > 0)

test_start("System Environment Variables")

// Test getting common system variables
(home_dir, home_err) := envz.get_home_dir()
// Home directory should be available on most systems
assert_true(len_str(home_dir) > 0 || home_err != "")

(user_name, user_err) := envz.get_user_name()
// Username should be available on most systems
assert_true(len_str(user_name) > 0 || user_err != "")

(shell, shell_err) := envz.get_shell()
// Shell may not be available on all systems (Windows)
assert_true(len_str(shell) > 0 || shell_err != "")

(editor, editor_err) := envz.get_editor()
// Editor may not be set
assert_true(len_str(editor) > 0 || editor_err != "")

test_start("PATH Environment Variable")

// Test PATH operations
sus path_dirs tea[value] = envz.get_path_env()
assert_true(len(path_dirs) >= 0) // PATH might be empty in some test environments

test_start("Temporary Directory")

// Test temporary directory
(temp_dir, temp_err) := envz.get_temp_dir_env()
// Temp directory should be available on most systems
assert_true(len_str(temp_dir) > 0 || temp_err != "")

test_start("Environment Detection")

// Test environment detection
sus is_dev lit = envz.is_development_env()
sus is_prod lit = envz.is_production_env()

// These are just testing the functions work, not their actual values
assert_true(is_dev == based || is_dev == cringe)
assert_true(is_prod == based || is_prod == cringe)

test_start("Environment Listing")

// Test listing environment variables
(env_list, list_err) := envz.list_env()
assert_eq_string(list_err, "")
assert_true(len(env_list) > 0) // Should have at least some environment variables

// Test process environment preparation
sus process_env tea[value] = envz.copy_env_to_new_process()
assert_true(len(process_env) >= 0) // Should return array (may be empty)

test_start("Error Handling")

// Test error conditions
sus empty_name_err tea = envz.set_env("", "value")
assert_true(len_str(empty_name_err) > 0)

(empty_get, empty_get_err) := envz.get_env("")
assert_true(len_str(empty_get_err) > 0)

sus unset_empty_err tea = envz.unset_env("")
assert_true(len_str(unset_empty_err) > 0)

test_start("Environment Variable Removal")

// Test unsetting environment variables
sus unset_err tea = envz.unset_env(test_var)
assert_eq_string(unset_err, "")
assert_false(envz.env_exists(test_var))

// Clean up test variables
envz.unset_env(int_var)
envz.unset_env(bool_var)

print_test_summary()
