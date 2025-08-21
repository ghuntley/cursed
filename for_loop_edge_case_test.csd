yeet "vibez"

vibez.spill("Testing CURSED for loop edge cases:")

# Test for loop with problematic range calculation
bestie x := 0; x < 1; x++ {
    vibez.spill("Single iteration loop:", x)
}

# Test empty loop body - this might trigger the range issue
bestie y := 10; y < 10; y++ {
    # This should not execute at all
    vibez.spill("Should not print")
}

# Test loop with immediate exit condition
bestie z := 5; z < 5; z++ {
    vibez.spill("Should also not print")
}

vibez.spill("Edge case testing completed!")
