slay demonstrate_defer() {
    vibez.spill("Function started")
    
    later {
        vibez.spill("First defer - executed last")
    }
    
    vibez.spill("Middle of function")
    
    later {
        vibez.spill("Second defer - executed first")
    }
    
    vibez.spill("Function ending")
}

slay demonstrate_defer_with_error() {
    vibez.spill("Error function started")
    
    later {
        vibez.spill("Error-safe defer cleanup")
    }
    
    vibez.spill("About to trigger error")
    damn 42
}

vibez.spill("=== Demonstrating CURSED Defer Statement Compilation ===")
demonstrate_defer()
vibez.spill("---")
demonstrate_defer_with_error()
vibez.spill("=== Demo Complete ===")
