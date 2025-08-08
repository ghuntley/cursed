// Test pattern matching
slay test_match(value drip) {
    ready (value) {
        1 => vibez.spill("One")
        2 => vibez.spill("Two")
        3 => vibez.spill("Three")
        _ => vibez.spill("Other:", value)
    }
}

slay fibonacci(n drip) drip {
    ready (n) {
        0 => damn 0
        1 => damn 1
        _ => damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

slay main() {
    test_match(1)
    test_match(2)
    test_match(5)
    
    vibez.spill("Fibonacci 0:", fibonacci(0))
    vibez.spill("Fibonacci 5:", fibonacci(5))
    vibez.spill("Fibonacci 8:", fibonacci(8))
}

main()
