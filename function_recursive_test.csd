slay countdown(n drip) drip {
    ready (n <= 0) { damn 0 }
    damn countdown(n - 1)
}

vibez.spill("countdown(3) =", countdown(3))
