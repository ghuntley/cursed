yeet "testz"
yeet "pathing"

// Comprehensive test suite for pathing module
// Tests all path manipulation functions with various scenarios

// Test path_join function
slay test_path_join() {
    test_start("path_join basic")
    sus parts [tea] = ["usr", "local", "bin"]
    sus result tea = path_join(parts)
    assert_eq_string(result, "usr/local/bin")
    
    test_start("path_join empty")
    sus empty_parts [tea] = []
    sus empty_result tea = path_join(empty_parts)
    assert_eq_string(empty_result, "")
    
    test_start("path_join single")
    sus single_parts [tea] = ["home"]
    sus single_result tea = path_join(single_parts)
    assert_eq_string(single_result, "home")
    
    test_start("path_join with root")
    sus root_parts [tea] = ["", "usr", "bin"]
    sus root_result tea = path_join(root_parts)
    assert_eq_string(root_result, "/usr/bin")
}

// Test path_split function
slay test_path_split() {
    test_start("path_split basic")
    sus parts [tea] = path_split("usr/local/bin")
    assert_eq_int(parts.length, 3)
    assert_eq_string(parts[0], "usr")
    assert_eq_string(parts[1], "local")
    assert_eq_string(parts[2], "bin")
    
    test_start("path_split absolute")
    sus abs_parts [tea] = path_split("/usr/local/bin")
    assert_eq_int(abs_parts.length, 4)
    assert_eq_string(abs_parts[0], "")
    assert_eq_string(abs_parts[1], "usr")
    assert_eq_string(abs_parts[2], "local")
    assert_eq_string(abs_parts[3], "bin")
    
    test_start("path_split single")
    sus single_parts [tea] = path_split("file.txt")
    assert_eq_int(single_parts.length, 1)
    assert_eq_string(single_parts[0], "file.txt")
}

// Test path_basename function
slay test_path_basename() {
    test_start("path_basename basic")
    sus basename tea = path_basename("usr/local/bin/file.txt")
    assert_eq_string(basename, "file.txt")
    
    test_start("path_basename no path")
    sus no_path tea = path_basename("file.txt")
    assert_eq_string(no_path, "file.txt")
    
    test_start("path_basename empty")
    sus empty tea = path_basename("")
    assert_eq_string(empty, "")
    
    test_start("path_basename root")
    sus root tea = path_basename("/")
    assert_eq_string(root, "")
    
    test_start("path_basename directory")
    sus dir tea = path_basename("/usr/local/")
    assert_eq_string(dir, "")
}

// Test path_dirname function
slay test_path_dirname() {
    test_start("path_dirname basic")
    sus dirname tea = path_dirname("usr/local/bin/file.txt")
    assert_eq_string(dirname, "usr/local/bin")
    
    test_start("path_dirname no path")
    sus no_path tea = path_dirname("file.txt")
    assert_eq_string(no_path, ".")
    
    test_start("path_dirname empty")
    sus empty tea = path_dirname("")
    assert_eq_string(empty, ".")
    
    test_start("path_dirname root")
    sus root tea = path_dirname("/file.txt")
    assert_eq_string(root, "/")
    
    test_start("path_dirname absolute")
    sus abs tea = path_dirname("/usr/local/bin")
    assert_eq_string(abs, "/usr/local")
}

// Test path_ext function
slay test_path_ext() {
    test_start("path_ext basic")
    sus ext tea = path_ext("file.txt")
    assert_eq_string(ext, ".txt")
    
    test_start("path_ext multiple dots")
    sus multi tea = path_ext("archive.tar.gz")
    assert_eq_string(multi, ".gz")
    
    test_start("path_ext no extension")
    sus no_ext tea = path_ext("README")
    assert_eq_string(no_ext, "")
    
    test_start("path_ext hidden file")
    sus hidden tea = path_ext(".bashrc")
    assert_eq_string(hidden, "")
    
    test_start("path_ext path with extension")
    sus path_ext_test tea = path_ext("usr/local/bin/file.txt")
    assert_eq_string(path_ext_test, ".txt")
}

// Test path_clean function
slay test_path_clean() {
    test_start("path_clean basic")
    sus clean tea = path_clean("usr/./local/../bin")
    assert_eq_string(clean, "usr/bin")
    
    test_start("path_clean empty")
    sus empty tea = path_clean("")
    assert_eq_string(empty, ".")
    
    test_start("path_clean current dir")
    sus current tea = path_clean("./file.txt")
    assert_eq_string(current, "file.txt")
    
    test_start("path_clean parent dir")
    sus parent tea = path_clean("../file.txt")
    assert_eq_string(parent, "../file.txt")
    
    test_start("path_clean complex")
    sus complex tea = path_clean("usr/local/./bin/../lib/./")
    assert_eq_string(complex, "usr/local/lib")
}

// Test path_is_abs function
slay test_path_is_abs() {
    test_start("path_is_abs unix absolute")
    sus abs tea = "/usr/local/bin"
    assert_true(path_is_abs(abs))
    
    test_start("path_is_abs unix relative")
    sus rel tea = "usr/local/bin"
    assert_false(path_is_abs(rel))
    
    test_start("path_is_abs current dir")
    sus current tea = "./file.txt"
    assert_false(path_is_abs(current))
    
    test_start("path_is_abs parent dir")
    sus parent tea = "../file.txt"
    assert_false(path_is_abs(parent))
    
    test_start("path_is_abs empty")
    sus empty tea = ""
    assert_false(path_is_abs(empty))
}

// Test path_abs function
slay test_path_abs() {
    test_start("path_abs already absolute")
    sus abs tea = "/usr/local/bin"
    sus abs_result tea = path_abs(abs)
    assert_eq_string(abs_result, "/usr/local/bin")
    
    test_start("path_abs relative")
    sus rel tea = "file.txt"
    sus rel_result tea = path_abs(rel)
    assert_true(path_is_abs(rel_result))
    
    test_start("path_abs current dir")
    sus current tea = "./file.txt"
    sus current_result tea = path_abs(current)
    assert_true(path_is_abs(current_result))
}

// Test path_rel function
slay test_path_rel() {
    test_start("path_rel basic")
    sus base tea = "/usr/local"
    sus target tea = "/usr/local/bin"
    sus rel tea = path_rel(base, target)
    assert_eq_string(rel, "bin")
    
    test_start("path_rel same path")
    sus same_base tea = "/usr/local/bin"
    sus same_target tea = "/usr/local/bin"
    sus same_rel tea = path_rel(same_base, same_target)
    assert_eq_string(same_rel, ".")
    
    test_start("path_rel parent")
    sus child tea = "/usr/local/bin"
    sus parent tea = "/usr/local"
    sus parent_rel tea = path_rel(child, parent)
    assert_eq_string(parent_rel, "..")
}

// Test path_match function
slay test_path_match() {
    test_start("path_match exact")
    assert_true(path_match("file.txt", "file.txt"))
    
    test_start("path_match no match")
    assert_false(path_match("file.txt", "other.txt"))
    
    test_start("path_match wildcard")
    assert_true(path_match("*.txt", "file.txt"))
    
    test_start("path_match wildcard no match")
    assert_false(path_match("*.txt", "file.md"))
    
    test_start("path_match complex wildcard")
    assert_true(path_match("test_*.txt", "test_file.txt"))
}

// Test path_from_slash function
slay test_path_from_slash() {
    test_start("path_from_slash unix")
    sus unix_path tea = "usr/local/bin"
    sus from_slash tea = path_from_slash(unix_path)
    assert_eq_string(from_slash, "usr/local/bin")
}

// Test path_to_slash function
slay test_path_to_slash() {
    test_start("path_to_slash unix")
    sus unix_path tea = "usr/local/bin"
    sus to_slash tea = path_to_slash(unix_path)
    assert_eq_string(to_slash, "usr/local/bin")
}

// Test string helper functions
slay test_string_helpers() {
    test_start("string_split basic")
    sus parts [tea] = string_split("a,b,c", ",")
    assert_eq_int(parts.length, 3)
    assert_eq_string(parts[0], "a")
    assert_eq_string(parts[1], "b")
    assert_eq_string(parts[2], "c")
    
    test_start("string_last_index basic")
    sus pos normie = string_last_index("hello.world.txt", ".")
    assert_eq_int(pos, 11)
    
    test_start("string_starts_with true")
    assert_true(string_starts_with("hello world", "hello"))
    
    test_start("string_starts_with false")
    assert_false(string_starts_with("hello world", "world"))
    
    test_start("string_ends_with true")
    assert_true(string_ends_with("hello world", "world"))
    
    test_start("string_ends_with false")
    assert_false(string_ends_with("hello world", "hello"))
    
    test_start("string_contains true")
    assert_true(string_contains("hello world", "lo wo"))
    
    test_start("string_contains false")
    assert_false(string_contains("hello world", "xyz"))
    
    test_start("string_replace_all basic")
    sus replaced tea = string_replace_all("hello world world", "world", "universe")
    assert_eq_string(replaced, "hello universe universe")
}

// Run all tests
slay main_character() {
    vibez.spill("Running pathing module tests...")
    
    test_path_join()
    test_path_split()
    test_path_basename()
    test_path_dirname()
    test_path_ext()
    test_path_clean()
    test_path_is_abs()
    test_path_abs()
    test_path_rel()
    test_path_match()
    test_path_from_slash()
    test_path_to_slash()
    test_string_helpers()
    
    print_test_summary()
    
    vibez.spill("Pathing module tests completed!")
}
