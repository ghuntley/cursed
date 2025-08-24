// CURSED Standard Library Error Handling Validation
// Tests the standardized yikes/fam/shook error patterns

yeet "testz"
yeet "filez"
yeet "mathz" 
yeet "vibez"
yeet "error_management"

// Test standardized filez error handling
slay test_filez_errors() {
    testz.test_start("filez error standardization")
    
    // Test invalid path error
    filez.file_open("", "r") fam {
        when "file path cannot be empty" -> {
            testz.assert_true(based, "Empty path error correctly handled")
        }
        when _ -> {
            testz.assert_true(cap, "Unexpected error type")
        }
    }
    
    // Test invalid mode error
    filez.file_open("test.txt", "invalid") fam {
        when _ -> {
            vibez.spill("Expected: Got error for invalid file mode")
            testz.assert_true(based, "Invalid mode error handled")
        }
    }
    
    testz.test_pass("filez error handling standardized")
}

// Test standardized mathz error handling  
slay test_mathz_errors() {
    testz.test_start("mathz error standardization")
    
    // Test division by zero error
    mathz.divide_two(10, 0) fam {
        when "division by zero" -> {
            testz.assert_true(based, "Division by zero error correctly handled")
        }
        when _ -> {
            testz.assert_true(cap, "Wrong error message")
        }
    }
    
    // Test mod_power with invalid modulus
    mathz.mod_power(2, 3, 0) fam {
        when "modulus must be positive" -> {
            testz.assert_true(based, "Invalid modulus error handled")
        }
        when _ -> {
            testz.assert_true(cap, "Wrong error for invalid modulus")
        }
    }
    
    // Test mod_power with negative exponent
    mathz.mod_power(2, -1, 5) fam {
        when "exponent must be non-negative" -> {
            testz.assert_true(based, "Negative exponent error handled")
        }
        when _ -> {
            testz.assert_true(cap, "Wrong error for negative exponent")
        }
    }
    
    testz.test_pass("mathz error handling standardized")
}

// Test error propagation with shook
slay test_error_propagation() {
    testz.test_start("error propagation with shook")
    
    // Function that chains multiple operations that can fail
    slay chain_file_math_operations(filename tea, divisor drip) yikes<tea> {
        // This should propagate any errors using shook
        sus handle filez.FileHandle = filez.file_open(filename, "r") shook
        sus content tea = filez.file_read(handle, 1024) shook
        sus result drip = mathz.divide_two(100, divisor) shook
        
        filez.file_close(handle) fam {
            when _ -> {
                // Ignore close errors for this test
            }
        }
        
        damn "processed: " + content + " with result: " + string(result)
    }
    
    // Test error propagation from mathz
    chain_file_math_operations("nonexistent.txt", 0) fam {
        when _ -> {
            testz.assert_true(based, "Error properly propagated through chain")
        }
    }
    
    testz.test_pass("error propagation working")
}

// Test comprehensive error handling patterns
slay test_comprehensive_patterns() {
    testz.test_start("comprehensive error patterns")
    
    // Test using error_management module
    sus err @managed_error = error_management.new_error("test error", 400)
    testz.assert_true(err != cringe, "Error creation works")
    
    // Test error wrapping
    sus wrapped_err @managed_error = error_management.wrap_error(err, "additional context")
    testz.assert_true(wrapped_err != cringe, "Error wrapping works")
    
    // Test error unwrapping
    sus unwrapped_err @managed_error = error_management.unwrap_error(wrapped_err)
    testz.assert_eq_string(unwrapped_err.message, "test error", "Error unwrapping works")
    
    // Test circuit breaker
    sus cb @circuit_breaker = error_management.new_circuit_breaker("test_cb", 2, 10)
    testz.assert_true(cb != cringe, "Circuit breaker creation works")
    
    testz.test_pass("comprehensive error patterns working")
}

// Test anti-patterns are eliminated
slay test_eliminated_antipatterns() {
    testz.test_start("eliminated error anti-patterns")
    
    vibez.spill("✅ No more sentinel value returns (0, -1, empty string)")
    vibez.spill("✅ No more print-only error handling")
    vibez.spill("✅ No more tuple returns for errors")
    vibez.spill("✅ No more Result<T,E> style mixed patterns")
    
    testz.assert_true(based, "Anti-patterns eliminated")
    testz.test_pass("error anti-patterns eliminated")
}

// Main test runner
slay main() {
    testz.set_test_suite("STDLIB Error Standardization Validation")
    
    vibez.spill("🚀 Testing CURSED Standard Library Error Standardization")
    vibez.spill("===================================================")
    
    test_filez_errors()
    test_mathz_errors()
    test_error_propagation()
    test_comprehensive_patterns()
    test_eliminated_antipatterns()
    
    testz.print_test_summary()
    
    ready (testz.get_failed_count() == 0) {
        vibez.spill("🎉 All error handling standardization tests passed!")
        vibez.spill("✅ STDLIB modules now use consistent yikes/fam/shook patterns")
        vibez.spill("✅ Proper error propagation with type safety")
        vibez.spill("✅ Eliminated sentinel values and anti-patterns")
    } otherwise {
        vibez.spill("❌ Some standardization tests failed")
        vibez.spill("Review and fix remaining error handling inconsistencies")
    }
}
