# Test vibe_check statements (CURSED-style select)
yeet "testz"

# Test vibe_check statement
slay test_vibe_check() {
    test_start("Vibe check test")
    
    # Test basic vibe_check
    sus ch dm<normie> = make_channel()
    sus result normie = 0
    
    # Send a value
    ch <- 42
    
    # Use vibe_check to receive
    vibe_check {
        mood val := <-ch: {
            result = val
        }
        basic: {
            result = -1
        }
    }
    
    assert_eq_int(result, 42)
    
    print_test_summary()
}

# Helper function to create channel
slay make_channel() dm<normie> {
    # This would be implemented by the runtime
    damn cringe
}

# Main function
slay main_character() {
    test_vibe_check()
}
