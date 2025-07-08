# Pure CURSED Math Integer Module

A comprehensive big integer and integer mathematical operations library implemented entirely in pure CURSED without any FFI dependencies.

## Features

### Basic Integer Operations
- `abs_int(x)` - Absolute value for integers
- `sign_int(x)` - Sign function (-1, 0, 1)
- `min_int(a, b)`, `max_int(a, b)` - Minimum and maximum
- `clamp_int(x, min, max)` - Clamp value to range

### Power Functions
- `pow_int(base, exp)` - Integer exponentiation (fast binary method)
- `sqrt_int(x)` - Integer square root (Newton's method)

### Number Theory
- `gcd(a, b)` - Greatest Common Divisor (Euclidean algorithm)
- `gcd_extended(a, b)` - Extended Euclidean algorithm
- `lcm(a, b)` - Least Common Multiple
- `euler_totient(n)` - Euler's totient function
- `chinese_remainder(remainders, moduli)` - Chinese remainder theorem

### Combinatorics
- `factorial(n)` - Factorial function
- `combination(n, k)` - Binomial coefficient C(n,k)
- `permutation(n, k)` - Permutation P(n,k)

### Sequences
- `fibonacci(n)` - Fibonacci sequence
- `fibonacci_fast(n)` - Fast Fibonacci using matrix exponentiation

### Prime Numbers
- `is_prime(n)` - Primality test
- `next_prime(n)` - Find next prime number
- `prime_factors(n)` - Prime factorization

### Modular Arithmetic
- `mod_add(a, b, m)` - Modular addition
- `mod_sub(a, b, m)` - Modular subtraction
- `mod_mul(a, b, m)` - Modular multiplication
- `mod_pow(base, exp, m)` - Modular exponentiation
- `mod_inverse(a, m)` - Modular multiplicative inverse

### Bitwise Operations
- `popcount(n)` - Count set bits
- `leading_zeros(n)` - Count leading zeros
- `trailing_zeros(n)` - Count trailing zeros
- `reverse_bits(n)` - Reverse all bits

### Digital Operations
- `digital_root(n)` - Digital root (repeated digit sum)
- `digit_sum(n)` - Sum of digits
- `digit_product(n)` - Product of digits

### Perfect Numbers
- `is_perfect(n)` - Check if number is perfect
- `is_abundant(n)` - Check if number is abundant
- `is_deficient(n)` - Check if number is deficient
- `sum_proper_divisors(n)` - Sum of proper divisors

### Collatz Conjecture
- `collatz_length(n)` - Length of Collatz sequence

### Base Conversion
- `to_base(n, base)` - Convert integer to string in given base
- `from_base(s, base)` - Convert string in given base to integer

## Implementation Details

### Fast Exponentiation
- Binary exponentiation for O(log n) complexity
- Handles negative exponents correctly
- Optimized for large exponents

### Euclidean Algorithm
- Extended Euclidean algorithm for GCD computation
- Returns Bézout coefficients for linear combination
- Handles negative inputs correctly

### Primality Testing
- Trial division algorithm
- Optimized to check only up to sqrt(n)
- Efficient for moderate-sized integers

### Newton's Method
- Integer square root using Newton's method
- Convergence guaranteed for positive integers
- Handles edge cases (negative inputs, zero)

### Matrix Exponentiation
- Fast Fibonacci using matrix exponentiation
- O(log n) complexity for large Fibonacci numbers
- More efficient than recursive approach

## Usage Examples

```cursed
yeet "math_int"

// Basic operations
sus x normie = abs_int(-42);
sus y normie = sqrt_int(100);
sus z normie = pow_int(2, 10);

// Number theory
sus greatest normie = gcd(48, 18);
sus least normie = lcm(12, 8);
sus totient normie = euler_totient(12);

// Combinatorics
sus fact normie = factorial(5);
sus comb normie = combination(10, 3);
sus perm normie = permutation(10, 3);

// Sequences
sus fib normie = fibonacci(20);
sus fib_fast normie = fibonacci_fast(30);

// Prime numbers
sus is_p lit = is_prime(17);
sus next_p normie = next_prime(20);
sus factors [normie] = prime_factors(60);

// Modular arithmetic
sus mod_result normie = mod_pow(2, 10, 1000);
sus inverse normie = mod_inverse(3, 7);

// Bitwise operations
sus bits normie = popcount(255);
sus leading normie = leading_zeros(1024);
sus trailing normie = trailing_zeros(1024);

// Digital operations
sus root normie = digital_root(123);
sus sum normie = digit_sum(456);
sus product normie = digit_product(789);

// Perfect numbers
sus perfect lit = is_perfect(6);
sus abundant lit = is_abundant(12);
sus deficient lit = is_deficient(8);

// Base conversion
sus binary tea = to_base(255, 2);
sus hex tea = to_base(255, 16);
sus decimal normie = from_base("FF", 16);
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/math_int/test_math_int.csd
```

The test suite includes:
- Basic integer operations
- Power functions and edge cases
- GCD and LCM correctness
- Factorial and combinatorial functions
- Fibonacci sequence accuracy
- Prime number algorithms
- Modular arithmetic operations
- Bitwise operation correctness
- Digital operation accuracy
- Perfect number classification
- Collatz sequence computation
- Base conversion round-trip testing

## Performance

- Pure CURSED implementation with no FFI overhead
- Optimized algorithms for best time complexity
- Efficient recursive implementations with tail optimization
- Suitable for production integer computations

## Accuracy

- Exact integer arithmetic (no floating-point errors)
- Proper handling of edge cases and overflow conditions
- Extensive test coverage for correctness
- Consistent behavior across all supported integer ranges

## Limitations

- Integer overflow behavior depends on platform integer size
- Very large integers may cause overflow in some operations
- Some functions have practical limits due to computational complexity
- Base conversion supports bases 2-36
