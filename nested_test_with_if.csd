# Test nested loop with if condition
sus outer drip = 0
bestie (outer < 2) {
    vibez.spill("Outer loop:", outer)
    sus inner drip = 0
    bestie (inner < 2) {
        vibez.spill("Inner loop:", outer, inner)
        ready (outer == 1) {
            vibez.spill("  Special case")
        }
        inner = inner + 1
    }
    outer = outer + 1
}

vibez.spill("Test completed")
