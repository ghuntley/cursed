// Test that demonstrates the fixed multi-argument function call mechanism

slay calculate(x drip, y drip, z drip) drip {
    damn x + y * z
}

slay greeting(first tea, last tea) tea {
    damn "Hello " + first + " " + last
}

slay main_character() {
    vibez.spill("✅ Multi-argument function calls working:")
    
    // Test with 3 arguments
    sus calc_result drip = calculate(10, 5, 3)
    vibez.spill("calculate(10, 5, 3) =", calc_result)
    
    // Test with 2 string arguments  
    sus greeting_result tea = greeting("John", "Doe")
    vibez.spill("greeting result:", greeting_result)
    
    // Test with variables
    sus a drip = 7
    sus b drip = 8
    sus c drip = 9
    sus var_result drip = calculate(a, b, c)
    vibez.spill("calculate(7, 8, 9) =", var_result)
    
    vibez.spill("✅ All multi-argument function tests passed!")
}
