// Minimal Error Handling Test

yeet "testz"

slay test_simple() {
    test_start("Simple Test")
    
    sus message tea = "Hello"
    assert_true(message == "Hello")
    
    print_test_summary()
}

slay main() {
    test_simple()
}
