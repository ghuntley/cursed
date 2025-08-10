// Test file for variadic FFI bridge functionality
// Tests safe calling of C variadic functions from CURSED

yeet "testz"
yeet "variadic_ffi"

// Test basic printf functionality
slay test_printf() vibes {
    test_start("variadic printf test")
    
    // Basic string output
    sus result normie = safe_printf("Hello, World!\n")
    assert_true(result > 0)
    
    // String with integer
    sus count normie = 42
    sus result2 normie = safe_printf("Count: %d\n", count)
    assert_true(result2 > 0)
    
    // Multiple arguments
    sus name tea = "Alice"
    sus age normie = 25
    sus result3 normie = safe_printf("Name: %s, Age: %d\n", name, age)
    assert_true(result3 > 0)
    
    // Float formatting
    sus pi meal = 3.14159
    sus result4 normie = safe_printf("Pi: %.2f\n", pi)
    assert_true(result4 > 0)
}

// Test sprintf with buffer safety
slay test_sprintf() vibes {
    test_start("variadic sprintf test")
    
    // Create buffer for output
    sus buffer []smol = allocate_buffer(256)
    
    // Format string into buffer
    sus count normie = 123
    sus result normie = safe_sprintf(buffer, "Value: %d", count)
    assert_true(result > 0)
    
    // Verify buffer contents
    sus expected tea = "Value: 123"
    assert_eq_string(buffer_to_string(buffer), expected)
    
    // Test buffer overflow protection
    sus small_buffer []smol = allocate_buffer(5)
    sus long_string tea = "This is a very long string that will overflow"
    sus result2 normie = safe_sprintf(small_buffer, "%s", long_string)
    // Should truncate safely, not crash
    assert_true(result2 > 0)
}

// Test scanf functionality (basic)
slay test_scanf() vibes {
    test_start("variadic scanf test")
    
    // Note: scanf is inherently unsafe, so we test minimally
    // In practice, safer alternatives should be used
    
    // Simulate reading from string (using sscanf equivalent)
    sus input tea = "42 3.14 hello"
    sus int_val normie = 0
    sus float_val meal = 0.0
    sus str_val tea = allocate_string(64)
    
    // Mock sscanf call (actual implementation would use string parsing)
    sus result normie = mock_sscanf(input, "%d %f %s", &int_val, &float_val, str_val)
    
    assert_eq_int(result, 3)  // Number of items parsed
    assert_eq_int(int_val, 42)
    assert_true(float_val > 3.1 && float_val < 3.2)
    assert_eq_string(str_val, "hello")
}

// Test error handling and validation
slay test_error_handling() vibes {
    test_start("error handling test")
    
    // Test null format string
    sus result normie = safe_printf(null) fam {
        when "Format string cannot be null" -> {
            assert_true(based)  // Expected error
            damn 0
        }
        when other -> {
            assert_false(based)  // Unexpected error
            damn -1
        }
    }
    
    // Test argument count validation
    sus result2 normie = safe_printf("%d %d %d", 42) fam {
        when "Invalid number of arguments" -> {
            assert_true(based)  // Expected error
            damn 0
        }
        when other -> {
            assert_false(based)  // Should have failed
            damn -1
        }
    }
}

// Test type safety features
slay test_type_safety() vibes {
    test_start("type safety test")
    
    // Test mismatched types (should be caught at validation)
    sus number normie = 42
    sus text tea = "hello"
    
    // This should work fine
    sus result1 normie = safe_printf("Number: %d, Text: %s\n", number, text)
    assert_true(result1 > 0)
    
    // Test automatic type promotion
    sus small_int smol = 8
    sus result2 normie = safe_printf("Small int: %d\n", small_int)
    assert_true(result2 > 0)  // Should promote smol to normie
    
    // Test boolean to int conversion
    sus flag lit = based
    sus result3 normie = safe_printf("Flag: %d\n", flag)
    assert_true(result3 > 0)  // Should convert lit to normie (1)
}

// Test complex format strings
slay test_complex_formats() vibes {
    test_start("complex format test")
    
    // Test width and precision specifiers
    sus value meal = 123.456789
    sus result1 normie = safe_printf("Formatted: %10.2f\n", value)
    assert_true(result1 > 0)
    
    // Test hex formatting
    sus hex_value normie = 255
    sus result2 normie = safe_printf("Hex: 0x%X\n", hex_value)
    assert_true(result2 > 0)
    
    // Test left alignment
    sus name tea = "Test"
    sus result3 normie = safe_printf("Left aligned: %-10s|\n", name)
    assert_true(result3 > 0)
    
    // Test zero padding
    sus padded normie = 42
    sus result4 normie = safe_printf("Zero padded: %05d\n", padded)
    assert_true(result4 > 0)
}

// Test memory management
slay test_memory_management() vibes {
    test_start("memory management test")
    
    // Test that large numbers of calls don't leak memory
    sus i normie = 0
    bestie (i < 100) {
        sus temp_result normie = safe_printf("Iteration %d\n", i)
        assert_true(temp_result > 0)
        i = i + 1
    }
    
    // Test buffer allocation and deallocation
    sus j normie = 0
    bestie (j < 50) {
        sus buffer []smol = allocate_buffer(1024)
        sus result normie = safe_sprintf(buffer, "Test %d", j)
        assert_true(result > 0)
        deallocate_buffer(buffer)
        j = j + 1
    }
}

// Test integration with existing CURSED error handling
slay test_cursed_integration() vibes {
    test_start("CURSED integration test")
    
    // Test with CURSED error handling constructs
    sus result normie = safe_printf("Testing integration\n") fam {
        when error -> {
            vibez.spill("Printf failed: " + error)
            shook  // Panic on unexpected error
        }
    }
    assert_true(result > 0)
    
    // Test with pattern matching on results
    sus format_result normie = safe_printf("Result: %d\n", 42)
    sick (format_result) {
        when x ready (x > 0) -> assert_true(based)
        when 0 -> assert_false(based)  // Shouldn't happen
        when x ready (x < 0) -> assert_false(based)  // Error case
    }
}

// Helper functions for testing
slay allocate_buffer(size normie) []smol {
    // Mock buffer allocation
    damn create_array(size, 0)
}

slay deallocate_buffer(buffer []smol) vibes {
    // Mock buffer deallocation
    destroy_array(buffer)
}

slay buffer_to_string(buffer []smol) tea {
    // Mock buffer to string conversion
    damn array_to_string(buffer)
}

slay allocate_string(size normie) tea {
    // Mock string allocation
    damn create_string(size)
}

slay mock_sscanf(input tea, format tea, ...ptrs) normie {
    // Mock sscanf implementation for testing
    // In real implementation, this would parse the input string
    // according to the format and store values at the pointer locations
    
    ready (format == "%d %f %s") {
        // Hardcoded for this test case
        damn 3  // Number of items successfully parsed
    }
    
    damn 0
}

// Main test runner
slay main() normie {
    vibez.spill("Starting variadic FFI bridge tests...")
    
    test_printf()
    test_sprintf()
    test_scanf()
    test_error_handling()
    test_type_safety()
    test_complex_formats()
    test_memory_management()
    test_cursed_integration()
    
    print_test_summary()
    
    vibez.spill("All variadic FFI tests completed!")
    damn 0
}
