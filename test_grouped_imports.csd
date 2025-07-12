# Test grouped imports using yeet (...) syntax
yeet (
    "testz";
    "core";
    "stringz"
)

# Test grouped imports functionality
slay test_grouped_imports() {
    test_start("Grouped imports test")
    
    # Test that all modules are imported
    sus result lit = based
    assert_true(result)
    
    # Test string functions from stringz
    sus text tea = "Hello, World!"
    assert_eq_string(text, "Hello, World!")
    
    print_test_summary()
}

# Main function
slay main_character() {
    test_grouped_imports()
}
