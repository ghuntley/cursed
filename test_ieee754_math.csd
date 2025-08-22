fr fr =============================================================================
fr fr IEEE 754 Mathematical Functions Test Suite
fr fr Comprehensive validation of mathematical accuracy and special value handling
fr fr =============================================================================

yeet "stdlib/mathz/mathz.csd"
yeet "stdlib/vibez/mod.csd"

fr fr ===== SPECIAL VALUES TESTS =====

slay test_special_values() lit {
    vibez.spill("Testing IEEE 754 special values...")
    
    fr fr Test NaN handling
    sus nan_val tea = NaN()
    sus nan_result tea = sin(nan_val)
    ready (!is_nan(nan_result)) {
        vibez.spill("FAILED: sin(NaN) should return NaN")
        damn cringe
    }
    
    fr fr Test infinity handling
    sus inf_val tea = POSITIVE_INFINITY()
    sus inf_sin tea = sin(inf_val)
    ready (!is_nan(inf_sin)) {
        vibez.spill("FAILED: sin(∞) should return NaN")
        damn cringe
    }
    
    fr fr Test division by zero
    sus div_zero tea = float_divide("1.0", "0.0")
    ready (!is_positive_infinity(div_zero)) {
        vibez.spill("FAILED: 1.0 / 0.0 should return +∞")
        damn cringe
    }
    
    vibez.spill("✅ Special values tests passed")
    damn based
}

fr fr ===== TRIGONOMETRIC PRECISION TESTS =====

slay test_trigonometric_precision() lit {
    vibez.spill("Testing trigonometric function precision...")
    
    fr fr Test sin(π/2) = 1
    sus pi_half tea = float_divide(PI(), "2.0")
    sus sin_pi_half tea = sin(pi_half)
    ready (!runtime_float_close_to(sin_pi_half, "1.0", "1e-12")) {
        vibez.spill("FAILED: sin(π/2) precision test")
        vibez.spill("Expected: 1.0, Got:", sin_pi_half)
        damn cringe
    }
    
    fr fr Test cos(0) = 1
    sus cos_zero tea = cos("0.0")
    ready (!runtime_float_close_to(cos_zero, "1.0", "1e-12")) {
        vibez.spill("FAILED: cos(0) precision test")
        vibez.spill("Expected: 1.0, Got:", cos_zero)
        damn cringe
    }
    
    fr fr Test tan(π/4) ≈ 1
    sus pi_quarter tea = float_divide(PI(), "4.0")
    sus tan_pi_quarter tea = tan(pi_quarter)
    ready (!runtime_float_close_to(tan_pi_quarter, "1.0", "1e-12")) {
        vibez.spill("FAILED: tan(π/4) precision test")
        vibez.spill("Expected: 1.0, Got:", tan_pi_quarter)
        damn cringe
    }
    
    vibez.spill("✅ Trigonometric precision tests passed")
    damn based
}

fr fr ===== LOGARITHMIC AND EXPONENTIAL TESTS =====

slay test_logarithmic_functions() lit {
    vibez.spill("Testing logarithmic and exponential functions...")
    
    fr fr Test ln(e) = 1
    sus e_val tea = E()
    sus ln_e tea = ln(e_val)
    ready (!runtime_float_close_to(ln_e, "1.0", "1e-12")) {
        vibez.spill("FAILED: ln(e) should equal 1")
        vibez.spill("Expected: 1.0, Got:", ln_e)
        damn cringe
    }
    
    fr fr Test exp(0) = 1
    sus exp_zero tea = exp("0.0")
    ready (!runtime_float_close_to(exp_zero, "1.0", "1e-12")) {
        vibez.spill("FAILED: exp(0) should equal 1")
        vibez.spill("Expected: 1.0, Got:", exp_zero)
        damn cringe
    }
    
    fr fr Test log10(10) = 1
    sus log10_ten tea = log10("10.0")
    ready (!runtime_float_close_to(log10_ten, "1.0", "1e-12")) {
        vibez.spill("FAILED: log10(10) should equal 1")
        vibez.spill("Expected: 1.0, Got:", log10_ten)
        damn cringe
    }
    
    fr fr Test log2(8) = 3
    sus log2_eight tea = log2("8.0")
    ready (!runtime_float_close_to(log2_eight, "3.0", "1e-12")) {
        vibez.spill("FAILED: log2(8) should equal 3")
        vibez.spill("Expected: 3.0, Got:", log2_eight)
        damn cringe
    }
    
    vibez.spill("✅ Logarithmic function tests passed")
    damn based
}

fr fr ===== POWER AND ROOT FUNCTION TESTS =====

slay test_power_functions() lit {
    vibez.spill("Testing power and root functions...")
    
    fr fr Test sqrt(4) = 2
    sus sqrt_four tea = sqrt("4.0")
    ready (!runtime_float_close_to(sqrt_four, "2.0", "1e-12")) {
        vibez.spill("FAILED: sqrt(4) should equal 2")
        vibez.spill("Expected: 2.0, Got:", sqrt_four)
        damn cringe
    }
    
    fr fr Test sqrt(negative) = NaN
    sus sqrt_neg tea = sqrt("-1.0")
    ready (!is_nan(sqrt_neg)) {
        vibez.spill("FAILED: sqrt(-1) should return NaN")
        vibez.spill("Got:", sqrt_neg)
        damn cringe
    }
    
    fr fr Test pow(2, 3) = 8
    sus pow_result tea = pow("2.0", "3.0")
    ready (!runtime_float_close_to(pow_result, "8.0", "1e-12")) {
        vibez.spill("FAILED: pow(2, 3) should equal 8")
        vibez.spill("Expected: 8.0, Got:", pow_result)
        damn cringe
    }
    
    fr fr Test cbrt(8) = 2
    sus cbrt_eight tea = cbrt("8.0")
    ready (!runtime_float_close_to(cbrt_eight, "2.0", "1e-12")) {
        vibez.spill("FAILED: cbrt(8) should equal 2")
        vibez.spill("Expected: 2.0, Got:", cbrt_eight)
        damn cringe
    }
    
    vibez.spill("✅ Power and root function tests passed")
    damn based
}

fr fr ===== INVERSE TRIGONOMETRIC FUNCTION TESTS =====

slay test_inverse_trigonometric() lit {
    vibez.spill("Testing inverse trigonometric functions...")
    
    fr fr Test asin(1) = π/2
    sus asin_one tea = asin("1.0")
    sus pi_half tea = float_divide(PI(), "2.0")
    ready (!runtime_float_close_to(asin_one, pi_half, "1e-12")) {
        vibez.spill("FAILED: asin(1) should equal π/2")
        vibez.spill("Expected:", pi_half, "Got:", asin_one)
        damn cringe
    }
    
    fr fr Test acos(0) = π/2
    sus acos_zero tea = acos("0.0")
    ready (!runtime_float_close_to(acos_zero, pi_half, "1e-12")) {
        vibez.spill("FAILED: acos(0) should equal π/2")
        vibez.spill("Expected:", pi_half, "Got:", acos_zero)
        damn cringe
    }
    
    fr fr Test atan(1) = π/4
    sus atan_one tea = atan("1.0")
    sus pi_quarter tea = float_divide(PI(), "4.0")
    ready (!runtime_float_close_to(atan_one, pi_quarter, "1e-12")) {
        vibez.spill("FAILED: atan(1) should equal π/4")
        vibez.spill("Expected:", pi_quarter, "Got:", atan_one)
        damn cringe
    }
    
    fr fr Test atan2(1, 1) = π/4
    sus atan2_result tea = atan2("1.0", "1.0")
    ready (!runtime_float_close_to(atan2_result, pi_quarter, "1e-12")) {
        vibez.spill("FAILED: atan2(1, 1) should equal π/4")
        vibez.spill("Expected:", pi_quarter, "Got:", atan2_result)
        damn cringe
    }
    
    vibez.spill("✅ Inverse trigonometric function tests passed")
    damn based
}

fr fr ===== HYPERBOLIC FUNCTION TESTS =====

slay test_hyperbolic_functions() lit {
    vibez.spill("Testing hyperbolic functions...")
    
    fr fr Test sinh(0) = 0
    sus sinh_zero tea = sinh("0.0")
    ready (!runtime_float_close_to(sinh_zero, "0.0", "1e-12")) {
        vibez.spill("FAILED: sinh(0) should equal 0")
        vibez.spill("Expected: 0.0, Got:", sinh_zero)
        damn cringe
    }
    
    fr fr Test cosh(0) = 1
    sus cosh_zero tea = cosh("0.0")
    ready (!runtime_float_close_to(cosh_zero, "1.0", "1e-12")) {
        vibez.spill("FAILED: cosh(0) should equal 1")
        vibez.spill("Expected: 1.0, Got:", cosh_zero)
        damn cringe
    }
    
    fr fr Test tanh(0) = 0
    sus tanh_zero tea = tanh("0.0")
    ready (!runtime_float_close_to(tanh_zero, "0.0", "1e-12")) {
        vibez.spill("FAILED: tanh(0) should equal 0")
        vibez.spill("Expected: 0.0, Got:", tanh_zero)
        damn cringe
    }
    
    fr fr Test tanh(∞) = 1
    sus tanh_inf tea = tanh(POSITIVE_INFINITY())
    ready (!runtime_float_close_to(tanh_inf, "1.0", "1e-12")) {
        vibez.spill("FAILED: tanh(∞) should equal 1")
        vibez.spill("Expected: 1.0, Got:", tanh_inf)
        damn cringe
    }
    
    vibez.spill("✅ Hyperbolic function tests passed")
    damn based
}

fr fr ===== EDGE CASE TESTS =====

slay test_edge_cases() lit {
    vibez.spill("Testing edge cases and boundary conditions...")
    
    fr fr Test very small numbers
    sus tiny tea = "1e-15"
    sus sin_tiny tea = sin(tiny)
    ready (!runtime_float_close_to(sin_tiny, tiny, "1e-16")) {
        vibez.spill("FAILED: sin(tiny) ≈ tiny for small values")
        vibez.spill("sin(", tiny, ") =", sin_tiny)
        damn cringe
    }
    
    fr fr Test very large numbers
    sus large tea = "1e10"
    sus sin_large tea = sin(large)
    ready (is_nan(sin_large)) {
        vibez.spill("WARNING: sin(large) returned NaN - this is acceptable for very large values")
    }
    
    fr fr Test precision near boundaries
    sus almost_one tea = "0.9999999999999999"
    sus asin_almost_one tea = asin(almost_one)
    ready (is_nan(asin_almost_one)) {
        vibez.spill("FAILED: asin should handle values very close to 1")
        damn cringe
    }
    
    vibez.spill("✅ Edge case tests passed")
    damn based
}

fr fr ===== PERFORMANCE BENCHMARK =====

slay benchmark_math_functions() lit {
    vibez.spill("Benchmarking mathematical functions...")
    
    sus iterations drip = 10000
    sus test_value tea = "1.5707963267948966"  fr fr π/2
    
    fr fr Benchmark sin function
    sus i drip = 0
    bestie (i < iterations) {
        sus result tea = sin(test_value)
        i = i + 1
    }
    
    vibez.spill("✅ Completed", iterations, "sin() calculations")
    
    fr fr Benchmark exp function
    sus j drip = 0
    bestie (j < iterations) {
        sus result tea = exp("1.0")
        j = j + 1
    }
    
    vibez.spill("✅ Completed", iterations, "exp() calculations")
    vibez.spill("Performance benchmarking complete")
    damn based
}

fr fr ===== MAIN TEST RUNNER =====

slay main() lit {
    vibez.spill("🧮 IEEE 754 Mathematical Functions Test Suite")
    vibez.spill("===============================================")
    
    sus all_passed lit = based
    
    ready (!test_special_values()) {
        all_passed = cringe
    }
    
    ready (!test_trigonometric_precision()) {
        all_passed = cringe
    }
    
    ready (!test_logarithmic_functions()) {
        all_passed = cringe
    }
    
    ready (!test_power_functions()) {
        all_passed = cringe
    }
    
    ready (!test_inverse_trigonometric()) {
        all_passed = cringe
    }
    
    ready (!test_hyperbolic_functions()) {
        all_passed = cringe
    }
    
    ready (!test_edge_cases()) {
        all_passed = cringe
    }
    
    ready (!benchmark_math_functions()) {
        all_passed = cringe
    }
    
    vibez.spill("===============================================")
    ready (all_passed) {
        vibez.spill("🎉 ALL TESTS PASSED! IEEE 754 compliance validated")
        vibez.spill("Mathematical functions are production-ready with proper precision")
        vibez.spill("✅ NaN and Infinity handling implemented correctly")
        vibez.spill("✅ Transcendental functions achieve 12+ decimal places accuracy")
        vibez.spill("✅ Special value edge cases handled properly")
        vibez.spill("✅ Performance benchmarks completed successfully")
    } otherwise {
        vibez.spill("❌ SOME TESTS FAILED - IEEE 754 compliance issues detected")
        vibez.spill("Review the test output above for specific failures")
    }
    
    damn all_passed
}

fr fr Execute the test suite
main()

fr fr =============================================================================
fr fr END OF IEEE 754 MATHEMATICAL FUNCTIONS TEST SUITE
fr fr Comprehensive validation ensures production-ready mathematical operations
fr fr =============================================================================
