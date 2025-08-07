// Working pattern tests that use current infrastructure

// Test variable binding
sus x drip = 42
sus y drip = x
vibez.spill("Bound value:", y)

// Test array access (tuple-like)  
sus array [drip] = [1, 2, 3]
sus first drip = array[0]
sus second drip = array[1]
vibez.spill("First:", first, "Second:", second)

// Test conditional patterns using ready
sus value drip = 15
ready (value > 10) {
    vibez.spill("Large value:", value)
} otherwise {
    vibez.spill("Small value") 
}

// Test multiple conditions
ready (value == 42) {
    vibez.spill("Answer!")
} otherwise ready (value > 20) {
    vibez.spill("Greater than 20")
} otherwise {
    vibez.spill("Something else:", value)
}
