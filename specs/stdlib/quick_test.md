# quick_test (testing/quick)

## Overview
The `quick_test` module provides support for property-based testing, allowing developers to test code against a wide range of automatically generated inputs. This approach helps discover edge cases and bugs that might be missed by traditional unit tests.

## Core Types and Interfaces

### Config
Configuration for how a test is run.

```csd
type Config struct {
  MaxCount         int     // Maximum number of test iterations
  MaxSize          int     // Maximum size of generated values
  MinSize          int     // Minimum size of generated values
  Rand             *math_rand_tea.Rand // Random source for testing
  Values           []interface{}   // Values generated and tested
  ExpectFailure    bool           // Whether the test should fail
  MaxFailures      int            // Maximum failures before stopping
  MaxShrinkCount   int            // Maximum number of shrink iterations
  ShrinkStrategy   ShrinkStrategy // Strategy for value shrinking
  MaxShrinkTime    timez.Duration // Maximum time spent shrinking
  Quiet            bool           // Do not log failure details
  ShrinkPreserveOut bool          // Preserve output during shrinking
}
```

### Generator
Interface for generating test data.

```csd
type Generator interface {
  Generate(rand *math_rand_tea.Rand, size int) interface{}
}
```

### GeneratorFunc
Function type that implements Generator.

```csd
type GeneratorFunc func(rand *math_rand_tea.Rand, size int) interface{}

func (f GeneratorFunc) Generate(rand *math_rand_tea.Rand, size int) interface{}
```

### Result
Result of a test run.

```csd
type Result struct {
  Passed      bool           // Did the test pass?
  Count       int            // Number of iterations performed
  FailedAfter int            // Iteration that caused failure
  Input       interface{}    // Input that caused failure
  ShrunkInput interface{}    // Shrunk version of input that still fails
  ShrinkCount int            // Number of shrink iterations
  Runtime     timez.Duration // Total time spent testing
}
```

### ShrinkStrategy
Strategy for value shrinking.

```csd
type ShrinkStrategy int

const (
  NoShrink ShrinkStrategy = iota
  DefaultShrink
  FullShrink
  SmartShrink
)
```

## Core Functions

```csd
// Run a single test function with the given configuration
func Check(f interface{}, config *Config) Result

// Test a property for many random values
func CheckProperty(prop interface{}, args ...interface{}) bool

// Generate a random value using the given generator
func Generate(rand *math_rand_tea.Rand, size int, gen Generator) interface{}

// Value creates a generator that always returns the given value
func Value(value interface{}) Generator

// Function to shrink a failing case
func Shrink(f interface{}, input interface{}, config *Config) interface{}
```

## Built-in Generators

```csd
// Generate 8-bit integers
func Int8() Generator
func Int8Range(min, max int8) Generator

// Generate 16-bit integers
func Int16() Generator
func Int16Range(min, max int16) Generator

// Generate 32-bit integers
func Int32() Generator
func Int32Range(min, max int32) Generator

// Generate 64-bit integers
func Int64() Generator
func Int64Range(min, max int64) Generator

// Generate native integers
func Int() Generator
func IntRange(min, max int) Generator

// Generate unsigned integers
func Uint8() Generator
func Uint16() Generator
func Uint32() Generator
func Uint64() Generator
func Uint() Generator

// Generate floating-point numbers
func Float32() Generator
func Float32Range(min, max float32) Generator
func Float64() Generator
func Float64Range(min, max float64) Generator

// Generate complex numbers
func Complex64() Generator
func Complex128() Generator

// Generate strings
func String() Generator
func StringOf(charGen Generator) Generator
func StringOfN(minLen, maxLen int, charGen Generator) Generator

// Generate composite types
func SliceOf(elemGen Generator) Generator
func SliceOfN(minLen, maxLen int, elemGen Generator) Generator
func MapOf(keyGen, valueGen Generator) Generator
func StructOf(fieldGens map[string]Generator) Generator

// Generate specific patterns
func ASCII() Generator
func AlphaNumeric() Generator
func AnyOf(gens ...Generator) Generator
func OneOf(values ...interface{}) Generator
func Boolean() Generator
func Byte() Generator
func Rune() Generator
```

## Enhanced Features

- **Custom Shrinkers**: Define custom shrinking logic for complex types
  ```csd
  myShrinker := quick_test.NewShrinker(func(v MyType) []MyType {
    // Return simpler versions of v
    return []MyType{...}
  })
  ```

- **Stateful Testing**: Test stateful systems with sequences of actions
  ```csd
  model := quick_test.NewStateMachine(initialState)
  model.AddAction("increment", incAction, incPrecondition)
  model.AddAction("reset", resetAction, resetPrecondition)
  result := model.Run(config)
  ```

- **Combinators**: Combine generators to create complex data patterns
  ```csd
  personGen := quick_test.Combine(
    quick_test.StringOf(quick_test.AlphaNumeric()), // name
    quick_test.IntRange(0, 120),                  // age
    func(name string, age int) Person {
      return Person{Name: name, Age: age}
    },
  )
  ```

- **Custom Distributions**: Control the distribution of generated values
  ```csd
  weightedGen := quick_test.Weighted([
    {Weight: 10, Gen: quick_test.IntRange(0, 10)},    // 10x more likely
    {Weight: 1, Gen: quick_test.IntRange(11, 100)},
  ])
  ```

- **Test Reproducibility**: Replay failed tests with exact inputs
  ```csd
  seed := result.Seed
  replay := quick_test.ReplayConfig(seed, failedValue)
  quick_test.Check(prop, replay)
  ```

## Usage Examples

```csd
// Basic property testing example
// Test that reversing a slice twice returns the original slice
reverseTwiceProperty := func(xs []int) bool {
  // Create a copy of the original slice
  original := make([]int, len(xs))
  copy(original, xs)
  
  // Reverse twice
  reversed := reverse(xs) // Assume reverse() is defined elsewhere
  reversedTwice := reverse(reversed)
  
  // Check equality
  if len(original) != len(reversedTwice) {
    return false
  }
  
  for i := range original {
    if original[i] != reversedTwice[i] {
      return false
    }
  }
  
  return true
}

// Run the test
config := &quick_test.Config{
  MaxCount: 100,  // Test 100 different inputs
}

result := quick_test.Check(reverseTwiceProperty, config)
vibez.spill("Reverse twice property test passed: %t", result.Passed)
vibez.spill("Number of test cases run: %d", result.Count)

// Testing with specific generators
// Test that abs(x) >= 0 for all integers x
absPositiveProperty := func(x int) bool {
  return abs(x) >= 0  // Assume abs() is defined elsewhere
}

intGen := quick_test.Int()
config = &quick_test.Config{
  MaxCount: 1000,
}

result = quick_test.Check(func(rand *math_rand_tea.Rand, size int) bool {
  x := quick_test.Generate(rand, size, intGen).(int)
  return absPositiveProperty(x)
}, config)

vibez.spill("\nAbs positive property test passed: %t", result.Passed)
vibez.spill("Number of test cases run: %d", result.Count)

// Testing with custom generators and multiple parameters
// Test that max(x, y) >= x and max(x, y) >= y
maxProperty := func(x, y int) bool {
  m := max(x, y)  // Assume max() is defined elsewhere
  return m >= x && m >= y
}

config = &quick_test.Config{
  MaxCount: 500,
}

result = quick_test.Check(maxProperty, config)
vibez.spill("\nMax property test passed: %t", result.Passed)
vibez.spill("Number of test cases run: %d", result.Count)

// Testing with shrinking
// Define a property that fails for some input
failingProperty := func(xs []int) bool {
  // This property will fail if the slice contains a negative number
  for _, x := range xs {
    if x < 0 {
      return false
    }
  }
  return true
}

// Configure test with shrinking enabled
config = &quick_test.Config{
  MaxCount:       200,
  MaxFailures:    1,  // Stop after first failure
  ShrinkStrategy: quick_test.SmartShrink,
}

result = quick_test.Check(failingProperty, config)
vibez.spill("\nExpected failing property test passed: %t", result.Passed)

if !result.Passed {
  vibez.spill("Failed after %d iterations", result.FailedAfter)
  vibez.spill("Original failing input: %v", result.Input)
  vibez.spill("Shrunk failing input: %v", result.ShrunkInput)
  vibez.spill("Shrink iterations: %d", result.ShrinkCount)
}

// Custom generators example
// Define a Person type
type Person struct {
  Name    string
  Age     int
  Hobbies []string
}

// Define a generator for Person
personGen := quick_test.GeneratorFunc(func(rand *math_rand_tea.Rand, size int) interface{} {
  // Generate name
  nameLen := rand.Intn(size) + 1 // At least 1 character
  name := ""
  for i := 0; i < nameLen; i++ {
    name += string('A' + rand.Intn(26)) // Random uppercase letter
  }
  
  // Generate age
  age := rand.Intn(100) // 0-99
  
  // Generate hobbies
  hobbiesCount := rand.Intn(size/2 + 1)
  hobbies := make([]string, hobbiesCount)
  
  hobbyOptions := []string{"Reading", "Swimming", "Coding", "Gaming", "Cooking"}
  for i := 0; i < hobbiesCount; i++ {
    hobbies[i] = hobbyOptions[rand.Intn(len(hobbyOptions))]
  }
  
  return Person{Name: name, Age: age, Hobbies: hobbies}
})

// Define a property for testing
personProperty := func(p Person) bool {
  // Test that age is valid
  if p.Age < 0 || p.Age >= 100 {
    return false
  }
  
  // Test that name is not empty
  if p.Name == "" {
    return false
  }
  
  return true
}

// Run the test with our custom generator
config = &quick_test.Config{
  MaxCount: 50,
}

result = quick_test.Check(func(rand *math_rand_tea.Rand, size int) bool {
  person := quick_test.Generate(rand, size, personGen).(Person)
  return personProperty(person)
}, config)

vibez.spill("\nPerson property test passed: %t", result.Passed)
vibez.spill("Number of test cases run: %d", result.Count)

// Using combinators to create complex generators
namegen := quick_test.StringOfN(1, 20, quick_test.AnyOf(
  quick_test.GeneratorFunc(func(rand *math_rand_tea.Rand, _ int) interface{} {
    return rune('A' + rand.Intn(26))
  }),
  quick_test.GeneratorFunc(func(rand *math_rand_tea.Rand, _ int) interface{} {
    return rune('a' + rand.Intn(26))
  }),
))

ageGen := quick_test.IntRange(0, 99)

hobbiesGen := quick_test.SliceOfN(0, 5, quick_test.OneOf(
  "Reading", "Swimming", "Coding", "Gaming", "Cooking",
  "Music", "Dancing", "Hiking", "Painting", "Chess",
))

// Combine the generators
combinedPersonGen := quick_test.Combine(
  namegen,
  ageGen,
  hobbiesGen,
  func(name string, age int, hobbies []string) Person {
    return Person{Name: name, Age: age, Hobbies: hobbies}
  },
)

// Run the test with the combined generator
config = &quick_test.Config{
  MaxCount: 50,
}

result = quick_test.Check(func(rand *math_rand_tea.Rand, size int) bool {
  person := quick_test.Generate(rand, size, combinedPersonGen).(Person)
  return personProperty(person)
}, config)

vibez.spill("\nCombined generator test passed: %t", result.Passed)
vibez.spill("Number of test cases run: %d", result.Count)

// Stateful testing example
// Define a simple counter
type Counter struct {
  Value int
}

func (c *Counter) Increment() {
  c.Value++
}

func (c *Counter) Reset() {
  c.Value = 0
}

// Create a state machine model
model := quick_test.NewStateMachine(func() *Counter {
  return &Counter{Value: 0}
})

// Add actions
model.AddAction("increment", 
  // Action function
  func(c *Counter) {
    c.Increment()
  },
  // Precondition
  func(c *Counter) bool {
    return true // Can always increment
  },
  // Invariant check
  func(c *Counter, prevValue int) bool {
    return c.Value == prevValue + 1
  },
)

model.AddAction("reset",
  // Action function
  func(c *Counter) {
    c.Reset()
  },
  // Precondition
  func(c *Counter) bool {
    return true // Can always reset
  },
  // Invariant check
  func(c *Counter, _ int) bool {
    return c.Value == 0
  },
)

// Run the state machine
config = &quick_test.Config{
  MaxCount: 100, // Run 100 sequences of actions
}

result = model.Run(config)
vibez.spill("\nState machine test passed: %t", result.Passed)
vibez.spill("Number of action sequences run: %d", result.Count)

if !result.Passed {
  sequence := result.ShrunkInput.([]string)
  vibez.spill("Failing action sequence: %v", sequence)
}
```

## Implementation Guidelines

- Implement efficient generators that produce a good distribution of values
- Ensure good randomization by using a high-quality random number generator
- Provide comprehensive shrinking strategies to find minimal failing cases
- Support reproducible tests by using seedable random generators
- Implement thread-safe testing to handle concurrent property checks
- Provide clear failure messages that help identify the root cause
- Balance test case diversity with execution time constraints
- Support integration with the standard testing framework
- Implement adequate statistics collection for test quality evaluation
- Support custom generators and combinators for complex data structures