fr fr Memory stress test with heavy variable usage
sus large_number drip = 999999
sus counter drip = 0

fr fr Create many temporary variables in a loop
bestie (counter < 50) {
    sus temp1 drip = counter * 123
    sus temp2 drip = temp1 + 456
    sus temp3 drip = temp2 - 789
    sus result drip = temp3 * 2
    
    counter = counter + 1
}

fr fr Test multiple string concatenations
vibez.spill("Testing", "multiple", "string", "arguments", "to", "stress", "test")

fr fr Test nested arithmetic
sus complex_calc drip = ((42 + 18) * 3 - 15) / 5 + 100
vibez.spill("Complex calculation result:", complex_calc)

fr fr Test variable reassignment patterns
sus x drip = 1
x = x + x
x = x + x  
x = x + x
x = x + x
x = x + x
vibez.spill("x after doubling 5 times:", x)

vibez.spill("Stress test completed - no memory leaks!")
