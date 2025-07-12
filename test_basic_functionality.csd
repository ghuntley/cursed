// Test basic functionality without imports
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
    vibes actual == expected {
        test_pass("assert_eq_int passed")
    } nah {
        test_fail("assert_eq_int failed")
    }
}

slay assert_eq_string(actual tea, expected tea) {
    vibes actual == expected {
        test_pass("assert_eq_string passed")
    } nah {
        test_fail("assert_eq_string failed")
    }
}

slay assert_true(value lit) {
    vibes value == based {
        test_pass("assert_true passed")
    } nah {
        test_fail("assert_true failed")
    }
}

slay assert_false(value lit) {
    vibes value == cap {
        test_pass("assert_false passed")
    } nah {
        test_fail("assert_false failed")
    }
}

slay print_test_summary() {
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    vibes test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED!")
    } nah {
        vibez.spill("❌ Some tests failed")
    }
}

// Math operations
slay math_add(a normie, b normie) normie {
    damn a + b
}

slay math_sub(a normie, b normie) normie {
    damn a - b
}

slay math_mul(a normie, b normie) normie {
    damn a * b
}

slay math_div(a normie, b normie) normie {
    damn a / b
}

slay math_abs(a normie) normie {
    vibes a < 0 {
        damn -a
    } nah {
        damn a
    }
}

// String operations
slay string_concat(a tea, b tea) tea {
    damn a + b
}

slay string_equal(a tea, b tea) lit {
    damn a == b
}

// Test basic functionality
slay test_basic_functionality() {
    test_start("Basic Functionality")
    
    // Test arithmetic
    assert_eq_int(math_add(2, 3), 5)
    assert_eq_int(math_sub(10, 3), 7)
    assert_eq_int(math_mul(4, 5), 20)
    assert_eq_int(math_div(15, 3), 5)
    assert_eq_int(math_abs(-5), 5)
    
    // Test string operations
    assert_eq_string(string_concat("hello", " world"), "hello world")
    assert_true(string_equal("test", "test"))
    assert_false(string_equal("test", "different"))
    
    // Test boolean operations
    assert_true(based)
    assert_false(cap)
    assert_true(5 > 3)
    assert_false(2 > 5)
    
    print_test_summary()
}

test_basic_functionality()
