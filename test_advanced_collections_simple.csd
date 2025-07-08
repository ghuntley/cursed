yeet "testz"

// Simple test to verify basic parsing
slay test_simple_parsing() {
    test_start("Simple Parsing Test")
    
    vibez.spill("Testing simple variable declaration")
    sus x normie = 42
    assert_eq_int(x, 42)
    
    vibez.spill("Testing simple loop")
    sus i normie = 0
    bestie i < 3 {
        vibez.spill("Loop iteration: " + tea(i))
        i = i + 1
    }
    
    vibez.spill("Simple parsing test completed")
}

// Run the test
test_simple_parsing()
print_test_summary()
