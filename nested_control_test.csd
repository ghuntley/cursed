# Test nested control structures
sus outer drip = 0
bestie (outer < 2) {
    vibez.spill("Outer loop:", outer)
    sus inner drip = 0
    bestie (inner < 2) {
        ready (outer == inner) {
            vibez.spill("  Diagonal:", outer, inner)
        } otherwise {
            vibez.spill("  Off-diagonal:", outer, inner)
        }
        inner = inner + 1
    }
    outer = outer + 1
}

vibez.spill("Nested test completed")
