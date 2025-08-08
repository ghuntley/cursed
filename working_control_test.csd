# Working control structures test
vibez.spill("=== CURSED Control Structures Test ===")

# Test 1: Basic if statement
ready (5 > 3) { 
    vibez.spill("✓ If statement works") 
}

# Test 2: If/else statement
sus number drip = 42
ready (number > 50) {
    vibez.spill("Number is large")
} otherwise {
    vibez.spill("✓ Number is not large")
}

# Test 3: While loop (bestie)
sus count drip = 0
bestie (count < 3) {
    vibez.spill("✓ Count:", count)
    count = count + 1
}

# Test 4: Simple nested if
sus value drip = 8
ready (value > 5) {
    vibez.spill("✓ Value > 5")
    ready (value < 10) {
        vibez.spill("✓ Value < 10")
    }
}

# Test 5: Boolean conditions
ready (based) {
    vibez.spill("✓ Boolean 'based' works")
}

ready (cringe) {
    vibez.spill("This shouldn't print")
} otherwise {
    vibez.spill("✓ Boolean 'cringe' else clause works")
}

vibez.spill("=== All tests completed successfully! ===")
