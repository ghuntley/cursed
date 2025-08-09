// Performance test program for CURSED compiler optimizations
// Tests various language features to measure compilation and runtime speed

yeet "mathz"
yeet "stringz" 
yeet "arrayz"
yeet "cryptz"

// Test function definitions and calls
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    } otherwise {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// Test array operations
slay test_arrays() drip {
    sus numbers []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus total drip = 0
    
    sus i drip = 0
    bestie (i < len(numbers)) {
        total = total + numbers[i]
        i = i + 1
    }
    
    damn total
}

// Test string operations
slay test_strings() tea {
    sus message tea = "Hello, CURSED!"
    sus processed tea = slice_tea(message, 0, 5)
    damn concat_tea(processed, " World")
}

// Test pattern matching
slay test_patterns(value drip) tea {
    ready (value) {
        1 => damn "one"
        2 => damn "two" 
        3 => damn "three"
        10..20 => damn "teen"
        _ => damn "other"
    }
}

// Test concurrency (if supported)
slay test_concurrency() drip {
    // Simple goroutine test
    spawn {
        vibez.spill("Goroutine 1")
    }
    
    spawn {
        vibez.spill("Goroutine 2")
    }
    
    damn 42
}

// Test mathematical operations
slay test_math() drip {
    sus result drip = abs_normie(-42)
    result = result + power_normie(2, 8)
    result = result * sqrt_normie(16)
    damn result
}

// Test cryptographic operations
slay test_crypto() tea {
    sus data tea = "sensitive data"
    sus hash tea = sha256_hash(data)
    damn hash
}

// Main function testing all features
slay main() drip {
    vibez.spill("Starting performance test...")
    
    // Test arithmetic and recursion
    sus fib_result drip = fibonacci(10)
    vibez.spill("Fibonacci(10):", fib_result)
    
    // Test arrays
    sus array_sum drip = test_arrays()
    vibez.spill("Array sum:", array_sum)
    
    // Test strings
    sus string_result tea = test_strings()
    vibez.spill("String result:", string_result)
    
    // Test pattern matching
    sus pattern_result tea = test_patterns(15)
    vibez.spill("Pattern result:", pattern_result)
    
    // Test mathematical operations
    sus math_result drip = test_math()
    vibez.spill("Math result:", math_result)
    
    // Test cryptographic operations
    sus crypto_result tea = test_crypto()
    vibez.spill("Crypto hash:", crypto_result)
    
    // Test concurrency
    sus concurrency_result drip = test_concurrency()
    vibez.spill("Concurrency result:", concurrency_result)
    
    vibez.spill("Performance test completed!")
    damn 0
}

// Entry point
main()
