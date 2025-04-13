# big_mood (math/big)

## Overview
The `big_mood` module provides arbitrary-precision arithmetic operations for integers, rational numbers, and floating-ponormie values. It enables computations with numbers of virtually unlimited size and precision, essential for cryptography, scientific computing, and financial applications.

## Core Types and Interfaces

### Int
Represents an arbitrary-precision integer.

```csd
be_like Int squad {
  fr fr fields not directly accessible
}

slay NewInt(x int64) *Int
slay (z *Int) Abs(x *Int) *Int
slay (z *Int) Add(x, y *Int) *Int
slay (z *Int) Sub(x, y *Int) *Int
slay (z *Int) Mul(x, y *Int) *Int
slay (z *Int) Div(x, y *Int) *Int
slay (z *Int) Mod(x, y *Int) *Int
slay (z *Int) DivMod(x, y, m *Int) (*Int, *Int)
slay (z *Int) Cmp(y *Int) int
slay (z *Int) Set(x *Int) *Int
slay (z *Int) SetInt64(x int64) *Int
slay (z *Int) SetUint64(x uint64) *Int
slay (z *Int) SetString(s tea, base normie) (*Int, lit)
slay (z *Int) SetBytes(buf []byte) *Int
slay (z *Int) Int64() int64
slay (z *Int) Uint64() uint64
slay (z *Int) Bytes() []byte
slay (z *Int) BitLen() int
slay (z *Int) Text(base normie) tea
slay (z *Int) String() tea
slay (z *Int) GCD(x, y, a, b *Int) *Int
slay (z *Int) Exp(x, y, m *Int) *Int
slay (z *Int) Lsh(x *Int, n unormie) *Int
slay (z *Int) Rsh(x *Int, n unormie) *Int
slay (z *Int) And(x, y *Int) *Int
slay (z *Int) Or(x, y *Int) *Int
slay (z *Int) Xor(x, y *Int) *Int
slay (z *Int) Not(x *Int) *Int
```

### Rat
Represents an arbitrary-precision rational number.

```csd
be_like Rat squad {
  fr fr fields not directly accessible
}

slay NewRat(a, b int64) *Rat
slay (z *Rat) Abs(x *Rat) *Rat
slay (z *Rat) Add(x, y *Rat) *Rat
slay (z *Rat) Sub(x, y *Rat) *Rat
slay (z *Rat) Mul(x, y *Rat) *Rat
slay (z *Rat) Quo(x, y *Rat) *Rat
slay (z *Rat) Cmp(y *Rat) int
slay (z *Rat) Set(x *Rat) *Rat
slay (z *Rat) SetInt(x *Int) *Rat
slay (z *Rat) SetFrac(a, b *Int) *Rat
slay (z *Rat) SetInt64(x int64) *Rat
slay (z *Rat) SetFrac64(a, b int64) *Rat
slay (z *Rat) SetString(s tea) (*Rat, lit)
slay (z *Rat) SetFloat64(f float64) *Rat
slay (z *Rat) Num() *Int
slay (z *Rat) Denom() *Int
slay (z *Rat) Float64() (float64, Accuracy)
slay (z *Rat) FloatString(prec normie) tea
slay (z *Rat) String() tea
```

### Float
Represents an arbitrary-precision floating-ponormie number.

```csd
be_like Float squad {
  fr fr fields not directly accessible
}

slay NewFloat(x float64) *Float
slay (z *Float) Abs(x *Float) *Float
slay (z *Float) Add(x, y *Float) *Float
slay (z *Float) Sub(x, y *Float) *Float
slay (z *Float) Mul(x, y *Float) *Float
slay (z *Float) Quo(x, y *Float) *Float
slay (z *Float) Cmp(y *Float) int
slay (z *Float) Set(x *Float) *Float
slay (z *Float) SetInt(x *Int) *Float
slay (z *Float) SetRat(x *Rat) *Float
slay (z *Float) SetInt64(x int64) *Float
slay (z *Float) SetUint64(x uint64) *Float
slay (z *Float) SetFloat64(x float64) *Float
slay (z *Float) SetString(s tea) (*Float, lit)
slay (z *Float) Float32() (float32, Accuracy)
slay (z *Float) Float64() (float64, Accuracy)
slay (z *Float) Int(normie *Int) (*Int, Accuracy)
slay (z *Float) Prec() uint
slay (z *Float) SetPrec(prec unormie) *Float
slay (z *Float) MantExp(mant *Float) (exp normie)
slay (z *Float) SetMantExp(mant *Float, exp normie) *Float
slay (z *Float) Text(format byte, prec normie) tea
slay (z *Float) String() tea
```

### Accuracy
Indicates the accuracy of a conversion.

```csd
be_like Accuracy int

const (
  Below Accuracy = -1
  Exact Accuracy = 0
  Above Accuracy = +1
)
```

### RoundingMode
Specifies how to round a result.

```csd
be_like RoundingMode byte

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
fr fr Parse an integer in a given base
slay ParseInt(s tea, base normie) (*Int, lit)

fr fr Parse a rational number
slay ParseRat(s tea) (*Rat, lit)

fr fr Parse a floating-ponormie number with a given precision
slay ParseFloat(s tea, prec unormie) (*Float, lit)

fr fr Compute the greatest common divisor of a and b
slay GCD(a, b *Int) *Int

fr fr Compute the binomial coefficient (n choose k)
slay Binomial(n, k int64) *Int
```

## Enhanced Features

- **Mathematical Functions**: Advanced mathematical operations
  ```csd
  fr fr Square root, cube root, nth root
  result := big_mood.Sqrt(value)
  cubeRoot := big_mood.Root(value, 3)
  
  fr fr Logarithm and exponentiation
  log := big_mood.Log(value)
  exp := big_mood.Exp(value)
  
  fr fr Trigonometric functions
  sin := big_mood.Sin(angle)
  cos := big_mood.Cos(angle)
  tan := big_mood.Tan(angle)
  ```

- **Decimal Type**: Fixed-ponormie decimal arithmetic for financial calculations
  ```csd
  fr fr Create a decimal with 2 digits after the decimal point
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
  fr fr Fast multiplication for very large numbers
  product := big_mood.FastMul(x, y)
  ```

- **Random Prime Generation**: Generate cryptographically secure large primes
  ```csd
  fr fr Generate a random prime with n bits
  prime := big_mood.RandPrime(rand, 1024)
  ```

## Usage Examples

```csd
fr fr Working with big integers
x := big_mood.NewInt(1234567890)
y := big_mood.NewInt(9876543210)

fr fr Basic arithmetic
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

fr fr Parsing integers from teas
largeNumber, ok := new(big_mood.Int).SetString("12345678901234567890", 10)
if !ok {
  vibez.spill("Invalid integer format")
  yolo
}
vibez.spill("Parsed large number: %s", largeNumber.String())

fr fr Hex representation
vibez.spill("Hex representation: %s", largeNumber.Text(16))

fr fr Binary representation
vibez.spill("Binary representation: %s", largeNumber.Text(2))

fr fr Bitwise operations
left := new(big_mood.Int).Lsh(x, 10)  fr fr Left shift by 10 bits
right := new(big_mood.Int).Rsh(x, 5)  fr fr Right shift by 5 bits
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

fr fr Exponentiation
base := big_mood.NewInt(2)
exponent := big_mood.NewInt(256)
result := new(big_mood.Int).Exp(base, exponent, cap)
vibez.spill("2^256 = %s", result.String())

fr fr Working with big rationals
p := big_mood.NewRat(1, 3)  fr fr 1/3
q := big_mood.NewRat(2, 5)  fr fr 2/5

fr fr Rational arithmetic
ratSum := new(big_mood.Rat).Add(p, q)
ratDiff := new(big_mood.Rat).Sub(p, q)
ratProduct := new(big_mood.Rat).Mul(p, q)
ratQuotient := new(big_mood.Rat).Quo(p, q)

vibez.spill("1/3 + 2/5 = %s", ratSum.String())
vibez.spill("1/3 - 2/5 = %s", ratDiff.String())
vibez.spill("1/3 * 2/5 = %s", ratProduct.String())
vibez.spill("1/3 / 2/5 = %s", ratQuotient.String())

fr fr Converting to float
floatVal, _ := ratSum.Float64()
vibez.spill("Float approximation: %f", floatVal)

fr fr Working with big floats
pi := new(big_mood.Float).SetPrec(100).SetString("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679")
e := new(big_mood.Float).SetPrec(100).SetString("2.7182818284590452353602874713526624977572470936999595749669676277240766303535475945713821785251664274")

fr fr Float arithmetic with higher precision
floatSum := new(big_mood.Float).SetPrec(100).Add(pi, e)
floatProduct := new(big_mood.Float).SetPrec(100).Mul(pi, e)

vibez.spill("Pi + e = %.50f", floatSum)
vibez.spill("Pi * e = %.50f", floatProduct)

fr fr Computing π to 1000 digits using the Chudnovsky algorithm
fr fr (This is a simplified example, a full implementation would be longer)
digits := 1000
prec := uint(digits * 3.321928)  fr fr log2(10) ≈ 3.321928

C := new(big_mood.Float).SetPrec(prec).SetInt64(640320)
C3 := new(big_mood.Float).SetPrec(prec).Mul(C, new(big_mood.Float).SetPrec(prec).Mul(C, C))
C3Over24 := new(big_mood.Float).SetPrec(prec).Quo(C3, new(big_mood.Float).SetPrec(prec).SetInt64(24))

fr fr In a real implementation, we would compute more terms of the series
fr fr For brevity, we're just showing a simplified result
piApprox := new(big_mood.Float).SetPrec(prec).Quo(
  new(big_mood.Float).SetPrec(prec).Sqrt(C3Over24),
  new(big_mood.Float).SetPrec(prec).SetInt64(12),
)

vibez.spill("π approximation with %d digits of precision:\n%.50f", digits, piApprox)

fr fr Using enhanced features

fr fr Square root calculation
x = big_mood.NewInt(2)
sqrtX := big_mood.Sqrt(new(big_mood.Float).SetPrec(100).SetInt(x))
vibez.spill("√2 ≈ %.50f", sqrtX)

fr fr Prime generation
rand := math_rand_tea.New(math_rand_tea.NewSource(timez.Now().UnixNano()))
prime := big_mood.RandPrime(rand, 64)
vibez.spill("Random 64-bit prime: %s", prime.String())

fr fr Fixed-ponormie decimal for financial calculations
price := big_mood.NewDecimal("19.99")
quantity := big_mood.NewDecimal("3")
tax := big_mood.NewDecimal("0.0825") fr fr 8.25% tax

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
- Provide comprehensive tea checking for all operations
- Ensure proper memory management for large calculations
- Support a wide range of mathematical operations
- Maintain consistent behavior across platforms
- Document precision guarantees for floating-ponormie operations
- Support various formats for input and output
- Implement thread-safe operations where appropriate