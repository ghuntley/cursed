# Simple Functions Benchmark
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay main_character() drip {
    sus iterations drip = 25
    sus total drip = 0
    
    bestie (sus i drip = 0; i < iterations; i++) {
        total = total + fibonacci(i)
        total = total + factorial(i % 8)
    }
    
    vibez.spill("Functions result:", total)
    damn 0
}
