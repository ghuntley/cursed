# Sample Math Utils

A sample mathematical utilities package for CURSED, demonstrating package manager functionality.

## Features

- Basic mathematical operations (factorial, GCD, LCM)
- Prime number checking
- Fibonacci sequence calculation
- Power and square root functions
- Statistical functions (with `statistics` feature)
- Trigonometric approximations (with `trigonometry` feature)

## Installation

```bash
cursed-pkg install sample-math-utils
```

## Usage

```cursed
yeet "sample-math-utils"

// Basic usage
sus result := sample_math_utils.factorial(5.0)
vibez.spill("5! =", result)  // Output: 120.0

// Prime checking
lowkey sample_math_utils.is_prime(17) {
    vibez.spill("17 is prime!")
}

// Fibonacci numbers
bestie i := 0; i < 10; i++ {
    vibez.spill("fib({}): {}", i, sample_math_utils.fibonacci(i))
}

// Square root
sus sqrt_val := sample_math_utils.sqrt(25.0)
vibez.spill("sqrt(25) =", sqrt_val)  // Output: 5.0
```

## Features

### Basic (default)
All basic mathematical functions are included by default.

### Statistics
Enable statistical functions:

```toml
[dependencies]
sample-math-utils = { version = "1.0.0", features = ["statistics"] }
```

```cursed
yeet "sample-math-utils"

sus numbers []drip = [1.0, 2.0, 3.0, 4.0, 5.0]
sus avg := sample_math_utils.mean(numbers)
vibez.spill("Average:", avg)  // Output: 3.0
```

### Trigonometry
Enable trigonometric approximations:

```toml
[dependencies]
sample-math-utils = { version = "1.0.0", features = ["trigonometry"] }
```

```cursed
yeet "sample-math-utils"

sus pi drip = 3.14159265359
sus sin_val := sample_math_utils.sin_approx(pi / 2.0)
sus cos_val := sample_math_utils.cos_approx(0.0)

vibez.spill("sin(π/2) ≈", sin_val)  // Output: ~1.0
vibez.spill("cos(0) ≈", cos_val)    // Output: ~1.0
```

### All Features
Enable all features:

```toml
[dependencies]
sample-math-utils = { version = "1.0.0", features = ["advanced"] }
```

## API Reference

### Basic Functions

- `factorial(n drip) drip` - Calculate factorial of n
- `gcd(a normie, b normie) normie` - Greatest common divisor
- `lcm(a normie, b normie) normie` - Least common multiple
- `is_prime(n normie) lit` - Check if number is prime
- `fibonacci(n normie) normie` - Calculate nth Fibonacci number
- `power(base drip, exponent normie) drip` - Calculate base^exponent
- `sqrt(x drip) drip` - Square root using Newton's method
- `abs(x drip) drip` - Absolute value
- `max(a drip, b drip) drip` - Maximum of two numbers
- `min(a drip, b drip) drip` - Minimum of two numbers
- `round(x drip) normie` - Round to nearest integer

### Statistics Functions (feature: "statistics")

- `mean(numbers []drip) drip` - Calculate arithmetic mean
- `median(numbers []drip) drip` - Calculate median value

### Trigonometry Functions (feature: "trigonometry")

- `sin_approx(x drip) drip` - Sine approximation using Taylor series
- `cos_approx(x drip) drip` - Cosine approximation using Taylor series

## Testing

The package includes comprehensive tests:

```bash
# Run tests
cursed-pkg test

# Run tests with all features
cursed-pkg test --features advanced
```

## Building

```bash
# Build with default features
cursed-pkg build

# Build with specific features
cursed-pkg build --features statistics,trigonometry

# Build with all features
cursed-pkg build --features advanced
```

## Examples

See the `examples/` directory for more usage examples:

- `basic_usage.csd` - Basic mathematical operations
- `advanced_usage.csd` - Using statistics and trigonometry features
- `performance_test.csd` - Performance benchmarking

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This package is licensed under the MIT License. See LICENSE file for details.

## Changelog

### Version 1.0.0
- Initial release
- Basic mathematical functions
- Optional statistics and trigonometry features
- Comprehensive test suite
- Documentation and examples
