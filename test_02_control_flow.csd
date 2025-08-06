# Test 2: Control Flow (if/else, loops)
sus x drip = 10

# If statement test
lowkey (x > 0) {
    vibez.spill("x is positive")
} highkey {
    vibez.spill("x is not positive")
}

# Loop test
sus i drip = 0
bestie (i < 3) {
    vibez.spill("Loop iteration:")
    vibez.spill(i)
    i = i + 1
}

# For-style loop
bestie (sus j drip = 0; j < 3; j = j + 1) {
    vibez.spill("For-style loop:")
    vibez.spill(j)
}
