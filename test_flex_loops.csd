# Test flex loops with range expressions
yeet "testz"

# Test flex range loops
slay test_flex_loops() {
    test_start("Flex loops test")
    
    # Test basic flex loop
    sus sum normie = 0
    flex i in 1..5 {
        sum = sum + i
    }
    
    # Sum should be 1 + 2 + 3 + 4 = 10
    assert_eq_int(sum, 10)
    
    # Test flex loop with variables
    sus start normie = 0
    sus end normie = 3
    sus count normie = 0
    flex j in start..end {
        count = count + 1
    }
    
    assert_eq_int(count, 3)
    
    print_test_summary()
}

# Main function
slay main_character() {
    test_flex_loops()
}
