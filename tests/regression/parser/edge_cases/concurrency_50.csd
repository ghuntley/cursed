// Concurrency test 50
stan {
    sus j drip = 0
    bestie (j < 50) {
        vibez.spill("Goroutine 50:", j)
        j = j + 1
    }
}
vibez.spill("Main 50: concurrent execution")
