# Test goroutines communicating via channels

sus global_result drip = 0

slay worker_goroutine(ch dm<drip>) {
    vibez.spill("Worker goroutine started")
    
    # Receive value from channel
    sus value drip = dm_recv(ch)
    vibez.spillf("Worker received: %d", value)
    
    # Process and send result back
    sus result drip = value * 2
    dm_send(ch, result)
    
    vibez.spill("Worker goroutine finished")
}

slay main() {
    vibez.spill("Starting goroutine-channel communication test")
    
    # Create unbuffered channel for synchronous communication
    sus ch dm<drip> = dm_new<drip>(0)
    
    # Spawn worker goroutine
    stan worker_goroutine(ch)
    
    # Send work to goroutine
    sus input drip = 21
    dm_send(ch, input)
    vibez.spillf("Sent input: %d", input)
    
    # Receive result
    sus output drip = dm_recv(ch)
    vibez.spillf("Received output: %d", output)
    
    expect output == 42
    
    vibez.spill("Goroutine-channel communication test passed!")
}
