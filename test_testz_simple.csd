yeet "testz"

slay test_basic_functionality() {
    test_start("Basic Test Framework")
    
    assert_eq_int(1 + 1, 2)
    assert_eq_string("hello", "hello")
    assert_true(based)
    assert_false(cap)
    
    vibez.spill("Test framework is working!")
}

test_basic_functionality()
print_test_summary()
