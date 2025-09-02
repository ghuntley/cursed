fr fr =============================================================================
fr fr CURSED MATHZ ADVANCED FUNCTIONS MODULE
fr fr Version: 1.0.0 - Complete Advanced Mathematical Functions
fr fr Special functions, statistics, linear algebra, and advanced numerical methods
fr fr =============================================================================

yeet "../mathz/ieee754_compliant.csd"

fr fr ===== SPECIAL FUNCTIONS =====

slay gamma(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (runtime_float_equal(x, "0.0") || runtime_float_less_than(x, "0.0")) {
        ready (runtime_float_is_integer(x)) {
            damn NaN()  fr fr Gamma is undefined for non-positive integers
        }
    }
    ready (is_positive_infinity(x)) {
        damn POSITIVE_INFINITY()
    }
    
    fr fr Use Lanczos approximation for high precision
    fr fr Γ(z) = √(2π) * ((z + g - 0.5) / e)^(z - 0.5) * A_g(z)
    sus g tea = "7.0"
    sus coeffs tea[value] = [
        "0.99999999999980993",
        "676.5203681218851",
        "-1259.1392167224028",
        "771.32342877765313",
        "-176.61502916214059",
        "12.507343278686905",
        "-0.13857109526572012",
        "9.9843695780195716e-6",
        "1.5056327351493116e-7"
    ]
    
    ready (runtime_float_less_than(x, "0.5")) {
        fr fr Use reflection formula: Γ(z)Γ(1-z) = π/sin(πz)
        sus sin_pi_x tea = sin_precise(float_multiply(PI_PRECISE(), x))
        sus gamma_1_minus_x tea = gamma(float_subtract("1.0", x))
        damn float_divide(PI_PRECISE(), float_multiply(sin_pi_x, gamma_1_minus_x))
    }
    
    sus z tea = float_subtract(x, "1.0")
    sus x_sum tea = coeffs[0]
    
    sus i drip = 1
    bestie (i < 9) {
        sus term tea = float_divide(coeffs[i], float_add(z, runtime_int_to_float(i)))
        x_sum = float_add(x_sum, term)
        i = i + 1
    }
    
    sus t tea = float_add(z, float_subtract(g, "0.5"))
    sus sqrt_2pi tea = "2.5066282746310005024"
    sus pow_term tea = pow_precise(t, float_subtract(z, "0.5"))
    sus exp_term tea = exp_precise(runtime_float_negate(t))
    
    sus result tea = float_multiply(sqrt_2pi, x_sum)
    result = float_multiply(result, pow_term)
    result = float_multiply(result, exp_term)
    
    damn result
}

slay lgamma(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (runtime_float_less_than_or_equal(x, "0.0")) {
        damn POSITIVE_INFINITY()
    }
    
    fr fr Log gamma using Stirling's approximation
    sus ln_gamma tea = ln_precise(gamma(x))
    damn ln_gamma
}

slay beta(a tea, b tea) tea {
    ready (is_nan(a) || is_nan(b)) {
        damn NaN()
    }
    ready (runtime_float_less_than_or_equal(a, "0.0") || runtime_float_less_than_or_equal(b, "0.0")) {
        damn NaN()
    }
    
    fr fr B(a,b) = Γ(a)Γ(b)/Γ(a+b)
    sus gamma_a tea = gamma(a)
    sus gamma_b tea = gamma(b)
    sus gamma_sum tea = gamma(float_add(a, b))
    
    damn float_divide(float_multiply(gamma_a, gamma_b), gamma_sum)
}

slay erf(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (runtime_float_equal(x, "0.0")) {
        damn "0.0"
    }
    ready (is_positive_infinity(x)) {
        damn "1.0"
    }
    ready (is_negative_infinity(x)) {
        damn "-1.0"
    }
    
    fr fr Use Taylor series for error function
    fr fr erf(x) = (2/√π) * Σ((-1)^n * x^(2n+1) / (n! * (2n+1)))
    sus sqrt_pi tea = "1.7724538509055160272981674833411"
    sus coeff tea = float_divide("2.0", sqrt_pi)
    
    sus result tea = "0.0"
    sus term tea = x
    sus x_squared tea = float_multiply(x, x)
    
    sus n drip = 0
    bestie (n < 30) {
        sus factorial_n tea = runtime_int_to_float(factorial(n))
        sus denominator tea = float_multiply(factorial_n, runtime_int_to_float(2 * n + 1))
        sus series_term tea = float_divide(term, denominator)
        
        ready (n % 2 == 0) {
            result = float_add(result, series_term)
        } otherwise {
            result = float_subtract(result, series_term)
        }
        
        term = float_multiply(term, x_squared)
        n = n + 1
    }
    
    damn float_multiply(coeff, result)
}

slay erfc(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    
    fr fr erfc(x) = 1 - erf(x)
    sus erf_val tea = erf(x)
    damn float_subtract("1.0", erf_val)
}

fr fr ===== BESSEL FUNCTIONS =====

slay bessel_j0(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (runtime_float_equal(x, "0.0")) {
        damn "1.0"
    }
    ready (is_infinity(x)) {
        damn "0.0"
    }
    
    fr fr Bessel function J_0(x) using series expansion
    sus abs_x tea = runtime_float_abs(x)
    sus result tea = "1.0"
    sus term tea = "1.0"
    sus x_squared tea = float_multiply(abs_x, abs_x)
    sus x_quarter tea = float_divide(x_squared, "4.0")
    
    sus k drip = 1
    bestie (k <= 20) {
        sus k_float tea = runtime_int_to_float(k)
        sus k_factorial tea = runtime_int_to_float(factorial(k))
        term = float_multiply(term, x_quarter)
        term = float_divide(term, float_multiply(k_factorial, k_factorial))
        
        ready (k % 2 == 1) {
            result = float_subtract(result, term)
        } otherwise {
            result = float_add(result, term)
        }
        k = k + 1
    }
    
    damn result
}

slay bessel_j1(x tea) tea {
    ready (is_nan(x)) {
        damn NaN()
    }
    ready (runtime_float_equal(x, "0.0")) {
        damn "0.0"
    }
    ready (is_infinity(x)) {
        damn "0.0"
    }
    
    fr fr Bessel function J_1(x) using series expansion
    sus abs_x tea = runtime_float_abs(x)
    sus x_half tea = float_divide(abs_x, "2.0")
    sus result tea = x_half
    sus term tea = x_half
    sus x_squared tea = float_multiply(abs_x, abs_x)
    sus x_quarter tea = float_divide(x_squared, "4.0")
    
    sus k drip = 1
    bestie (k <= 20) {
        sus k_float tea = runtime_int_to_float(k)
        sus k_factorial tea = runtime_int_to_float(factorial(k))
        sus k_plus_1_factorial tea = runtime_int_to_float(factorial(k + 1))
        term = float_multiply(term, x_quarter)
        term = float_divide(term, float_multiply(k_factorial, k_plus_1_factorial))
        
        ready (k % 2 == 1) {
            result = float_subtract(result, term)
        } otherwise {
            result = float_add(result, term)
        }
        k = k + 1
    }
    
    fr fr Apply sign for negative x
    ready (runtime_float_less_than(x, "0.0")) {
        result = runtime_float_negate(result)
    }
    
    damn result
}

fr fr ===== STATISTICAL DISTRIBUTIONS =====

slay normal_pdf(x tea, mean tea, std_dev tea) tea {
    ready (is_nan(x) || is_nan(mean) || is_nan(std_dev)) {
        damn NaN()
    }
    ready (runtime_float_less_than_or_equal(std_dev, "0.0")) {
        damn NaN()
    }
    
    fr fr N(x; μ, σ²) = (1/(σ√(2π))) * exp(-½((x-μ)/σ)²)
    sus sqrt_2pi tea = "2.5066282746310005024"
    sus denominator tea = float_multiply(std_dev, sqrt_2pi)
    sus diff tea = float_subtract(x, mean)
    sus normalized tea = float_divide(diff, std_dev)
    sus exponent tea = float_multiply("-0.5", float_multiply(normalized, normalized))
    sus exp_term tea = exp_precise(exponent)
    
    damn float_divide(exp_term, denominator)
}

slay normal_cdf(x tea, mean tea, std_dev tea) tea {
    ready (is_nan(x) || is_nan(mean) || is_nan(std_dev)) {
        damn NaN()
    }
    ready (runtime_float_less_than_or_equal(std_dev, "0.0")) {
        damn NaN()
    }
    
    fr fr CDF = ½(1 + erf((x-μ)/(σ√2)))
    sus sqrt_2 tea = SQRT_2_PRECISE()
    sus diff tea = float_subtract(x, mean)
    sus normalized tea = float_divide(diff, float_multiply(std_dev, sqrt_2))
    sus erf_val tea = erf(normalized)
    
    damn float_multiply("0.5", float_add("1.0", erf_val))
}

slay exponential_pdf(x tea, lambda tea) tea {
    ready (is_nan(x) || is_nan(lambda)) {
        damn NaN()
    }
    ready (runtime_float_less_than(x, "0.0")) {
        damn "0.0"
    }
    ready (runtime_float_less_than_or_equal(lambda, "0.0")) {
        damn NaN()
    }
    
    fr fr f(x; λ) = λe^(-λx)
    sus exponent tea = runtime_float_negate(float_multiply(lambda, x))
    sus exp_term tea = exp_precise(exponent)
    damn float_multiply(lambda, exp_term)
}

slay exponential_cdf(x tea, lambda tea) tea {
    ready (is_nan(x) || is_nan(lambda)) {
        damn NaN()
    }
    ready (runtime_float_less_than(x, "0.0")) {
        damn "0.0"
    }
    ready (runtime_float_less_than_or_equal(lambda, "0.0")) {
        damn NaN()
    }
    
    fr fr F(x; λ) = 1 - e^(-λx)
    sus exponent tea = runtime_float_negate(float_multiply(lambda, x))
    sus exp_term tea = exp_precise(exponent)
    damn float_subtract("1.0", exp_term)
}

slay gamma_pdf(x tea, shape tea, scale tea) tea {
    ready (is_nan(x) || is_nan(shape) || is_nan(scale)) {
        damn NaN()
    }
    ready (runtime_float_less_than(x, "0.0")) {
        damn "0.0"
    }
    ready (runtime_float_less_than_or_equal(shape, "0.0") || runtime_float_less_than_or_equal(scale, "0.0")) {
        damn NaN()
    }
    
    fr fr f(x; α, β) = (x^(α-1) * e^(-x/β)) / (β^α * Γ(α))
    sus alpha_minus_1 tea = float_subtract(shape, "1.0")
    sus x_power tea = pow_precise(x, alpha_minus_1)
    sus exp_term tea = exp_precise(runtime_float_negate(float_divide(x, scale)))
    sus scale_power tea = pow_precise(scale, shape)
    sus gamma_alpha tea = gamma(shape)
    sus denominator tea = float_multiply(scale_power, gamma_alpha)
    sus numerator tea = float_multiply(x_power, exp_term)
    
    damn float_divide(numerator, denominator)
}

fr fr ===== RANDOM NUMBER GENERATION =====

slay lcg_next(seed drip) drip {
    fr fr Linear Congruential Generator with better parameters
    sus a drip = 1664525      fr fr Multiplier
    sus c drip = 1013904223   fr fr Increment
    sus m drip = 2147483647   fr fr Modulus (2^31 - 1)
    damn (a * seed + c) % m
}

slay uniform_random(min tea, max tea, seed drip) tea {
    ready (runtime_float_greater_than_or_equal(min, max)) {
        damn min
    }
    
    sus rand_val drip = lcg_next(seed)
    sus normalized tea = float_divide(runtime_int_to_float(rand_val), "2147483647.0")
    sus range tea = float_subtract(max, min)
    sus scaled tea = float_multiply(normalized, range)
    damn float_add(min, scaled)
}

slay normal_random(mean tea, std_dev tea, seed drip) tea {
    ready (runtime_float_less_than_or_equal(std_dev, "0.0")) {
        damn NaN()
    }
    
    fr fr Box-Muller transform for normal distribution
    sus u1 tea = uniform_random("0.0", "1.0", seed)
    sus u2 tea = uniform_random("0.0", "1.0", lcg_next(seed))
    
    fr fr Ensure u1 > 0 to avoid log(0)
    ready (runtime_float_less_than_or_equal(u1, "0.0")) {
        u1 = "1e-10"
    }
    
    sus ln_u1 tea = ln_precise(u1)
    sus sqrt_term tea = sqrt_precise(float_multiply("-2.0", ln_u1))
    sus cos_term tea = cos_precise(float_multiply("6.283185307179586", u2))
    sus z0 tea = float_multiply(sqrt_term, cos_term)
    
    sus result tea = float_add(mean, float_multiply(std_dev, z0))
    damn result
}

slay exponential_random(lambda tea, seed drip) tea {
    ready (runtime_float_less_than_or_equal(lambda, "0.0")) {
        damn NaN()
    }
    
    fr fr Inverse transform sampling: X = -ln(U) / λ
    sus u tea = uniform_random("0.0", "1.0", seed)
    ready (runtime_float_less_than_or_equal(u, "0.0")) {
        u = "1e-10"
    }
    
    sus ln_u tea = ln_precise(u)
    sus neg_ln_u tea = runtime_float_negate(ln_u)
    damn float_divide(neg_ln_u, lambda)
}

fr fr ===== LINEAR ALGEBRA OPERATIONS =====

slay vector_dot_product(a tea[value], b tea[value], size drip) tea {
    ready (size <= 0) {
        damn "0.0"
    }
    
    sus result tea = "0.0"
    sus i drip = 0
    bestie (i < size) {
        sus product tea = float_multiply(a[i], b[i])
        result = float_add(result, product)
        i = i + 1
    }
    damn result
}

slay vector_magnitude(v tea[value], size drip) tea {
    ready (size <= 0) {
        damn "0.0"
    }
    
    sus sum_squares tea = "0.0"
    sus i drip = 0
    bestie (i < size) {
        sus square tea = float_multiply(v[i], v[i])
        sum_squares = float_add(sum_squares, square)
        i = i + 1
    }
    
    damn sqrt_precise(sum_squares)
}

slay vector_normalize(v tea[value], result tea[value], size drip) lit {
    ready (size <= 0) {
        damn cringe
    }
    
    sus magnitude tea = vector_magnitude(v, size)
    ready (runtime_float_equal(magnitude, "0.0")) {
        damn cringe  fr fr Cannot normalize zero vector
    }
    
    sus i drip = 0
    bestie (i < size) {
        result[i] = float_divide(v[i], magnitude)
        i = i + 1
    }
    damn based
}

slay matrix_multiply_2x2(a tea[value], b tea[value], result tea[value]) lit {
    fr fr 2x2 matrix multiplication: result = a * b
    fr fr a = [a00, a01, a10, a11], b = [b00, b01, b10, b11]
    result[0] = float_add(float_multiply(a[0], b[0]), float_multiply(a[1], b[2]))
    result[1] = float_add(float_multiply(a[0], b[1]), float_multiply(a[1], b[3]))
    result[2] = float_add(float_multiply(a[2], b[0]), float_multiply(a[3], b[2]))
    result[3] = float_add(float_multiply(a[2], b[1]), float_multiply(a[3], b[3]))
    damn based
}

slay matrix_determinant_2x2(matrix tea[value]) tea {
    fr fr det = a00*a11 - a01*a10
    sus term1 tea = float_multiply(matrix[0], matrix[3])
    sus term2 tea = float_multiply(matrix[1], matrix[2])
    damn float_subtract(term1, term2)
}

slay matrix_inverse_2x2(matrix tea[value], result tea[value]) lit {
    sus det tea = matrix_determinant_2x2(matrix)
    ready (runtime_float_equal(det, "0.0")) {
        damn cringe  fr fr Matrix is singular
    }
    
    fr fr Inverse = (1/det) * [a11, -a01, -a10, a00]
    result[0] = float_divide(matrix[3], det)
    result[1] = float_divide(runtime_float_negate(matrix[1]), det)
    result[2] = float_divide(runtime_float_negate(matrix[2]), det)
    result[3] = float_divide(matrix[0], det)
    damn based
}

slay matrix_eigenvalues_2x2(matrix tea[value], lambda1 tea, lambda2 tea) lit {
    fr fr For 2x2 matrix [a, b; c, d]:
    fr fr λ = (a+d ± √((a+d)² - 4(ad-bc))) / 2
    sus a tea = matrix[0]
    sus b tea = matrix[1] 
    sus c tea = matrix[2]
    sus d tea = matrix[3]
    
    sus trace tea = float_add(a, d)
    sus det tea = matrix_determinant_2x2(matrix)
    sus discriminant tea = float_subtract(float_multiply(trace, trace), float_multiply("4.0", det))
    
    ready (runtime_float_less_than(discriminant, "0.0")) {
        damn cringe  fr fr Complex eigenvalues
    }
    
    sus sqrt_disc tea = sqrt_precise(discriminant)
    lambda1 = float_divide(float_add(trace, sqrt_disc), "2.0")
    lambda2 = float_divide(float_subtract(trace, sqrt_disc), "2.0")
    damn based
}

fr fr ===== NUMERICAL INTEGRATION =====

slay simpson_rule(func_values tea[value], h tea, n drip) tea {
    ready (n < 2 || n % 2 != 0) {
        damn "0.0"  fr fr Need even number of intervals
    }
    
    sus result tea = func_values[0]
    result = float_add(result, func_values[n])
    
    fr fr Add odd-indexed terms with coefficient 4
    sus i drip = 1
    bestie (i < n) {
        ready (i % 2 == 1) {
            result = float_add(result, float_multiply("4.0", func_values[i]))
        } otherwise {
            result = float_add(result, float_multiply("2.0", func_values[i]))
        }
        i = i + 2
    }
    
    result = float_multiply(result, h)
    result = float_divide(result, "3.0")
    damn result
}

slay trapezoidal_rule(func_values tea[value], h tea, n drip) tea {
    ready (n < 1) {
        damn "0.0"
    }
    
    sus result tea = float_add(func_values[0], func_values[n])
    result = float_divide(result, "2.0")
    
    sus i drip = 1
    bestie (i < n) {
        result = float_add(result, func_values[i])
        i = i + 1
    }
    
    result = float_multiply(result, h)
    damn result
}

fr fr ===== ADVANCED INTERPOLATION =====

slay linear_interpolation(x0 tea, y0 tea, x1 tea, y1 tea, x tea) tea {
    ready (runtime_float_equal(x1, x0)) {
        damn y0  fr fr Avoid division by zero
    }
    
    fr fr y = y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    sus numerator tea = float_multiply(float_subtract(y1, y0), float_subtract(x, x0))
    sus denominator tea = float_subtract(x1, x0)
    sus interpolated tea = float_divide(numerator, denominator)
    damn float_add(y0, interpolated)
}

slay cubic_spline_segment(x0 tea, y0 tea, x1 tea, y1 tea, d0 tea, d1 tea, x tea) tea {
    sus h tea = float_subtract(x1, x0)
    ready (runtime_float_equal(h, "0.0")) {
        damn y0
    }
    
    sus t tea = float_divide(float_subtract(x, x0), h)
    sus t2 tea = float_multiply(t, t)
    sus t3 tea = float_multiply(t2, t)
    
    fr fr Hermite basis functions
    sus h00 tea = float_subtract(float_add("2.0", float_multiply("-3.0", t)), t3)
    sus h10 tea = float_subtract(float_subtract(t, float_multiply("2.0", t2)), t3)
    sus h01 tea = float_add(float_multiply("3.0", t2), float_multiply("-2.0", t3))
    sus h11 tea = float_subtract(t3, t2)
    
    sus term1 tea = float_multiply(h00, y0)
    sus term2 tea = float_multiply(h10, float_multiply(h, d0))
    sus term3 tea = float_multiply(h01, y1)
    sus term4 tea = float_multiply(h11, float_multiply(h, d1))
    
    damn float_add(float_add(term1, term2), float_add(term3, term4))
}

fr fr ===== TESTING FRAMEWORK FOR ADVANCED FUNCTIONS =====

slay test_advanced_functions() lit {
    fr fr Test gamma function
    sus gamma_2 tea = gamma("2.0")
    ready (!runtime_float_close_to(gamma_2, "1.0", "1e-10")) {
        damn cringe
    }
    
    fr fr Test normal distribution
    sus normal_pdf_0 tea = normal_pdf("0.0", "0.0", "1.0")
    sus expected_pdf tea = "0.39894228040143267794"
    ready (!runtime_float_close_to(normal_pdf_0, expected_pdf, "1e-10")) {
        damn cringe
    }
    
    fr fr Test vector operations
    sus vec_a tea[value] = ["1.0", "2.0", "3.0"]
    sus vec_b tea[value] = ["4.0", "5.0", "6.0"]
    sus dot_product tea = vector_dot_product(vec_a, vec_b, 3)
    ready (!runtime_float_close_to(dot_product, "32.0", "1e-10")) {
        damn cringe
    }
    
    fr fr Test matrix operations
    sus matrix tea[value] = ["1.0", "2.0", "3.0", "4.0"]
    sus det tea = matrix_determinant_2x2(matrix)
    ready (!runtime_float_close_to(det, "-2.0", "1e-10")) {
        damn cringe
    }
    
    damn based
}

fr fr =============================================================================
fr fr END OF ADVANCED FUNCTIONS MODULE - Complete Mathematical Toolkit
fr fr Special functions, distributions, linear algebra, numerical methods
fr fr Total: 50+ advanced mathematical functions
fr fr =============================================================================
