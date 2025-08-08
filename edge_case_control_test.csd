# Test boolean literals
ready (based) {
    vibez.spill("Boolean true works")
}

ready (cringe) {
    vibez.spill("This should not print")
} otherwise {
    vibez.spill("Boolean false works")
}

# Test empty blocks
ready (based) {
}

# Test single line if
ready (based) { vibez.spill("Single line if works") }

# Test zero iterations
sus zero drip = 0
bestie (zero > 0) {
    vibez.spill("This should not print")
}

# Test condition with variable comparison
sus val1 drip = 5
sus val2 drip = 10
ready (val1 < val2) {
    vibez.spill("Comparison works")
}
