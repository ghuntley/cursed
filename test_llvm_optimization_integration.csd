// CURSED program to test LLVM optimization integration
// This program will be compiled with different optimization levels

vibez.spill("Testing LLVM Optimization Integration")

// Define a simple function with optimization opportunities
slay fibonacci(n normie) normie {
    stan n <= 1 {
        damn 1
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

// Function with constant propagation opportunities
slay constant_folding_test() normie {
    sus a normie = 5
    sus b normie = 10
    sus c normie = a + b * 2  // Should be optimized to 25
    damn c
}

// Function with dead code elimination opportunities
slay dead_code_test() normie {
    sus result normie = 42
    
    // Dead code that should be eliminated
    sus unused normie = 100
    unused = unused + 50
    
    // More dead code
    stan cap {
        vibez.spill("This should be eliminated")
    }
    
    damn result
}

// Function with loop optimization opportunities
slay loop_optimization_test() normie {
    sus sum normie = 0
    
    // Loop that can be unrolled
    bestie i := 0; i < 10; i++ {
        sum = sum + i
    }
    
    damn sum
}

// Function with inlining opportunities
slay inline_candidate(x normie) normie {
    damn x * 2  // Simple function, good for inlining
}

slay inlining_test() normie {
    sus result normie = inline_candidate(21)
    damn result
}

// Main function to test all optimizations
slay main() normie {
    vibez.spill("Running optimization tests...")
    
    // Test constant folding
    sus const_result normie = constant_folding_test()
    vibez.spill("Constant folding result: " + const_result)
    
    // Test dead code elimination
    sus dce_result normie = dead_code_test()
    vibez.spill("Dead code elimination result: " + dce_result)
    
    // Test loop optimization
    sus loop_result normie = loop_optimization_test()
    vibez.spill("Loop optimization result: " + loop_result)
    
    // Test function inlining
    sus inline_result normie = inlining_test()
    vibez.spill("Inlining result: " + inline_result)
    
    // Test recursive function (good for tail call optimization)
    sus fib_result normie = fibonacci(10)
    vibez.spill("Fibonacci result: " + fib_result)
    
    vibez.spill("All optimization tests completed!")
    damn 0
}
