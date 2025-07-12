// Test complex_vibe module
vibez.spill("🧪 Testing complex_vibe functions...")

// Import all complex_vibe functions directly for testing
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
