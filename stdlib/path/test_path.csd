yeet "testz"
yeet "path"

# Path Module Test Suite
test_start("Path Module Tests")

# Test path joining
test_start("Path Joining")
joined := path.join([]tea{"home", "user", "documents"})
assert_eq_string(joined, "home/user/documents")

empty_join := path.join([]tea{})
assert_eq_string(empty_join, "")

single_join := path.join([]tea{"single"})
assert_eq_string(single_join, "single")

# Test path splitting
test_start("Path Splitting")
components := path.split("home/user/documents")
assert_eq_int(len(components), 3)
assert_eq_string(components[0], "home")
assert_eq_string(components[1], "user")
assert_eq_string(components[2], "documents")

# Test basename
test_start("Path Basename")
base := path.basename("/home/user/file.txt")
assert_eq_string(base, "file.txt")

base_dir := path.basename("/home/user/")
assert_eq_string(base_dir, "user")

base_empty := path.basename("")
assert_eq_string(base_empty, "")

# Test dirname
test_start("Path Dirname")
dir := path.dirname("/home/user/file.txt")
assert_eq_string(dir, "/home/user")

dir_root := path.dirname("/file.txt")
assert_eq_string(dir_root, "/")

dir_relative := path.dirname("file.txt")
assert_eq_string(dir_relative, ".")

# Test file extension
test_start("File Extension")
ext := path.ext("file.txt")
assert_eq_string(ext, ".txt")

ext_none := path.ext("file")
assert_eq_string(ext_none, "")

ext_hidden := path.ext(".hidden")
assert_eq_string(ext_hidden, "")

# Test file stem
test_start("File Stem")
stem := path.stem("file.txt")
assert_eq_string(stem, "file")

stem_no_ext := path.stem("file")
assert_eq_string(stem_no_ext, "file")

# Test absolute path checking
test_start("Absolute Path Checking")
assert_true(path.is_absolute("/home/user"))
assert_false(path.is_absolute("home/user"))
assert_false(path.is_absolute("./file.txt"))
assert_false(path.is_absolute("../file.txt"))

# Test relative path checking
test_start("Relative Path Checking")
assert_true(path.is_relative("home/user"))
assert_true(path.is_relative("./file.txt"))
assert_true(path.is_relative("../file.txt"))
assert_false(path.is_relative("/home/user"))

# Test path cleaning
test_start("Path Cleaning")
clean1 := path.clean("home/user/./documents")
assert_eq_string(clean1, "home/user/documents")

clean2 := path.clean("home/user/../user/documents")
assert_eq_string(clean2, "home/user/documents")

clean3 := path.clean("")
assert_eq_string(clean3, ".")

# Test absolute path conversion
test_start("Absolute Path Conversion")
abs_path := path.abs("documents/file.txt")
assert_true(path.is_absolute(abs_path))
assert_true(len(abs_path) > len("documents/file.txt"))

# Test home directory expansion
test_start("Home Directory Expansion")
path.set_home_dir("/home/test")
expanded := path.expand_home("~/documents")
assert_eq_string(expanded, "/home/test/documents")

home_only := path.expand_home("~")
assert_eq_string(home_only, "/home/test")

no_expand := path.expand_home("/absolute/path")
assert_eq_string(no_expand, "/absolute/path")

# Test environment variable expansion
test_start("Environment Variable Expansion")
expanded_env := path.expand_env("$HOME/documents")
assert_true(len(expanded_env) > 0)

# Test path validation
test_start("Path Validation")
assert_true(path.validate("/home/user/file.txt"))
assert_true(path.validate("relative/path"))
assert_false(path.validate(""))

# Test filename validation
test_start("Filename Validation")
assert_true(path.is_valid_filename("file.txt"))
assert_true(path.is_valid_filename("document"))
assert_false(path.is_valid_filename(""))
assert_false(path.is_valid_filename("file/name.txt"))

# Test path matching
test_start("Path Matching")
assert_true(path.match("*.txt", "file.txt"))
assert_true(path.match("file.*", "file.txt"))
assert_false(path.match("*.doc", "file.txt"))
assert_true(path.match("exact", "exact"))

# Test extension checking
test_start("Extension Checking")
assert_true(path.has_extension("file.txt", []tea{".txt", ".doc"}))
assert_false(path.has_extension("file.jpg", []tea{".txt", ".doc"}))

# Test directory management
test_start("Directory Management")
current := path.get_current_dir()
assert_true(len(current) > 0)

home := path.get_home_dir()
assert_true(len(home) > 0)

temp := path.get_temp_dir()
assert_true(len(temp) > 0)

# Test path conversion
test_start("Path Conversion")
slash_path := path.to_slash("home\\user\\file.txt")
assert_eq_string(slash_path, "home/user/file.txt")

native_path := path.from_slash("home/user/file.txt")
assert_eq_string(native_path, "home/user/file.txt")

# Test path list operations
test_start("Path List Operations")
path_list := "/usr/bin:/bin:/usr/local/bin"
paths := path.split_list(path_list)
assert_eq_int(len(paths), 3)
assert_eq_string(paths[0], "/usr/bin")
assert_eq_string(paths[1], "/bin")
assert_eq_string(paths[2], "/usr/local/bin")

rejoined := path.join_list(paths)
assert_eq_string(rejoined, path_list)

# Test path info
test_start("Path Information")
info := path.info("/home/user/documents/file.txt")
assert_eq_string(info.filename, "file.txt")
assert_eq_string(info.basename, "file")
assert_eq_string(info.extension, ".txt")
assert_eq_string(info.directory, "/home/user/documents")
assert_true(info.is_absolute)

# Test relative path calculation
test_start("Relative Path Calculation")
rel_path := path.rel("/home/user", "/home/user/documents/file.txt")
assert_eq_string(rel_path, "documents/file.txt")

rel_parent := path.rel("/home/user/documents", "/home/user")
assert_eq_string(rel_parent, "..")

# Test special path checking
test_start("Special Path Checking")
assert_true(path.is_root("/"))
assert_false(path.is_root("/home"))

assert_true(path.is_current_dir("."))
assert_false(path.is_current_dir("./"))

assert_true(path.is_parent_dir(".."))
assert_false(path.is_parent_dir("../"))

# Test debug functionality
test_start("Debug Functions")
path.debug_path_manager()  # Should print debug info

# Test cleanup
test_start("Cleanup")
path.cleanup_path_manager()

print_test_summary()
