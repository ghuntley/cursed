# Control Structures Test
sus x drip = 7
sus i drip = 0

# If/else test
ready (x > 5) {
    vibez.spill("x is greater than 5")
} otherwise {
    vibez.spill("x is not greater than 5")
}

# While loop test
vibez.spill("Counting:")
bestie (i < 3) {
    vibez.spill("i is:", i)
    i = i + 1
}

# Nested conditions
sus age drip = 25
ready (age >= 18) {
    ready (age >= 65) {
        vibez.spill("Senior citizen")
    } otherwise {
        vibez.spill("Adult")
    }
} otherwise {
    vibez.spill("Minor")
}
