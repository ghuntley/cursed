// Concurrency test 35
stan {
    sus j drip = 0
    bestie (j < 35) {
        vibez.spill("Goroutine 35:", j)
        j = j + 1
    }
}
vibez.spill("Main 35: concurrent execution")
