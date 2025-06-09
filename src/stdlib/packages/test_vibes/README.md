# TestVibes - CURSED Testing Framework 🧪✨

TestVibes provides comprehensive testing and benchmarking capabilities for the CURSED programming language with Gen Z energy and modern development practices.

## Features

### 🎯 Core Testing
- **VibeTest**: Individual test execution with pass/fail/skip states
- **VibeBench**: Performance benchmarking with detailed metrics
- **VibeTestingManager**: Test suite coordination and execution

### ✅ Rich Assertions
- Basic assertions (equal, true, false, nil)
- Error assertions (error, no_error, error_contains)
- Collection assertions (length, empty, contains)
- String assertions (substring, prefix, suffix, regex)
- Numeric assertions (greater, less, range, zero)
- Type assertions and panic testing ("shook" assertions)

### 🏗️ Test Infrastructure
- **Fixtures**: Setup/teardown with resource management
- **Table-driven tests**: Parameterized test execution
- **Test suites**: Organized test groupings
- **Mocking**: Mock objects with expectations and verification
- **Spies**: Call recording and verification

### ⚡ Performance Testing
- **Benchmarking**: CPU performance measurement
- **Memory benchmarks**: Memory usage tracking
- **Parallel benchmarks**: Concurrent performance testing
- **Benchmark comparison**: Performance regression detection

### 🔧 Advanced Features
- **Test runners**: Configurable test execution
- **Test registry**: Global test registration and discovery
- **Matchers**: Advanced assertion patterns
- **Utilities**: Temporary files, random data, timing helpers
- **Parallel execution**: Concurrent test running

## Quick Start

```rust
use cursed::stdlib::packages::test_vibes::*;

fn main() {
    // Initialize the framework
    init_test_vibes();
    
    // Create a test
    let mut test = VibeTest::new("my_test".to_string());
    
    // Make assertions
    assert_equal(&mut test, 4, 2 + 2, "math should work");
    assert_true(&mut test, true, "true is true");
    
    // Check results
    if test.failed() {
        println!("Test failed! 😞");
    } else {
        test.pass_vibe();
        println!("Test passed! ✨");
    }
}
```

## Assertion Examples

```rust
let mut test = VibeTest::new("assertions".to_string());

// Basic assertions
assert_equal(&mut test, 42, 42, "numbers should match");
assert_true(&mut test, true, "should be true");
assert_false(&mut test, false, "should be false");

// String assertions
assert_contains_substr(&mut test, "hello world", "world", "should contain text");
assert_has_prefix(&mut test, "TestVibes", "Test", "should start with Test");

// Collection assertions
let items = vec![1, 2, 3];
assert_len(&mut test, &items, 3, "should have 3 items");
assert_contains(&mut test, &items, &2, "should contain 2");

// Error assertions
let result: Result<(), &str> = Ok(());
assert_no_error(&mut test, result, "should succeed");
```

## Fixtures and Setup

```rust
let fixture = FixtureVibe::new(
    |_t| {
        // Setup: create test resources
        DatabaseFixture::new("test://localhost/db")
    },
    |_t, mut db| {
        // Teardown: cleanup resources
        db.disconnect();
    },
);

let mut test = VibeTest::new("database_test".to_string());
fixture.run(&mut test, |t, mut db| {
    // Test with the fixture
    let result = db.connect();
    assert_no_error(t, result, "should connect");
});
```

## Table-Driven Tests

```rust
let test_cases = vec![
    TestCase::new("addition", (2, 3), 5, |t, (a, b), expected| {
        let result = a + b;
        assert_equal(t, expected, result, "addition should work");
    }),
    TestCase::new("multiplication", (4, 5), 20, |t, (a, b), expected| {
        let result = a * b;
        assert_equal(t, expected, result, "multiplication should work");
    }),
];

for case in test_cases {
    let mut test = VibeTest::new(format!("math::{}", case.name));
    case.run(&mut test);
}
```

## Mocking

```rust
let mock = MockVibe::new("api_service");

// Set expectations
let _expectation = mock.expect("get_user").times(1);

// Use the mock
mock.record_call("get_user", vec!["123".to_string()]);

// Verify expectations
let mut test = VibeTest::new("mock_test".to_string());
mock.verify(&mut test);
```

## Benchmarking

```rust
let benchmark = Benchmark::new("string_ops", |b| {
    for _ in 0..b.iterations() {
        let _ = format!("{}{}", "hello", "world");
    }
}).iterations(1000);

let result = benchmark.run();
println!("Benchmark: {:.2} ns/op", result.ns_per_op);
```

## Test Runner

```rust
let mut runner = TestRunner::new(TestRunnerConfig::default());

runner.add_test("basic_test", |t| {
    assert_equal(t, 4, 2 + 2, "math works");
});

runner.add_benchmark("perf_test", |b| {
    for _ in 0..b.iterations() {
        let _ = expensive_operation();
    }
});

let summary = runner.run_tests();
println!("Tests: {} passed, {} failed", summary.passed_tests, summary.failed_tests);
```

## Advanced Matchers

```rust
use cursed::stdlib::packages::test_vibes::matchers::*;

let mut test = VibeTest::new("matchers".to_string());

// Use advanced matchers
expect(&mut test, 42, equal(42));
expect(&mut test, 10, greater_than(5));
expect(&mut test, "hello world".to_string(), contains_substring("world"));
expect(&mut test, vec![1, 2, 3], has_length(3));

// Combine matchers
expect(&mut test, 7, not(equal(5)));
```

## Utilities

```rust
let mut test = VibeTest::new("utilities".to_string());

// Random data generation
let random_str = random_string(10);
let random_num = random_int(1, 100);
let random_data = random_bytes(256);

// Temporary resources
let (temp_file, path) = temp_file(&mut test, "test").unwrap();
let (temp_dir, dir_path) = temp_dir(&mut test, "workspace").unwrap();

// Timing utilities
let (result, duration) = time_function(|| expensive_operation());

// Eventually assertions (retry with timeout)
eventually_assert(
    &mut test,
    || check_condition(),
    Duration::from_secs(5),
    "condition should eventually be true",
);
```

## Integration

TestVibes integrates seamlessly with the CURSED standard library and provides:

- **Package initialization**: `init_test_vibes()`
- **Quick helpers**: `quick_test()`, `quick_bench()`
- **Global registry**: Automatic test discovery
- **LLVM integration**: Native performance testing
- **Error system integration**: Rich error reporting

## Philosophy

TestVibes follows the CURSED language philosophy of being expressive, modern, and developer-friendly while maintaining high performance and reliability. The framework emphasizes:

- **Clear intent**: Readable assertions and test structure
- **Good vibes**: Positive, encouraging test output
- **Modern practices**: Property-based testing, fixtures, mocking
- **Performance focus**: Built-in benchmarking and optimization
- **Developer experience**: Helpful error messages and debugging tools

## Examples

See `examples/test_vibes_example.rs` for a comprehensive demonstration of all TestVibes features.

---

*TestVibes - Testing with good vibes! 🧪✨*
