# Simple race condition test for goroutine scheduler

sus counter drip = 0

slay increment_counter() {
    counter = counter + 1
}

slay test_basic_goroutines() {
    # Test basic goroutine functionality
    vibez.spill("Testing basic goroutine functionality...")
    
    counter = 0
    
    # Spawn a simple goroutine
    stan increment_counter()
    
    # Wait a moment
    timez.sleep(100)
    
    vibez.spill("Counter value: {}", counter)
}

slay main() {
    test_basic_goroutines()
}
