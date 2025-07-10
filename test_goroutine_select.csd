// Test goroutines with select statements
vibez.spill("Testing goroutines with select statements")

// Create channels for communication
sus workChannel dm<normie>
sus resultChannel dm<tea>

// Spawn a goroutine to send data
yolo {
    workChannel <- 42
    resultChannel <- "work completed"
}

// Use select to receive from multiple channels
ready {
    mood work := <-workChannel -> {
        vibez.spill("Got work: " + work)
    }
    mood result := <-resultChannel -> {
        vibez.spill("Got result: " + result)
    }
    basic -> {
        vibez.spill("No messages yet")
    }
}

vibez.spill("Goroutine select test completed")
