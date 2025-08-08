// Test concurrent operations with fixed implementation
yeet "concurrenz"

// Create a buffered channel
sus ch = make_channel(drip, 5)

// Test 1: Simple send/receive
send_channel(ch, 42)
sus received = recv_channel(ch)
vibez.spill("Received:", received)

// Test 2: Multiple sends
sus i drip = 0
bestie (i < 3) {
    send_channel(ch, i)
    i = i + 1
}

// Receive all
sus j drip = 0
bestie (j < 3) {
    sus val = recv_channel(ch)
    vibez.spill("Value:", val)
    j = j + 1
}

vibez.spill("Concurrent test completed successfully")
