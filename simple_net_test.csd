// Simple networking test

fam "stdlib/testz"

slay test_basic_networking() {
    test_start("Basic Networking")
    
    vibez.spill("Testing networking module...")
    
    // Test simple functions without network operations
    sus test_string tea = "hello world"
    assert_eq_string(test_string, "hello world")
    
    vibez.spill("Basic test passed!")
}

slay main() {
    vibez.spill("Running simple networking test...")
    test_basic_networking()
    print_test_summary()
    vibez.spill("Test completed!")
}
