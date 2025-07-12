// Test complex_vibe module directly without imports
vibez.spill("🧪 Testing complex_vibe functions directly...")

// Import all complex_vibe functions directly
slay ComplexAdd(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    damn r1 + r2
}

slay ComplexAddImag(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    damn i1 + i2
}

slay ComplexMul(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    damn r1 * r2 - i1 * i2
}

slay ComplexMulImag(r1 meal, i1 meal, r2 meal, i2 meal) meal {
    damn r1 * i2 + i1 * r2
}

slay ComplexAbs(r meal, i meal) meal {
    damn mathz.Sqrt(r * r + i * i)
}

slay Sin(x meal) meal {
    sus result meal = x
    sus term meal = x
    sus x_squared meal = x * x
    
    term = term * (0.0 - x_squared) / 6.0
    result = result + term
    
    term = term * (0.0 - x_squared) / 20.0
    result = result + term
    
    damn result
}

slay Cos(x meal) meal {
    sus result meal = 1.0
    sus term meal = 1.0
    sus x_squared meal = x * x
    
    term = term * (0.0 - x_squared) / 2.0
    result = result + term
    
    term = term * (0.0 - x_squared) / 12.0
    result = result + term
    
    damn result
}

// Test basic operations
vibez.spill("📐 Testing Complex Number Operations:")

sus r1 meal = 3.0
sus i1 meal = 4.0
sus r2 meal = 1.0
sus i2 meal = 2.0

vibez.spill("Complex number 1: " + r1.(tea) + " + " + i1.(tea) + "i")
vibez.spill("Complex number 2: " + r2.(tea) + " + " + i2.(tea) + "i")

// Test addition
sus sum_r meal = ComplexAdd(r1, i1, r2, i2)
sus sum_i meal = ComplexAddImag(r1, i1, r2, i2)
vibez.spill("Sum: " + sum_r.(tea) + " + " + sum_i.(tea) + "i")

// Test multiplication
sus prod_r meal = ComplexMul(r1, i1, r2, i2)
sus prod_i meal = ComplexMulImag(r1, i1, r2, i2)
vibez.spill("Product: " + prod_r.(tea) + " + " + prod_i.(tea) + "i")

// Test absolute value
sus abs_z1 meal = ComplexAbs(r1, i1)
vibez.spill("Absolute value: " + abs_z1.(tea))

// Test trigonometric functions
sus sin_val meal = Sin(1.5708)
sus cos_val meal = Cos(0.0)
vibez.spill("sin(π/2): " + sin_val.(tea))
vibez.spill("cos(0): " + cos_val.(tea))

// Test mathematical identity
sus expected_sum_r meal = 4.0
sus expected_sum_i meal = 6.0
sus add_correct lit = (sum_r == expected_sum_r) && (sum_i == expected_sum_i)
vibez.spill("Addition correct: " + add_correct.(tea))

sus expected_prod_r meal = -5.0
sus expected_prod_i meal = 10.0
sus mul_correct lit = (prod_r == expected_prod_r) && (prod_i == expected_prod_i)
vibez.spill("Multiplication correct: " + mul_correct.(tea))

vibez.spill("🎉 Complex vibe testing complete!")
