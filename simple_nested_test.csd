# Test simple nested loop
sus outer drip = 0
bestie (outer < 1) {
    vibez.spill("In outer loop")
    sus inner drip = 0
    bestie (inner < 1) {
        vibez.spill("In inner loop")
        inner = inner + 1
    }
    outer = outer + 1
}

vibez.spill("Done")
