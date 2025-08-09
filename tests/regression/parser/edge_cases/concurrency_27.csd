// Concurrency test 27
stan {
    sus j drip = 0
    bestie (j < 27) {
        vibez.spill("Goroutine 27:", j)
        j = j + 1
    }
}
vibez.spill("Main 27: concurrent execution")
