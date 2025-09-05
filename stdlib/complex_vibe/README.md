# complex_vibe Module

The `complex_vibe` module provides comprehensive mathematical functions for complex numbers in CURSED. It includes operations like trigonometric functions, logarithms, exponentials, and other mathematical operations specifically for complex values.

## Overview

This module implements a pure CURSED solution for complex number mathematics, providing all the essential operations needed for complex analysis and numerical computations. Complex numbers are represented using separate real and imaginary parts as `meal` (float64) values.

## Core Functions

### Complex Number Basic Operations

- `ComplexAdd(r1, i1, r2, i2 meal) meal` - Add two complex numbers (returns real part)
- `ComplexAddImag(r1, i1, r2, i2 meal) meal` - Add two complex numbers (returns imaginary part)
- `ComplexSub(r1, i1, r2, i2 meal) meal` - Subtract two complex numbers (returns real part)
- `ComplexSubImag(r1, i1, r2, i2 meal) meal` - Subtract two complex numbers (returns imaginary part)
- `ComplexMul(r1, i1, r2, i2 meal) meal` - Multiply two complex numbers (returns real part)
- `ComplexMulImag(r1, i1, r2, i2 meal) meal` - Multiply two complex numbers (returns imaginary part)
- `ComplexDiv(r1, i1, r2, i2 meal) meal` - Divide two complex numbers (returns real part)
- `ComplexDivImag(r1, i1, r2, i2 meal) meal` - Divide two complex numbers (returns imaginary part)

### Complex Number Properties

- `ComplexAbs(r, i meal) meal` - Compute absolute value (modulus) of complex number
- `ComplexPhase(r, i meal) meal` - Compute phase (argument) of complex number
- `ComplexConj(r, i meal) meal` - Compute complex conjugate (returns real part)
- `ComplexConjImag(r, i meal) meal` - Compute complex conjugate (returns imaginary part)

### Polar Coordinates

- `PolarToRect(r, theta meal) meal` - Convert polar to rectangular (returns real part)
- `PolarToRectImag(r, theta meal) meal` - Convert polar to rectangular (returns imaginary part)

### Basic Mathematical Functions

- `Sin(x meal) meal` - Compute sine using Taylor series
- `Cos(x meal) meal` - Compute cosine using Taylor series
- `Exp(x meal) meal` - Compute exponential using Taylor series

### Complex Mathematical Functions

- `ComplexExp(r, i meal) meal` - Compute e^(r+i*i) (returns real part)
- `ComplexExpImag(r, i meal) meal` - Compute e^(r+i*i) (returns imaginary part)
- `ComplexSin(r, i meal) meal` - Compute sin(r+i*i) (returns real part)
- `ComplexSinImag(r, i meal) meal` - Compute sin(r+i*i) (returns imaginary part)
- `ComplexCos(r, i meal) meal` - Compute cos(r+i*i) (returns real part)
- `ComplexCosImag(r, i meal) meal` - Compute cos(r+i*i) (returns imaginary part)

### Utility Functions

- `IsComplexZero(r, i meal) lit` - Check if complex number is zero
- `IsComplexReal(r, i meal) lit` - Check if complex number is real
- `IsComplexImag(r, i meal) lit` - Check if complex number is purely imaginary
- `ComplexEqual(r1, i1, r2, i2 meal) lit` - Check if two complex numbers are equal

## Usage Examples

```cursed
yeet "complex_vibe"
yeet "mathz"

// Working with complex numbers (represented as separate real and imaginary parts)
sus r1 meal = 3.0
sus i1 meal = 4.0
sus r2 meal = 1.0
sus i2 meal = 2.0

vibez.spill("Complex number 1: " + r1.(tea) + " + " + i1.(tea) + "i")
vibez.spill("Complex number 2: " + r2.(tea) + " + " + i2.(tea) + "i")

// Basic operations
sus sum_r meal = complex_vibe.ComplexAdd(r1, i1, r2, i2)
sus sum_i meal = complex_vibe.ComplexAddImag(r1, i1, r2, i2)
sus prod_r meal = complex_vibe.ComplexMul(r1, i1, r2, i2)
sus prod_i meal = complex_vibe.ComplexMulImag(r1, i1, r2, i2)

vibez.spill("Sum: " + sum_r.(tea) + " + " + sum_i.(tea) + "i")
vibez.spill("Product: " + prod_r.(tea) + " + " + prod_i.(tea) + "i")

// Absolute value and phase
sus abs_z1 meal = complex_vibe.ComplexAbs(r1, i1)
sus phase_z1 meal = complex_vibe.ComplexPhase(r1, i1)

vibez.spill("Absolute value: " + abs_z1.(tea))
vibez.spill("Phase: " + phase_z1.(tea) + " radians")

// Complex conjugate
sus conj_r meal = complex_vibe.ComplexConj(r1, i1)
sus conj_i meal = complex_vibe.ComplexConjImag(r1, i1)
vibez.spill("Conjugate: " + conj_r.(tea) + " + " + conj_i.(tea) + "i")

// Polar coordinates
sus rect_r meal = complex_vibe.PolarToRect(5.0, 0.7854)  // π/4 radians
sus rect_i meal = complex_vibe.PolarToRectImag(5.0, 0.7854)
vibez.spill("From polar (5, π/4): " + rect_r.(tea) + " + " + rect_i.(tea) + "i")

// Exponential and trigonometric functions
sus exp_r meal = complex_vibe.ComplexExp(1.0, 0.0)
sus exp_i meal = complex_vibe.ComplexExpImag(1.0, 0.0)
vibez.spill("e^(1+0i): " + exp_r.(tea) + " + " + exp_i.(tea) + "i")

sus sin_r meal = complex_vibe.ComplexSin(0.5, 0.0)
sus sin_i meal = complex_vibe.ComplexSinImag(0.5, 0.0)
vibez.spill("sin(0.5+0i): " + sin_r.(tea) + " + " + sin_i.(tea) + "i")

sus cos_r meal = complex_vibe.ComplexCos(0.0, 0.0)
sus cos_i meal = complex_vibe.ComplexCosImag(0.0, 0.0)
vibez.spill("cos(0+0i): " + cos_r.(tea) + " + " + cos_i.(tea) + "i")

// Utility functions
sus is_zero lit = complex_vibe.IsComplexZero(0.0, 0.0)
sus is_real lit = complex_vibe.IsComplexReal(5.0, 0.0)
sus is_imag lit = complex_vibe.IsComplexImag(0.0, 3.0)
sus equal lit = complex_vibe.ComplexEqual(r1, i1, r1, i1)

vibez.spill("Is zero: " + is_zero.(tea))
vibez.spill("Is real: " + is_real.(tea))
vibez.spill("Is purely imaginary: " + is_imag.(tea))
vibez.spill("Equal to itself: " + equal.(tea))

// Basic trigonometric functions
sus sin_val meal = complex_vibe.Sin(1.5708)  // π/2
sus cos_val meal = complex_vibe.Cos(0.0)
sus exp_val meal = complex_vibe.Exp(1.0)

vibez.spill("sin(π/2): " + sin_val.(tea))
vibez.spill("cos(0): " + cos_val.(tea))
vibez.spill("exp(1): " + exp_val.(tea))

// Mathematical identities
sus abs_squared meal = abs_z1 * abs_z1
sus z_conj_prod meal = complex_vibe.ComplexMul(r1, i1, conj_r, conj_i)
sus identity_check lit = mathz.Abs(abs_squared - z_conj_prod) < 0.001
vibez.spill("Modulus identity |z|² = z·z*: " + identity_check.(tea))
```

## Mathematical Identities

The module maintains important mathematical identities:

1. **Euler's Identity**: e^(iπ) + 1 = 0
2. **Trigonometric Identity**: sin²(z) + cos²(z) = 1
3. **Conjugate Properties**: z · z̄ = |z|²
4. **Polar Form**: z = r · e^(iθ) = r(cos θ + i sin θ)

## Implementation Features

- **Pure CURSED Implementation**: No external dependencies
- **Numerical Stability**: Algorithms designed for accuracy using Taylor series
- **Mathematical Consistency**: Preserves real-valued behavior for real inputs
- **Comprehensive Coverage**: All major complex math operations
- **Separate Component Design**: Real and imaginary parts handled separately for clarity
- **Mathematical Precision**: Functions use appropriate precision for calculations

## Implementation Notes

### Complex Number Representation

Complex numbers are represented as separate real and imaginary parts (both `meal` type) rather than a single complex type. This approach provides:

- **Clarity**: Each component is explicitly accessible
- **Flexibility**: Easy to work with real and imaginary parts separately
- **Compatibility**: Works with existing CURSED math functions
- **Simplicity**: No need for custom complex number types

### Mathematical Accuracy

The module uses Taylor series expansions for trigonometric and exponential functions:

- **Sin/Cos**: Taylor series with 4 terms for reasonable accuracy
- **Exp**: Taylor series with 6 terms for exponential functions
- **Phase**: Simplified arctangent approximation for phase calculation

### Performance Considerations

- Functions are optimized for accuracy over speed
- Taylor series provide good approximations for most use cases
- For high-precision applications, consider increasing series terms
- Complex operations are decomposed into real arithmetic for efficiency

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/complex_vibe/test_complex_vibe.💀
```

The test suite includes:
- Basic complex number operations
- Trigonometric and exponential functions
- Polar coordinate conversions
- Utility function verification
- Mathematical identity verification
- Edge cases and special values

## Dependencies

- `mathz` - Basic mathematical functions

## Future Enhancements

- **Matrix Operations**: Complex matrix multiplication and determinants
- **Polynomial Operations**: Root finding and polynomial evaluation
- **Vector Operations**: Operations on arrays of complex numbers
- **Series Expansions**: Taylor and Laurent series support
- **Numerical Integration**: Contour integration in the complex plane
- **Enhanced Precision**: Higher-order Taylor series for better accuracy

## License

This module is part of the CURSED standard library and follows the same license terms.
