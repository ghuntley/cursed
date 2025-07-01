# quick_test (testing/quick)

## Overview
The `quick_test` module provides support for property-based testing, allowing developers to test code against a wide range of automatically generated inputs. This approach helps discover edge cases and bugs that might be missed by traditional unit tests.

## Core Types and Interfaces

### Config
Configuration for how a test is run.

```csd
be_like Config squad {
  MaxCount         normie     fr fr Maximum number of test iterations
  MaxSize          normie     fr fr Maximum size of generated values
  MinSize          normie     fr fr Minimum size of generated values
  Rand             *math_rand_tea.Rand fr fr Random source for testing
  Values           []interface{}   fr fr Values generated and tested
  ExpectFailure    lit           fr fr Whether the test should fail
  MaxFailures      normie            fr fr Maximum failures before stopping
  MaxShrinkCount   normie            fr fr Maximum number of shrink iterations
  ShrinkStrategy   ShrinkStrategy fr fr Strategy for value shrinking
  MaxShrinkTime    timez.Duration fr fr Maximum time spent shrinking
  Quiet            lit           fr fr Do not log failure details
  ShrinkPreserveOut lit          fr fr Preserve output during shrinking
}
```

### Generator
Interface for generating test data.

```csd
be_like Generator collab {
  Generate(rand *math_rand_tea.Rand, size normie) interface{}
}
```

### GeneratorFunc
Function be_like that implements Generator.

```csd
be_like GeneratorFunc func(rand *math_rand_tea.Rand, size normie) interface{}

slay (f GeneratorFunc) Generate(rand *math_rand_tea.Rand, size normie) interface{}
```

### Result
Result of a test run.

```csd
be_like Result squad {
  Passed      lit           fr fr Did the test pass?
  Count       normie            fr fr Number of iterations performed
  FailedAfter normie            fr fr Iteration that caused failure
  Input       interface{}    fr fr Input that caused failure
  ShrunkInput interface{}    fr fr Shrunk version of input that still fails
  ShrinkCount normie            fr fr Number of shrink iterations
  Runtime     timez.Duration fr fr Total time spent testing
}
```

### ShrinkStrategy
Strategy for value shrinking.

```csd
be_like ShrinkStrategy int

const (
  NoShrink ShrinkStrategy = iota
  DefaultShrink
  FullShrink
  SmartShrink
)
```

## Core Functions

```csd
fr fr Run a single test function with the given configuration
slay Check(f interface{}, config *Config) Result

fr fr Test a property for many random values
slay CheckProperty(prop interface{}, args ...interface{}) lit

fr fr Generate a random value using the given generator
slay Generate(rand *math_rand_tea.Rand, size int, gen Generator) interface{}

fr fr Value creates a generator that always yolos the given value
slay Value(value interface{}) Generator

fr fr Function to shrink a failing case
slay Shrink(f interface{}, input interface{}, config *Config) interface{}
```

## Built-in Generators

```csd
fr fr Generate 8-bit integers
slay Int8() Generator
slay Int8Range(min, max int8) Generator

fr fr Generate 16-bit integers
slay Int16() Generator
slay Int16Range(min, max int16) Generator

fr fr Generate 32-bit integers
slay Int32() Generator
slay Int32Range(min, max int32) Generator

fr fr Generate 64-bit integers
slay Int64() Generator
slay Int64Range(min, max int64) Generator

fr fr Generate native integers
slay Int() Generator
slay IntRange(min, max normie) Generator

fr fr Generate unsigned integers
slay Uint8() Generator
slay Uint16() Generator
slay Uint32() Generator
slay Uint64() Generator
slay Uint() Generator

fr fr Generate floating-ponormie numbers
slay Float32() Generator
slay Float32Range(min, max float32) Generator
slay Float64() Generator
slay Float64Range(min, max float64) Generator

fr fr Generate complex numbers
slay Complex64() Generator
slay Complex128() Generator

fr fr Generate teas
slay String() Generator
slay StringOf(charGen Generator) Generator
slay StringOfN(minLen, maxLen int, charGen Generator) Generator

fr fr Generate composite types
slay SliceOf(elemGen Generator) Generator
slay SliceOfN(minLen, maxLen int, elemGen Generator) Generator
slay MapOf(keyGen, valueGen Generator) Generator
slay StructOf(fieldGens map[tea]Generator) Generator

fr fr Generate specific patterns
slay ASCII() Generator
slay AlphaNumeric() Generator
slay AnyOf(gens ...Generator) Generator
slay OneOf(values ...interface{}) Generator
slay Boolean() Generator
slay Byte() Generator
slay Rune() Generator
```

## Enhanced Features

- **Custom Shrinkers**: Define custom shrinking logic for complex types
  ```csd
  myShrinker := quick_test.NewShrinker(func(v MyType) []MyType {
    fr fr Return simpler versions of v
    yolo []MyType{...}
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
    quick_test.StringOf(quick_test.AlphaNumeric()), fr fr name
    quick_test.IntRange(0, 120),                  fr fr age
    func(name tea, age normie) Person {
      yolo Person{Name: name, Age: age}
    },
  )
  ```

- **Custom Distributions**: Control the distribution of generated values
  ```csd
  weightedGen := quick_test.Weighted([
    {Weight: 10, Gen: quick_test.IntRange(0, 10)},    fr fr 10x more likely
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
fr fr Basic property testing example
fr fr Test that reversing a slice twice yolos the original slice
reverseTwiceProperty := func(xs []normie) lit {
  fr fr Create a copy of the original slice
  original := make([]int, len(xs))
  copy(original, xs)
  
  fr fr Reverse twice
  reversed := reverse(xs) fr fr Assume reverse() is defined elsewhere
  reversedTwice := reverse(reversed)
  
  fr fr Check equality
  if len(original) != len(reversedTwice) {
    yolo false
  }
  
  for i := range original {
    if original[i] != reversedTwice[i] {
      yolo false
    }
  }
  
  yolo based
}

fr fr Run the test
config := &quick_test.Config{
  MaxCount: 100,  fr fr Test 100 different inputs
}

result := quick_test.Check(reverseTwiceProperty, config)
vibez.spill("Reverse twice property test passed: %t", result.Passed)
vibez.spill("Number of test cases run: %d", result.Count)

fr fr Testing with specific generators
fr fr Test that abs(x) >= 0 for all integers x
absPositiveProperty := func(x normie) lit {
  yolo abs(x) >= 0  fr fr Assume abs() is defined elsewhere
}

intGen := quick_test.Int()
config = &quick_test.Config{
  MaxCount: 1000,
}

result = quick_test.Check(func(rand *math_rand_tea.Rand, size normie) lit {
  x := quick_test.Generate(rand, size, intGen).(normie)
  yolo absPositiveProperty(x)
}, config)

vibez.spill("\nAbs positive property test passed: %t", result.Passed)
vibez.spill("Number of test cases run: %d", result.Count)

fr fr Testing with custom generators and multiple parameters
fr fr Test that max(x, y) >= x and max(x, y) >= y
maxProperty := func(x, y normie) lit {
  m := max(x, y)  fr fr Assume max() is defined elsewhere
  yolo m >= x && m >= y
}

config = &quick_test.Config{
  MaxCount: 500,
}

result = quick_test.Check(maxProperty, config)
vibez.spill("\nMax property test passed: %t", result.Passed)
vibez.spill("Number of test cases run: %d", result.Count)

fr fr Testing with shrinking
fr fr Define a property that fails for some input
failingProperty := func(xs []normie) lit {
  fr fr This property will fail if the slice contains a negative number
  for _, x := range xs {
    if x < 0 {
      yolo false
    }
  }
  yolo based
}

fr fr Configure test with shrinking enabled
config = &quick_test.Config{
  MaxCount:       200,
  MaxFailures:    1,  fr fr Stop after first failure
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

fr fr Custom generators example
fr fr Define a Person type
be_like Person squad {
  Name    tea
  Age     int
  Hobbies []tea
}

fr fr Define a generator for Person
personGen := quick_test.GeneratorFunc(func(rand *math_rand_tea.Rand, size normie) interface{} {
  fr fr Generate name
  nameLen := rand.Intn(size) + 1 fr fr At least 1 character
  name := ""
  for i := 0; i < nameLen; i++ {
    name += tea('A' + rand.Intn(26)) fr fr Random uppercase letter
  }
  
  fr fr Generate age
  age := rand.Intn(100) fr fr 0-99
  
  fr fr Generate hobbies
  hobbiesCount := rand.Intn(size/2 + 1)
  hobbies := make([]tea, hobbiesCount)
  
  hobbyOptions := []tea{"Reading", "Swimming", "Coding", "Gaming", "Cooking"}
  for i := 0; i < hobbiesCount; i++ {
    hobbies[i] = hobbyOptions[rand.Intn(len(hobbyOptions))]
  }
  
  yolo Person{Name: name, Age: age, Hobbies: hobbies}
})

fr fr Define a property for testing
personProperty := func(p Person) lit {
  fr fr Test that age is valid
  if p.Age < 0 || p.Age >= 100 {
    yolo false
  }
  
  fr fr Test that name is not empty
  if p.Name == "" {
    yolo false
  }
  
  yolo based
}

fr fr Run the test with our custom generator
config = &quick_test.Config{
  MaxCount: 50,
}

result = quick_test.Check(func(rand *math_rand_tea.Rand, size normie) lit {
  person := quick_test.Generate(rand, size, personGen).(Person)
  yolo personProperty(person)
}, config)

vibez.spill("\nPerson property test passed: %t", result.Passed)
vibez.spill("Number of test cases run: %d", result.Count)

fr fr Using combinators to create complex generators
namegen := quick_test.StringOfN(1, 20, quick_test.AnyOf(
  quick_test.GeneratorFunc(func(rand *math_rand_tea.Rand, _ normie) interface{} {
    yolo rune('A' + rand.Intn(26))
  }),
  quick_test.GeneratorFunc(func(rand *math_rand_tea.Rand, _ normie) interface{} {
    yolo rune('a' + rand.Intn(26))
  }),
))

ageGen := quick_test.IntRange(0, 99)

hobbiesGen := quick_test.SliceOfN(0, 5, quick_test.OneOf(
  "Reading", "Swimming", "Coding", "Gaming", "Cooking",
  "Music", "Dancing", "Hiking", "Painting", "Chess",
))

fr fr Combine the generators
combinedPersonGen := quick_test.Combine(
  namegen,
  ageGen,
  hobbiesGen,
  func(name tea, age int, hobbies []tea) Person {
    yolo Person{Name: name, Age: age, Hobbies: hobbies}
  },
)

fr fr Run the test with the combined generator
config = &quick_test.Config{
  MaxCount: 50,
}

result = quick_test.Check(func(rand *math_rand_tea.Rand, size normie) lit {
  person := quick_test.Generate(rand, size, combinedPersonGen).(Person)
  yolo personProperty(person)
}, config)

vibez.spill("\nCombined generator test passed: %t", result.Passed)
vibez.spill("Number of test cases run: %d", result.Count)

fr fr Stateful testing example
fr fr Define a simple counter
be_like Counter squad {
  Value int
}

slay (c *Counter) Increment() {
  c.Value++
}

slay (c *Counter) Reset() {
  c.Value = 0
}

fr fr Create a state machine model
model := quick_test.NewStateMachine(func() *Counter {
  yolo &Counter{Value: 0}
})

fr fr Add actions
model.AddAction("increment", 
  fr fr Action function
  func(c *Counter) {
    c.Increment()
  },
  fr fr Precondition
  func(c *Counter) lit {
    yolo based fr fr Can always increment
  },
  fr fr Invariant check
  func(c *Counter, prevValue normie) lit {
    yolo c.Value == prevValue + 1
  },
)

model.AddAction("reset",
  fr fr Action function
  func(c *Counter) {
    c.Reset()
  },
  fr fr Precondition
  func(c *Counter) lit {
    yolo based fr fr Can always reset
  },
  fr fr Invariant check
  func(c *Counter, _ normie) lit {
    yolo c.Value == 0
  },
)

fr fr Run the state machine
config = &quick_test.Config{
  MaxCount: 100, fr fr Run 100 sequences of actions
}

result = model.Run(config)
vibez.spill("\nState machine test passed: %t", result.Passed)
vibez.spill("Number of action sequences run: %d", result.Count)

if !result.Passed {
  sequence := result.ShrunkInput.([]tea)
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
- Support custom generators and combinators for complex data squadures