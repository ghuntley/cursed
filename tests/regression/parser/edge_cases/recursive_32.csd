// Recursive test 32
slay factorial_32(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial_32(n - 1)
}

sus result drip = factorial_32(3)
vibez.spill("Recursive 32:", result)
