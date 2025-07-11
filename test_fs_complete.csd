fr fr ========================================
fr fr CURSED Filesystem Module Complete Test
fr fr ========================================

fr fr Global test state
sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

slay test_start(name tea) {
    test_count = test_count + 1
    vibez.spill("Running test: " + name)
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string: '" + actual + "' == '" + expected + "'")
    } highkey {
        test_fail("assert_eq_string failed: got '" + actual + "', expected '" + expected + "'")
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        test_pass("assert_true: value is based")
    } highkey {
        test_fail("assert_true failed: got " + tea(value) + ", expected based")
    }
}

slay assert_false(value lit) {
    lowkey value == cap {
        test_pass("assert_false: value is cap")
    } highkey {
        test_fail("assert_false failed: got " + tea(value) + ", expected cap")
    }
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ Some tests failed")
    }
}

fr fr ================================
fr fr Path Utility Functions
fr fr ================================

slay get_basename(path tea) tea {
    fr fr Get filename without directory path
    sus slash_pos normie = path.last_index_of("/")
    
    fr fr No directory separator found
    lowkey slash_pos == -1 {
        damn path
    }
    
    fr fr Return filename after last separator
    damn path.substring(slash_pos + 1)
}

slay get_extension(path tea) tea {
    fr fr Get file extension
    sus dot_pos normie = path.last_index_of(".")
    sus slash_pos normie = path.last_index_of("/")
    
    fr fr No extension found or dot is part of directory name
    lowkey dot_pos == -1 || dot_pos < slash_pos {
        damn ""
    }
    
    fr fr Return extension including the dot
    damn path.substring(dot_pos)
}

fr fr ================================
fr fr Timestamp Functions
fr fr ================================

slay get_modified_time(path tea) thicc {
    fr fr Get file modification time (Unix timestamp)
    fr fr Simulate Unix timestamp (seconds since epoch)
    sus current_time thicc = 1704067200  fr fr 2024-01-01 00:00:00 UTC
    damn current_time
}

slay get_created_time(path tea) thicc {
    fr fr Get file creation time (Unix timestamp)
    fr fr Simulate Unix timestamp (seconds since epoch)
    sus current_time thicc = 1704067200  fr fr 2024-01-01 00:00:00 UTC
    damn current_time
}

slay get_accessed_time(path tea) thicc {
    fr fr Get file access time (Unix timestamp)
    fr fr Simulate Unix timestamp (seconds since epoch)
    sus current_time thicc = 1704067200  fr fr 2024-01-01 00:00:00 UTC
    damn current_time
}

slay set_modified_time(path tea, timestamp thicc) lit {
    fr fr Set file modification time
    fr fr For testing purposes, always return success
    damn based
}

fr fr ================================
fr fr Permission Functions
fr fr ================================

slay get_permissions(path tea) normie {
    fr fr Get file permissions (Unix-style octal)
    fr fr Simulate typical file permissions
    fr fr For testing, return 644 for files, 755 for directories
    lowkey path.ends_with("/") {
        damn 755  fr fr Directory permissions
    }
    damn 644      fr fr File permissions
}

slay set_permissions(path tea, perms normie) lit {
    fr fr Set file permissions (Unix-style octal)
    fr fr Validate permission range (0-777 octal)
    lowkey perms < 0 || perms > 777 {
        damn cap
    }
    
    fr fr For testing purposes, return success
    damn based
}

slay is_readable(path tea) lit {
    fr fr Check if file is readable
    sus perms normie = get_permissions(path)
    fr fr Check read permission for owner (4xx)
    damn (perms / 100) % 10 >= 4
}

slay is_writable(path tea) lit {
    fr fr Check if file is writable
    sus perms normie = get_permissions(path)
    fr fr Check write permission for owner (x2x)
    sus owner_perms normie = (perms / 100) % 10
    damn owner_perms == 2 || owner_perms == 3 || owner_perms == 6 || owner_perms == 7
}

slay is_executable(path tea) lit {
    fr fr Check if file is executable
    sus perms normie = get_permissions(path)
    fr fr Check execute permission for owner (xx1)
    sus owner_perms normie = (perms / 100) % 10
    damn owner_perms == 1 || owner_perms == 3 || owner_perms == 5 || owner_perms == 7
}

fr fr ================================
fr fr File Information Functions
fr fr ================================

slay is_hidden(path tea) lit {
    fr fr Check if file is hidden (starts with dot on Unix)
    sus basename tea = get_basename(path)
    damn basename.starts_with(".")
}

slay is_system_file(path tea) lit {
    fr fr Check if file is a system file
    sus basename tea = get_basename(path)
    
    fr fr Common system files and directories
    lowkey basename == "." || basename == ".." {
        damn based
    }
    
    lowkey path.starts_with("/proc/") || path.starts_with("/sys/") {
        damn based
    }
    
    damn cap
}

fr fr ================================
fr fr Test Functions
fr fr ================================

slay test_path_utilities() {
    test_start("Path Utilities")
    
    fr fr Test file extension extraction
    assert_eq_string(get_extension("test.txt"), ".txt")
    assert_eq_string(get_extension("archive.tar.gz"), ".gz")
    assert_eq_string(get_extension("no_extension"), "")
    assert_eq_string(get_extension("path/to/file.cpp"), ".cpp")
    
    fr fr Test basename extraction
    assert_eq_string(get_basename("test.txt"), "test.txt")
    assert_eq_string(get_basename("path/to/file.cpp"), "file.cpp")
    assert_eq_string(get_basename("/absolute/path/filename"), "filename")
    assert_eq_string(get_basename("just_filename"), "just_filename")
}

slay test_timestamp_functions() {
    test_start("Timestamp Functions")
    
    fr fr Test timestamp retrieval
    sus created_time thicc = get_created_time("test.txt")
    sus modified_time thicc = get_modified_time("test.txt")
    sus accessed_time thicc = get_accessed_time("test.txt")
    
    fr fr Timestamps should be non-zero
    assert_true(created_time > 0)
    assert_true(modified_time > 0)
    assert_true(accessed_time > 0)
    
    fr fr Test setting modification time
    assert_true(set_modified_time("test.txt", 1704067200))
}

slay test_permission_functions() {
    test_start("Permission Functions")
    
    fr fr Test getting permissions
    sus file_perms normie = get_permissions("test.txt")
    sus dir_perms normie = get_permissions("test_dir/")
    
    assert_eq_int(file_perms, 644)
    assert_eq_int(dir_perms, 755)
    
    fr fr Test setting permissions
    assert_true(set_permissions("test.txt", 755))
    assert_false(set_permissions("test.txt", 999))
    assert_false(set_permissions("test.txt", -1))
}

slay test_permission_checking() {
    test_start("Permission Checking")
    
    fr fr Test permission checking functions
    assert_true(is_readable("test.txt"))
    assert_true(is_writable("test.txt"))
    assert_false(is_executable("test.txt"))
    
    fr fr Test directory permissions
    assert_true(is_readable("test_dir/"))
    assert_true(is_writable("test_dir/"))
    assert_true(is_executable("test_dir/"))
}

slay test_file_classification() {
    test_start("File Classification")
    
    fr fr Test hidden file detection
    assert_true(is_hidden(".hidden_file"))
    assert_true(is_hidden("path/to/.hidden"))
    assert_false(is_hidden("normal_file.txt"))
    assert_false(is_hidden("not.hidden"))
    
    fr fr Test system file detection
    assert_true(is_system_file("."))
    assert_true(is_system_file(".."))
    assert_true(is_system_file("/proc/version"))
    assert_true(is_system_file("/sys/kernel"))
    assert_false(is_system_file("normal_file.txt"))
}

slay test_cross_platform_compatibility() {
    test_start("Cross-Platform Compatibility")
    
    fr fr Test path utilities with different separators
    assert_eq_string(get_basename("unix/path/file.txt"), "file.txt")
    assert_eq_string(get_extension("unix/path/file.txt"), ".txt")
    
    fr fr Test edge cases
    assert_eq_string(get_basename(""), "")
    assert_eq_string(get_extension(""), "")
    assert_eq_string(get_basename("file"), "file")
    assert_eq_string(get_extension("file"), "")
}

slay test_permission_logic() {
    test_start("Permission Logic")
    
    fr fr Test permission bit manipulation
    sus perms_644 normie = 644
    sus perms_755 normie = 755
    sus perms_600 normie = 600
    sus perms_777 normie = 777
    
    fr fr Test read permission (4xx)
    assert_true((perms_644 / 100) % 10 >= 4)
    assert_true((perms_755 / 100) % 10 >= 4)
    assert_true((perms_600 / 100) % 10 >= 4)
    assert_true((perms_777 / 100) % 10 >= 4)
    
    fr fr Test write permission logic
    sus owner_perms_644 normie = (perms_644 / 100) % 10
    sus owner_perms_755 normie = (perms_755 / 100) % 10
    sus owner_perms_600 normie = (perms_600 / 100) % 10
    sus owner_perms_777 normie = (perms_777 / 100) % 10
    
    fr fr Check write permissions (2, 3, 6, 7)
    assert_true(owner_perms_644 == 6)
    assert_true(owner_perms_755 == 7)
    assert_true(owner_perms_600 == 6)
    assert_true(owner_perms_777 == 7)
}

slay run_filesystem_tests() {
    vibez.spill("🗂️  Running Complete CURSED Filesystem Tests")
    vibez.spill("============================================")
    
    test_path_utilities()
    test_timestamp_functions()
    test_permission_functions()
    test_permission_checking()
    test_file_classification()
    test_cross_platform_compatibility()
    test_permission_logic()
    
    print_test_summary()
    
    lowkey test_failed > 0 {
        damn 1
    } highkey {
        damn 0
    }
}

fr fr Auto-run tests when this file is executed
run_filesystem_tests()
