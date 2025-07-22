yeet "testz"

fr fr Example of using testz framework for stdlib module development
fr fr This demonstrates best practices for testing stdlib modules

fr fr Mock a simple math module for demonstration
slay add_numbers(a normie, b normie) normie {
    damn a + b
}

slay multiply_numbers(a normie, b normie) normie {
    damn a * b
}

slay divide_numbers(a normie, b normie) normie {
    highkey b == 0 {
        damn -1 fr fr Error case
    } else {
        damn a / b
    }
}

slay factorial(n normie) normie {
    highkey n <= 1 {
        damn 1
    } else {
        damn n * factorial(n - 1)
    }
}

slay is_prime(n normie) lit {
    highkey n <= 1 {
        damn cap
    }
    highkey n == 2 {
        damn based
    }
    highkey n % 2 == 0 {
        damn cap
    }
    
    sus i normie = 3
    bestie i = 3; i * i <= n; i = i + 2 {
        highkey n % i == 0 {
            damn cap
        }
    }
    damn based
}

fr fr Test fixtures for complex test scenarios
slay setup_test_environment() {
    set_fixture_data("test_environment_initialized")
    vibez.spill("🔧 Setting up test environment")
}

slay teardown_test_environment() {
    vibez.spill("🧹 Cleaning up test environment")
}

fr fr Basic functionality tests
slay test_basic_arithmetic() {
    test_start("Basic Arithmetic Operations") fr fr Test addition
    assert_eq_int(add_numbers(2, 3), 5)
    assert_eq_int(add_numbers(0, 0), 0)
    assert_eq_int(add_numbers(-5, 3), -2) fr fr Test multiplication
    assert_eq_int(multiply_numbers(4, 3), 12)
    assert_eq_int(multiply_numbers(0, 5), 0)
    assert_eq_int(multiply_numbers(-2, 3), -6) fr fr Test division
    assert_eq_int(divide_numbers(10, 2), 5)
    assert_eq_int(divide_numbers(7, 3), 2) fr fr Integer division
    assert_eq_int(divide_numbers(5, 0), -1) fr fr Error case
    
    test_end()
}

fr fr Edge case testing
slay test_edge_cases() {
    test_start("Edge Cases") fr fr Test large numbers
    assert_eq_int(add_numbers(999999, 1), 1000000)
    assert_eq_int(multiply_numbers(1000, 1000), 1000000) fr fr Test negative numbers
    assert_eq_int(add_numbers(-10, -5), -15)
    assert_eq_int(multiply_numbers(-3, -4), 12) fr fr Test zero cases
    assert_eq_int(multiply_numbers(0, 999), 0)
    assert_eq_int(add_numbers(0, 0), 0)
    
    test_end()
}

fr fr Performance benchmarking
slay test_performance_benchmarks() {
    test_start("Performance Benchmarks")
    
    set_benchmark_mode(based)
    set_benchmark_iterations(1000) fr fr Benchmark addition
    benchmark_start("Addition Performance")
    sus i normie = 0
    bestie i = 0; i < 1000; i = i + 1 {
        benchmark_iteration_start()
        sus result normie = add_numbers(i, i + 1)
        benchmark_iteration_end()
    }
    benchmark_end() fr fr Benchmark multiplication
    benchmark_start("Multiplication Performance")
    sus j normie = 0
    bestie j = 0; j < 1000; j = j + 1 {
        benchmark_iteration_start()
        sus result normie = multiply_numbers(j, j + 1)
        benchmark_iteration_end()
    }
    benchmark_end()
    
    test_end()
}

fr fr Property-based testing for mathematical properties
slay test_mathematical_properties() {
    test_start("Mathematical Properties") fr fr Test commutative property of addition
    property_test_start("Addition Commutativity", 100)
    sus i normie = 0
    bestie i = 0; i < 100; i = i + 1 {
        property_test_iteration()
        
        sus a normie = random_int(1, 1000)
        sus b normie = random_int(1, 1000)
        
        sus result1 normie = add_numbers(a, b)
        sus result2 normie = add_numbers(b, a)
        
        highkey result1 != result2 {
            property_test_fail("Addition not commutative: " + tea(a) + " + " + tea(b) + " != " + tea(b) + " + " + tea(a))
        }
    }
    property_test_end() fr fr Test associative property of multiplication
    property_test_start("Multiplication Associativity", 50)
    sus k normie = 0
    bestie k = 0; k < 50; k = k + 1 {
        property_test_iteration()
        
        sus a normie = random_int(1, 10)
        sus b normie = random_int(1, 10)
        sus c normie = random_int(1, 10)
        
        sus result1 normie = multiply_numbers(multiply_numbers(a, b), c)
        sus result2 normie = multiply_numbers(a, multiply_numbers(b, c))
        
        highkey result1 != result2 {
            property_test_fail("Multiplication not associative")
        }
    }
    property_test_end()
    
    test_end()
}

fr fr Complex algorithm testing
slay test_complex_algorithms() {
    test_start("Complex Algorithms") fr fr Test factorial function
    assert_eq_int(factorial(0), 1)
    assert_eq_int(factorial(1), 1)
    assert_eq_int(factorial(5), 120)
    assert_eq_int(factorial(6), 720) fr fr Test prime checking
    assert_true(is_prime(2))
    assert_true(is_prime(3))
    assert_true(is_prime(5))
    assert_true(is_prime(7))
    assert_false(is_prime(1))
    assert_false(is_prime(4))
    assert_false(is_prime(6))
    assert_false(is_prime(8))
    assert_false(is_prime(9))
    
    test_end()
}

fr fr Error handling tests
slay test_error_handling() {
    test_start("Error Handling") fr fr Test division by zero
    sus result normie = divide_numbers(5, 0)
    assert_eq_int(result, -1) fr fr Our error sentinel value fr fr Test edge cases in prime checking
    assert_false(is_prime(0))
    assert_false(is_prime(-1))
    assert_false(is_prime(-5))
    
    test_end()
}

fr fr Integration tests
slay test_integration() {
    test_start("Integration Tests") fr fr Test combining operations
    sus step1 normie = add_numbers(5, 3) fr fr 8
    sus step2 normie = multiply_numbers(step1, 2) fr fr 16
    sus step3 normie = divide_numbers(step2, 4) fr fr 4
    
    assert_eq_int(step3, 4) fr fr Test mathematical sequence
    sus a normie = 1
    sus b normie = 1
    sus c normie = add_numbers(a, b) fr fr 2
    sus d normie = add_numbers(b, c) fr fr 3
    sus e normie = add_numbers(c, d) fr fr 5
    
    assert_eq_int(e, 5) fr fr Fibonacci-like sequence
    
    test_end()
}

fr fr Main test runner
slay run_example_stdlib_module_test() {
    vibez.spill("🧪 Example Stdlib Module Test Suite")
    vibez.spill("=" * 50) fr fr Initialize comprehensive test environment
    before_all_tests()
    set_verbose_mode(based)
    set_test_suite("Example Math Module Test Suite") fr fr Set up test fixtures
    set_setup_function("setup_test_environment")
    set_teardown_function("teardown_test_environment") fr fr Run all test categories
    test_basic_arithmetic()
    test_edge_cases()
    test_performance_benchmarks()
    test_mathematical_properties()
    test_complex_algorithms()
    test_error_handling()
    test_integration() fr fr Generate final report
    after_all_tests()
    
    vibez.spill("")
    vibez.spill("📊 Test Suite Summary")
    vibez.spill("This example demonstrates comprehensive stdlib module testing:")
    vibez.spill("✅ Unit tests for individual functions")
    vibez.spill("✅ Edge case validation")
    vibez.spill("✅ Performance benchmarking")
    vibez.spill("✅ Property-based testing")
    vibez.spill("✅ Complex algorithm verification")
    vibez.spill("✅ Error handling validation")
    vibez.spill("✅ Integration testing")
    vibez.spill("✅ Test fixtures and lifecycle management")
    
    highkey all_tests_passed() {
        vibez.spill("🎉 ALL TESTS PASSED - MODULE READY FOR PRODUCTION")
    } else {
        vibez.spill("❌ SOME TESTS FAILED - MODULE NEEDS FIXES")
    }
}

fr fr Execute the example test suite
run_example_stdlib_module_test()
