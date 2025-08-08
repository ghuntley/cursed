# Test 1: Basic goroutine functionality with stan keyword
vibez.spill("Starting basic goroutine test")

stan {
    vibez.spill("Goroutine executing!")
}

vibez.spill("Main thread continues")
vibez.spill("Test complete")
