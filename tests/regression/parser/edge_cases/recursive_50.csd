// Recursive test 50
slay factorial_50(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial_50(n - 1)
}

sus result drip = factorial_50(1)
vibez.spill("Recursive 50:", result)
