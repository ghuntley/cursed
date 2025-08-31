// Comprehensive test of working CURSED features in both modes
sus a drip = 5
sus b drip = 3

// Test basic arithmetic
sus add_result drip = a + b
sus sub_result drip = a - b  
sus mul_result drip = a * b

// Test stdlib function
sus mathz_result drip = mathz.add_two(10, 15)

// Test multiple stdlib calls
sus second_mathz drip = mathz.add_two(mathz_result, 5)
