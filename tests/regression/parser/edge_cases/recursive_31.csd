// Recursive test 31
slay factorial_31(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial_31(n - 1)
}

sus result drip = factorial_31(2)
vibez.spill("Recursive 31:", result)
