// Recursive functions to test stack and heap management
slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay build_list(n drip) []drip {
    ready (n <= 0) {
        damn []
    }
    sus rest []drip = build_list(n - 1)
    damn append_drip(rest, n)
}

sus fact_result drip = factorial(10)
sus fib_result drip = fibonacci(10)
sus list_result []drip = build_list(5)

vibez.spill("Factorial(10):", fact_result)
vibez.spill("Fibonacci(10):", fib_result)
vibez.spill("List:", list_result)
