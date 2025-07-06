fr fr Basic Assertions Test
fr fr Tests the fundamental assertion functions in the testz framework

yeet "testz"

fr fr ================================
fr fr Integer Assertion Tests
fr fr ================================

slay test_assert_eq_integers() {
    testz.test_start("test_assert_eq_integers")
    
    sus a normie = 42
    sus b normie = 42
    sus c normie = 24
    
    testz.assert_eq(a, b)
    testz.assert_ne(a, c)
    testz.assert_greater_than(a, c)
    testz.assert_less_than(c, a)
    testz.assert_in_range(a, 40, 50)
}

slay test_assert_eq_zero() {
    testz.test_start("test_assert_eq_zero")
    
    sus zero normie = 0
    sus also_zero normie = 1 - 1
    
    testz.assert_eq(zero, also_zero)
    testz.assert_eq(zero, 0)
}

slay test_assert_negative_numbers() {
    testz.test_start("test_assert_negative_numbers")
    
    sus neg normie = -42
    sus pos normie = 42
    
    testz.assert_ne(neg, pos)
    testz.assert_less_than(neg, 0)
    testz.assert_greater_than(pos, 0)
}

fr fr ================================
fr fr Float Assertion Tests
fr fr ================================

slay test_assert_eq_floats() {
    testz.test_start("test_assert_eq_floats")
    
    sus pi meal = 3.14159
    sus also_pi meal = 3.14159
    sus not_pi meal = 2.71828
    
    testz.assert_eq_float(pi, also_pi)
    testz.assert_eq_float(pi, 3.14159)
}

slay test_assert_float_precision() {
    testz.test_start("test_assert_float_precision")
    
    sus a meal = 1.0
    sus b meal = 1.0001
    
    fr fr This should pass because difference is > 0.0001
    testz.assert_eq_float(a, b)
}

fr fr ================================
fr fr String Assertion Tests
fr fr ================================

slay test_assert_eq_strings() {
    testz.test_start("test_assert_eq_strings")
    
    sus hello tea = "hello"
    sus world tea = "world"
    sus hello_again tea = "hello"
    
    testz.assert_eq_string(hello, hello_again)
    testz.assert_ne_string(hello, world)
    testz.assert_eq_string("test", "test")
}

slay test_assert_empty_strings() {
    testz.test_start("test_assert_empty_strings")
    
    sus empty tea = ""
    sus also_empty tea = ""
    sus not_empty tea = "content"
    
    testz.assert_eq_string(empty, also_empty)
    testz.assert_ne_string(empty, not_empty)
}

slay test_assert_string_operations() {
    testz.test_start("test_assert_string_operations")
    
    sus text tea = "hello world"
    sus prefix tea = "hello"
    sus suffix tea = "world"
    sus substring tea = "lo wo"
    
    testz.assert_string_starts_with(text, prefix)
    testz.assert_string_ends_with(text, suffix)
    testz.assert_string_contains(text, substring)
}

fr fr ================================
fr fr Boolean Assertion Tests
fr fr ================================

slay test_assert_booleans() {
    testz.test_start("test_assert_booleans")
    
    sus true_val lit = based
    sus false_val lit = cap
    sus condition lit = 5 > 3
    sus false_condition lit = 5 < 3
    
    testz.assert_true(true_val)
    testz.assert_false(false_val)
    testz.assert_true(condition)
    testz.assert_false(false_condition)
    testz.assert_eq_bool(true_val, based)
    testz.assert_eq_bool(false_val, cap)
}

slay test_assert_boolean_logic() {
    testz.test_start("test_assert_boolean_logic")
    
    sus a lit = based
    sus b lit = cap
    
    testz.assert_true(a && !b)
    testz.assert_true(a || b)
    testz.assert_false(a && b)
    testz.assert_false(!a && !b)
}

fr fr ================================
fr fr Array Assertion Tests
fr fr ================================

slay test_assert_arrays() {
    testz.test_start("test_assert_arrays")
    
    sus arr1 [normie] = [1, 2, 3, 4, 5]
    sus arr2 [normie] = [1, 2, 3, 4, 5]
    sus arr3 [normie] = [5, 4, 3, 2, 1]
    
    testz.assert_array_eq(arr1, arr2)
    testz.assert_array_contains(arr1, 3)
    testz.assert_array_not_contains(arr1, 42)
}

slay test_assert_empty_arrays() {
    testz.test_start("test_assert_empty_arrays")
    
    sus empty1 [normie] = []
    sus empty2 [normie] = []
    sus not_empty [normie] = [1]
    
    testz.assert_array_eq(empty1, empty2)
    testz.assert_array_not_contains(empty1, 1)
}

fr fr ================================
fr fr Nil Assertion Tests
fr fr ================================

slay test_assert_nil() {
    testz.test_start("test_assert_nil")
    
    sus nil_val any = cringe
    sus not_nil_val any = 42
    
    testz.assert_nil(nil_val)
    testz.assert_not_nil(not_nil_val)
}

fr fr ================================
fr fr Test Runner
fr fr ================================

slay main() {
    vibez.spill("Running Basic Assertions Test Suite")
    vibez.spill("===================================")
    
    fr fr Run all test functions
    test_assert_eq_integers()
    test_assert_eq_zero()
    test_assert_negative_numbers()
    test_assert_eq_floats()
    test_assert_float_precision()
    test_assert_eq_strings()
    test_assert_empty_strings()
    test_assert_string_operations()
    test_assert_booleans()
    test_assert_boolean_logic()
    test_assert_arrays()
    test_assert_empty_arrays()
    test_assert_nil()
    
    fr fr Print summary
    testz.print_test_summary()
    
    fr fr Return appropriate exit code
    lowkey testz.test_failed > 0 {
        yolo 1
    } highkey {
        yolo 0
    }
}
