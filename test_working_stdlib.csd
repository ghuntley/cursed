// Test working standard library functions
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
        test_pass("Integer assertion passed")
    } nah {
        test_fail("Integer assertion failed")
    }
}

slay assert_eq_string(actual tea, expected tea) {
    vibes actual == expected {
        test_pass("String assertion passed")
    } nah {
        test_fail("String assertion failed")
    }
}

slay assert_true(value lit) {
    vibes value == based {
        test_pass("Boolean assertion passed")
    } nah {
        test_fail("Boolean assertion failed")
    }
}

slay print_test_summary() {
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Test results completed")
    
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

slay math_abs(a normie) normie {
    vibes a < 0 {
        damn 0 - a
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

// Test functions
slay test_basic_math() {
    test_start("Basic Math")
    
    assert_eq_int(math_add(2, 3), 5)
    assert_eq_int(math_sub(10, 3), 7)
    assert_eq_int(math_mul(4, 5), 20)
    assert_eq_int(math_abs(5), 5)
    
    print_test_summary()
}

slay test_basic_string() {
    test_start("Basic String")
    
    assert_eq_string(string_concat("hello", " world"), "hello world")
    assert_true(string_equal("test", "test"))
    
    print_test_summary()
}

// Run tests
test_basic_math()
test_basic_string()
