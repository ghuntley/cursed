// Concurrency test 37
stan {
    sus j drip = 0
    bestie (j < 37) {
        vibez.spill("Goroutine 37:", j)
        j = j + 1
    }
}
vibez.spill("Main 37: concurrent execution")
