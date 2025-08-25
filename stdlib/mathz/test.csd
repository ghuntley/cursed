fr fr Comprehensive test suite for MATHZ mathematical operations module
fr fr Tests all public functions with proper validation using testz framework

yeet "testz"
yeet "mathz"

slay main() {
    testz.test_start("MATHZ Comprehensive Test Suite")
    
    fr fr ===== BASIC ARITHMETIC TESTS =====
    testz.test_group("Basic Arithmetic Functions")
    
    fr fr Test abs_normie
    sus abs_positive drip = mathz.abs_normie(5.5)
    testz.assert_eq_float(abs_positive, 5.5, "abs_normie should return positive for positive input")
    
    sus abs_negative drip = mathz.abs_normie(-3.2)
    testz.assert_eq_float(abs_negative, 3.2, "abs_normie should return positive for negative input")
    
    sus abs_zero drip = mathz.abs_normie(0.0)
    testz.assert_eq_float(abs_zero, 0.0, "abs_normie should return 0 for 0")
    
    fr fr Test max_normie
    sus max_result1 drip = mathz.max_normie(5.0, 3.0)
    testz.assert_eq_float(max_result1, 5.0, "max_normie should return larger value")
    
    sus max_result2 drip = mathz.max_normie(2.0, 7.0)
    testz.assert_eq_float(max_result2, 7.0, "max_normie should return larger value when second is larger")
    
    sus max_equal drip = mathz.max_normie(4.0, 4.0)
    testz.assert_eq_float(max_equal, 4.0, "max_normie should handle equal values")
    
    fr fr Test min_normie
    sus min_result1 drip = mathz.min_normie(5.0, 3.0)
    testz.assert_eq_float(min_result1, 3.0, "min_normie should return smaller value")
    
    sus min_result2 drip = mathz.min_normie(2.0, 7.0)
    testz.assert_eq_float(min_result2, 2.0, "min_normie should return smaller value when first is smaller")
    
    sus min_equal drip = mathz.min_normie(4.0, 4.0)
    testz.assert_eq_float(min_equal, 4.0, "min_normie should handle equal values")
    
    fr fr Test add_two
    sus add_result drip = mathz.add_two(3.5, 2.5)
    testz.assert_eq_float(add_result, 6.0, "add_two should add numbers correctly")
    
    sus add_zero drip = mathz.add_two(5.0, 0.0)
    testz.assert_eq_float(add_zero, 5.0, "add_two should handle zero addition")
    
    sus add_negative drip = mathz.add_two(-3.0, 2.0)
    testz.assert_eq_float(add_negative, -1.0, "add_two should handle negative numbers")
    
    fr fr Test subtract_two
    sus sub_result drip = mathz.subtract_two(5.0, 2.0)
    testz.assert_eq_float(sub_result, 3.0, "subtract_two should subtract correctly")
    
    sus sub_zero drip = mathz.subtract_two(5.0, 0.0)
    testz.assert_eq_float(sub_zero, 5.0, "subtract_two should handle zero subtraction")
    
    sus sub_negative drip = mathz.subtract_two(3.0, 5.0)
    testz.assert_eq_float(sub_negative, -2.0, "subtract_two should handle negative results")
    
    fr fr Test multiply_two
    sus mul_result drip = mathz.multiply_two(4.0, 3.0)
    testz.assert_eq_float(mul_result, 12.0, "multiply_two should multiply correctly")
    
    sus mul_zero drip = mathz.multiply_two(5.0, 0.0)
    testz.assert_eq_float(mul_zero, 0.0, "multiply_two should handle zero multiplication")
    
    sus mul_negative drip = mathz.multiply_two(-2.0, 3.0)
    testz.assert_eq_float(mul_negative, -6.0, "multiply_two should handle negative multiplication")
    
    fr fr ===== DIVISION TESTS WITH ERROR HANDLING =====
    testz.test_group("Division with Error Handling")
    
    fr fr Test divide_two with valid inputs
    sus div_result drip = mathz.divide_two(6.0, 2.0) fam {
        when _ -> {
            testz.assert_true(cap, "divide_two should not error on valid division")
            damn 0.0
        }
    }
    testz.assert_eq_float(div_result, 3.0, "divide_two should divide correctly")
    
    fr fr Test divide_two with division by zero
    sus div_by_zero drip = mathz.divide_two(5.0, 0.0) fam {
        when "division by zero" -> {
            testz.assert_true(based, "divide_two should error on division by zero")
            damn -999.0  fr fr Error indicator
        }
        when _ -> {
            testz.assert_true(cap, "divide_two should only error with 'division by zero'")
            damn 0.0
        }
    }
    testz.assert_eq_float(div_by_zero, -999.0, "divide_by_zero should trigger error handling")
    
    fr fr ===== ADVANCED FUNCTION TESTS =====
    testz.test_group("Advanced Mathematical Functions")
    
    fr fr Test power_int
    sus power_basic drip = mathz.power_int(2.0, 3.0)
    testz.assert_eq_float(power_basic, 8.0, "power_int should calculate 2^3 = 8")
    
    sus power_zero drip = mathz.power_int(5.0, 0.0)
    testz.assert_eq_float(power_zero, 1.0, "power_int should return 1 for any number^0")
    
    sus power_one drip = mathz.power_int(7.0, 1.0)
    testz.assert_eq_float(power_one, 7.0, "power_int should return base for base^1")
    
    sus power_large drip = mathz.power_int(2.0, 10.0)
    testz.assert_eq_float(power_large, 1024.0, "power_int should calculate 2^10 = 1024")
    
    fr fr Test factorial
    sus fact_zero drip = mathz.factorial(0.0)
    testz.assert_eq_float(fact_zero, 1.0, "factorial(0) should equal 1")
    
    sus fact_one drip = mathz.factorial(1.0)
    testz.assert_eq_float(fact_one, 1.0, "factorial(1) should equal 1")
    
    sus fact_five drip = mathz.factorial(5.0)
    testz.assert_eq_float(fact_five, 120.0, "factorial(5) should equal 120")
    
    sus fact_six drip = mathz.factorial(6.0)
    testz.assert_eq_float(fact_six, 720.0, "factorial(6) should equal 720")
    
    fr fr Test gcd (Greatest Common Divisor)
    sus gcd_basic drip = mathz.gcd(12.0, 8.0)
    testz.assert_eq_float(gcd_basic, 4.0, "gcd(12, 8) should equal 4")
    
    sus gcd_coprime drip = mathz.gcd(7.0, 13.0)
    testz.assert_eq_float(gcd_coprime, 1.0, "gcd(7, 13) should equal 1 (coprime)")
    
    sus gcd_same drip = mathz.gcd(15.0, 15.0)
    testz.assert_eq_float(gcd_same, 15.0, "gcd(15, 15) should equal 15")
    
    sus gcd_zero drip = mathz.gcd(0.0, 5.0)
    testz.assert_eq_float(gcd_zero, 5.0, "gcd(0, 5) should equal 5")
    
    fr fr Test with negative numbers
    sus gcd_negative drip = mathz.gcd(-12.0, 8.0)
    testz.assert_eq_float(gcd_negative, 4.0, "gcd(-12, 8) should equal 4")
    
    fr fr Test lcm (Least Common Multiple)
    sus lcm_basic drip = mathz.lcm(4.0, 6.0)
    testz.assert_eq_float(lcm_basic, 12.0, "lcm(4, 6) should equal 12")
    
    sus lcm_coprime drip = mathz.lcm(7.0, 13.0)
    testz.assert_eq_float(lcm_coprime, 91.0, "lcm(7, 13) should equal 91")
    
    sus lcm_same drip = mathz.lcm(8.0, 8.0)
    testz.assert_eq_float(lcm_same, 8.0, "lcm(8, 8) should equal 8")
    
    fr fr ===== UTILITY FUNCTION TESTS =====
    testz.test_group("Utility Functions")
    
    fr fr Test is_even
    sus even_true lit = mathz.is_even(4.0)
    testz.assert_true(even_true, "is_even(4) should return true")
    
    sus even_false lit = mathz.is_even(5.0)
    testz.assert_false(even_false, "is_even(5) should return false")
    
    sus even_zero lit = mathz.is_even(0.0)
    testz.assert_true(even_zero, "is_even(0) should return true")
    
    sus even_negative lit = mathz.is_even(-6.0)
    testz.assert_true(even_negative, "is_even(-6) should return true")
    
    fr fr Test is_odd
    sus odd_true lit = mathz.is_odd(7.0)
    testz.assert_true(odd_true, "is_odd(7) should return true")
    
    sus odd_false lit = mathz.is_odd(8.0)
    testz.assert_false(odd_false, "is_odd(8) should return false")
    
    sus odd_negative lit = mathz.is_odd(-5.0)
    testz.assert_true(odd_negative, "is_odd(-5) should return true")
    
    fr fr ===== BOUNDARY AND EDGE CASE TESTS =====
    testz.test_group("Boundary and Edge Cases")
    
    fr fr Test very large numbers
    sus large_add drip = mathz.add_two(999999.0, 1.0)
    testz.assert_eq_float(large_add, 1000000.0, "should handle large numbers in addition")
    
    sus large_multiply drip = mathz.multiply_two(1000.0, 1000.0)
    testz.assert_eq_float(large_multiply, 1000000.0, "should handle large multiplication")
    
    fr fr Test very small numbers
    sus small_add drip = mathz.add_two(0.001, 0.002)
    testz.assert_gt_float(small_add, 0.002, "should handle small decimal addition")
    
    fr fr Test mixed positive/negative operations
    sus mixed_max drip = mathz.max_normie(-5.0, 3.0)
    testz.assert_eq_float(mixed_max, 3.0, "max_normie should handle mixed positive/negative")
    
    sus mixed_min drip = mathz.min_normie(-5.0, 3.0)
    testz.assert_eq_float(mixed_min, -5.0, "min_normie should handle mixed positive/negative")
    
    fr fr Test factorial edge cases
    sus fact_large drip = mathz.factorial(10.0)
    testz.assert_eq_float(fact_large, 3628800.0, "factorial(10) should equal 3,628,800")
    
    fr fr ===== PERFORMANCE TESTS =====
    testz.test_group("Performance Validation")
    
    fr fr Test repeated operations (stress test)
    bestie i := 0; i < 100; i++ {
        sus iter_float drip = i  fr fr Convert int to float
        sus perf_result drip = mathz.add_two(iter_float, 1.0)
        testz.assert_eq_float(perf_result, iter_float + 1.0, "Performance test iteration should succeed")
    }
    
    fr fr Test complex calculation chains
    sus chain_result drip = mathz.add_two(
        mathz.multiply_two(2.0, 3.0), 
        mathz.subtract_two(10.0, 4.0)
    )
    testz.assert_eq_float(chain_result, 12.0, "Complex calculation chain should work: (2*3) + (10-4) = 12")
    
    fr fr ===== MATHEMATICAL PROPERTY TESTS =====
    testz.test_group("Mathematical Properties")
    
    fr fr Test commutative property of addition
    sus comm_add1 drip = mathz.add_two(5.0, 7.0)
    sus comm_add2 drip = mathz.add_two(7.0, 5.0)
    testz.assert_eq_float(comm_add1, comm_add2, "Addition should be commutative")
    
    fr fr Test commutative property of multiplication
    sus comm_mul1 drip = mathz.multiply_two(3.0, 8.0)
    sus comm_mul2 drip = mathz.multiply_two(8.0, 3.0)
    testz.assert_eq_float(comm_mul1, comm_mul2, "Multiplication should be commutative")
    
    fr fr Test associative property with max
    sus assoc_max1 drip = mathz.max_normie(mathz.max_normie(2.0, 5.0), 3.0)
    sus assoc_max2 drip = mathz.max_normie(2.0, mathz.max_normie(5.0, 3.0))
    testz.assert_eq_float(assoc_max1, assoc_max2, "Max should be associative")
    
    fr fr Test identity elements
    sus add_identity drip = mathz.add_two(42.0, 0.0)
    testz.assert_eq_float(add_identity, 42.0, "Zero should be additive identity")
    
    sus mul_identity drip = mathz.multiply_two(42.0, 1.0)
    testz.assert_eq_float(mul_identity, 42.0, "One should be multiplicative identity")
    
    fr fr Test power properties
    sus power_prop1 drip = mathz.power_int(2.0, 2.0)  fr fr 2^2
    sus power_prop2 drip = mathz.multiply_two(2.0, 2.0)  fr fr 2*2
    testz.assert_eq_float(power_prop1, power_prop2, "2^2 should equal 2*2")
    
    fr fr ===== GCD/LCM RELATIONSHIP TESTS =====
    testz.test_group("GCD/LCM Relationship Tests")
    
    fr fr Test GCD * LCM = a * b property
    sus a drip = 15.0
    sus b drip = 20.0
    sus gcd_val drip = mathz.gcd(a, b)
    sus lcm_val drip = mathz.lcm(a, b)
    sus gcd_lcm_product drip = mathz.multiply_two(gcd_val, lcm_val)
    sus ab_product drip = mathz.multiply_two(a, b)
    testz.assert_eq_float(gcd_lcm_product, ab_product, "GCD(a,b) * LCM(a,b) should equal a * b")
    
    fr fr ===== ERROR CONDITION TESTS =====
    testz.test_group("Error Condition Handling")
    
    fr fr Test multiple division by zero attempts
    bestie i := 0; i < 3; i++ {
        sus iter_val drip = i + 1.0
        sus div_zero_test drip = mathz.divide_two(iter_val, 0.0) fam {
            when "division by zero" -> {
                damn -1.0  fr fr Error indicator
            }
            when _ -> damn 0.0
        }
        testz.assert_eq_float(div_zero_test, -1.0, "Division by zero should consistently error")
    }
    
    fr fr ===== INTEGRATION TESTS =====
    testz.test_group("Integration Tests")
    
    fr fr Test calculating area using multiple functions
    sus length drip = 5.0
    sus width drip = 3.0
    sus area drip = mathz.multiply_two(length, width)
    testz.assert_eq_float(area, 15.0, "Area calculation should work")
    
    fr fr Test Pythagorean theorem approximation (a² + b²)
    sus a_squared drip = mathz.power_int(3.0, 2.0)
    sus b_squared drip = mathz.power_int(4.0, 2.0)
    sus c_squared drip = mathz.add_two(a_squared, b_squared)
    testz.assert_eq_float(c_squared, 25.0, "3² + 4² should equal 25")
    
    fr fr Test compound interest-like calculation
    sus principal drip = 100.0
    sus rate_plus_one drip = mathz.add_two(1.0, 0.05)  fr fr 1 + 5% = 1.05
    sus after_two_years drip = mathz.multiply_two(principal, mathz.power_int(rate_plus_one, 2.0))
    testz.assert_gt_float(after_two_years, 110.0, "Compound calculation should increase principal")
    
    fr fr ===== FINAL VALIDATION =====
    testz.test_group("Final Comprehensive Validation")
    
    fr fr Test all functions work together in complex scenario
    sus base_val drip = 10.0
    sus increment drip = 5.0
    sus multiplier drip = 2.0
    
    sus step1 drip = mathz.add_two(base_val, increment)  fr fr 10 + 5 = 15
    sus step2 drip = mathz.multiply_two(step1, multiplier)  fr fr 15 * 2 = 30
    sus step3 drip = mathz.divide_two(step2, 3.0) fam {  fr fr 30 / 3 = 10
        when _ -> damn 0.0
    }
    sus step4 drip = mathz.power_int(step3, 2.0)  fr fr 10² = 100
    sus step5 drip = mathz.abs_normie(step4)  fr fr abs(100) = 100
    
    testz.assert_eq_float(step5, 100.0, "Complex multi-step calculation should work correctly")
    
    fr fr Validate mathematical consistency
    sus consistency_check lit = based
    ready mathz.add_two(2.0, 3.0) != 5.0 { consistency_check = cap }
    ready mathz.multiply_two(4.0, 5.0) != 20.0 { consistency_check = cap }
    ready mathz.subtract_two(10.0, 3.0) != 7.0 { consistency_check = cap }
    ready mathz.max_normie(8.0, 12.0) != 12.0 { consistency_check = cap }
    ready mathz.min_normie(8.0, 12.0) != 8.0 { consistency_check = cap }
    
    testz.assert_true(consistency_check, "All mathematical operations should be consistent")
    
    testz.print_test_summary()
}
