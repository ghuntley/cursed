// Test file to debug LLVM stdlib function calls
// This should work in interpreter but currently returns wrong values in compiled mode

sus main() -> i32 {
    // Test mathz module function call
    yap("Testing stdlib function calls:")
    yap("mathz.add_two(5, 3) should return 8")
    
    sus result drip = mathz.add_two(5, 3) 
    yap(result)
    
    // Test another mathz function
    yap("mathz.multiply(4, 7) should return 28")
    sus multiply_result drip = mathz.multiply(4, 7)
    yap(multiply_result)
    
    // Test stringz function
    yap("Testing stringz.concat:")
    sus greeting drip = stringz.concat("Hello", " World")
    yap(greeting)
    
    return 0
}
