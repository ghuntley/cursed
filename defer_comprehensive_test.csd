slay test_basic_defer() lit {
    vibez.spill("=== Testing Basic Defer ===")
    
    vibez.spill("Function start")
    
    later vibez.spill("Defer 1: This should execute last")
    vibez.spill("After defer 1")
    
    later vibez.spill("Defer 2: This should execute second to last")
    vibez.spill("After defer 2")
    
    later vibez.spill("Defer 3: This should execute first (LIFO)")
    vibez.spill("After defer 3")
    
    vibez.spill("Function end")
    damn based
}

slay test_defer_with_early_return() lit {
    vibez.spill("=== Testing Defer with Early Return ===")
    
    vibez.spill("Function start")
    
    later vibez.spill("Defer cleanup: This should execute before return")
    
    lowkey based {
        vibez.spill("Taking early return path")
        damn based
    }
    
    vibez.spill("This should not be reached")
    damn cap
}

slay test_nested_defer() lit {
    vibez.spill("=== Testing Nested Defer ===")
    
    vibez.spill("Outer function start")
    
    later vibez.spill("Outer defer 1")
    
    lowkey based {
        vibez.spill("Inner block start")
        later vibez.spill("Inner defer 1")
        later vibez.spill("Inner defer 2")
        vibez.spill("Inner block end")
    }
    
    later vibez.spill("Outer defer 2")
    
    vibez.spill("Outer function end")
    damn based
}

slay test_defer_with_variables() lit {
    vibez.spill("=== Testing Defer with Variables ===")
    
    sus resource tea = "Resource Handle"
    vibez.spill("Opening resource")
    
    later vibez.spill("Closing resource")
    later vibez.spill("Cleaning up")
    
    vibez.spill("Using resource")
    vibez.spill("Resource operations complete")
    
    damn based
}

slay test_multiple_functions_with_defer() lit {
    vibez.spill("=== Testing Multiple Functions with Defer ===")
    
    vibez.spill("Function 1 start")
    later vibez.spill("Function 1 defer cleanup")
    
    vibez.spill("Calling function 2")
    test_basic_defer()
    
    vibez.spill("Function 1 end")
    damn based
}

slay main() lit {
    vibez.spill("🚀 Starting Comprehensive Defer Tests")
    vibez.spill("=" * 50)
    
    test_basic_defer()
    vibez.spill("")
    
    test_defer_with_early_return()
    vibez.spill("")
    
    test_nested_defer()
    vibez.spill("")
    
    test_defer_with_variables()
    vibez.spill("")
    
    test_multiple_functions_with_defer()
    vibez.spill("")
    
    vibez.spill("=" * 50)
    vibez.spill("✅ All defer tests completed")
    damn based
}
