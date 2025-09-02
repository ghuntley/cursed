vibe main
yeet "vibez"
yeet "mathz"

slay main_character() {
    // Test direct arithmetic overflow
    sus max_int drip = 2147483647
    sus result1 drip = max_int + 1
    vibez.spill("Direct overflow (should promote to float):")
    vibez.spill(result1)
    
    // Test mathz.add function overflow
    sus result2 drip = mathz.add(max_int, 1)
    vibez.spill("Mathz add overflow (should promote to float):")
    vibez.spill(result2)
    
    // Test subtraction overflow (min int - 1)
    sus min_int drip = -2147483648
    sus result3 drip = min_int - 1
    vibez.spill("Subtraction overflow:")
    vibez.spill(result3)
    
    // Test multiplication overflow
    sus big_num drip = 1000000
    sus result4 drip = big_num * 3000
    vibez.spill("Multiplication overflow:")
    vibez.spill(result4)
    
    // Test abs of minimum int
    sus result5 drip = mathz.abs_normie(min_int)
    vibez.spill("Abs of min int (should promote to float):")
    vibez.spill(result5)
    
    // Test unary minus overflow
    sus result6 drip = -min_int
    vibez.spill("Unary minus of min int:")
    vibez.spill(result6)
}
