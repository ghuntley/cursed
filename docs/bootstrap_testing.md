# Bootstrap Compiler Testing System

This document describes the comprehensive testing pipeline for the CURSED language bootstrap compiler system.

## Overview

The bootstrap testing system validates that the CURSED compiler can successfully compile itself through multiple generations (Stage 1 → Stage 2 → Stage 3 → ...), ensuring self-hosting capabilities and detecting regressions in the bootstrap process.

## Test Architecture

### Test Categories

1. **Minimal Subset Tests** (`tests/bootstrap/minimal_subset.rs`)
   - Validates core language features required for bootstrapping
   - Tests arithmetic, control flow, functions, variables, structs, arrays, strings
   - Ensures the minimal subset is sufficient for compiler implementation

2. **Stage 2 Compiler Tests** (`tests/bootstrap/stage2_compiler.rs`)
   - Tests that Stage 2 compiler (written in CURSED) can be compiled by Stage 1
   - Validates Stage 2 compiler functionality: lexing, parsing, code generation
   - Tests error handling and output generation

3. **Self-Compilation Tests** (`tests/bootstrap/self_compilation.rs`)
   - Tests complete bootstrap cycles: Stage 1 → Stage 2 → Stage 3 → ...
   - Validates bootstrap convergence (stable results across generations)
   - Tests cross-compilation compatibility

4. **Performance Benchmarks** (`tests/bootstrap/performance_benchmarks.rs`)
   - Measures compilation time, memory usage, and binary size across stages
   - Compares performance between compiler generations
   - Tests throughput and optimization impact

5. **Regression Tests** (`tests/bootstrap/regression_tests.rs`)
   - Ensures previously working bootstrap functionality continues to work
   - Tests backwards compatibility with older language versions
   - Validates consistent error messages and feature flags

6. **CI/CD Integration Tests** (`tests/bootstrap/ci_integration.rs`)
   - Tests bootstrap in clean environments (containers, fresh installations)
   - Validates dependency isolation and cross-platform compatibility
   - Tests resource-constrained environments

7. **Memory Usage Tests** (`tests/bootstrap/memory_usage.rs`)
   - Monitors memory consumption during compilation
   - Detects memory leaks in the bootstrap process
   - Tests memory scaling with program complexity

## Test Infrastructure

### Common Utilities (`tests/bootstrap/utils.rs`)

- **Compilation Functions**: `compile_with_stage1()`, `execute_binary()`
- **Memory Monitoring**: `measure_memory_usage()`, `get_process_memory_usage()`
- **File Management**: `create_test_source()`, `get_file_size()`
- **Environment Setup**: `validate_bootstrap_environment()`

### Test Configuration (`tests/bootstrap/mod.rs`)

```rust
pub struct BootstrapTestConfig {
    pub stage1_binary: String,      // Path to Stage 1 compiler
    pub test_data_dir: String,      // Test source files directory
    pub output_dir: String,         // Compiled binaries directory
    pub timeout_seconds: u64,       // Test timeout
    pub enable_debug: bool,         // Debug mode
    pub enable_benchmarks: bool,    // Performance benchmarks
}
```

### Test Metrics

```rust
pub struct BootstrapTestMetrics {
    pub stage1_compile_time_ms: u64,    // Stage 1 compilation time
    pub stage2_compile_time_ms: u64,    // Stage 2 compilation time
    pub stage3_compile_time_ms: u64,    // Stage 3 compilation time
    pub memory_usage_mb: u64,           // Peak memory usage
    pub binary_size_bytes: u64,         // Generated binary size
    pub tests_passed: usize,            // Number of passed tests
    pub tests_failed: usize,            // Number of failed tests
}
```

## Running Tests

### Command Line Interface

The bootstrap test system provides multiple ways to run tests:

#### Using the Test Script

```bash
# Run full test suite
./scripts/run_bootstrap_tests.sh

# Run quick test suite (faster, subset of tests)
./scripts/run_bootstrap_tests.sh --quick

# Run specific test category
./scripts/run_bootstrap_tests.sh --category minimal_subset

# Run with verbose output
./scripts/run_bootstrap_tests.sh --verbose

# Clean outputs before running
./scripts/run_bootstrap_tests.sh --clean

# Generate report from existing results
./scripts/run_bootstrap_tests.sh --report-only
```

#### Using Make Targets

```bash
# Run comprehensive bootstrap test suite
make bootstrap-test

# Run quick bootstrap tests
make bootstrap-test-quick

# Run specific test category
make bootstrap-test-category CATEGORY=minimal_subset

# Generate test report
make bootstrap-test-report

# Clean test outputs
make bootstrap-test-clean

# Show help
make bootstrap-test-help
```

#### Using Cargo Directly

```bash
# Run all bootstrap tests
cargo test --test bootstrap_comprehensive_test

# Run specific test categories
cargo test --test bootstrap::minimal_subset
cargo test --test bootstrap::stage2_compiler
cargo test --test bootstrap::self_compilation

# Run with environment variables
RUST_LOG=debug cargo test bootstrap::performance_benchmarks
```

### Environment Variables

- `BOOTSTRAP_TEST_OUTPUT_DIR`: Override test output directory
- `BOOTSTRAP_CURSED_BINARY`: Override Stage 1 compiler path
- `RUST_LOG`: Set logging level (error, warn, info, debug, trace)
- `TEST_TIMEOUT`: Set test timeout in seconds
- `KEEP_OUTPUTS`: Keep test outputs after completion

## Test Reports

### Report Generation

The test system generates comprehensive reports in multiple formats:

- **Console Output**: Real-time test progress and results
- **Markdown Reports**: Detailed test results with metrics
- **JSON Logs**: Machine-readable test data
- **Artifacts**: Test binaries, source files, and debug information

### Report Structure

```
test_results/bootstrap/
├── logs/                    # Test execution logs
│   ├── minimal_subset.log
│   ├── stage2_compiler.log
│   └── ...
├── reports/                 # Generated reports
│   ├── bootstrap_test_report.md
│   └── coverage_report.md
└── artifacts/              # Test artifacts
    ├── binaries/
    ├── source_files/
    └── debug_info/
```

### Coverage Metrics

The test system tracks coverage across multiple dimensions:

- **Test Category Coverage**: Percentage of tests passing per category
- **Feature Coverage**: Language features tested in bootstrap
- **Platform Coverage**: Tested operating systems and architectures
- **Performance Coverage**: Performance characteristics validated

## CI/CD Integration

### GitHub Actions Workflow

The bootstrap tests are integrated into CI/CD via `.github/workflows/bootstrap-tests.yml`:

- **Matrix Testing**: Tests across multiple OS and configurations
- **Scheduled Runs**: Nightly comprehensive test execution
- **Performance Monitoring**: Continuous performance regression detection
- **Memory Leak Detection**: Automated memory analysis

### Workflow Triggers

- **Push Events**: Quick tests on main branch pushes
- **Pull Requests**: Full test suite on PR creation/updates
- **Scheduled**: Nightly comprehensive tests and performance benchmarks
- **Manual**: On-demand testing with configurable parameters

## Test Development

### Adding New Tests

1. **Identify Test Category**: Determine which category the test belongs to
2. **Create Test Function**: Add test function with `#[test]` attribute
3. **Use Test Infrastructure**: Leverage existing utilities and helpers
4. **Add Documentation**: Document test purpose and expected behavior
5. **Update Test Script**: Add test to the appropriate category runner

### Test Naming Conventions

- Test files: `test_<category>_<feature>.rs`
- Test functions: `test_<feature>_<scenario>()`
- Helper functions: `<action>_<object>()`
- Test data: `create_<scenario>_test_program()`

### Best Practices

1. **Isolation**: Each test should be independent and not affect others
2. **Cleanup**: Clean up temporary files and resources after tests
3. **Timeouts**: Set reasonable timeouts to prevent hanging tests
4. **Logging**: Use structured logging for debugging and analysis
5. **Error Handling**: Provide clear error messages for test failures

## Performance Considerations

### Test Execution Time

- **Quick Mode**: ~2-5 minutes (essential tests only)
- **Comprehensive Mode**: ~15-30 minutes (full test suite)
- **Performance Benchmarks**: ~5-10 minutes (detailed measurements)

### Resource Usage

- **Memory**: Tests monitor and limit memory usage (< 1GB typical)
- **Disk**: Test artifacts cleaned up automatically (configurable)
- **CPU**: Tests run in parallel where possible

### Optimization Strategies

- **Parallel Execution**: Independent tests run concurrently
- **Caching**: Compiled binaries cached between test runs
- **Incremental Testing**: Only run affected tests when possible

## Troubleshooting

### Common Issues

1. **Compilation Failures**
   - Check Stage 1 compiler is built and accessible
   - Verify LLVM dependencies are installed
   - Check system linker configuration

2. **Test Timeouts**
   - Increase timeout values for slow systems
   - Check for infinite loops in test programs
   - Monitor system resource availability

3. **Memory Issues**
   - Verify sufficient system memory available
   - Check for memory leaks in test programs
   - Monitor memory usage during test execution

4. **Environment Issues**
   - Validate test environment setup
   - Check file permissions and disk space
   - Verify network connectivity if required

### Debug Information

The test system provides extensive debug information:

- **Structured Logging**: Detailed execution traces
- **Test Artifacts**: All intermediate files preserved (optional)
- **Performance Metrics**: Detailed timing and resource usage
- **Error Context**: Rich error messages with context

### Support

For issues with the bootstrap testing system:

1. Check the troubleshooting section above
2. Review test logs in `test_results/bootstrap/logs/`
3. Run tests with `--verbose` flag for detailed output
4. Check CI/CD results for environment-specific issues
5. Report issues with complete log files and environment details

## Future Enhancements

### Planned Features

- **Cross-Compilation Testing**: Test bootstrap across different target architectures
- **Fuzzing Integration**: Automated test case generation for edge cases
- **Performance Regression Detection**: Automated alerts for performance degradation
- **Visual Test Reports**: Web-based test result visualization
- **Test Parallelization**: Improved parallel test execution

### Extensibility

The bootstrap testing system is designed to be extensible:

- **Plugin Architecture**: Easy addition of new test categories
- **Custom Metrics**: Framework for additional performance measurements
- **External Integration**: APIs for integration with external tools
- **Configuration Management**: Flexible test configuration system

This comprehensive testing system ensures the reliability and correctness of the CURSED language bootstrap compiler, providing confidence in self-hosting capabilities and catching regressions early in the development process.
