// Complex math expressions combining arithmetic and stdlib
// Tests: Mixed operations with mathz functions
// Expected: Both modes should handle complex expressions identically

import mathz

fn main() {
    yap("=== Complex Math Expression Test ===")
    
    // Test 1: Simple combination
    let base = 2
    let stdlib_result = mathz.add_two(4, 5)
    let combined = base + stdlib_result
    yap("2 + mathz.add_two(4, 5) =")
    yap(combined)
    
    // Test 2: Multiple operations
    let val1 = mathz.add_two(3, 2)
    let val2 = mathz.add_two(1, 4)
    let final_result = val1 + val2
    yap("mathz.add_two(3, 2) + mathz.add_two(1, 4) =")
    yap(final_result)
    
    // Test 3: Nested with multiplication
    let nested_val = mathz.add_two(2, 3)
    let multiplied = nested_val * 2
    yap("mathz.add_two(2, 3) * 2 =")
    yap(multiplied)
    
    // Test 4: Chain operations
    let step1 = mathz.add_two(1, 1)
    let step2 = mathz.add_two(step1, 3)
    let step3 = mathz.add_two(step2, 2)
    yap("Chain: mathz.add_two(mathz.add_two(mathz.add_two(1, 1), 3), 2) =")
    yap(step3)
    
    yap("=== Complex Math Test Complete ===")
}
