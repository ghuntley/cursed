yeet "testz"
yeet "mathz_optimized"

test_start("MATHZ_OPTIMIZED High-Performance Mathematical Tests")

// Test SIMD-optimized basic operations
sus vec1 drip[value] = [1.0, 2.0, 3.0, 4.0]
sus vec2 drip[value] = [2.0, 3.0, 4.0, 5.0]
sus vec_sum drip[value] = simd_vector_add(vec1, vec2)
assert_eq_float(vec_sum[0], 3.0)
assert_eq_float(vec_sum[3], 9.0)

// Test vectorized mathematical functions
sus angles drip[value] = [0.0, 0.5, 1.0, 1.57]
sus sines drip[value] = vectorized_sin(angles)
assert_eq_float_approx(sines[0], 0.0, 0.001)
assert_eq_float_approx(sines[3], 1.0, 0.001)

sus cosines drip[value] = vectorized_cos(angles)
assert_eq_float_approx(cosines[0], 1.0, 0.001)

// Test fast polynomial evaluation
sus coefficients drip[value] = [1.0, 2.0, 3.0] // 3x² + 2x + 1
sus poly_result drip = fast_polynomial_eval(coefficients, 2.0)
assert_eq_float(poly_result, 17.0) // 3*4 + 2*2 + 1 = 17

// Test optimized matrix operations
sus matrix1 drip[value][value] = [[1.0, 2.0], [3.0, 4.0]]
sus matrix2 drip[value][value] = [[2.0, 0.0], [1.0, 2.0]]
sus matrix_product drip[value][value] = optimized_matrix_multiply(matrix1, matrix2)
assert_eq_float(matrix_product[0][0], 4.0)
assert_eq_float(matrix_product[1][1], 14.0)

// Test fast Fourier transform
sus signal drip[value] = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]
sus fft_result drip[value] = fast_fourier_transform(signal)
assert_eq_int(len(fft_result), 8)

// Test statistical functions with SIMD
sus data drip[value] = range_float(1.0, 1000.0, 1.0)
sus mean drip = vectorized_mean(data)
assert_eq_float_approx(mean, 500.5, 0.1)

sus variance drip = vectorized_variance(data)
assert_true(variance > 80000.0)

// Test optimized sorting algorithms
sus unsorted drip[value] = generate_random_array(10000)
sus sorted drip[value] = parallel_quicksort(unsorted)
assert_true(is_sorted_ascending(sorted))

// Test cache-optimized mathematical operations
sus large_array drip[value] = range_float(1.0, 100000.0, 1.0)
sus sqrt_results drip[value] = cache_optimized_sqrt(large_array)
assert_eq_float_approx(sqrt_results[0], 1.0, 0.001)
assert_eq_float_approx(sqrt_results[99999], 316.227, 0.001)

// Test numerical integration with adaptive precision
slay test_function(x drip) drip {
    damn x * x // f(x) = x²
}
sus integral drip = adaptive_integrate(test_function, 0.0, 10.0, 0.0001)
assert_eq_float_approx(integral, 333.333, 0.1) // ∫₀¹⁰ x² dx = 1000/3

// Test optimized prime number generation
sus primes drip[value] = optimized_sieve_of_eratosthenes(1000)
assert_eq_int(primes[0], 2)
assert_eq_int(primes[1], 3)
assert_eq_int(primes[24], 97) // 25th prime

// Test parallel mathematical reduction
sus reduction_array drip[value] = range_float(1.0, 10000.0, 1.0)
sus parallel_sum drip = parallel_reduce_sum_float(reduction_array)
assert_eq_float(parallel_sum, 50005000.0)

// Test optimized transcendental functions
sus exp_results drip[value] = vectorized_exp([0.0, 1.0, 2.0])
assert_eq_float_approx(exp_results[0], 1.0, 0.001)
assert_eq_float_approx(exp_results[1], 2.718, 0.001)

// Test performance benchmarking
sus perf_data drip[value] = generate_random_array(1000000)
sus benchmark_start drip = get_nanoseconds()
sus sum_result drip = vectorized_sum_float(perf_data)
sus benchmark_end drip = get_nanoseconds()
sus benchmark_duration drip = benchmark_end - benchmark_start
assert_true(benchmark_duration < 100000000) // Less than 100ms for 1M elements
assert_true(sum_result > 0.0)

// Test memory-aligned operations
sus aligned_data drip[value] = create_aligned_float_array(1024, 64)
sus alignment_valid lit = verify_float_alignment(aligned_data, 64)
assert_true(alignment_valid)

print_test_summary()
