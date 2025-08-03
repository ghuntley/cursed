slay compute_heavy() {
    sus total drip = 0
    sus i drip = 0
    bestie (i < 100000) {
        sus j drip = 0
        bestie (j < 100) {
            total = total + (i * j)
            j = j + 1
        }
        i = i + 1
    }
    vibez.spill("Computation result:", total)
}

compute_heavy()
