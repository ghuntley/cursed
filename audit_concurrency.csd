yeet "concurrenz"

// Test goroutines and channels
sus ch dm = dm_new<normie>(10)

stan {
    dm_send(ch, 42)
    dm_send(ch, 24)
}

stan {
    sus value drip = dm_recv(ch)
    vibez.spill("Received: ")
    vibez.spill(value)
}

// Wait a bit
sus i drip = 0
bestie (i < 1000) {
    i = i + 1
}
