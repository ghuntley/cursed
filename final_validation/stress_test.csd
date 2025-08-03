slay stress_computation() {
    sus total drip = 0
    sus outer drip = 0
    bestie (outer < 1000) {
        sus inner drip = 0
        bestie (inner < 100) {
            total = total + (outer * inner)
            inner = inner + 1
        }
        outer = outer + 1
    }
    vibez.spill("Stress test result:", total)
}

stress_computation()
