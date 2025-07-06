fr fr Demo program showing how to use the CURSED testing framework

fr fr Test simple math functions
slay add_numbers(a normie, b normie) normie {
    damn a + b
}

slay multiply_numbers(a normie, b normie) normie {
    damn a * b
}

slay is_even(n normie) lit {
    damn n % 2 == 0
}

fr fr Test functions using the framework
slay test_add_numbers() {
    test_start("Addition Tests")
    
    assert_eq_int(add_numbers(2, 3), 5)
    assert_eq_int(add_numbers(10, 5), 15)
    assert_eq_int(add_numbers(0, 0), 0)
    assert_eq_int(add_numbers(-5, 5), 0)
}

slay test_multiply_numbers() {
    test_start("Multiplication Tests")
    
    assert_eq_int(multiply_numbers(2, 3), 6)
    assert_eq_int(multiply_numbers(5, 4), 20)
    assert_eq_int(multiply_numbers(0, 10), 0)
    assert_eq_int(multiply_numbers(-2, 3), -6)
}

slay test_is_even() {
    test_start("Even Number Tests")
    
    assert_true(is_even(2))
    assert_true(is_even(0))
    assert_false(is_even(1))
    assert_false(is_even(3))
}

slay test_strings() {
    test_start("String Tests")
    
    sus greeting tea = "Hello"
    sus name tea = "World"
    
    assert_eq_string(greeting, "Hello")
    assert_eq_string(name, "World")
}

fr fr Run all tests
test_add_numbers()
test_multiply_numbers()
test_is_even()
test_strings()

print_test_summary()
