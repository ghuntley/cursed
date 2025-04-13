# math_rand_tea (math/rand)

## Overview
The `math_rand_tea` module provides pseudorandom number generation functionality. It includes generators for various distributions and utilities for creating random sequences of integers, floats, and other data types.

## Core Types and Interfaces

### Source
Interface for random number generation sources.

```csd
type Source interface {
  Int63() int64
  Seed(seed int64)
}
```

### Source64
Extended source interface with 64-bit generation.

```csd
type Source64 interface {
  Source
  Uint64() uint64
}
```

### Rand
Primary random number generator type.

```csd
type Rand struct {
  // fields not directly accessible
}

func New(src Source) *Rand
func (r *Rand) Seed(seed int64)
func (r *Rand) Int() int
func (r *Rand) Int31() int32
func (r *Rand) Int31n(n int32) int32
func (r *Rand) Int63() int64
func (r *Rand) Int63n(n int64) int64
func (r *Rand) Uint32() uint32
func (r *Rand) Uint64() uint64
func (r *Rand) Float32() float32
func (r *Rand) Float64() float64
func (r *Rand) Perm(n int) []int
func (r *Rand) Shuffle(n int, swap func(i, j int))
```

### PCG
High-quality PCG random number generator.

```csd
type PCG struct {
  // fields not directly accessible
}

func NewPCG(seed int64) *PCG
func (p *PCG) Seed(seed int64)
func (p *PCG) Int63() int64
func (p *PCG) Uint64() uint64
```

### Xoshiro256
Xoshiro256** fast random number generator.

```csd
type Xoshiro256 struct {
  // fields not directly accessible
}

func NewXoshiro256(seed int64) *Xoshiro256
func (x *Xoshiro256) Seed(seed int64)
func (x *Xoshiro256) Int63() int64
func (x *Xoshiro256) Uint64() uint64
```

## Core Global Functions

```csd
// Seed the default random generator
func Seed(seed int64)

// Generate a random int
func Int() int

// Generate a random int in [0, n)
func Intn(n int) int

// Generate a random int31
func Int31() int32

// Generate a random int31 in [0, n)
func Int31n(n int32) int32

// Generate a random int63
func Int63() int64

// Generate a random int63 in [0, n)
func Int63n(n int64) int64

// Generate a random uint32
func Uint32() uint32

// Generate a random uint64
func Uint64() uint64

// Generate a random float32 in [0.0, 1.0)
func Float32() float32

// Generate a random float64 in [0.0, 1.0)
func Float64() float64

// Generate a random permutation of integers
func Perm(n int) []int

// Shuffle a slice
func Shuffle(n int, swap func(i, j int))

// Read random bytes
func Read(p []byte) (n int, err error)
```

## Probability Distributions

```csd
// Generate a random value from a normal distribution
func (r *Rand) NormFloat64() float64
func NormFloat64() float64

// Generate a random value from an exponential distribution
func (r *Rand) ExpFloat64() float64
func ExpFloat64() float64

// Zipf distribution generator
type Zipf struct {
  // fields not directly accessible
}

func NewZipf(r *Rand, s float64, v float64, imax uint64) *Zipf
func (z *Zipf) Uint64() uint64
```

## Enhanced Features

- **Cryptographically Secure RNG**: Wrapper for cryptographic random sources
  ```csd
  secureRand := math_rand_tea.NewSecureRand()
  value := secureRand.Int()
  ```

- **Additional Distributions**: More probability distributions
  ```csd
  // Poisson distribution
  value := rand.PoissonFloat64(lambda)
  
  // Binomial distribution
  value := rand.BinomialInt(n, p)
  
  // Gamma distribution
  value := rand.GammaFloat64(alpha, beta)
  ```

- **Random Sampling**: Methods for random sampling from collections
  ```csd
  // Sample k items from a population
  samples := rand.SampleSlice(slice, k)
  
  // Weighted random selection
  selected := rand.WeightedChoice(items, weights)
  ```

- **Seedable Random Bytes**: Generate random byte sequences with a seed
  ```csd
  bytes := rand.Bytes(1024) // 1KB of random data
  ```

- **Random Text Generation**: Utilities for generating random strings
  ```csd
  // Generate a random alphanumeric string
  str := rand.AlphaNumeric(16)
  
  // Generate a random string matching a pattern
  email := rand.StringPattern("????@example.com")
  ```

## Usage Examples

```csd
// Basic random number generation
vibez.spill("Random integer: %d", math_rand_tea.Int())
vibez.spill("Random integer in [0,100): %d", math_rand_tea.Intn(100))
vibez.spill("Random float: %f", math_rand_tea.Float64())

// Seeding the random number generator
math_rand_tea.Seed(42) // Seed with a constant for reproducible results
vibez.spill("First random after seed 42: %d", math_rand_tea.Int())
vibez.spill("Second random after seed 42: %d", math_rand_tea.Int())

// Using a custom random source
source := math_rand_tea.NewSource(timez.Now().UnixNano())
rand := math_rand_tea.New(source)
vibez.spill("Custom random: %d", rand.Int())

// Creating a random permutation
perm := math_rand_tea.Perm(10)
vibez.spill("Random permutation: %v", perm)

// Shuffling a slice
names := []string{"Alice", "Bob", "Charlie", "Dave", "Eve"}
math_rand_tea.Shuffle(len(names), func(i, j int) {
  names[i], names[j] = names[j], names[i]
})
vibez.spill("Shuffled names: %v", names)

// Generate random bytes
bytes := make([]byte, 16) // 16 random bytes
_, err := math_rand_tea.Read(bytes)
if err != nil {
  vibez.spill("Error: %v", err)
  return
}
vibez.spill("Random bytes: %v", bytes)

// High-quality random generator
pcg := math_rand_tea.NewPCG(timez.Now().UnixNano())
vibez.spill("PCG random: %d", pcg.Int63())

// Fast random generator
xoshiro := math_rand_tea.NewXoshiro256(timez.Now().UnixNano())
vibez.spill("Xoshiro256 random: %d", xoshiro.Int63())

// Normal distribution
vibez.spill("Normal distribution value: %f", math_rand_tea.NormFloat64())

// Create a custom random instance for a specific purpose
gameRand := math_rand_tea.New(math_rand_tea.NewSource(42))

// Roll a six-sided die
rollDie := func() int {
  return gameRand.Intn(6) + 1
}

vibez.spill("Die rolls:")
for i := 0; i < 10; i++ {
  vibez.spill("  Roll %d: %d", i+1, rollDie())
}

// Create a Zipf distribution (for modeling frequency distributions)
r := math_rand_tea.New(math_rand_tea.NewSource(timez.Now().UnixNano()))
zipf := math_rand_tea.NewZipf(r, 1.5, 1.0, 1000)

// Generate values with Zipf distribution
counts := make(map[uint64]int)
for i := 0; i < 1000; i++ {
  value := zipf.Uint64()
  counts[value]++
}

// Print the most frequent values
vibez.spill("Zipf distribution frequencies:")
for i := uint64(0); i < 10; i++ {
  vibez.spill("  Value %d: %d occurrences", i, counts[i])
}

// Using enhanced features

// Secure random number generation
secureRand := math_rand_tea.NewSecureRand()
vibez.spill("Cryptographically secure random: %d", secureRand.Int63())

// Generate random string
randomID := math_rand_tea.AlphaNumeric(12)
vibez.spill("Random ID: %s", randomID)

// Weighted random selection
items := []string{"rare", "uncommon", "common"}
weights := []float64{0.1, 0.3, 0.6}
selection := math_rand_tea.WeightedChoice(items, weights)
vibez.spill("Weighted random selection: %s", selection)

// Additional distributions
poissonValue := math_rand_tea.PoissonFloat64(3.5)
vibez.spill("Poisson distribution (lambda=3.5): %f", poissonValue)

binomialValue := math_rand_tea.BinomialInt(20, 0.25)
vibez.spill("Binomial distribution (n=20, p=0.25): %d", binomialValue)
```

## Implementation Guidelines

- Use high-quality algorithms for random number generation
- Ensure period length is sufficiently large for practical applications
- Make the default global source thread-safe
- Provide fast, non-cryptographic algorithms for general use
- Support seedable generation for reproducible sequences
- Implement correct statistical properties for various distributions
- Optimize performance for common random generation tasks
- Allow custom sources to be plugged in
- Provide escape hatches to cryptographic randomness when needed
- Ensure random number generators pass statistical quality tests