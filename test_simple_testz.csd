yeet "testz"

slay test_basic_functionality() {
    test_start("Basic TestZ Framework Test")
    
    # Test basic assertions
    assert_true(based)
    assert_false(cap)
    assert_eq_int(42, 42)
    assert_eq_string("hello", "hello")
    
    # Test range assertions  
    assert_range_int(50, 1, 100)
    
    # Test string operations
    assert_contains("hello world", "world")
    assert_starts_with("hello world", "hello")
    assert_ends_with("hello world", "world")
    
    test_end()
}

slay test_random_generators() {
    test_start("Random Generator Test")
    
    # Test random number generation
    sus rand_num normie = random_int(1, 10)
    assert_range_int(rand_num, 1, 10)
    
    # Test random string generation
    sus rand_str tea = random_string(5)
    assert_eq_int(stringz.Length(rand_str), 5)
    
    test_end()
}

slay simple_testz_demo() {
    vibez.spill("🧪 Simple TestZ Framework Demo")
    vibez.spill("=" * 40)
    
    # Initialize test environment
    before_all_tests()
    set_verbose_mode(based)
    set_test_suite("Simple TestZ Demo")
    
    # Run basic tests
    test_basic_functionality()
    test_random_generators()
    
    # Generate summary
    after_all_tests()
    
    vibez.spill("")
    vibez.spill("🎯 TestZ Framework Demo Complete")
    vibez.spill("✅ Advanced assertion functions work")
    vibez.spill("✅ Random data generation works")
    vibez.spill("✅ Test reporting works")
    
    highkey all_tests_passed() {
        vibez.spill("🎉 ALL TESTS PASSED!")
    } else {
        vibez.spill("❌ SOME TESTS FAILED")
    }
}

# Run the demo
simple_testz_demo()
