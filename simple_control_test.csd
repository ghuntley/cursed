# Test basic if statement
ready (5 > 3) {
    vibez.spill("Basic if works")
}

# Test if/else with variable
sus x drip = 7
ready (x > 10) {
    vibez.spill("x is large")
} otherwise {
    vibez.spill("x is small")
}

# Test while loop
sus counter drip = 0
bestie (counter < 3) {
    vibez.spill("Counter:", counter)
    counter = counter + 1
}

# Test nested if
sus y drip = 4
ready (y > 2) {
    ready (y < 6) {
        vibez.spill("y is between 2 and 6")
    }
}
