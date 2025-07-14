# CURSED Self-Hosting CI Documentation

## Overview

The CURSED Self-Hosting CI Pipeline ensures that the compiler can reliably compile itself and maintain production quality. This comprehensive validation system tests the "compiler compiles compiler" capability and catches regressions before they reach production.

## Components

### 1. Self-Hosting Validation (`ci/self_hosting_validation.sh`)

**Purpose**: Tests that the compiler can compile itself and produce identical outputs.

**Key Features**:
- Validates compiler-compiles-compiler capability
- Compares output consistency between interpretation and compilation modes
- Tests basic self-hosting functionality
- Performance benchmarking of compilation process

**Test Scenarios**:
- Bootstrap compilation test
- Self-hosting compiler simulation
- Output identity verification
- Performance regression detection

### 2. Bootstrap Validation Tests (`ci/bootstrap_validation_tests.sh`)

**Purpose**: Comprehensive testing of bootstrap compilation process.

**Test Suites**:
- Basic Language Features (variables, arithmetic, boolean logic, arrays)
- Function Definitions (simple functions, recursive functions)
- Control Flow (if-else chains, for loops)
- Type System (type conversions, tuple operations)
- Module System (core module integration)
- Error Handling (basic error recovery)
- Advanced Features (short declarations, increment/decrement)
- Stdlib Integration (string operations)

**Validation Process**:
- Each test runs in both interpretation and compilation modes
- Outputs are compared for consistency
- Performance metrics are collected
- Detailed reporting of pass/fail status

### 3. Performance Regression Detection (`ci/performance_regression_detection.sh`)

**Purpose**: Monitors compilation and execution performance over time.

**Benchmark Tests**:
- Arithmetic operations benchmark
- Function calls benchmark
- Array operations benchmark
- String operations benchmark
- Recursive function benchmark
- Complex computation benchmark

**Regression Detection**:
- Compares current performance with baseline
- Configurable regression threshold (default: 1.5x)
- Tracks compilation time, execution time, and memory usage
- Generates performance reports in JSON format

**Baseline Management**:
- Maintains performance baseline in `ci/performance_baseline.json`
- Updates baseline automatically when no regressions detected
- Provides historical performance tracking

### 4. Comprehensive Self-Hosting Test Suite (`ci/comprehensive_self_hosting_test_suite.sh`)

**Purpose**: Extensive testing of all self-hosting scenarios.

**Test Suites**:
- Basic Self-Hosting Capability
- Advanced Language Features
- Module System Integration
- Error Handling and Edge Cases
- Performance and Scalability
- Self-Hosting Bootstrap
- Cross-Platform Compatibility

**Features**:
- Parallel test execution support
- Comprehensive language feature coverage
- Performance metrics collection
- Detailed reporting and analysis

## Configuration

### Environment Variables

- `COMPILER_BINARY`: Path to the compiler executable (default: `target/release/cursed`)
- `TIMEOUT_SECONDS`: Test timeout in seconds (default: 300)
- `REGRESSION_THRESHOLD`: Performance regression threshold (default: 1.5)
- `PARALLEL_JOBS`: Number of parallel test jobs (default: 4)

### CI Integration

The CI pipeline integrates with Cirrus CI through the following steps:

1. **Build Phase**: Compile the CURSED compiler
2. **Test Phase**: Run standard unit tests
3. **Self-Hosting Validation**: Execute all self-hosting validation scripts
4. **Artifact Collection**: Collect performance reports and validation results
5. **Regression Analysis**: Compare with baseline performance metrics

## Usage

### Local Testing

```bash
# Setup CI environment
bash ci/setup_self_hosting_ci.sh

# Run full local validation
bash ci/test_self_hosting_locally.sh

# Run individual components
bash ci/self_hosting_validation.sh
bash ci/bootstrap_validation_tests.sh
bash ci/performance_regression_detection.sh
bash ci/comprehensive_self_hosting_test_suite.sh
```

### CI Integration

Add the following to your `.cirrus.yml` file:

```yaml
self_hosting_validation_script: |
  . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
  devenv shell bash ci/self_hosting_validation.sh
  
bootstrap_validation_script: |
  . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
  devenv shell bash ci/bootstrap_validation_tests.sh
  
performance_regression_script: |
  . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
  devenv shell bash ci/performance_regression_detection.sh
  
comprehensive_self_hosting_script: |
  . /nix/var/nix/profiles/default/etc/profile.d/nix-daemon.sh
  devenv shell bash ci/comprehensive_self_hosting_test_suite.sh
```

## Outputs and Reports

### Performance Report (`performance_report.json`)

```json
{
  "report_timestamp": "2025-01-01T00:00:00Z",
  "benchmarks": {
    "arithmetic_benchmark": {
      "compile_time": 1.0,
      "execution_time": 0.1,
      "memory_usage": "1024"
    }
  },
  "summary": {
    "total_benchmarks": 6,
    "successful_benchmarks": 6,
    "failed_benchmarks": 0,
    "regressions_detected": 0,
    "average_compile_time": 1.183,
    "average_execution_time": 0.295
  }
}
```

### Validation Report (`self_hosting_validation_report.json`)

```json
{
  "timestamp": "2025-01-01T00:00:00Z",
  "build_id": "123456",
  "commit": "abc123",
  "validation_status": "PASSED",
  "tests": {
    "self_hosting_validation": "PASSED",
    "bootstrap_validation": "PASSED",
    "performance_regression": "PASSED",
    "comprehensive_self_hosting": "PASSED"
  }
}
```

## Exit Codes

- `0`: All tests passed
- `1`: Test failures detected
- `2`: Performance regressions detected
- `3`: Configuration or setup errors

## Troubleshooting

### Common Issues

1. **Timeout Errors**: Increase `TIMEOUT_SECONDS` for slower environments
2. **Performance Regressions**: Check if actual performance degradation or increase `REGRESSION_THRESHOLD`
3. **Missing Dependencies**: Ensure `bc` and `jq` are installed
4. **Compilation Failures**: Verify compiler builds successfully with `cargo build --release`

### Debug Mode

Enable debug output by setting:
```bash
export CURSED_DEBUG=1
```

### Log Analysis

Check generated log files:
- `*_interp.out`: Interpretation mode outputs
- `*_compiled.out`: Compilation mode outputs
- `*_memory.txt`: Memory usage reports
- `*_results.json`: Individual test results

## Best Practices

1. **Run locally first**: Always test locally before CI
2. **Monitor baselines**: Review performance baseline updates
3. **Investigate regressions**: Don't ignore performance warnings
4. **Update documentation**: Keep this documentation current

## Maintenance

### Updating Baselines

Performance baselines are automatically updated when no regressions are detected. To manually update:

```bash
# After running performance tests
cp performance_report.json ci/performance_baseline.json
```

### Adding New Tests

To add new self-hosting tests:

1. Add test code to appropriate test suite function
2. Update test count tracking
3. Add validation logic
4. Update documentation

### Script Modifications

When modifying CI scripts:

1. Test locally with `bash ci/test_self_hosting_locally.sh`
2. Validate syntax with `bash -n script.sh`
3. Check timeout values for new tests
4. Update this documentation

## Security Considerations

- All test programs are created dynamically and cleaned up
- No sensitive information is logged
- Test directories are isolated and temporary
- Performance data doesn't include sensitive compiler internals

## Integration with Existing CI

The self-hosting CI pipeline is designed to integrate seamlessly with existing CI workflows:

- Runs after standard unit tests
- Provides additional validation layer
- Maintains backward compatibility
- Generates machine-readable reports

## Future Enhancements

Planned improvements:
- Cross-platform testing automation
- Advanced performance analytics
- Integration with external monitoring systems
- Automated baseline optimization

---

For support or questions, refer to the CURSED project documentation or CI pipeline logs.
