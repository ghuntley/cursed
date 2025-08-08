slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    } otherwise {
        damn n * factorial(n - 1)
    }
}
vibez.spill("factorial(5) =", factorial(5))
vibez.spill("factorial(0) =", factorial(0))
vibez.spill("factorial(1) =", factorial(1))
