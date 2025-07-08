yeet "testz"
yeet "pathing"

test_start("Pathing Module Tests")

// Test path_join function
sus parts [tea] = ["usr", "local", "bin"]
sus result tea = path_join(parts)
assert_eq_string(result, "usr/local/bin")

// Test path_split function
sus split_parts [tea] = path_split("usr/local/bin")
assert_eq_int(split_parts.length, 3)
assert_eq_string(split_parts[0], "usr")
assert_eq_string(split_parts[1], "local")
assert_eq_string(split_parts[2], "bin")

// Test path_basename function
sus basename tea = path_basename("usr/local/bin/file.txt")
assert_eq_string(basename, "file.txt")

// Test path_dirname function
sus dirname tea = path_dirname("usr/local/bin/file.txt")
assert_eq_string(dirname, "usr/local/bin")

// Test path_ext function
sus ext tea = path_ext("file.txt")
assert_eq_string(ext, ".txt")

// Test path_is_abs function
assert_true(path_is_abs("/usr/local/bin"))
assert_false(path_is_abs("usr/local/bin"))

// Test path_clean function
sus clean tea = path_clean("usr/./local/../bin")
assert_eq_string(clean, "usr/bin")

// Test path_match function
assert_true(path_match("*.txt", "file.txt"))
assert_false(path_match("*.txt", "file.md"))

// Test string helper functions
sus str_parts [tea] = string_split("a,b,c", ",")
assert_eq_int(str_parts.length, 3)
assert_eq_string(str_parts[0], "a")

assert_true(string_starts_with("hello world", "hello"))
assert_false(string_starts_with("hello world", "world"))

assert_true(string_ends_with("hello world", "world"))
assert_false(string_ends_with("hello world", "hello"))

assert_true(string_contains("hello world", "lo wo"))
assert_false(string_contains("hello world", "xyz"))

sus replaced tea = string_replace_all("hello world world", "world", "universe")
assert_eq_string(replaced, "hello universe universe")

print_test_summary()
