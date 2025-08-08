# Test 7: Concurrency features (goroutines, channels)
vibez.spill("Concurrency test:")

# Test basic goroutine
vibez.spill("Starting goroutine...")
stan {
    vibez.spill("Goroutine 1 executing")
    vibez.spill("Goroutine 1 done")
}

# Test multiple goroutines
stan {
    vibez.spill("Goroutine 2 executing")
}

stan {
    vibez.spill("Goroutine 3 executing")
}

vibez.spill("Main thread continuing...")

# Test goroutine with computation
stan {
    sus i drip = 0
    bestie (i < 3) {
        vibez.spill("Goroutine computing:", i)
        i = i + 1
    }
    vibez.spill("Goroutine computation done")
}

vibez.spill("Main thread finished")
