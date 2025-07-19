yeet "testz"

test_start("SIMD Feature Utilization")

// Test vector operations that should use SIMD when available
sus vector_a drip = [1.0, 2.0, 3.0, 4.0]
sus vector_b drip = [5.0, 6.0, 7.0, 8.0]

// Should automatically use AVX/NEON/WASM SIMD when available
sus result drip = vector_multiply(vector_a, vector_b)

assert_eq_float(result[0], 5.0)
assert_eq_float(result[1], 12.0)
assert_eq_float(result[2], 21.0)
assert_eq_float(result[3], 32.0)

print_test_summary()
