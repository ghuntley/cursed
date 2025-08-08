yeet "testz"
yeet "tensorz"
yeet "mathz"

fr fr Simplified Tensor Operations Test Suite

test_start("tensor_creation_1d")

fr fr Test 1D tensor creation
sus tensor1 []meal = tensor_zeros_1d(5)
assert_eq_int(len(tensor1), 5)
assert_near(tensor_sum_1d(tensor1), 0.0, 0.001)

fr fr Test ones tensor
sus ones []meal = tensor_ones_1d(3)
assert_near(tensor_sum_1d(ones), 3.0, 0.001)

fr fr Test tensor from array
sus data []meal = [1.0, 2.0, 3.0, 4.0]
sus tensor2 []meal = tensor_from_array_1d(data)
assert_eq_int(len(tensor2), 4)
assert_near(tensor2[0], 1.0, 0.001)
assert_near(tensor2[3], 4.0, 0.001)

test_start("tensor_arithmetic_1d")

fr fr Test tensor addition
sus a []meal = [1.0, 2.0, 3.0]
sus b []meal = [4.0, 5.0, 6.0]
sus sum_tensor []meal = tensor_add_1d(a, b)
assert_near(sum_tensor[0], 5.0, 0.001)  fr fr 1 + 4
assert_near(sum_tensor[2], 9.0, 0.001)  fr fr 3 + 6

fr fr Test tensor subtraction
sus diff_tensor []meal = tensor_subtract_1d(b, a)
assert_near(diff_tensor[0], 3.0, 0.001)  fr fr 4 - 1
assert_near(diff_tensor[2], 3.0, 0.001)  fr fr 6 - 3

fr fr Test scalar operations
sus scaled []meal = tensor_multiply_scalar_1d(a, 2.0)
assert_near(scaled[0], 2.0, 0.001)  fr fr 1 * 2
assert_near(scaled[2], 6.0, 0.001)  fr fr 3 * 2

sus shifted []meal = tensor_add_scalar_1d(a, 10.0)
assert_near(shifted[0], 11.0, 0.001)  fr fr 1 + 10

test_start("matrix_operations")

fr fr Test matrix creation and operations
sus matrix []meal = matrix_zeros(2, 3)
assert_eq_int(len(matrix), 6)  fr fr 2 * 3

fr fr Test identity matrix
sus identity []meal = matrix_eye(3)
assert_near(matrix_get(identity, 3, 3, 0, 0), 1.0, 0.001)
assert_near(matrix_get(identity, 3, 3, 1, 1), 1.0, 0.001)
assert_near(matrix_get(identity, 3, 3, 0, 1), 0.0, 0.001)

fr fr Test matrix multiplication
sus mat_a []meal = [1.0, 2.0, 3.0, 4.0]  fr fr 2x2 matrix
sus mat_b []meal = [5.0, 6.0, 7.0, 8.0]  fr fr 2x2 matrix
sus result []meal = matrix_multiply(mat_a, 2, 2, mat_b, 2, 2)

fr fr Expected result: [[19, 22], [43, 50]]
assert_near(matrix_get(result, 2, 2, 0, 0), 19.0, 0.001)  fr fr 1*5 + 2*7
assert_near(matrix_get(result, 2, 2, 0, 1), 22.0, 0.001)  fr fr 1*6 + 2*8
assert_near(matrix_get(result, 2, 2, 1, 0), 43.0, 0.001)  fr fr 3*5 + 4*7
assert_near(matrix_get(result, 2, 2, 1, 1), 50.0, 0.001)  fr fr 3*6 + 4*8

fr fr Test matrix transpose
sus transposed []meal = matrix_transpose(mat_a, 2, 2)
assert_near(matrix_get(transposed, 2, 2, 0, 0), 1.0, 0.001)  fr fr (0,0) -> (0,0)
assert_near(matrix_get(transposed, 2, 2, 0, 1), 3.0, 0.001)  fr fr (1,0) -> (0,1)
assert_near(matrix_get(transposed, 2, 2, 1, 0), 2.0, 0.001)  fr fr (0,1) -> (1,0)
assert_near(matrix_get(transposed, 2, 2, 1, 1), 4.0, 0.001)  fr fr (1,1) -> (1,1)

test_start("tensor_statistics_1d")

fr fr Test statistics functions
sus test_data []meal = [1.0, 2.0, 3.0, 4.0, 5.0]
sus mean_val meal = tensor_mean_1d(test_data)
assert_near(mean_val, 3.0, 0.001)

sus sum_val meal = tensor_sum_1d(test_data)
assert_near(sum_val, 15.0, 0.001)

sus min_val meal = tensor_min_1d(test_data)
assert_near(min_val, 1.0, 0.001)

sus max_val meal = tensor_max_1d(test_data)
assert_near(max_val, 5.0, 0.001)

sus var_val meal = tensor_variance_1d(test_data)
assert_near(var_val, 2.5, 0.1)  fr fr Sample variance

sus std_val meal = tensor_std_dev_1d(test_data)
assert_near(std_val, sqrt_meal(2.5), 0.1)

test_start("mathematical_operations_1d")

fr fr Test mathematical operations
sus pos_data []meal = [1.0, 4.0, 9.0, 16.0]

fr fr Test square root
sus sqrt_result []meal = tensor_sqrt_1d(pos_data)
assert_near(sqrt_result[0], 1.0, 0.001)  fr fr sqrt(1)
assert_near(sqrt_result[1], 2.0, 0.001)  fr fr sqrt(4)
assert_near(sqrt_result[2], 3.0, 0.001)  fr fr sqrt(9)
assert_near(sqrt_result[3], 4.0, 0.001)  fr fr sqrt(16)

fr fr Test exponential
sus exp_data []meal = [0.0, 1.0, 2.0]
sus exp_result []meal = tensor_exp_1d(exp_data)
assert_near(exp_result[0], 1.0, 0.001)     fr fr exp(0)
assert_near(exp_result[1], E, 0.01)        fr fr exp(1)

fr fr Test absolute value
sus neg_data []meal = [-2.0, -1.0, 0.0, 1.0, 2.0]
sus abs_result []meal = tensor_abs_1d(neg_data)
assert_near(abs_result[0], 2.0, 0.001)
assert_near(abs_result[1], 1.0, 0.001)
assert_near(abs_result[2], 0.0, 0.001)
assert_near(abs_result[3], 1.0, 0.001)
assert_near(abs_result[4], 2.0, 0.001)

test_start("normalization_1d")

fr fr Test z-score normalization
sus norm_data []meal = [1.0, 2.0, 3.0, 4.0, 5.0]
sus normalized []meal = tensor_normalize_1d(norm_data)
sus norm_mean meal = tensor_mean_1d(normalized)
sus norm_std meal = tensor_std_dev_1d(normalized)
assert_near(norm_mean, 0.0, 0.1)   fr fr Mean should be ~0
assert_near(norm_std, 1.0, 0.1)    fr fr Std dev should be ~1

fr fr Test min-max normalization
sus minmax_normalized []meal = tensor_min_max_normalize_1d(norm_data)
sus minmax_min meal = tensor_min_1d(minmax_normalized)
sus minmax_max meal = tensor_max_1d(minmax_normalized)
assert_near(minmax_min, 0.0, 0.001)  fr fr Min should be 0
assert_near(minmax_max, 1.0, 0.001)  fr fr Max should be 1

test_start("tensor_utilities_1d")

fr fr Test tensor equality
sus comp_a []meal = [1.0, 2.0, 3.0]
sus comp_b []meal = [1.0, 2.0, 3.0]
sus comp_c []meal = [1.1, 2.1, 3.1]
assert_true(tensor_equals_1d(comp_a, comp_b, 0.001))
assert_true(tensor_equals_1d(comp_a, comp_c, 0.2))
assert_false(tensor_equals_1d(comp_a, comp_c, 0.05))

fr fr Test tensor slicing
sus slice_data []meal = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
sus sliced []meal = tensor_slice_1d(slice_data, 2, 5)
assert_eq_int(len(sliced), 3)
assert_near(sliced[0], 3.0, 0.001)  fr fr slice_data[2]
assert_near(sliced[2], 5.0, 0.001)  fr fr slice_data[4]

fr fr Test tensor concatenation
sus cat_a []meal = [1.0, 2.0]
sus cat_b []meal = [3.0, 4.0]
sus concatenated []meal = tensor_concat_1d(cat_a, cat_b)
assert_eq_int(len(concatenated), 4)
assert_near(concatenated[0], 1.0, 0.001)
assert_near(concatenated[3], 4.0, 0.001)

test_start("dot_product_and_magnitude")

fr fr Test dot product
sus vec_a []meal = [1.0, 2.0, 3.0]
sus vec_b []meal = [4.0, 5.0, 6.0]
sus dot_result meal = tensor_dot_product_1d(vec_a, vec_b)
fr fr Expected: 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
assert_near(dot_result, 32.0, 0.001)

fr fr Test vector magnitude
sus magnitude meal = vector_magnitude_1d(vec_a)
fr fr Expected: sqrt(1² + 2² + 3²) = sqrt(14)
assert_near(magnitude, sqrt_meal(14.0), 0.001)

fr fr Test vector normalization
sus normalized_vec []meal = vector_normalize_1d(vec_a)
sus norm_magnitude meal = vector_magnitude_1d(normalized_vec)
assert_near(norm_magnitude, 1.0, 0.001)  fr fr Should be unit vector

test_start("activation_functions")

fr fr Test sigmoid activation
sus sigmoid_data []meal = [-2.0, -1.0, 0.0, 1.0, 2.0]
sus sigmoid_result []meal = apply_sigmoid_1d(sigmoid_data)
assert_near(sigmoid_result[2], 0.5, 0.001)  fr fr sigmoid(0) = 0.5
assert_true(sigmoid_result[0] > 0.0)        fr fr sigmoid(-2) > 0
assert_true(sigmoid_result[0] < 0.5)        fr fr sigmoid(-2) < 0.5
assert_true(sigmoid_result[4] > 0.5)        fr fr sigmoid(2) > 0.5
assert_true(sigmoid_result[4] < 1.0)        fr fr sigmoid(2) < 1.0

fr fr Test ReLU activation
sus relu_data []meal = [-2.0, -1.0, 0.0, 1.0, 2.0]
sus relu_result []meal = apply_relu_1d(relu_data)
assert_near(relu_result[0], 0.0, 0.001)  fr fr ReLU(-2) = 0
assert_near(relu_result[1], 0.0, 0.001)  fr fr ReLU(-1) = 0
assert_near(relu_result[2], 0.0, 0.001)  fr fr ReLU(0) = 0
assert_near(relu_result[3], 1.0, 0.001)  fr fr ReLU(1) = 1
assert_near(relu_result[4], 2.0, 0.001)  fr fr ReLU(2) = 2

fr fr Test tanh activation
sus tanh_data []meal = [0.0, 1.0, -1.0]
sus tanh_result []meal = apply_tanh_1d(tanh_data)
assert_near(tanh_result[0], 0.0, 0.001)   fr fr tanh(0) = 0

test_start("loss_functions")

fr fr Test mean squared error
sus pred_data []meal = [1.0, 2.0, 3.0]
sus target_data []meal = [1.5, 2.5, 2.5]
sus mse meal = mean_squared_error_1d(pred_data, target_data)
fr fr Expected: ((1-1.5)² + (2-2.5)² + (3-2.5)²) / 3 = (0.25 + 0.25 + 0.25) / 3 = 0.25
assert_near(mse, 0.25, 0.001)

fr fr Test mean absolute error
sus mae meal = mean_absolute_error_1d(pred_data, target_data)
fr fr Expected: (|1-1.5| + |2-2.5| + |3-2.5|) / 3 = (0.5 + 0.5 + 0.5) / 3 = 0.5
assert_near(mae, 0.5, 0.001)

test_start("random_tensors")

fr fr Test random tensor generation
set_random_seed(42)  fr fr Set seed for reproducible tests
sus random_tensor []meal = tensor_random_1d(10, 0.0, 1.0)
assert_eq_int(len(random_tensor), 10)

fr fr All values should be between 0 and 1
sus all_in_range lit = based
sus i normie = 0
bestie i < len(random_tensor) {
    lowkey random_tensor[i] < 0.0 || random_tensor[i] > 1.0 {
        all_in_range = cringe
        ghosted
    }
    i = i + 1
}
assert_true(all_in_range)

test_start("cosine_similarity")

fr fr Test cosine similarity
sus vec1 []meal = [1.0, 0.0, 0.0]
sus vec2 []meal = [0.0, 1.0, 0.0]
sus vec3 []meal = [1.0, 0.0, 0.0]

sus sim_orthogonal meal = cosine_similarity_1d(vec1, vec2)
assert_near(sim_orthogonal, 0.0, 0.001)  fr fr Orthogonal vectors

sus sim_identical meal = cosine_similarity_1d(vec1, vec3)
assert_near(sim_identical, 1.0, 0.001)   fr fr Identical vectors

test_start("edge_cases")

fr fr Test edge cases
fr fr Empty tensor
sus empty_tensor []meal = tensor_zeros_1d(0)
assert_eq_int(len(empty_tensor), 0)
assert_near(tensor_sum_1d(empty_tensor), 0.0, 0.001)

fr fr Single element tensor
sus single_tensor []meal = tensor_ones_1d(1)
assert_eq_int(len(single_tensor), 1)
assert_near(tensor_mean_1d(single_tensor), 1.0, 0.001)
assert_near(tensor_std_dev_1d(single_tensor), 0.0, 0.001)

fr fr Division by zero in tensor operations
sus zero_tensor []meal = tensor_zeros_1d(3)
sus ones_tensor []meal = tensor_ones_1d(3)
sus div_result []meal = tensor_divide_1d(ones_tensor, zero_tensor)
fr fr Should handle gracefully (return zeros)
assert_near(div_result[0], 0.0, 0.001)
assert_near(div_result[2], 0.0, 0.001)

print_test_summary()
