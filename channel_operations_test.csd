// CURSED Channel Operations Test
// Demonstrates dm<T> channel types with dm_send, dm_recv, and dm_close

vibe main

// Create channels of different types
sus int_channel dm<normie> = dm<normie>(3)      // Buffered channel with capacity 3
sus msg_channel dm<tea> = dm<tea>(0)            // Unbuffered channel
sus bool_channel dm<lit> = dm<lit>(1)           // Small buffered channel

// Test 1: Basic send and receive operations
vibez.spill("=== Test 1: Basic Channel Operations ===")

// Send values to buffered channel
dm_send(int_channel, 42)
dm_send(int_channel, 84) 
dm_send(int_channel, 126)

// Receive values
sus value1 normie = dm_recv(int_channel)
sus value2 normie = dm_recv(int_channel) 
sus value3 normie = dm_recv(int_channel)

vibez.spill("Received integers:", value1, value2, value3)

// Test 2: Goroutine communication
vibez.spill("=== Test 2: Goroutine Communication ===")

// Spawn goroutine to send messages
stan {
    dm_send(msg_channel, "Hello from goroutine!")
    dm_send(msg_channel, "Second message")
    dm_send(msg_channel, "Final message")
    dm_close(msg_channel)
}

// Receive messages in main goroutine
bestie (!dm_closed(msg_channel)) {
    sus message tea = dm_recv(msg_channel)
    ready (message != "") {
        vibez.spill("Received:", message)
    } otherwise {
        // Channel is closed and empty
        vibez.spill("Channel closed")
        cap
    }
}

// Test 3: Channel closing and error handling
vibez.spill("=== Test 3: Channel Closing ===")

dm_send(bool_channel, based)
dm_close(bool_channel)

// Try to send to closed channel (should handle gracefully)
sus send_result = dm_send(bool_channel, cap)
ready (send_result == "closed") {
    vibez.spill("Cannot send to closed channel - handled correctly")
}

// Try to receive from closed channel with buffered data
sus final_value lit = dm_recv(bool_channel)
ready (final_value != null) {
    vibez.spill("Received final value before close:", final_value)
}

// Test 4: Select statement with channels
vibez.spill("=== Test 4: Select Statement ===")

sus ch1 dm<normie> = dm<normie>(1)
sus ch2 dm<tea> = dm<tea>(1)

// Fill channels
dm_send(ch1, 999)
dm_send(ch2, "urgent")

// Select statement for channel multiplexing
ready {
    mood value := dm_recv(ch1):
        vibez.spill("Received from int channel:", value)
    mood message := dm_recv(ch2):
        vibez.spill("Received from string channel:", message)
    basic:
        vibez.spill("No channels ready")
}

// Clean up
dm_close(int_channel)
dm_close(bool_channel)
dm_close(ch1)
dm_close(ch2)

vibez.spill("=== Channel Operations Test Complete ===")
