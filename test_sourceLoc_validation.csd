yeet "testz"

slay test_drip_parsing() {
    test_start("Drip token parsing test")
    
    // Test basic drip variable declaration
    sus pi drip = 3.14159
    assert_eq_string("3.14159", "3.14159")
    
    // Test drip in function parameters
    slay add_floats(a drip, b drip) drip {
        damn a + b
    }
    
    // Test function call
    sus result drip = add_floats(1.5, 2.5)
    assert_true(based)
    
    print_test_summary()
}

test_drip_parsing()
