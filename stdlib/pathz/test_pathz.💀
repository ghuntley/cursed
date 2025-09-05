yeet "testz"
yeet "pathz"

test_start("Path joining")
sus result tea = path_join(["home", "user", "projects"])
assert_eq_string(result, "home/user/projects")

test_start("Path directory extraction")
sus dir tea = path_dir("/home/user/file.txt")
assert_eq_string(dir, "/home/user")

test_start("Path base filename")
sus base tea = path_base("/home/user/file.txt")
assert_eq_string(base, "file.txt")

test_start("Path extension")
sus ext tea = path_ext("/home/user/file.txt")
assert_eq_string(ext, ".txt")

test_start("Path absolute check")
sus is_abs lit = path_is_absolute("/home/user")
assert_true(is_abs)

sus is_rel lit = path_is_absolute("home/user")
assert_false(is_rel)

test_start("Path validation")
sus valid, err = path_validate("/home/user/valid_file.txt")
assert_true(valid)

sus invalid, err2 = path_validate("/path/with|invalid:char")
assert_false(invalid)

test_start("Path cleaning")
sus cleaned tea = path_clean("/home/user/../user/./file.txt")
assert_eq_string(cleaned, "/home/user/file.txt")

test_start("Path existence simulation")
sus exists lit = path_exists("/home/user")
assert_true(exists)

sus not_exists lit = path_exists("/nonexistent/path")
assert_false(not_exists)

test_start("Directory vs file detection")
sus is_dir lit = path_is_dir("/home/user")
assert_true(is_dir)

sus is_file lit = path_is_file("/home/user/file.txt")
assert_true(is_file)

test_start("Tilde expansion")
sus expanded tea = path_expand_tilde("~/projects")
assert_eq_string(expanded, "/home/user/projects")

test_start("Temporary paths")
sus temp_dir tea = path_temp_dir()
assert_eq_string(temp_dir, "/tmp")

sus temp_file tea = path_temp_file("test", ".tmp")
assert_true(string_starts_with(temp_file, "/tmp/test_"))

print_test_summary()
