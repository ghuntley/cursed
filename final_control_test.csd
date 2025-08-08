# Comprehensive control structures test
vibez.spill("=== CURSED Control Structures Test ===")

# Test 1: Basic if statement
vibez.spill("\n1. Basic if statement:")
ready (5 > 3) { 
    vibez.spill("✓ If statement works") 
}

# Test 2: If/else statement
vibez.spill("\n2. If/else statement:")
sus number drip = 42
ready (number > 50) {
    vibez.spill("Number is large")
} otherwise {
    vibez.spill("✓ Number is not large")
}

# Test 3: While loop (bestie)
vibez.spill("\n3. While loop with bestie:")
sus count drip = 0
bestie (count < 3) {
    vibez.spill("✓ Count:", count)
    count = count + 1
}

# Test 4: Nested structures
vibez.spill("\n4. Nested control structures:")
sus outer drip = 2
sus inner drip = 1
ready (outer > 1) {
    vibez.spill("✓ Outer condition met")
    bestie (inner <= 2) {
        vibez.spill("  ✓ Inner loop:", inner)
        inner = inner + 1
    }
}

# Test 5: Boolean conditions
vibez.spill("\n5. Boolean literal conditions:")
ready (based) {
    vibez.spill("✓ Boolean 'based' works")
}

ready (cringe) {
    vibez.spill("This shouldn't print")
} otherwise {
    vibez.spill("✓ Boolean 'cringe' else clause works")
}

vibez.spill("\n=== All tests completed successfully! ===")
