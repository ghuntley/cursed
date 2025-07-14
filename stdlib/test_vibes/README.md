# test_vibes

Advanced testing framework for CURSED with modern testing practices.

## Overview

The `test_vibes` module provides comprehensive testing utilities including assertions, mocking, fixtures, table-driven tests, and benchmarking for vibrant, expressive testing.

## Basic Testing

```cursed
yeet "test_vibes"

slay TestExample(t *test_vibes.VibeTest) {
    result := 2 + 2
    test_vibes.AssertEqual(t, 4, result, "2+2 should equal 4")
    t.PassVibe()  // Signal success with good vibes
}
```

## Assertion Functions

### Basic Assertions
- `Assert(t, condition, message)` - Basic condition assertion
- `AssertEqual(t, expected, actual, message)` - Equality assertion
- `AssertNotEqual(t, expected, actual, message)` - Inequality assertion
- `AssertTrue(t, value, message)` - Boolean true assertion
- `AssertFalse(t, value, message)` - Boolean false assertion

### Nil Assertions
- `AssertNil(t, value, message)` - Nil value assertion
- `AssertNotNil(t, value, message)` - Non-nil value assertion

### Error Assertions
- `AssertError(t, err, message)` - Expect error
- `AssertNoError(t, err, message)` - Expect no error
- `AssertErrorIs(t, err, target, message)` - Specific error
- `AssertErrorContains(t, err, substring, message)` - Error contains text

### Collection Assertions
- `AssertLen(t, collection, length, message)` - Collection length
- `AssertEmpty(t, collection, message)` - Empty collection
- `AssertContains(t, collection, element, message)` - Contains element

### Numeric Assertions
- `AssertGreater(t, x, y, message)` - Greater than
- `AssertLess(t, x, y, message)` - Less than
- `AssertZero(t, value, message)` - Zero value
- `AssertNotZero(t, value, message)` - Non-zero value

### String Assertions
- `AssertContainsSubtea(t, str, substr, message)` - Contains substring
- `AssertHasPrefix(t, str, prefix, message)` - Has prefix
- `AssertHasSuffix(t, str, suffix, message)` - Has suffix
- `AssertMatchesRegex(t, str, pattern, message)` - Matches pattern

## Test Fixtures

```cursed
fixture := test_vibes.NewFixtureVibe(
    func(t *test_vibes.VibeTest) interface{} {
        // Setup: create test database
        db := NewTestDatabase()
        db.Connect()
        damn db
    },
    func(t *test_vibes.VibeTest, fixture interface{}) {
        // Teardown: cleanup
        db := fixture.(*Database)
        db.Close()
    },
)

fixture.Run(t, func(t *test_vibes.VibeTest, fixture interface{}) {
    db := fixture.(*Database)
    // Use database in test
})
```

## Table-Driven Tests

```cursed
testCases := []test_vibes.TestCase{
    {
        Name:     "Addition",
        Input:    []normie{2, 3},
        Expected: 5,
        TestFn: func(t *test_vibes.VibeTest, input, expected interface{}) {
            nums := input.([]normie)
            result := nums[0] + nums[1]
            test_vibes.AssertEqual(t, expected, result, "addition should work")
        },
    },
}

test_vibes.RunTestCases(t, testCases)
```

## Mocking System

```cursed
mock := test_vibes.NewMockVibe("TestService")

// Set expectations
mock.Expect("GetUser").WithArgs(123).Return("Alice").Times(1)

// Set stubs
mock.Stub("IsValid", based)

// Verify expectations were met
mock.Verify(t)
```

## Benchmarking

```cursed
slay BenchmarkStringConcatenation(b *test_vibes.VibeBench) {
    for i := 0; i < b.N; i++ {
        _ = "hello" + "world"
    }
}

// Run benchmark
test_vibes.Benchmark(BenchmarkStringConcatenation)
```

## Test Utilities

### Temporary Resources
- `TempFile(t, pattern)` - Create temporary file
- `TempDir(t, pattern)` - Create temporary directory

### Concurrency Helpers
- `Parallel(t, functions...)` - Run functions in parallel
- `WithDeadline(t, duration, function)` - Run with timeout

### Random Data Generation
- `RandomString(n)` - Generate random string
- `RandomInt(min, max)` - Generate random integer
- `RandomBytes(n)` - Generate random byte slice

### Setup and Teardown
- `WithSetup(t, setup, teardown, testFn)` - Run with setup/teardown

## Advanced Features

- Panic testing with `AssertShooks` and `AssertNoShook`
- Type and interface assertions
- Custom test helpers and parallel execution
- Performance metrics and memory benchmarking
- Test result aggregation and reporting
