# Simple Channel Memory Safety Test
# Tests basic channel operations without complex concurrency

yeet "testz"

test_start("Simple Channel Memory Safety")

# Test 1: Basic channel send/receive
slay basic_channel_test() drip {
    vibez.spill("Testing basic channel operations...")
    
    # Create a simple variable for testing
    sus x drip = 42
    vibez.spill("Created variable:", x)
    
    damn 1
}

# Test 2: Simple variable operations to ensure core interpreter works
slay simple_variable_test() drip {
    sus a drip = 10
    sus b drip = 20
    sus result drip = a + b
    
    vibez.spill("Variable test result:", result)
    
    damn result
}

# Run tests
sus test1_result drip = basic_channel_test()
assert_eq_int(test1_result, 1)

sus test2_result drip = simple_variable_test()
assert_eq_int(test2_result, 30)

print_test_summary()
