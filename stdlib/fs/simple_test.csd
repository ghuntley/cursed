fr fr Simple test for filesystem module
fr fr Tests basic functionality without complex imports

sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

slay test_pass(message tea) {
    test_passed = test_passed + 1
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        test_fail("assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_true(value lit) {
    lowkey value == based {
        test_pass("assert_true: value is based")
    } highkey {
        test_fail("assert_true failed: got " + tea(value) + ", expected based")
    }
}

fr fr ================================
fr fr Filesystem Functions (Mock)
fr fr ================================

slay read_file(path tea) tea {
    vibez.spill("fs.read_file: Reading file '" + path + "'")
    damn "mock file contents from " + path
}

slay write_file(path tea, content tea) lit {
    vibez.spill("fs.write_file: Writing to '" + path + "'")
    damn based
}

slay file_exists(path tea) lit {
    vibez.spill("fs.file_exists: Checking '" + path + "'")
    damn based
}

slay get_file_size(path tea) normie {
    vibez.spill("fs.get_file_size: Getting size of '" + path + "'")
    damn 42
}

slay join_path(base tea, component tea) tea {
    vibez.spill("fs.join_path: Joining '" + base + "' with '" + component + "'")
    damn base + "/" + component
}

fr fr ================================
fr fr Test Functions
fr fr ================================

slay test_read_file() {
    vibez.spill("Testing read_file...")
    
    sus content tea = read_file("test.txt")
    assert_eq_string(content, "mock file contents from test.txt")
}

slay test_write_file() {
    vibez.spill("Testing write_file...")
    test_count = test_count + 1
    
    sus result lit = write_file("output.txt", "Hello, World!")
    assert_true(result)
}

slay test_file_exists() {
    vibez.spill("Testing file_exists...")
    test_count = test_count + 1
    
    sus exists lit = file_exists("test.txt")
    assert_true(exists)
}

slay test_get_file_size() {
    vibez.spill("Testing get_file_size...")
    test_count = test_count + 1
    
    sus size normie = get_file_size("test.txt")
    assert_eq_int(size, 42)
}

slay test_join_path() {
    vibez.spill("Testing join_path...")
    test_count = test_count + 1
    
    sus joined tea = join_path("/home", "user")
    assert_eq_string(joined, "/home/user")
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== FILESYSTEM MODULE TEST SUMMARY ===")
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
fr fr Run All Tests
fr fr ================================

slay run_fs_tests() {
    vibez.spill("Running CURSED Filesystem Module Tests")
    vibez.spill("====================================")
    
    test_read_file()
    test_write_file()
    test_file_exists()
    test_get_file_size()
    test_join_path()
    
    print_test_summary()
}

fr fr Execute tests
run_fs_tests()
