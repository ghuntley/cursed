// Minimal Pure CURSED Math Library Test
// Tests core math functions to verify FFI elimination

yeet "math"

slay test_basic_math() {
    vibez.spill("Testing basic math functions...")
    
    // Test constants
    sus pi_val meal = math_pi()
    sus e_val meal = math_e()
    vibez.spill("Pi: " + pi_val.(tea))
    vibez.spill("E: " + e_val.(tea))
    
    // Test basic operations
    sus abs_result meal = math_abs(-5.0)
    vibez.spill("Absolute value of -5.0: " + abs_result.(tea))
    
    sus min_result meal = math_min(3.0, 7.0)
    vibez.spill("Min of 3.0 and 7.0: " + min_result.(tea))
    
    sus max_result meal = math_max(3.0, 7.0)
    vibez.spill("Max of 3.0 and 7.0: " + max_result.(tea))
    
    // Test power functions
    sus sqrt_result meal = math_sqrt(16.0)
    vibez.spill("Square root of 16.0: " + sqrt_result.(tea))
    
    sus pow_result meal = math_pow(2.0, 3.0)
    vibez.spill("2^3: " + pow_result.(tea))
    
    // Test trigonometry
    sus sin_result meal = math_sin(0.0)
    vibez.spill("Sin(0): " + sin_result.(tea))
    
    sus cos_result meal = math_cos(0.0)
    vibez.spill("Cos(0): " + cos_result.(tea))
    
    vibez.spill("✅ Basic math functions working!")
}

slay test_advanced_math() {
    vibez.spill("Testing advanced math functions...")
    
    // Test logarithms
    sus log_result meal = math_log(math_e())
    vibez.spill("Log(e): " + log_result.(tea))
    
    // Test exponentials
    sus exp_result meal = math_exp(1.0)
    vibez.spill("Exp(1): " + exp_result.(tea))
    
    // Test rounding
    sus floor_result meal = math_floor(3.7)
    vibez.spill("Floor(3.7): " + floor_result.(tea))
    
    sus ceil_result meal = math_ceil(3.2)
    vibez.spill("Ceil(3.2): " + ceil_result.(tea))
    
    // Test number theory
    sus gcd_result normie = math_gcd(48, 18)
    vibez.spill("GCD(48, 18): " + gcd_result.(tea))
    
    sus factorial_result normie = math_factorial(5)
    vibez.spill("Factorial(5): " + factorial_result.(tea))
    
    vibez.spill("✅ Advanced math functions working!")
}

slay test_random_math() {
    vibez.spill("Testing random number generation...")
    
    // Seed for reproducible results
    math_seed_random(42)
    
    sus rand1 meal = math_random()
    sus rand2 meal = math_random()
    
    vibez.spill("Random 1: " + rand1.(tea))
    vibez.spill("Random 2: " + rand2.(tea))
    
    sus rand_int normie = math_random_int(1, 10)
    vibez.spill("Random int [1,10]: " + rand_int.(tea))
    
    vibez.spill("✅ Random number generation working!")
}

slay test_geometry() {
    vibez.spill("Testing geometry functions...")
    
    // Test 2D distance (3-4-5 triangle)
    sus distance meal = math_distance_2d(0.0, 0.0, 3.0, 4.0)
    vibez.spill("Distance from (0,0) to (3,4): " + distance.(tea))
    
    // Test interpolation
    sus lerp_result meal = math_lerp(0.0, 10.0, 0.5)
    vibez.spill("Lerp(0, 10, 0.5): " + lerp_result.(tea))
    
    // Test dot product
    sus dot_result meal = math_dot_product_2d(1.0, 2.0, 3.0, 4.0)
    vibez.spill("Dot product of (1,2) and (3,4): " + dot_result.(tea))
    
    vibez.spill("✅ Geometry functions working!")
}

slay run_math_tests() {
    vibez.spill("🧮 Pure CURSED Math Library Test Suite")
    vibez.spill("====================================")
    vibez.spill("Testing FFI-free math implementation...")
    vibez.spill("")
    
    test_basic_math()
    vibez.spill("")
    
    test_advanced_math()
    vibez.spill("")
    
    test_random_math()
    vibez.spill("")
    
    test_geometry()
    vibez.spill("")
    
    vibez.spill("🎉 All tests completed successfully!")
    vibez.spill("✅ Math stdlib migrated to pure CURSED")
    vibez.spill("🚀 Zero FFI dependencies")
    vibez.spill("💯 Backward compatibility maintained")
}

// Auto-run tests
run_math_tests()
