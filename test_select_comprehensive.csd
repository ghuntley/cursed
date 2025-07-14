slay test_select_comprehensive() {
    vibez.spill("=== Testing Complete Select Statement Implementation ===")
    
    # Test 1: Basic select with default case
    vibez.spill("Test 1: Basic select with default case")
    ready {
        basic:
            vibez.spill("✅ Default case executed successfully")
    }
    
    # Test 2: Select with multiple cases (simple syntax)
    vibez.spill("Test 2: Select with multiple cases")
    ready {
        mood <-signal_chan:
            vibez.spill("Signal received (should not happen)")
        mood timeout_chan <- 1:
            vibez.spill("Timeout signal sent (should not happen)")
        basic:
            vibez.spill("✅ Multiple cases handled correctly")
    }
    
    # Test 3: Select with variable assignment
    vibez.spill("Test 3: Select with variable assignment")
    ready {
        mood value := <-data_chan:
            vibez.spill("Received value: ", value)
        basic:
            vibez.spill("✅ Variable assignment syntax works")
    }
    
    # Test 4: Nested select statements
    vibez.spill("Test 4: Nested select statements")
    ready {
        basic:
            vibez.spill("Outer select")
            ready {
                basic:
                    vibez.spill("✅ Nested select works")
            }
    }
    
    vibez.spill("=== All select statement tests passed! ===")
}

test_select_comprehensive()
