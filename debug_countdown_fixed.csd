slay countdown(n drip) drip {
    vibez.spill("countdown:", n)
    ready (n <= 0) { damn 0 } otherwise { damn countdown(n - 1) }
}

vibez.spill("Starting countdown test")
sus result drip = countdown(3)
vibez.spill("Final result:", result)
