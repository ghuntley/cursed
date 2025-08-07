slay test_with_params(x drip, y tea) drip {
    vibez.spill("x:", x, "y:", y)
    damn x + 5
}

sus result drip = test_with_params(10, "hello")
vibez.spill("Result:", result)

slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    } otherwise {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

vibez.spill("Fibonacci 6:", fibonacci(6))
