# Test nested control structures
sus count drip = 0
ready (count < 5) {
    ready (count % 2 == 0) {
        vibez.spill("Even:", count)
    } otherwise {
        vibez.spill("Odd:", count)
    }
    count = count + 1
} otherwise {
    vibez.spill("Count too high")
}

# Test while loop with break conditions
sus j drip = 0
bestie (j < 10) {
    ready (j == 5) {
        vibez.spill("Breaking at 5")
        break
    }
    vibez.spill("j =", j)
    j = j + 1
}

# Test boolean expressions in conditions
sus flag lit = based
ready (flag) {
    vibez.spill("Flag is true")
} otherwise {
    vibez.spill("Flag is false")
}

# Test complex conditions
sus a drip = 7
sus b drip = 3
ready (a > 5 and b < 5) {
    vibez.spill("Complex condition met")
}
