slay count_down(n drip) drip {
    ready (n <= 1) { damn 1 }
    damn count_down(n - 1)
}

vibez.spill("count_down(3) =", count_down(3))
