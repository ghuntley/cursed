// Recursive test 37
slay factorial_37(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial_37(n - 1)
}

sus result drip = factorial_37(8)
vibez.spill("Recursive 37:", result)
