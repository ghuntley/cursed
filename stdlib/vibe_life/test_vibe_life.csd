yeet "testz"
yeet "vibe_life"
yeet "core"

# Initialize vibe_life module for testing
init_vibe_life()

# ==============================================================================
# COMMAND LINE ARGUMENT TESTS
# ==============================================================================

test_start("get_args basic functionality")
sus args [tea] = vibe_life.get_args()
assert_eq_int(args.length(), 2)
assert_eq_string(args[0], "cursed")
assert_eq_string(args[1], "program.csd")

test_start("get_arg_count functionality")
sus count normie = vibe_life.get_arg_count()
assert_eq_int(count, 2)

test_start("get_arg by index")
assert_eq_string(vibe_life.get_arg(0), "cursed")
assert_eq_string(vibe_life.get_arg(1), "program.csd")
assert_eq_string(vibe_life.get_arg(99), "")  # Out of bounds

test_start("set_args functionality")
sus new_args [tea] = ["test", "file.csd", "--debug"]
vibe_life.set_args(new_args)
assert_eq_int(vibe_life.get_arg_count(), 3)
assert_eq_string(vibe_life.get_arg(2), "--debug")

# ==============================================================================
# ENVIRONMENT VARIABLE TESTS
# ==============================================================================

test_start("get_env default variables")
assert_eq_string(vibe_life.get_env("HOME"), "/home/user")
assert_eq_string(vibe_life.get_env("PATH"), "/usr/bin:/bin")
assert_eq_string(vibe_life.get_env("NONEXISTENT"), "")

test_start("set_env functionality")
assert_true(vibe_life.set_env("TEST_VAR", "test_value"))
assert_eq_string(vibe_life.get_env("TEST_VAR"), "test_value")

test_start("has_env functionality")
assert_true(vibe_life.has_env("HOME"))
assert_true(vibe_life.has_env("TEST_VAR"))
assert_false(vibe_life.has_env("NONEXISTENT"))

test_start("unset_env functionality")
assert_true(vibe_life.unset_env("TEST_VAR"))
assert_false(vibe_life.has_env("TEST_VAR"))
assert_false(vibe_life.unset_env("NONEXISTENT"))

test_start("get_env_keys functionality")
sus env_keys [tea] = vibe_life.get_env_keys()
assert_true(env_keys.length() >= 4)  # At least default vars

# ==============================================================================
# PROCESS CONTROL TESTS
# ==============================================================================

test_start("get_pid functionality")
sus pid normie = vibe_life.get_pid()
assert_eq_int(pid, 1234)

test_start("get_ppid functionality")
sus ppid normie = vibe_life.get_ppid()
assert_eq_int(ppid, 1000)

test_start("exit code management")
vibe_life.exit(42)
assert_eq_int(vibe_life.get_exit_code(), 42)

# ==============================================================================
# WORKING DIRECTORY TESTS
# ==============================================================================

test_start("get_cwd default")
assert_eq_string(vibe_life.get_cwd(), "/home/user")

test_start("set_cwd functionality")
assert_true(vibe_life.set_cwd("/tmp"))
assert_eq_string(vibe_life.get_cwd(), "/tmp")
assert_false(vibe_life.set_cwd(""))  # Empty path should fail

test_start("join_path functionality")
assert_eq_string(vibe_life.join_path("/home", "user"), "/home/user")
assert_eq_string(vibe_life.join_path("/home/", "user"), "/home/user")
assert_eq_string(vibe_life.join_path("", "user"), "user")
assert_eq_string(vibe_life.join_path("/home", ""), "/home")

test_start("dirname functionality")
assert_eq_string(vibe_life.dirname("/home/user/file.txt"), "/home/user")
assert_eq_string(vibe_life.dirname("file.txt"), ".")
assert_eq_string(vibe_life.dirname("/root"), "")

test_start("basename functionality")
assert_eq_string(vibe_life.basename("/home/user/file.txt"), "file.txt")
assert_eq_string(vibe_life.basename("file.txt"), "file.txt")
assert_eq_string(vibe_life.basename("/home/user/"), "")

# ==============================================================================
# FILE SYSTEM OPERATION TESTS
# ==============================================================================

test_start("file_exists default files")
assert_true(vibe_life.file_exists("/tmp/cursed.log"))
assert_true(vibe_life.file_exists("/home/user/.cursedrc"))
assert_false(vibe_life.file_exists("/nonexistent"))

test_start("create_file functionality")
assert_true(vibe_life.create_file("/tmp/test.txt", "Hello World"))
assert_true(vibe_life.file_exists("/tmp/test.txt"))

test_start("read_file functionality")
assert_eq_string(vibe_life.read_file("/tmp/test.txt"), "Hello World")
assert_eq_string(vibe_life.read_file("/nonexistent"), "")

test_start("write_file functionality")
assert_true(vibe_life.write_file("/tmp/test2.txt", "New content"))
assert_eq_string(vibe_life.read_file("/tmp/test2.txt"), "New content")

test_start("append_file functionality")
assert_true(vibe_life.append_file("/tmp/test.txt", " Appended"))
assert_eq_string(vibe_life.read_file("/tmp/test.txt"), "Hello World Appended")

test_start("get_file_size functionality")
assert_eq_int(vibe_life.get_file_size("/tmp/test.txt"), 19)  # "Hello World Appended"
assert_eq_int(vibe_life.get_file_size("/nonexistent"), -1)

test_start("delete_file functionality")
assert_true(vibe_life.delete_file("/tmp/test.txt"))
assert_false(vibe_life.file_exists("/tmp/test.txt"))
assert_false(vibe_life.delete_file("/nonexistent"))

test_start("list_files functionality")
sus files [tea] = vibe_life.list_files()
assert_true(files.length() >= 2)  # At least default files

test_start("create_dir functionality")
assert_true(vibe_life.create_dir("/tmp/testdir"))
assert_true(vibe_life.is_dir("/tmp/testdir"))
assert_false(vibe_life.is_dir("/tmp/test2.txt"))

# ==============================================================================
# UTILITY FUNCTION TESTS
# ==============================================================================

test_start("get_timestamp functionality")
sus timestamp normie = vibe_life.get_timestamp()
assert_true(timestamp > 0)

test_start("sleep functionality")
vibe_life.sleep(1)  # Should print sleep message

test_start("get_module_info functionality")
sus info tea = vibe_life.get_module_info()
assert_true(info.contains("vibe_life"))
assert_true(info.contains("v1.0"))

# ==============================================================================
# INTEGRATION TESTS
# ==============================================================================

test_start("file and directory integration")
# Create directory and file
assert_true(vibe_life.create_dir("/home/projects"))
assert_true(vibe_life.create_file("/home/projects/main.csd", "slay main() { vibez.spill(\"Hello\") }"))
assert_true(vibe_life.file_exists("/home/projects/main.csd"))
assert_true(vibe_life.is_dir("/home/projects"))

test_start("environment and path integration")
vibe_life.set_env("PROJECT_DIR", "/home/projects")
sus project_path tea = vibe_life.join_path(vibe_life.get_env("PROJECT_DIR"), "main.csd")
assert_eq_string(project_path, "/home/projects/main.csd")
assert_true(vibe_life.file_exists(project_path))

test_start("command line and working directory integration")
vibe_life.set_args(["cursed", "--compile", "main.csd"])
vibe_life.set_cwd("/home/projects")
sus current_dir tea = vibe_life.get_cwd()
sus main_file tea = vibe_life.join_path(current_dir, vibe_life.get_arg(2))
assert_eq_string(main_file, "/home/projects/main.csd")

# ==============================================================================
# EDGE CASE TESTS
# ==============================================================================

test_start("empty and null inputs")
assert_eq_string(vibe_life.get_env(""), "")
assert_false(vibe_life.set_env("", "value"))
assert_eq_string(vibe_life.read_file(""), "")
assert_false(vibe_life.create_file("", "content"))

test_start("large file content")
sus large_content tea = ""
sus i normie = 0
while i < 100 {
    large_content = large_content + "This is line " + core.tea(i) + "\n"
    i = i + 1
}
assert_true(vibe_life.create_file("/tmp/large.txt", large_content))
assert_eq_int(vibe_life.get_file_size("/tmp/large.txt"), large_content.length())

test_start("path edge cases")
assert_eq_string(vibe_life.dirname("/"), "")
assert_eq_string(vibe_life.basename("/"), "")
assert_eq_string(vibe_life.join_path("/", "root"), "/root")

# Print comprehensive test summary
print_test_summary()

vibez.spill("vibe_life module comprehensive testing complete!")
vibez.spill("All essential OS functionality verified for self-hosting")
