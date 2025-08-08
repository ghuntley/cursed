sus level drip = 5

ready (level > 0) {
    vibez.spill("Level is positive")
    ready (level > 3) {
        vibez.spill("Level is greater than 3")
    } otherwise {
        vibez.spill("Level is 3 or less")
    }
}

fr fr Test with no else block
sus simple drip = 2
ready (simple > 1) {
    vibez.spill("Simple test passed")
}

fr fr Test with complex expressions
sus x drip = 15
sus y drip = 10
ready ((x - y) * 2 > 8) {
    vibez.spill("Complex expression works")
}
