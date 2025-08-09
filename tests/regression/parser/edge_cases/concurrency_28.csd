// Concurrency test 28
stan {
    sus j drip = 0
    bestie (j < 28) {
        vibez.spill("Goroutine 28:", j)
        j = j + 1
    }
}
vibez.spill("Main 28: concurrent execution")
