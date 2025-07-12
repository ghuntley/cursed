# Test all new parser features together
yeet (
    "testz";
    "core"
)

# Type aliases
be_like Counter = normie
be_like Message = tea

# Test all features combined
slay test_all_parser_features() {
    test_start("All parser features test")
    
    # Test type aliases
    sus count Counter = 0
    sus msg Message = "Hello"
    
    # Test flex loops
    flex i in 1..4 {
        count = count + i
    }
    
    # Test vibe_check (basic syntax)
    sus selected lit = cap
    vibe_check {
        mood count > 5: {
            selected = based
        }
        basic: {
            selected = cap
        }
    }
    
    # Assertions
    assert_eq_int(count, 6)  # 1 + 2 + 3 = 6
    assert_eq_string(msg, "Hello")
    assert_true(selected)
    
    print_test_summary()
}

# Main function
slay main_character() {
    test_all_parser_features()
}
