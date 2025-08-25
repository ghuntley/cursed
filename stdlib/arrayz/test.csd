fr fr Comprehensive test suite for ARRAYZ array operations module
fr fr Tests all public functions with proper validation using testz framework

yeet "testz"
yeet "arrayz"

slay main() {
    testz.test_start("ARRAYZ Comprehensive Test Suite")
    
    fr fr ===== ARRAY ARITHMETIC TESTS =====
    testz.test_group("Array Arithmetic Functions")
    
    fr fr Test sum_array
    sus test_array []drip = [1.0, 2.0, 3.0, 4.0, 5.0]
    sus sum_result drip = arrayz.sum_array(test_array)
    testz.assert_eq_float(sum_result, 15.0, "sum_array should sum all elements: 1+2+3+4+5=15")
    
    fr fr Test sum_array with empty array
    sus empty_array []drip = []
    sus sum_empty drip = arrayz.sum_array(empty_array)
    testz.assert_eq_float(sum_empty, 0.0, "sum_array should return 0 for empty array")
    
    fr fr Test sum_array with single element
    sus single_array []drip = [42.0]
    sus sum_single drip = arrayz.sum_array(single_array)
    testz.assert_eq_float(sum_single, 42.0, "sum_array should return single element")
    
    fr fr Test sum_array with negative numbers
    sus negative_array []drip = [-1.0, -2.0, 3.0]
    sus sum_negative drip = arrayz.sum_array(negative_array)
    testz.assert_eq_float(sum_negative, 0.0, "sum_array should handle negative numbers: -1-2+3=0")
    
    fr fr Test average_array
    sus avg_result drip = arrayz.average_array(test_array)
    testz.assert_eq_float(avg_result, 3.0, "average_array should calculate mean: 15/5=3")
    
    fr fr Test average_array with empty array
    sus avg_empty drip = arrayz.average_array(empty_array)
    testz.assert_eq_float(avg_empty, 0.0, "average_array should return 0 for empty array")
    
    fr fr Test product_array
    sus product_test []drip = [2.0, 3.0, 4.0]
    sus product_result drip = arrayz.product_array(product_test)
    testz.assert_eq_float(product_result, 24.0, "product_array should multiply all elements: 2*3*4=24")
    
    fr fr Test product_array with empty array
    sus product_empty drip = arrayz.product_array(empty_array)
    testz.assert_eq_float(product_empty, 0.0, "product_array should return 0 for empty array")
    
    fr fr Test product_array with zero
    sus product_zero_array []drip = [1.0, 0.0, 5.0]
    sus product_zero drip = arrayz.product_array(product_zero_array)
    testz.assert_eq_float(product_zero, 0.0, "product_array should return 0 when array contains 0")
    
    fr fr ===== ARRAY SEARCH TESTS =====
    testz.test_group("Array Search Functions")
    
    fr fr Test find_max
    sus mixed_array []drip = [3.0, 7.0, 1.0, 9.0, 2.0]
    sus max_result drip = arrayz.find_max(mixed_array)
    testz.assert_eq_float(max_result, 9.0, "find_max should find largest element")
    
    fr fr Test find_max with single element
    sus max_single drip = arrayz.find_max(single_array)
    testz.assert_eq_float(max_single, 42.0, "find_max should return single element")
    
    fr fr Test find_max with empty array
    sus max_empty drip = arrayz.find_max(empty_array)
    testz.assert_eq_float(max_empty, 0.0, "find_max should return 0 for empty array")
    
    fr fr Test find_max with negative numbers
    sus neg_max_array []drip = [-5.0, -2.0, -8.0, -1.0]
    sus neg_max_result drip = arrayz.find_max(neg_max_array)
    testz.assert_eq_float(neg_max_result, -1.0, "find_max should find largest negative number")
    
    fr fr Test find_min
    sus min_result drip = arrayz.find_min(mixed_array)
    testz.assert_eq_float(min_result, 1.0, "find_min should find smallest element")
    
    fr fr Test find_min with single element
    sus min_single drip = arrayz.find_min(single_array)
    testz.assert_eq_float(min_single, 42.0, "find_min should return single element")
    
    fr fr Test find_min with empty array
    sus min_empty drip = arrayz.find_min(empty_array)
    testz.assert_eq_float(min_empty, 0.0, "find_min should return 0 for empty array")
    
    fr fr Test find_min with negative numbers
    sus neg_min_result drip = arrayz.find_min(neg_max_array)
    testz.assert_eq_float(neg_min_result, -8.0, "find_min should find smallest negative number")
    
    fr fr ===== ARRAY SEARCH VALUE TESTS =====
    testz.test_group("Array Value Search Functions")
    
    fr fr Test contains_value
    sus contains_true lit = arrayz.contains_value(mixed_array, 7.0)
    testz.assert_true(contains_true, "contains_value should return true for existing value")
    
    sus contains_false lit = arrayz.contains_value(mixed_array, 99.0)
    testz.assert_false(contains_false, "contains_value should return false for non-existing value")
    
    sus contains_empty lit = arrayz.contains_value(empty_array, 5.0)
    testz.assert_false(contains_empty, "contains_value should return false for empty array")
    
    fr fr Test find_index
    sus index_found drip = arrayz.find_index(mixed_array, 7.0)
    testz.assert_eq_float(index_found, 1.0, "find_index should return correct index for existing value")
    
    sus index_not_found drip = arrayz.find_index(mixed_array, 99.0)
    testz.assert_eq_float(index_not_found, -1.0, "find_index should return -1 for non-existing value")
    
    sus index_first drip = arrayz.find_index(mixed_array, 3.0)
    testz.assert_eq_float(index_first, 0.0, "find_index should return 0 for first element")
    
    sus index_empty drip = arrayz.find_index(empty_array, 5.0)
    testz.assert_eq_float(index_empty, -1.0, "find_index should return -1 for empty array")
    
    fr fr ===== EDGE CASE TESTS =====
    testz.test_group("Edge Case Handling")
    
    fr fr Test with very large numbers
    sus large_array []drip = [1000000.0, 2000000.0, 3000000.0]
    sus large_sum drip = arrayz.sum_array(large_array)
    testz.assert_eq_float(large_sum, 6000000.0, "sum_array should handle large numbers")
    
    fr fr Test with very small numbers
    sus small_array []drip = [0.001, 0.002, 0.003]
    sus small_sum drip = arrayz.sum_array(small_array)
    testz.assert_gt_float(small_sum, 0.005, "sum_array should handle small decimal numbers")
    
    fr fr Test with identical elements
    sus identical_array []drip = [5.0, 5.0, 5.0, 5.0]
    sus identical_max drip = arrayz.find_max(identical_array)
    sus identical_min drip = arrayz.find_min(identical_array)
    testz.assert_eq_float(identical_max, 5.0, "find_max should handle identical elements")
    testz.assert_eq_float(identical_min, 5.0, "find_min should handle identical elements")
    
    sus identical_avg drip = arrayz.average_array(identical_array)
    testz.assert_eq_float(identical_avg, 5.0, "average_array should handle identical elements")
    
    fr fr ===== ARRAY BUILDING TESTS =====
    testz.test_group("Dynamic Array Building")
    
    fr fr Test build_array_from_function if available
    ready arrayz.build_array_from_function {
        sus built_array []drip = arrayz.build_array_from_function(3.0, "double", [1.0, 2.0, 3.0])
        testz.assert_gt_float(len(built_array), 0.0, "build_array_from_function should create non-empty array")
    }
    
    fr fr ===== PERFORMANCE TESTS =====
    testz.test_group("Performance Validation")
    
    fr fr Test with moderately large array (stress test)
    sus performance_array []drip = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0,
        11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0
    ]
    
    sus perf_sum drip = arrayz.sum_array(performance_array)
    testz.assert_eq_float(perf_sum, 210.0, "Performance test: sum of 1-20 should be 210")
    
    sus perf_avg drip = arrayz.average_array(performance_array)
    testz.assert_eq_float(perf_avg, 10.5, "Performance test: average of 1-20 should be 10.5")
    
    sus perf_max drip = arrayz.find_max(performance_array)
    testz.assert_eq_float(perf_max, 20.0, "Performance test: max of 1-20 should be 20")
    
    sus perf_min drip = arrayz.find_min(performance_array)
    testz.assert_eq_float(perf_min, 1.0, "Performance test: min of 1-20 should be 1")
    
    fr fr ===== CONSISTENCY TESTS =====
    testz.test_group("Consistency Validation")
    
    fr fr Test that sum and average are consistent
    sus consistency_array []drip = [10.0, 20.0, 30.0, 40.0]
    sus cons_sum drip = arrayz.sum_array(consistency_array)
    sus cons_avg drip = arrayz.average_array(consistency_array)
    sus calculated_avg drip = cons_sum / 4.0
    testz.assert_eq_float(cons_avg, calculated_avg, "Average should equal sum divided by count")
    
    fr fr Test that max and min are in bounds
    sus bounds_array []drip = [15.0, 25.0, 35.0, 45.0]
    sus bounds_sum drip = arrayz.sum_array(bounds_array)
    sus bounds_max drip = arrayz.find_max(bounds_array)
    sus bounds_min drip = arrayz.find_min(bounds_array)
    
    testz.assert_le_float(bounds_max * 4.0, bounds_sum + bounds_max * 3.0, "Max should not exceed reasonable bound")
    testz.assert_ge_float(bounds_min * 4.0, bounds_sum - bounds_min * 3.0, "Min should not be below reasonable bound")
    
    fr fr ===== MATHEMATICAL PROPERTY TESTS =====
    testz.test_group("Mathematical Properties")
    
    fr fr Test sum properties
    sus prop_array1 []drip = [1.0, 2.0, 3.0]
    sus prop_array2 []drip = [4.0, 5.0]
    sus sum1 drip = arrayz.sum_array(prop_array1)
    sus sum2 drip = arrayz.sum_array(prop_array2)
    sus total_sum drip = sum1 + sum2
    
    sus combined_array []drip = [1.0, 2.0, 3.0, 4.0, 5.0]
    sus combined_sum drip = arrayz.sum_array(combined_array)
    testz.assert_eq_float(total_sum, combined_sum, "Sum should be associative: sum(A) + sum(B) = sum(A+B)")
    
    fr fr Test max/min properties
    sus max_prop1 drip = arrayz.find_max(prop_array1)
    sus max_prop2 drip = arrayz.find_max(prop_array2)
    sus combined_max drip = arrayz.find_max(combined_array)
    
    ready max_prop1 > max_prop2 {
        testz.assert_eq_float(combined_max, max_prop1, "Combined max should be larger individual max")
    } otherwise {
        testz.assert_eq_float(combined_max, max_prop2, "Combined max should be larger individual max")
    }
    
    fr fr ===== SEARCH ACCURACY TESTS =====
    testz.test_group("Search Accuracy Tests")
    
    fr fr Test multiple occurrences of same value
    sus duplicate_array []drip = [1.0, 3.0, 2.0, 3.0, 5.0]
    sus dup_contains lit = arrayz.contains_value(duplicate_array, 3.0)
    testz.assert_true(dup_contains, "contains_value should find duplicated value")
    
    sus dup_index drip = arrayz.find_index(duplicate_array, 3.0)
    testz.assert_eq_float(dup_index, 1.0, "find_index should return first occurrence")
    
    fr fr Test boundary values
    sus boundary_max drip = arrayz.find_max(duplicate_array)
    sus boundary_min drip = arrayz.find_min(duplicate_array)
    testz.assert_eq_float(boundary_max, 5.0, "find_max should find correct maximum")
    testz.assert_eq_float(boundary_min, 1.0, "find_min should find correct minimum")
    
    fr fr ===== TYPE SAFETY TESTS =====
    testz.test_group("Type Safety and Error Handling")
    
    fr fr Test functions return appropriate defaults for edge cases
    sus zero_array []drip = [0.0, 0.0, 0.0]
    sus zero_sum drip = arrayz.sum_array(zero_array)
    sus zero_avg drip = arrayz.average_array(zero_array)
    sus zero_product drip = arrayz.product_array(zero_array)
    
    testz.assert_eq_float(zero_sum, 0.0, "sum of zeros should be zero")
    testz.assert_eq_float(zero_avg, 0.0, "average of zeros should be zero")
    testz.assert_eq_float(zero_product, 0.0, "product including zero should be zero")
    
    fr fr Test search functions with non-existent values
    sus search_array []drip = [10.0, 20.0, 30.0]
    sus search_contains lit = arrayz.contains_value(search_array, 15.0)
    sus search_index drip = arrayz.find_index(search_array, 15.0)
    
    testz.assert_false(search_contains, "contains_value should return false for non-existent value")
    testz.assert_eq_float(search_index, -1.0, "find_index should return -1 for non-existent value")
    
    fr fr ===== INTEGRATION TESTS =====
    testz.test_group("Integration Tests")
    
    fr fr Test using multiple functions together
    sus integration_array []drip = [8.0, 3.0, 15.0, 1.0, 9.0, 7.0]
    
    sus int_sum drip = arrayz.sum_array(integration_array)
    sus int_avg drip = arrayz.average_array(integration_array)
    sus int_max drip = arrayz.find_max(integration_array)
    sus int_min drip = arrayz.find_min(integration_array)
    
    fr fr Validate relationships between results
    testz.assert_eq_float(int_avg, int_sum / 6.0, "Integration: average should equal sum/count")
    testz.assert_ge_float(int_max, int_avg, "Integration: max should be >= average")
    testz.assert_le_float(int_min, int_avg, "Integration: min should be <= average")
    
    fr fr Test search functions with calculated values
    sus max_found lit = arrayz.contains_value(integration_array, int_max)
    sus min_found lit = arrayz.contains_value(integration_array, int_min)
    testz.assert_true(max_found, "Integration: array should contain its maximum value")
    testz.assert_true(min_found, "Integration: array should contain its minimum value")
    
    sus max_index drip = arrayz.find_index(integration_array, int_max)
    sus min_index drip = arrayz.find_index(integration_array, int_min)
    testz.assert_ge_float(max_index, 0.0, "Integration: max value should have valid index")
    testz.assert_ge_float(min_index, 0.0, "Integration: min value should have valid index")
    
    fr fr ===== FINAL COMPREHENSIVE VALIDATION =====
    testz.test_group("Final Comprehensive Validation")
    
    fr fr Test all functions work with same dataset
    sus final_test_array []drip = [4.0, 2.0, 8.0, 6.0, 1.0, 3.0, 7.0, 5.0]
    
    sus final_sum drip = arrayz.sum_array(final_test_array)
    sus final_avg drip = arrayz.average_array(final_test_array)
    sus final_product drip = arrayz.product_array(final_test_array)
    sus final_max drip = arrayz.find_max(final_test_array)
    sus final_min drip = arrayz.find_min(final_test_array)
    
    fr fr Validate expected results
    testz.assert_eq_float(final_sum, 36.0, "Final: sum of test array should be 36")
    testz.assert_eq_float(final_avg, 4.5, "Final: average of test array should be 4.5")
    testz.assert_eq_float(final_max, 8.0, "Final: max of test array should be 8")
    testz.assert_eq_float(final_min, 1.0, "Final: min of test array should be 1")
    
    fr fr Test comprehensive search
    bestie i := 1.0; i <= 8.0; i += 1.0 {
        sus found lit = arrayz.contains_value(final_test_array, i)
        testz.assert_true(found, "Final: array should contain all values 1-8")
    }
    
    fr fr Test search for non-existent values
    sus not_found1 lit = arrayz.contains_value(final_test_array, 0.0)
    sus not_found2 lit = arrayz.contains_value(final_test_array, 9.0)
    testz.assert_false(not_found1, "Final: array should not contain 0")
    testz.assert_false(not_found2, "Final: array should not contain 9")
    
    fr fr Validate all core functionality works together
    sus validation_passed lit = based
    ready final_sum <= 0.0 { validation_passed = cap }
    ready final_avg <= 0.0 { validation_passed = cap }
    ready final_max < final_min { validation_passed = cap }
    ready !arrayz.contains_value(final_test_array, final_max) { validation_passed = cap }
    ready !arrayz.contains_value(final_test_array, final_min) { validation_passed = cap }
    
    testz.assert_true(validation_passed, "Final: all array operations should work correctly together")
    
    testz.print_test_summary()
}
