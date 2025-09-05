# Performance Testing Framework

A comprehensive performance testing framework for CURSED applications, providing load testing, stress testing, memory leak detection, and throughput analysis capabilities.

## Overview

The performance testing framework offers a complete suite of performance testing primitives designed to validate application performance under various conditions. All implementations are pure CURSED with no FFI dependencies.

## Core Functions

### Load Testing
```cursed
yeet "performance_testing"
sus result lit = performance_testing.load_test("my_function", 10, 30)
```
- **Purpose**: Test system behavior under expected load
- **Parameters**: target_function, concurrent_users, duration_seconds
- **Returns**: Boolean indicating test success

### Stress Testing
```cursed
sus result lit = performance_testing.stress_test("my_function", 100)
```
- **Purpose**: Find the breaking point of your system
- **Parameters**: function_name, max_load
- **Returns**: Boolean indicating successful stress testing

### Memory Leak Detection
```cursed
sus result lit = performance_testing.memory_leak_test("my_function", 10000)
```
- **Purpose**: Detect memory leaks over extended operation
- **Parameters**: function_name, iterations
- **Returns**: Boolean indicating no significant memory growth

### Throughput Testing
```cursed
sus result lit = performance_testing.throughput_test("my_function", 1000.0)
```
- **Purpose**: Validate system meets throughput requirements
- **Parameters**: function_name, expected_transactions_per_second
- **Returns**: Boolean indicating throughput requirements met

## Performance Measurement Utilities

### Function Benchmarking
```cursed
sus avg_time drip = performance_testing.benchmark_function("my_function", 1000)
```
Returns average execution time in milliseconds.

### Percentile Analysis
```cursed
performance_testing.percentile_analysis("my_function", 5000)
```
Provides P50, P95, and P99 latency measurements.

### Performance Comparison
```cursed
performance_testing.compare_performance("function_a", "function_b", 1000)
```
Compares relative performance of two functions.

## Features

### Concurrent Load Testing
- Spawns multiple goroutines to simulate concurrent users
- Coordinated start/stop using channels
- Real-time metrics collection
- Configurable test duration

### Stress Testing with Ramp-up
- Gradually increases load until system fails
- Monitors response time degradation
- Identifies maximum sustainable load
- Automatic threshold detection

### Memory Leak Detection
- Monitors memory usage over time
- Configurable growth thresholds
- Progress tracking for long tests
- Statistical analysis of memory patterns

### Throughput Validation
- Sustained load testing
- Real-time TPS calculation
- Performance percentage reporting
- Configurable success criteria

### Advanced Analytics
- Percentile-based latency analysis
- Min/max/average timing
- Performance comparison utilities
- Comprehensive reporting

## Integration with testz

The framework integrates seamlessly with the testz testing framework:

```cursed
yeet "testz"
yeet "performance_testing"

test_start("API performance test")
sus result lit = performance_testing.load_test("api_endpoint", 50, 60)
assert_true(result)
print_test_summary()
```

## Example Usage

### Basic Load Test
```cursed
# Test with 10 concurrent users for 30 seconds
sus load_result lit = performance_testing.load_test("user_login", 10, 30)
lowkey load_result {
    vibez.spill("Load test passed")
} fam {
    vibez.spill("Load test failed")
}
```

### Comprehensive Performance Suite
```cursed
# Run complete performance validation
sus load_ok lit = performance_testing.load_test("api_call", 20, 60)
sus stress_ok lit = performance_testing.stress_test("api_call", 100)
sus memory_ok lit = performance_testing.memory_leak_test("api_call", 10000)
sus throughput_ok lit = performance_testing.throughput_test("api_call", 500.0)

lowkey load_ok && stress_ok && memory_ok && throughput_ok {
    vibez.spill("All performance tests passed")
} fam {
    vibez.spill("Performance issues detected")
}
```

### Benchmarking and Analysis
```cursed
# Detailed performance analysis
sus avg_time drip = performance_testing.benchmark_function("data_processing", 1000)
performance_testing.percentile_analysis("data_processing", 5000)
performance_testing.compare_performance("old_algorithm", "new_algorithm", 1000)
```

## Function Registry

The framework includes a built-in function registry for testing:

- `fast_function`: 0.1ms execution time, 100% success rate
- `slow_function`: 10ms execution time, 100% success rate  
- `unreliable_function`: Variable execution time, 80% success rate

For production use, extend `execute_function_safely()` to support your application's functions.

## Performance Metrics

### Load Test Metrics
- Total requests processed
- Success/failure counts
- Transactions per second (TPS)
- Success rate percentage
- Test duration

### Stress Test Metrics
- Maximum sustainable load
- Breaking point identification
- Response time degradation
- Load ramp-up progression

### Memory Test Metrics
- Initial vs final memory usage
- Peak memory consumption
- Memory growth rate
- Growth percentage

### Throughput Metrics
- Expected vs actual TPS
- Performance percentage
- Operation counts
- Duration measurements

## Configuration

### Test Thresholds
- Memory growth threshold: 50% during test
- Memory leak threshold: 10% total growth
- Stress test response time: 5 second limit
- Throughput success criteria: 80% of expected

### Timing Configuration
- Load test worker delay: 1ms between operations
- Memory check interval: Every 100 iterations
- Progress reporting: Every 1000 operations
- Default stress test duration: 5 seconds

## Best Practices

1. **Start with load testing** to establish baseline performance
2. **Use stress testing** to find system limits
3. **Run memory leak tests** for long-running applications
4. **Validate throughput** against business requirements
5. **Use percentile analysis** for latency-sensitive applications
6. **Compare performance** when optimizing code
7. **Integrate with CI/CD** for continuous performance monitoring

## Testing

Run the test suite to verify framework functionality:

```bash
cargo run --bin cursed stdlib/performance_testing/test_performance_testing.💀
```

The test suite includes:
- Basic functionality tests for all core functions
- Edge case validation
- Integration testing with testz framework
- Comprehensive performance test scenarios

## Architecture

### Pure CURSED Implementation
- Zero FFI dependencies
- Native CURSED concurrency using goroutines and channels
- Integrated timing using timez module
- Mathematical operations using mathz module

### Modular Design
- Separate functions for each test type
- Reusable measurement utilities
- Configurable test parameters
- Extensible function registry

### Performance Optimized
- Efficient memory usage during testing
- Minimal overhead measurement
- Parallel execution support
- Real-time metrics collection

The performance testing framework provides enterprise-grade performance validation capabilities while maintaining the pure CURSED philosophy of zero external dependencies.
