# mathz - Mathematical Operations

## Overview

The `mathz` module provides comprehensive mathematical functions, constants, and algorithms for CURSED programs. It includes basic arithmetic, trigonometry, statistics, number theory, and advanced mathematical operations optimized for performance and accuracy.

## Quick Start

```cursed
yeet "mathz"

sus result drip = mathz.sqrt(16.0)        // 4.0
sus angle drip = mathz.sin(mathz.PI / 2)  // 1.0  
sus maximum drip = mathz.max(10, 20)      // 20
```

## API Reference

### Mathematical Constants

```cursed
mathz.PI          // 3.141592653589793
mathz.E           // 2.718281828459045
mathz.TAU         // 6.283185307179586 (2 * PI)
mathz.GOLDEN      // 1.618033988749895 (Golden ratio)
mathz.SQRT2       // 1.4142135623730951
mathz.LN2         // 0.6931471805599453
mathz.LN10        // 2.302585092994046
mathz.INFINITY    // Positive infinity
mathz.NAN         // Not a Number
```

### Basic Operations

#### `abs(x drip) drip`
Returns the absolute value of x.

```cursed
mathz.abs(-5.5)    // 5.5
mathz.abs(3.14)    // 3.14
mathz.abs(0)       // 0
```

#### `min(a drip, b drip) drip` / `max(a drip, b drip) drip`
Returns the minimum or maximum of two values.

```cursed
mathz.min(10, 20)  // 10
mathz.max(10, 20)  // 20
```

#### `clamp(value drip, min_val drip, max_val drip) drip`
Constrains value to the range [min_val, max_val].

```cursed
mathz.clamp(15, 10, 20)  // 15 (within range)
mathz.clamp(5, 10, 20)   // 10 (clamped to min)
mathz.clamp(25, 10, 20)  // 20 (clamped to max)
```

### Power and Root Functions

#### `pow(base drip, exponent drip) drip`
Raises base to the power of exponent.

```cursed
mathz.pow(2, 3)      // 8.0
mathz.pow(9, 0.5)    // 3.0 (square root)
mathz.pow(27, 1/3)   // 3.0 (cube root)
```

#### `sqrt(x drip) drip` / `cbrt(x drip) drip`
Square root and cube root functions.

```cursed
mathz.sqrt(16)    // 4.0
mathz.sqrt(2)     // 1.414...
mathz.cbrt(27)    // 3.0
mathz.cbrt(8)     // 2.0
```

#### `hypot(x drip, y drip) drip`
Returns sqrt(x² + y²) with better numerical stability.

```cursed
mathz.hypot(3, 4)    // 5.0 (Pythagorean theorem)
mathz.hypot(1, 1)    // 1.414... (√2)
```

### Trigonometric Functions

#### Basic Trigonometry
```cursed
mathz.sin(mathz.PI / 2)     // 1.0
mathz.cos(0)                // 1.0
mathz.tan(mathz.PI / 4)     // 1.0

mathz.asin(1)               // π/2
mathz.acos(0)               // π/2
mathz.atan(1)               // π/4
mathz.atan2(1, 1)           // π/4
```

#### Hyperbolic Functions
```cursed
mathz.sinh(0)               // 0.0
mathz.cosh(0)               // 1.0
mathz.tanh(0)               // 0.0

mathz.asinh(0)              // 0.0
mathz.acosh(1)              // 0.0
mathz.atanh(0)              // 0.0
```

#### Degree Conversion
```cursed
mathz.degrees(mathz.PI)     // 180.0
mathz.radians(180)          // π
```

### Logarithmic Functions

```cursed
mathz.log(mathz.E)          // 1.0 (natural log)
mathz.log10(100)            // 2.0 (base-10 log)
mathz.log2(8)               // 3.0 (base-2 log)
mathz.logb(125, 5)          // 3.0 (arbitrary base)

mathz.exp(1)                // e
mathz.exp2(3)               // 8.0 (2³)
mathz.expm1(0)              // 0.0 (exp(x) - 1, accurate for small x)
mathz.log1p(0)              // 0.0 (log(1 + x), accurate for small x)
```

### Rounding and Truncation

```cursed
mathz.floor(3.7)            // 3.0
mathz.ceil(3.2)             // 4.0
mathz.round(3.6)            // 4.0
mathz.round(3.4)            // 3.0
mathz.trunc(3.9)            // 3.0 (towards zero)

mathz.fmod(5.3, 2.0)        // 1.3 (floating-point remainder)
mathz.remainder(5.3, 2.0)   // -0.7 (IEEE remainder)
```

### Number Classification

```cursed
mathz.is_finite(42.0)       // based
mathz.is_infinite(mathz.INFINITY)  // based
mathz.is_nan(mathz.NAN)     // based
mathz.is_normal(42.0)       // based (not zero, subnormal, infinite, or NaN)

mathz.sign(5.0)             // 1
mathz.sign(-3.0)            // -1
mathz.sign(0.0)             // 0
```

### Random Numbers

```cursed
mathz.random()              // Random float in [0, 1)
mathz.random_int(10, 20)    // Random integer in [10, 20]
mathz.random_float(1.0, 5.0) // Random float in [1.0, 5.0)

// Seed random number generator
mathz.set_random_seed(12345)
```

### Statistical Functions

#### `sum(values []drip) drip`
Sum of all values in array.

```cursed
sus numbers []drip = [1, 2, 3, 4, 5]
mathz.sum(numbers)          // 15.0
```

#### `mean(values []drip) drip`
Arithmetic mean of values.

```cursed
mathz.mean([1, 2, 3, 4, 5]) // 3.0
```

#### `median(values []drip) drip`
Middle value when sorted.

```cursed
mathz.median([1, 2, 3, 4, 5]) // 3.0
mathz.median([1, 2, 3, 4])    // 2.5
```

#### `mode(values []drip) []drip`
Most frequently occurring value(s).

```cursed
mathz.mode([1, 2, 2, 3, 3, 3]) // [3]
```

#### `variance(values []drip) drip` / `std_dev(values []drip) drip`
Statistical variance and standard deviation.

```cursed
sus data []drip = [2, 4, 4, 4, 5, 5, 7, 9]
mathz.variance(data)        // 4.0
mathz.std_dev(data)         // 2.0
```

#### `correlation(x []drip, y []drip) drip`
Pearson correlation coefficient between two datasets.

```cursed
sus x []drip = [1, 2, 3, 4, 5]
sus y []drip = [2, 4, 6, 8, 10]
mathz.correlation(x, y)     // 1.0 (perfect positive correlation)
```

### Number Theory

#### `gcd(a drip, b drip) drip` / `lcm(a drip, b drip) drip`
Greatest common divisor and least common multiple.

```cursed
mathz.gcd(48, 18)           // 6
mathz.lcm(4, 6)             // 12
```

#### `factorial(n drip) drip`
Factorial function (n!).

```cursed
mathz.factorial(5)          // 120.0
mathz.factorial(0)          // 1.0
```

#### `is_prime(n drip) lit`
Prime number test.

```cursed
mathz.is_prime(17)          // based
mathz.is_prime(15)          // false
```

#### `fibonacci(n drip) drip`
nth Fibonacci number.

```cursed
mathz.fibonacci(10)         // 55.0
mathz.fibonacci(0)          // 0.0
mathz.fibonacci(1)          // 1.0
```

### Matrix Operations

#### Basic Matrix Type
```cursed
struct Matrix {
    rows drip,
    cols drip,
    data []drip
}
```

#### `matrix_create(rows drip, cols drip) Matrix`
Create a new zero-initialized matrix.

```cursed
sus m Matrix = mathz.matrix_create(3, 3)
```

#### `matrix_multiply(a Matrix, b Matrix) Matrix yikes<tea>`
Matrix multiplication.

```cursed
sus result Matrix = mathz.matrix_multiply(matrix_a, matrix_b) fam {
    when "dimension mismatch" -> {
        vibez.spill_error("Cannot multiply matrices: incompatible dimensions")
        damn Matrix{0, 0, []}
    }
}
```

#### `matrix_transpose(m Matrix) Matrix`
Matrix transpose.

```cursed
sus transposed Matrix = mathz.matrix_transpose(original)
```

#### `matrix_determinant(m Matrix) drip yikes<tea>`
Calculate matrix determinant (square matrices only).

```cursed
sus det drip = mathz.matrix_determinant(square_matrix) fam {
    when "not square" -> damn 0.0
}
```

## Advanced Usage

### Numerical Integration
```cursed
// Simpson's rule integration
slay integrate_simpsons(func slay(drip) drip, a drip, b drip, n drip) drip {
    sus h drip = (b - a) / n
    sus sum drip = func(a) + func(b)
    
    bestie (sus i drip = 1; i < n; i++) {
        sus x drip = a + i * h
        ready (i % 2 == 0) {
            sum += 2 * func(x)
        } otherwise {
            sum += 4 * func(x)
        }
    }
    
    damn (h / 3) * sum
}

// Usage
slay square(x drip) drip { damn x * x }
sus area drip = integrate_simpsons(square, 0, 2, 100) // ∫₀² x² dx ≈ 8/3
```

### Polynomial Operations
```cursed
// Evaluate polynomial using Horner's method
slay polynomial_eval(coeffs []drip, x drip) drip {
    sus result drip = coeffs[0]
    bestie (sus i drip = 1; i < coeffs.length; i++) {
        result = result * x + coeffs[i]
    }
    damn result
}

// Find polynomial roots using bisection method
slay find_root(coeffs []drip, a drip, b drip, tolerance drip) drip yikes<tea> {
    sus fa drip = polynomial_eval(coeffs, a)
    sus fb drip = polynomial_eval(coeffs, b)
    
    ready (fa * fb > 0) {
        yikes "no root in interval"
    }
    
    bestie (mathz.abs(b - a) > tolerance) {
        sus c drip = (a + b) / 2
        sus fc drip = polynomial_eval(coeffs, c)
        
        ready (fa * fc < 0) {
            b = c
            fb = fc
        } otherwise {
            a = c
            fa = fc
        }
    }
    
    damn (a + b) / 2
}
```

### Statistical Analysis
```cursed
// Linear regression
struct LinearRegression {
    slope drip,
    intercept drip,
    r_squared drip
}

slay linear_regression(x []drip, y []drip) LinearRegression yikes<tea> {
    ready (x.length != y.length) {
        yikes "arrays must have same length"
    }
    
    sus n drip = x.length.(drip)
    sus sum_x drip = mathz.sum(x)
    sus sum_y drip = mathz.sum(y)
    sus sum_xy drip = 0.0
    sus sum_x2 drip = 0.0
    
    bestie (sus i drip = 0; i < n; i++) {
        sum_xy += x[i] * y[i]
        sum_x2 += x[i] * x[i]
    }
    
    sus slope drip = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x)
    sus intercept drip = (sum_y - slope * sum_x) / n
    
    // Calculate R²
    sus y_mean drip = sum_y / n
    sus ss_tot drip = 0.0
    sus ss_res drip = 0.0
    
    bestie (sus i drip = 0; i < n; i++) {
        sus y_pred drip = slope * x[i] + intercept
        ss_tot += mathz.pow(y[i] - y_mean, 2)
        ss_res += mathz.pow(y[i] - y_pred, 2)
    }
    
    sus r_squared drip = 1 - (ss_res / ss_tot)
    
    damn LinearRegression{slope, intercept, r_squared}
}
```

## Performance Optimization

### Fast Mathematical Operations
```cursed
// Fast inverse square root (Quake algorithm)
slay fast_inv_sqrt(x drip) drip {
    sus half_x drip = x * 0.5
    sus bits drip = x.as_bits()
    bits = 0x5f3759df - (bits >> 1)
    sus y drip = bits.as_float()
    y = y * (1.5 - (half_x * y * y))  // Newton-Raphson iteration
    damn y
}

// Fast integer operations
slay fast_log2(x drip) drip {
    sus bits drip = x.as_bits()
    damn ((bits >> 23) & 0xFF) - 127
}

slay is_power_of_two(x drip) lit {
    damn x > 0 && (x & (x - 1)) == 0
}
```

### SIMD Operations (Platform Specific)
```cursed
// Vector addition using platform SIMD when available
slay vector_add_simd(a []drip, b []drip, result []drip) {
    ready (has_simd_support()) {
        vector_add_simd_native(a, b, result)
    } otherwise {
        bestie (sus i drip = 0; i < a.length; i++) {
            result[i] = a[i] + b[i]
        }
    }
}
```

## Error Handling

### Domain Errors
```cursed
slay safe_sqrt(x drip) drip yikes<tea> {
    ready (x < 0) {
        yikes "square root of negative number"
    }
    damn mathz.sqrt(x)
}

slay safe_log(x drip) drip yikes<tea> {
    ready (x <= 0) {
        yikes "logarithm of non-positive number"
    }
    damn mathz.log(x)
}

slay safe_divide(a drip, b drip) drip yikes<tea> {
    ready (mathz.abs(b) < 1e-15) {
        yikes "division by zero"
    }
    damn a / b
}
```

### Numerical Stability
```cursed
// Numerically stable computation of (a² + b²)
slay stable_hypot(a drip, b drip) drip {
    sus abs_a drip = mathz.abs(a)
    sus abs_b drip = mathz.abs(b)
    
    ready (abs_a > abs_b) {
        sus ratio drip = abs_b / abs_a
        damn abs_a * mathz.sqrt(1 + ratio * ratio)
    } otherwise ready (abs_b > 0) {
        sus ratio drip = abs_a / abs_b
        damn abs_b * mathz.sqrt(1 + ratio * ratio)
    } otherwise {
        damn 0.0
    }
}

// Kahan summation for improved accuracy
slay precise_sum(values []drip) drip {
    sus sum drip = 0.0
    sus compensation drip = 0.0
    
    bestie (sus value drip : values) {
        sus adjusted drip = value - compensation
        sus new_sum drip = sum + adjusted
        compensation = (new_sum - sum) - adjusted
        sum = new_sum
    }
    
    damn sum
}
```

## Testing

### Unit Tests
```cursed
// stdlib/mathz/test_mathz.csd
yeet "testz"
yeet "mathz"

slay test_basic_operations() {
    testz.assert_eq_float(mathz.abs(-5.5), 5.5, 1e-10)
    testz.assert_eq_float(mathz.min(10, 20), 10, 1e-10)
    testz.assert_eq_float(mathz.max(10, 20), 20, 1e-10)
}

slay test_trigonometry() {
    testz.assert_eq_float(mathz.sin(mathz.PI / 2), 1.0, 1e-10)
    testz.assert_eq_float(mathz.cos(0), 1.0, 1e-10)
    testz.assert_eq_float(mathz.tan(mathz.PI / 4), 1.0, 1e-10)
}

slay test_statistics() {
    sus data []drip = [1, 2, 3, 4, 5]
    testz.assert_eq_float(mathz.mean(data), 3.0, 1e-10)
    testz.assert_eq_float(mathz.sum(data), 15.0, 1e-10)
}

slay test_number_theory() {
    testz.assert_eq_int(mathz.gcd(48, 18), 6)
    testz.assert_eq_int(mathz.lcm(4, 6), 12)
    testz.assert_eq_int(mathz.factorial(5), 120)
    testz.assert_true(mathz.is_prime(17))
    testz.assert_false(mathz.is_prime(15))
}

slay main() {
    testz.start_suite("mathz Tests")
    test_basic_operations()
    test_trigonometry()
    test_statistics()
    test_number_theory()
    testz.print_summary()
}
```

### Performance Tests
```cursed
// Performance benchmarks
yeet "mathz"
yeet "timez"

slay benchmark_sqrt() {
    sus iterations drip = 1000000
    sus start drip = timez.get_time_microseconds()
    
    bestie (sus i drip = 0; i < iterations; i++) {
        mathz.sqrt(i.(drip) + 1)
    }
    
    sus elapsed drip = timez.get_time_microseconds() - start
    vibez.spill("sqrt:", iterations, "iterations in", elapsed, "μs")
    vibez.spill("Average:", elapsed / iterations, "μs per operation")
}
```

## Platform Support

### Floating Point Standards
- **IEEE 754**: Full compliance for all operations
- **Precision**: 64-bit double precision by default
- **Special Values**: Proper handling of ±∞, NaN, and subnormal numbers

### Performance Optimizations
- **Hardware Instructions**: Uses native CPU math instructions when available
- **SIMD**: Vector operations for array processing
- **Cache Optimization**: Memory layout optimized for cache efficiency

## Best Practices

### Numerical Accuracy
1. **Use appropriate epsilon for floating-point comparisons**
2. **Prefer stable algorithms for ill-conditioned problems**
3. **Consider numerical precision requirements**
4. **Handle edge cases (zero, infinity, NaN)**

### Performance
1. **Pre-compute constants when possible**
2. **Use integer operations when precision allows**
3. **Batch operations on arrays for SIMD benefits**
4. **Profile mathematical hotspots**

### Error Handling
1. **Validate input domains for mathematical functions**
2. **Handle numerical overflow/underflow**
3. **Provide meaningful error messages**
4. **Use appropriate error propagation patterns**

---

The `mathz` module provides comprehensive mathematical functionality with emphasis on correctness, performance, and numerical stability. It serves as the foundation for scientific computing, graphics, statistics, and any application requiring reliable mathematical operations.
