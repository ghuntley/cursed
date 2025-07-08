// Test working string functions in pure CURSED

slay test_working_operations() {
    vibez.spill("Testing working string operations...")
    
    // String concatenation works
    sus s1 tea = "hello"
    sus s2 tea = " world"
    sus result tea = s1 + s2
    vibez.spill("✓ Concatenation: " + result)
    
    // Character access works
    sus ch sip = s1[0]
    vibez.spill("✓ First character: " + tea(ch))
    
    // String comparison works
    sus same lit = s1 == "hello"
    vibez.spill("✓ String equality: " + tea(same))
    
    // Basic character literals work
    sus a_char sip = 'a'
    sus z_char sip = 'z'
    vibez.spill("✓ Character literals: " + tea(a_char) + " and " + tea(z_char))
    
    // String building works
    sus built tea = ""
    built = built + "H"
    built = built + "e"
    built = built + "l"
    built = built + "l"
    built = built + "o"
    vibez.spill("✓ String building: " + built)
}

slay test_conditional_logic() {
    vibez.spill("Testing conditional string logic...")
    
    sus test_str tea = "hello"
    sus empty_str tea = ""
    
    // Test empty string detection
    lowkey test_str == "" {
        vibez.spill("Test string is empty")
    } highkey {
        vibez.spill("✓ Test string is not empty")
    }
    
    lowkey empty_str == "" {
        vibez.spill("✓ Empty string detected correctly")
    } highkey {
        vibez.spill("Empty string not detected")
    }
}

slay test_basic_loops() {
    vibez.spill("Testing basic loops with strings...")
    
    sus s tea = "abc"
    sus result tea = ""
    
    // Simple loop to build string
    sus i normie = 0
    bestie i < 3; i++ {
        result = result + s[i]
    }
    
    vibez.spill("✓ Loop result: " + result)
}

slay run_working_tests() {
    vibez.spill("🔥 Working String Function Tests")
    vibez.spill("=================================")
    
    test_working_operations()
    test_conditional_logic()
    test_basic_loops()
    
    vibez.spill("✓ All working tests completed!")
}

// Run tests
run_working_tests()
