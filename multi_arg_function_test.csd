// Test multi-argument function calls
slay add(a drip, b drip) drip {
    damn a + b
}

slay multiply(x drip, y drip, z drip) drip {
    damn x * y * z
}

slay main_character() {
    vibez.spill("Testing multi-argument functions")
    
    sus result1 drip = add(5, 3)
    vibez.spill("add(5, 3) =", result1)
    
    sus result2 drip = multiply(2, 3, 4)
    vibez.spill("multiply(2, 3, 4) =", result2)
    
    // Test with variables
    sus a drip = 10
    sus b drip = 20
    sus c drip = 5
    
    sus result3 drip = add(a, b)
    vibez.spill("add(10, 20) =", result3)
    
    sus result4 drip = multiply(a, b, c)
    vibez.spill("multiply(10, 20, 5) =", result4)
}
