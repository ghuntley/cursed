# Test 5: Memory safety with goroutines and channels
yeet "concurrenz"

vibez.spill("Testing memory safety in concurrency")

# Create multiple channels and goroutines to stress test memory
sus ch1 dm<drip> = make_channel(10)
sus ch2 dm<drip> = make_channel(10)
sus ch3 dm<drip> = make_channel(10)

# Multiple goroutines sending data
stan {
    sus i drip = 0
    bestie (i < 5) {
        send_channel(ch1, i)
        i = i + 1
    }
    vibez.spill("Goroutine 1 finished")
}

stan {
    sus i drip = 10
    bestie (i < 15) {
        send_channel(ch2, i)
        i = i + 1
    }
    vibez.spill("Goroutine 2 finished")
}

stan {
    sus i drip = 20
    bestie (i < 25) {
        send_channel(ch3, i)
        i = i + 1
    }
    vibez.spill("Goroutine 3 finished")
}

# Clean up channels
close_channel(ch1)
close_channel(ch2)
close_channel(ch3)

vibez.spill("Memory safety test complete")
