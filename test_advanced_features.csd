yeet "vibez"
yeet "mathz"
yeet "stringz"

slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay test_control_flow() {
    sus x drip = 5
    ready (x > 3) {
        vibez.spill("x is greater than 3:", x)
    } otherwise {
        vibez.spill("x is not greater than 3:", x)
    }
    
    sus i drip = 0
    bestie (i < 3) {
        vibez.spill("Loop iteration:", i)
        i = i + 1
    }
}

slay test_functions() {
    sus fib_result drip = fibonacci(8)
    vibez.spill("Fibonacci(8) =", fib_result)
    
    sus pi drip = mathz.PI
    sus text tea = "Testing CURSED"
    sus length drip = stringz.len_string(text)
    
    vibez.spill("PI =", pi)
    vibez.spill("Text:", text, "Length:", length)
}

slay main() {
    vibez.spill("=== CURSED Advanced Features Test ===")
    
    test_control_flow()
    test_functions()
    
    vibez.spill("=== All advanced features working! ===")
}
