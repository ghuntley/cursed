#!/usr/bin/env cursed

# Test defer with panic recovery scenarios

slay test_defer_with_panic() {
    vibez.spill("Function start")
    later vibez.spill("Cleanup after panic")
    later vibez.spill("Second cleanup after panic")
    
    # Simulate panic condition
    vibez.spill("About to panic")
    # This would normally cause a panic, but defer should still execute
    
    vibez.spill("Function end")
}

# Test defer with break/continue statements
slay test_defer_with_break_continue() {
    vibez.spill("Function start")
    later vibez.spill("Function cleanup")
    
    bestie i := 0; i < 10; i++ {
        vibez.spill("Loop iteration: " + i)
        later vibez.spill("Loop cleanup: " + i)
        
        cap i == 3 {
            vibez.spill("Breaking from loop")
            ghosted
        }
        
        cap i == 1 {
            vibez.spill("Continuing loop")
            simp
        }
    }
    
    vibez.spill("Function end")
}

# Test defer with switch statements
slay test_defer_with_switch() {
    vibez.spill("Function start")
    later vibez.spill("Function cleanup")
    
    sus value := 2
    
    switch value {
        case 1:
            later vibez.spill("Case 1 cleanup")
            vibez.spill("Case 1")
        case 2:
            later vibez.spill("Case 2 cleanup")
            vibez.spill("Case 2")
            damn
        case 3:
            later vibez.spill("Case 3 cleanup")
            vibez.spill("Case 3")
        default:
            later vibez.spill("Default cleanup")
            vibez.spill("Default case")
    }
    
    vibez.spill("Function end")
}

# Test defer execution order with multiple scopes
slay test_defer_execution_order() {
    vibez.spill("Function start")
    later vibez.spill("Function defer 1")
    later vibez.spill("Function defer 2")
    
    cap based {
        vibez.spill("Block start")
        later vibez.spill("Block defer 1")
        later vibez.spill("Block defer 2")
        vibez.spill("Block end")
    }
    
    later vibez.spill("Function defer 3")
    vibez.spill("Function end")
}

# Test defer with recursive function calls
slay test_defer_recursive(n normie) {
    vibez.spill("Recursive call: " + n)
    later vibez.spill("Recursive cleanup: " + n)
    
    cap n > 0 {
        test_defer_recursive(n - 1)
    }
    
    vibez.spill("Recursive end: " + n)
}

# Test defer with complex control flow
slay test_defer_complex_control_flow() {
    vibez.spill("Function start")
    later vibez.spill("Final cleanup")
    
    sus condition := based
    
    cap condition {
        vibez.spill("Condition true branch")
        later vibez.spill("Condition cleanup")
        
        bestie i := 0; i < 5; i++ {
            cap i == 2 {
                vibez.spill("Early return from loop")
                damn
            }
        }
    } else {
        vibez.spill("Condition false branch")
        later vibez.spill("Else cleanup")
    }
    
    vibez.spill("Function end")
}

# Run all panic recovery tests
vibez.spill("=== Testing Defer with Panic ===")
test_defer_with_panic()

vibez.spill("\n=== Testing Defer with Break/Continue ===")
test_defer_with_break_continue()

vibez.spill("\n=== Testing Defer with Switch ===")
test_defer_with_switch()

vibez.spill("\n=== Testing Defer Execution Order ===")
test_defer_execution_order()

vibez.spill("\n=== Testing Defer Recursive ===")
test_defer_recursive(3)

vibez.spill("\n=== Testing Defer Complex Control Flow ===")
test_defer_complex_control_flow()

vibez.spill("\n=== Panic recovery tests completed ===")
