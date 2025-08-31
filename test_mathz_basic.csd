// Basic mathz stdlib test - simple function calls
// Tests: mathz.add_two() with different inputs
// Expected: Both interpreter and compiled mode should produce identical output

import mathz

fn main() {
    yap("=== Basic Mathz Stdlib Test ===")
    
    // Test 1: Simple addition
    let result1 = mathz.add_two(5, 3)
    yap("mathz.add_two(5, 3) =")
    yap(result1)
    
    // Test 2: Zero handling
    let result2 = mathz.add_two(0, 7)
    yap("mathz.add_two(0, 7) =")
    yap(result2)
    
    // Test 3: Negative numbers
    let result3 = mathz.add_two(-2, 8)
    yap("mathz.add_two(-2, 8) =")
    yap(result3)
    
    // Test 4: Both negative
    let result4 = mathz.add_two(-3, -4)
    yap("mathz.add_two(-3, -4) =")
    yap(result4)
    
    yap("=== Basic Mathz Test Complete ===")
}
