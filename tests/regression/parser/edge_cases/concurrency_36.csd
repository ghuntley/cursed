// Concurrency test 36
stan {
    sus j drip = 0
    bestie (j < 36) {
        vibez.spill("Goroutine 36:", j)
        j = j + 1
    }
}
vibez.spill("Main 36: concurrent execution")
