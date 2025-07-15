yeet "vibez"

slay test_defer_basic() lit {
    vibez.spill("Function start")
    
    # Basic defer statement
    later vibez.spill("Defer 1: executed at end")
    
    vibez.spill("After defer 1")
    
    # Multiple defers (LIFO order)
    later vibez.spill("Defer 2: executed second")
    later vibez.spill("Defer 3: executed first")
    
    vibez.spill("Function end")
    damn based
}

slay test_defer_with_cleanup() lit {
    vibez.spill("Opening resource")
    
    # Defer cleanup operation
    later vibez.spill("Closing resource")
    
    vibez.spill("Using resource")
    damn based
}

slay test_defer_with_early_return() lit {
    vibez.spill("Function start")
    
    later vibez.spill("Cleanup: executed before return")
    
    lowkey based {
        vibez.spill("Taking early return")
        damn based  # defer still executes
    }
    
    vibez.spill("This won't execute")
    damn cap
}

slay main() lit {
    vibez.spill("=== Defer Grammar Tests ===")
    
    test_defer_basic()
    vibez.spill("")
    
    test_defer_with_cleanup()
    vibez.spill("")
    
    test_defer_with_early_return()
    vibez.spill("")
    
    vibez.spill("=== Tests Complete ===")
    damn based
}
