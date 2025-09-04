fr fr CURSED Mathematics Library (mathz) Example
fr fr Demonstrates comprehensive mathematical capabilities

yeet "mathz"
yeet "vibez"
yeet "testz"

slay demonstrate_constants() cringe {
    vibez.spill("=== Mathematical Constants ===")
    
    vibez.spillf("PI = %f", mathz.PI)
    vibez.spillf("E = %f", mathz.E)
    vibez.spillf("TAU (2π) = %f", mathz.TAU)
    vibez.spillf("√2 = %f", mathz.SQRT_2)
    vibez.spillf("√3 = %f", mathz.SQRT_3)
    vibez.spillf("Golden Ratio = %f", mathz.GOLDEN_RATIO)
    vibez.spillf("Euler-Mascheroni = %f", mathz.EULER_MASCHERONI)
    vibez.spillf("Degrees to Radians = %f", mathz.DEGREES_TO_RADIANS)
    vibez.spillf("Radians to Degrees = %f", mathz.RADIANS_TO_DEGREES)
    
    vibez.spill("")
}

slay demonstrate_basic_arithmetic() cringe {
    vibez.spill("=== Basic Arithmetic Operations ===")
    
    sus a meal = 15.5
    sus b meal = 3.2
    
    vibez.spillf("%f + %f = %f", a, b, mathz.math_add(a, b))
    vibez.spillf("%f - %f = %f", a, b, mathz.math_subtract(a, b))
    vibez.spillf("%f × %f = %f", a, b, mathz.math_multiply(a, b))
    vibez.spillf("%f ÷ %f = %f", a, b, mathz.math_divide(a, b))
    
    fr fr Test division by zero safety
    sus safe_div meal = mathz.math_divide(10.0, 0.0)
    vibez.spillf("10.0 ÷ 0.0 = %f (safe fallback)", safe_div)
    
    vibez.spill("")
}

slay demonstrate_absolute_values() cringe {
    vibez.spill("=== Absolute Value Functions ===")
    
    vibez.spillf("abs(%f) = %f", -42.7, mathz.abs_meal(-42.7))
    vibez.spillf("abs(%f) = %f", 15.3, mathz.abs_meal(15.3))
    vibez.spillf("abs(%d) = %d", -25, mathz.abs_normie(-25))
    vibez.spillf("abs(%d) = %d", 18, mathz.abs_normie(18))
    
    vibez.spill("")
}

slay demonstrate_min_max() cringe {
    vibez.spill("=== Min/Max Functions ===")
    
    sus x meal = 12.5
    sus y meal = 8.7
    sus a normie = 15
    sus b normie = 23
    
    vibez.spillf("max(%f, %f) = %f", x, y, mathz.max_meal(x, y))
    vibez.spillf("min(%f, %f) = %f", x, y, mathz.min_meal(x, y))
    vibez.spillf("max(%d, %d) = %d", a, b, mathz.max_normie(a, b))
    vibez.spillf("min(%d, %d) = %d", a, b, mathz.min_normie(a, b))
    
    vibez.spill("")
}

slay demonstrate_rounding() cringe {
    vibez.spill("=== Rounding Functions ===")
    
    sus values [meal] = [3.2, 3.7, -2.3, -2.8, 5.5, -4.5]
    
    bestie i := 0; i < 6; i++ {
        sus val meal = values[i]
        vibez.spillf("floor(%f) = %d", val, mathz.floor_meal(val))
        vibez.spillf("ceil(%f) = %d", val, mathz.ceil_meal(val))
        vibez.spillf("round(%f) = %d", val, mathz.round_meal(val))
        vibez.spill("---")
    }
    
    vibez.spill("")
}

slay demonstrate_power_functions() cringe {
    vibez.spill("=== Power and Root Functions ===")
    
    vibez.spillf("2^3 = %f", mathz.pow_meal(2.0, 3))
    vibez.spillf("5^2 = %f", mathz.pow_meal(5.0, 2))
    vibez.spillf("3^(-2) = %f", mathz.pow_meal(3.0, -2))
    
    vibez.spillf("√25 = %f", mathz.sqrt_meal(25.0))
    vibez.spillf("√2 = %f", mathz.sqrt_meal(2.0))
    vibez.spillf("√(-4) = %f (safe fallback)", mathz.sqrt_meal(-4.0))
    
    vibez.spillf("2.5^3.0 = %f", mathz.pow_meal_meal(2.5, 3.0))
    
    vibez.spill("")
}

slay demonstrate_logarithms() cringe {
    vibez.spill("=== Logarithmic Functions ===")
    
    vibez.spillf("ln(e) = %f", mathz.ln_meal(mathz.E))
    vibez.spillf("ln(1) = %f", mathz.ln_meal(1.0))
    vibez.spillf("ln(2) = %f", mathz.ln_meal(2.0))
    vibez.spillf("ln(10) = %f", mathz.ln_meal(10.0))
    
    vibez.spillf("exp(1) = %f", mathz.exp_meal(1.0))
    vibez.spillf("exp(0) = %f", mathz.exp_meal(0.0))
    vibez.spillf("exp(2) = %f", mathz.exp_meal(2.0))
    
    vibez.spill("")
}

slay demonstrate_trigonometry() cringe {
    vibez.spill("=== Trigonometric Functions ===")
    
    fr fr Test common angles in radians
    vibez.spillf("sin(0) = %f", mathz.sin_meal(0.0))
    vibez.spillf("sin(π/2) = %f", mathz.sin_meal(mathz.PI / 2.0))
    vibez.spillf("cos(0) = %f", mathz.cos_meal(0.0))
    vibez.spillf("cos(π) = %f", mathz.cos_meal(mathz.PI))
    vibez.spillf("tan(π/4) = %f", mathz.tan_meal(mathz.PI / 4.0))
    
    fr fr Test common angles in degrees
    vibez.spillf("sin(30°) = %f", mathz.sin_deg(30.0))
    vibez.spillf("sin(90°) = %f", mathz.sin_deg(90.0))
    vibez.spillf("cos(60°) = %f", mathz.cos_deg(60.0))
    vibez.spillf("cos(180°) = %f", mathz.cos_deg(180.0))
    vibez.spillf("tan(45°) = %f", mathz.tan_deg(45.0))
    
    vibez.spill("")
}

slay demonstrate_number_theory() cringe {
    vibez.spill("=== Number Theory Functions ===")
    
    fr fr Factorial
    bestie i := 0; i <= 8; i++ {
        vibez.spillf("%d! = %d", i, mathz.factorial(i))
    }
    
    fr fr Fibonacci
    vibez.spill("Fibonacci sequence:")
    bestie i := 0; i <= 10; i++ {
        vibez.spillf("fib(%d) = %d", i, mathz.fibonacci(i))
    }
    
    fr fr GCD and LCM
    vibez.spillf("gcd(48, 18) = %d", mathz.gcd(48, 18))
    vibez.spillf("lcm(12, 8) = %d", mathz.lcm(12, 8))
    
    vibez.spill("")
}

slay demonstrate_random_numbers() cringe {
    vibez.spill("=== Random Number Generation ===")
    
    mathz.set_random_seed(42)
    
    vibez.spill("Random integers:")
    bestie i := 0; i < 5; i++ {
        vibez.spillf("random_int() = %d", mathz.random_int())
    }
    
    vibez.spill("Random floats:")
    bestie i := 0; i < 5; i++ {
        vibez.spillf("random_meal() = %f", mathz.random_meal())
    }
    
    vibez.spill("Random range [10, 20]:")
    bestie i := 0; i < 5; i++ {
        vibez.spillf("random_range(10, 20) = %d", mathz.random_range(10, 20))
    }
    
    vibez.spill("")
}

slay demonstrate_utility_functions() cringe {
    vibez.spill("=== Utility Functions ===")
    
    sus epsilon meal = 0.001
    vibez.spillf("is_approximately_equal(3.14159, π, 0.001) = %b", 
                 mathz.is_approximately_equal(3.14159, mathz.PI, epsilon))
    
    vibez.spillf("is_zero(0.0) = %b", mathz.is_zero(0.0))
    vibez.spillf("is_zero(0.001) = %b", mathz.is_zero(0.001))
    
    vibez.spillf("is_positive(5.0) = %b", mathz.is_positive_meal(5.0))
    vibez.spillf("is_negative(-3.0) = %b", mathz.is_negative_meal(-3.0))
    
    vibez.spillf("is_even(4) = %b", mathz.is_even(4))
    vibez.spillf("is_odd(7) = %b", mathz.is_odd(7))
    
    vibez.spill("")
}

slay run_comprehensive_tests() cringe {
    test_start("mathz comprehensive test")
    
    fr fr Test basic operations
    assert_true(mathz.math_add(2.0, 3.0) == 5.0)
    assert_true(mathz.abs_meal(-10.0) == 10.0)
    assert_true(mathz.max_meal(5.0, 10.0) == 10.0)
    
    fr fr Test mathematical constants
    assert_true(mathz.PI > 3.14)
    assert_true(mathz.PI < 3.15)
    assert_true(mathz.E > 2.71)
    
    fr fr Test functions
    assert_eq_int(mathz.factorial(5), 120)
    assert_true(mathz.sqrt_meal(25.0) == 5.0)
    assert_true(mathz.gcd(12, 8) == 4)
    
    print_test_summary()
}

slay main_character() cringe {
    vibez.spill("📐 CURSED Mathematics Library (mathz) Demonstration")
    vibez.spill("==================================================")
    vibez.spill("")
    
    demonstrate_constants()
    demonstrate_basic_arithmetic()
    demonstrate_absolute_values()
    demonstrate_min_max()
    demonstrate_rounding()
    demonstrate_power_functions()
    demonstrate_logarithms()
    demonstrate_trigonometry()
    demonstrate_number_theory()
    demonstrate_random_numbers()
    demonstrate_utility_functions()
    
    run_comprehensive_tests()
    
    vibez.spill("✅ mathz Mathematics Library Demonstration Complete!")
    vibez.spill("")
    vibez.spill("Key Features Demonstrated:")
    vibez.spill("- Mathematical constants (π, e, φ, etc.)")
    vibez.spill("- Basic arithmetic with safety features")
    vibez.spill("- Power, root, and logarithmic functions")
    vibez.spill("- Trigonometric functions (radians & degrees)")
    vibez.spill("- Number theory (factorial, fibonacci, gcd, lcm)")
    vibez.spill("- Random number generation with seeding")
    vibez.spill("- Utility functions for comparisons and checks")
    vibez.spill("- Type-safe operations (meal vs normie)")
}
