yeet "testz"
yeet "vibe_life"
yeet "core"

# Initialize vibe_life module for testing
init_vibe_life()

# ==============================================================================
# COMMAND LINE ARGUMENT TESTS
# ==============================================================================

test_start("get_args basic functionality")
sus args [tea] = get_args()
assert_eq_int(args.length(), 2)
assert_eq_string(args[0], "cursed")
assert_eq_string(args[1], "program.csd")

test_start("get_arg_count functionality")
sus count normie = get_arg_count()
assert_eq_int(count, 2)

test_start("get_arg by index")
assert_eq_string(get_arg(0), "cursed")
assert_eq_string(get_arg(1), "program.csd")
assert_eq_string(get_arg(99), "")  # Out of bounds

test_start("set_args functionality")
sus new_args [tea] = ["test", "file.csd", "--debug"]
set_args(new_args)
assert_eq_int(get_arg_count(), 3)
assert_eq_string(get_arg(2), "--debug")

# ==============================================================================
# ENVIRONMENT VARIABLE TESTS
# ==============================================================================

test_start("get_env default variables")
assert_eq_string(get_env("HOME"), "/home/user")
assert_eq_string(get_env("PATH"), "/usr/bin:/bin")
assert_eq_string(get_env("NONEXISTENT"), "")

test_start("set_env functionality")
assert_true(set_env("TEST_VAR", "test_value"))
assert_eq_string(get_env("TEST_VAR"), "test_value")

test_start("has_env functionality")
assert_true(has_env("HOME"))
assert_true(has_env("TEST_VAR"))
assert_false(has_env("NONEXISTENT"))

test_start("unset_env functionality")
assert_true(unset_env("TEST_VAR"))
assert_false(has_env("TEST_VAR"))
assert_false(unset_env("NONEXISTENT"))

test_start("get_env_keys functionality")
sus env_keys [tea] = get_env_keys()
assert_true(env_keys.length() >= 4)  # At least default vars

# ==============================================================================
# PROCESS CONTROL TESTS
# ==============================================================================

test_start("get_pid functionality")
sus pid normie = get_pid()
assert_eq_int(pid, 1234)

test_start("get_ppid functionality")
sus ppid normie = get_ppid()
assert_eq_int(ppid, 1000)

test_start("exit code management")
exit(42)
assert_eq_int(get_exit_code(), 42)

# ==============================================================================
# WORKING DIRECTORY TESTS
# ==============================================================================

test_start("get_cwd default")
assert_eq_string(get_cwd(), "/home/user")

test_start("set_cwd functionality")
assert_true(set_cwd("/tmp"))
assert_eq_string(get_cwd(), "/tmp")
assert_false(set_cwd(""))  # Empty path should fail

test_start("join_path functionality")
assert_eq_string(join_path("/home", "user"), "/home/user")
assert_eq_string(join_path("/home/", "user"), "/home/user")
assert_eq_string(join_path("", "user"), "user")
assert_eq_string(join_path("/home", ""), "/home")

test_start("dirname functionality")
assert_eq_string(dirname("/home/user/file.txt"), "/home/user")
assert_eq_string(dirname("file.txt"), ".")
assert_eq_string(dirname("/root"), "")

test_start("basename functionality")
assert_eq_string(basename("/home/user/file.txt"), "file.txt")
assert_eq_string(basename("file.txt"), "file.txt")
assert_eq_string(basename("/home/user/"), "")

# ==============================================================================
# UTILITY FUNCTION TESTS
# ==============================================================================

test_start("get_module_info functionality")
sus info tea = get_module_info()
assert_true(info.contains("vibe_life"))
assert_true(info.contains("v1.0"))

# Print test summary
print_test_summary()

vibez.spill("vibe_life module testing complete!")
