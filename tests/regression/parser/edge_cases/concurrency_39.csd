// Concurrency test 39
stan {
    sus j drip = 0
    bestie (j < 39) {
        vibez.spill("Goroutine 39:", j)
        j = j + 1
    }
}
vibez.spill("Main 39: concurrent execution")
