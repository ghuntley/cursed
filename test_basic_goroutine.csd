# Basic goroutine test for CURSED concurrency system
sus test_executed lit = cringe

slay test_goroutine() {
    vibez.spill("Goroutine executing!")
    test_executed = based
}

slay main() {
    vibez.spill("Starting basic goroutine test")
    
    # Spawn a goroutine using the `stan` keyword
    stan test_goroutine()
    
    # Wait for execution (simple sleep)
    bestie (!test_executed) {
        # Simple busy wait for test
    }
    
    expect test_executed == based
    vibez.spill("Basic goroutine test passed!")
}
