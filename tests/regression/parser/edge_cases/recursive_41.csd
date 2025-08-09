// Recursive test 41
slay factorial_41(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial_41(n - 1)
}

sus result drip = factorial_41(2)
vibez.spill("Recursive 41:", result)
