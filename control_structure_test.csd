sus x drip = 10
ready (x > 5) {
    vibez.spill("x is greater than 5")
} otherwise {
    vibez.spill("x is not greater than 5")
}

sus y drip = 3
ready (y > 5) {
    vibez.spill("y is greater than 5")
} otherwise {
    vibez.spill("y is not greater than 5")
}

fr fr Test if/else with different conditions
ready (10 == 10) {
    vibez.spill("Equal condition works")
}

ready (5 < 3) {
    vibez.spill("This should not print")
} otherwise {
    vibez.spill("Less than condition works correctly")
}
