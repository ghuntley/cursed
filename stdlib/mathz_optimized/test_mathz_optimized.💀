yeet "testz"
yeet "mathz_optimized"

# Comprehensive test suite for mathz_optimized module
# Real functional tests for optimized mathematical operations

test_start("test_initialize_math_cache")
# Test math cache initialization
sus result lit = initialize_math_cache()
assert_true(result)

# Test re-initialization (should succeed)
sus result2 lit = initialize_math_cache()
assert_true(result2)
print_test_summary()

test_start("test_precompute_primes")
# Test precomputing prime numbers up to limit
sus result lit = precompute_primes(100)
assert_true(result)

# Test with larger limit
sus result2 lit = precompute_primes(1000)
assert_true(result2)

# Test with small limit
sus result3 lit = precompute_primes(10)
assert_true(result3)
print_test_summary()

test_start("test_array_sum_vectorized")
# Test vectorized array summation
sus test_array drip[value] = [1, 2, 3, 4, 5]
sus sum drip = array_sum_vectorized(test_array)
assert_eq_int(sum, 15)

# Test with larger array
sus large_array drip[value] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
sus large_sum drip = array_sum_vectorized(large_array)
assert_eq_int(large_sum, 550)

# Test with empty array
sus empty_array drip[value] = []
sus empty_sum drip = array_sum_vectorized(empty_array)
assert_eq_int(empty_sum, 0)

# Test with negative numbers
sus mixed_array drip[value] = [-5, -3, 0, 3, 5]
sus mixed_sum drip = array_sum_vectorized(mixed_array)
assert_eq_int(mixed_sum, 0)
print_test_summary()

test_start("test_power_fast")
# Test fast exponentiation
sus result drip = power_fast(2, 10)
assert_eq_int(result, 1024)

# Test with base 3
sus result2 drip = power_fast(3, 4)
assert_eq_int(result2, 81)

# Test with exponent 0
sus result3 drip = power_fast(5, 0)
assert_eq_int(result3, 1)

# Test with exponent 1
sus result4 drip = power_fast(7, 1)
assert_eq_int(result4, 7)

# Test with negative base
sus result5 drip = power_fast(-2, 3)
assert_eq_int(result5, -8)
print_test_summary()

test_start("test_gcd_optimized")
# Test optimized greatest common divisor
sus gcd1 drip = gcd_optimized(48, 18)
assert_eq_int(gcd1, 6)

sus gcd2 drip = gcd_optimized(100, 25)
assert_eq_int(gcd2, 25)

sus gcd3 drip = gcd_optimized(17, 13)
assert_eq_int(gcd3, 1)

# Test with same numbers
sus gcd4 drip = gcd_optimized(42, 42)
assert_eq_int(gcd4, 42)

# Test with one zero
sus gcd5 drip = gcd_optimized(15, 0)
assert_eq_int(gcd5, 15)
print_test_summary()

test_start("test_factorial_optimized")
# Test optimized factorial calculation
sus fact0 drip = factorial_optimized(0)
assert_eq_int(fact0, 1)

sus fact5 drip = factorial_optimized(5)
assert_eq_int(fact5, 120)

sus fact10 drip = factorial_optimized(10)
assert_eq_int(fact10, 3628800)

# Test with 1
sus fact1 drip = factorial_optimized(1)
assert_eq_int(fact1, 1)
print_test_summary()

test_start("test_fibonacci_optimized")
# Test optimized Fibonacci calculation
sus fib0 drip = fibonacci_optimized(0)
assert_eq_int(fib0, 0)

sus fib1 drip = fibonacci_optimized(1)
assert_eq_int(fib1, 1)

sus fib10 drip = fibonacci_optimized(10)
assert_eq_int(fib10, 55)

sus fib15 drip = fibonacci_optimized(15)
assert_eq_int(fib15, 610)

# Test larger Fibonacci number
sus fib20 drip = fibonacci_optimized(20)
assert_eq_int(fib20, 6765)
print_test_summary()

test_start("test_fibonacci_matrix_power")
# Test matrix-based Fibonacci calculation
sus matrix_fib10 drip = fibonacci_matrix_power(10)
assert_eq_int(matrix_fib10, 55)

sus matrix_fib15 drip = fibonacci_matrix_power(15)
assert_eq_int(matrix_fib15, 610)

# Test small values
sus matrix_fib2 drip = fibonacci_matrix_power(2)
assert_eq_int(matrix_fib2, 1)
print_test_summary()

test_start("test_matrix_multiply_2x2")
# Test 2x2 matrix multiplication
sus matrix_a drip[value][value] = [[1, 2], [3, 4]]
sus matrix_b drip[value][value] = [[5, 6], [7, 8]]
sus result drip[value][value] = matrix_multiply_2x2(matrix_a, matrix_b)

# Result should be [[19, 22], [43, 50]]
assert_eq_int(result[0][0], 19)
assert_eq_int(result[0][1], 22)
assert_eq_int(result[1][0], 43)
assert_eq_int(result[1][1], 50)

# Test with identity matrix
sus identity drip[value][value] = [[1, 0], [0, 1]]
sus identity_result drip[value][value] = matrix_multiply_2x2(matrix_a, identity)
assert_eq_int(identity_result[0][0], 1)
assert_eq_int(identity_result[0][1], 2)
assert_eq_int(identity_result[1][0], 3)
assert_eq_int(identity_result[1][1], 4)
print_test_summary()

test_start("test_matrix_power")
# Test matrix exponentiation
sus base_matrix drip[value][value] = [[1, 1], [1, 0]]  # Fibonacci matrix
sus power2 drip[value][value] = matrix_power(base_matrix, 2)
sus power5 drip[value][value] = matrix_power(base_matrix, 5)

# Test power 0 (should return identity)
sus power0 drip[value][value] = matrix_power(base_matrix, 0)
assert_eq_int(power0[0][0], 1)
assert_eq_int(power0[0][1], 0)
assert_eq_int(power0[1][0], 0)
assert_eq_int(power0[1][1], 1)

# Test power 1
sus power1 drip[value][value] = matrix_power(base_matrix, 1)
assert_eq_int(power1[0][0], 1)
assert_eq_int(power1[0][1], 1)
assert_eq_int(power1[1][0], 1)
assert_eq_int(power1[1][1], 0)
print_test_summary()

test_start("test_is_prime_optimized")
# Test optimized primality testing
assert_true(!is_prime_optimized(1))   # 1 is not prime
assert_true(is_prime_optimized(2))    # 2 is prime
assert_true(is_prime_optimized(3))    # 3 is prime
assert_true(!is_prime_optimized(4))   # 4 is not prime
assert_true(is_prime_optimized(5))    # 5 is prime
assert_true(!is_prime_optimized(6))   # 6 is not prime
assert_true(is_prime_optimized(7))    # 7 is prime
assert_true(!is_prime_optimized(8))   # 8 is not prime
assert_true(!is_prime_optimized(9))   # 9 is not prime
assert_true(!is_prime_optimized(10))  # 10 is not prime

# Test larger primes
assert_true(is_prime_optimized(97))
assert_true(is_prime_optimized(101))
assert_true(!is_prime_optimized(100))
assert_true(!is_prime_optimized(121))  # 11^2
print_test_summary()

test_start("test_sqrt_fast")
# Test fast square root calculation
sus sqrt4 drip = sqrt_fast(4)
assert_eq_int(sqrt4, 2)

sus sqrt9 drip = sqrt_fast(9)
assert_eq_int(sqrt9, 3)

sus sqrt16 drip = sqrt_fast(16)
assert_eq_int(sqrt16, 4)

sus sqrt25 drip = sqrt_fast(25)
assert_eq_int(sqrt25, 5)

# Test with non-perfect squares (approximate)
sus sqrt10 drip = sqrt_fast(10)
assert_true(sqrt10 >= 3 && sqrt10 <= 4)

sus sqrt2 drip = sqrt_fast(2)
assert_true(sqrt2 >= 1 && sqrt2 <= 2)
print_test_summary()

test_start("test_array_multiply_scalar")
# Test scalar multiplication of arrays
sus test_array drip[value] = [1, 2, 3, 4, 5]
sus result drip[value] = array_multiply_scalar(test_array, 3)

assert_eq_int(result[0], 3)
assert_eq_int(result[1], 6)
assert_eq_int(result[2], 9)
assert_eq_int(result[3], 12)
assert_eq_int(result[4], 15)

# Test with zero scalar
sus zero_result drip[value] = array_multiply_scalar(test_array, 0)
assert_eq_int(zero_result[0], 0)
assert_eq_int(zero_result[4], 0)

# Test with negative scalar
sus negative_result drip[value] = array_multiply_scalar(test_array, -2)
assert_eq_int(negative_result[0], -2)
assert_eq_int(negative_result[1], -4)
print_test_summary()

test_start("test_array_dot_product")
# Test dot product of two arrays
sus array_a drip[value] = [1, 2, 3]
sus array_b drip[value] = [4, 5, 6]
sus dot_product drip = array_dot_product(array_a, array_b)
assert_eq_int(dot_product, 32)  # 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32

# Test with orthogonal vectors
sus ortho_a drip[value] = [1, 0]
sus ortho_b drip[value] = [0, 1]
sus ortho_dot drip = array_dot_product(ortho_a, ortho_b)
assert_eq_int(ortho_dot, 0)

# Test with identical arrays
sus same_array drip[value] = [2, 3, 4]
sus same_dot drip = array_dot_product(same_array, same_array)
assert_eq_int(same_dot, 29)  # 2*2 + 3*3 + 4*4 = 4 + 9 + 16 = 29
print_test_summary()

test_start("test_array_mean_variance")
# Test calculating mean and variance of array
sus test_data drip[value] = [1, 2, 3, 4, 5]
sus stats MeanVarianceResult = array_mean_variance(test_data)
assert_eq_int(stats.mean, 3)    # (1+2+3+4+5)/5 = 3
assert_true(stats.variance >= 2 && stats.variance <= 3)  # Approximate variance

# Test with identical values
sus uniform_data drip[value] = [5, 5, 5, 5, 5]
sus uniform_stats MeanVarianceResult = array_mean_variance(uniform_data)
assert_eq_int(uniform_stats.mean, 5)
assert_eq_int(uniform_stats.variance, 0)

# Test with two values
sus simple_data drip[value] = [10, 20]
sus simple_stats MeanVarianceResult = array_mean_variance(simple_data)
assert_eq_int(simple_stats.mean, 15)
assert_true(simple_stats.variance > 0)
print_test_summary()

# Performance tests
test_start("performance_mathematical_operations")
# Initialize cache first
initialize_math_cache()
precompute_primes(1000)

# Test rapid operations
bestie i := 1; i <= 20; i++ {
    sus power_result drip = power_fast(2, i)
    assert_true(power_result > 0)
    
    sus fib_result drip = fibonacci_optimized(i)
    assert_true(fib_result >= 0)
    
    sus is_prime lit = is_prime_optimized(i)
    # Just check it doesn't crash
}

# Test large array operations
sus large_array drip[value] = []
bestie i := 1; i <= 1000; i++ {
    large_array = append(large_array, i)
}
sus large_sum drip = array_sum_vectorized(large_array)
assert_eq_int(large_sum, 500500)  # Sum of 1 to 1000
print_test_summary()

# Edge cases
test_start("edge_cases_mathz_optimized")
# Test very large numbers
sus large_power drip = power_fast(2, 30)
assert_true(large_power > 1000000000)

# Test edge cases for GCD
sus gcd_zero drip = gcd_optimized(0, 5)
assert_eq_int(gcd_zero, 5)

# Test prime checking on edge cases
assert_true(!is_prime_optimized(0))
assert_true(!is_prime_optimized(-5))

# Test factorial of larger number
sus large_factorial drip = factorial_optimized(12)
assert_true(large_factorial > 400000000)

# Test empty array operations
sus empty_array drip[value] = []
sus empty_sum drip = array_sum_vectorized(empty_array)
assert_eq_int(empty_sum, 0)

sus empty_scalar drip[value] = array_multiply_scalar(empty_array, 5)
assert_eq_int(len(empty_scalar), 0)
print_test_summary()

# Integration test
test_start("integration_mathematical_workflow")
# Complete mathematical computation workflow
initialize_math_cache()
precompute_primes(100)

# Generate test data
sus data drip[value] = []
bestie i := 1; i <= 20; i++ {
    sus fib drip = fibonacci_optimized(i)
    data = append(data, fib)
}

# Analyze the data
sus sum drip = array_sum_vectorized(data)
sus scaled drip[value] = array_multiply_scalar(data, 2)
sus stats MeanVarianceResult = array_mean_variance(data)

assert_true(sum > 0)
assert_true(len(scaled) == len(data))
assert_true(stats.mean > 0)

# Verify mathematical relationships
sus first_ten drip[value] = data[0:10]
sus last_ten drip[value] = data[10:20]
sus dot drip = array_dot_product(first_ten, last_ten)
assert_true(dot > 0)

vibez.spill("Mathematical integration test completed successfully")
print_test_summary()
