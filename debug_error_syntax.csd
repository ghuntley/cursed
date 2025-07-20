# Simple syntax test for error_drip
vibez.spill("Testing error_drip syntax")

# Test simple function call
slay test_simple() {
    vibez.spill("Simple test works")
    damn "success"
}

sus result := test_simple()
vibez.spill("Result: " + result)

# Test tuple creation
sus simple_tuple := ("type", "message", "wrapped")
vibez.spill("Tuple created")

# Test tuple access (simplified)
vibez.spill("Basic syntax test complete")
