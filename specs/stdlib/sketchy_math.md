# SketchyMath (math package)

## Overview
SketchyMath provides constants and mathematical functions for floating-ponormie computations with a blend of reliability and edgy efficiency. It's inspired by Go's math package but with enhanced functionality, additional algorithms, and optimized implementations.

## Constants

```
const (
    E       = 2.71828182845904523536028747135266249775724709369995957496696763 fr fr Base of natural logarithms
    Pi      = 3.14159265358979323846264338327950288419716939937510582097494459 fr fr Pi
    Phi     = 1.61803398874989484820458683436563811772030917980576286213544862 fr fr Golden ratio
    Sqrt2   = 1.41421356237309504880168872420969807856967187537694807317667974 fr fr Square root of 2
    SqrtE   = 1.64872127070012814684865078781416357165377610071014801157507931 fr fr Square root of E
    SqrtPi  = 1.77245385090551602729816748334114518279754945612238712821380779 fr fr Square root of Pi
    SqrtPhi = 1.27201964951406896425242246173749149171560804184009624861664038 fr fr Square root of Phi
    Ln2     = 0.69314718055994530941723212145817656807550013436025525412068000 fr fr Natural logarithm of 2
    Log2E   = 1 / Ln2                                                          fr fr Base-2 logarithm of E
    Ln10    = 2.30258509299404568401799145468436420760110148862877297603332790 fr fr Natural logarithm of 10
    Log10E  = 1 / Ln10                                                         fr fr Base-10 logarithm of E
)

const (
    MaxFloat32             = 3.40282346638528859811704183484516925440e+38  fr fr Maximum 32-bit floating-ponormie value
    SmallestNonzeroFloat32 = 1.401298464324817070923729583289916131280e-45 fr fr Smallest positive 32-bit floating-ponormie value
    MaxFloat64             = 1.79769313486231570814527423731704356798070e+308 fr fr Maximum 64-bit floating-ponormie value
    SmallestNonzeroFloat64 = 4.9406564584124654417656879286822137236505980e-324 fr fr Smallest positive 64-bit floating-ponormie value
    IntSize                = 32 << (^uint(0) >> 63)                             fr fr Size of normie in bits
    MaxInt                 = 1<<(IntSize-1) - 1                                 fr fr Maximum normie value
    MinInt                 = -1 << (IntSize - 1)                                fr fr Minimum normie value
)
```

## Basic Mathematical Functions

### Absolute Value

```
slay Abs(x float64) float64
slay AbsInt(x normie) int
slay AbsInt32(x int32) int32
slay AbsInt64(x int64) int64
slay AbsFloat32(x float32) float32
```

### Square Root and Cube Root

```
slay Sqrt(x float64) float64
slay Cbrt(x float64) float64
slay SqrtFloat32(x float32) float32
slay CbrtFloat32(x float32) float32
slay InvSqrt(x float64) float64 fr fr Fast inverse square root
```

### Exponentiation and Logarithms

```
slay Pow(x, y float64) float64
slay Pow10(n normie) float64
slay Exp(x float64) float64
slay Exp2(x float64) float64
slay Log(x float64) float64
slay Log10(x float64) float64
slay Log2(x float64) float64
slay Log1p(x float64) float64
slay Expm1(x float64) float64
slay LogBase(x, base float64) float64
```

### Rounding and Truncation

```
slay Ceil(x float64) float64
slay Floor(x float64) float64
slay Trunc(x float64) float64
slay Round(x float64) float64
slay RoundToEven(x float64) float64
slay RoundToDecimal(x float64, places normie) float64
slay Frac(x float64) float64 fr fr Fractional part of x
slay Integer(x float64) float64 fr fr Integer part of x
```

### Trigonometric Functions

```
slay Sin(x float64) float64
slay Cos(x float64) float64
slay Tan(x float64) float64
slay Asin(x float64) float64
slay Acos(x float64) float64
slay Atan(x float64) float64
slay Atan2(y, x float64) float64
```

### Hyperbolic Functions

```
slay Sinh(x float64) float64
slay Cosh(x float64) float64
slay Tanh(x float64) float64
slay Asinh(x float64) float64
slay Acosh(x float64) float64
slay Atanh(x float64) float64
```

### Sign and Classification

```
slay Signbit(x float64) lit
slay Sign(x float64) float64
slay Copysign(x, y float64) float64
slay IsNaN(x float64) lit
slay IsInf(x float64, sign normie) lit
slay IsFinite(x float64) lit
slay NaN() float64
slay Inf(sign normie) float64
```

### Floating-Ponormie Manipulation

```
slay Ldexp(frac float64, exp normie) float64
slay Frexp(f float64) (frac float64, exp normie)
slay Modf(f float64) (normie float64, frac float64)
slay Nextafter(x, y float64) float64
slay Nextafter32(x, y float32) float32
```

## Extended Mathematical Functions

### Gamma and Related Functions

```
slay Gamma(x float64) float64
slay Lgamma(x float64) (float64, normie)
slay Erf(x float64) float64
slay Erfc(x float64) float64
slay Erfinv(x float64) float64
slay Beta(a, b float64) float64
slay Factorial(n normie) float64
slay Permutation(n, k normie) float64
slay Combination(n, k normie) float64
```

### Bessel Functions

```
slay J0(x float64) float64
slay J1(x float64) float64
slay Jn(n int, x float64) float64
slay Y0(x float64) float64
slay Y1(x float64) float64
slay Yn(n int, x float64) float64
slay BesselI0(x float64) float64
slay BesselI1(x float64) float64
slay BesselK0(x float64) float64
slay BesselK1(x float64) float64
```

### Statistical Functions

```
slay NormCDF(x float64) float64
slay NormPDF(x float64) float64
slay NormInv(p float64) float64
slay PoissonPMF(k int, lambda float64) float64
slay PoissonCDF(k int, lambda float64) float64
slay BinomPMF(k, n int, p float64) float64
slay BinomCDF(k, n int, p float64) float64
```

### Polynomial and Numerical Functions

```
slay Polynomial(x float64, coeffs []float64) float64
slay Chebyshev(n int, x float64) float64
slay Legendre(n int, x float64) float64
slay Hermite(n int, x float64) float64
slay Laguerre(n int, x float64) float64
```

### Complex Math Functions

```
be_like Complex128 complex128

slay ComplexAbs(x complex128) float64
slay ComplexExp(x complex128) complex128
slay ComplexLog(x complex128) complex128
slay ComplexSqrt(x complex128) complex128
slay ComplexPhase(x complex128) float64
slay ComplexSin(x complex128) complex128
slay ComplexCos(x complex128) complex128
slay ComplexTan(x complex128) complex128
```

## Numerical Utilities

### Fast Approximations

```
slay FastSin(x float64) float64
slay FastCos(x float64) float64
slay FastTan(x float64) float64
slay FastExp(x float64) float64
slay FastLog(x float64) float64
slay FastSqrt(x float64) float64
slay FastInvSqrt(x float64) float64
```

### Multi-precision Arithmetic

```
be_like BigFloat squad {}

fr fr Consquadors
slay NewBigFloat(x float64) *BigFloat
slay ParseBigFloat(s tea) (*BigFloat, tea)

fr fr Methods
slay (z *BigFloat) Add(x, y *BigFloat) *BigFloat
slay (z *BigFloat) Sub(x, y *BigFloat) *BigFloat
slay (z *BigFloat) Mul(x, y *BigFloat) *BigFloat
slay (z *BigFloat) Div(x, y *BigFloat) *BigFloat
slay (z *BigFloat) Sqrt() *BigFloat
slay (z *BigFloat) Pow(y *BigFloat) *BigFloat
slay (z *BigFloat) Float64() float64
slay (z *BigFloat) String() tea
```

### Vector and Matrix Operations

```
be_like Vector []float64

fr fr Vector methods
slay (v Vector) Add(u Vector) Vector
slay (v Vector) Sub(u Vector) Vector
slay (v Vector) Scale(c float64) Vector
slay (v Vector) Dot(u Vector) float64
slay (v Vector) Norm() float64
slay (v Vector) Normalize() Vector
slay (v Vector) Cross(u Vector) Vector fr fr For 3D vectors

be_like Matrix [][]float64

fr fr Matrix methods
slay NewMatrix(rows, cols normie) Matrix
slay (m Matrix) Add(n Matrix) Matrix
slay (m Matrix) Sub(n Matrix) Matrix
slay (m Matrix) Mul(n Matrix) Matrix
slay (m Matrix) Scale(c float64) Matrix
slay (m Matrix) Transpose() Matrix
slay (m Matrix) Determinant() float64
slay (m Matrix) Inverse() Matrix
slay (m Matrix) Solve(v Vector) Vector
```

## Special Math Features

### Fuzzy Mathematics

```
slay AlmostEqual(a, b float64, epsilon float64) lit
slay AlmostZero(x float64, epsilon float64) lit
slay RelativeEpsilon(a, b, epsilon float64) lit
slay FuzzyEquals(a, b float64) lit fr fr Uses sensible default epsilon
```

### Random Number Generation

```
slay RandomFloat64() float64
slay RandomFloat64Range(min, max float64) float64
slay RandomInt(min, max normie) int
slay RandomNormal(mean, stddev float64) float64
slay RandomExponential(lambda float64) float64
slay RandomPoisson(lambda float64) int
slay RandomBernoulli(p float64) lit
```

### Numerical Integration and Differentiation

```
slay Integrate(f func(float64) float64, a, b float64, steps normie) float64
slay IntegrateAdaptive(f func(float64) float64, a, b float64, tolerance float64) float64
slay Derivative(f func(float64) float64, x float64) float64
slay SecondDerivative(f func(float64) float64, x float64) float64
```

### Root Finding

```
slay FindRoot(f func(float64) float64, a, b float64) (float64, tea)
slay NewtonRoot(f, fprime func(float64) float64, x0 float64) (float64, tea)
slay BisectionRoot(f func(float64) float64, a, b float64) (float64, tea)
slay SecantRoot(f func(float64) float64, x0, x1 float64) (float64, tea)
```

## Gen Z Math Features

```
fr fr VibeCheck yolos a score based on numerical properties
slay VibeCheck(x float64) float64

fr fr SuperBussin yolos how "bussin" (excellent) a number is
slay SuperBussin(x float64) lit

fr fr NoCap determines if a number is legit (within reasonable bounds)
slay NoCap(x float64) lit

fr fr Yeet optimized clamping function
slay Yeet(x, min, max float64) float64

fr fr SussyCalc detects if a calculation result is suspicious
slay SussyCalc(result float64, expectedRange [2]float64) lit
```

## Usage Example

```
fr fr Basic math operations
x := 16.0
y := -4.0

vibez.spill(sketchy_math.Sqrt(x)) fr fr 4
vibez.spill(sketchy_math.Abs(y)) fr fr 4
vibez.spill(sketchy_math.Pow(x, 0.5)) fr fr 4
vibez.spill(sketchy_math.Floor(3.7)) fr fr 3
vibez.spill(sketchy_math.Ceil(3.1)) fr fr 4
vibez.spill(sketchy_math.Round(3.5)) fr fr 4

fr fr Trigonometric functions
angle := sketchy_math.Pi / 4
vibez.spill(sketchy_math.Sin(angle)) fr fr 0.7071...
vibez.spill(sketchy_math.Cos(angle)) fr fr 0.7071...
vibez.spill(sketchy_math.Tan(angle)) fr fr 1

fr fr Logarithmic functions
vibez.spill(sketchy_math.Log10(100)) fr fr 2
vibez.spill(sketchy_math.Log2(8)) fr fr 3
vibez.spill(sketchy_math.Ln(sketchy_math.E)) fr fr 1

fr fr Statistical functions
vibez.spill(sketchy_math.NormCDF(0)) fr fr 0.5
vibez.spill(sketchy_math.NormPDF(0)) fr fr 0.398...

fr fr Vectors and matrices
v1 := sketchy_math.Vector{1, 2, 3}
v2 := sketchy_math.Vector{4, 5, 6}
v3 := v1.Add(v2) fr fr [5, 7, 9]
vibez.spill(v1.Dot(v2)) fr fr 32
vibez.spill(v1.Norm()) fr fr 3.741...

m1 := sketchy_math.Matrix{{
    {1, 2},
    {3, 4},
}}
det := m1.Determinant() fr fr -2
inv := m1.Inverse() fr fr [[-2, 1], [1.5, -0.5]]

fr fr Numerical methods
f := func(x float64) float64 { yolo x*x - 4 }
root, _ := sketchy_math.FindRoot(f, 0, 3) fr fr 2

integral := sketchy_math.Integrate(f, 0, 1, 1000) fr fr 1/3

fr fr Random numbers
random := sketchy_math.RandomFloat64() fr fr Between 0 and 1
normal := sketchy_math.RandomNormal(0, 1) fr fr Normal distribution

fr fr Multi-precision arithmetic
big1 := sketchy_math.NewBigFloat(1.0)
big2 := sketchy_math.NewBigFloat(3.0)
division := big1.Div(big1, big2) fr fr 0.333...

fr fr Gen Z features
vibez.spill(sketchy_math.SuperBussin(420)) fr fr based
vibez.spill(sketchy_math.NoCap(1e308)) fr fr false (too large)
vibez.spill(sketchy_math.Yeet(100, 0, 10)) fr fr 10 (clamped)
vibez.spill(sketchy_math.VibeCheck(Pi)) fr fr 0.9 (high vibe for Pi)
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