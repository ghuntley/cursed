# Test 4: Concurrency Features
yeet "concurrenz"

# Basic goroutine test
stan {
    vibez.spill("Hello from goroutine!")
}

# Channel test
sus ch drip = dm_create(drip)

stan {
    dm_send(ch, 42)
}

sus value drip = dm_recv(ch)
vibez.spill("Received from channel:")
vibez.spill(value)

# Channel with timeout
sus ch2 drip = dm_create(tea)
stan {
    vibez.spill("Goroutine sending message")
    dm_send(ch2, "Hello concurrency!")
}

sus msg tea = dm_recv(ch2)
vibez.spill("Received message:")
vibez.spill(msg)
