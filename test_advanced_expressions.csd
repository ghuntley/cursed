yeet "testz"

slay test_advanced_expressions() {
    test_start("Advanced Expression Tests")
    
    // Test direct integer/float/string expressions
    sus x = 100
    sus y = 2.5
    sus s = "test"
    
    assert_eq_int(x, 100)
    assert_eq_string(s, "test")
    
    // Test increment/decrement expressions
    x++
    assert_eq_int(x, 101)
    
    x--
    assert_eq_int(x, 100)
    
    // Test array expressions
    sus arr = [10, 20, 30]
    sus first = arr[0]
    assert_eq_int(first, 10)
    
    // Test slice access
    sus slice = arr[1:3]
    sus second = slice[0]
    assert_eq_int(second, 20)
    
    // Test character expressions
    sus ch = 'Z'
    
    print_test_summary()
}
