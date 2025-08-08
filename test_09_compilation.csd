# Test 9: LLVM compilation and native binary execution
vibez.spill("Compilation test program")

slay compute_fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn compute_fibonacci(n - 1) + compute_fibonacci(n - 2)
}

slay main() {
    vibez.spill("Computing Fibonacci numbers:")
    sus i drip = 0
    bestie (i <= 8) {
        sus fib drip = compute_fibonacci(i)
        vibez.spill("Fibonacci", i, "=", fib)
        i = i + 1
    }
    
    squad Point {
        spill x drip
        spill y drip
    }
    
    sus p Point = Point{x: 42, y: 84}
    vibez.spill("Point created:", p.x, p.y)
    
    vibez.spill("Native compilation test completed!")
}

main()
