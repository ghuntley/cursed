// Test improved channel blocking performance
vibez.spill("Channel blocking improvements test started")

// Create an unbuffered channel
sus ch = make(Channel, 0)

// Test that the timeout operations use proper blocking instead of busy-wait
sus start_time = time.now()

// Try to send with timeout - should block briefly without busy-wait
go suspend {
    time.sleep(50) // 50ms delay
    sus received = ch.recv()
    vibez.spill("Received: " + received)
}

sus result = ch.send_timeout("test_message", 100) // 100ms timeout
sus end_time = time.now()
sus elapsed = end_time - start_time

vibez.spill("Send with timeout completed")
vibez.spill("Elapsed time: " + elapsed + "ms")

// The operation should complete in roughly 50ms (not busy-wait for 100ms)
sus expected_max = 80 // Allow some overhead
if elapsed < expected_max {
    vibez.spill("SUCCESS: Proper blocking detected (no busy-wait)")
} else {
    vibez.spill("WARNING: May still be using busy-wait")
}

vibez.spill("Channel blocking improvements test completed")
