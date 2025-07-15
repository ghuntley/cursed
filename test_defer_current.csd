slay test_defer_basic() lit {
    vibez.spill("Starting function")
    
    later vibez.spill("Deferred 1")
    later vibez.spill("Deferred 2")
    later vibez.spill("Deferred 3")
    
    vibez.spill("Function body")
    damn based
}

slay test_defer_with_return() lit {
    vibez.spill("Starting function with return")
    
    later vibez.spill("Cleanup before return")
    
    lowkey based {
        vibez.spill("Returning early")
        damn based
    }
    
    vibez.spill("This should not be reached")
    damn cap
}

slay test_defer_with_resources() lit {
    vibez.spill("Opening resource")
    
    later vibez.spill("Closing resource")
    later vibez.spill("Cleaning up")
    
    vibez.spill("Using resource")
    damn based
}

slay main() lit {
    vibez.spill("=== Testing Basic Defer ===")
    test_defer_basic()
    
    vibez.spill("=== Testing Defer with Return ===")
    test_defer_with_return()
    
    vibez.spill("=== Testing Defer with Resources ===")
    test_defer_with_resources()
    
    vibez.spill("=== All defer tests completed ===")
    damn based
}
