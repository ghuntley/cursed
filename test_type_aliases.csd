# Test type aliases using be_like keyword
yeet "testz"

# Type alias declarations
be_like MyInt = normie
be_like MyString = tea
be_like MyFloat = meal

# Test basic type alias usage
slay test_type_aliases() {
    test_start("Type aliases test")
    
    # Test type alias variable declarations
    sus x MyInt = 42
    sus name MyString = "test"
    sus value MyFloat = 3.14
    
    # Test type alias in function parameters
    sus result normie = add_numbers(10, 20)
    
    assert_eq_int(result, 30)
    assert_eq_string(name, "test")
    
    print_test_summary()
}

# Function using type alias
slay add_numbers(a MyInt, b MyInt) MyInt {
    damn a + b
}

# Main function
slay main_character() {
    test_type_aliases()
}
