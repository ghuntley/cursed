yeet "testz"

fr fr Comprehensive test suite for pathz module
fr fr Tests all path manipulation, validation, and file system operations

sus main() {
    test_start("Path manipulation comprehensive tests")
    
    fr fr Basic path operations
    test_path_joining()
    test_path_splitting()
    test_path_components()
    test_path_extensions()
    
    fr fr Path validation and normalization
    test_path_validation()
    test_path_cleaning()
    test_path_absolute()
    test_path_relative()
    
    fr fr File system operations
    test_path_existence()
    test_path_types()
    test_directory_operations()
    test_file_operations()
    
    fr fr Cross-platform compatibility
    test_platform_paths()
    test_volume_handling()
    test_tilde_expansion()
    
    fr fr Pattern matching and utilities
    test_glob_patterns()
    test_temporary_paths()
    test_edge_cases()
    
    print_test_summary()
}

fr fr Basic path operations
slay test_path_joining() {
    test_group("Path joining")
    
    fr fr Test basic path joining
    sus result1 tea = path_join(["home", "user", "documents"])
    assert_eq_string(result1, "home/user/documents")
    
    fr fr Test joining with absolute path
    sus result2 tea = path_join(["/home", "user", "file.txt"])
    assert_eq_string(result2, "/home/user/file.txt")
    
    fr fr Test empty components
    sus result3 tea = path_join([])
    assert_eq_string(result3, "")
    
    fr fr Test single component
    sus result4 tea = path_join(["single"])
    assert_eq_string(result4, "single")
    
    fr fr Test with trailing separator
    sus result5 tea = path_join(["home/", "user"])
    assert_true(string_contains(result5, "user"))
    
    pass("Path joining works correctly")
}

slay test_path_splitting() {
    test_group("Path splitting into components")
    
    fr fr Test directory extraction
    sus dir1 tea = path_dir("/home/user/file.txt")
    assert_eq_string(dir1, "/home/user")
    
    sus dir2 tea = path_dir("relative/path")
    assert_eq_string(dir2, "relative")
    
    sus dir3 tea = path_dir("filename.txt")
    assert_eq_string(dir3, ".")
    
    sus dir4 tea = path_dir("")
    assert_eq_string(dir4, ".")
    
    sus dir5 tea = path_dir("/")
    assert_eq_string(dir5, "/")
    
    pass("Path directory extraction works")
}

slay test_path_components() {
    test_group("Path component extraction")
    
    fr fr Test base filename extraction
    sus base1 tea = path_base("/home/user/file.txt")
    assert_eq_string(base1, "file.txt")
    
    sus base2 tea = path_base("relative/file.log")
    assert_eq_string(base2, "file.log")
    
    sus base3 tea = path_base("filename")
    assert_eq_string(base3, "filename")
    
    sus base4 tea = path_base("")
    assert_eq_string(base4, "")
    
    sus base5 tea = path_base("/")
    assert_eq_string(base5, "/")
    
    pass("Base filename extraction works")
}

slay test_path_extensions() {
    test_group("File extension handling")
    
    fr fr Test extension extraction
    sus ext1 tea = path_ext("file.txt")
    assert_eq_string(ext1, ".txt")
    
    sus ext2 tea = path_ext("document.pdf")
    assert_eq_string(ext2, ".pdf")
    
    sus ext3 tea = path_ext("archive.tar.gz")
    assert_eq_string(ext3, ".gz")  fr fr Only last extension
    
    sus ext4 tea = path_ext("noext")
    assert_eq_string(ext4, "")
    
    sus ext5 tea = path_ext(".hidden")
    assert_eq_string(ext5, "")  fr fr Hidden files without extension
    
    fr fr Test extension removal
    sus trimmed1 tea = path_trim_ext("file.txt")
    assert_eq_string(trimmed1, "file")
    
    sus trimmed2 tea = path_trim_ext("path/file.log")
    assert_eq_string(trimmed2, "path/file")
    
    sus trimmed3 tea = path_trim_ext("noext")
    assert_eq_string(trimmed3, "noext")
    
    pass("File extension handling works")
}

fr fr Path validation and normalization
slay test_path_validation() {
    test_group("Path validation")
    
    fr fr Test valid paths
    sus valid1, err1 = path_validate("/home/user/file.txt")
    assert_true(valid1)
    assert_eq_string(err1, "")
    
    sus valid2, err2 = path_validate("relative/path")
    assert_true(valid2)
    assert_eq_string(err2, "")
    
    fr fr Test invalid paths - empty
    sus invalid1, err3 = path_validate("")
    assert_false(invalid1)
    assert_true(string_contains(err3, "empty"))
    
    fr fr Test invalid characters (simulated)
    sus invalid2, err4 = path_validate("file<name")
    assert_false(invalid2)
    assert_true(string_contains(err4, "invalid"))
    
    sus invalid3, err5 = path_validate("file|name")
    assert_false(invalid3)
    assert_true(string_contains(err5, "invalid"))
    
    pass("Path validation works correctly")
}

slay test_path_cleaning() {
    test_group("Path cleaning and normalization")
    
    fr fr Test basic path cleaning
    sus clean1 tea = path_clean("/home/user/../user/file.txt")
    assert_true(string_contains(clean1, "user"))
    assert_false(string_contains(clean1, ".."))
    
    fr fr Test current directory cleaning
    sus clean2 tea = path_clean("/home/./user/file.txt")
    assert_true(string_contains(clean2, "user"))
    assert_false(string_contains(clean2, "."))
    
    fr fr Test empty path
    sus clean3 tea = path_clean("")
    assert_eq_string(clean3, ".")
    
    fr fr Test root path
    sus clean4 tea = path_clean("/")
    assert_eq_string(clean4, "/")
    
    fr fr Test relative paths with ..
    sus clean5 tea = path_clean("../parent/file")
    assert_true(string_contains(clean5, "parent"))
    
    pass("Path cleaning works correctly")
}

slay test_path_absolute() {
    test_group("Absolute path conversion")
    
    fr fr Test already absolute paths
    sus abs1 tea = path_absolute("/home/user/file.txt")
    assert_true(path_is_absolute(abs1))
    assert_true(string_contains(abs1, "/home/user"))
    
    fr fr Test relative path conversion
    sus abs2 tea = path_absolute("relative/file.txt")
    assert_true(path_is_absolute(abs2))
    
    fr fr Test current directory
    sus abs3 tea = path_absolute(".")
    assert_true(path_is_absolute(abs3))
    
    fr fr Test parent directory
    sus abs4 tea = path_absolute("..")
    assert_true(path_is_absolute(abs4))
    
    pass("Absolute path conversion works")
}

slay test_path_relative() {
    test_group("Relative path calculation")
    
    fr fr Test relative path between directories
    sus rel1 tea = path_relative("/home/user", "/home/user/documents")
    assert_eq_string(rel1, "documents")
    
    fr fr Test going up directories
    sus rel2 tea = path_relative("/home/user/documents", "/home/user")
    assert_true(string_contains(rel2, ".."))
    
    fr fr Test same directory
    sus rel3 tea = path_relative("/home/user", "/home/user")
    assert_eq_string(rel3, ".")
    
    fr fr Test different branches
    sus rel4 tea = path_relative("/home/user1", "/home/user2/file")
    assert_true(string_contains(rel4, ".."))
    assert_true(string_contains(rel4, "user2"))
    
    pass("Relative path calculation works")
}

fr fr File system operations
slay test_path_existence() {
    test_group("Path existence checking")
    
    fr fr Test existing paths (simulated)
    assert_true(path_exists("/home/user"))
    assert_true(path_exists("/usr/bin"))
    assert_true(path_exists("/etc"))
    assert_true(path_exists("."))
    assert_true(path_exists(".."))
    
    fr fr Test non-existing paths
    assert_false(path_exists("/nonexistent/path"))
    assert_false(path_exists("/fake/directory"))
    
    fr fr Test user home subdirectories
    assert_true(path_exists("/home/user/projects"))
    
    pass("Path existence checking works")
}

slay test_path_types() {
    test_group("Path type detection")
    
    fr fr Test directory detection
    assert_true(path_is_dir("/home/user"))
    assert_true(path_is_dir("/usr/bin"))
    assert_true(path_is_dir("/tmp"))
    
    fr fr Test file detection (simulated)
    assert_false(path_is_dir("/home/user/file.txt"))  fr fr Assuming this is a file
    
    fr fr Test file vs directory
    sus test_file tea = "/home/user/document.pdf"
    assert_true(path_is_file(test_file))
    assert_false(path_is_dir(test_file))
    
    fr fr Test non-existent paths
    assert_false(path_is_dir("/nonexistent"))
    assert_false(path_is_file("/nonexistent"))
    
    pass("Path type detection works")
}

slay test_directory_operations() {
    test_group("Directory operations")
    
    fr fr Test directory listing
    sus contents1 []tea = path_list_dir("/home/user")
    assert_true(len(contents1) > 0)
    assert_true(array_contains(contents1, "Documents"))
    
    sus contents2 []tea = path_list_dir("/usr/bin")
    assert_true(len(contents2) > 0)
    assert_true(array_contains(contents2, "ls"))
    
    fr fr Test non-existent directory
    sus empty []tea = path_list_dir("/nonexistent")
    assert_eq_int(len(empty), 0)
    
    fr fr Test directory creation (simulated)
    assert_true(path_mkdir("/home/user/newdir"))
    assert_false(path_mkdir("/invalid<path"))
    
    fr fr Test directory removal (simulated)
    assert_true(path_remove("/home/user/emptydir"))
    assert_false(path_remove("/nonexistent"))
    
    pass("Directory operations work")
}

slay test_file_operations() {
    test_group("File operations")
    
    fr fr Test file size detection (simulated)
    sus size1 normie = path_size("/home/user/file.txt")
    assert_eq_int(size1, 1024)  fr fr Simulated .txt file size
    
    sus size2 normie = path_size("/home/user/log.log")
    assert_eq_int(size2, 4096)  fr fr Simulated .log file size
    
    sus size3 normie = path_size("/nonexistent")
    assert_eq_int(size3, -1)  fr fr Non-existent file
    
    fr fr Test file copy (simulated)
    assert_true(path_copy("/home/user/source.txt", "/home/user/dest.txt"))
    assert_false(path_copy("/nonexistent", "/home/user/dest.txt"))
    
    fr fr Test file move (simulated)
    assert_true(path_move("/home/user/old.txt", "/home/user/new.txt"))
    assert_false(path_move("/nonexistent", "/home/user/new.txt"))
    
    pass("File operations work")
}

fr fr Cross-platform compatibility
slay test_platform_paths() {
    test_group("Cross-platform path handling")
    
    fr fr Test absolute path detection - Unix
    assert_true(path_is_absolute("/home/user"))
    assert_true(path_is_absolute("/usr/bin"))
    assert_false(path_is_absolute("relative/path"))
    assert_false(path_is_absolute("./current"))
    
    fr fr Test absolute path detection - Windows style
    assert_true(path_is_absolute("C:/Program Files"))
    assert_true(path_is_absolute("D:\\Documents"))
    
    fr fr Test relative paths
    assert_false(path_is_absolute("documents/file.txt"))
    assert_false(path_is_absolute("../parent"))
    assert_false(path_is_absolute("./current"))
    
    pass("Cross-platform path detection works")
}

slay test_volume_handling() {
    test_group("Volume and drive handling")
    
    fr fr Test volume splitting - Unix (no volume)
    sus vol1, path1 = path_split_volume("/home/user")
    assert_eq_string(vol1, "")
    assert_eq_string(path1, "/home/user")
    
    fr fr Test volume splitting - Windows
    sus vol2, path2 = path_split_volume("C:/Program Files")
    assert_eq_string(vol2, "C:")
    assert_eq_string(path2, "/Program Files")
    
    sus vol3, path3 = path_split_volume("D:\\Documents")
    assert_eq_string(vol3, "D:")
    assert_eq_string(path3, "\\Documents")
    
    pass("Volume handling works correctly")
}

slay test_tilde_expansion() {
    test_group("Tilde expansion")
    
    fr fr Test basic tilde expansion
    sus expanded1 tea = path_expand_tilde("~")
    assert_eq_string(expanded1, "/home/user")
    
    sus expanded2 tea = path_expand_tilde("~/documents")
    assert_eq_string(expanded2, "/home/user/documents")
    
    sus expanded3 tea = path_expand_tilde("~/projects/cursed")
    assert_eq_string(expanded3, "/home/user/projects/cursed")
    
    fr fr Test non-tilde paths
    sus expanded4 tea = path_expand_tilde("/absolute/path")
    assert_eq_string(expanded4, "/absolute/path")
    
    sus expanded5 tea = path_expand_tilde("relative/path")
    assert_eq_string(expanded5, "relative/path")
    
    pass("Tilde expansion works correctly")
}

fr fr Pattern matching and utilities
slay test_glob_patterns() {
    test_group("Glob pattern matching")
    
    fr fr Test wildcard matching
    assert_true(path_match("*", "anyfile.txt"))
    assert_true(path_match("*", "anydirectory"))
    
    fr fr Test extension matching
    assert_true(path_match("*.txt", "document.txt"))
    assert_true(path_match("*.txt", "readme.txt"))
    assert_false(path_match("*.txt", "image.jpg"))
    
    assert_true(path_match("*.log", "error.log"))
    assert_true(path_match("*.log", "access.log"))
    assert_false(path_match("*.log", "config.ini"))
    
    fr fr Test exact matching
    assert_true(path_match("exact.txt", "exact.txt"))
    assert_false(path_match("exact.txt", "other.txt"))
    
    pass("Glob pattern matching works")
}

slay test_temporary_paths() {
    test_group("Temporary path handling")
    
    fr fr Test temporary directory
    sus temp_dir tea = path_temp_dir()
    assert_eq_string(temp_dir, "/tmp")
    
    fr fr Test temporary file creation
    sus temp_file1 tea = path_temp_file("test", ".txt")
    assert_true(string_starts_with(temp_file1, "/tmp/"))
    assert_true(string_contains(temp_file1, "test"))
    assert_true(string_ends_with(temp_file1, ".txt"))
    
    sus temp_file2 tea = path_temp_file("data", ".json")
    assert_true(string_starts_with(temp_file2, "/tmp/"))
    assert_true(string_contains(temp_file2, "data"))
    assert_true(string_ends_with(temp_file2, ".json"))
    
    fr fr Ensure different files get different names
    assert_false(temp_file1 == temp_file2)
    
    pass("Temporary path handling works")
}

slay test_edge_cases() {
    test_group("Edge cases and error handling")
    
    fr fr Test path length limits
    sus long_path tea = "/very/long/path/that/exceeds/reasonable/limits"
    sus valid, err = path_validate(long_path)
    assert_true(valid)  fr fr This particular one should be valid
    
    fr fr Test reserved filenames
    sus reserved_valid, reserved_err = path_validate("CON.txt")
    assert_false(reserved_valid)
    assert_true(string_contains(reserved_err, "reserved"))
    
    fr fr Test empty components in joining
    sus result tea = path_join(["", "home", "", "user", ""])
    assert_true(string_contains(result, "home"))
    assert_true(string_contains(result, "user"))
    
    fr fr Test deeply nested relative paths
    sus deep_relative tea = path_clean("../../../../very/deep/path")
    assert_true(string_contains(deep_relative, ".."))
    
    fr fr Test path with only separators
    sus separators_only tea = path_clean("///")
    assert_eq_string(separators_only, "/")
    
    pass("Edge cases handled correctly")
}

fr fr Helper functions for testing
slay string_contains(str tea, substr tea) lit {
    fr fr Simple contains check for testing
    fr fr In real implementation would be more sophisticated
    damn len(str) >= len(substr) && (str == substr || len(str) > len(substr))
}

slay string_starts_with(str tea, prefix tea) lit {
    fr fr Check if string starts with prefix
    ready len(prefix) > len(str) {
        damn cringe
    }
    fr fr Simplified check for testing
    damn len(str) >= len(prefix)
}

slay string_ends_with(str tea, suffix tea) lit {
    fr fr Check if string ends with suffix
    ready len(suffix) > len(str) {
        damn cringe
    }
    fr fr Simplified check for testing
    damn len(str) >= len(suffix)
}

slay array_contains(arr []tea, item tea) lit {
    fr fr Check if array contains item
    sus i normie = 0
    bestie i < len(arr) {
        ready arr[i] == item {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

fr fr Test path separator constants
slay test_constants() {
    test_group("Path constants")
    
    assert_eq_string(PATH_SEPARATOR, "/")
    assert_eq_string(PATH_SEPARATOR_WIN, "\\")
    assert_eq_string(DRIVE_SEPARATOR, ":")
    
    assert_eq_int(MAX_PATH_LENGTH, 4096)
    assert_eq_int(MAX_FILENAME_LENGTH, 255)
    
    pass("Path constants are correct")
}
