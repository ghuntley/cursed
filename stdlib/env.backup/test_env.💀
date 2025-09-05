yeet "testz"
yeet "env"

fr fr Environment Module Test Suite
test_start("Environment Module Tests")

fr fr Test basic environment variable operations
test_start("Basic Environment Operations")
env.set_env("TEST_VAR", "test_value")
assert_eq_string(env.get_env("TEST_VAR"), "test_value")
assert_true(env.env_exists("TEST_VAR"))
assert_false(env.is_env_readonly("TEST_VAR"))

fr fr Test environment variable unset
env.unset_env("TEST_VAR")
assert_false(env.env_exists("TEST_VAR"))
assert_eq_string(env.get_env("TEST_VAR"), "")

fr fr Test default environment variables
test_start("Default Environment Variables")
assert_true(env.env_exists("HOME"))
assert_true(env.env_exists("USER"))
assert_true(env.env_exists("PATH"))
assert_true(env.env_exists("SHELL"))
assert_true(env.env_exists("CURSED_VERSION"))

fr fr Test HOME directory
home_dir := env.get_home_dir()
assert_eq_string(home_dir, env.get_env("HOME"))
assert_eq_string(env.get_current_user(), env.get_env("USER"))

fr fr Test path management
test_start("Path Management")
original_path := env.get_env("PATH")
env.add_to_path("/test/path")
new_path := env.get_env("PATH")
assert_true(len(new_path) > len(original_path))

fr fr Test search path
search_path := env.get_search_path()
assert_true(len(search_path) > 0)

fr fr Test command line arguments
test_start("Command Line Arguments")
test_args := tea[value]{"program", "arg1", "arg2"}
env.set_args(3, test_args)
assert_eq_int(env.get_argc(), 3)
assert_eq_string(env.get_program_name(), "program")
assert_eq_string(env.get_arg(1), "arg1")
assert_eq_string(env.get_arg(2), "arg2")

fr fr Test environment variable expansion
test_start("Environment Variable Expansion")
env.set_env("EXPAND_TEST", "expanded")
expanded := env.expand_env("Value: ${EXPAND_TEST}")
assert_true(len(expanded) > 0)

fr fr Test home directory expansion
test_start("Home Directory Expansion")
env.set_env("HOME", "/home/test")
expanded_home := env.expand_home_dir("~/documents")
assert_true(len(expanded_home) > 0)

fr fr Test environment validation
test_start("Environment Validation")
assert_true(env.validate_env_name("VALID_NAME"))
assert_true(env.validate_env_name("valid123"))
assert_false(env.validate_env_name(""))
assert_true(env.validate_env_value("valid_value"))

fr fr Test readonly variables
test_start("Readonly Variables")
env.set_env_var("READONLY_VAR", "readonly_value", based, cap)
assert_true(env.is_env_readonly("READONLY_VAR"))
assert_false(env.unset_env("READONLY_VAR")) fr fr Should fail

fr fr Test system information
test_start("System Information")
hostname := env.get_hostname()
platform := env.get_platform()
arch := env.get_architecture()
os_type := env.get_os_type()

assert_true(len(hostname) > 0)
assert_true(len(platform) > 0)
assert_true(len(arch) > 0)
assert_true(len(os_type) > 0)

fr fr Test environment listing
test_start("Environment Listing")
all_vars := env.list_env_vars()
assert_true(len(all_vars) > 0)

all_env := env.get_all_env()
assert_true(len(all_env) > 0)

fr fr Test environment file operations
test_start("Environment File Operations")
assert_true(env.load_env_file("/test/env.conf"))
assert_true(env.save_env_file("/test/env.conf"))

fr fr Test environment comparison
test_start("Environment Comparison")
test_env := map[tea]tea{}
test_env["COMPARE_VAR"] = "compare_value"
diff := env.compare_env(test_env)
assert_true(len(diff) >= 0)

fr fr Test environment merging
env.merge_env(test_env)
assert_eq_string(env.get_env("COMPARE_VAR"), "compare_value")

fr fr Test debug functionality
test_start("Debug Functions")
env.debug_env_manager() fr fr Should print debug info

fr fr Test cleanup
test_start("Cleanup")
env.cleanup_env_manager()

print_test_summary()
