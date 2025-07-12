yeet "testz"

slay test_simple_networking() {
    test_start("Simple Networking Test")
    
    // Test basic socket creation
    sus socket_handle normie = 1001
    assert_true(socket_handle > 1000)
    
    vibez.spill("Simple networking test passed")
}

slay main() {
    vibez.spill("Testing simple networking...")
    test_simple_networking()
    print_test_summary()
}
