fr fr =============================================================================
fr fr CURSED MATHZ COMPREHENSIVE TEST SUITE
fr fr Version: 1.0.0 - Complete Mathematical Functions Validation
fr fr Tests all advanced mathematical functions for accuracy and edge cases
fr fr =============================================================================

yeet "../mathz/mathz.csd"
yeet "../mathz/advanced_functions.csd"
yeet "../mathz/optimization.csd"
yeet "../testz/testz.csd"

fr fr ===== MAIN TEST RUNNER =====

slay main_character() drip {
    test_start("CURSED MATHZ Comprehensive Test Suite")
    
    fr fr Core mathematical functions
    test_basic_arithmetic()
    test_trigonometric_functions()
    test_exponential_logarithmic()
    test_hyperbolic_functions()
    test_inverse_trigonometric()
    
    fr fr Advanced mathematical functions
    test_special_functions()
    test_statistical_distributions()
    test_bessel_functions()
    test_random_number_generation()
    
    fr fr Linear algebra operations
    test_vector_operations()
    test_matrix_operations()
    test_eigenvalue_computation()
    
    fr fr Numerical methods
    test_root_finding_algorithms()
    test_optimization_algorithms()
    test_numerical_differentiation()
    test_numerical_integration()
    test_curve_fitting()
    
    fr fr IEEE 754 compliance
    test_ieee754_special_values()
    test_precision_accuracy()
    test_edge_cases()
    
    print_test_summary()
    damn 0
}

fr fr ===== BASIC ARITHMETIC TESTS =====

slay test_basic_arithmetic() lit {
    test_group("Basic Arithmetic Operations")
    
    fr fr Test basic operations
    assert_eq_int(add(5, 3), 8)
    assert_eq_int(subtract(10, 4), 6)
    assert_eq_int(multiply(7, 6), 42)
    assert_eq_int(divide(20, 4), 5)
    
    fr fr Test edge cases
    assert_eq_int(divide(10, 0), 0)  fr fr Safe division by zero
    assert_eq_int(abs(-15), 15)
    assert_eq_int(abs(15), 15)
    assert_eq_int(max(10, 5), 10)
    assert_eq_int(min(10, 5), 5)
    
    fr fr Test power function
    assert_eq_int(power(2, 3), 8)
    assert_eq_int(power(5, 0), 1)
    assert_eq_int(power(3, 1), 3)
    
    test_pass("Basic arithmetic operations")
    damn based
}

fr fr ===== TRIGONOMETRIC FUNCTION TESTS =====

slay test_trigonometric_functions() lit {
    test_group("Trigonometric Functions")
    
    fr fr Test fundamental trigonometric identities
    sus pi_half tea = float_divide(PI_PRECISE(), "2.0")
    sus sin_pi_half tea = sin_precise(pi_half)
    assert_close_float(sin_pi_half, "1.0", "1e-12")
    
    sus cos_pi_half tea = cos_precise(pi_half)
    assert_close_float(cos_pi_half, "0.0", "1e-12")
    
    sus sin_0 tea = sin_precise("0.0")
    assert_close_float(sin_0, "0.0", "1e-15")
    
    sus cos_0 tea = cos_precise("0.0")
    assert_close_float(cos_0, "1.0", "1e-15")
    
    fr fr Test tan function
    sus tan_pi_4 tea = tan_precise(float_divide(PI_PRECISE(), "4.0"))
    assert_close_float(tan_pi_4, "1.0", "1e-12")
    
    fr fr Test periodicity
    sus sin_2pi tea = sin_precise(TAU_PRECISE())
    assert_close_float(sin_2pi, "0.0", "1e-12")
    
    sus cos_2pi tea = cos_precise(TAU_PRECISE())
    assert_close_float(cos_2pi, "1.0", "1e-12")
    
    test_pass("Trigonometric functions")
    damn based
}

fr fr ===== EXPONENTIAL AND LOGARITHMIC TESTS =====

slay test_exponential_logarithmic() lit {
    test_group("Exponential and Logarithmic Functions")
    
    fr fr Test fundamental identities
    sus exp_1 tea = exp_precise("1.0")
    assert_close_float(exp_1, E_PRECISE(), "1e-12")
    
    sus ln_e tea = ln_precise(E_PRECISE())
    assert_close_float(ln_e, "1.0", "1e-15")
    
    sus log10_10 tea = log10_precise("10.0")
    assert_close_float(log10_10, "1.0", "1e-15")
    
    sus log2_8 tea = log2_precise("8.0")
    assert_close_float(log2_8, "3.0", "1e-15")
    
    fr fr Test exp(ln(x)) = x identity
    sus x tea = "5.7"
    sus ln_x tea = ln_precise(x)
    sus exp_ln_x tea = exp_precise(ln_x)
    assert_close_float(exp_ln_x, x, "1e-12")
    
    fr fr Test ln(exp(x)) = x identity
    sus y tea = "2.3"
    sus exp_y tea = exp_precise(y)
    sus ln_exp_y tea = ln_precise(exp_y)
    assert_close_float(ln_exp_y, y, "1e-12")
    
    test_pass("Exponential and logarithmic functions")
    damn based
}

fr fr ===== HYPERBOLIC FUNCTION TESTS =====

slay test_hyperbolic_functions() lit {
    test_group("Hyperbolic Functions")
    
    fr fr Test fundamental identities
    sus sinh_0 tea = sinh_precise("0.0")
    assert_close_float(sinh_0, "0.0", "1e-15")
    
    sus cosh_0 tea = cosh_precise("0.0")
    assert_close_float(cosh_0, "1.0", "1e-15")
    
    sus tanh_0 tea = tanh_precise("0.0")
    assert_close_float(tanh_0, "0.0", "1e-15")
    
    fr fr Test cosh²(x) - sinh²(x) = 1
    sus x tea = "1.5"
    sus sinh_x tea = sinh_precise(x)
    sus cosh_x tea = cosh_precise(x)
    sus sinh_squared tea = float_multiply(sinh_x, sinh_x)
    sus cosh_squared tea = float_multiply(cosh_x, cosh_x)
    sus identity tea = float_subtract(cosh_squared, sinh_squared)
    assert_close_float(identity, "1.0", "1e-12")
    
    fr fr Test tanh = sinh/cosh
    sus tanh_x_direct tea = tanh_precise(x)
    sus tanh_x_computed tea = float_divide(sinh_x, cosh_x)
    assert_close_float(tanh_x_direct, tanh_x_computed, "1e-12")
    
    test_pass("Hyperbolic functions")
    damn based
}

fr fr ===== INVERSE TRIGONOMETRIC TESTS =====

slay test_inverse_trigonometric() lit {
    test_group("Inverse Trigonometric Functions")
    
    fr fr Test asin(sin(x)) = x
    sus x tea = "0.5"
    sus sin_x tea = sin_precise(x)
    sus asin_sin_x tea = asin_precise(sin_x)
    assert_close_float(asin_sin_x, x, "1e-10")
    
    fr fr Test acos(cos(x)) = x
    sus cos_x tea = cos_precise(x)
    sus acos_cos_x tea = acos_precise(cos_x)
    assert_close_float(acos_cos_x, x, "1e-10")
    
    fr fr Test atan(tan(x)) = x
    sus tan_x tea = tan_precise(x)
    sus atan_tan_x tea = atan_precise(tan_x)
    assert_close_float(atan_tan_x, x, "1e-10")
    
    fr fr Test special values
    sus asin_1 tea = asin_precise("1.0")
    sus pi_half tea = float_divide(PI_PRECISE(), "2.0")
    assert_close_float(asin_1, pi_half, "1e-12")
    
    sus acos_1 tea = acos_precise("1.0")
    assert_close_float(acos_1, "0.0", "1e-15")
    
    test_pass("Inverse trigonometric functions")
    damn based
}

fr fr ===== SPECIAL FUNCTION TESTS =====

slay test_special_functions() lit {
    test_group("Special Functions")
    
    fr fr Test gamma function
    sus gamma_1 tea = gamma("1.0")
    assert_close_float(gamma_1, "1.0", "1e-12")
    
    sus gamma_2 tea = gamma("2.0")
    assert_close_float(gamma_2, "1.0", "1e-12")
    
    sus gamma_3 tea = gamma("3.0")
    assert_close_float(gamma_3, "2.0", "1e-12")
    
    sus gamma_half tea = gamma("0.5")
    sus sqrt_pi tea = sqrt_precise(PI_PRECISE())
    assert_close_float(gamma_half, sqrt_pi, "1e-10")
    
    fr fr Test error function
    sus erf_0 tea = erf("0.0")
    assert_close_float(erf_0, "0.0", "1e-15")
    
    sus erf_1 tea = erf("1.0")
    assert_close_float(erf_1, "0.8427007929497148693412", "1e-10")
    
    fr fr Test complementary error function
    sus erfc_0 tea = erfc("0.0")
    assert_close_float(erfc_0, "1.0", "1e-15")
    
    fr fr Test erf + erfc = 1
    sus x tea = "1.5"
    sus erf_x tea = erf(x)
    sus erfc_x tea = erfc(x)
    sus sum tea = float_add(erf_x, erfc_x)
    assert_close_float(sum, "1.0", "1e-12")
    
    test_pass("Special functions")
    damn based
}

fr fr ===== STATISTICAL DISTRIBUTION TESTS =====

slay test_statistical_distributions() lit {
    test_group("Statistical Distributions")
    
    fr fr Test normal distribution
    sus normal_mean_0_std_1 tea = normal_pdf("0.0", "0.0", "1.0")
    sus expected_pdf tea = "0.39894228040143267794"
    assert_close_float(normal_mean_0_std_1, expected_pdf, "1e-10")
    
    fr fr Test normal CDF at mean
    sus normal_cdf_mean tea = normal_cdf("0.0", "0.0", "1.0")
    assert_close_float(normal_cdf_mean, "0.5", "1e-12")
    
    fr fr Test exponential distribution
    sus exp_pdf_1 tea = exponential_pdf("1.0", "1.0")
    sus expected_exp tea = float_divide("1.0", E_PRECISE())
    assert_close_float(exp_pdf_1, expected_exp, "1e-10")
    
    fr fr Test exponential CDF
    sus exp_cdf_0 tea = exponential_cdf("0.0", "1.0")
    assert_close_float(exp_cdf_0, "0.0", "1e-15")
    
    fr fr Test gamma distribution shape=1 equals exponential
    sus gamma_pdf_1_1 tea = gamma_pdf("1.0", "1.0", "1.0")
    sus exp_pdf_1_1 tea = exponential_pdf("1.0", "1.0")
    assert_close_float(gamma_pdf_1_1, exp_pdf_1_1, "1e-10")
    
    test_pass("Statistical distributions")
    damn based
}

fr fr ===== BESSEL FUNCTION TESTS =====

slay test_bessel_functions() lit {
    test_group("Bessel Functions")
    
    fr fr Test Bessel J_0
    sus j0_0 tea = bessel_j0("0.0")
    assert_close_float(j0_0, "1.0", "1e-15")
    
    sus j0_pi tea = bessel_j0(PI_PRECISE())
    assert_close_float(j0_pi, "-0.30442907374700075", "1e-10")
    
    fr fr Test Bessel J_1
    sus j1_0 tea = bessel_j1("0.0")
    assert_close_float(j1_0, "0.0", "1e-15")
    
    sus j1_pi tea = bessel_j1(PI_PRECISE())
    assert_close_float(j1_pi, "-0.28461534317975508", "1e-10")
    
    fr fr Test J_n(-x) = (-1)^n * J_n(x) for J_1
    sus x tea = "2.5"
    sus j1_pos tea = bessel_j1(x)
    sus j1_neg tea = bessel_j1(runtime_float_negate(x))
    sus expected_neg tea = runtime_float_negate(j1_pos)
    assert_close_float(j1_neg, expected_neg, "1e-10")
    
    test_pass("Bessel functions")
    damn based
}

fr fr ===== RANDOM NUMBER GENERATION TESTS =====

slay test_random_number_generation() lit {
    test_group("Random Number Generation")
    
    fr fr Test uniform random bounds
    sus seed drip = 12345
    sus uniform_val tea = uniform_random("0.0", "1.0", seed)
    assert_true(runtime_float_greater_than_or_equal(uniform_val, "0.0"))
    assert_true(runtime_float_less_than_or_equal(uniform_val, "1.0"))
    
    fr fr Test normal random (basic sanity check)
    sus normal_val tea = normal_random("0.0", "1.0", seed)
    assert_true(!is_nan(normal_val))
    assert_true(is_finite(normal_val))
    
    fr fr Test exponential random (should be positive)
    sus exp_val tea = exponential_random("1.0", seed)
    assert_true(runtime_float_greater_than_or_equal(exp_val, "0.0"))
    
    fr fr Test LCG determinism
    sus val1 drip = lcg_next(seed)
    sus val2 drip = lcg_next(seed)
    assert_eq_int(val1, val2)  fr fr Same seed should give same result
    
    test_pass("Random number generation")
    damn based
}

fr fr ===== VECTOR OPERATION TESTS =====

slay test_vector_operations() lit {
    test_group("Vector Operations")
    
    fr fr Test dot product
    sus vec_a tea[value] = ["1.0", "2.0", "3.0"]
    sus vec_b tea[value] = ["4.0", "5.0", "6.0"]
    sus dot_result tea = vector_dot_product(vec_a, vec_b, 3)
    assert_close_float(dot_result, "32.0", "1e-15")
    
    fr fr Test vector magnitude
    sus vec_unit tea[value] = ["1.0", "0.0", "0.0"]
    sus mag_unit tea = vector_magnitude(vec_unit, 3)
    assert_close_float(mag_unit, "1.0", "1e-15")
    
    sus vec_345 tea[value] = ["3.0", "4.0", "0.0"]
    sus mag_345 tea = vector_magnitude(vec_345, 3)
    assert_close_float(mag_345, "5.0", "1e-15")
    
    fr fr Test vector normalization
    sus vec_to_normalize tea[value] = ["3.0", "4.0", "0.0"]
    sus normalized tea[value] = ["0.0", "0.0", "0.0"]
    sus success lit = vector_normalize(vec_to_normalize, normalized, 3)
    assert_true(success)
    
    sus norm_magnitude tea = vector_magnitude(normalized, 3)
    assert_close_float(norm_magnitude, "1.0", "1e-12")
    
    test_pass("Vector operations")
    damn based
}

fr fr ===== MATRIX OPERATION TESTS =====

slay test_matrix_operations() lit {
    test_group("Matrix Operations")
    
    fr fr Test 2x2 matrix determinant
    sus matrix_2x2 tea[value] = ["1.0", "2.0", "3.0", "4.0"]
    sus det tea = matrix_determinant_2x2(matrix_2x2)
    assert_close_float(det, "-2.0", "1e-15")
    
    fr fr Test 2x2 matrix multiplication
    sus a tea[value] = ["1.0", "2.0", "3.0", "4.0"]
    sus b tea[value] = ["5.0", "6.0", "7.0", "8.0"]
    sus result tea[value] = ["0.0", "0.0", "0.0", "0.0"]
    sus mult_success lit = matrix_multiply_2x2(a, b, result)
    assert_true(mult_success)
    
    fr fr Expected: [19, 22, 43, 50]
    assert_close_float(result[0], "19.0", "1e-15")
    assert_close_float(result[1], "22.0", "1e-15")
    assert_close_float(result[2], "43.0", "1e-15")
    assert_close_float(result[3], "50.0", "1e-15")
    
    fr fr Test matrix inverse
    sus inv_matrix tea[value] = ["4.0", "7.0", "2.0", "6.0"]
    sus inverse tea[value] = ["0.0", "0.0", "0.0", "0.0"]
    sus inv_success lit = matrix_inverse_2x2(inv_matrix, inverse)
    assert_true(inv_success)
    
    fr fr Verify A * A^(-1) = I
    sus identity tea[value] = ["0.0", "0.0", "0.0", "0.0"]
    matrix_multiply_2x2(inv_matrix, inverse, identity)
    assert_close_float(identity[0], "1.0", "1e-12")
    assert_close_float(identity[1], "0.0", "1e-12")
    assert_close_float(identity[2], "0.0", "1e-12")
    assert_close_float(identity[3], "1.0", "1e-12")
    
    test_pass("Matrix operations")
    damn based
}

fr fr ===== EIGENVALUE COMPUTATION TESTS =====

slay test_eigenvalue_computation() lit {
    test_group("Eigenvalue Computation")
    
    fr fr Test eigenvalues of diagonal matrix
    sus diag_matrix tea[value] = ["5.0", "0.0", "0.0", "3.0"]
    sus lambda1 tea = "0.0"
    sus lambda2 tea = "0.0"
    sus eigen_success lit = matrix_eigenvalues_2x2(diag_matrix, lambda1, lambda2)
    assert_true(eigen_success)
    
    fr fr Eigenvalues should be 5.0 and 3.0 (or vice versa)
    sus sum_eigenvals tea = float_add(lambda1, lambda2)
    assert_close_float(sum_eigenvals, "8.0", "1e-12")
    
    sus product_eigenvals tea = float_multiply(lambda1, lambda2)
    assert_close_float(product_eigenvals, "15.0", "1e-12")
    
    test_pass("Eigenvalue computation")
    damn based
}

fr fr ===== ROOT FINDING TESTS =====

slay test_root_finding_algorithms() lit {
    test_group("Root Finding Algorithms")
    
    fr fr Test Newton-Raphson for √2 (x² - 2 = 0)
    sus sqrt2_approx tea = newton_raphson_root("1.5", "1e-12", 50)
    assert_close_float(sqrt2_approx, SQRT_2_PRECISE(), "1e-10")
    
    fr fr Test secant method
    sus sqrt2_secant tea = secant_root("1.0", "2.0", "1e-12", 50)
    assert_close_float(sqrt2_secant, SQRT_2_PRECISE(), "1e-8")
    
    fr fr Test bisection method (should be slower but reliable)
    sus sqrt2_bisect tea = bisection_root("f", "1.0", "2.0", "1e-10", 100)
    assert_close_float(sqrt2_bisect, SQRT_2_PRECISE(), "1e-8")
    
    test_pass("Root finding algorithms")
    damn based
}

fr fr ===== OPTIMIZATION ALGORITHM TESTS =====

slay test_optimization_algorithms() lit {
    test_group("Optimization Algorithms")
    
    fr fr Test golden section search (minimize x²)
    sus minimum tea = golden_section_minimize("-2.0", "2.0", "1e-10", 100)
    assert_close_float(minimum, "0.0", "1e-8")
    
    fr fr Test gradient descent (minimize x²)
    sus opt_result tea = gradient_descent_1d("5.0", "0.1", "1e-10", 1000)
    assert_close_float(opt_result, "0.0", "1e-8")
    
    fr fr Test ternary search
    sus ternary_min tea = ternary_search_minimize("-3.0", "3.0", "1e-10", 100)
    assert_close_float(ternary_min, "0.0", "1e-8")
    
    fr fr Test 2D gradient descent
    sus x_start tea[value] = ["10.0", "-5.0"]
    sus gd_success lit = gradient_descent_2d(x_start, "0.1", "1e-10", 1000)
    assert_true(gd_success)
    assert_close_float(x_start[0], "0.0", "1e-8")
    assert_close_float(x_start[1], "0.0", "1e-8")
    
    test_pass("Optimization algorithms")
    damn based
}

fr fr ===== NUMERICAL DIFFERENTIATION TESTS =====

slay test_numerical_differentiation() lit {
    test_group("Numerical Differentiation")
    
    fr fr Test derivatives of x² at x = 2 (should be 4)
    fr fr Function values: f(1.9) = 3.61, f(2.0) = 4.0, f(2.1) = 4.41
    sus func_vals tea[value] = ["3.61", "4.0", "4.41"]
    sus h tea = "0.1"
    
    sus forward_deriv tea = forward_difference(func_vals, h, 0)
    assert_close_float(forward_deriv, "3.9", "0.2")  fr fr Approximate
    
    sus backward_deriv tea = backward_difference(func_vals, h, 2)
    assert_close_float(backward_deriv, "4.1", "0.2")  fr fr Approximate
    
    sus central_deriv tea = central_difference(func_vals, h, 1)
    assert_close_float(central_deriv, "4.0", "0.1")  fr fr More accurate
    
    test_pass("Numerical differentiation")
    damn based
}

fr fr ===== NUMERICAL INTEGRATION TESTS =====

slay test_numerical_integration() lit {
    test_group("Numerical Integration")
    
    fr fr Test integration of x² from 0 to 2 (should be 8/3)
    fr fr Using function values at x = 0, 0.5, 1.0, 1.5, 2.0
    sus func_vals tea[value] = ["0.0", "0.25", "1.0", "2.25", "4.0"]
    sus h tea = "0.5"
    sus n drip = 4
    
    sus simpson_result tea = simpson_rule(func_vals, h, n)
    assert_close_float(simpson_result, "2.6666666666666665", "0.01")
    
    sus trap_result tea = trapezoidal_rule(func_vals, h, n)
    assert_close_float(trap_result, "2.75", "0.1")  fr fr Less accurate than Simpson's
    
    test_pass("Numerical integration")
    damn based
}

fr fr ===== CURVE FITTING TESTS =====

slay test_curve_fitting() lit {
    test_group("Curve Fitting")
    
    fr fr Test linear regression on y = 2x + 1
    sus x_data tea[value] = ["1.0", "2.0", "3.0", "4.0", "5.0"]
    sus y_data tea[value] = ["3.0", "5.0", "7.0", "9.0", "11.0"]
    sus slope tea = "0.0"
    sus intercept tea = "0.0"
    
    sus lin_success lit = linear_regression(x_data, y_data, 5, slope, intercept)
    assert_true(lin_success)
    assert_close_float(slope, "2.0", "1e-10")
    assert_close_float(intercept, "1.0", "1e-10")
    
    fr fr Test quadratic fitting on y = x²
    sus x_quad tea[value] = ["0.0", "1.0", "2.0", "3.0", "4.0"]
    sus y_quad tea[value] = ["0.0", "1.0", "4.0", "9.0", "16.0"]
    sus coeffs tea[value] = ["0.0", "0.0", "0.0"]
    
    sus quad_success lit = polynomial_fit_quadratic(x_quad, y_quad, 5, coeffs)
    assert_true(quad_success)
    assert_close_float(coeffs[0], "1.0", "1e-10")  fr fr x² coefficient
    assert_close_float(coeffs[1], "0.0", "1e-10")  fr fr x coefficient
    assert_close_float(coeffs[2], "0.0", "1e-10")  fr fr constant term
    
    test_pass("Curve fitting")
    damn based
}

fr fr ===== IEEE 754 SPECIAL VALUE TESTS =====

slay test_ieee754_special_values() lit {
    test_group("IEEE 754 Special Values")
    
    fr fr Test NaN propagation
    sus nan_val tea = NaN()
    sus nan_result tea = float_add(nan_val, "1.0")
    assert_true(is_nan(nan_result))
    
    sus nan_mult tea = float_multiply(nan_val, "5.0")
    assert_true(is_nan(nan_mult))
    
    fr fr Test infinity arithmetic
    sus pos_inf tea = POSITIVE_INFINITY()
    sus inf_add tea = float_add(pos_inf, "100.0")
    assert_true(is_positive_infinity(inf_add))
    
    sus inf_mult tea = float_multiply(pos_inf, "2.0")
    assert_true(is_positive_infinity(inf_mult))
    
    fr fr Test infinity - infinity = NaN
    sus inf_minus_inf tea = float_subtract(pos_inf, pos_inf)
    assert_true(is_nan(inf_minus_inf))
    
    fr fr Test 0 * infinity = NaN
    sus zero_times_inf tea = float_multiply("0.0", pos_inf)
    assert_true(is_nan(zero_times_inf))
    
    fr fr Test division by zero
    sus one_div_zero tea = float_divide("1.0", "0.0")
    assert_true(is_positive_infinity(one_div_zero))
    
    sus neg_one_div_zero tea = float_divide("-1.0", "0.0")
    assert_true(is_negative_infinity(neg_one_div_zero))
    
    test_pass("IEEE 754 special values")
    damn based
}

fr fr ===== PRECISION AND ACCURACY TESTS =====

slay test_precision_accuracy() lit {
    test_group("Precision and Accuracy")
    
    fr fr Test high-precision constants
    sus pi_computed tea = float_multiply("4.0", atan_precise("1.0"))
    assert_close_float(pi_computed, PI_PRECISE(), "1e-12")
    
    fr fr Test mathematical identities with high precision
    sus e_computed tea = exp_precise("1.0")
    assert_close_float(e_computed, E_PRECISE(), "1e-12")
    
    fr fr Test precision of Taylor series
    sus sin_small tea = sin_precise("0.1")
    sus expected_sin_small tea = "0.09983341664682815"
    assert_close_float(sin_small, expected_sin_small, "1e-12")
    
    fr fr Test precision near zero
    sus sin_tiny tea = sin_precise("1e-10")
    assert_close_float(sin_tiny, "1e-10", "1e-15")
    
    fr fr Test precision of power function
    sus pow_result tea = pow_precise("2.0", "10.0")
    assert_close_float(pow_result, "1024.0", "1e-12")
    
    test_pass("Precision and accuracy")
    damn based
}

fr fr ===== EDGE CASE TESTS =====

slay test_edge_cases() lit {
    test_group("Edge Cases")
    
    fr fr Test very large numbers
    sus large_exp tea = exp_precise("100.0")
    assert_true(is_positive_infinity(large_exp))
    
    fr fr Test very small numbers
    sus small_exp tea = exp_precise("-100.0")
    assert_close_float(small_exp, "0.0", "1e-40")
    
    fr fr Test domain boundaries
    sus asin_boundary tea = asin_precise("1.0")
    assert_close_float(asin_boundary, float_divide(PI_PRECISE(), "2.0"), "1e-12")
    
    sus acos_boundary tea = acos_precise("-1.0")
    assert_close_float(acos_boundary, PI_PRECISE(), "1e-12")
    
    fr fr Test negative zero
    sus log_neg_zero tea = ln_precise("-0.0")
    assert_true(is_negative_infinity(log_neg_zero))
    
    fr fr Test sqrt of negative numbers
    sus sqrt_negative tea = sqrt_precise("-1.0")
    assert_true(is_nan(sqrt_negative))
    
    fr fr Test gamma function singularities
    sus gamma_zero tea = gamma("0.0")
    assert_true(is_nan(gamma_zero))
    
    sus gamma_neg_int tea = gamma("-2.0")
    assert_true(is_nan(gamma_neg_int))
    
    test_pass("Edge cases")
    damn based
}

fr fr ===== HELPER FUNCTIONS FOR TESTING =====

slay assert_close_float(actual tea, expected tea, tolerance tea) lit {
    sus diff tea = runtime_float_abs(float_subtract(actual, expected))
    ready (runtime_float_less_than_or_equal(diff, tolerance)) {
        damn based
    }
    
    fr fr Print error details for debugging
    vibez.spill("Assertion failed:")
    vibez.spill("  Expected:", expected)
    vibez.spill("  Actual:  ", actual)
    vibez.spill("  Diff:    ", diff)
    vibez.spill("  Tolerance:", tolerance)
    test_fail("Float values not close enough")
    damn cringe
}

slay assert_true(condition lit) lit {
    ready (condition) {
        damn based
    }
    test_fail("Assertion failed: expected true")
    damn cringe
}

fr fr =============================================================================
fr fr END OF COMPREHENSIVE TEST SUITE
fr fr Complete validation of all mathematical functions and edge cases
fr fr =============================================================================
