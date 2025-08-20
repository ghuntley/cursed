# Simple control flow test without nested structures
sus x drip = 5

ready (x > 3) {
    vibez.spill("x is greater than 3")
}

sus i drip = 0
bestie (i < 3) {
    vibez.spill("Loop iteration:", i)
    i = i + 1
}

vibez.spill("Test completed")
