// Test basic goroutine execution
stan {
    vibez.spill("Goroutine working")
}
vibez.spill("Main thread")

// Test channel operations
yeet "concurrenz"
sus ch = make_channel()
send_channel(ch, 42)
sus value = recv_channel(ch)
vibez.spill("Received:", value)
