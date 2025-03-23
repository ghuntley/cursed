fr fr Fibonacci sequence calculator in CURSED

vibe main

yeet (
    "vibez"
    "stringz"
    "vibe_life"
)

fr fr Calculate fibonacci recursively
slay fib(n normie) normie {
    lowkey n <= 1 {
        yolo n
    }
    yolo fib(n-1) + fib(n-2)
}

fr fr Calculate fibonacci iteratively
slay fib_fast(n normie) normie {
    lowkey n <= 1 {
        yolo n
    }
    
    sus a normie = 0
    sus b normie = 1
    sus c normie = 0
    
    bestie i := 2; i <= n; i++ {
        c = a + b
        a = b
        b = c
    }
    
    yolo b
}

slay main() {
    fr fr Get command line argument or use default
    sus n normie = 10
    
    lowkey len(vibe_life.Args) > 1 {
        sus arg tea = vibe_life.Args[1]
        sus parsed, err = stringz.Atoi(arg)
        
        lowkey err == cap {
            n = parsed
        } highkey {
            vibez.spill("Invalid input, using default n =", n)
        }
    }
    
    vibez.spill("Calculating Fibonacci numbers:")
    
    fr fr Print fibonacci numbers
    bestie i := 0; i <= n; i++ {
        lowkey i < 20 {
            fr fr Use recursive for small numbers
            vibez.spillf("fib(%d) = %d\n", i, fib(i))
        } highkey {
            fr fr Use iterative for larger numbers
            vibez.spillf("fib(%d) = %d\n", i, fib_fast(i))
        }
    }
    
    fr fr Performance comparison
    vibez.spill("\nPerformance test:")
    
    sus test_n normie = 30
    
    vibez.spillf("Computing fib(%d)...\n", test_n)
    result := fib_fast(test_n)
    vibez.spillf("Result: %d\n", result)
} 