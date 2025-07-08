// Final working string demonstration

slay test_string_operations() {
    vibez.spill("Testing string operations...")
    
    // String concatenation
    sus s1 tea = "hello"
    sus s2 tea = " world"
    sus result tea = s1 + s2
    vibez.spill("✓ Concatenation: " + result)
    
    // Character access
    sus ch sip = s1[0]
    vibez.spill("✓ First character: " + tea(ch))
    
    // String comparison
    sus same lit = s1 == "hello"
    vibez.spill("✓ String equality: " + tea(same))
    
    // String building with loop
    sus built tea = ""
    bestie i := 0; i < 3; i++ {
        built = built + "a"
    }
    vibez.spill("✓ String building with loop: " + built)
}

slay test_character_operations() {
    vibez.spill("Testing character operations...")
    
    sus ch1 sip = 'H'
    sus ch2 sip = 'e'
    sus ch3 sip = 'y'
    
    vibez.spill("✓ Character 1: " + tea(ch1))
    vibez.spill("✓ Character 2: " + tea(ch2)) 
    vibez.spill("✓ Character 3: " + tea(ch3))
}

slay test_string_conditionals() {
    vibez.spill("Testing string conditionals...")
    
    sus test_str tea = "hello"
    sus empty_str tea = ""
    
    lowkey test_str == "hello" {
        vibez.spill("✓ String matches hello")
    } highkey {
        vibez.spill("String does not match hello")
    }
    
    lowkey empty_str == "" {
        vibez.spill("✓ Empty string detected")
    } highkey {
        vibez.spill("Empty string not detected")
    }
}

slay demonstrate_string_features() {
    vibez.spill("🔥 CURSED String Feature Demonstration")
    vibez.spill("=====================================")
    
    test_string_operations()
    test_character_operations()
    test_string_conditionals()
    
    vibez.spill("✓ All string demonstrations completed!")
}

// Run demonstration
demonstrate_string_features()
