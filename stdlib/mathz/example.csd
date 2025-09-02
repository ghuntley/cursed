fr fr =============================================================================
fr fr MATHZ MODULE USAGE EXAMPLES
fr fr Comprehensive demonstration of the mathz standard library
fr fr =============================================================================

yeet "mathz"

vibez.spill("🧮 CURSED MATHZ Module - Usage Examples")
vibez.spill("==========================================")

fr fr ===== MATHEMATICAL CONSTANTS EXAMPLES =====
vibez.spill("\n📐 Mathematical Constants:")
vibez.spill("PI =", PI(), "(scaled by 10000)")
vibez.spill("E =", E(), "(scaled by 10000)")
vibez.spill("Golden Ratio =", GOLDEN_RATIO(), "(scaled by 10000)")
vibez.spill("Square Root of 2 =", SQRT_2(), "(scaled by 10000)")

fr fr ===== BASIC ARITHMETIC EXAMPLES =====
vibez.spill("\n➕ Basic Arithmetic:")
vibez.spill("abs(-42) =", abs(-42))
vibez.spill("max(15, 23) =", max(15, 23))
vibez.spill("min(15, 23) =", min(15, 23))
vibez.spill("power(3, 4) =", power(3, 4))
vibez.spill("sqrt(144) =", sqrt(144))

fr fr ===== ADVANCED MATHEMATICAL FUNCTIONS =====
vibez.spill("\n🧠 Advanced Functions:")
vibez.spill("factorial(6) =", factorial(6))
vibez.spill("gcd(48, 18) =", gcd(48, 18))
vibez.spill("lcm(12, 8) =", lcm(12, 8))

fr fr ===== TRIGONOMETRIC FUNCTIONS =====
vibez.spill("\n📐 Trigonometric Functions:")
sus angle_90_rad drip = degrees_to_radians(90)
vibez.spill("90 degrees in radians =", angle_90_rad)
vibez.spill("sin(0) =", sin(0))
vibez.spill("cos(0) =", cos(0))

fr fr ===== NUMBER THEORY =====
vibez.spill("\n🔢 Number Theory:")
vibez.spill("is_prime(17) =", is_prime(17))
vibez.spill("is_prime(18) =", is_prime(18))
vibez.spill("next_prime(20) =", next_prime(20))

fr fr ===== COMBINATORICS =====
vibez.spill("\n🎯 Combinatorics:")
vibez.spill("combinations(5, 2) =", combinations(5, 2))
vibez.spill("permutations(5, 3) =", permutations(5, 3))

fr fr ===== SEQUENCES =====
vibez.spill("\n📊 Sequences:")
vibez.spill("fibonacci(10) =", fibonacci(10))
vibez.spill("sum_range(1, 100) =", sum_range(1, 100))
vibez.spill("triangular_number(10) =", triangular_number(10))

fr fr ===== STATISTICAL FUNCTIONS =====
vibez.spill("\n📈 Statistical Functions:")
sus data drip[value] = [10, 20, 30, 40, 50]
sus data_size drip = 5
vibez.spill("Data: [10, 20, 30, 40, 50]")
vibez.spill("Sum =", sum_array(data, data_size))
vibez.spill("Average =", average(data, data_size))
vibez.spill("Min =", find_min(data, data_size))
vibez.spill("Max =", find_max(data, data_size))

fr fr ===== GEOMETRIC CALCULATIONS =====
vibez.spill("\n📏 Geometric Functions:")
vibez.spill("Distance from (0,0) to (3,4) =", distance_2d(0, 0, 3, 4))
vibez.spill("Area of circle (radius=5) =", area_circle(5))
vibez.spill("Area of rectangle (4x6) =", area_rectangle(4, 6))

fr fr ===== CONVERSION FUNCTIONS =====
vibez.spill("\n🔄 Unit Conversions:")
vibez.spill("0°C in Fahrenheit =", celsius_to_fahrenheit(0))
vibez.spill("100 km in miles =", km_to_miles(100))

fr fr ===== ROUNDING AND PRECISION =====
vibez.spill("\n🔢 Rounding Functions:")
sus value drip = 15678  fr fr 1.5678 scaled
vibez.spill("Original value (scaled) =", value)
vibez.spill("Rounded =", round(value))
vibez.spill("Floor =", floor(value))
vibez.spill("Ceiling =", ceil(value))

fr fr ===== LOGARITHMIC FUNCTIONS =====
vibez.spill("\n📊 Logarithmic Functions:")
vibez.spill("log2(16) =", log2(16))
vibez.spill("log10(1000) =", log10(1000))
vibez.spill("exp(0) =", exp(0), "(e^0)")

fr fr ===== MODULAR ARITHMETIC =====
vibez.spill("\n🔢 Modular Arithmetic:")
vibez.spill("(7 + 8) mod 5 =", mod_add(7, 8, 5))
vibez.spill("(3 * 4) mod 7 =", mod_multiply(3, 4, 7))
vibez.spill("2^10 mod 1000 =", mod_power(2, 10, 1000))

fr fr ===== SAFE OPERATIONS =====
vibez.spill("\n🛡️ Safe Operations:")
vibez.spill("safe_divide(10, 0, default=999) =", safe_divide(10, 0, 999))
vibez.spill("safe_sqrt(-4) =", safe_sqrt(-4))

fr fr ===== UTILITY FUNCTIONS =====
vibez.spill("\n🔧 Utility Functions:")
vibez.spill("clamp(150, 0, 100) =", clamp(150, 0, 100))
vibez.spill("sign(-42) =", sign(-42))
vibez.spill("lerp(0, 100, 0.75) =", lerp(0, 100, 7500))  fr fr 0.75 scaled

fr fr ===== BITWISE OPERATIONS =====
vibez.spill("\n💻 Bitwise Operations:")
vibez.spill("count_set_bits(15) =", count_set_bits(15))  fr fr 1111 binary
vibez.spill("is_power_of_2(64) =", is_power_of_2(64))
vibez.spill("next_power_of_2(100) =", next_power_of_2(100))

fr fr ===== RANDOM HELPERS =====
vibez.spill("\n🎲 Random Number Helpers:")
sus seed drip = 12345
vibez.spill("random_range(1, 100, seed) =", random_range(1, 100, seed))
vibez.spill("simple_hash(seed) =", simple_hash(seed))

fr fr ===== ADVANCED COMPUTATIONS =====
vibez.spill("\n🧮 Advanced Computations:")
vibez.spill("sum_of_squares(10) =", sum_of_squares(10))
vibez.spill("sum_of_cubes(5) =", sum_of_cubes(5))
vibez.spill("arithmetic_mean(20, 30) =", arithmetic_mean(20, 30))
vibez.spill("geometric_mean(16, 25) =", geometric_mean(16, 25))

fr fr ===== PRECISION SCALING =====
vibez.spill("\n⚖️ Precision Scaling:")
sus original drip = 42
sus scaled drip = scale_up(original)
sus back_down drip = scale_down(scaled)
vibez.spill("Original:", original)
vibez.spill("Scaled up:", scaled)
vibez.spill("Scaled back down:", back_down)

fr fr ===== PRACTICAL EXAMPLE: COMPOUND INTEREST CALCULATOR =====
vibez.spill("\n💰 Practical Example: Compound Interest")
vibez.spill("=====================================")

sus principal drip = 1000  fr fr $1000
sus rate drip = 500        fr fr 5% annual rate (scaled: 0.05 * 10000)
sus years drip = 10

fr fr A = P(1 + r)^t formula
sus rate_plus_one drip = 10000 + rate  fr fr 1 + rate
sus compound_factor drip = power(rate_plus_one, years)
sus final_amount drip = (principal * compound_factor) / power(10000, years - 1)

vibez.spill("Principal: $", principal)
vibez.spill("Rate: 5% annually")
vibez.spill("Time:", years, "years")
vibez.spill("Final amount: $", final_amount)

fr fr ===== PRACTICAL EXAMPLE: TRIANGLE CALCULATOR =====
vibez.spill("\n📐 Practical Example: Triangle Properties")
vibez.spill("=========================================")

sus side_a drip = 3
sus side_b drip = 4
sus side_c drip = 5

fr fr Check if it's a right triangle (Pythagorean theorem)
sus a_squared drip = power(side_a, 2)
sus b_squared drip = power(side_b, 2)
sus c_squared drip = power(side_c, 2)
sus is_right lit = (a_squared + b_squared) == c_squared

vibez.spill("Triangle sides:", side_a, ",", side_b, ",", side_c)
vibez.spill("Is right triangle?", is_right)
vibez.spill("Area (using base=", side_a, ", height=", side_b, "):", area_triangle(side_a, side_b))

fr fr ===== PRACTICAL EXAMPLE: STATISTICS ON DATASET =====
vibez.spill("\n📊 Practical Example: Dataset Analysis")
vibez.spill("======================================")

sus dataset drip[value] = [85, 92, 78, 96, 88, 91, 79, 94, 87, 90]
sus dataset_size drip = 10

sus total drip = sum_array(dataset, dataset_size)
sus mean drip = average(dataset, dataset_size)
sus minimum drip = find_min(dataset, dataset_size)
sus maximum drip = find_max(dataset, dataset_size)
sus range drip = maximum - minimum

vibez.spill("Dataset: Test scores")
vibez.spill("Count:", dataset_size)
vibez.spill("Total:", total)
vibez.spill("Mean:", mean)
vibez.spill("Range:", range, "(", minimum, "to", maximum, ")")

vibez.spill("\n🎉 MATHZ Module Examples Complete!")
vibez.spill("All 80+ mathematical functions demonstrated")
vibez.spill("Ready for production use in CURSED applications!")

fr fr =============================================================================
fr fr END OF MATHZ USAGE EXAMPLES
fr fr =============================================================================
