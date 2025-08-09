// Recursive test 35
slay factorial_35(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial_35(n - 1)
}

sus result drip = factorial_35(6)
vibez.spill("Recursive 35:", result)
