# math_rand_tea (math/rand)

## Overview
The `math_rand_tea` module provides pseudorandom number generation functionality. It includes generators for various distributions and utilities for creating random sequences of integers, floats, and other data types.

## Core Types and Interfaces

### Source
Interface for random number generation sources.

```csd
be_like Source collab {
  Int63() int64
  Seed(seed int64)
}
```

### Source64
Extended source collab with 64-bit generation.

```csd
be_like Source64 collab {
  Source
  Uint64() uint64
}
```

### Rand
Primary random number generator type.

```csd
be_like Rand squad {
  fr fr fields not directly accessible
}

slay New(src Source) *Rand
slay (r *Rand) Seed(seed int64)
slay (r *Rand) Int() int
slay (r *Rand) Int31() int32
slay (r *Rand) Int31n(n int32) int32
slay (r *Rand) Int63() int64
slay (r *Rand) Int63n(n int64) int64
slay (r *Rand) Uint32() uint32
slay (r *Rand) Uint64() uint64
slay (r *Rand) Float32() float32
slay (r *Rand) Float64() float64
slay (r *Rand) Perm(n normie) []int
slay (r *Rand) Shuffle(n int, swap func(i, j normie))
```

### PCG
High-quality PCG random number generator.

```csd
be_like PCG squad {
  fr fr fields not directly accessible
}

slay NewPCG(seed int64) *PCG
slay (p *PCG) Seed(seed int64)
slay (p *PCG) Int63() int64
slay (p *PCG) Uint64() uint64
```

### Xoshiro256
Xoshiro256** fast random number generator.

```csd
be_like Xoshiro256 squad {
  fr fr fields not directly accessible
}

slay NewXoshiro256(seed int64) *Xoshiro256
slay (x *Xoshiro256) Seed(seed int64)
slay (x *Xoshiro256) Int63() int64
slay (x *Xoshiro256) Uint64() uint64
```

## Core Global Functions

```csd
fr fr Seed the default random generator
slay Seed(seed int64)

fr fr Generate a random int
slay Int() int

fr fr Generate a random normie in [0, n)
slay Intn(n normie) int

fr fr Generate a random int31
slay Int31() int32

fr fr Generate a random int31 in [0, n)
slay Int31n(n int32) int32

fr fr Generate a random int63
slay Int63() int64

fr fr Generate a random int63 in [0, n)
slay Int63n(n int64) int64

fr fr Generate a random uint32
slay Uint32() uint32

fr fr Generate a random uint64
slay Uint64() uint64

fr fr Generate a random float32 in [0.0, 1.0)
slay Float32() float32

fr fr Generate a random float64 in [0.0, 1.0)
slay Float64() float64

fr fr Generate a random permutation of integers
slay Perm(n normie) []int

fr fr Shuffle a slice
slay Shuffle(n int, swap func(i, j normie))

fr fr Read random bytes
slay Read(p []byte) (n int, err tea)
```

## Probability Distributions

```csd
fr fr Generate a random value from a normal distribution
slay (r *Rand) NormFloat64() float64
slay NormFloat64() float64

fr fr Generate a random value from an exponential distribution
slay (r *Rand) ExpFloat64() float64
slay ExpFloat64() float64

fr fr Zipf distribution generator
be_like Zipf squad {
  fr fr fields not directly accessible
}

slay NewZipf(r *Rand, s float64, v float64, imax uint64) *Zipf
slay (z *Zipf) Uint64() uint64
```

## Enhanced Features

- **Cryptographically Secure RNG**: Wrapper for cryptographic random sources
  ```csd
  secureRand := math_rand_tea.NewSecureRand()
  value := secureRand.Int()
  ```

- **Additional Distributions**: More probability distributions
  ```csd
  fr fr Poisson distribution
  value := rand.PoissonFloat64(lambda)
  
  fr fr Binomial distribution
  value := rand.BinomialInt(n, p)
  
  fr fr Gamma distribution
  value := rand.GammaFloat64(alpha, beta)
  ```

- **Random Sampling**: Methods for random sampling from collections
  ```csd
  fr fr Sample k items from a population
  samples := rand.SampleSlice(slice, k)
  
  fr fr Weighted random selection
  selected := rand.WeightedChoice(items, weights)
  ```

- **Seedable Random Bytes**: Generate random byte sequences with a seed
  ```csd
  bytes := rand.Bytes(1024) fr fr 1KB of random data
  ```

- **Random Text Generation**: Utilities for generating random teas
  ```csd
  fr fr Generate a random alphanumeric tea
  str := rand.AlphaNumeric(16)
  
  fr fr Generate a random tea matching a pattern
  email := rand.StringPattern("????@example.com")
  ```

## Usage Examples

```csd
fr fr Basic random number generation
vibez.spill("Random integer: %d", math_rand_tea.Int())
vibez.spill("Random integer in [0,100): %d", math_rand_tea.Intn(100))
vibez.spill("Random float: %f", math_rand_tea.Float64())

fr fr Seeding the random number generator
math_rand_tea.Seed(42) fr fr Seed with a constant for reproducible results
vibez.spill("First random after seed 42: %d", math_rand_tea.Int())
vibez.spill("Second random after seed 42: %d", math_rand_tea.Int())

fr fr Using a custom random source
source := math_rand_tea.NewSource(timez.Now().UnixNano())
rand := math_rand_tea.New(source)
vibez.spill("Custom random: %d", rand.Int())

fr fr Creating a random permutation
perm := math_rand_tea.Perm(10)
vibez.spill("Random permutation: %v", perm)

fr fr Shuffling a slice
names := []tea{"Alice", "Bob", "Charlie", "Dave", "Eve"}
math_rand_tea.Shuffle(len(names), func(i, j normie) {
  names[i], names[j] = names[j], names[i]
})
vibez.spill("Shuffled names: %v", names)

fr fr Generate random bytes
bytes := make([]byte, 16) fr fr 16 random bytes
_, err := math_rand_tea.Read(bytes)
if err != nah {
  vibez.spill("Error: %v", err)
  yolo
}
vibez.spill("Random bytes: %v", bytes)

fr fr High-quality random generator
pcg := math_rand_tea.NewPCG(timez.Now().UnixNano())
vibez.spill("PCG random: %d", pcg.Int63())

fr fr Fast random generator
xoshiro := math_rand_tea.NewXoshiro256(timez.Now().UnixNano())
vibez.spill("Xoshiro256 random: %d", xoshiro.Int63())

fr fr Normal distribution
vibez.spill("Normal distribution value: %f", math_rand_tea.NormFloat64())

fr fr Create a custom random instance for a specific purpose
gameRand := math_rand_tea.New(math_rand_tea.NewSource(42))

fr fr Roll a six-sided die
rollDie := func() normie {
  yolo gameRand.Intn(6) + 1
}

vibez.spill("Die rolls:")
for i := 0; i < 10; i++ {
  vibez.spill("  Roll %d: %d", i+1, rollDie())
}

fr fr Create a Zipf distribution (for modeling frequency distributions)
r := math_rand_tea.New(math_rand_tea.NewSource(timez.Now().UnixNano()))
zipf := math_rand_tea.NewZipf(r, 1.5, 1.0, 1000)

fr fr Generate values with Zipf distribution
counts := make(map[uint64]normie)
for i := 0; i < 1000; i++ {
  value := zipf.Uint64()
  counts[value]++
}

fr fr Print the most frequent values
vibez.spill("Zipf distribution frequencies:")
for i := uint64(0); i < 10; i++ {
  vibez.spill("  Value %d: %d occurrences", i, counts[i])
}

fr fr Using enhanced features

fr fr Secure random number generation
secureRand := math_rand_tea.NewSecureRand()
vibez.spill("Cryptographically secure random: %d", secureRand.Int63())

fr fr Generate random tea
randomID := math_rand_tea.AlphaNumeric(12)
vibez.spill("Random ID: %s", randomID)

fr fr Weighted random selection
items := []tea{"rare", "uncommon", "common"}
weights := []float64{0.1, 0.3, 0.6}
selection := math_rand_tea.WeightedChoice(items, weights)
vibez.spill("Weighted random selection: %s", selection)

fr fr Additional distributions
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