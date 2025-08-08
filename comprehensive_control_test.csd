fr fr Test 1: Multi-line if/else
sus x drip = 10
ready (x > 5) {
    vibez.spill("x is greater than 5")
} otherwise {
    vibez.spill("x is not greater than 5")
}

fr fr Test 2: Single-line if without else
sus y drip = 7
ready (y > 5) {
    vibez.spill("y is greater than 5")
}

fr fr Test 3: Complex expressions
sus a drip = 15
sus b drip = 10
ready ((a - b) > 3) {
    vibez.spill("Difference is greater than 3")
}

fr fr Test 4: Loop
sus i drip = 0
bestie (i < 3) {
    vibez.spill("Loop iteration:", i)
    i = i + 1
}

fr fr Test 5: Nested control structures
sus level drip = 8
ready (level > 5) {
    vibez.spill("Level is high")
    ready (level > 7) {
        vibez.spill("Level is very high")
    }
}
