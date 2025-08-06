yeet "mathz"
yeet "vibez"

slay demonstrate_constants() {
    vibez.spill("=== Mathematical Constants ===")
    vibez.spillf("PI = %.6f", mathz.PI)
    vibez.spillf("E = %.6f", mathz.E)
    vibez.spillf("TAU (2π) = %.6f", mathz.TAU)
    vibez.spillf("√2 = %.6f", mathz.SQRT_2)
    vibez.spillf("Golden Ratio φ = %.6f", mathz.GOLDEN_RATIO)
    vibez.spill("")
}

slay demonstrate_basic_ops() {
    vibez.spill("=== Basic Operations ===")
    vibez.spillf("abs(-5.5) = %.2f", mathz.abs_meal(-5.5))
    vibez.spillf("max(3.14, 2.71) = %.2f", mathz.max_meal(3.14, 2.71))
    vibez.spillf("min(3.14, 2.71) = %.2f", mathz.min_meal(3.14, 2.71))
    vibez.spillf("clamp(15.0, 1.0, 10.0) = %.2f", mathz.clamp_meal(15.0, 1.0, 10.0))
    vibez.spill("")
}

slay demonstrate_rounding() {
    vibez.spill("=== Rounding Functions ===")
    sus test_val meal = 3.7
    vibez.spillf("floor(%.1f) = %d", test_val, mathz.floor_meal(test_val))
    vibez.spillf("ceil(%.1f) = %d", test_val, mathz.ceil_meal(test_val))
    vibez.spillf("round(%.1f) = %d", test_val, mathz.round_meal(test_val))
    vibez.spillf("trunc(%.1f) = %d", test_val, mathz.trunc_meal(test_val))
    vibez.spillf("frac(%.1f) = %.2f", test_val, mathz.frac_meal(test_val))
    vibez.spill("")
}

slay demonstrate_power_root() {
    vibez.spill("=== Power and Root Functions ===")
    vibez.spillf("2^8 = %.0f", mathz.pow_meal(2.0, 8))
    vibez.spillf("√25 = %.2f", mathz.sqrt_meal(25.0))
    vibez.spillf("∛27 = %.2f", mathz.pow_meal_meal(27.0, 1.0/3.0))
    vibez.spill("")
}

slay demonstrate_exponential_log() {
    vibez.spill("=== Exponential and Logarithmic Functions ===")
    vibez.spillf("e^1 = %.6f", mathz.exp_meal(1.0))
    vibez.spillf("ln(e) = %.6f", mathz.ln_meal(mathz.E))
    vibez.spillf("log10(100) = %.2f", mathz.log10_meal(100.0))
    vibez.spillf("log2(8) = %.2f", mathz.log2_meal(8.0))
    vibez.spill("")
}

slay demonstrate_trigonometry() {
    vibez.spill("=== Trigonometric Functions ===")
    sus angle meal = mathz.PI / 4.0 fr fr 45 degrees in radians
    vibez.spillf("sin(π/4) = %.6f", mathz.sin_meal(angle))
    vibez.spillf("cos(π/4) = %.6f", mathz.cos_meal(angle))
    vibez.spillf("tan(π/4) = %.6f", mathz.tan_meal(angle))
    
    vibez.spill("--- Degree versions ---")
    vibez.spillf("sin(45°) = %.6f", mathz.sin_deg(45.0))
    vibez.spillf("cos(45°) = %.6f", mathz.cos_deg(45.0))
    vibez.spillf("tan(45°) = %.6f", mathz.tan_deg(45.0))
    vibez.spill("")
}

slay demonstrate_inverse_trig() {
    vibez.spill("=== Inverse Trigonometric Functions ===")
    vibez.spillf("asin(0.5) = %.6f", mathz.asin_meal(0.5))
    vibez.spillf("acos(0.5) = %.6f", mathz.acos_meal(0.5))
    vibez.spillf("atan(1.0) = %.6f", mathz.atan_meal(1.0))
    vibez.spillf("atan(1.0) in degrees = %.2f°", mathz.atan_meal(1.0) * mathz.RADIANS_TO_DEGREES)
    vibez.spill("")
}

slay demonstrate_hyperbolic() {
    vibez.spill("=== Hyperbolic Functions ===")
    sus x meal = 1.0
    vibez.spillf("sinh(%.1f) = %.6f", x, mathz.sinh_meal(x))
    vibez.spillf("cosh(%.1f) = %.6f", x, mathz.cosh_meal(x))
    vibez.spillf("tanh(%.1f) = %.6f", x, mathz.tanh_meal(x))
    vibez.spill("")
}

slay demonstrate_utility() {
    vibez.spill("=== Utility Functions ===")
    vibez.spillf("sign(-3.2) = %.0f", mathz.sign_meal(-3.2))
    vibez.spillf("sign(4.7) = %.0f", mathz.sign_meal(4.7))
    vibez.spillf("lerp(0, 10, 0.3) = %.1f", mathz.lerp_meal(0.0, 10.0, 0.3))
    vibez.spillf("fmod(7.5, 2.5) = %.1f", mathz.fmod_meal(7.5, 2.5))
    vibez.spill("")
}

slay demonstrate_number_theory() {
    vibez.spill("=== Number Theory Functions ===")
    vibez.spillf("5! = %d", mathz.factorial(5))
    vibez.spillf("gcd(48, 18) = %d", mathz.gcd(48, 18))
    vibez.spillf("lcm(12, 18) = %d", mathz.lcm(12, 18))
    vibez.spillf("fibonacci(10) = %d", mathz.fibonacci(10))
    
    vibez.spill("--- Prime checking ---")
    bestie i := 2; i <= 20; i++ {
        lowkey mathz.is_prime(i) {
            vibez.spillf("%d is prime", i)
        }
    }
    vibez.spill("")
}

slay demonstrate_series() {
    vibez.spill("=== Mathematical Series ===")
    vibez.spillf("Arithmetic sum 1+2+...+10 = %d", mathz.arithmetic_sum(1, 10, 10))
    vibez.spillf("Geometric sum 1+2+4+8 = %.1f", mathz.geometric_sum(1.0, 2.0, 4))
    vibez.spill("")
}

slay demonstrate_distance() {
    vibez.spill("=== Distance Functions ===")
    vibez.spillf("2D distance (0,0) to (3,4) = %.2f", mathz.distance_2d(0.0, 0.0, 3.0, 4.0))
    vibez.spillf("3D distance (0,0,0) to (1,1,1) = %.6f", mathz.distance_3d(0.0, 0.0, 0.0, 1.0, 1.0, 1.0))
    vibez.spill("")
}

slay demonstrate_random() {
    vibez.spill("=== Random Number Generation ===")
    mathz.set_random_seed(12345)
    
    vibez.spill("Random integers:")
    bestie i := 0; i < 5; i++ {
        vibez.spillf("  %d", mathz.random_int())
    }
    
    vibez.spill("Random floats [0,1]:")
    bestie i := 0; i < 5; i++ {
        vibez.spillf("  %.6f", mathz.random_meal())
    }
    
    vibez.spill("Random range [10,20]:")
    bestie i := 0; i < 5; i++ {
        vibez.spillf("  %d", mathz.random_range(10, 20))
    }
    
    vibez.spill("Random float range [1.0, 5.0]:")
    bestie i := 0; i < 5; i++ {
        vibez.spillf("  %.2f", mathz.random_meal_range(1.0, 5.0))
    }
    
    vibez.spill("Gaussian distribution:")
    bestie i := 0; i < 5; i++ {
        vibez.spillf("  %.6f", mathz.random_gaussian())
    }
    vibez.spill("")
}

slay demonstrate_practical_examples() {
    vibez.spill("=== Practical Examples ===")
    
    fr fr Circle area calculation
    sus radius meal = 5.0
    sus area meal = mathz.PI * mathz.pow_meal(radius, 2)
    vibez.spillf("Circle area (r=%.1f): %.2f", radius, area)
    
    fr fr Compound interest calculation
    sus principal meal = 1000.0
    sus rate meal = 0.05
    sus time meal = 10.0
    sus amount meal = principal * mathz.exp_meal(rate * time)
    vibez.spillf("Compound interest: $%.2f -> $%.2f", principal, amount)
    
    fr fr Physics: projectile motion
    sus velocity meal = 30.0
    sus angle_deg meal = 45.0
    sus g meal = 9.81
    sus angle_rad meal = angle_deg * mathz.DEGREES_TO_RADIANS
    sus range_proj meal = velocity * velocity * mathz.sin_meal(2.0 * angle_rad) / g
    vibez.spillf("Projectile range (v=%.1f m/s, θ=%.0f°): %.2f m", velocity, angle_deg, range_proj)
    
    fr fr Statistics: standard deviation
    sus data_sum meal = 10.0 + 12.0 + 14.0 + 16.0 + 18.0
    sus data_count meal = 5.0
    sus mean meal = data_sum / data_count
    sus variance meal = (mathz.pow_meal(10.0 - mean, 2) + mathz.pow_meal(12.0 - mean, 2) + mathz.pow_meal(14.0 - mean, 2) + mathz.pow_meal(16.0 - mean, 2) + mathz.pow_meal(18.0 - mean, 2)) / data_count
    sus std_dev meal = mathz.sqrt_meal(variance)
    vibez.spillf("Standard deviation of [10,12,14,16,18]: %.6f", std_dev)
    
    vibez.spill("")
}

slay main() {
    vibez.spill("CURSED Mathematical Library (mathz) - Comprehensive Demo")
    vibez.spill("=" * 60)
    vibez.spill("")
    
    demonstrate_constants()
    demonstrate_basic_ops()
    demonstrate_rounding()
    demonstrate_power_root()
    demonstrate_exponential_log()
    demonstrate_trigonometry()
    demonstrate_inverse_trig()
    demonstrate_hyperbolic()
    demonstrate_utility()
    demonstrate_number_theory()
    demonstrate_series()
    demonstrate_distance()
    demonstrate_random()
    demonstrate_practical_examples()
    
    vibez.spill("Demo completed successfully!")
}
