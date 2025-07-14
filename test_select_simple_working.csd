slay test_select_simple_working() {
    vibez.spill("Testing basic select statement...")
    
    # Test select with default case
    ready {
        basic:
            vibez.spill("Default case executed")
    }
    
    vibez.spill("Select test completed!")
}

test_select_simple_working()
