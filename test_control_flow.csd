// Test control flow
sus x drip = 10

ready (x > 5) {
    vibez.spill("x is greater than 5")
} otherwise {
    vibez.spill("x is not greater than 5")
}

sus i drip = 0
bestie (i < 3) {
    vibez.spill("Loop iteration:", i)
    i = i + 1
}
