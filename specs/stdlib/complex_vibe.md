# complex_vibe (math/cmplx)

## Overview
The `complex_vibe` module provides mathematical functions for complex numbers. It includes operations like trigonometric functions, logarithms, exponentials, and other mathematical operations specifically for complex values.

## Core Types

### Complex128
Represents a complex number with float64 real and imaginary parts (built-in type).

```csd
type complex128 complex128
```

### Complex64
Represents a complex number with float32 real and imaginary parts (built-in type).

```csd
type complex64 complex64
```

## Core Functions

### Complex Number Creation and Conversion

```csd
// Create a complex number from real and imaginary parts
func Complex(r, i float64) complex128

// Extract real part of a complex number
func Real(x complex128) float64

// Extract imaginary part of a complex number
func Imag(x complex128) float64

// Convert complex number to polar coordinates (r, θ)
func Polar(x complex128) (r, θ float64)

// Create complex number from polar coordinates (r, θ)
func Rect(r, θ float64) complex128
```

### Basic Operations

```csd
// Compute absolute value (modulus) of a complex number
func Abs(x complex128) float64

// Compute phase (argument) of a complex number
func Phase(x complex128) float64

// Compute complex conjugate
func Conj(x complex128) complex128
```

### Exponential and Logarithmic Functions

```csd
// Compute e^x
func Exp(x complex128) complex128

// Compute natural logarithm of x
func Log(x complex128) complex128

// Compute base-10 logarithm of x
func Log10(x complex128) complex128

// Compute x^y
func Pow(x, y complex128) complex128

// Compute square root of x
func Sqrt(x complex128) complex128
```

### Trigonometric Functions

```csd
// Compute sine of x
func Sin(x complex128) complex128

// Compute cosine of x
func Cos(x complex128) complex128

// Compute tangent of x
func Tan(x complex128) complex128

// Compute hyperbolic sine of x
func Sinh(x complex128) complex128

// Compute hyperbolic cosine of x
func Cosh(x complex128) complex128

// Compute hyperbolic tangent of x
func Tanh(x complex128) complex128
```

### Inverse Trigonometric Functions

```csd
// Compute inverse sine of x
func Asin(x complex128) complex128

// Compute inverse cosine of x
func Acos(x complex128) complex128

// Compute inverse tangent of x
func Atan(x complex128) complex128

// Compute inverse hyperbolic sine of x
func Asinh(x complex128) complex128

// Compute inverse hyperbolic cosine of x
func Acosh(x complex128) complex128

// Compute inverse hyperbolic tangent of x
func Atanh(x complex128) complex128
```

## Enhanced Features

- **Complex Vector Operations**: Operations on vectors of complex numbers
  ```csd
  vecSum := complex_vibe.VectorSum(complexVector)
  vecProduct := complex_vibe.VectorProduct(complexVector)
  ```

- **Complex Matrix Operations**: Operations on matrices of complex numbers
  ```csd
  matrixProduct := complex_vibe.MatrixMul(matrixA, matrixB)
  determinant := complex_vibe.Determinant(matrix)
  ```

- **Complex Polynomial Functions**: Operations with complex polynomials
  ```csd
  roots := complex_vibe.FindRoots(coefficients)
  polyValue := complex_vibe.EvaluatePolynomial(coefficients, x)
  ```

- **Complex Series Expansions**: Taylor and Laurent series expansions
  ```csd
  taylorCoeffs := complex_vibe.TaylorSeries(f, z0, order)
  laurentCoeffs := complex_vibe.LaurentSeries(f, z0, order)
  ```

- **Complex Numerical Integration**: Integration in the complex plane
  ```csd
  result := complex_vibe.ContourIntegral(f, contour)
  ```

## Usage Examples

```csd
// Creating complex numbers
z1 := complex(3, 4) // 3 + 4i
z2 := complex(1, 2) // 1 + 2i

vibez.spill("z1 = %v", z1)
vibez.spill("z2 = %v", z2)

// Accessing real and imaginary parts
vibez.spill("Re(z1) = %v", complex_vibe.Real(z1))
vibez.spill("Im(z1) = %v", complex_vibe.Imag(z1))

// Basic operations (built-in)
sum := z1 + z2
difference := z1 - z2
product := z1 * z2
quotient := z1 / z2

vibez.spill("z1 + z2 = %v", sum)
vibez.spill("z1 - z2 = %v", difference)
vibez.spill("z1 * z2 = %v", product)
vibez.spill("z1 / z2 = %v", quotient)

// Absolute value and phase
absZ1 := complex_vibe.Abs(z1)
phaseZ1 := complex_vibe.Phase(z1)

vibez.spill("|z1| = %v", absZ1)
vibez.spill("arg(z1) = %v radians", phaseZ1)
vibez.spill("arg(z1) = %v degrees", phaseZ1 * 180 / mathz.Pi)

// Complex conjugate
conjZ1 := complex_vibe.Conj(z1)
vibez.spill("Conjugate of z1 = %v", conjZ1)

// Polar form
r, theta := complex_vibe.Polar(z1)
vibez.spill("z1 in polar form: %v ∠ %v radians", r, theta)

// Construct from polar form
z3 := complex_vibe.Rect(2, mathz.Pi/4)
vibez.spill("2 ∠ π/4 = %v", z3)

// Exponential and logarithm
expZ1 := complex_vibe.Exp(z1)
vibez.spill("e^z1 = %v", expZ1)

logZ1 := complex_vibe.Log(z1)
vibez.spill("ln(z1) = %v", logZ1)

log10Z1 := complex_vibe.Log10(z1)
vibez.spill("log10(z1) = %v", log10Z1)

// Power and square root
powZ1 := complex_vibe.Pow(z1, z2)
vibez.spill("z1^z2 = %v", powZ1)

sqrtZ1 := complex_vibe.Sqrt(z1)
vibez.spill("√z1 = %v", sqrtZ1)

// Verify sqrt
vibez.spill("sqrtZ1^2 = %v", sqrtZ1*sqrtZ1)

// Trigonometric functions
sinZ1 := complex_vibe.Sin(z1)
cosZ1 := complex_vibe.Cos(z1)
tanZ1 := complex_vibe.Tan(z1)

vibez.spill("sin(z1) = %v", sinZ1)
vibez.spill("cos(z1) = %v", cosZ1)
vibez.spill("tan(z1) = %v", tanZ1)

// Verify identity: sin²(z) + cos²(z) = 1
identity := sinZ1*sinZ1 + cosZ1*cosZ1
vibez.spill("sin²(z1) + cos²(z1) = %v", identity)

// Hyperbolic functions
sinhZ1 := complex_vibe.Sinh(z1)
coshZ1 := complex_vibe.Cosh(z1)
tanhZ1 := complex_vibe.Tanh(z1)

vibez.spill("sinh(z1) = %v", sinhZ1)
vibez.spill("cosh(z1) = %v", coshZ1)
vibez.spill("tanh(z1) = %v", tanhZ1)

// Inverse trigonometric functions
asinZ2 := complex_vibe.Asin(z2)
acosZ2 := complex_vibe.Acos(z2)
atanZ2 := complex_vibe.Atan(z2)

vibez.spill("asin(z2) = %v", asinZ2)
vibez.spill("acos(z2) = %v", acosZ2)
vibez.spill("atan(z2) = %v", atanZ2)

// Verify composition: sin(asin(z)) should be close to z
sinAsinZ2 := complex_vibe.Sin(asinZ2)
vibez.spill("sin(asin(z2)) = %v", sinAsinZ2)

// Inverse hyperbolic functions
asinhZ2 := complex_vibe.Asinh(z2)
acoshZ2 := complex_vibe.Acosh(z2)
atanhZ2 := complex_vibe.Atanh(z2)

vibez.spill("asinh(z2) = %v", asinhZ2)
vibez.spill("acosh(z2) = %v", acoshZ2)
vibez.spill("atanh(z2) = %v", atanhZ2)

// Working with complex vectors
complexVector := []complex128{z1, z2, complex(2, 3), complex(0, 1)}

// Sum of vector elements
vectorSum := complex_vibe.VectorSum(complexVector)
vibez.spill("Vector sum: %v", vectorSum)

// Product of vector elements
vectorProduct := complex_vibe.VectorProduct(complexVector)
vibez.spill("Vector product: %v", vectorProduct)

// Complex polynomial roots
// Solve z^3 - 1 = 0 (cube roots of unity)
coefficients := []complex128{complex(-1, 0), complex(0, 0), complex(0, 0), complex(1, 0)}
roots := complex_vibe.FindRoots(coefficients)

vibez.spill("Cube roots of unity:")
for i, root := range roots {
  vibez.spill("  root %d: %v", i+1, root)
  // Verify each root
  cube := complex_vibe.Pow(root, complex(3, 0))
  vibez.spill("  %v^3 = %v", root, cube)
}

// Complex matrix operations
matrixA := [][]complex128{
  {complex(1, 0), complex(2, 1)},
  {complex(0, 1), complex(3, 0)},
}

matrixB := [][]complex128{
  {complex(3, 0), complex(1, 1)},
  {complex(2, 1), complex(0, 2)},
}

// Matrix multiplication
matrixC := complex_vibe.MatrixMul(matrixA, matrixB)

vibez.spill("Matrix A:")
for _, row := range matrixA {
  vibez.spill("  %v", row)
}

vibez.spill("Matrix B:")
for _, row := range matrixB {
  vibez.spill("  %v", row)
}

vibez.spill("Matrix C = A × B:")
for _, row := range matrixC {
  vibez.spill("  %v", row)
}

// Determinant of a complex matrix
detA := complex_vibe.Determinant(matrixA)
vibez.spill("det(A) = %v", detA)

// Taylor series of e^z around z=0 (first 5 terms)
f := func(z complex128) complex128 { return complex_vibe.Exp(z) }
taylorCoeffs := complex_vibe.TaylorSeries(f, complex(0, 0), 5)

vibez.spill("Taylor series of e^z around z=0:")
for i, coeff := range taylorCoeffs {
  vibez.spill("  z^%d coefficient: %v", i, coeff)
}
```

## Implementation Guidelines

- Implement complex functions with high numerical stability
- Use appropriate branch cuts for multivalued functions
- Ensure consistency with real-valued functions when input is purely real
- Handle special values correctly (infinities, NaN)
- Optimize performance for frequently used functions
- Maintain accuracy across the complex plane
- Implement proper error bounds and numerical precision
- Provide consistent results across platforms
- Ensure that all mathematical identities are preserved when possible
- Handle edge cases and singularities gracefully