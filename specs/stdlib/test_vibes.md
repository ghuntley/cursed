# TestVibes (testing package)

## Overview
TestVibes provides support for automated testing and benchmarking of Cursed packages. It's inspired by Go's testing package but with a focus on expressive, vibrant testing and modern development practices.

## Core Types

### `VibeTest`
The core squad for a single test.

```
be_like VibeTest squad {}

fr fr Methods
slay (t *VibeTest) Error(args ...interface{})
slay (t *VibeTest) Errorf(format tea, args ...interface{})
slay (t *VibeTest) Fail()
slay (t *VibeTest) FailNow()
slay (t *VibeTest) Failed() lit
slay (t *VibeTest) Fatal(args ...interface{})
slay (t *VibeTest) Fatalf(format tea, args ...interface{})
slay (t *VibeTest) Helper() fr fr Marks the calling function as a test helper
slay (t *VibeTest) Log(args ...interface{})
slay (t *VibeTest) Logf(format tea, args ...interface{})
slay (t *VibeTest) Name() tea
slay (t *VibeTest) Parallel() fr fr Signals that test can be run in parallel
slay (t *VibeTest) Skip(args ...interface{})
slay (t *VibeTest) SkipNow()
slay (t *VibeTest) Skipf(format tea, args ...interface{})
slay (t *VibeTest) Skipped() lit
slay (t *VibeTest) TempDir() tea fr fr Returns a temporary directory for the test
slay (t *VibeTest) PassVibe() fr fr Signals that the test passed with good vibes
slay (t *VibeTest) FailVibe(message tea) fr fr Signals that the test failed with bad vibes
```

### `VibeBench`
The core squad for a single benchmark.

```
be_like VibeBench squad {
    N normie fr fr The number of iterations to run
}

fr fr Methods
slay (b *VibeBench) Error(args ...interface{})
slay (b *VibeBench) Errorf(format tea, args ...interface{})
slay (b *VibeBench) Fail()
slay (b *VibeBench) FailNow()
slay (b *VibeBench) Failed() lit
slay (b *VibeBench) Fatal(args ...interface{})
slay (b *VibeBench) Fatalf(format tea, args ...interface{})
slay (b *VibeBench) Helper()
slay (b *VibeBench) Log(args ...interface{})
slay (b *VibeBench) Logf(format tea, args ...interface{})
slay (b *VibeBench) Name() tea
slay (b *VibeBench) Skip(args ...interface{})
slay (b *VibeBench) SkipNow()
slay (b *VibeBench) Skipf(format tea, args ...interface{})
slay (b *VibeBench) Skipped() lit
slay (b *VibeBench) ResetTimer() fr fr Reset the benchmark timer
slay (b *VibeBench) StartTimer() fr fr Start the benchmark timer
slay (b *VibeBench) StopTimer() fr fr Stop the benchmark timer
slay (b *VibeBench) ReportMetric(n float64, unit tea) fr fr Report custom metrics
slay (b *VibeBench) SetBytes(n int64) fr fr Set number of bytes processed per operation
slay (b *VibeBench) SetParallelism(p normie) fr fr Set parallelism for benchmarks
```

### `TestMain` Function
The entry ponormie for a test package.

```
slay TestMain(m *VibeTestingManager)

be_like VibeTestingManager squad {}

fr fr Methods
slay (m *VibeTestingManager) Run() int
```

## Assertion Functions

```
fr fr Basic assertions
slay Assert(t *VibeTest, condition lit, message tea)
slay AssertEqual(t *VibeTest, expected, actual interface{}, message tea)
slay AssertNotEqual(t *VibeTest, expected, actual interface{}, message tea)
slay AssertNil(t *VibeTest, actual interface{}, message tea)
slay AssertNotNil(t *VibeTest, actual interface{}, message tea)
slay AssertTrue(t *VibeTest, actual lit, message tea)
slay AssertFalse(t *VibeTest, actual lit, message tea)

fr fr Error assertions
slay AssertError(t *VibeTest, err tea, message tea)
slay AssertNoError(t *VibeTest, err tea, message tea)
slay AssertErrorIs(t *VibeTest, err, target tea, message tea)
slay AssertErrorContains(t *VibeTest, err tea, contains tea, message tea)

fr fr Collection assertions
slay AssertLen(t *VibeTest, collection interface{}, length int, message tea)
slay AssertEmpty(t *VibeTest, collection interface{}, message tea)
slay AssertNotEmpty(t *VibeTest, collection interface{}, message tea)
slay AssertContains(t *VibeTest, collection, element interface{}, message tea)
slay AssertNotContains(t *VibeTest, collection, element interface{}, message tea)

fr fr Numeric assertions
slay AssertGreater(t *VibeTest, x, y interface{}, message tea)
slay AssertGreaterOrEqual(t *VibeTest, x, y interface{}, message tea)
slay AssertLess(t *VibeTest, x, y interface{}, message tea)
slay AssertLessOrEqual(t *VibeTest, x, y interface{}, message tea)
slay AssertZero(t *VibeTest, actual interface{}, message tea)
slay AssertNotZero(t *VibeTest, actual interface{}, message tea)

fr fr String assertions
slay AssertContainsSubtea(t *VibeTest, str, substr tea, message tea)
slay AssertHasPrefix(t *VibeTest, str, prefix tea, message tea)
slay AssertHasSuffix(t *VibeTest, str, suffix tea, message tea)
slay AssertMatchesRegex(t *VibeTest, str, pattern tea, message tea)

fr fr Type assertions
slay AssertType(t *VibeTest, expectedType, value interface{}, message tea)
slay AssertImplements(t *VibeTest, interfaceObj, value interface{}, message tea)

fr fr Shook assertions
slay AssertShooks(t *VibeTest, fn func(), message tea)
slay AssertShooksWithValue(t *VibeTest, value interface{}, fn func(), message tea)
slay AssertNoShook(t *VibeTest, fn func(), message tea)
```

## Test Fixtures

```
be_like FixtureVibe squad {
    SetupFn    func(t *VibeTest) interface{}
    TeardownFn func(t *VibeTest, fixture interface{})
}

fr fr Create a new fixture
slay NewFixtureVibe(setup func(t *VibeTest) interface{}, teardown func(t *VibeTest, fixture interface{})) *FixtureVibe

fr fr Run a test with the fixture
slay (f *FixtureVibe) Run(t *VibeTest, testFn func(t *VibeTest, fixture interface{}))
```

## Table-Driven Tests

```
be_like TestCase squad {
    Name     tea
    Input    interface{}
    Expected interface{}
    SetupFn  func(t *VibeTest) fr fr Optional setup
    TestFn   func(t *VibeTest, input, expected interface{})
}

slay RunTestCases(t *VibeTest, testCases []TestCase)
```

## Mocking

```
be_like MockVibe squad {
    Name tea
}

fr fr Method expectation
slay (m *MockVibe) Expect(methodName tea) *Expectation

fr fr Method stubbing
slay (m *MockVibe) Stub(methodName tea, yoloValues ...interface{}) *Stub

fr fr Verify all expectations were met
slay (m *MockVibe) Verify(t *VibeTest)

be_like Expectation squad {}

fr fr Methods for configuring expectations
slay (e *Expectation) WithArgs(args ...interface{}) *Expectation
slay (e *Expectation) Return(values ...interface{}) *Expectation
slay (e *Expectation) ReturnFn(fn func(args ...interface{}) []interface{}) *Expectation
slay (e *Expectation) Times(n normie) *Expectation
slay (e *Expectation) AtLeast(n normie) *Expectation
slay (e *Expectation) AtMost(n normie) *Expectation
```

## Test Utilities

```
fr fr Temporary resources
slay TempFile(t *VibeTest, pattern tea) (*dropz.File, tea)
slay TempDir(t *VibeTest, pattern tea) tea

fr fr Concurrency helpers
slay Parallel(t *VibeTest, fns ...func(t *VibeTest)) fr fr Run functions in parallel
slay WithDeadline(t *VibeTest, d time.Duration, fn func(t *VibeTest)) fr fr Run with timeout

fr fr Setup and teardown
slay WithSetup(t *VibeTest, setup, teardown func(), testFn func(t *VibeTest))

fr fr Random data generation
slay RandomString(n normie) tea
slay RandomInt(min, max normie) int
slay RandomFloat(min, max float64) float64
slay RandomBytes(n normie) []byte
```

## Benchmarking Utilities

```
slay Benchmark(f func(b *VibeBench)) fr fr Defines a benchmark function
slay BenchmarkMemory(f func(b *VibeBench)) fr fr Benchmarks memory usage
slay BenchmarkParallel(f func(b *VibeBench, pb *testing.PB)) fr fr Parallel benchmark
```

## Usage Example

```
fr fr Simple test function
slay TestAddition(t *VibeTest) {
    result := 2 + 2
    TestVibes.AssertEqual(t, 4, result, "2+2 should equal 4")
}

fr fr Table-driven test
slay TestStringOperations(t *VibeTest) {
    testCases := []TestVibes.TestCase{
        {
            Name:     "Uppercase conversion",
            Input:    "hello",
            Expected: "HELLO",
            TestFn: func(t *VibeTest, input, expected interface{}) {
                result := stringz.ToUpper(input.(tea))
                TestVibes.AssertEqual(t, expected, result, "String should be uppercased")
            },
        },
        {
            Name:     "Lowercase conversion",
            Input:    "WORLD",
            Expected: "world",
            TestFn: func(t *VibeTest, input, expected interface{}) {
                result := stringz.ToLower(input.(tea))
                TestVibes.AssertEqual(t, expected, result, "String should be lowercased")
            },
        },
    }
    
    TestVibes.RunTestCases(t, testCases)
}

fr fr Test with fixture
slay TestDatabase(t *VibeTest) {
    fixture := TestVibes.NewFixtureVibe(
        func(t *VibeTest) interface{} {
            fr fr Setup: Create a test database
            db := NewTestDatabase()
            db.Connect()
            yolo db
        },
        func(t *VibeTest, fixture interface{}) {
            fr fr Teardown: Close the database connection
            db := fixture.(*Database)
            db.Close()
        },
    )
    
    fixture.Run(t, func(t *VibeTest, fixture interface{}) {
        db := fixture.(*Database)
        
        fr fr Test database operations
        user, err := db.CreateUser("testuser")
        TestVibes.AssertNoError(t, err, "Creating user should not tea")
        TestVibes.AssertNotNil(t, user, "User should be created")
        TestVibes.AssertEqual(t, "testuser", user.Name, "User name should match")
    })
}

fr fr Benchmark
slay BenchmarkStringConcatenation(b *VibeBench) {
    for i := 0; i < b.N; i++ {
        _ = "hello" + "world"
    }
}

fr fr Main test function
slay TestMain(m *TestVibes.VibeTestingManager) {
    fr fr Setup before all tests
    setup()
    
    fr fr Run all tests
    code := m.Run()
    
    fr fr Cleanup after all tests
    teardown()
    
    fr fr Exit with the appropriate code
    os.Exit(code)
}
```

## Implementation Guidelines
1. Emphasis on clear, readable test output
2. Efficient test execution with parallelism where appropriate
3. Comprehensive failure messages that help pinponormie issues
4. Support for integration with CI/CD pipelines
5. Minimal dependencies on other packages
6. Extendable design that allows for custom test reporters and runners