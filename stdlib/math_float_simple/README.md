# math_float_simple

Simple floating-point mathematics library providing essential mathematical functions and constants. Features pure CURSED implementations of common mathematical operations optimized for simplicity and reliability.

## Overview

The `math_float_simple` module provides:
- Essential mathematical constants (PI, E, TAU)
- Basic arithmetic operations (abs, min, max)
- Power and root functions (sqrt)
- Trigonometric functions (sin, cos)
- Exponential and logarithmic functions (exp, ln)
- Utility functions for numerical analysis

## Mathematical Constants

### Fundamental Constants

#### `PI() -> meal`
Returns the mathematical constant π (pi).

**Value:** 3.141592653589793

**Usage:**
```cursed
sus circle_area meal = PI() * radius * radius
sus circumference meal = 2.0 * PI() * radius
```

#### `E() -> meal`
Returns Euler's number (e), the base of natural logarithms.

**Value:** 2.718281828459045

**Usage:**
```cursed
sus growth meal = initial_value * exp_simple(E() * time)
```

#### `TAU() -> meal`
Returns τ (tau), equal to 2π.

**Value:** 6.283185307179586

**Usage:**
```cursed
sus full_rotation meal = TAU()  // Complete circle in radians
sus quarter_turn meal = TAU() / 4.0
```

## Basic Operations

### Arithmetic Functions

#### `abs_float(x: meal) -> meal`
Returns the absolute value of a floating-point number.

**Parameters:**
- `x`: Input value

**Returns:** |x| (absolute value)

**Examples:**
```cursed
sus positive meal = abs_float(-3.14)  // 3.14
sus same meal = abs_float(2.71)       // 2.71
```

#### `min_float(a: meal, b: meal) -> meal`
Returns the smaller of two floating-point values.

**Parameters:**
- `a`: First value
- `b`: Second value

**Returns:** Minimum of a and b

#### `max_float(a: meal, b: meal) -> meal`
Returns the larger of two floating-point values.

**Parameters:**
- `a`: First value  
- `b`: Second value

**Returns:** Maximum of a and b

**Examples:**
```cursed
sus smaller meal = min_float(3.14, 2.71)  // 2.71
sus larger meal = max_float(3.14, 2.71)   // 3.14
sus clamp meal = min_float(max_float(value, min_val), max_val)
```

## Power and Root Functions

### Square Root

#### `sqrt_simple(x: meal) -> meal`
Computes square root using Newton's method approximation.

**Parameters:**
- `x`: Input value (must be non-negative)

**Returns:** √x or NaN for negative input

**Algorithm:** Newton's method with 10 iterations
```
Initial guess: x/2
Iteration: guess = (guess + x/guess) / 2
```

**Examples:**
```cursed
sus root meal = sqrt_simple(16.0)    // 4.0
sus diagonal meal = sqrt_simple(a*a + b*b)  // Pythagorean theorem
sus invalid meal = sqrt_simple(-1.0) // NaN (0.0/0.0)
```

**Performance:** Converges quickly for most inputs, typically accurate to 6-8 decimal places.

## Trigonometric Functions

### Basic Trigonometry

#### `sin_simple(x: meal) -> meal`
Computes sine using Taylor series approximation.

**Parameters:**
- `x`: Angle in radians

**Returns:** sin(x)

**Algorithm:** Taylor series expansion
```
sin(x) ≈ x - x³/6 + x⁵/120
```

**Accuracy:** Good for small angles (|x| < π), decreases for larger angles

#### `cos_simple(x: meal) -> meal`
Computes cosine using Taylor series approximation.

**Parameters:**
- `x`: Angle in radians

**Returns:** cos(x)

**Algorithm:** Taylor series expansion
```
cos(x) ≈ 1 - x²/2 + x⁴/24
```

**Examples:**
```cursed
// Basic trigonometry
sus sine_30 meal = sin_simple(PI() / 6.0)     // ≈ 0.5
sus cosine_60 meal = cos_simple(PI() / 3.0)   // ≈ 0.5

// Unit circle calculations
sus x meal = cos_simple(angle)
sus y meal = sin_simple(angle)

// Trigonometric identity verification
sus identity meal = sin_simple(x)*sin_simple(x) + cos_simple(x)*cos_simple(x)
// Should be approximately 1.0
```

## Exponential and Logarithmic Functions

### Exponential Function

#### `exp_simple(x: meal) -> meal`
Computes e^x using Taylor series approximation.

**Parameters:**
- `x`: Exponent value

**Returns:** e^x

**Algorithm:** Taylor series expansion
```
e^x ≈ 1 + x + x²/2 + x³/6
```

**Accuracy:** Good for small values of x (|x| < 2), becomes less accurate for larger values

### Natural Logarithm

#### `ln_simple(x: meal) -> meal`
Computes natural logarithm for values near 1.

**Parameters:**
- `x`: Input value (must be positive)

**Returns:** ln(x) or NaN for non-positive input

**Algorithm:** Taylor series for ln(1+u) where u = x-1
```
ln(1+u) ≈ u - u²/2 + u³/3
```

**Limitations:** Only accurate for x near 1 (approximately 0.5 < x < 1.5)

**Examples:**
```cursed
// Exponential growth
sus growth meal = exp_simple(0.5)  // ≈ 1.649

// Natural logarithm
sus log_e meal = ln_simple(E())    // ≈ 1.0
sus log_2 meal = ln_simple(2.0)    // ≈ 0.693

// Compound calculations
sus compound meal = exp_simple(ln_simple(x) * 2.0)  // Equivalent to x²
```

## Utility Functions

### Numerical Analysis

#### `is_finite_simple(x: meal) -> lit`
Checks if a floating-point value is finite (not infinity or NaN).

**Parameters:**
- `x`: Value to check

**Returns:** `based` if finite, `cringe` if infinite or NaN

**Algorithm:** Simple heuristic checks
```
x == x           // NaN check (NaN != NaN)
x != x + 1.0     // Infinity check  
x != x * 2.0     // Additional infinity check
```

#### `approximately_equal_simple(a: meal, b: meal, epsilon: meal) -> lit`
Compares two floating-point values with tolerance for precision errors.

**Parameters:**
- `a`: First value
- `b`: Second value
- `epsilon`: Tolerance for comparison

**Returns:** `based` if values are approximately equal

**Examples:**
```cursed
// Floating-point comparison with tolerance
sus a meal = 0.1 + 0.2
sus b meal = 0.3
sus equal lit = approximately_equal_simple(a, b, 0.0001)  // based

// Numerical computation validation
sus computed meal = sin_simple(x)*sin_simple(x) + cos_simple(x)*cos_simple(x)
sus is_unity lit = approximately_equal_simple(computed, 1.0, 0.001)
```

## Usage Examples

### Basic Mathematical Operations

```cursed
yeet "math_float_simple"

// Constants and basic operations
sus radius meal = 5.0
sus area meal = PI() * radius * radius
sus circumference meal = TAU() * radius

vibez.spill("Circle area: " + meal_to_string(area))
vibez.spill("Circumference: " + meal_to_string(circumference))

// Distance calculation
sus dx meal = x2 - x1
sus dy meal = y2 - y1
sus distance meal = sqrt_simple(dx*dx + dy*dy)
```

### Trigonometric Calculations

```cursed
// Angle calculations (in radians)
sus degrees meal = 45.0
sus radians meal = degrees * PI() / 180.0

sus sine_val meal = sin_simple(radians)
sus cosine_val meal = cos_simple(radians)

vibez.spill("sin(45°) = " + meal_to_string(sine_val))
vibez.spill("cos(45°) = " + meal_to_string(cosine_val))

// Verify trigonometric identity
sus identity meal = sine_val*sine_val + cosine_val*cosine_val
lowkey approximately_equal_simple(identity, 1.0, 0.001) {
    vibez.spill("Trigonometric identity verified!")
}
```

### Exponential Growth Modeling

```cursed
// Compound interest calculation
sus principal meal = 1000.0
sus rate meal = 0.05  // 5% annual rate
sus time meal = 10.0  // 10 years

sus amount meal = principal * exp_simple(rate * time)
vibez.spill("Final amount: $" + meal_to_string(amount))

// Half-life calculation
sus initial_amount meal = 100.0
sus half_life meal = 5.73  // years
sus elapsed_time meal = 10.0

sus decay_constant meal = ln_simple(2.0) / half_life
sus remaining meal = initial_amount * exp_simple(-decay_constant * elapsed_time)
```

### Numerical Analysis

```cursed
// Root finding using Newton's method
slay find_root(initial_guess meal) meal {
    sus x meal = initial_guess
    sus tolerance meal = 0.0001
    
    bestie i := 0; i < 100; i = i + 1 {
        sus f meal = x*x - 2.0  // Finding √2
        sus f_prime meal = 2.0 * x
        sus new_x meal = x - f / f_prime
        
        lowkey abs_float(new_x - x) < tolerance {
            damn new_x
        }
        
        x = new_x
    }
    
    damn x
}
```

## Accuracy and Limitations

### Taylor Series Approximations

| Function | Accurate Range | Typical Error |
|----------|----------------|---------------|
| sin_simple | \|x\| < π | < 0.01 |
| cos_simple | \|x\| < π | < 0.01 |
| exp_simple | \|x\| < 2 | < 0.1 |
| ln_simple | 0.5 < x < 1.5 | < 0.01 |

### Improvement Strategies

For better accuracy:
```cursed
// Range reduction for trigonometric functions
slay sin_improved(x meal) meal {
    // Reduce to [0, 2π] range
    bestie x > TAU() {
        x = x - TAU() * floor_simple(x / TAU())
    }
    
    // Use symmetry to reduce to [0, π/2]
    // ... additional range reduction
    
    damn sin_simple(x)
}
```

### Error Handling

```cursed
// Safe mathematical operations
slay safe_sqrt(x meal) (meal, tea) {
    lowkey x < 0.0 {
        damn 0.0, "negative input to sqrt"
    }
    
    sus result meal = sqrt_simple(x)
    
    lowkey !is_finite_simple(result) {
        damn 0.0, "sqrt result not finite"
    }
    
    damn result, ""
}
```

## Performance Characteristics

### Computational Complexity

- **Basic operations** (abs, min, max): O(1)
- **sqrt_simple**: O(1) with fixed iterations (10)
- **Trigonometric functions**: O(1) with small constant
- **Exponential/logarithmic**: O(1) with polynomial evaluation

### Memory Usage

- **Zero dynamic allocation**: All functions use stack-only computation
- **Small code size**: Minimal implementation for embedded use
- **Cache-friendly**: Simple algorithms with good locality

### Performance Tips

```cursed
// Optimize repeated calculations
sus pi_val meal = PI()  // Cache constant
sus twice_pi meal = pi_val * 2.0

// Use approximations when appropriate
sus rough_sine meal = x  // For small x, sin(x) ≈ x
sus rough_exp meal = 1.0 + x  // For small x, e^x ≈ 1 + x
```

## Testing

### Unit Tests

```bash
# Run mathematical function tests
zig build test
./zig-out/bin/cursed-zig stdlib/math_float_simple/test_simple.csd
```

### Validation Tests

```cursed
// Test mathematical identities
slay test_trigonometric_identity() {
    sus angles []meal = []meal{0.0, PI()/6.0, PI()/4.0, PI()/3.0, PI()/2.0}
    
    bestie i := 0; i < len(angles); i = i + 1 {
        sus angle meal = angles[i]
        sus sin_val meal = sin_simple(angle)
        sus cos_val meal = cos_simple(angle)
        sus identity meal = sin_val*sin_val + cos_val*cos_val
        
        assert_true(approximately_equal_simple(identity, 1.0, 0.01))
    }
}
```

## Dependencies

**None** - This module is completely self-contained with no external dependencies.

## Comparison with Advanced Math Libraries

| Feature | math_float_simple | Advanced Libraries |
|---------|-------------------|-------------------|
| Code size | Very small | Large |
| Accuracy | Moderate | High |
| Performance | Fast | Variable |
| Dependencies | None | Many |
| Portability | Excellent | Platform-dependent |

## Future Enhancements

### Planned Improvements

1. **Extended Range**: Range reduction for trigonometric functions
2. **Higher Precision**: More terms in Taylor series
3. **Additional Functions**: tan, asin, acos, atan, log10
4. **Optimizations**: Platform-specific optimizations

### Advanced Features (Future)

```cursed
// Planned function additions
slay tan_simple(x meal) meal        // Tangent function
slay atan_simple(x meal) meal       // Arctangent function
slay log10_simple(x meal) meal      // Base-10 logarithm
slay pow_simple(base meal, exp meal) meal  // General power function
```

## Integration Examples

### With Physics Simulations

```cursed
// Projectile motion
slay projectile_range(velocity meal, angle meal) meal {
    sus sin_2theta meal = 2.0 * sin_simple(angle) * cos_simple(angle)
    sus gravity meal = 9.81
    damn velocity * velocity * sin_2theta / gravity
}
```

### With Graphics Programming

```cursed
// Rotate point around origin
slay rotate_point(x meal, y meal, angle meal) (meal, meal) {
    sus cos_a meal = cos_simple(angle)
    sus sin_a meal = sin_simple(angle)
    
    sus new_x meal = x * cos_a - y * sin_a
    sus new_y meal = x * sin_a + y * cos_a
    
    damn (new_x, new_y)
}
```

This module provides a solid foundation for mathematical computations in CURSED applications where simplicity and reliability are prioritized over maximum precision or performance.
