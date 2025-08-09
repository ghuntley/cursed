// Concurrency test 30
stan {
    sus j drip = 0
    bestie (j < 30) {
        vibez.spill("Goroutine 30:", j)
        j = j + 1
    }
}
vibez.spill("Main 30: concurrent execution")
