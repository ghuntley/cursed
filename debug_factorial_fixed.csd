slay factorial(n drip) drip {
    vibez.spill("factorial:", n)
    ready (n <= 1) { damn 1 } otherwise { damn n * factorial(n - 1) }
}

vibez.spill("Testing factorial")
sus result drip = factorial(5)
vibez.spill("5! =", result)

sus result2 drip = factorial(0)
vibez.spill("0! =", result2)

sus result3 drip = factorial(3)
vibez.spill("3! =", result3)
