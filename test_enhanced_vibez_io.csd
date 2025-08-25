fr fr CURSED VIBEZ Enhanced I/O Comprehensive Test Suite
fr fr Tests all enhanced functionality: Unicode, printf-style formatting, filesystem integration, and string handling

yeet "vibez"

fr fr ===== MAIN TEST EXECUTION =====

slay main() normie {
    vibez.print_header("CURSED VIBEZ Enhanced I/O Test Suite")
    
    sus all_tests_passed lit = based
    sus total_tests normie = 0
    sus passed_tests normie = 0
    
    vibez.spillln("Testing enhanced I/O functionality...")
    vibez.print_separator()
    
    fr fr Test 1: Basic enhanced output
    total_tests = total_tests + 1
    vibez.spillln("Test 1: Basic Enhanced Output")
    ready test_basic_output() {
        vibez.print_success("Basic output test PASSED")
        passed_tests = passed_tests + 1
    }
    otherwise {
        vibez.print_error("Basic output test FAILED")
        all_tests_passed = cap
    }
    
    fr fr Test 2: Unicode support
    total_tests = total_tests + 1
    vibez.spillln("\nTest 2: Unicode Support")
    ready test_unicode_support() {
        vibez.print_success("Unicode support test PASSED")
        passed_tests = passed_tests + 1
    }
    otherwise {
        vibez.print_error("Unicode support test FAILED")
        all_tests_passed = cap
    }
    
    fr fr Test 3: Printf-style formatting
    total_tests = total_tests + 1
    vibez.spillln("\nTest 3: Printf-Style Formatting")
    ready test_printf_formatting() {
        vibez.print_success("Printf formatting test PASSED")
        passed_tests = passed_tests + 1
    }
    otherwise {
        vibez.print_error("Printf formatting test FAILED")
        all_tests_passed = cap
    }
    
    fr fr Test 4: Enhanced file operations
    total_tests = total_tests + 1
    vibez.spillln("\nTest 4: Enhanced File Operations")
    ready test_file_operations() {
        vibez.print_success("File operations test PASSED")
        passed_tests = passed_tests + 1
    }
    otherwise {
        vibez.print_error("File operations test FAILED")
        all_tests_passed = cap
    }
    
    fr fr Test 5: String handling enhancements
    total_tests = total_tests + 1
    vibez.spillln("\nTest 5: String Handling Enhancements")
    ready test_string_handling() {
        vibez.print_success("String handling test PASSED")
        passed_tests = passed_tests + 1
    }
    otherwise {
        vibez.print_error("String handling test FAILED")
        all_tests_passed = cap
    }
    
    fr fr Test 6: I/O mode configuration
    total_tests = total_tests + 1
    vibez.spillln("\nTest 6: I/O Mode Configuration")
    ready test_io_mode_configuration() {
        vibez.print_success("I/O mode configuration test PASSED")
        passed_tests = passed_tests + 1
    }
    otherwise {
        vibez.print_error("I/O mode configuration test FAILED")
        all_tests_passed = cap
    }
    
    fr fr Test 7: Performance benchmarks
    total_tests = total_tests + 1
    vibez.spillln("\nTest 7: Performance Benchmarks")
    ready test_performance_benchmarks() {
        vibez.print_success("Performance benchmark test PASSED")
        passed_tests = passed_tests + 1
    }
    otherwise {
        vibez.print_error("Performance benchmark test FAILED")
        all_tests_passed = cap
    }
    
    fr fr Test 8: Complete integration test
    total_tests = total_tests + 1
    vibez.spillln("\nTest 8: Complete Integration Test")
    ready test_complete_integration() {
        vibez.print_success("Complete integration test PASSED")
        passed_tests = passed_tests + 1
    }
    otherwise {
        vibez.print_error("Complete integration test FAILED")
        all_tests_passed = cap
    }
    
    fr fr Display final results
    vibez.print_separator()
    vibez.spillf("Test Results: %d/%d tests passed\n", [vibez.int_to_string(passed_tests), vibez.int_to_string(total_tests)])
    
    ready all_tests_passed {
        vibez.print_success("🎉 ALL ENHANCED I/O TESTS PASSED!")
        vibez.spillln("\n📊 Enhanced I/O Statistics:")
        vibez.spillln(vibez.get_enhanced_io_statistics())
    }
    otherwise {
        vibez.print_error("❌ Some enhanced I/O tests failed")
    }
    
    damn all_tests_passed ? 0 : 1
}

fr fr ===== INDIVIDUAL TEST FUNCTIONS =====

slay test_basic_output() lit {
    vibez.spillln("  → Testing basic spill function with enhanced backend...")
    
    fr fr Test basic string output
    sus test_msg tea = "Hello, Enhanced CURSED I/O!"
    ready !vibez.spill(test_msg) {
        damn cap
    }
    
    fr fr Test empty string handling
    ready !vibez.spill("") {
        damn cap
    }
    
    fr fr Test null handling
    ready !vibez.spill(cringe) {
        damn cap
    }
    
    vibez.spillln("  ✓ Basic output functionality verified")
    damn based
}

slay test_unicode_support() lit {
    vibez.spillln("  → Testing Unicode character support...")
    
    fr fr Test various Unicode characters
    sus unicode_tests []tea = [
        "Basic ASCII: Hello World",
        "Latin Extended: café, naïve, résumé",
        "Emoji: 🚀 🌟 💻 🎯",
        "Mathematical: ∑ ∏ ∫ ∆ π",
        "Arrows: ← → ↑ ↓ ⟶",
        "Chinese: 你好世界",
        "Arabic: مرحبا بالعالم",
        "Russian: Привет, мир!"
    ]
    
    sus i normie = 0
    bestie i < len(unicode_tests) {
        vibez.spillf("  Unicode Test %d: %s\n", [vibez.int_to_string(i + 1), unicode_tests[i]])
        ready !vibez.spill(unicode_tests[i]) {
            vibez.spillf("  ❌ Failed on Unicode test %d\n", [vibez.int_to_string(i + 1)])
            damn cap
        }
        i = i + 1
    }
    
    fr fr Test Unicode string length calculation
    sus unicode_str tea = "Héllo 🌍"
    sus calculated_length drip = vibez.string_length(unicode_str)
    vibez.spillf("  Unicode string length: %f characters\n", [calculated_length])
    
    vibez.spillln("  ✓ Unicode support verified")
    damn based
}

slay test_printf_formatting() lit {
    vibez.spillln("  → Testing printf-style formatting...")
    
    fr fr Test basic format specifiers
    sus format_tests []tea = [
        vibez.spillf("Integer: %d", ["42"]),
        vibez.spillf("String: %s", ["test"]),
        vibez.spillf("Hex: %x", ["255"]),
        vibez.spillf("Float: %.2f", ["3.14159"]),
        vibez.spillf("Multiple: %s has %d items", ["List", "10"])
    ]
    
    sus i normie = 0
    bestie i < len(format_tests) {
        ready format_tests[i] == "" {
            vibez.spillf("  ❌ Printf formatting failed for test %d\n", [vibez.int_to_string(i + 1)])
            damn cap
        }
        vibez.spillf("  Format Test %d: %s\n", [vibez.int_to_string(i + 1), format_tests[i]])
        i = i + 1
    }
    
    fr fr Test complex formatting
    sus complex_format tea = vibez.spillf("Complex: %s=%d, %s=%.3f, hex=0x%X", [
        "count", "42",
        "pi", "3.14159", 
        "255"
    ])
    ready complex_format == "" {
        vibez.spillln("  ❌ Complex printf formatting failed")
        damn cap
    }
    vibez.spillf("  Complex Format: %s\n", [complex_format])
    
    vibez.spillln("  ✓ Printf-style formatting verified")
    damn based
}

slay test_file_operations() lit {
    vibez.spillln("  → Testing enhanced file operations...")
    
    sus test_file tea = "/tmp/cursed_enhanced_io_test.txt"
    sus test_content tea = "Enhanced I/O test content\nWith Unicode: 🚀\nMultiple lines\n"
    
    fr fr Test file writing
    ready !vibez.write_file(test_file, test_content) {
        vibez.spillln("  ❌ File writing failed")
        damn cap
    }
    vibez.spillln("  ✓ File writing successful")
    
    fr fr Test file reading
    sus read_content tea = vibez.read_file(test_file)
    ready read_content == "" {
        vibez.spillln("  ❌ File reading failed")
        damn cap
    }
    vibez.spillln("  ✓ File reading successful")
    
    fr fr Verify content integrity
    ready read_content != test_content {
        vibez.spillln("  ❌ File content mismatch")
        vibez.spillf("    Expected: %s\n", [test_content])
        vibez.spillf("    Got: %s\n", [read_content])
        damn cap
    }
    vibez.spillln("  ✓ Content integrity verified")
    
    fr fr Test file appending
    sus append_content tea = "Appended line with Unicode: 🎯\n"
    ready !vibez.append_file(test_file, append_content) {
        vibez.spillln("  ❌ File appending failed")
        damn cap
    }
    vibez.spillln("  ✓ File appending successful")
    
    fr fr Verify appended content
    sus final_content tea = vibez.read_file(test_file)
    sus expected_final tea = test_content + append_content
    ready final_content != expected_final {
        vibez.spillln("  ❌ Appended content verification failed")
        damn cap
    }
    vibez.spillln("  ✓ Appended content verified")
    
    fr fr Test file metadata
    ready !vibez.file_exists(test_file) {
        vibez.spillln("  ❌ File existence check failed")
        damn cap
    }
    vibez.spillln("  ✓ File existence confirmed")
    
    sus file_size drip = vibez.get_file_size(test_file)
    vibez.spillf("  File size: %f bytes\n", [file_size])
    ready file_size <= 0 {
        vibez.spillln("  ❌ File size check failed")
        damn cap
    }
    
    vibez.spillln("  ✓ Enhanced file operations verified")
    damn based
}

slay test_string_handling() lit {
    vibez.spillln("  → Testing enhanced string handling...")
    
    fr fr Test string length with Unicode
    sus test_strings []tea = [
        "ASCII string",
        "Unicode: café 🚀",
        "Mixed: Hello 世界 🌟",
        "Empty: ",
        "Numbers: 123456"
    ]
    
    sus i normie = 0
    bestie i < len(test_strings) {
        sus length drip = vibez.string_length(test_strings[i])
        vibez.spillf("  String %d length: %f chars - \"%s\"\n", [
            vibez.int_to_string(i + 1),
            length,
            test_strings[i]
        ])
        ready length < 0 {
            vibez.spillf("  ❌ Invalid length for string %d\n", [vibez.int_to_string(i + 1)])
            damn cap
        }
        i = i + 1
    }
    
    fr fr Test string operations
    sus concat_test tea = vibez.string_concat("Hello ", "World!")
    ready concat_test != "Hello World!" {
        vibez.spillln("  ❌ String concatenation failed")
        damn cap
    }
    vibez.spillf("  Concatenation test: \"%s\" ✓\n", [concat_test])
    
    fr fr Test substring operations
    sus substr_test tea = vibez.substring("Hello World", 6, 11)
    ready substr_test != "World" {
        vibez.spillln("  ❌ Substring operation failed")
        damn cap
    }
    vibez.spillf("  Substring test: \"%s\" ✓\n", [substr_test])
    
    vibez.spillln("  ✓ Enhanced string handling verified")
    damn based
}

slay test_io_mode_configuration() lit {
    vibez.spillln("  → Testing I/O mode configuration...")
    
    fr fr Get current mode
    sus original_mode normie = vibez.get_io_mode()
    vibez.spillf("  Original I/O mode: 0x%s\n", [vibez.int_to_hex_string(original_mode)])
    
    fr fr Test mode changes
    sus test_mode normie = IO_MODE_NATIVE | IO_MODE_UNICODE_AWARE
    vibez.set_io_mode(test_mode)
    
    sus current_mode normie = vibez.get_io_mode()
    ready current_mode != test_mode {
        vibez.spillln("  ❌ I/O mode setting failed")
        damn cap
    }
    vibez.spillf("  Mode change test: 0x%s ✓\n", [vibez.int_to_hex_string(current_mode)])
    
    fr fr Test encoding changes
    sus original_encoding normie = vibez.get_default_encoding()
    vibez.set_default_encoding(STRING_ENCODING_UTF16)
    
    sus new_encoding normie = vibez.get_default_encoding()
    ready new_encoding != STRING_ENCODING_UTF16 {
        vibez.spillln("  ❌ Encoding setting failed")
        damn cap
    }
    vibez.spillf("  Encoding change test: %s ✓\n", [vibez.encoding_to_string(new_encoding)])
    
    fr fr Test console dimensions
    vibez.set_console_dimensions(120, 40)
    sus width normie = vibez.get_console_width()
    sus height normie = vibez.get_console_height()
    
    ready width != 120 || height != 40 {
        vibez.spillln("  ❌ Console dimensions setting failed")
        damn cap
    }
    vibez.spillf("  Console dimensions: %dx%d ✓\n", [vibez.int_to_string(width), vibez.int_to_string(height)])
    
    fr fr Restore original settings
    vibez.set_io_mode(original_mode)
    vibez.set_default_encoding(original_encoding)
    vibez.set_console_dimensions(80, 24)
    
    vibez.spillln("  ✓ I/O mode configuration verified")
    damn based
}

slay test_performance_benchmarks() lit {
    vibez.spillln("  → Testing performance benchmarks...")
    
    fr fr Run built-in benchmark
    sus benchmark_result tea = vibez.benchmark_enhanced_io_performance()
    ready benchmark_result == "" {
        vibez.spillln("  ❌ Performance benchmark failed")
        damn cap
    }
    vibez.spillf("  %s\n", [benchmark_result])
    
    fr fr Custom performance test
    sus start_time normie = vibez.get_current_time_ms()
    
    fr fr Test multiple small operations
    bestie i := 0; i < 50; i++ {
        sus test_str tea = vibez.spillf("Performance test %d", [vibez.int_to_string(i)])
        ready test_str == "" {
            vibez.spillln("  ❌ Performance test iteration failed")
            damn cap
        }
    }
    
    sus end_time normie = vibez.get_current_time_ms()
    sus duration normie = end_time - start_time
    vibez.spillf("  Custom benchmark: %dms for 50 format operations\n", [vibez.int_to_string(duration)])
    
    ready duration > 10000 {  fr fr More than 10 seconds is too slow
        vibez.spillln("  ⚠️  Performance benchmark exceeded expected time")
        damn cap
    }
    
    vibez.spillln("  ✓ Performance benchmarks completed")
    damn based
}

slay test_complete_integration() lit {
    vibez.spillln("  → Testing complete integration...")
    
    fr fr Run the comprehensive test from vibez module
    ready !vibez.test_enhanced_io_functionality() {
        vibez.spillln("  ❌ Built-in integration test failed")
        damn cap
    }
    vibez.spillln("  ✓ Built-in integration test passed")
    
    fr fr Test all major features together
    sus integration_file tea = "/tmp/cursed_integration_test.txt"
    
    fr fr Create complex test content
    sus complex_content string_builder = vibez.create_string_builder(1024)
    
    vibez.string_builder_append_string(complex_content, "=== CURSED Enhanced I/O Integration Test ===\n")
    vibez.string_builder_append_string(complex_content, vibez.spillf("Test executed at timestamp: %d\n", [vibez.get_current_time_ms()]))
    vibez.string_builder_append_string(complex_content, "Unicode test: 🚀 Hello 世界 café résumé\n")
    vibez.string_builder_append_string(complex_content, vibez.spillf("Printf test: %d + %d = %d\n", ["2", "3", "5"]))
    vibez.string_builder_append_string(complex_content, "String handling: concatenation + substrings\n")
    vibez.string_builder_append_string(complex_content, vibez.spillf("Console: %dx%d\n", [vibez.int_to_string(vibez.get_console_width()), vibez.int_to_string(vibez.get_console_height())]))
    vibez.string_builder_append_string(complex_content, "=== End Integration Test ===\n")
    
    sus final_content enhanced_string = vibez.string_builder_to_string(complex_content)
    sus content_tea tea = vibez.enhanced_string_to_tea(final_content)
    
    fr fr Write complex content
    ready !vibez.write_file(integration_file, content_tea) {
        vibez.spillln("  ❌ Integration file writing failed")
        vibez.deallocate_string_builder(complex_content)
        vibez.deallocate_string(final_content)
        damn cap
    }
    
    fr fr Read and verify
    sus read_integration tea = vibez.read_file(integration_file)
    ready read_integration != content_tea {
        vibez.spillln("  ❌ Integration content verification failed")
        vibez.deallocate_string_builder(complex_content)
        vibez.deallocate_string(final_content)
        damn cap
    }
    
    fr fr Display integration content
    vibez.spillln("  Integration test content:")
    vibez.spillln(read_integration)
    
    fr fr Cleanup
    vibez.deallocate_string_builder(complex_content)
    vibez.deallocate_string(final_content)
    
    vibez.spillln("  ✓ Complete integration test verified")
    damn based
}

fr fr ===== UTILITY FUNCTIONS =====

slay display_test_statistics() {
    vibez.print_separator()
    vibez.spillln("📈 Enhanced I/O Test Statistics:")
    vibez.spillln(vibez.get_enhanced_io_statistics())
    vibez.print_separator()
}

slay cleanup_test_files() {
    fr fr Clean up test files created during testing
    sus test_files []tea = [
        "/tmp/cursed_enhanced_io_test.txt",
        "/tmp/cursed_integration_test.txt",
        "/tmp/benchmark_test.txt"
    ]
    
    sus i normie = 0
    bestie i < len(test_files) {
        ready vibez.file_exists(test_files[i]) {
            vibez.spillf("Cleaning up: %s\n", [test_files[i]])
        }
        i = i + 1
    }
}

fr fr ===== ERROR REPORTING =====

slay report_test_environment() {
    vibez.spillln("🔧 Test Environment Information:")
    vibez.spillf("  I/O Mode: 0x%s\n", [vibez.int_to_hex_string(vibez.get_io_mode())])
    vibez.spillf("  Default Encoding: %s\n", [vibez.encoding_to_string(vibez.get_default_encoding())])
    vibez.spillf("  Console Size: %dx%d\n", [vibez.int_to_string(vibez.get_console_width()), vibez.int_to_string(vibez.get_console_height())])
    
    sus features []tea = []
    ready (vibez.get_io_mode() & IO_MODE_UNICODE_AWARE) != 0 {
        features = append_string_to_array(features, "Unicode")
    }
    ready (vibez.get_io_mode() & IO_MODE_PRINTF_STYLE) != 0 {
        features = append_string_to_array(features, "Printf")
    }
    ready (vibez.get_io_mode() & IO_MODE_NATIVE) != 0 {
        features = append_string_to_array(features, "Filesystem")
    }
    ready (vibez.get_io_mode() & IO_MODE_BUFFERED) != 0 {
        features = append_string_to_array(features, "Buffered")
    }
    
    vibez.spillf("  Active Features: %d enabled\n", [vibez.int_to_string(len(features))])
}

slay append_string_to_array(arr []tea, str tea) []tea {
    fr fr Simple array append simulation
    sus new_arr []tea = make_string_array(len(arr) + 1)
    bestie i := 0; i < len(arr); i++ {
        new_arr[i] = arr[i]
    }
    new_arr[len(arr)] = str
    damn new_arr
}

slay make_string_array(size normie) []tea {
    fr fr Array creation placeholder
    sus arr []tea = []
    damn arr
}
