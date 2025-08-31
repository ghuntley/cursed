// Basic builtin functions test
// Tests: Simple arithmetic and yap calls without imports
// Expected: Both interpreter and compiled mode should produce identical output

fn main() {
    yap("=== Basic Builtin Test ===")
    
    // Test 1: Simple arithmetic
    let result1 = 5 + 3
    yap("5 + 3 =")
    yap(result1)
    
    // Test 2: More arithmetic  
    let result2 = 10 - 4
    yap("10 - 4 =")
    yap(result2)
    
    // Test 3: Multiplication
    let result3 = 6 * 7
    yap("6 * 7 =")
    yap(result3)
    
    // Test 4: Complex expression
    let result4 = 2 + 3 * 4
    yap("2 + 3 * 4 =")
    yap(result4)
    
    yap("=== Basic Builtin Test Complete ===")
}
