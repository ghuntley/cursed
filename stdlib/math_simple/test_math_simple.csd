yeet "testz"
yeet "math_simple"

slay test_basic_arithmetic() {
    test_start("Basic Arithmetic")
    
    // Test integer arithmetic
    assert_eq_int(math_add(2, 3), 5)
    assert_eq_int(math_sub(10, 3), 7)
    assert_eq_int(math_mul(4, 5), 20)
    assert_eq_int(math_div(15, 3), 5)
    assert_eq_int(math_mod(10, 3), 1)
    
    // Test comparison functions
    assert_eq_int(math_max_int(5, 3), 5)
    assert_eq_int(math_min_int(5, 3), 3)
    assert_eq_int(math_abs_int(-5), 5)
    assert_eq_int(math_abs_int(5), 5)
    
    print_test_summary()
}

test_basic_arithmetic()
