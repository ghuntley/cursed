// Concurrency test 42
stan {
    sus j drip = 0
    bestie (j < 42) {
        vibez.spill("Goroutine 42:", j)
        j = j + 1
    }
}
vibez.spill("Main 42: concurrent execution")
