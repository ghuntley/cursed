// Concurrency test 40
stan {
    sus j drip = 0
    bestie (j < 40) {
        vibez.spill("Goroutine 40:", j)
        j = j + 1
    }
}
vibez.spill("Main 40: concurrent execution")
