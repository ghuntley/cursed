fr fr CURSED Tensor Operations Module - Pure CURSED Implementation
fr fr Simplified version compatible with current parser

yeet "mathz"
yeet "arrayz"

fr fr === TENSOR CREATION FUNCTIONS ===

slay tensor_new_1d(size normie) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < size {
        result = append(result, 0.0)
        i = i + 1
    }
    damn result
}

slay tensor_zeros_1d(size normie) meal[value]{
    damn tensor_new_1d(size)
}

slay tensor_ones_1d(size normie) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < size {
        result = append(result, 1.0)
        i = i + 1
    }
    damn result
}

slay tensor_from_array_1d(data meal[value]) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(data) {
        result = append(result, data[i])
        i = i + 1
    }
    damn result
}

slay tensor_random_1d(size normie, min_val meal, max_val meal) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < size {
        sus random_val meal = random_meal_range(min_val, max_val)
        result = append(result, random_val)
        i = i + 1
    }
    damn result
}

slay tensor_random_normal_1d(size normie, mean meal, std_dev meal) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < size {
        sus normal_val meal = mean + std_dev * random_gaussian()
        result = append(result, normal_val)
        i = i + 1
    }
    damn result
}

fr fr === MATRIX OPERATIONS (2D arrays as flattened 1D) ===

slay matrix_new(rows normie, cols normie) meal[value]{
    sus size normie = rows * cols
    damn tensor_new_1d(size)
}

slay matrix_zeros(rows normie, cols normie) meal[value]{
    damn matrix_new(rows, cols)
}

slay matrix_ones(rows normie, cols normie) meal[value]{
    sus size normie = rows * cols
    damn tensor_ones_1d(size)
}

slay matrix_eye(n normie) meal[value]{
    sus result meal[value] = matrix_zeros(n, n)
    sus i normie = 0
    bestie i < n {
        sus index normie = i * n + i
        result[index] = 1.0
        i = i + 1
    }
    damn result
}

slay matrix_get(matrix meal[value], rows normie, cols normie, row normie, col normie) meal {
    lowkey row < 0 || row >= rows || col < 0 || col >= cols {
        damn 0.0
    }
    sus index normie = row * cols + col
    damn matrix[index]
}

slay matrix_set(matrix meal[value], rows normie, cols normie, row normie, col normie, value meal) meal[value]{
    lowkey row < 0 || row >= rows || col < 0 || col >= cols {
        damn matrix
    }
    sus index normie = row * cols + col
    matrix[index] = value
    damn matrix
}

fr fr === TENSOR ARITHMETIC OPERATIONS ===

slay tensor_add_1d(a meal[value], b meal[value]) meal[value]{
    lowkey len(a) != len(b) {
        damn a
    }
    
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(a) {
        result = append(result, a[i] + b[i])
        i = i + 1
    }
    damn result
}

slay tensor_subtract_1d(a meal[value], b meal[value]) meal[value]{
    lowkey len(a) != len(b) {
        damn a
    }
    
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(a) {
        result = append(result, a[i] - b[i])
        i = i + 1
    }
    damn result
}

slay tensor_multiply_1d(a meal[value], b meal[value]) meal[value]{
    lowkey len(a) != len(b) {
        damn a
    }
    
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(a) {
        result = append(result, a[i] * b[i])
        i = i + 1
    }
    damn result
}

slay tensor_divide_1d(a meal[value], b meal[value]) meal[value]{
    lowkey len(a) != len(b) {
        damn a
    }
    
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(a) {
        lowkey abs_meal(b[i]) < EPSILON {
            result = append(result, 0.0)
        } {
            result = append(result, a[i] / b[i])
        }
        i = i + 1
    }
    damn result
}

slay tensor_add_scalar_1d(tensor meal[value], scalar meal) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        result = append(result, tensor[i] + scalar)
        i = i + 1
    }
    damn result
}

slay tensor_multiply_scalar_1d(tensor meal[value], scalar meal) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        result = append(result, tensor[i] * scalar)
        i = i + 1
    }
    damn result
}

fr fr === MATRIX MULTIPLICATION ===

slay matrix_multiply(a meal[value], a_rows normie, a_cols normie, b meal[value], b_rows normie, b_cols normie) meal[value]{
    lowkey a_cols != b_rows {
        damn a
    }
    
    sus result meal[value] = matrix_zeros(a_rows, b_cols)
    
    sus i normie = 0
    bestie i < a_rows {
        sus j normie = 0
        bestie j < b_cols {
            sus sum meal = 0.0
            sus k normie = 0
            bestie k < a_cols {
                sus a_val meal = matrix_get(a, a_rows, a_cols, i, k)
                sus b_val meal = matrix_get(b, b_rows, b_cols, k, j)
                sum = sum + a_val * b_val
                k = k + 1
            }
            result = matrix_set(result, a_rows, b_cols, i, j, sum)
            j = j + 1
        }
        i = i + 1
    }
    
    damn result
}

slay matrix_transpose(matrix meal[value], rows normie, cols normie) meal[value]{
    sus result meal[value] = matrix_zeros(cols, rows)
    
    sus i normie = 0
    bestie i < rows {
        sus j normie = 0
        bestie j < cols {
            sus value meal = matrix_get(matrix, rows, cols, i, j)
            result = matrix_set(result, cols, rows, j, i, value)
            j = j + 1
        }
        i = i + 1
    }
    
    damn result
}

slay tensor_dot_product_1d(a meal[value], b meal[value]) meal {
    lowkey len(a) != len(b) {
        damn 0.0
    }
    
    sus result meal = 0.0
    sus i normie = 0
    bestie i < len(a) {
        result = result + a[i] * b[i]
        i = i + 1
    }
    damn result
}

fr fr === TENSOR STATISTICS ===

slay tensor_sum_1d(tensor meal[value]) meal {
    sus sum meal = 0.0
    sus i normie = 0
    bestie i < len(tensor) {
        sum = sum + tensor[i]
        i = i + 1
    }
    damn sum
}

slay tensor_mean_1d(tensor meal[value]) meal {
    lowkey len(tensor) == 0 {
        damn 0.0
    }
    damn tensor_sum_1d(tensor) / len(tensor)
}

slay tensor_min_1d(tensor meal[value]) meal {
    lowkey len(tensor) == 0 {
        damn 0.0
    }
    
    sus min_val meal = tensor[0]
    sus i normie = 1
    bestie i < len(tensor) {
        lowkey tensor[i] < min_val {
            min_val = tensor[i]
        }
        i = i + 1
    }
    damn min_val
}

slay tensor_max_1d(tensor meal[value]) meal {
    lowkey len(tensor) == 0 {
        damn 0.0
    }
    
    sus max_val meal = tensor[0]
    sus i normie = 1
    bestie i < len(tensor) {
        lowkey tensor[i] > max_val {
            max_val = tensor[i]
        }
        i = i + 1
    }
    damn max_val
}

slay tensor_variance_1d(tensor meal[value]) meal {
    lowkey len(tensor) <= 1 {
        damn 0.0
    }
    
    sus mean meal = tensor_mean_1d(tensor)
    sus sum_sq_diff meal = 0.0
    sus i normie = 0
    bestie i < len(tensor) {
        sus diff meal = tensor[i] - mean
        sum_sq_diff = sum_sq_diff + diff * diff
        i = i + 1
    }
    damn sum_sq_diff / (len(tensor) - 1)
}

slay tensor_std_dev_1d(tensor meal[value]) meal {
    damn sqrt_meal(tensor_variance_1d(tensor))
}

fr fr === MATHEMATICAL OPERATIONS ===

slay tensor_abs_1d(tensor meal[value]) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        result = append(result, abs_meal(tensor[i]))
        i = i + 1
    }
    damn result
}

slay tensor_pow_1d(tensor meal[value], exponent meal) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        result = append(result, pow_meal_meal(tensor[i], exponent))
        i = i + 1
    }
    damn result
}

slay tensor_exp_1d(tensor meal[value]) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        result = append(result, exp_meal(tensor[i]))
        i = i + 1
    }
    damn result
}

slay tensor_log_1d(tensor meal[value]) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        lowkey tensor[i] <= 0.0 {
            result = append(result, 0.0)
        } {
            result = append(result, ln_meal(tensor[i]))
        }
        i = i + 1
    }
    damn result
}

slay tensor_sqrt_1d(tensor meal[value]) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        lowkey tensor[i] < 0.0 {
            result = append(result, 0.0)
        } {
            result = append(result, sqrt_meal(tensor[i]))
        }
        i = i + 1
    }
    damn result
}

fr fr === NORMALIZATION ===

slay tensor_normalize_1d(tensor meal[value]) meal[value]{
    sus mean meal = tensor_mean_1d(tensor)
    sus std_dev meal = tensor_std_dev_1d(tensor)
    
    lowkey std_dev == 0.0 {
        damn tensor_add_scalar_1d(tensor_zeros_1d(len(tensor)), -mean)
    }
    
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        sus normalized_val meal = (tensor[i] - mean) / std_dev
        result = append(result, normalized_val)
        i = i + 1
    }
    damn result
}

slay tensor_min_max_normalize_1d(tensor meal[value]) meal[value]{
    sus min_val meal = tensor_min_1d(tensor)
    sus max_val meal = tensor_max_1d(tensor)
    sus range meal = max_val - min_val
    
    lowkey range == 0.0 {
        damn tensor_zeros_1d(len(tensor))
    }
    
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        sus normalized_val meal = (tensor[i] - min_val) / range
        result = append(result, normalized_val)
        i = i + 1
    }
    damn result
}

fr fr === UTILITY FUNCTIONS ===

slay tensor_equals_1d(a meal[value], b meal[value], tolerance meal) lit {
    lowkey len(a) != len(b) {
        damn cringe
    }
    
    sus i normie = 0
    bestie i < len(a) {
        sus diff meal = abs_meal(a[i] - b[i])
        lowkey diff > tolerance {
            damn cringe
        }
        i = i + 1
    }
    damn based
}

slay tensor_copy_1d(tensor meal[value]) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        result = append(result, tensor[i])
        i = i + 1
    }
    damn result
}

slay tensor_fill_1d(size normie, value meal) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < size {
        result = append(result, value)
        i = i + 1
    }
    damn result
}

slay tensor_slice_1d(tensor meal[value], start normie, end normie) meal[value]{
    lowkey start < 0 { start = 0 }
    lowkey end > len(tensor) { end = len(tensor) }
    lowkey start >= end { damn [] }
    
    sus result meal[value] = []
    sus i normie = start
    bestie i < end {
        result = append(result, tensor[i])
        i = i + 1
    }
    damn result
}

slay tensor_concat_1d(a meal[value], b meal[value]) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(a) {
        result = append(result, a[i])
        i = i + 1
    }
    i = 0
    bestie i < len(b) {
        result = append(result, b[i])
        i = i + 1
    }
    damn result
}

fr fr === PRINT FUNCTIONS ===

slay tensor_print_1d(tensor meal[value], name tea) cringe {
    vibez.spill(name, " [", len(tensor), "]: [")
    sus i normie = 0
    bestie i < len(tensor) {
        vibez.spill(tensor[i])
        lowkey i < len(tensor) - 1 {
            vibez.spill(", ")
        }
        i = i + 1
    }
    vibez.spill("]")
}

slay matrix_print(matrix meal[value], rows normie, cols normie, name tea) cringe {
    vibez.spill(name, " [", rows, "x", cols, "]:")
    sus i normie = 0
    bestie i < rows {
        vibez.spill("  [")
        sus j normie = 0
        bestie j < cols {
            sus value meal = matrix_get(matrix, rows, cols, i, j)
            vibez.spill(value)
            lowkey j < cols - 1 {
                vibez.spill(", ")
            }
            j = j + 1
        }
        vibez.spill("]")
        i = i + 1
    }
}

fr fr === ACTIVATION FUNCTIONS FOR ML ===

slay apply_sigmoid_1d(tensor meal[value]) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        sus x meal = tensor[i]
        lowkey x > 500.0 {
            result = append(result, 1.0)
        } highkey x < -500.0 {
            result = append(result, 0.0)
        } {
            sus sigmoid_val meal = 1.0 / (1.0 + exp_meal(-x))
            result = append(result, sigmoid_val)
        }
        i = i + 1
    }
    damn result
}

slay apply_relu_1d(tensor meal[value]) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        lowkey tensor[i] > 0.0 {
            result = append(result, tensor[i])
        } {
            result = append(result, 0.0)
        }
        i = i + 1
    }
    damn result
}

slay apply_tanh_1d(tensor meal[value]) meal[value]{
    sus result meal[value] = []
    sus i normie = 0
    bestie i < len(tensor) {
        result = append(result, tanh_meal(tensor[i]))
        i = i + 1
    }
    damn result
}

fr fr === LOSS FUNCTIONS ===

slay mean_squared_error_1d(predictions meal[value], targets meal[value]) meal {
    lowkey len(predictions) != len(targets) {
        damn 0.0
    }
    
    sus sum_squared_error meal = 0.0
    sus i normie = 0
    bestie i < len(predictions) {
        sus diff meal = predictions[i] - targets[i]
        sum_squared_error = sum_squared_error + diff * diff
        i = i + 1
    }
    damn sum_squared_error / len(predictions)
}

slay mean_absolute_error_1d(predictions meal[value], targets meal[value]) meal {
    lowkey len(predictions) != len(targets) {
        damn 0.0
    }
    
    sus sum_absolute_error meal = 0.0
    sus i normie = 0
    bestie i < len(predictions) {
        sus diff meal = abs_meal(predictions[i] - targets[i])
        sum_absolute_error = sum_absolute_error + diff
        i = i + 1
    }
    damn sum_absolute_error / len(predictions)
}

fr fr === LINEAR ALGEBRA UTILITIES ===

slay vector_magnitude_1d(vector meal[value]) meal {
    sus sum_squares meal = 0.0
    sus i normie = 0
    bestie i < len(vector) {
        sum_squares = sum_squares + vector[i] * vector[i]
        i = i + 1
    }
    damn sqrt_meal(sum_squares)
}

slay vector_normalize_1d(vector meal[value]) meal[value]{
    sus magnitude meal = vector_magnitude_1d(vector)
    lowkey magnitude == 0.0 {
        damn vector
    }
    damn tensor_multiply_scalar_1d(vector, 1.0 / magnitude)
}

slay cosine_similarity_1d(a meal[value], b meal[value]) meal {
    lowkey len(a) != len(b) {
        damn 0.0
    }
    
    sus dot_product meal = tensor_dot_product_1d(a, b)
    sus magnitude_a meal = vector_magnitude_1d(a)
    sus magnitude_b meal = vector_magnitude_1d(b)
    
    lowkey magnitude_a == 0.0 || magnitude_b == 0.0 {
        damn 0.0
    }
    
    damn dot_product / (magnitude_a * magnitude_b)
}
