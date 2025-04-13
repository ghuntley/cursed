# big_mood (math/big)

## Overview
The `big_mood` module provides arbitrary-precision arithmetic operations for integers, rational numbers, and floating-point values. It enables computations with numbers of virtually unlimited size and precision, essential for cryptography, scientific computing, and financial applications.

## Core Types and Interfaces

### Int
Represents an arbitrary-precision integer.

```csd
type Int struct {
  // fields not directly accessible
}

func NewInt(x int64) *Int
func (z *Int) Abs(x *Int) *Int
func (z *Int) Add(x, y *Int) *Int
func (z *Int) Sub(x, y *Int) *Int
func (z *Int) Mul(x, y *Int) *Int
func (z *Int) Div(x, y *Int) *Int
func (z *Int) Mod(x, y *Int) *Int
func (z *Int) DivMod(x, y, m *Int) (*Int, *Int)
func (z *Int) Cmp(y *Int) int
func (z *Int) Set(x *Int) *Int
func (z *Int) SetInt64(x int64) *Int
func (z *Int) SetUint64(x uint64) *Int
func (z *Int) SetString(s string, base int) (*Int, bool)
func (z *Int) SetBytes(buf []byte) *Int
func (z *Int) Int64() int64
func (z *Int) Uint64() uint64
func (z *Int) Bytes() []byte
func (z *Int) BitLen() int
func (z *Int) Text(base int) string
func (z *Int) String() string
func (z *Int) GCD(x, y, a, b *Int) *Int
func (z *Int) Exp(x, y, m *Int) *Int
func (z *Int) Lsh(x *Int, n uint) *Int
func (z *Int) Rsh(x *Int, n uint) *Int
func (z *Int) And(x, y *Int) *Int
func (z *Int) Or(x, y *Int) *Int
func (z *Int) Xor(x, y *Int) *Int
func (z *Int) Not(x *Int) *Int
```

### Rat
Represents an arbitrary-precision rational number.

```csd
type Rat struct {
  // fields not directly accessible
}

func NewRat(a, b int64) *Rat
func (z *Rat) Abs(x *Rat) *Rat
func (z *Rat) Add(x, y *Rat) *Rat
func (z *Rat) Sub(x, y *Rat) *Rat
func (z *Rat) Mul(x, y *Rat) *Rat
func (z *Rat) Quo(x, y *Rat) *Rat
func (z *Rat) Cmp(y *Rat) int
func (z *Rat) Set(x *Rat) *Rat
func (z *Rat) SetInt(x *Int) *Rat
func (z *Rat) SetFrac(a, b *Int) *Rat
func (z *Rat) SetInt64(x int64) *Rat
func (z *Rat) SetFrac64(a, b int64) *Rat
func (z *Rat) SetString(s string) (*Rat, bool)
func (z *Rat) SetFloat64(f float64) *Rat
func (z *Rat) Num() *Int
func (z *Rat) Denom() *Int
func (z *Rat) Float64() (float64, Accuracy)
func (z *Rat) FloatString(prec int) string
func (z *Rat) String() string
```

### Float
Represents an arbitrary-precision floating-point number.

```csd
type Float struct {
  // fields not directly accessible
}

func NewFloat(x float64) *Float
func (z *Float) Abs(x *Float) *Float
func (z *Float) Add(x, y *Float) *Float
func (z *Float) Sub(x, y *Float) *Float
func (z *Float) Mul(x, y *Float) *Float
func (z *Float) Quo(x, y *Float) *Float
func (z *Float) Cmp(y *Float) int
func (z *Float) Set(x *Float) *Float
func (z *Float) SetInt(x *Int) *Float
func (z *Float) SetRat(x *Rat) *Float
func (z *Float) SetInt64(x int64) *Float
func (z *Float) SetUint64(x uint64) *Float
func (z *Float) SetFloat64(x float64) *Float
func (z *Float) SetString(s string) (*Float, bool)
func (z *Float) Float32() (float32, Accuracy)
func (z *Float) Float64() (float64, Accuracy)
func (z *Float) Int(int *Int) (*Int, Accuracy)
func (z *Float) Prec() uint
func (z *Float) SetPrec(prec uint) *Float
func (z *Float) MantExp(mant *Float) (exp int)
func (z *Float) SetMantExp(mant *Float, exp int) *Float
func (z *Float) Text(format byte, prec int) string
func (z *Float) String() string
```

### Accuracy
Indicates the accuracy of a conversion.

```csd
type Accuracy int

const (
  Below Accuracy = -1
  Exact Accuracy = 0
  Above Accuracy = +1
)
```

### RoundingMode
Specifies how to round a result.

```csd
type RoundingMode byte

const (
  ToNearestEven RoundingMode = iota
  ToNearestAway
  ToZero
  AwayFromZero
  ToNegativeInf
  ToPositiveInf
)
```

## Core Functions

```csd
// Parse an integer in a given base
func ParseInt(s string, base int) (*Int, bool)

// Parse a rational number
func ParseRat(s string) (*Rat, bool)

// Parse a floating-point number with a given precision
func ParseFloat(s string, prec uint) (*Float, bool)

// Compute the greatest common divisor of a and b
func GCD(a, b *Int) *Int

// Compute the binomial coefficient (n choose k)
func Binomial(n, k int64) *Int
```

## Enhanced Features

- **Mathematical Functions**: Advanced mathematical operations
  ```csd
  // Square root, cube root, nth root
  result := big_mood.Sqrt(value)
  cubeRoot := big_mood.Root(value, 3)
  
  // Logarithm and exponentiation
  log := big_mood.Log(value)
  exp := big_mood.Exp(value)
  
  // Trigonometric functions
  sin := big_mood.Sin(angle)
  cos := big_mood.Cos(angle)
  tan := big_mood.Tan(angle)
  ```

- **Decimal Type**: Fixed-point decimal arithmetic for financial calculations
  ```csd
  // Create a decimal with 2 digits after the decimal point
  amount := big_mood.NewDecimal("123.45")
  tax := big_mood.NewDecimal("9.99")
  total := amount.Add(tax)
  ```

- **Complex Numbers**: Arbitrary-precision complex number support
  ```csd
  c1 := big_mood.NewComplex(real1, imag1)
  c2 := big_mood.NewComplex(real2, imag2)
  result := c1.Mul(c2)
  ```

- **Performance Optimizations**: Specialized algorithms for large numbers
  ```csd
  // Fast multiplication for very large numbers
  product := big_mood.FastMul(x, y)
  ```

- **Random Prime Generation**: Generate cryptographically secure large primes
  ```csd
  // Generate a random prime with n bits
  prime := big_mood.RandPrime(rand, 1024)
  ```

## Usage Examples

```csd
// Working with big integers
x := big_mood.NewInt(1234567890)
y := big_mood.NewInt(9876543210)

// Basic arithmetic
sum := new(big_mood.Int).Add(x, y)
diff := new(big_mood.Int).Sub(x, y)
product := new(big_mood.Int).Mul(x, y)
quotient := new(big_mood.Int).Div(x, y)
remainder := new(big_mood.Int).Mod(x, y)

vibez.spill("Sum: %s", sum.String())
vibez.spill("Difference: %s", diff.String())
vibez.spill("Product: %s", product.String())
vibez.spill("Quotient: %s", quotient.String())
vibez.spill("Remainder: %s", remainder.String())

// Parsing integers from strings
largeNumber, ok := new(big_mood.Int).SetString("12345678901234567890", 10)
if !ok {
  vibez.spill("Invalid integer format")
  return
}
vibez.spill("Parsed large number: %s", largeNumber.String())

// Hex representation
vibez.spill("Hex representation: %s", largeNumber.Text(16))

// Binary representation
vibez.spill("Binary representation: %s", largeNumber.Text(2))

// Bitwise operations
left := new(big_mood.Int).Lsh(x, 10)  // Left shift by 10 bits
right := new(big_mood.Int).Rsh(x, 5)  // Right shift by 5 bits
bitwiseAnd := new(big_mood.Int).And(x, y)
bitwiseOr := new(big_mood.Int).Or(x, y)
bitwiseXor := new(big_mood.Int).Xor(x, y)
bitwiseNot := new(big_mood.Int).Not(x)

vibez.spill("Left shift: %s", left.String())
vibez.spill("Right shift: %s", right.String())
vibez.spill("Bitwise AND: %s", bitwiseAnd.String())
vibez.spill("Bitwise OR: %s", bitwiseOr.String())
vibez.spill("Bitwise XOR: %s", bitwiseXor.String())
vibez.spill("Bitwise NOT: %s", bitwiseNot.String())

// Exponentiation
base := big_mood.NewInt(2)
exponent := big_mood.NewInt(256)
result := new(big_mood.Int).Exp(base, exponent, nil)
vibez.spill("2^256 = %s", result.String())

// Working with big rationals
p := big_mood.NewRat(1, 3)  // 1/3
q := big_mood.NewRat(2, 5)  // 2/5

// Rational arithmetic
ratSum := new(big_mood.Rat).Add(p, q)
ratDiff := new(big_mood.Rat).Sub(p, q)
ratProduct := new(big_mood.Rat).Mul(p, q)
ratQuotient := new(big_mood.Rat).Quo(p, q)

vibez.spill("1/3 + 2/5 = %s", ratSum.String())
vibez.spill("1/3 - 2/5 = %s", ratDiff.String())
vibez.spill("1/3 * 2/5 = %s", ratProduct.String())
vibez.spill("1/3 / 2/5 = %s", ratQuotient.String())

// Converting to float
floatVal, _ := ratSum.Float64()
vibez.spill("Float approximation: %f", floatVal)

// Working with big floats
pi := new(big_mood.Float).SetPrec(100).SetString("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679")
e := new(big_mood.Float).SetPrec(100).SetString("2.7182818284590452353602874713526624977572470936999595749669676277240766303535475945713821785251664274")

// Float arithmetic with higher precision
floatSum := new(big_mood.Float).SetPrec(100).Add(pi, e)
floatProduct := new(big_mood.Float).SetPrec(100).Mul(pi, e)

vibez.spill("Pi + e = %.50f", floatSum)
vibez.spill("Pi * e = %.50f", floatProduct)

// Computing π to 1000 digits using the Chudnovsky algorithm
// (This is a simplified example, a full implementation would be longer)
digits := 1000
prec := uint(digits * 3.321928)  // log2(10) ≈ 3.321928

C := new(big_mood.Float).SetPrec(prec).SetInt64(640320)
C3 := new(big_mood.Float).SetPrec(prec).Mul(C, new(big_mood.Float).SetPrec(prec).Mul(C, C))
C3Over24 := new(big_mood.Float).SetPrec(prec).Quo(C3, new(big_mood.Float).SetPrec(prec).SetInt64(24))

// In a real implementation, we would compute more terms of the series
// For brevity, we're just showing a simplified result
piApprox := new(big_mood.Float).SetPrec(prec).Quo(
  new(big_mood.Float).SetPrec(prec).Sqrt(C3Over24),
  new(big_mood.Float).SetPrec(prec).SetInt64(12),
)

vibez.spill("π approximation with %d digits of precision:\n%.50f", digits, piApprox)

// Using enhanced features

// Square root calculation
x = big_mood.NewInt(2)
sqrtX := big_mood.Sqrt(new(big_mood.Float).SetPrec(100).SetInt(x))
vibez.spill("√2 ≈ %.50f", sqrtX)

// Prime generation
rand := math_rand_tea.New(math_rand_tea.NewSource(timez.Now().UnixNano()))
prime := big_mood.RandPrime(rand, 64)
vibez.spill("Random 64-bit prime: %s", prime.String())

// Fixed-point decimal for financial calculations
price := big_mood.NewDecimal("19.99")
quantity := big_mood.NewDecimal("3")
tax := big_mood.NewDecimal("0.0825") // 8.25% tax

subtotal := price.Mul(quantity)
taxAmount := subtotal.Mul(tax)
total := subtotal.Add(taxAmount)

vibez.spill("Price: $%s", price.String())
vibez.spill("Quantity: %s", quantity.String())
vibez.spill("Subtotal: $%s", subtotal.String())
vibez.spill("Tax (8.25%%): $%s", taxAmount.String())
vibez.spill("Total: $%s", total.String())
```

## Implementation Guidelines

- Prioritize correctness and numerical stability over performance
- Implement algorithms that scale well for very large numbers
- Use optimized algorithms for different number size ranges
- Provide comprehensive error checking for all operations
- Ensure proper memory management for large calculations
- Support a wide range of mathematical operations
- Maintain consistent behavior across platforms
- Document precision guarantees for floating-point operations
- Support various formats for input and output
- Implement thread-safe operations where appropriate