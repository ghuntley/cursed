// Recursive test 40
slay factorial_40(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial_40(n - 1)
}

sus result drip = factorial_40(1)
vibez.spill("Recursive 40:", result)
