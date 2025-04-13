# TestVibes (testing package)

## Overview
TestVibes provides support for automated testing and benchmarking of Cursed packages. It's inspired by Go's testing package but with a focus on expressive, vibrant testing and modern development practices.

## Core Types

### `VibeTest`
The core struct for a single test.

```go
type VibeTest struct {}

// Methods
func (t *VibeTest) Error(args ...interface{})
func (t *VibeTest) Errorf(format string, args ...interface{})
func (t *VibeTest) Fail()
func (t *VibeTest) FailNow()
func (t *VibeTest) Failed() bool
func (t *VibeTest) Fatal(args ...interface{})
func (t *VibeTest) Fatalf(format string, args ...interface{})
func (t *VibeTest) Helper() // Marks the calling function as a test helper
func (t *VibeTest) Log(args ...interface{})
func (t *VibeTest) Logf(format string, args ...interface{})
func (t *VibeTest) Name() string
func (t *VibeTest) Parallel() // Signals that test can be run in parallel
func (t *VibeTest) Skip(args ...interface{})
func (t *VibeTest) SkipNow()
func (t *VibeTest) Skipf(format string, args ...interface{})
func (t *VibeTest) Skipped() bool
func (t *VibeTest) TempDir() string // Returns a temporary directory for the test
func (t *VibeTest) PassVibe() // Signals that the test passed with good vibes
func (t *VibeTest) FailVibe(message string) // Signals that the test failed with bad vibes
```

### `VibeBench`
The core struct for a single benchmark.

```go
type VibeBench struct {
    N int // The number of iterations to run
}

// Methods
func (b *VibeBench) Error(args ...interface{})
func (b *VibeBench) Errorf(format string, args ...interface{})
func (b *VibeBench) Fail()
func (b *VibeBench) FailNow()
func (b *VibeBench) Failed() bool
func (b *VibeBench) Fatal(args ...interface{})
func (b *VibeBench) Fatalf(format string, args ...interface{})
func (b *VibeBench) Helper()
func (b *VibeBench) Log(args ...interface{})
func (b *VibeBench) Logf(format string, args ...interface{})
func (b *VibeBench) Name() string
func (b *VibeBench) Skip(args ...interface{})
func (b *VibeBench) SkipNow()
func (b *VibeBench) Skipf(format string, args ...interface{})
func (b *VibeBench) Skipped() bool
func (b *VibeBench) ResetTimer() // Reset the benchmark timer
func (b *VibeBench) StartTimer() // Start the benchmark timer
func (b *VibeBench) StopTimer() // Stop the benchmark timer
func (b *VibeBench) ReportMetric(n float64, unit string) // Report custom metrics
func (b *VibeBench) SetBytes(n int64) // Set number of bytes processed per operation
func (b *VibeBench) SetParallelism(p int) // Set parallelism for benchmarks
```

### `TestMain` Function
The entry point for a test package.

```go
func TestMain(m *VibeTestingManager)

type VibeTestingManager struct {}

// Methods
func (m *VibeTestingManager) Run() int
```

## Assertion Functions

```go
// Basic assertions
func Assert(t *VibeTest, condition bool, message string)
func AssertEqual(t *VibeTest, expected, actual interface{}, message string)
func AssertNotEqual(t *VibeTest, expected, actual interface{}, message string)
func AssertNil(t *VibeTest, actual interface{}, message string)
func AssertNotNil(t *VibeTest, actual interface{}, message string)
func AssertTrue(t *VibeTest, actual bool, message string)
func AssertFalse(t *VibeTest, actual bool, message string)

// Error assertions
func AssertError(t *VibeTest, err error, message string)
func AssertNoError(t *VibeTest, err error, message string)
func AssertErrorIs(t *VibeTest, err, target error, message string)
func AssertErrorContains(t *VibeTest, err error, contains string, message string)

// Collection assertions
func AssertLen(t *VibeTest, collection interface{}, length int, message string)
func AssertEmpty(t *VibeTest, collection interface{}, message string)
func AssertNotEmpty(t *VibeTest, collection interface{}, message string)
func AssertContains(t *VibeTest, collection, element interface{}, message string)
func AssertNotContains(t *VibeTest, collection, element interface{}, message string)

// Numeric assertions
func AssertGreater(t *VibeTest, x, y interface{}, message string)
func AssertGreaterOrEqual(t *VibeTest, x, y interface{}, message string)
func AssertLess(t *VibeTest, x, y interface{}, message string)
func AssertLessOrEqual(t *VibeTest, x, y interface{}, message string)
func AssertZero(t *VibeTest, actual interface{}, message string)
func AssertNotZero(t *VibeTest, actual interface{}, message string)

// String assertions
func AssertContainsSubstring(t *VibeTest, str, substr string, message string)
func AssertHasPrefix(t *VibeTest, str, prefix string, message string)
func AssertHasSuffix(t *VibeTest, str, suffix string, message string)
func AssertMatchesRegex(t *VibeTest, str, pattern string, message string)

// Type assertions
func AssertType(t *VibeTest, expectedType, value interface{}, message string)
func AssertImplements(t *VibeTest, interfaceObj, value interface{}, message string)

// Panic assertions
func AssertPanics(t *VibeTest, fn func(), message string)
func AssertPanicsWithValue(t *VibeTest, value interface{}, fn func(), message string)
func AssertNoPanic(t *VibeTest, fn func(), message string)
```

## Test Fixtures

```go
type FixtureVibe struct {
    SetupFn    func(t *VibeTest) interface{}
    TeardownFn func(t *VibeTest, fixture interface{})
}

// Create a new fixture
func NewFixtureVibe(setup func(t *VibeTest) interface{}, teardown func(t *VibeTest, fixture interface{})) *FixtureVibe

// Run a test with the fixture
func (f *FixtureVibe) Run(t *VibeTest, testFn func(t *VibeTest, fixture interface{}))
```

## Table-Driven Tests

```go
type TestCase struct {
    Name     string
    Input    interface{}
    Expected interface{}
    SetupFn  func(t *VibeTest) // Optional setup
    TestFn   func(t *VibeTest, input, expected interface{})
}

func RunTestCases(t *VibeTest, testCases []TestCase)
```

## Mocking

```go
type MockVibe struct {
    Name string
}

// Method expectation
func (m *MockVibe) Expect(methodName string) *Expectation

// Method stubbing
func (m *MockVibe) Stub(methodName string, returnValues ...interface{}) *Stub

// Verify all expectations were met
func (m *MockVibe) Verify(t *VibeTest)

type Expectation struct {}

// Methods for configuring expectations
func (e *Expectation) WithArgs(args ...interface{}) *Expectation
func (e *Expectation) Return(values ...interface{}) *Expectation
func (e *Expectation) ReturnFn(fn func(args ...interface{}) []interface{}) *Expectation
func (e *Expectation) Times(n int) *Expectation
func (e *Expectation) AtLeast(n int) *Expectation
func (e *Expectation) AtMost(n int) *Expectation
```

## Test Utilities

```go
// Temporary resources
func TempFile(t *VibeTest, pattern string) (*dropz.File, string)
func TempDir(t *VibeTest, pattern string) string

// Concurrency helpers
func Parallel(t *VibeTest, fns ...func(t *VibeTest)) // Run functions in parallel
func WithDeadline(t *VibeTest, d time.Duration, fn func(t *VibeTest)) // Run with timeout

// Setup and teardown
func WithSetup(t *VibeTest, setup, teardown func(), testFn func(t *VibeTest))

// Random data generation
func RandomString(n int) string
func RandomInt(min, max int) int
func RandomFloat(min, max float64) float64
func RandomBytes(n int) []byte
```

## Benchmarking Utilities

```go
func Benchmark(f func(b *VibeBench)) // Defines a benchmark function
func BenchmarkMemory(f func(b *VibeBench)) // Benchmarks memory usage
func BenchmarkParallel(f func(b *VibeBench, pb *testing.PB)) // Parallel benchmark
```

## Usage Example

```go
// Simple test function
func TestAddition(t *VibeTest) {
    result := 2 + 2
    TestVibes.AssertEqual(t, 4, result, "2+2 should equal 4")
}

// Table-driven test
func TestStringOperations(t *VibeTest) {
    testCases := []TestVibes.TestCase{
        {
            Name:     "Uppercase conversion",
            Input:    "hello",
            Expected: "HELLO",
            TestFn: func(t *VibeTest, input, expected interface{}) {
                result := stringz.ToUpper(input.(string))
                TestVibes.AssertEqual(t, expected, result, "String should be uppercased")
            },
        },
        {
            Name:     "Lowercase conversion",
            Input:    "WORLD",
            Expected: "world",
            TestFn: func(t *VibeTest, input, expected interface{}) {
                result := stringz.ToLower(input.(string))
                TestVibes.AssertEqual(t, expected, result, "String should be lowercased")
            },
        },
    }
    
    TestVibes.RunTestCases(t, testCases)
}

// Test with fixture
func TestDatabase(t *VibeTest) {
    fixture := TestVibes.NewFixtureVibe(
        func(t *VibeTest) interface{} {
            // Setup: Create a test database
            db := NewTestDatabase()
            db.Connect()
            return db
        },
        func(t *VibeTest, fixture interface{}) {
            // Teardown: Close the database connection
            db := fixture.(*Database)
            db.Close()
        },
    )
    
    fixture.Run(t, func(t *VibeTest, fixture interface{}) {
        db := fixture.(*Database)
        
        // Test database operations
        user, err := db.CreateUser("testuser")
        TestVibes.AssertNoError(t, err, "Creating user should not error")
        TestVibes.AssertNotNil(t, user, "User should be created")
        TestVibes.AssertEqual(t, "testuser", user.Name, "User name should match")
    })
}

// Benchmark
func BenchmarkStringConcatenation(b *VibeBench) {
    for i := 0; i < b.N; i++ {
        _ = "hello" + "world"
    }
}

// Main test function
func TestMain(m *TestVibes.VibeTestingManager) {
    // Setup before all tests
    setup()
    
    // Run all tests
    code := m.Run()
    
    // Cleanup after all tests
    teardown()
    
    // Exit with the appropriate code
    os.Exit(code)
}
```

## Implementation Guidelines
1. Emphasis on clear, readable test output
2. Efficient test execution with parallelism where appropriate
3. Comprehensive failure messages that help pinpoint issues
4. Support for integration with CI/CD pipelines
5. Minimal dependencies on other packages
6. Extendable design that allows for custom test reporters and runners