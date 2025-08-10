// Simple test for basic unwinding

slay test_basic_defer() {
    vibez.spill("Testing basic defer...")
    
    defer {
        vibez.spill("Defer executed!")
    }
    
    vibez.spill("About to return")
}

slay test_panic_with_defer() {
    vibez.spill("Testing panic with defer...")
    
    defer {
        vibez.spill("Cleanup during panic!")
    }
    
    vibez.spill("About to panic...")
    // This would panic in a full implementation
    vibez.spill("Simulated panic")
}

slay main() {
    vibez.spill("🚀 Starting simple unwind test")
    
    test_basic_defer()
    test_panic_with_defer()
    
    vibez.spill("✅ Simple unwind test completed")
}

main()
