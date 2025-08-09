// Concurrency test 38
stan {
    sus j drip = 0
    bestie (j < 38) {
        vibez.spill("Goroutine 38:", j)
        j = j + 1
    }
}
vibez.spill("Main 38: concurrent execution")
