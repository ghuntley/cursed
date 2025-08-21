// CURSED Optimization Validation Test
// Tests function inlining, dead code elimination, and constant propagation

yeet "vibez"
yeet "mathz"

// Test constants for propagation
sus MAGIC_NUMBER drip = 42
sus MESSAGE tea = "optimization test"

// Small function for inlining test
slay small_function(x drip) drip {
    damn x * MAGIC_NUMBER
}

// Function with dead code for elimination
slay function_with_dead_code(flag lit) drip {
    sus result drip = 10
    
    ready (flag) {
        result = result + 5
    } otherwise {
        result = result + 3
    }
    
    // Dead code - never executed
    ready (based) {
        sus dead_var drip = 999
        dead_var = dead_var * 2
    }
    
    damn result
}

// Function with constant folding opportunities  
slay constant_folding_test() drip {
    sus a drip = 2 + 3  // Should fold to 5
    sus b drip = a * 4  // Should fold to 20
    sus c drip = b - 10 // Should fold to 10
    
    damn c
}

// Main function to test optimizations
slay main() drip {
    vibez.spill("Starting optimization test...")
    
    // Test inlining
    sus result1 drip = small_function(10)  // Should inline
    vibez.spill("Inlined function result:", result1)
    
    // Test dead code elimination
    sus result2 drip = function_with_dead_code(based)
    vibez.spill("Function with dead code result:", result2)
    
    // Test constant propagation and folding
    sus result3 drip = constant_folding_test()
    vibez.spill("Constant folding result:", result3)
    
    // Test constant propagation
    vibez.spill("Message:", MESSAGE)
    
    damn 0
}
