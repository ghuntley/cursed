fr fr Test Runner Demo
fr fr Demonstrates the complete testing framework functionality

yeet "testz"

fr fr ================================
fr fr Demo Test Suite
fr fr ================================

slay demo_simple_test() {
    testz.test_start("demo_simple_test")
    
    sus result normie = 2 + 2
    testz.assert_eq(result, 4)
    
    sus message tea = "Hello, CURSED!"
    testz.assert_eq_string(message, "Hello, CURSED!")
    
    sus flag lit = based
    testz.assert_true(flag)
}

slay demo_arithmetic_test() {
    testz.test_start("demo_arithmetic_test")
    
    sus a normie = 10
    sus b normie = 5
    
    testz.assert_eq(a + b, 15)
    testz.assert_eq(a - b, 5)
    testz.assert_eq(a * b, 50)
    testz.assert_eq(a / b, 2)
    testz.assert_eq(a % b, 0)
}

slay demo_string_test() {
    testz.test_start("demo_string_test")
    
    sus greeting tea = "Hello"
    sus target tea = "World"
    sus full_greeting tea = greeting + " " + target
    
    testz.assert_eq_string(full_greeting, "Hello World")
    testz.assert_eq(full_greeting.length, 11)
    
    testz.assert_string_starts_with(full_greeting, "Hello")
    testz.assert_string_ends_with(full_greeting, "World")
    testz.assert_string_contains(full_greeting, " ")
}

slay demo_array_test() {
    testz.test_start("demo_array_test")
    
    sus numbers [normie] = [1, 2, 3, 4, 5]
    
    testz.assert_eq(numbers.length, 5)
    testz.assert_eq(numbers[0], 1)
    testz.assert_eq(numbers[4], 5)
    
    testz.assert_array_contains(numbers, 3)
    testz.assert_array_not_contains(numbers, 10)
    
    sus expected [normie] = [1, 2, 3, 4, 5]
    testz.assert_array_eq(numbers, expected)
}

slay demo_boolean_test() {
    testz.test_start("demo_boolean_test")
    
    sus condition1 lit = 5 > 3
    sus condition2 lit = 10 < 8
    
    testz.assert_true(condition1)
    testz.assert_false(condition2)
    
    testz.assert_eq_bool(condition1, based)
    testz.assert_eq_bool(condition2, cap)
}

slay demo_type_test() {
    testz.test_start("demo_type_test")
    
    sus number normie = 42
    sus float_val meal = 3.14
    sus char_val sip = 'A'
    
    fr fr Type assertions
    sus converted_float meal = number.(meal)
    sus converted_int normie = char_val.(normie)
    
    testz.assert_eq_float(converted_float, 42.0)
    testz.assert_eq(converted_int, 65)
}

slay demo_failing_test() {
    testz.test_start("demo_failing_test")
    
    fr fr This test is designed to fail to demonstrate failure reporting
    sus wrong_result normie = 2 + 2
    testz.assert_eq(wrong_result, 5)  fr fr This will fail
    
    sus wrong_string tea = "Hello"
    testz.assert_eq_string(wrong_string, "Goodbye")  fr fr This will also fail
}

slay demo_range_test() {
    testz.test_start("demo_range_test")
    
    sus value normie = 50
    
    testz.assert_in_range(value, 40, 60)
    testz.assert_greater_than(value, 30)
    testz.assert_less_than(value, 70)
}

slay demo_edge_cases_test() {
    testz.test_start("demo_edge_cases_test")
    
    fr fr Test with zero
    sus zero normie = 0
    testz.assert_eq(zero, 0)
    testz.assert_false(zero != 0)
    
    fr fr Test with negative numbers
    sus negative normie = -10
    testz.assert_less_than(negative, 0)
    testz.assert_greater_than(negative, -20)
    
    fr fr Test with empty string
    sus empty_string tea = ""
    testz.assert_eq_string(empty_string, "")
    testz.assert_eq(empty_string.length, 0)
    
    fr fr Test with empty array
    sus empty_array [normie] = []
    testz.assert_eq(empty_array.length, 0)
}

fr fr ================================
fr fr Benchmark Demo
fr fr ================================

slay demo_benchmark_test() {
    testz.test_start("demo_benchmark_test")
    
    sus start_time normie = testz.benchmark_start()
    
    fr fr Simulate some work
    sus result normie = 0
    sus i normie = 0
    periodt i < 1000 {
        result = result + i
        i = i + 1
    }
    
    testz.benchmark_end(start_time)
    testz.assert_eq(result, 499500)  fr fr Sum of 0 to 999
}

fr fr ================================
fr fr Test Discovery Demo
fr fr ================================

slay demo_test_discovery() {
    vibez.spill("=== Test Discovery Demo ===")
    
    fr fr Simulate test discovery
    testz.discover_tests_in_directory("tests/testz/")
    
    fr fr Show discovered tests
    vibez.spill("Discovered " + testz.discovered_tests.length + " tests")
    
    sus i normie = 0
    periodt i < testz.discovered_tests.length {
        sus test_func testz.TestFunction = testz.discovered_tests[i]
        vibez.spill("  - " + test_func.name + " in " + test_func.file)
        i = i + 1
    }
}

fr fr ================================
fr fr Test Report Demo
fr fr ================================

slay demo_test_reporting() {
    vibez.spill("=== Test Report Demo ===")
    
    fr fr Run a few tests first
    demo_simple_test()
    demo_arithmetic_test()
    demo_failing_test()
    
    vibez.spill("")
    vibez.spill("--- JSON Report ---")
    testz.generate_json_report()
    
    vibez.spill("")
    vibez.spill("--- XML Report ---")
    testz.generate_xml_report()
    
    vibez.spill("")
    vibez.spill("--- HTML Report ---")
    testz.generate_html_report()
}

fr fr ================================
fr fr Configuration Demo
fr fr ================================

slay demo_test_configuration() {
    vibez.spill("=== Test Configuration Demo ===")
    
    sus config testz.TestConfig = testz.create_default_config()
    
    fr fr Customize configuration
    config.verbose = based
    config.fail_fast = cap
    config.test_dir = "tests/testz/"
    config.pattern = "demo_*"
    
    vibez.spill("Configuration:")
    vibez.spill("  Verbose: " + config.verbose)
    vibez.spill("  Fail fast: " + config.fail_fast)
    vibez.spill("  Test directory: " + config.test_dir)
    vibez.spill("  Pattern: " + config.pattern)
    vibez.spill("  Timeout: " + config.timeout + "ms")
    vibez.spill("  Parallel: " + config.parallel)
}

fr fr ================================
fr fr Main Demo Runner
fr fr ================================

slay main() {
    vibez.spill("CURSED Testing Framework Demo")
    vibez.spill("=============================")
    
    fr fr Reset test state
    testz.reset_test_state()
    
    fr fr Run basic tests
    vibez.spill("")
    vibez.spill("=== Running Basic Tests ===")
    demo_simple_test()
    demo_arithmetic_test()
    demo_string_test()
    demo_array_test()
    demo_boolean_test()
    demo_type_test()
    demo_range_test()
    demo_edge_cases_test()
    
    fr fr Run benchmark
    vibez.spill("")
    vibez.spill("=== Running Benchmark ===")
    demo_benchmark_test()
    
    fr fr Show test discovery
    vibez.spill("")
    demo_test_discovery()
    
    fr fr Show configuration
    vibez.spill("")
    demo_test_configuration()
    
    fr fr Run a failing test to demonstrate failure reporting
    vibez.spill("")
    vibez.spill("=== Testing Failure Reporting ===")
    demo_failing_test()
    
    fr fr Generate various reports
    vibez.spill("")
    demo_test_reporting()
    
    fr fr Print final summary
    vibez.spill("")
    vibez.spill("=== Final Test Summary ===")
    testz.print_test_summary()
    
    fr fr Return appropriate exit code
    lowkey testz.test_failed > 0 {
        vibez.spill("")
        vibez.spill("Some tests failed. Check the output above.")
        yolo 1
    } highkey {
        vibez.spill("")
        vibez.spill("All tests passed!")
        yolo 0
    }
}
