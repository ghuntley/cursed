// Test case to identify LLVM optimizer crashes with generic parameters

// Test 1: Large const generic values that could overflow
slay large_array<const N: drip>(data: [N]tea) tea {
    ready (N > 0) {
        damn data[0]
    } otherwise {
        damn "empty"
    }
}

// Test 2: Nested const generics with complex bounds
slay nested_generic<const OUTER: drip, const INNER: drip>(matrix: [OUTER][INNER]drip) drip {
    sus sum drip = 0
    bestie (i = 0; i < OUTER; i += 1) {
        bestie (j = 0; j < INNER; j += 1) {
            sum += matrix[i][j]
        }
    }
    damn sum
}

// Test 3: Const generic with potentially problematic arithmetic
slay arithmetic_const<const A: drip, const B: drip>() drip {
    const RESULT = A * B * A + B - A // Could overflow
    damn RESULT
}

// Test 4: Zero-sized and negative const generics (should be caught)
slay edge_cases<const SIZE: drip>() drip {
    ready (SIZE <= 0) {
        // This should trigger bounds checking
        damn -1
    }
    damn SIZE
}

// Test 5: Very large const generic that might cause optimizer issues
slay potential_ice<const HUGE: drip>() drip {
    const CALCULATED = HUGE * HUGE * HUGE // Potential overflow
    damn CALCULATED
}

slay main() drip {
    // Normal cases that should work
    sus test_data [3]tea = ["a", "b", "c"]
    sus result1 tea = large_array(test_data)
    
    sus matrix [2][2]drip = [[1, 2], [3, 4]]
    sus result2 drip = nested_generic(matrix)
    
    // Test arithmetic const generics
    sus result3 drip = arithmetic_const<5, 3>()
    
    // Test edge cases (should be handled safely)
    sus result4 drip = edge_cases<0>()
    
    // These would potentially cause optimizer ICE if bounds checking fails:
    // sus ice_test1 drip = potential_ice<2147483647>()  // Max i32 
    // sus ice_test2 drip = arithmetic_const<100000, 100000>()  // Large multiplication
    // sus ice_test3 tea = large_array<-5>(test_data)  // Negative size
    
    damn result1.len + result2 + result3 + result4
}
