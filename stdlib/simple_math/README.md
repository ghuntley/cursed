# Simple Mathematics (simple_math)

The `simple_math` module provides basic mathematical operations and functions for CURSED programs.

## Purpose

This module implements essential mathematical operations including arithmetic, trigonometry, logarithms, and statistical functions with a focus on simplicity and accuracy.

## Main Functions

### Basic Arithmetic
- `simple_math.add(a, b)` - Addition
- `simple_math.subtract(a, b)` - Subtraction  
- `simple_math.multiply(a, b)` - Multiplication
- `simple_math.divide(a, b)` - Division
- `simple_math.mod(a, b)` - Modulo operation
- `simple_math.pow(base, exponent)` - Power operation
- `simple_math.sqrt(x)` - Square root
- `simple_math.abs(x)` - Absolute value

### Rounding and Precision
- `simple_math.floor(x)` - Round down to integer
- `simple_math.ceil(x)` - Round up to integer
- `simple_math.round(x)` - Round to nearest integer
- `simple_math.round_to(x, decimals)` - Round to N decimal places
- `simple_math.trunc(x)` - Truncate to integer

### Trigonometric Functions
- `simple_math.sin(x)` - Sine function
- `simple_math.cos(x)` - Cosine function
- `simple_math.tan(x)` - Tangent function
- `simple_math.asin(x)` - Arcsine function
- `simple_math.acos(x)` - Arccosine function
- `simple_math.atan(x)` - Arctangent function
- `simple_math.atan2(y, x)` - Two-argument arctangent

### Logarithmic Functions
- `simple_math.log(x)` - Natural logarithm
- `simple_math.log10(x)` - Base-10 logarithm
- `simple_math.log2(x)` - Base-2 logarithm
- `simple_math.exp(x)` - Exponential function (e^x)

### Constants
- `simple_math.PI` - Pi constant (3.14159...)
- `simple_math.E` - Euler's number (2.71828...)
- `simple_math.TAU` - Tau constant (2*PI)

### Comparison and Utilities
- `simple_math.min(a, b)` - Minimum of two values
- `simple_math.max(a, b)` - Maximum of two values
- `simple_math.clamp(x, min, max)` - Clamp value to range
- `simple_math.sign(x)` - Sign of number (-1, 0, or 1)
- `simple_math.is_nan(x)` - Check if value is NaN
- `simple_math.is_infinite(x)` - Check if value is infinite

### Random Numbers
- `simple_math.random()` - Random float between 0 and 1
- `simple_math.random_int(min, max)` - Random integer in range
- `simple_math.random_range(min, max)` - Random float in range

## Usage Examples

### Basic Arithmetic Operations

```cursed
yeet "simple_math"

sus a meal = 10.5
sus b meal = 3.2

vibez.spillf("Addition: {} + {} = {}", a, b, simple_math.add(a, b))
vibez.spillf("Subtraction: {} - {} = {}", a, b, simple_math.subtract(a, b))
vibez.spillf("Multiplication: {} * {} = {}", a, b, simple_math.multiply(a, b))
vibez.spillf("Division: {} / {} = {}", a, b, simple_math.divide(a, b))

sus power = simple_math.pow(2, 8)
vibez.spillf("2^8 = {}", power)

sus root = simple_math.sqrt(25)
vibez.spillf("√25 = {}", root)
```

### Trigonometric Calculations

```cursed
yeet "simple_math"

sus angle meal = simple_math.PI / 4  # 45 degrees in radians

vibez.spillf("sin(π/4) = {}", simple_math.sin(angle))
vibez.spillf("cos(π/4) = {}", simple_math.cos(angle))
vibez.spillf("tan(π/4) = {}", simple_math.tan(angle))

fr fr Convert degrees to radians
slay degrees_to_radians(degrees meal) meal {
    damn degrees * simple_math.PI / 180.0
}

sus angle_deg meal = 60.0
sus angle_rad meal = degrees_to_radians(angle_deg)
vibez.spillf("sin(60°) = {}", simple_math.sin(angle_rad))
```

### Rounding and Precision

```cursed
yeet "simple_math"

sus value meal = 3.14159265

vibez.spillf("Original: {}", value)
vibez.spillf("Floor: {}", simple_math.floor(value))
vibez.spillf("Ceiling: {}", simple_math.ceil(value))
vibez.spillf("Rounded: {}", simple_math.round(value))
vibez.spillf("2 decimals: {}", simple_math.round_to(value, 2))
vibez.spillf("4 decimals: {}", simple_math.round_to(value, 4))
```

### Statistical Operations

```cursed
yeet "simple_math"

sus numbers []meal = [1.5, 2.8, 3.1, 4.7, 5.2, 6.9, 7.3]

fr fr Find minimum and maximum
sus min_val meal = numbers[0]
sus max_val meal = numbers[0]

bestie num in numbers {
    min_val = simple_math.min(min_val, num)
    max_val = simple_math.max(max_val, num)
}

vibez.spillf("Min: {}, Max: {}", min_val, max_val)

fr fr Calculate average
sus sum meal = 0.0
bestie num in numbers {
    sum = simple_math.add(sum, num)
}
sus average meal = simple_math.divide(sum, numbers.len())
vibez.spillf("Average: {}", average)
```

### Random Number Generation

```cursed
yeet "simple_math"

fr fr Generate random numbers
vibez.spillf("Random float [0,1): {}", simple_math.random())
vibez.spillf("Random int [1,10]: {}", simple_math.random_int(1, 10))
vibez.spillf("Random float [5.0,15.0): {}", simple_math.random_range(5.0, 15.0))

fr fr Simulate dice roll
sus dice_roll = simple_math.random_int(1, 6)
vibez.spillf("Dice roll: {}", dice_roll)

fr fr Generate random point in circle
sus angle = simple_math.random_range(0.0, simple_math.TAU)
sus radius = simple_math.random()
sus x = simple_math.multiply(radius, simple_math.cos(angle))
sus y = simple_math.multiply(radius, simple_math.sin(angle))
vibez.spillf("Random point: ({}, {})", x, y)
```

### Mathematical Utilities

```cursed
yeet "simple_math"

sus values []meal = [-5.2, 0.0, 3.7, simple_math.sqrt(-1)]

bestie value in values {
    vibez.spillf("Value: {}", value)
    vibez.spillf("  Absolute: {}", simple_math.abs(value))
    vibez.spillf("  Sign: {}", simple_math.sign(value))
    vibez.spillf("  Is NaN: {}", simple_math.is_nan(value))
    vibez.spillf("  Is Infinite: {}", simple_math.is_infinite(value))
}

fr fr Clamp values to range
sus input meal = 15.7
sus clamped = simple_math.clamp(input, 0.0, 10.0)
vibez.spillf("Clamped {} to [0,10]: {}", input, clamped)
```

## Compilation Examples

### Interpretation Mode
```bash
echo 'yeet "simple_math"
sus result = simple_math.pow(2, 10)
vibez.spillf("2^10 = {}", result)' > math_test.csd

./cursed-unified math_test.csd
```

### Compilation Mode
```bash
./cursed-unified --compile math_test.csd
./math_test
```

## Advanced Examples

### Distance Calculation

```cursed
yeet "simple_math"

slay distance_2d(x1 meal, y1 meal, x2 meal, y2 meal) meal {
    sus dx = simple_math.subtract(x2, x1)
    sus dy = simple_math.subtract(y2, y1)
    sus dx_squared = simple_math.pow(dx, 2)
    sus dy_squared = simple_math.pow(dy, 2)
    damn simple_math.sqrt(simple_math.add(dx_squared, dy_squared))
}

sus dist = distance_2d(0.0, 0.0, 3.0, 4.0)
vibez.spillf("Distance: {}", dist)  # Should be 5.0
```

### Compound Interest Calculator

```cursed
yeet "simple_math"

slay compound_interest(principal meal, rate meal, time meal, compounds_per_year normie) meal {
    sus rate_per_period = simple_math.divide(rate, compounds_per_year)
    sus total_periods = simple_math.multiply(time, compounds_per_year)
    sus base = simple_math.add(1.0, rate_per_period)
    sus multiplier = simple_math.pow(base, total_periods)
    damn simple_math.multiply(principal, multiplier)
}

sus final_amount = compound_interest(1000.0, 0.05, 10.0, 12)
vibez.spillf("$1000 at 5% for 10 years: ${}", simple_math.round_to(final_amount, 2))
```

## Implementation Notes

- High precision floating-point arithmetic
- Proper handling of edge cases (NaN, infinity)
- Optimized algorithms for trigonometric functions
- Thread-safe random number generation
- Pure CURSED implementation (no FFI)

## Dependencies

- Core numeric types from runtime
- `memory` - For internal calculations
- No external mathematical libraries

## Performance Considerations

- Efficient implementations of mathematical functions
- Lookup tables for trigonometric functions
- Optimized for common mathematical operations
- Minimal memory allocation for calculations

## Best Practices

1. **Check for division by zero** before calling divide
2. **Validate input ranges** for trigonometric functions
3. **Use appropriate precision** for your use case
4. **Handle NaN and infinity** in calculations
5. **Seed random numbers** for reproducible results
6. **Use constants** instead of recalculating values
7. **Consider numerical stability** for iterative algorithms

## Error Handling

```cursed
slay safe_divide(a meal, b meal) Result<meal, tea> {
    if simple_math.abs(b) < 0.000001 {
        damn error_drip.new_error("Division by zero")
    }
    damn error_drip.ok(simple_math.divide(a, b))
}
```
