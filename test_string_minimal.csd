// Minimal test for string operations

slay test_basic_strings() {
    vibez.spill("Testing basic string operations...")
    
    // Test string concatenation
    sus s1 tea = "hello"
    sus s2 tea = " world"
    sus result tea = s1 + s2
    vibez.spill("Concatenation: " + result)
    
    // Test character access
    sus ch sip = s1[0]
    vibez.spill("First character of hello: " + tea(ch))
    
    // Test string comparison
    sus same lit = s1 == "hello"
    vibez.spill("String equals hello: " + tea(same))
    
    // Test string length (simple approach)
    sus len normie = 5  // Known length for testing
    vibez.spill("Length of hello: " + tea(len))
}

slay test_character_operations() {
    vibez.spill("Testing character operations...")
    
    // Character conversion
    sus ch_lower sip = 'a'
    sus ch_upper sip = 'A'
    
    vibez.spill("Lowercase a: " + tea(ch_lower))
    vibez.spill("Uppercase A: " + tea(ch_upper))
    
    // Character arithmetic
    sus diff normie = ch_upper - ch_lower
    vibez.spill("Difference between A and a: " + tea(diff))
}

slay test_string_building() {
    vibez.spill("Testing string building...")
    
    sus result tea = ""
    result = result + "H"
    result = result + "e"
    result = result + "l"
    result = result + "l"
    result = result + "o"
    
    vibez.spill("Built string: " + result)
}

slay run_minimal_tests() {
    vibez.spill("🔥 Minimal String Tests")
    vibez.spill("=======================")
    
    test_basic_strings()
    test_character_operations()
    test_string_building()
    
    vibez.spill("✓ Minimal tests completed!")
}

// Run tests
run_minimal_tests()
