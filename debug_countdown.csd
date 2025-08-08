slay countdown(n drip) drip {
    vibez.spill("countdown:", n)
    ready (n <= 0) {
        vibez.spill("base case reached")
        damn 0
    } otherwise {
        vibez.spill("recursive case, calling countdown with:", n - 1)
        damn countdown(n - 1)
    }
}

vibez.spill("Starting countdown test")
sus result drip = countdown(3)
vibez.spill("Final result:", result)
