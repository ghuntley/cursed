# Test 4: Control flow (lowkey/highkey, bestie loops)
sus x drip = 5

lowkey x > 3 {
    vibez.spill("x is greater than 3")
} highkey x < 10 {
    vibez.spill("x is less than 10")
} cap {
    vibez.spill("x is something else")
}

sus i drip = 0
bestie i < 3 {
    vibez.spill("Loop iteration: " + i)
    i = i + 1
}
