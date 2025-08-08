# Test 3: Control structures (if/else, loops, pattern matching)
vibez.spill("Control structures test:")

# If/else test
sus x drip = 15
ready (x > 10) {
    vibez.spill("x is greater than 10")
} otherwise {
    vibez.spill("x is not greater than 10")
}

# While loop test
sus i drip = 0
vibez.spill("While loop counting to 3:")
bestie (i < 3) {
    vibez.spill("Loop iteration:", i)
    i = i + 1
}

# For-like loop with counter
sus counter drip = 0
sus total drip = 0
bestie (counter < 5) {
    total = total + counter
    counter = counter + 1
}
vibez.spill("Sum 0 to 4 =", total)

# Nested conditions
sus a drip = 5
sus b drip = 3
ready (a > b) {
    ready (a > 4) {
        vibez.spill("a > b and a > 4")
    } otherwise {
        vibez.spill("a > b but a <= 4")
    }
} otherwise {
    vibez.spill("a <= b")
}
