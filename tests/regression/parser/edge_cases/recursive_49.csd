// Recursive test 49
slay factorial_49(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial_49(n - 1)
}

sus result drip = factorial_49(10)
vibez.spill("Recursive 49:", result)
