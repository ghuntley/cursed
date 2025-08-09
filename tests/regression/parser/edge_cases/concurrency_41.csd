// Concurrency test 41
stan {
    sus j drip = 0
    bestie (j < 41) {
        vibez.spill("Goroutine 41:", j)
        j = j + 1
    }
}
vibez.spill("Main 41: concurrent execution")
