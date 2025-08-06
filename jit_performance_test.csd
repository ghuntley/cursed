// JIT Performance Test - Tests tier-up behavior and optimization

slay fibonacci(n normie) normie {
    lowkey (n <= 1) {
        damn n
    } highkey {
        damn fibonacci(n - 1) + fibonacci(n - 2)
    }
}

slay string_concatenation_test() {
    sus result tea = ""
    sus i normie = 0
    bestie (i < 100) {
        result = result + "Hello"
        i = i + 1
    }
    damn result
}

slay mathematical_operations() {
    sus sum normie = 0
    sus i normie = 0
    bestie (i < 10000) {
        sum = sum + (i * 2 + 3) / 2
        i = i + 1
    }
    damn sum
}

slay array_operations() {
    sus numbers [10]normie = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    sus sum normie = 0
    sus i normie = 0
    bestie (i < 10) {
        sum = sum + numbers[i]
        i = i + 1
    }
    damn sum
}

slay hot_function() normie {
    // This function will be called many times to trigger tier-up
    sus x normie = 42
    sus y normie = x * 2 + 1
    damn y % 10
}

slay main() {
    vibez.spill("🚀 Starting JIT Performance Tests")
    
    // Test fibonacci with multiple calls to trigger optimization
    vibez.spill("Testing Fibonacci (tier-up trigger)...")
    sus i normie = 0
    bestie (i < 50) {
        sus result normie = fibonacci(i % 10)  // Keep numbers small
        lowkey (i % 10 == 0) {
            vibez.spillf("fib({}) = {}", i % 10, result)
        }
        i = i + 1
    }
    
    // Test string concatenation performance
    vibez.spill("Testing String Concatenation...")
    sus string_result tea = string_concatenation_test()
    vibez.spillf("String concat length: {}", string_result.len)
    
    // Test mathematical operations
    vibez.spill("Testing Mathematical Operations...")
    sus math_result normie = mathematical_operations()
    vibez.spillf("Math result: {}", math_result)
    
    // Test array operations
    vibez.spill("Testing Array Operations...")
    sus array_result normie = array_operations()
    vibez.spillf("Array sum: {}", array_result)
    
    // Call hot function many times to trigger tier-up
    vibez.spill("Testing Hot Function (tier-up optimization)...")
    sus hot_calls normie = 0
    bestie (hot_calls < 1000) {
        sus hot_result normie = hot_function()
        lowkey (hot_calls % 100 == 0) {
            vibez.spillf("Hot function call {}: {}", hot_calls, hot_result)
        }
        hot_calls = hot_calls + 1
    }
    
    vibez.spill("⚡ JIT Performance Tests Complete")
}

main()
