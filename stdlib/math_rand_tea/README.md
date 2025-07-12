# math_rand_tea Module

The `math_rand_tea` module provides comprehensive pseudorandom number generation functionality for the CURSED programming language. It includes high-quality random number generators, various probability distributions, and utilities for creating random sequences.

## Features

### Core Random Number Generation
- **Global Functions**: Easy-to-use global random number generation functions
- **Custom Generators**: Create your own random number generators with custom seeds
- **Multiple Algorithms**: Support for PCG, Xoshiro256**, and linear congruential generators
- **High-Quality Output**: Statistically sound random number generation

### Supported Distributions
- **Uniform**: Integer and floating-point uniform distributions
- **Normal**: Gaussian distribution using Box-Muller transform
- **Exponential**: Exponential distribution for modeling time between events
- **Poisson**: Poisson distribution for modeling count data
- **Binomial**: Binomial distribution for modeling binary outcomes
- **Gamma**: Gamma distribution for modeling continuous positive values
- **Zipf**: Zipf distribution for modeling frequency distributions

### Advanced Features
- **Secure Random**: Cryptographically secure random number generation
- **Random Sampling**: Sample from collections with and without replacement
- **Weighted Selection**: Random selection with custom probability weights
- **Random Text**: Generate random alphanumeric strings and pattern-based strings
- **Permutations**: Generate random permutations of integers
- **Shuffling**: Fisher-Yates shuffle for randomizing sequences

## Usage Examples

### Basic Random Number Generation

```cursed
yeet "math_rand_tea"

fr fr Generate basic random numbers
vibez.spill("Random integer: %d", math_rand_tea.Int())
vibez.spill("Random float: %f", math_rand_tea.Float64())
vibez.spill("Random integer [0,100): %d", math_rand_tea.Intn(100))
```

### Seeding for Reproducible Results

```cursed
yeet "math_rand_tea"

fr fr Seed for reproducible random sequences
math_rand_tea.Seed(42)
vibez.spill("First: %d", math_rand_tea.Int())
vibez.spill("Second: %d", math_rand_tea.Int())

fr fr Reset with same seed
math_rand_tea.Seed(42)
vibez.spill("First again: %d", math_rand_tea.Int())  fr fr Same as first
```

### Custom Random Generator

```cursed
yeet "math_rand_tea"

fr fr Create a custom generator
source := math_rand_tea.NewSource(123)
rand := math_rand_tea.NewRand(source)

vibez.spill("Custom random: %d", rand.Int())
vibez.spill("Custom float: %f", rand.Float64())
```

### High-Quality Generators

```cursed
yeet "math_rand_tea"

fr fr PCG (Permuted Congruential Generator)
pcg := math_rand_tea.NewPCG(456)
vibez.spill("PCG random: %d", pcg.Int63())

fr fr Xoshiro256** (very fast generator)
xoshiro := math_rand_tea.NewXoshiro256(789)
vibez.spill("Xoshiro256 random: %d", xoshiro.Int63())
```

### Probability Distributions

```cursed
yeet "math_rand_tea"

fr fr Normal distribution (mean=0, std=1)
normal := math_rand_tea.NormFloat64()
vibez.spill("Normal: %f", normal)

fr fr Exponential distribution
exponential := math_rand_tea.ExpFloat64()
vibez.spill("Exponential: %f", exponential)

fr fr Poisson distribution (lambda=3.5)
poisson := math_rand_tea.PoissonFloat64(3.5)
vibez.spill("Poisson: %f", poisson)

fr fr Binomial distribution (n=10, p=0.3)
binomial := math_rand_tea.BinomialInt(10, 0.3)
vibez.spill("Binomial: %d", binomial)

fr fr Gamma distribution (alpha=2.0, beta=1.0)
gamma := math_rand_tea.GammaFloat64(2.0, 1.0)
vibez.spill("Gamma: %f", gamma)
```

### Zipf Distribution

```cursed
yeet "math_rand_tea"

fr fr Create Zipf distribution for modeling frequency data
source := math_rand_tea.NewSource(111)
rand := math_rand_tea.NewRand(source)
zipf := math_rand_tea.NewZipf(rand, 1.5, 1.0, 1000)

fr fr Generate values following Zipf distribution
for i := 0; i < 10; i++ {
    value := zipf.Uint64()
    vibez.spill("Zipf value: %d", value)
}
```

### Random Permutations and Shuffling

```cursed
yeet "math_rand_tea"

fr fr Generate a random permutation
perm := math_rand_tea.Perm(5)
vibez.spill("Permutation: %v", perm)

fr fr Shuffle a slice (conceptual - actual implementation may vary)
fr fr In practice, you'd use the permutation to reorder your data
```

### Random Bytes

```cursed
yeet "math_rand_tea"

fr fr Generate random bytes
bytes := make([]byte, 16)
n, err := math_rand_tea.Read(bytes)
if err == "" {
    vibez.spill("Generated %d random bytes", n)
}

fr fr Or use the convenience function
randomBytes := math_rand_tea.Bytes(32)
vibez.spill("Random bytes length: %d", len(randomBytes))
```

### Text Generation

```cursed
yeet "math_rand_tea"

fr fr Generate random alphanumeric string
id := math_rand_tea.AlphaNumeric(12)
vibez.spill("Random ID: %s", id)

fr fr Generate string following a pattern
email := math_rand_tea.StringPattern("user????@example.com")
vibez.spill("Random email: %s", email)
```

### Sampling and Weighted Selection

```cursed
yeet "math_rand_tea"

fr fr Sample from a collection
items := []tea{"apple", "banana", "cherry", "date", "elderberry"}
samples := math_rand_tea.SampleSlice(items, 3)
vibez.spill("Random samples: %v", samples)

fr fr Weighted random selection
choices := []tea{"rare", "uncommon", "common"}
weights := []meal{0.1, 0.2, 0.7}
selected := math_rand_tea.WeightedChoice(choices, weights)
vibez.spill("Weighted choice: %s", selected)
```

### Secure Random Numbers

```cursed
yeet "math_rand_tea"

fr fr For cryptographic applications
secureRand := math_rand_tea.NewSecureRand()
secureValue := secureRand.Int63()
vibez.spill("Secure random: %d", secureValue)
```

## Function Reference

### Global Functions

| Function | Description |
|----------|-------------|
| `Seed(seed thicc)` | Seed the global random number generator |
| `Int() normie` | Generate a random integer |
| `Intn(n normie) normie` | Generate a random integer in [0, n) |
| `Int31() normie` | Generate a random 31-bit integer |
| `Int31n(n normie) normie` | Generate a random 31-bit integer in [0, n) |
| `Int63() thicc` | Generate a random 63-bit integer |
| `Int63n(n thicc) thicc` | Generate a random 63-bit integer in [0, n) |
| `Uint32() normie` | Generate a random 32-bit unsigned integer |
| `Uint64() thicc` | Generate a random 64-bit unsigned integer |
| `Float32() drip` | Generate a random float32 in [0.0, 1.0) |
| `Float64() meal` | Generate a random float64 in [0.0, 1.0) |
| `Perm(n normie) []normie` | Generate a random permutation of integers |
| `Read(p []byte) (normie, tea)` | Fill byte slice with random data |

### Distribution Functions

| Function | Description |
|----------|-------------|
| `NormFloat64() meal` | Generate from normal distribution |
| `ExpFloat64() meal` | Generate from exponential distribution |
| `PoissonFloat64(lambda meal) meal` | Generate from Poisson distribution |
| `BinomialInt(n normie, p meal) normie` | Generate from binomial distribution |
| `GammaFloat64(alpha meal, beta meal) meal` | Generate from gamma distribution |

### Generator Creation

| Function | Description |
|----------|-------------|
| `NewSource(seed thicc) RandSource` | Create a new random source |
| `NewRand(source RandSource) *Rand` | Create a new random generator |
| `NewPCG(seed thicc) *PCG` | Create a new PCG generator |
| `NewXoshiro256(seed thicc) *Xoshiro256` | Create a new Xoshiro256 generator |
| `NewZipf(r *Rand, s meal, v meal, imax thicc) *Zipf` | Create a Zipf distribution |
| `NewSecureRand() *SecureRand` | Create a secure random generator |

### Utility Functions

| Function | Description |
|----------|-------------|
| `AlphaNumeric(length normie) tea` | Generate random alphanumeric string |
| `StringPattern(pattern tea) tea` | Generate string matching pattern |
| `Bytes(n normie) []byte` | Generate n random bytes |
| `SampleSlice(slice []tea, k normie) []tea` | Sample k items from slice |
| `WeightedChoice(items []tea, weights []meal) tea` | Weighted random selection |

## Implementation Details

### Random Number Generation Algorithms

1. **Linear Congruential Generator (LCG)**: Default algorithm for basic random number generation
2. **PCG (Permuted Congruential Generator)**: High-quality generator with excellent statistical properties
3. **Xoshiro256****: Very fast generator suitable for simulations and games

### Statistical Quality

- All generators pass standard statistical tests for randomness
- Proper implementation of probability distributions
- Careful handling of edge cases and parameter validation
- Sufficient period length for practical applications

### Performance Considerations

- Optimized for common use cases
- Efficient memory usage
- Fast generation for high-throughput applications
- Minimal overhead for basic operations

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/math_rand_tea/test_math_rand_tea.csd
```

The test suite includes:
- Basic random number generation tests
- Seeding and reproducibility verification
- Bounded random number generation
- Distribution testing
- Custom generator functionality
- Text generation and sampling features
- Statistical validation of distributions

## Best Practices

1. **Seeding**: Always seed your random number generators for reproducible results in testing
2. **Generator Selection**: Use PCG or Xoshiro256 for high-quality random numbers
3. **Secure Random**: Use `NewSecureRand()` for cryptographic applications
4. **Distribution Parameters**: Validate distribution parameters to avoid edge cases
5. **Performance**: Consider the performance characteristics of different generators for your use case

## Thread Safety

The global random number generator is **not** thread-safe. For concurrent applications, create separate random number generators for each goroutine or use appropriate synchronization mechanisms.

## Version History

- **v1.0.0**: Initial implementation with basic random number generation
- **v1.1.0**: Added distribution functions and high-quality generators
- **v1.2.0**: Added sampling, text generation, and weighted selection
- **v1.3.0**: Added secure random number generation and enhanced distributions
