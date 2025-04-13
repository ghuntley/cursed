# SketchyMath (math package)

## Overview
SketchyMath provides constants and mathematical functions for floating-point computations with a blend of reliability and edgy efficiency. It's inspired by Go's math package but with enhanced functionality, additional algorithms, and optimized implementations.

## Constants

```go
const (
    E       = 2.71828182845904523536028747135266249775724709369995957496696763 // Base of natural logarithms
    Pi      = 3.14159265358979323846264338327950288419716939937510582097494459 // Pi
    Phi     = 1.61803398874989484820458683436563811772030917980576286213544862 // Golden ratio
    Sqrt2   = 1.41421356237309504880168872420969807856967187537694807317667974 // Square root of 2
    SqrtE   = 1.64872127070012814684865078781416357165377610071014801157507931 // Square root of E
    SqrtPi  = 1.77245385090551602729816748334114518279754945612238712821380779 // Square root of Pi
    SqrtPhi = 1.27201964951406896425242246173749149171560804184009624861664038 // Square root of Phi
    Ln2     = 0.69314718055994530941723212145817656807550013436025525412068000 // Natural logarithm of 2
    Log2E   = 1 / Ln2                                                          // Base-2 logarithm of E
    Ln10    = 2.30258509299404568401799145468436420760110148862877297603332790 // Natural logarithm of 10
    Log10E  = 1 / Ln10                                                         // Base-10 logarithm of E
)

const (
    MaxFloat32             = 3.40282346638528859811704183484516925440e+38  // Maximum 32-bit floating-point value
    SmallestNonzeroFloat32 = 1.401298464324817070923729583289916131280e-45 // Smallest positive 32-bit floating-point value
    MaxFloat64             = 1.79769313486231570814527423731704356798070e+308 // Maximum 64-bit floating-point value
    SmallestNonzeroFloat64 = 4.9406564584124654417656879286822137236505980e-324 // Smallest positive 64-bit floating-point value
    IntSize                = 32 << (^uint(0) >> 63)                             // Size of int in bits
    MaxInt                 = 1<<(IntSize-1) - 1                                 // Maximum int value
    MinInt                 = -1 << (IntSize - 1)                                // Minimum int value
)
```

## Basic Mathematical Functions

### Absolute Value

```go
func Abs(x float64) float64
func AbsInt(x int) int
func AbsInt32(x int32) int32
func AbsInt64(x int64) int64
func AbsFloat32(x float32) float32
```

### Square Root and Cube Root

```go
func Sqrt(x float64) float64
func Cbrt(x float64) float64
func SqrtFloat32(x float32) float32
func CbrtFloat32(x float32) float32
func InvSqrt(x float64) float64 // Fast inverse square root
```

### Exponentiation and Logarithms

```go
func Pow(x, y float64) float64
func Pow10(n int) float64
func Exp(x float64) float64
func Exp2(x float64) float64
func Log(x float64) float64
func Log10(x float64) float64
func Log2(x float64) float64
func Log1p(x float64) float64
func Expm1(x float64) float64
func LogBase(x, base float64) float64
```

### Rounding and Truncation

```go
func Ceil(x float64) float64
func Floor(x float64) float64
func Trunc(x float64) float64
func Round(x float64) float64
func RoundToEven(x float64) float64
func RoundToDecimal(x float64, places int) float64
func Frac(x float64) float64 // Fractional part of x
func Integer(x float64) float64 // Integer part of x
```

### Trigonometric Functions

```go
func Sin(x float64) float64
func Cos(x float64) float64
func Tan(x float64) float64
func Asin(x float64) float64
func Acos(x float64) float64
func Atan(x float64) float64
func Atan2(y, x float64) float64
```

### Hyperbolic Functions

```go
func Sinh(x float64) float64
func Cosh(x float64) float64
func Tanh(x float64) float64
func Asinh(x float64) float64
func Acosh(x float64) float64
func Atanh(x float64) float64
```

### Sign and Classification

```go
func Signbit(x float64) bool
func Sign(x float64) float64
func Copysign(x, y float64) float64
func IsNaN(x float64) bool
func IsInf(x float64, sign int) bool
func IsFinite(x float64) bool
func NaN() float64
func Inf(sign int) float64
```

### Floating-Point Manipulation

```go
func Ldexp(frac float64, exp int) float64
func Frexp(f float64) (frac float64, exp int)
func Modf(f float64) (int float64, frac float64)
func Nextafter(x, y float64) float64
func Nextafter32(x, y float32) float32
```

## Extended Mathematical Functions

### Gamma and Related Functions

```go
func Gamma(x float64) float64
func Lgamma(x float64) (float64, int)
func Erf(x float64) float64
func Erfc(x float64) float64
func Erfinv(x float64) float64
func Beta(a, b float64) float64
func Factorial(n int) float64
func Permutation(n, k int) float64
func Combination(n, k int) float64
```

### Bessel Functions

```go
func J0(x float64) float64
func J1(x float64) float64
func Jn(n int, x float64) float64
func Y0(x float64) float64
func Y1(x float64) float64
func Yn(n int, x float64) float64
func BesselI0(x float64) float64
func BesselI1(x float64) float64
func BesselK0(x float64) float64
func BesselK1(x float64) float64
```

### Statistical Functions

```go
func NormCDF(x float64) float64
func NormPDF(x float64) float64
func NormInv(p float64) float64
func PoissonPMF(k int, lambda float64) float64
func PoissonCDF(k int, lambda float64) float64
func BinomPMF(k, n int, p float64) float64
func BinomCDF(k, n int, p float64) float64
```

### Polynomial and Numerical Functions

```go
func Polynomial(x float64, coeffs []float64) float64
func Chebyshev(n int, x float64) float64
func Legendre(n int, x float64) float64
func Hermite(n int, x float64) float64
func Laguerre(n int, x float64) float64
```

### Complex Math Functions

```go
type Complex128 complex128

func ComplexAbs(x complex128) float64
func ComplexExp(x complex128) complex128
func ComplexLog(x complex128) complex128
func ComplexSqrt(x complex128) complex128
func ComplexPhase(x complex128) float64
func ComplexSin(x complex128) complex128
func ComplexCos(x complex128) complex128
func ComplexTan(x complex128) complex128
```

## Numerical Utilities

### Fast Approximations

```go
func FastSin(x float64) float64
func FastCos(x float64) float64
func FastTan(x float64) float64
func FastExp(x float64) float64
func FastLog(x float64) float64
func FastSqrt(x float64) float64
func FastInvSqrt(x float64) float64
```

### Multi-precision Arithmetic

```go
type BigFloat struct {}

// Constructors
func NewBigFloat(x float64) *BigFloat
func ParseBigFloat(s string) (*BigFloat, error)

// Methods
func (z *BigFloat) Add(x, y *BigFloat) *BigFloat
func (z *BigFloat) Sub(x, y *BigFloat) *BigFloat
func (z *BigFloat) Mul(x, y *BigFloat) *BigFloat
func (z *BigFloat) Div(x, y *BigFloat) *BigFloat
func (z *BigFloat) Sqrt() *BigFloat
func (z *BigFloat) Pow(y *BigFloat) *BigFloat
func (z *BigFloat) Float64() float64
func (z *BigFloat) String() string
```

### Vector and Matrix Operations

```go
type Vector []float64

// Vector methods
func (v Vector) Add(u Vector) Vector
func (v Vector) Sub(u Vector) Vector
func (v Vector) Scale(c float64) Vector
func (v Vector) Dot(u Vector) float64
func (v Vector) Norm() float64
func (v Vector) Normalize() Vector
func (v Vector) Cross(u Vector) Vector // For 3D vectors

type Matrix [][]float64

// Matrix methods
func NewMatrix(rows, cols int) Matrix
func (m Matrix) Add(n Matrix) Matrix
func (m Matrix) Sub(n Matrix) Matrix
func (m Matrix) Mul(n Matrix) Matrix
func (m Matrix) Scale(c float64) Matrix
func (m Matrix) Transpose() Matrix
func (m Matrix) Determinant() float64
func (m Matrix) Inverse() Matrix
func (m Matrix) Solve(v Vector) Vector
```

## Special Math Features

### Fuzzy Mathematics

```go
func AlmostEqual(a, b float64, epsilon float64) bool
func AlmostZero(x float64, epsilon float64) bool
func RelativeEpsilon(a, b, epsilon float64) bool
func FuzzyEquals(a, b float64) bool // Uses sensible default epsilon
```

### Random Number Generation

```go
func RandomFloat64() float64
func RandomFloat64Range(min, max float64) float64
func RandomInt(min, max int) int
func RandomNormal(mean, stddev float64) float64
func RandomExponential(lambda float64) float64
func RandomPoisson(lambda float64) int
func RandomBernoulli(p float64) bool
```

### Numerical Integration and Differentiation

```go
func Integrate(f func(float64) float64, a, b float64, steps int) float64
func IntegrateAdaptive(f func(float64) float64, a, b float64, tolerance float64) float64
func Derivative(f func(float64) float64, x float64) float64
func SecondDerivative(f func(float64) float64, x float64) float64
```

### Root Finding

```go
func FindRoot(f func(float64) float64, a, b float64) (float64, error)
func NewtonRoot(f, fprime func(float64) float64, x0 float64) (float64, error)
func BisectionRoot(f func(float64) float64, a, b float64) (float64, error)
func SecantRoot(f func(float64) float64, x0, x1 float64) (float64, error)
```

## Gen Z Math Features

```go
// VibeCheck returns a score based on numerical properties
func VibeCheck(x float64) float64

// SuperBussin returns how "bussin" (excellent) a number is
func SuperBussin(x float64) bool

// NoCap determines if a number is legit (within reasonable bounds)
func NoCap(x float64) bool

// Yeet optimized clamping function
func Yeet(x, min, max float64) float64

// SussyCalc detects if a calculation result is suspicious
func SussyCalc(result float64, expectedRange [2]float64) bool
```

## Usage Example

```go
// Basic math operations
x := 16.0
y := -4.0

vibez.spill(sketchy_math.Sqrt(x)) // 4
vibez.spill(sketchy_math.Abs(y)) // 4
vibez.spill(sketchy_math.Pow(x, 0.5)) // 4
vibez.spill(sketchy_math.Floor(3.7)) // 3
vibez.spill(sketchy_math.Ceil(3.1)) // 4
vibez.spill(sketchy_math.Round(3.5)) // 4

// Trigonometric functions
angle := sketchy_math.Pi / 4
vibez.spill(sketchy_math.Sin(angle)) // 0.7071...
vibez.spill(sketchy_math.Cos(angle)) // 0.7071...
vibez.spill(sketchy_math.Tan(angle)) // 1

// Logarithmic functions
vibez.spill(sketchy_math.Log10(100)) // 2
vibez.spill(sketchy_math.Log2(8)) // 3
vibez.spill(sketchy_math.Ln(sketchy_math.E)) // 1

// Statistical functions
vibez.spill(sketchy_math.NormCDF(0)) // 0.5
vibez.spill(sketchy_math.NormPDF(0)) // 0.398...

// Vectors and matrices
v1 := sketchy_math.Vector{1, 2, 3}
v2 := sketchy_math.Vector{4, 5, 6}
v3 := v1.Add(v2) // [5, 7, 9]
vibez.spill(v1.Dot(v2)) // 32
vibez.spill(v1.Norm()) // 3.741...

m1 := sketchy_math.Matrix{{
    {1, 2},
    {3, 4},
}}
det := m1.Determinant() // -2
inv := m1.Inverse() // [[-2, 1], [1.5, -0.5]]

// Numerical methods
f := func(x float64) float64 { return x*x - 4 }
root, _ := sketchy_math.FindRoot(f, 0, 3) // 2

integral := sketchy_math.Integrate(f, 0, 1, 1000) // 1/3

// Random numbers
random := sketchy_math.RandomFloat64() // Between 0 and 1
normal := sketchy_math.RandomNormal(0, 1) // Normal distribution

// Multi-precision arithmetic
big1 := sketchy_math.NewBigFloat(1.0)
big2 := sketchy_math.NewBigFloat(3.0)
division := big1.Div(big1, big2) // 0.333...

// Gen Z features
vibez.spill(sketchy_math.SuperBussin(420)) // true
vibez.spill(sketchy_math.NoCap(1e308)) // false (too large)
vibez.spill(sketchy_math.Yeet(100, 0, 10)) // 10 (clamped)
vibez.spill(sketchy_math.VibeCheck(Pi)) // 0.9 (high vibe for Pi)
```

## Implementation Guidelines
1. Optimize core functions for performance
2. Provide accurate implementation of mathematical algorithms
3. Handle edge cases (NaN, Inf, etc.) consistently
4. Use platform-specific optimizations where available
5. Document accuracy and performance characteristics
6. Support both float32 and float64 where appropriate
7. Implement thread-safe random number generation
8. Include numerical stability considerations for all functions