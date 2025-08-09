// Concurrency test 46
stan {
    sus j drip = 0
    bestie (j < 46) {
        vibez.spill("Goroutine 46:", j)
        j = j + 1
    }
}
vibez.spill("Main 46: concurrent execution")
