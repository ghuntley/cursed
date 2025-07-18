// Test program to verify LTO implementation
vibez.spill("Testing Link-Time Optimization implementation...")

// Define some functions for LTO testing
slay small_function(x normie) normie {
    damn x + 1
}

slay another_small_function(y normie) normie {
    damn y * 2
}

slay large_function(a normie, b normie, c normie) normie {
    sus result normie = a + b + c
    result = result * 2
    result = result + 100
    result = result / 3
    damn result
}

slay dead_function(unused normie) normie {
    // This function should be eliminated by dead code elimination
    damn unused * 42
}

slay recursive_function(n normie) normie {
    lowkey n <= 1 {
        damn 1
    }
    damn n * recursive_function(n - 1)
}

slay main() {
    vibez.spill("Starting LTO test...")
    
    // Test small function calls (should be inlined)
    sus result1 normie = small_function(5)
    sus result2 normie = another_small_function(10)
    
    // Test large function call (should not be inlined)
    sus result3 normie = large_function(1, 2, 3)
    
    // Test recursive function (should not be inlined)
    sus result4 normie = recursive_function(5)
    
    vibez.spill("Small function result: ", result1)
    vibez.spill("Another small function result: ", result2)
    vibez.spill("Large function result: ", result3)
    vibez.spill("Recursive function result: ", result4)
    
    vibez.spill("LTO test completed successfully!")
    damn 0
}
