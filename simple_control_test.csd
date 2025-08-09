# Simple Enhanced Control Test
sus a drip = 5
sus b drip = 10

# Test basic comparison
ready (a > 3) {
    vibez.spill("a is greater than 3")
}

# Test AND operator
ready (a > 3 && b < 15) {
    vibez.spill("AND condition works")
}

# Test OR operator
ready (a > 10 || b > 5) {
    vibez.spill("OR condition works")
}

# Test nested structure
sus i drip = 0
bestie (i < 2) {
    ready (i == 0) {
        vibez.spill("First iteration")
    } otherwise {
        vibez.spill("Other iteration")
    }
    i = i + 1
}

vibez.spill("Test completed")
