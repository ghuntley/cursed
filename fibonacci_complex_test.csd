slay fibonacci_complex(n drip) drip {
    ready (n <= 0) {
        damn 0
    }
    ready (n == 1) {
        damn 1
    }
    
    sus fib1 drip = fibonacci_complex(n - 1)
    sus fib2 drip = fibonacci_complex(n - 2)
    sus result drip = fib1 + fib2
    
    damn result
}

vibez.spill("Complex Fibonacci test starting...")
sus result drip = fibonacci_complex(8)
vibez.spill("Fibonacci(8) =", result)

# Test with multiple variables and deeper recursion
sus test1 drip = fibonacci_complex(6)
sus test2 drip = fibonacci_complex(7)
vibez.spill("Fibonacci(6) =", test1)
vibez.spill("Fibonacci(7) =", test2)
