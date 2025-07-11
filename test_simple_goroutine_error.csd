// Simple test for goroutine error isolation
yeet "testz"

test_start("basic goroutine error isolation")

// Test that runtime survives goroutine panics
slay test_basic_isolation() {
    vibez.spill("Starting basic isolation test")
    
    // This goroutine should panic but not crash the runtime
    slay panic_goroutine() {
        vibez.spill("Goroutine about to panic")
        sus x normie = 1
        sus y normie = 0
        sus z normie = x / y  // Should panic with division by zero
    }
    
    // This goroutine should complete normally
    slay normal_goroutine() {
        vibez.spill("Normal goroutine completed successfully")
    }
    
    // Spawn both goroutines
    yolo panic_goroutine()
    yolo normal_goroutine()
    
    // Simple wait
    sus i normie = 0
    bestie i < 1000; i++ {
        // Wait loop
    }
    
    vibez.spill("✅ Basic isolation test completed - runtime survived!")
}

test_basic_isolation()
print_test_summary()
