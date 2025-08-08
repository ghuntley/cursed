# Tensorz - CURSED Tensor Operations Module

A comprehensive tensor operations library implemented in pure CURSED, providing foundational building blocks for machine learning, scientific computing, and linear algebra operations.

## Features

### Tensor Creation
- **Basic tensors**: `tensor_new()`, `tensor_zeros()`, `tensor_ones()`
- **Identity matrices**: `tensor_eye()`
- **From arrays**: `tensor_from_array()`
- **Random tensors**: `tensor_random()`, `tensor_random_normal()`

### Tensor Operations
- **Arithmetic**: Element-wise addition, subtraction, multiplication, division
- **Scalar operations**: Addition and multiplication with scalars
- **Matrix operations**: Matrix multiplication (`tensor_matmul`), transpose, dot product
- **Mathematical functions**: `tensor_exp()`, `tensor_log()`, `tensor_sqrt()`, `tensor_abs()`, `tensor_pow()`

### Shape Manipulation
- **Reshaping**: `tensor_reshape()`, `tensor_flatten()`
- **Transposition**: `tensor_transpose_2d()`
- **Squeezing**: `tensor_squeeze()` (remove dimensions of size 1)
- **Slicing**: `tensor_slice_1d()`
- **Concatenation**: `tensor_concat_axis0()`

### Statistics
- **Basic stats**: `tensor_mean()`, `tensor_sum()`, `tensor_min()`, `tensor_max()`
- **Advanced stats**: `tensor_variance()`, `tensor_std_dev()`
- **Full statistics**: `tensor_stats()` (returns comprehensive stats struct)

### Normalization
- **Z-score normalization**: `tensor_normalize()` (mean=0, std=1)
- **Min-max normalization**: `tensor_min_max_normalize()` (scale to [0,1])

### Utilities
- **Comparison**: `tensor_equals()`, `tensor_same_shape()`
- **Copying**: `tensor_copy()`, `tensor_fill_value()`
- **Indexing**: `tensor_get()`, `tensor_set()` with multi-dimensional indices
- **Printing**: `tensor_print_shape()`, `tensor_print_data()`

## Data Structures

### Tensor
```cursed
squad Tensor {
    spill data []meal        fr fr Flattened data storage
    spill shape []normie     fr fr Dimensions
    spill ndim normie        fr fr Number of dimensions
    spill size normie        fr fr Total number of elements
}
```

### TensorStats
```cursed
squad TensorStats {
    spill mean meal
    spill variance meal
    spill std_dev meal
    spill min_val meal
    spill max_val meal
    spill sum meal
}
```

## Usage Examples

### Basic Tensor Creation
```cursed
yeet "tensorz"

fr fr Create a 3x3 zero tensor
sus shape []normie = [3, 3]
sus zeros Tensor = tensor_zeros(shape)

fr fr Create tensor from data
sus data []meal = [1.0, 2.0, 3.0, 4.0]
sus matrix_shape []normie = [2, 2]
sus matrix Tensor = tensor_from_array(data, matrix_shape)

fr fr Create identity matrix
sus identity Tensor = tensor_eye(3)
```

### Matrix Operations
```cursed
fr fr Matrix multiplication
sus a_data []meal = [1.0, 2.0, 3.0, 4.0]
sus b_data []meal = [5.0, 6.0, 7.0, 8.0]
sus shape []normie = [2, 2]
sus matrix_a Tensor = tensor_from_array(a_data, shape)
sus matrix_b Tensor = tensor_from_array(b_data, shape)
sus result Tensor = tensor_matmul(matrix_a, matrix_b)

fr fr Transpose
sus transposed Tensor = tensor_transpose_2d(matrix_a)
```

### Element-wise Operations
```cursed
fr fr Arithmetic operations
sus sum_tensor Tensor = tensor_add(matrix_a, matrix_b)
sus diff_tensor Tensor = tensor_subtract(matrix_a, matrix_b)
sus product_tensor Tensor = tensor_multiply(matrix_a, matrix_b)

fr fr Scalar operations
sus scaled Tensor = tensor_multiply_scalar(matrix_a, 2.0)
sus shifted Tensor = tensor_add_scalar(matrix_a, 1.0)
```

### Statistical Operations
```cursed
fr fr Basic statistics
sus mean_val meal = tensor_mean(matrix_a)
sus sum_val meal = tensor_sum(matrix_a)
sus variance_val meal = tensor_variance(matrix_a)

fr fr Complete statistics
sus stats TensorStats = tensor_stats(matrix_a)
vibez.spill("Mean: ", stats.mean)
vibez.spill("Std Dev: ", stats.std_dev)
```

### Normalization
```cursed
fr fr Z-score normalization
sus normalized Tensor = tensor_normalize(matrix_a)

fr fr Min-max normalization
sus minmax_normalized Tensor = tensor_min_max_normalize(matrix_a)
```

### Shape Manipulation
```cursed
fr fr Reshape tensor
sus new_shape []normie = [4, 1]
sus reshaped Tensor = tensor_reshape(matrix_a, new_shape)

fr fr Flatten to 1D
sus flattened Tensor = tensor_flatten(matrix_a)
```

### Random Tensors
```cursed
fr fr Random uniform distribution
sus random_tensor Tensor = tensor_random([3, 3], 0.0, 1.0)

fr fr Random normal distribution
sus normal_tensor Tensor = tensor_random_normal([100], 0.0, 1.0)
```

## Performance Considerations

- **Memory efficiency**: Tensors use flattened internal storage for cache locality
- **Numerical stability**: Division operations include zero-checking
- **Bounds checking**: Indexing operations validate tensor bounds
- **Broadcasting**: Limited broadcasting support (same-shape operations prioritized)

## Error Handling

The library follows CURSED's error handling patterns:
- Invalid operations return empty tensors or original tensors unchanged
- Out-of-bounds access returns zero values
- Division by zero returns zero (safe fallback)
- Invalid reshapes return the original tensor

## Dependencies

- `mathz`: Mathematical functions and constants
- `arrayz`: Array manipulation utilities

## Testing

Run the comprehensive test suite:
```bash
./zig-out/bin/cursed stdlib/tensorz/test_tensorz.csd
```

The test suite covers:
- Tensor creation and initialization
- All arithmetic and mathematical operations
- Shape manipulation and indexing
- Statistical computations
- Normalization methods
- Edge cases and error conditions

## Integration with MLz

The `tensorz` module is designed to work seamlessly with the `mlz` machine learning module, providing the foundational tensor operations needed for:
- Neural network computations
- Linear algebra operations in ML algorithms
- Data preprocessing and normalization
- Statistical analysis of datasets

## Implementation Notes

- **Pure CURSED**: No FFI dependencies, completely implemented in CURSED
- **Numerical precision**: Uses `meal` (float) type for numerical computations
- **Memory management**: Follows CURSED's automatic memory management patterns
- **Extensible**: Designed for easy extension with additional tensor operations
