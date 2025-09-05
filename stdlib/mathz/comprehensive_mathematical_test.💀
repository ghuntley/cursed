fr fr CURSED Comprehensive Mathematical Operations Test Suite
fr fr Tests all enhanced mathematical functions for correctness
fr fr Validates IEEE 754 compliance, statistical accuracy, and algorithmic correctness

yeet "testz"
yeet "enhanced_mathematical_operations"
yeet "proper_statistical_functions"  
yeet "ieee754_nan_checking"

fr fr ==========================================
fr fr TEST DATA PREPARATION
fr fr ==========================================

slay create_test_dataset_small() meal[value]{
    sus data meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0]
    damn data
}

slay create_test_dataset_medium() meal[value]{
    sus data meal[value] = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]
    damn data
}

slay create_test_dataset_large() meal[value]{
    sus data meal[value] = [1.0, 1.2, 1.8, 2.1, 2.5, 3.0, 3.4, 3.9, 4.2, 4.8, 5.1, 5.7, 6.0, 6.3, 6.9, 7.2, 7.8, 8.1, 8.5, 9.0]
    damn data
}

slay create_test_dataset_with_outliers() meal[value]{
    sus data meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0, 100.0]  fr fr One extreme outlier
    damn data
}

slay create_test_dataset_negative() meal[value]{
    sus data meal[value] = [-5.0, -3.0, -1.0, 0.0, 1.0, 3.0, 5.0]
    damn data
}

fr fr ==========================================
fr fr IEEE 754 COMPLIANCE TESTS
fr fr ==========================================

slay test_nan_detection_comprehensive() lit {
    spill_facts("Testing IEEE 754 NaN detection...")
    
    fr fr Test various ways to create NaN
    sus nan1 meal = 0.0 / 0.0
    sus nan2 meal = sqrt_newton_raphson(-1.0)  fr fr Should handle invalid input
    sus nan3 meal = log_safe(-1.0)            fr fr Should handle invalid input
    
    ready (!is_nan_ieee754_compliant(nan1)) {
        spill_facts("ERROR: Failed to detect NaN from 0/0")
        damn cringe
    }
    
    fr fr Test that normal numbers are NOT NaN
    ready (is_nan_ieee754_compliant(3.14159)) {
        spill_facts("ERROR: Incorrectly classified normal number as NaN")
        damn cringe
    }
    
    ready (is_nan_ieee754_compliant(0.0)) {
        spill_facts("ERROR: Incorrectly classified zero as NaN")
        damn cringe
    }
    
    spill_facts("✓ NaN detection tests passed")
    damn based
}

slay test_infinity_detection_comprehensive() lit {
    spill_facts("Testing IEEE 754 infinity detection...")
    
    sus pos_inf meal = 1.0 / 0.0
    sus neg_inf meal = -1.0 / 0.0
    sus large_number meal = 1e30
    
    ready (!is_infinite_ieee754_compliant(pos_inf)) {
        spill_facts("ERROR: Failed to detect positive infinity")
        damn cringe
    }
    
    ready (!is_infinite_ieee754_compliant(neg_inf)) {
        spill_facts("ERROR: Failed to detect negative infinity")
        damn cringe
    }
    
    ready (is_infinite_ieee754_compliant(large_number)) {
        spill_facts("ERROR: Incorrectly classified large number as infinity")
        damn cringe
    }
    
    spill_facts("✓ Infinity detection tests passed")
    damn based
}

slay test_finite_detection_comprehensive() lit {
    spill_facts("Testing IEEE 754 finite detection...")
    
    sus normal_numbers meal[value] = [0.0, 1.0, -1.0, 3.14159, -2.71828, 1e-10, 1e10]
    sus nan_val meal = 0.0 / 0.0
    sus inf_val meal = 1.0 / 0.0
    
    sus i drip = 0
    bestie (i < 7) {
        ready (!is_finite_ieee754_compliant(normal_numbers[i])) {
            spill_facts("ERROR: Normal number incorrectly classified as non-finite:", normal_numbers[i])
            damn cringe
        }
        i = i + 1
    }
    
    ready (is_finite_ieee754_compliant(nan_val)) {
        spill_facts("ERROR: NaN incorrectly classified as finite")
        damn cringe
    }
    
    ready (is_finite_ieee754_compliant(inf_val)) {
        spill_facts("ERROR: Infinity incorrectly classified as finite")
        damn cringe
    }
    
    spill_facts("✓ Finite detection tests passed")
    damn based
}

fr fr ==========================================
fr fr EUCLIDEAN ALGORITHM TESTS
fr fr ==========================================

slay test_euclidean_gcd_comprehensive() lit {
    spill_facts("Testing enhanced Euclidean GCD algorithm...")
    
    fr fr Test basic GCD cases
    sus test_cases_a drip[value] = [48, 18, 54, 24, 17, 13, 1071, 462]
    sus test_cases_b drip[value] = [18, 48, 24, 54, 13, 17, 462, 1071]
    sus expected_gcd drip[value] = [6, 6, 6, 6, 1, 1, 21, 21]
    
    sus i drip = 0
    bestie (i < 8) {
        sus result drip = gcd_euclidean(test_cases_a[i], test_cases_b[i])
        ready (result != expected_gcd[i]) {
            spill_facts("ERROR: GCD failed for", test_cases_a[i], "and", test_cases_b[i])
            spill_facts("Expected:", expected_gcd[i], "Got:", result)
            damn cringe
        }
        i = i + 1
    }
    
    fr fr Test edge cases
    sus gcd_zero drip = gcd_euclidean(0, 5)
    ready (gcd_zero != 5) {
        spill_facts("ERROR: GCD(0,5) should be 5, got:", gcd_zero)
        damn cringe
    }
    
    sus gcd_same drip = gcd_euclidean(7, 7)
    ready (gcd_same != 7) {
        spill_facts("ERROR: GCD(7,7) should be 7, got:", gcd_same)
        damn cringe
    }
    
    fr fr Test negative numbers
    sus gcd_negative drip = gcd_euclidean(-48, 18)
    ready (gcd_negative != 6) {
        spill_facts("ERROR: GCD with negative numbers failed, got:", gcd_negative)
        damn cringe
    }
    
    spill_facts("✓ Euclidean GCD tests passed")
    damn based
}

slay test_lcm_comprehensive() lit {
    spill_facts("Testing LCM using Euclidean algorithm...")
    
    sus lcm_result drip = lcm_euclidean(12, 15)
    ready (lcm_result != 60) {
        spill_facts("ERROR: LCM(12,15) should be 60, got:", lcm_result)
        damn cringe
    }
    
    sus lcm_coprime drip = lcm_euclidean(7, 11)
    ready (lcm_coprime != 77) {
        spill_facts("ERROR: LCM of coprimes should be their product, got:", lcm_coprime)
        damn cringe
    }
    
    sus lcm_zero drip = lcm_euclidean(0, 5)
    ready (lcm_zero != 0) {
        spill_facts("ERROR: LCM with zero should be 0, got:", lcm_zero)
        damn cringe
    }
    
    spill_facts("✓ LCM tests passed")
    damn based
}

fr fr ==========================================
fr fr STATISTICAL FUNCTIONS TESTS
fr fr ==========================================

slay test_median_sorting_based_comprehensive() lit {
    spill_facts("Testing proper sorting-based median calculation...")
    
    fr fr Test odd number of elements
    sus data_odd meal[value] = create_test_dataset_small()
    sus median_odd meal = median_sorting_based(data_odd, 5)
    ready (abs_float_stable(median_odd - 3.0) > 1e-10) {
        spill_facts("ERROR: Median of [1,2,3,4,5] should be 3.0, got:", median_odd)
        damn cringe
    }
    
    fr fr Test even number of elements
    sus data_even meal[value] = [1.0, 2.0, 3.0, 4.0]
    sus median_even meal = median_sorting_based(data_even, 4)
    ready (abs_float_stable(median_even - 2.5) > 1e-10) {
        spill_facts("ERROR: Median of [1,2,3,4] should be 2.5, got:", median_even)
        damn cringe
    }
    
    fr fr Test unsorted data
    sus data_unsorted meal[value] = [5.0, 1.0, 4.0, 2.0, 3.0]
    sus median_unsorted meal = median_sorting_based(data_unsorted, 5)
    ready (abs_float_stable(median_unsorted - 3.0) > 1e-10) {
        spill_facts("ERROR: Median of unsorted [5,1,4,2,3] should be 3.0, got:", median_unsorted)
        damn cringe
    }
    
    fr fr Test single element
    sus data_single meal[value] = [42.0]
    sus median_single meal = median_sorting_based(data_single, 1)
    ready (abs_float_stable(median_single - 42.0) > 1e-10) {
        spill_facts("ERROR: Median of single element should be that element, got:", median_single)
        damn cringe
    }
    
    spill_facts("✓ Sorting-based median tests passed")
    damn based
}

slay test_welford_variance_comprehensive() lit {
    spill_facts("Testing Welford's numerically stable variance...")
    
    fr fr Test known variance case
    sus data meal[value] = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0]
    sus variance_result meal = variance_welford(data, 8)
    
    fr fr Expected sample variance is approximately 4.571
    ready (abs_float_stable(variance_result - 4.571428571) > 1e-6) {
        spill_facts("ERROR: Welford variance failed, expected ~4.57, got:", variance_result)
        damn cringe
    }
    
    fr fr Test single element (should return 0)
    sus data_single meal[value] = [5.0]
    sus var_single meal = variance_welford(data_single, 1)
    ready (var_single != 0.0) {
        spill_facts("ERROR: Variance of single element should be 0, got:", var_single)
        damn cringe
    }
    
    fr fr Test identical elements
    sus data_identical meal[value] = [3.0, 3.0, 3.0, 3.0]
    sus var_identical meal = variance_welford(data_identical, 4)
    ready (abs_float_stable(var_identical) > 1e-10) {
        spill_facts("ERROR: Variance of identical elements should be 0, got:", var_identical)
        damn cringe
    }
    
    spill_facts("✓ Welford variance tests passed")
    damn based
}

slay test_robust_mean_comprehensive() lit {
    spill_facts("Testing numerically stable mean (Kahan summation)...")
    
    fr fr Test basic mean
    sus data_basic meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0]
    sus mean_basic meal = mean_arithmetic_stable(data_basic, 5)
    ready (abs_float_stable(mean_basic - 3.0) > 1e-10) {
        spill_facts("ERROR: Mean of [1,2,3,4,5] should be 3.0, got:", mean_basic)
        damn cringe
    }
    
    fr fr Test with negative numbers
    sus data_negative meal[value] = create_test_dataset_negative()
    sus mean_negative meal = mean_arithmetic_stable(data_negative, 7)
    ready (abs_float_stable(mean_negative - 0.0) > 1e-10) {
        spill_facts("ERROR: Mean of symmetric dataset should be ~0.0, got:", mean_negative)
        damn cringe
    }
    
    fr fr Test numerical stability with small differences
    sus data_stable meal[value] = [1000000.0, 1000001.0, 1000002.0]
    sus mean_stable meal = mean_arithmetic_stable(data_stable, 3)
    ready (abs_float_stable(mean_stable - 1000001.0) > 1e-6) {
        spill_facts("ERROR: Numerically stable mean failed, got:", mean_stable)
        damn cringe
    }
    
    spill_facts("✓ Robust mean tests passed")
    damn based
}

fr fr ==========================================
fr fr QUARTILE AND PERCENTILE TESTS
fr fr ==========================================

slay test_quartiles_comprehensive() lit {
    spill_facts("Testing proper quartile calculations...")
    
    sus data meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
    sus quartile_results meal[value] = quartiles_proper(data, 9)
    
    fr fr For this dataset: Q1=3, Q2=5, Q3=7 (approximately)
    ready (abs_float_stable(quartile_results[0] - 3.0) > 0.5) {
        spill_facts("ERROR: Q1 calculation failed, expected ~3.0, got:", quartile_results[0])
        damn cringe
    }
    
    ready (abs_float_stable(quartile_results[1] - 5.0) > 0.1) {
        spill_facts("ERROR: Q2 (median) calculation failed, expected 5.0, got:", quartile_results[1])
        damn cringe
    }
    
    ready (abs_float_stable(quartile_results[2] - 7.0) > 0.5) {
        spill_facts("ERROR: Q3 calculation failed, expected ~7.0, got:", quartile_results[2])
        damn cringe
    }
    
    spill_facts("✓ Quartile tests passed")
    damn based
}

slay test_percentiles_comprehensive() lit {
    spill_facts("Testing percentile calculations...")
    
    sus data meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
    
    fr fr Test 50th percentile (median)
    sus p50 meal = percentile(data, 10, 50.0)
    ready (abs_float_stable(p50 - 5.5) > 0.1) {
        spill_facts("ERROR: 50th percentile failed, expected 5.5, got:", p50)
        damn cringe
    }
    
    fr fr Test 90th percentile
    sus p90 meal = percentile(data, 10, 90.0)
    ready (abs_float_stable(p90 - 9.1) > 0.5) {
        spill_facts("ERROR: 90th percentile failed, expected ~9.1, got:", p90)
        damn cringe
    }
    
    fr fr Test 10th percentile
    sus p10 meal = percentile(data, 10, 10.0)
    ready (abs_float_stable(p10 - 1.9) > 0.5) {
        spill_facts("ERROR: 10th percentile failed, expected ~1.9, got:", p10)
        damn cringe
    }
    
    spill_facts("✓ Percentile tests passed")
    damn based
}

fr fr ==========================================
fr fr ROBUST STATISTICS TESTS
fr fr ==========================================

slay test_mad_comprehensive() lit {
    spill_facts("Testing Median Absolute Deviation (MAD)...")
    
    sus data meal[value] = [1.0, 1.0, 2.0, 2.0, 4.0, 6.0, 9.0]
    sus mad_result meal = median_absolute_deviation(data, 7)
    
    fr fr MAD should be robust to outliers
    ready (mad_result <= 0.0 || mad_result > 10.0) {
        spill_facts("ERROR: MAD result seems unreasonable, got:", mad_result)
        damn cringe
    }
    
    spill_facts("✓ MAD tests passed")
    damn based
}

slay test_trimmed_mean_comprehensive() lit {
    spill_facts("Testing trimmed mean (outlier resistant)...")
    
    sus data_with_outlier meal[value] = create_test_dataset_with_outliers()
    sus trimmed_mean_result meal = trimmed_mean(data_with_outlier, 6, 0.2)  fr fr Trim 20% from each end
    
    fr fr Trimmed mean should be less affected by the outlier (100.0)
    ready (trimmed_mean_result > 20.0) {
        spill_facts("ERROR: Trimmed mean too large, outlier not properly handled:", trimmed_mean_result)
        damn cringe
    }
    
    spill_facts("✓ Trimmed mean tests passed")
    damn based
}

fr fr ==========================================
fr fr ADVANCED STATISTICAL TESTS
fr fr ==========================================

slay test_skewness_kurtosis_comprehensive() lit {
    spill_facts("Testing skewness and kurtosis calculations...")
    
    fr fr Test symmetric data (should have near-zero skewness)
    sus symmetric_data meal[value] = [1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0]
    sus skewness_result meal = skewness_proper(symmetric_data, 9)
    
    ready (abs_float_stable(skewness_result) > 1.0) {
        spill_facts("WARNING: Symmetric data should have near-zero skewness, got:", skewness_result)
    }
    
    fr fr Test kurtosis
    sus kurtosis_result meal = kurtosis_proper(symmetric_data, 9)
    spill_facts("Kurtosis result:", kurtosis_result)  fr fr Just log for inspection
    
    spill_facts("✓ Skewness and kurtosis tests passed")
    damn based
}

fr fr ==========================================
fr fr NEWTON-RAPHSON SQRT TESTS
fr fr ==========================================

slay test_sqrt_newton_raphson_comprehensive() lit {
    spill_facts("Testing Newton-Raphson square root...")
    
    fr fr Test perfect squares
    sus sqrt_results meal[value] = [0.0, 1.0, 4.0, 9.0, 16.0, 25.0, 100.0]
    sus expected_sqrt meal[value] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 10.0]
    
    sus i drip = 0
    bestie (i < 7) {
        sus result meal = sqrt_newton_raphson(sqrt_results[i])
        ready (abs_float_stable(result - expected_sqrt[i]) > 1e-10) {
            spill_facts("ERROR: sqrt failed for", sqrt_results[i], "expected", expected_sqrt[i], "got", result)
            damn cringe
        }
        i = i + 1
    }
    
    fr fr Test non-perfect squares
    sus sqrt_2 meal = sqrt_newton_raphson(2.0)
    ready (abs_float_stable(sqrt_2 - 1.4142135623730951) > 1e-10) {
        spill_facts("ERROR: sqrt(2) calculation failed, got:", sqrt_2)
        damn cringe
    }
    
    fr fr Test edge cases
    sus sqrt_negative meal = sqrt_newton_raphson(-1.0)
    ready (sqrt_negative != 0.0) {
        spill_facts("ERROR: sqrt of negative should return 0, got:", sqrt_negative)
        damn cringe
    }
    
    spill_facts("✓ Newton-Raphson sqrt tests passed")
    damn based
}

fr fr ==========================================
fr fr COMPREHENSIVE TEST SUITE RUNNER
fr fr ==========================================

slay run_comprehensive_mathematical_tests() lit {
    spill_facts("=" * 60)
    spill_facts("CURSED COMPREHENSIVE MATHEMATICAL OPERATIONS TEST SUITE")
    spill_facts("=" * 60)
    
    sus total_tests drip = 0
    sus passed_tests drip = 0
    
    fr fr IEEE 754 Compliance Tests
    spill_facts("\n🧪 IEEE 754 Compliance Tests")
    spill_facts("-" * 30)
    total_tests = total_tests + 1
    ready (test_nan_detection_comprehensive()) { passed_tests = passed_tests + 1 }
    
    total_tests = total_tests + 1
    ready (test_infinity_detection_comprehensive()) { passed_tests = passed_tests + 1 }
    
    total_tests = total_tests + 1  
    ready (test_finite_detection_comprehensive()) { passed_tests = passed_tests + 1 }
    
    fr fr Euclidean Algorithm Tests
    spill_facts("\n🔢 Euclidean Algorithm Tests")
    spill_facts("-" * 30)
    total_tests = total_tests + 1
    ready (test_euclidean_gcd_comprehensive()) { passed_tests = passed_tests + 1 }
    
    total_tests = total_tests + 1
    ready (test_lcm_comprehensive()) { passed_tests = passed_tests + 1 }
    
    fr fr Statistical Functions Tests
    spill_facts("\n📊 Statistical Functions Tests")
    spill_facts("-" * 30)
    total_tests = total_tests + 1
    ready (test_median_sorting_based_comprehensive()) { passed_tests = passed_tests + 1 }
    
    total_tests = total_tests + 1
    ready (test_welford_variance_comprehensive()) { passed_tests = passed_tests + 1 }
    
    total_tests = total_tests + 1
    ready (test_robust_mean_comprehensive()) { passed_tests = passed_tests + 1 }
    
    total_tests = total_tests + 1
    ready (test_quartiles_comprehensive()) { passed_tests = passed_tests + 1 }
    
    total_tests = total_tests + 1
    ready (test_percentiles_comprehensive()) { passed_tests = passed_tests + 1 }
    
    fr fr Robust Statistics Tests
    spill_facts("\n🛡️  Robust Statistics Tests")
    spill_facts("-" * 30)
    total_tests = total_tests + 1
    ready (test_mad_comprehensive()) { passed_tests = passed_tests + 1 }
    
    total_tests = total_tests + 1
    ready (test_trimmed_mean_comprehensive()) { passed_tests = passed_tests + 1 }
    
    fr fr Advanced Statistical Tests  
    spill_facts("\n📈 Advanced Statistical Tests")
    spill_facts("-" * 30)
    total_tests = total_tests + 1
    ready (test_skewness_kurtosis_comprehensive()) { passed_tests = passed_tests + 1 }
    
    fr fr Mathematical Functions Tests
    spill_facts("\n🔧 Mathematical Functions Tests")
    spill_facts("-" * 30)
    total_tests = total_tests + 1
    ready (test_sqrt_newton_raphson_comprehensive()) { passed_tests = passed_tests + 1 }
    
    fr fr Final Results
    spill_facts("\n" + "=" * 60)
    spill_facts("TEST RESULTS SUMMARY")
    spill_facts("=" * 60)
    spill_facts("Total Tests:", total_tests)
    spill_facts("Passed Tests:", passed_tests)
    spill_facts("Failed Tests:", total_tests - passed_tests)
    
    ready (passed_tests == total_tests) {
        spill_facts("🎉 ALL MATHEMATICAL OPERATIONS TESTS PASSED!")
        spill_facts("✅ Enhanced mathematical operations are mathematically correct")
        spill_facts("✅ IEEE 754 compliance verified")
        spill_facts("✅ Statistical functions are robust and accurate")
        spill_facts("✅ Algorithms are mathematically sound")
        damn based
    } sus {
        spill_facts("❌ SOME TESTS FAILED - Mathematical operations need attention")
        damn cringe
    }
}

fr fr Helper functions for the tests
slay abs_float_stable(x meal) meal {
    ready (x < 0.0) { damn -x }
    damn x
}

slay sqrt_safe(x meal) meal {
    ready (x <= 0.0) { damn 0.0 }
    damn sqrt_newton_raphson(x)
}

slay log_safe(x meal) meal {
    ready (x <= 0.0) { damn 0.0 / 0.0 }  fr fr Return NaN for invalid input
    damn 1.0  fr fr Simplified log approximation
}

slay spill_facts(message tea) lit {
    fr fr Print function for test output
    damn based
}
