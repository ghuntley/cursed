# MathLib for CURSED

Extended mathematical functions and utilities for the CURSED programming language.

## Features

- **Number Theory**: Prime numbers, GCD, LCM, Euler's totient function
- **Statistics**: Mean, median, variance, standard deviation
- **Combinatorics**: Factorials, combinations, permutations
- **Matrix Operations**: Basic 2D matrix multiplication
- **Special Functions**: Fibonacci, Newton's square root, power functions
- **Optimized**: Includes memoization for expensive calculations

## Installation

```bash
cursed-pkg install mathlib
```

## Usage

```cursed
yeet "mathlib"

slay main() drip {
    # Basic math operations
    vibez.spill("Factorial of 5:", mathlib.factorial(5))
    vibez.spill("10th Fibonacci:", mathlib.fibonacci(10))
    
    # Statistical analysis
    sus data []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    vibez.spill("Mean:", mathlib.mean(data))
    vibez.spill("Standard deviation:", mathlib.standard_deviation(data))
    
    # Prime numbers
    sus primes []drip = mathlib.primes_up_to(50)
    vibez.spill("Primes up to 50:", primes)
    
    # Run demo
    mathlib.demo()
    
    damn 0
}
```

## API Reference

### Number Theory

- `factorial(n: drip) -> drip` - Calculate factorial with memoization
- `gcd(a: drip, b: drip) -> drip` - Greatest common divisor
- `lcm(a: drip, b: drip) -> drip` - Least common multiple
- `is_prime(n: drip) -> lit` - Check if number is prime
- `primes_up_to(limit: drip) -> []drip` - Generate prime numbers
- `euler_totient(n: drip) -> drip` - Euler's totient function

### Statistics

- `mean(values: []drip) -> drip` - Arithmetic mean
- `median(values: []drip) -> drip` - Median value
- `variance(values: []drip) -> drip` - Sample variance
- `standard_deviation(values: []drip) -> drip` - Standard deviation

### Combinatorics

- `combination(n: drip, k: drip) -> drip` - n choose k
- `permutation(n: drip, k: drip) -> drip` - n permute k
- `fibonacci(n: drip) -> drip` - nth Fibonacci number

### Advanced Functions

- `power(base: drip, exponent: drip) -> drip` - Integer exponentiation
- `sqrt_newton(x: drip) -> drip` - Square root using Newton's method
- `log2_int(n: drip) -> drip` - Integer base-2 logarithm
- `matrix_multiply(a: [][]drip, b: [][]drip) -> [][]drip` - Matrix multiplication

### Constants

- `PI_EXTENDED` - π with extended precision
- `E_EXTENDED` - e with extended precision
- `GOLDEN_RATIO` - φ (golden ratio)
- `SQRT_2` - √2

## Examples

### Statistical Analysis

```cursed
yeet "mathlib"

sus test_scores []drip = [85, 92, 78, 96, 88, 91, 84, 89, 93, 87]

vibez.spill("Test Score Analysis:")
vibez.spill("Mean score:", mathlib.mean(test_scores))
vibez.spill("Median score:", mathlib.median(test_scores))
vibez.spill("Standard deviation:", mathlib.standard_deviation(test_scores))
```

### Prime Number Generation

```cursed
yeet "mathlib"

sus limit drip = 100
sus primes []drip = mathlib.primes_up_to(limit)

vibez.spill("Found", arrayz.len(primes), "prime numbers up to", limit)
vibez.spill("First 10 primes:", arrayz.slice(primes, 0, 10))
```

### Matrix Operations

```cursed
yeet "mathlib"

sus matrix_a [][]drip = [[1, 2], [3, 4]]
sus matrix_b [][]drip = [[5, 6], [7, 8]]

sus result [][]drip = mathlib.matrix_multiply(matrix_a, matrix_b)
vibez.spill("Matrix multiplication result:", result)
```

## Performance

MathLib is optimized for performance:

- Factorial calculation uses memoization
- Prime checking uses optimized trial division
- Statistical functions use single-pass algorithms where possible
- Matrix operations use standard efficient algorithms

## Testing

Run the test suite:

```bash
cursed test/mathlib_test.csd
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Changelog

### Version 1.2.0
- Added matrix multiplication
- Improved performance with memoization
- Added Euler's totient function
- Enhanced statistical functions

### Version 1.1.0
- Added statistical functions
- Newton's method square root
- Improved prime number generation

### Version 1.0.0
- Initial release
- Basic number theory functions
- Factorial and Fibonacci
