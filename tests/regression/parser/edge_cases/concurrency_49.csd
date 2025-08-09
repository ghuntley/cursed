// Concurrency test 49
stan {
    sus j drip = 0
    bestie (j < 49) {
        vibez.spill("Goroutine 49:", j)
        j = j + 1
    }
}
vibez.spill("Main 49: concurrent execution")
