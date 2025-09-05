// Test file to debug LLVM binary operations
// This should work in interpreter but currently fails in compiled mode

sus main() -> i32 {
    // Test simple arithmetic
    sus a drip = 5
    sus b drip = 3
    sus result drip = a + b
    
    yap("Testing binary operations:")
    yap("5 + 3 should be 8")
    yap(result)
    
    // Test direct arithmetic  
    sus direct_result drip = 5 + 3
    yap("Direct 5 + 3:")
    yap(direct_result)
    
    // Test multiplication
    sus multiply_result drip = a * b
    yap("5 * 3 should be 15")
    yap(multiply_result)
    
    // Test subtraction
    sus subtract_result drip = a - b
    yap("5 - 3 should be 2") 
    yap(subtract_result)
    
    return 0
}
