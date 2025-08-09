// Concurrency test 26
stan {
    sus j drip = 0
    bestie (j < 26) {
        vibez.spill("Goroutine 26:", j)
        j = j + 1
    }
}
vibez.spill("Main 26: concurrent execution")
