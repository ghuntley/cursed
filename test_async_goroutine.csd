// Test async goroutine spawning with yolo keyword
vibez.spill("Testing async goroutine spawning...")

// Spawn a simple goroutine using yolo
yolo {
    vibez.spill("Goroutine 1 running")
}

// Spawn another goroutine with parameters
yolo {
    vibez.spill("Goroutine 2 running")
}

vibez.spill("Main thread continues")

// Wait a bit for goroutines to complete
bestie i := 0; i < 1000; i++ {
    // Simple busy wait
}

vibez.spill("Test complete")
