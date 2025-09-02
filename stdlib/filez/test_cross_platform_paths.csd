fr fr COMPREHENSIVE CROSS-PLATFORM PATH TESTING SUITE
fr fr Tests Windows, Unix, macOS path handling with edge cases

yeet "filez"
yeet "vibez"
yeet "testz"

fr fr ===== TEST UTILITIES =====

slay assert_path_equals(actual tea, expected tea, test_name tea) {
    ready (actual == expected) {
        vibez.spill("✓ " + test_name + " - PASSED")
    } otherwise {
        vibez.spill("✗ " + test_name + " - FAILED")
        vibez.spill("  Expected: " + expected)
        vibez.spill("  Actual:   " + actual)
        assert_eq_tea(actual, expected)
    }
}

slay assert_path_absolute(path tea, expected_absolute lit, test_name tea) {
    sus result lit = is_absolute_path(path)
    ready (result == expected_absolute) {
        vibez.spill("✓ " + test_name + " - PASSED")
    } otherwise {
        vibez.spill("✗ " + test_name + " - FAILED")
        vibez.spill("  Path: " + path)
        vibez.spill("  Expected absolute: " + bool_to_string(expected_absolute))
        vibez.spill("  Actual absolute: " + bool_to_string(result))
        assert_eq_lit(result, expected_absolute)
    }
}

slay bool_to_string(value lit) tea {
    ready (value) {
        damn "true"
    } otherwise {
        damn "false"
    }
}

fr fr ===== WINDOWS PATH TESTS =====

slay test_windows_drive_paths() {
    vibez.spill("\n=== Testing Windows Drive Paths ===")
    
    fr fr Test basic drive paths
    sus drive_path tea = "C:\\Users\\Developer"
    sus drive_info DriveInfo = parse_drive_info(drive_path)
    assert_eq_lit(drive_info.has_drive, based)
    assert_eq_tea(drive_info.letter, "C")
    assert_path_absolute(drive_path, based, "Drive path is absolute")
    
    fr fr Test drive without separator
    sus drive_only tea = "C:"
    sus drive_only_info DriveInfo = parse_drive_info(drive_only)
    assert_eq_lit(drive_only_info.has_drive, based)
    assert_path_absolute(drive_only, cringe, "Drive without separator is not absolute")
    
    fr fr Test mixed separators
    sus mixed_sep tea = "C:\\Users/Developer\\Documents"
    sus normalized tea = normalize_path_separators(mixed_sep)
    sus expected_normalized tea = "C:\\Users\\Developer\\Documents"
    assert_path_equals(normalized, expected_normalized, "Mixed separators normalized")
    
    fr fr Test path joining
    sus parts tea[value] = ["C:\\Users", "Developer", "Documents", "file.txt"]
    sus joined tea = cross_platform_join(parts)
    sus expected_join tea = "C:\\Users\\Developer\\Documents\\file.txt"
    assert_path_equals(joined, expected_join, "Windows path joining")
    
    vibez.spill("Windows drive path tests completed")
}

slay test_windows_unc_paths() {
    vibez.spill("\n=== Testing Windows UNC Paths ===")
    
    fr fr Test basic UNC path
    sus unc_path tea = "\\\\server\\share\\folder\\file.txt"
    sus unc_info DriveInfo = parse_drive_info(unc_path)
    assert_eq_lit(unc_info.is_unc, based)
    assert_eq_tea(unc_info.server_name, "server")
    assert_eq_tea(unc_info.share_name, "share")
    assert_path_absolute(unc_path, based, "UNC path is absolute")
    
    fr fr Test UNC with forward slashes
    sus unc_forward tea = "//server/share/folder/file.txt"
    sus unc_forward_info DriveInfo = parse_drive_info(unc_forward)
    assert_eq_lit(unc_forward_info.is_unc, based)
    assert_eq_tea(unc_forward_info.server_name, "server")
    assert_eq_tea(unc_forward_info.share_name, "share")
    
    fr fr Test UNC root path
    sus unc_root tea = get_root_path(unc_path)
    sus expected_root tea = "\\\\server\\share\\"
    assert_path_equals(unc_root, expected_root, "UNC root path")
    
    vibez.spill("Windows UNC path tests completed")
}

slay test_windows_path_validation() {
    vibez.spill("\n=== Testing Windows Path Validation ===")
    
    fr fr Test valid path
    sus valid_path tea = "C:\\Users\\Developer\\file.txt"
    assert_eq_lit(validate_path_chars(valid_path), based)
    
    fr fr Test invalid characters
    sus invalid_chars tea[value] = ["<", ">", ":", "\"", "|", "?", "*"]
    sus i drip = 0
    bestie (i < array_length(invalid_chars)) {
        sus invalid_path tea = "C:\\Users\\file" + invalid_chars[i] + ".txt"
        sus is_valid lit = validate_path_chars(invalid_path)
        assert_eq_lit(is_valid, cringe)
        i = i + 1
    }
    
    fr fr Test path length limits
    sus long_path tea = "C:\\" + repeat_string("verylongfoldername", 200) + "\\file.txt"
    assert_eq_lit(validate_path_length(long_path), cringe)
    
    vibez.spill("Windows path validation tests completed")
}

fr fr ===== UNIX/LINUX PATH TESTS =====

slay test_unix_absolute_paths() {
    vibez.spill("\n=== Testing Unix Absolute Paths ===")
    
    fr fr Test basic absolute path
    sus abs_path tea = "/home/user/documents/file.txt"
    assert_path_absolute(abs_path, based, "Unix absolute path")
    
    fr fr Test root path
    sus root_path tea = "/"
    assert_path_absolute(root_path, based, "Unix root path")
    
    fr fr Test relative path
    sus rel_path tea = "documents/file.txt"
    assert_path_absolute(rel_path, cringe, "Unix relative path")
    
    fr fr Test current directory
    sus current_path tea = "./file.txt"
    assert_path_absolute(current_path, cringe, "Current directory path")
    
    fr fr Test parent directory
    sus parent_path tea = "../file.txt"
    assert_path_absolute(parent_path, cringe, "Parent directory path")
    
    vibez.spill("Unix absolute path tests completed")
}

slay test_unix_path_operations() {
    vibez.spill("\n=== Testing Unix Path Operations ===")
    
    fr fr Test path joining
    sus parts tea[value] = ["/home", "user", "documents", "file.txt"]
    sus joined tea = cross_platform_join(parts)
    sus expected tea = "/home/user/documents/file.txt"
    assert_path_equals(joined, expected, "Unix path joining")
    
    fr fr Test path components
    sus components tea[value] = get_path_components("/home/user/documents/file.txt")
    assert_eq_tea(components[0], "/")
    assert_eq_tea(components[1], "home")
    assert_eq_tea(components[2], "user")
    assert_eq_tea(components[3], "documents")
    assert_eq_tea(components[4], "file.txt")
    
    fr fr Test filename extraction
    sus filename tea = get_filename_component("/home/user/documents/file.txt")
    assert_path_equals(filename, "file.txt", "Unix filename extraction")
    
    fr fr Test parent directory
    sus parent tea = get_parent_directory("/home/user/documents/file.txt")
    assert_path_equals(parent, "/home/user/documents", "Unix parent directory")
    
    vibez.spill("Unix path operations tests completed")
}

fr fr ===== PATH NORMALIZATION TESTS =====

slay test_path_normalization() {
    vibez.spill("\n=== Testing Path Normalization ===")
    
    fr fr Test current directory normalization
    sus current_dir tea = "/home/user/./documents/file.txt"
    sus normalized tea = cross_platform_normalize(current_dir)
    sus expected tea = "/home/user/documents/file.txt"
    assert_path_equals(normalized, expected, "Current directory normalization")
    
    fr fr Test parent directory normalization
    sus parent_dir tea = "/home/user/documents/../file.txt"
    sus parent_normalized tea = cross_platform_normalize(parent_dir)
    sus parent_expected tea = "/home/user/file.txt"
    assert_path_equals(parent_normalized, parent_expected, "Parent directory normalization")
    
    fr fr Test multiple parent directories
    sus multi_parent tea = "/home/user/documents/../../user/file.txt"
    sus multi_normalized tea = cross_platform_normalize(multi_parent)
    sus multi_expected tea = "/home/user/file.txt"
    assert_path_equals(multi_normalized, multi_expected, "Multiple parent directories")
    
    fr fr Test empty components
    sus empty_components tea = "/home//user///documents/file.txt"
    sus empty_normalized tea = cross_platform_normalize(empty_components)
    sus empty_expected tea = "/home/user/documents/file.txt"
    assert_path_equals(empty_normalized, empty_expected, "Empty components normalization")
    
    fr fr Test trailing separator
    sus trailing_sep tea = "/home/user/documents/"
    sus trailing_normalized tea = cross_platform_normalize(trailing_sep)
    sus trailing_expected tea = "/home/user/documents"
    assert_path_equals(trailing_normalized, trailing_expected, "Trailing separator normalization")
    
    vibez.spill("Path normalization tests completed")
}

fr fr ===== RELATIVE PATH CALCULATION TESTS =====

slay test_relative_path_calculation() {
    vibez.spill("\n=== Testing Relative Path Calculation ===")
    
    fr fr Test same directory
    sus from_path tea = "/home/user/documents"
    sus to_path tea = "/home/user/documents/file.txt"
    sus relative tea = calculate_relative_path(from_path, to_path)
    assert_path_equals(relative, "file.txt", "Same directory relative path")
    
    fr fr Test parent directory
    sus parent_from tea = "/home/user/documents/subfolder"
    sus parent_to tea = "/home/user/documents/file.txt"
    sus parent_relative tea = calculate_relative_path(parent_from, parent_to)
    assert_path_equals(parent_relative, "../file.txt", "Parent directory relative path")
    
    fr fr Test sibling directories
    sus sibling_from tea = "/home/user/documents"
    sus sibling_to tea = "/home/user/downloads/file.txt"
    sus sibling_relative tea = calculate_relative_path(sibling_from, sibling_to)
    assert_path_equals(sibling_relative, "../downloads/file.txt", "Sibling directory relative path")
    
    fr fr Test deep relative path
    sus deep_from tea = "/home/user/documents/projects/project1"
    sus deep_to tea = "/home/user/downloads/archives/archive.zip"
    sus deep_relative tea = calculate_relative_path(deep_from, deep_to)
    sus deep_expected tea = "../../../downloads/archives/archive.zip"
    assert_path_equals(deep_relative, deep_expected, "Deep relative path")
    
    vibez.spill("Relative path calculation tests completed")
}

fr fr ===== FILE EXTENSION TESTS =====

slay test_file_extensions() {
    vibez.spill("\n=== Testing File Extensions ===")
    
    fr fr Test basic extension
    sus file_with_ext tea = "/home/user/document.txt"
    sus extension tea = get_extension_component(file_with_ext)
    assert_path_equals(extension, ".txt", "Basic file extension")
    
    fr fr Test multiple dots
    sus multi_dot tea = "/home/user/archive.tar.gz"
    sus multi_extension tea = get_extension_component(multi_dot)
    assert_path_equals(multi_extension, ".gz", "Multiple dots extension")
    
    fr fr Test no extension
    sus no_ext tea = "/home/user/README"
    sus no_extension tea = get_extension_component(no_ext)
    assert_path_equals(no_extension, "", "No extension")
    
    fr fr Test hidden file
    sus hidden_file tea = "/home/user/.bashrc"
    sus hidden_extension tea = get_extension_component(hidden_file)
    assert_path_equals(hidden_extension, "", "Hidden file no extension")
    
    fr fr Test filename without extension
    sus filename_no_ext tea = get_filename_without_extension(file_with_ext)
    assert_path_equals(filename_no_ext, "document", "Filename without extension")
    
    vibez.spill("File extension tests completed")
}

fr fr ===== EDGE CASES AND ERROR HANDLING =====

slay test_edge_cases() {
    vibez.spill("\n=== Testing Edge Cases ===")
    
    fr fr Test empty path
    sus empty_path tea = ""
    sus empty_normalized tea = cross_platform_normalize(empty_path)
    assert_path_equals(empty_normalized, "", "Empty path normalization")
    
    fr fr Test single dot
    sus single_dot tea = "."
    sus dot_normalized tea = cross_platform_normalize(single_dot)
    assert_path_equals(dot_normalized, ".", "Single dot normalization")
    
    fr fr Test double dot
    sus double_dot tea = ".."
    sus double_normalized tea = cross_platform_normalize(double_dot)
    assert_path_equals(double_normalized, "..", "Double dot normalization")
    
    fr fr Test root with parent directory
    sus root_parent tea = "/.."
    sus root_parent_normalized tea = cross_platform_normalize(root_parent)
    assert_path_equals(root_parent_normalized, "/", "Root with parent directory")
    
    fr fr Test only separators
    sus only_seps tea = "///"
    sus seps_normalized tea = cross_platform_normalize(only_seps)
    assert_path_equals(seps_normalized, "/", "Only separators normalization")
    
    vibez.spill("Edge cases tests completed")
}

fr fr ===== CROSS-PLATFORM COMPATIBILITY TESTS =====

slay test_cross_platform_compatibility() {
    vibez.spill("\n=== Testing Cross-Platform Compatibility ===")
    
    fr fr Test path joining with mixed types
    sus mixed_parts tea[value] = ["C:\\Users", "user", "documents/subfolder", "file.txt"]
    sus mixed_joined tea = cross_platform_join(mixed_parts)
    vibez.spill("Mixed path joining: " + mixed_joined)
    
    fr fr Test separator consistency
    sus current_sep tea = get_platform_separator()
    sus alt_sep tea = get_alt_separator()
    vibez.spill("Platform separator: '" + current_sep + "'")
    vibez.spill("Alternative separator: '" + alt_sep + "'")
    
    fr fr Test platform detection
    sus platform tea = detect_platform()
    vibez.spill("Detected platform: " + platform)
    
    fr fr Test absolute path conversion
    sus rel_path tea = "documents/file.txt"
    sus abs_path tea = cross_platform_absolute(rel_path)
    vibez.spill("Relative to absolute: " + rel_path + " -> " + abs_path)
    
    vibez.spill("Cross-platform compatibility tests completed")
}

fr fr ===== HELPER FUNCTIONS =====

slay repeat_string(str tea, count drip) tea {
    fr fr Helper to repeat string for testing
    sus result tea = ""
    sus i drip = 0
    bestie (i < count) {
        result = result + str
        i = i + 1
    }
    damn result
}

fr fr ===== MAIN TEST RUNNER =====

slay run_all_tests() {
    vibez.spill("🧪 CURSED Cross-Platform Path Handling Test Suite")
    vibez.spill("=" + repeat_string("=", 50))
    
    fr fr Run all test suites
    test_windows_drive_paths()
    test_windows_unc_paths()
    test_windows_path_validation()
    test_unix_absolute_paths()
    test_unix_path_operations()
    test_path_normalization()
    test_relative_path_calculation()
    test_file_extensions()
    test_edge_cases()
    test_cross_platform_compatibility()
    
    vibez.spill("\n" + repeat_string("=", 50))
    vibez.spill("✅ All Cross-Platform Path Tests Completed!")
    vibez.spill("Ready for production use on Windows, Unix, and macOS")
}

fr fr Execute tests when module is imported
test_start("cross_platform_paths")
run_all_tests()
print_test_summary()
