// Test channel type parsing specifically
yeet "testz"

slay test_channel_type_parsing() {
    test_start("Channel type parsing test")
    
    // Test different channel types
    sus ch1 dm<normie> = make(dm<normie>)
    sus ch2 dm<tea> = make(dm<tea>, 5)
    sus ch3 dm<lit> = make(dm<lit>, 1)
    
    vibez.spill("Created channels with different types")
    assert_true(based)  // If we get here, parsing worked
    
    print_test_summary()
}

slay main() {
    test_channel_type_parsing()
}
