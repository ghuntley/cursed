# Debug System Test for CURSED
# This program tests the enhanced debug information system

# Test program with various constructs for debugging
slay main() {
    # Variable declarations with debug info
    sus greeting tea = "Hello, Debug World!"
    sus counter normie = 42
    sus active lit = based
    
    # Function calls with debug tracking
    vibez.spill(greeting)
    vibez.spill(counter)
    vibez.spill(active)
    
    # Test error handling with debug context
    test_error_handling()
    
    # Test complex expressions with debug info
    sus result normie = calculate_value(counter)
    vibez.spill(result)
    
    # Test control flow with debug tracking
    bestie i := 0; i < 5; i++ {
        vibez.spill(i)
    }
    
    # Test tuple destructuring with debug info
    sus coordinates := (10, 20, 30)
    (sus x, sus y, sus z) := coordinates
    vibez.spill(x)
    vibez.spill(y)
    vibez.spill(z)
}

# Function to test error handling with debug info
slay test_error_handling() {
    # This will generate debug context for error messages
    sus value normie = 100
    
    # Test division by zero with debug context
    sus divisor normie = 0
    
    # This should generate enhanced error message with source location
    # sus invalid_result normie = value / divisor
    
    vibez.spill("Error handling test completed")
}

# Function to test complex calculations with debug info
slay calculate_value(input normie) normie {
    sus multiplier normie = 2
    sus result normie = input * multiplier
    damn result
}

# Test struct with debug information
struct DebugTestStruct {
    name tea
    value normie
    active lit
}

# Test interface with debug information
interface DebugTestInterface {
    slay process_data(data tea) normie
    slay validate_input(input normie) lit
}
