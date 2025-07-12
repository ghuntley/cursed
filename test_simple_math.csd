yeet "testz"

slay test_basic_math() {
    test_start("Basic Math Test")
    
    assert_eq_int(2 + 3, 5)
    assert_eq_int(10 - 4, 6)
    assert_eq_int(3 * 4, 12)
    assert_eq_int(15 / 3, 5)
    
    print_test_summary()
}

test_basic_math()
