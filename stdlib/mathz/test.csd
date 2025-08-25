yeet "testz"
yeet "mathz"

test_start("mathz Comprehensive Tests")

fr fr ===== BASIC ARITHMETIC TESTS =====

test_group("Basic Arithmetic Functions")

sus result drip = abs_normie(-5)
assert_eq_int(result, 5, "abs_normie negative")
result = abs_normie(5)
assert_eq_int(result, 5, "abs_normie positive")

result = max_normie(10, 5)
assert_eq_int(result, 10, "max_normie first larger")
result = max_normie(3, 7)
assert_eq_int(result, 7, "max_normie second larger")

result = min_normie(10, 5)
assert_eq_int(result, 5, "min_normie first smaller")
result = min_normie(3, 7)
assert_eq_int(result, 3, "min_normie second smaller")

result = add_two(5, 3)
assert_eq_int(result, 8, "add_two basic")

result = subtract_two(10, 4)
assert_eq_int(result, 6, "subtract_two basic")

result = multiply_two(6, 7)
assert_eq_int(result, 42, "multiply_two basic")

fr fr Test division with error handling
sus div_result drip = divide_two(15, 3) fam {
    when _ -> damn -1
}
assert_eq_int(div_result, 5, "divide_two basic")

fr fr Test division by zero error
sus div_zero_result drip = divide_two(10, 0) fam {
    when "division by zero" -> damn 999
    when _ -> damn -1
}
assert_eq_int(div_zero_result, 999, "divide_two by zero handling")

fr fr ===== ADVANCED FUNCTION TESTS =====

test_group("Advanced Mathematical Functions")

result = power_int(2, 0)
assert_eq_int(result, 1, "power_int zero exponent")
result = power_int(2, 1)
assert_eq_int(result, 2, "power_int one exponent")
result = power_int(2, 3)
assert_eq_int(result, 8, "power_int basic")
result = power_int(5, 2)
assert_eq_int(result, 25, "power_int square")

result = factorial(0)
assert_eq_int(result, 1, "factorial zero")
result = factorial(1)
assert_eq_int(result, 1, "factorial one")
result = factorial(5)
assert_eq_int(result, 120, "factorial five")

result = gcd(48, 18)
assert_eq_int(result, 6, "gcd basic")
result = gcd(17, 13)
assert_eq_int(result, 1, "gcd coprime")
result = gcd(-12, 8)
assert_eq_int(result, 4, "gcd with negative")

result = lcm(4, 6)
assert_eq_int(result, 12, "lcm basic")
result = lcm(7, 5)
assert_eq_int(result, 35, "lcm coprime")

fr fr ===== UTILITY FUNCTION TESTS =====

test_group("Utility Functions")

sus bool_result lit = is_even(4)
assert_bool(bool_result, "is_even true")
bool_result = is_even(5)
assert_bool(!bool_result, "is_even false")

bool_result = is_odd(5)
assert_bool(bool_result, "is_odd true")
bool_result = is_odd(4)
assert_bool(!bool_result, "is_odd false")

result = clamp(5, 1, 10)
assert_eq_int(result, 5, "clamp within range")
result = clamp(-5, 1, 10)
assert_eq_int(result, 1, "clamp below min")
result = clamp(15, 1, 10)
assert_eq_int(result, 10, "clamp above max")

result = sign(5)
assert_eq_int(result, 1, "sign positive")
result = sign(-5)
assert_eq_int(result, -1, "sign negative")
result = sign(0)
assert_eq_int(result, 0, "sign zero")

fr fr ===== SEQUENCE TESTS =====

test_group("Sequence Operations")

result = sum_range(1, 5)
assert_eq_int(result, 15, "sum_range 1 to 5")
result = sum_range(1, 1)
assert_eq_int(result, 1, "sum_range single")

result = fibonacci(0)
assert_eq_int(result, 0, "fibonacci 0")
result = fibonacci(1)
assert_eq_int(result, 1, "fibonacci 1")
result = fibonacci(5)
assert_eq_int(result, 5, "fibonacci 5")
result = fibonacci(10)
assert_eq_int(result, 55, "fibonacci 10")

fr fr ===== TRIGONOMETRIC TESTS =====

test_group("Trigonometric Approximations")

result = pi_value()
assert_eq_int(result, 31416, "pi_value constant")

result = degrees_to_radians(180)
assert_eq_int(result, 31416, "degrees_to_radians 180")

result = sin_approximation(0)
assert_eq_int(result, 0, "sin_approximation 0")

result = cos_approximation(0)
assert_eq_int(result, 10000, "cos_approximation 0")

fr fr ===== MATHEMATICAL CONSTANTS =====

test_group("Mathematical Constants")

result = euler_number()
assert_eq_int(result, 27183, "euler_number constant")

result = golden_ratio()
assert_eq_int(result, 16180, "golden_ratio constant")

fr fr ===== NUMBER THEORY TESTS =====

test_group("Number Theory")

bool_result = is_prime(2)
assert_bool(bool_result, "is_prime 2")
bool_result = is_prime(17)
assert_bool(bool_result, "is_prime 17")
bool_result = is_prime(4)
assert_bool(!bool_result, "is_prime 4 false")
bool_result = is_prime(1)
assert_bool(!bool_result, "is_prime 1 false")

result = next_prime(10)
assert_eq_int(result, 11, "next_prime after 10")
result = next_prime(2)
assert_eq_int(result, 3, "next_prime after 2")

result = nth_prime(1)
assert_eq_int(result, 2, "nth_prime 1st")
result = nth_prime(5)
assert_eq_int(result, 11, "nth_prime 5th")

fr fr ===== MODULAR ARITHMETIC TESTS =====

test_group("Modular Arithmetic")

result = mod_add(7, 5, 10)
assert_eq_int(result, 2, "mod_add basic")

result = mod_multiply(7, 8, 10)
assert_eq_int(result, 6, "mod_multiply basic")

sus mod_power_result drip = mod_power(3, 4, 5) fam {
    when _ -> damn -1
}
assert_eq_int(mod_power_result, 1, "mod_power basic")

fr fr Test modular power error conditions
sus mod_power_error drip = mod_power(2, 3, 0) fam {
    when "modulus must be positive" -> damn 777
    when _ -> damn -1
}
assert_eq_int(mod_power_error, 777, "mod_power zero modulus")

fr fr ===== STATISTICAL FUNCTION TESTS =====

test_group("Statistical Functions")

result = sum_of_squares(3)
assert_eq_int(result, 14, "sum_of_squares 1+4+9")

result = sum_of_cubes(3)
assert_eq_int(result, 36, "sum_of_cubes 1+8+27")

fr fr ===== COMBINATORICS TESTS =====

test_group("Combinatorics")

result = combinations(5, 2)
assert_eq_int(result, 10, "combinations C(5,2)")
result = combinations(4, 0)
assert_eq_int(result, 1, "combinations C(4,0)")
result = combinations(4, 4)
assert_eq_int(result, 1, "combinations C(4,4)")
result = combinations(4, 5)
assert_eq_int(result, 0, "combinations C(4,5) invalid")

result = permutations(5, 2)
assert_eq_int(result, 20, "permutations P(5,2)")
result = permutations(4, 0)
assert_eq_int(result, 1, "permutations P(4,0)")
result = permutations(4, 5)
assert_eq_int(result, 0, "permutations P(4,5) invalid")

fr fr ===== NUMERIC SEQUENCE TESTS =====

test_group("Numeric Sequences")

result = triangular_number(4)
assert_eq_int(result, 10, "triangular_number 4th")

result = square_number(5)
assert_eq_int(result, 25, "square_number 5")

result = pentagonal_number(3)
assert_eq_int(result, 12, "pentagonal_number 3rd")

result = hexagonal_number(3)
assert_eq_int(result, 15, "hexagonal_number 3rd")

fr fr ===== LOGARITHMIC TESTS =====

test_group("Logarithmic Approximations")

result = log2_approximation(8)
assert_eq_int(result, 30000, "log2_approximation 8")
result = log2_approximation(1)
assert_eq_int(result, 0, "log2_approximation 1")

result = log10_approximation(100)
assert_eq_int(result, 20000, "log10_approximation 100")
result = log10_approximation(1)
assert_eq_int(result, 0, "log10_approximation 1")

fr fr ===== ROUNDING TESTS =====

test_group("Rounding and Precision")

result = round_to_nearest(23, 10)
assert_eq_int(result, 20, "round_to_nearest down")
result = round_to_nearest(27, 10)
assert_eq_int(result, 30, "round_to_nearest up")

fr fr ===== SQRT TESTS =====

test_group("Square Root Approximations")

result = sqrt_integer(0)
assert_eq_int(result, 0, "sqrt_integer 0")
result = sqrt_integer(1)
assert_eq_int(result, 1, "sqrt_integer 1")
result = sqrt_integer(4)
assert_eq_int(result, 2, "sqrt_integer 4")
result = sqrt_integer(9)
assert_eq_int(result, 3, "sqrt_integer 9")
result = sqrt_integer(16)
assert_eq_int(result, 4, "sqrt_integer 16")
result = sqrt_integer(25)
assert_eq_int(result, 5, "sqrt_integer 25")
result = sqrt_integer(100)
assert_eq_int(result, 10, "sqrt_integer 100")

fr fr ===== POWER FLOAT TESTS =====

test_group("Power Float Approximations")

result = power_float_approx(2, 0)
assert_eq_int(result, 1, "power_float_approx x^0")
result = power_float_approx(5, 1)
assert_eq_int(result, 5, "power_float_approx x^1")
result = power_float_approx(3, 2)
assert_eq_int(result, 9, "power_float_approx x^2")
result = power_float_approx(2, 3)
assert_eq_int(result, 8, "power_float_approx x^3")
result = power_float_approx(2, 4)
assert_eq_int(result, 16, "power_float_approx x^4")

fr fr ===== DIVISION TESTS =====

test_group("Advanced Division")

result = floor_divide(7, 3)
assert_eq_int(result, 2, "floor_divide positive")
result = floor_divide(-7, 3)
assert_eq_int(result, -3, "floor_divide negative dividend")
result = floor_divide(7, -3)
assert_eq_int(result, -3, "floor_divide negative divisor")
result = floor_divide(-7, -3)
assert_eq_int(result, 2, "floor_divide both negative")

result = ceiling_divide(7, 3)
assert_eq_int(result, 3, "ceiling_divide positive")
result = ceiling_divide(-7, 3)
assert_eq_int(result, -2, "ceiling_divide negative dividend")
result = ceiling_divide(7, -3)
assert_eq_int(result, -2, "ceiling_divide negative divisor")
result = ceiling_divide(-7, -3)
assert_eq_int(result, 3, "ceiling_divide both negative")

fr fr ===== EDGE CASE AND BOUNDARY TESTS =====

test_group("Edge Cases and Boundaries")

fr fr Test zero inputs
result = power_int(0, 5)
assert_eq_int(result, 0, "power_int zero base")
result = gcd(0, 5)
assert_eq_int(result, 5, "gcd zero first")
result = gcd(5, 0)
assert_eq_int(result, 5, "gcd zero second")

fr fr Test large numbers
result = power_int(2, 10)
assert_eq_int(result, 1024, "power_int large exponent")
result = factorial(7)
assert_eq_int(result, 5040, "factorial large")

fr fr Test negative inputs where applicable
result = abs_normie(-100)
assert_eq_int(result, 100, "abs_normie large negative")
result = sign(-1000)
assert_eq_int(result, -1, "sign large negative")

print_test_summary()
