# CURSED Bootstrap Verification System

This document explains the comprehensive bootstrap verification system for the CURSED programming language compiler, which validates the compiler's self-hosting capabilities.

## Overview

The bootstrap verification system implements the 4-stage bootstrap process defined in the compiler specifications to ensure that the CURSED compiler can successfully compile itself and produce equivalent output to the Rust-based implementation.

## Architecture

The verification system consists of several key components:

1. **Core Verification Engine** (`src/bootstrap/self_compilation_verification.rs`)
2. **Command-Line Tool** (`src/bin/bootstrap_verify.rs`)
3. **Integration Script** (`run_bootstrap_verification.sh`)
4. **Comprehensive Test Suite** (`tests/bootstrap_verification_test.rs`)

## Bootstrap Stages

The verification system validates the following stages:

### Stage 0: Bootstrap Environment Setup
- Project structure validation
- Build system verification
- Core utilities availability

### Stage 1: Minimal Bootstrap Compiler (Rust-based)
- Builds the Rust-based CURSED compiler
- Validates basic functionality
- Creates baseline for comparison

### Stage 2: Full Compiler in CURSED
- Uses Stage 1 to compile a CURSED-based compiler
- Validates functional equivalence with Stage 1
- Tests expanded language features

### Stage 3: Self-Compiled Full Compiler
- Uses Stage 2 to compile itself
- Validates convergence through multiple cycles
- Ensures stable self-hosting behavior

## Verification Process

### Multi-Stage Compilation Testing

The verification process includes:

1. **Stage Validation**: Each stage is compiled and validated independently
2. **Functional Equivalence**: Test programs are compiled with different stages and outputs compared
3. **Bootstrap Cycles**: Multiple self-compilation cycles to test convergence
4. **Performance Analysis**: Compilation times and binary sizes are tracked

### Test Programs

The system uses several test programs to validate functional equivalence:

- **Arithmetic Test**: Basic mathematical operations
- **String Operations Test**: String manipulation and formatting
- **Control Flow Test**: Loops, conditionals, and branching

### Convergence Detection

The system detects convergence through:

- **Binary Stability**: Checksums of compiled binaries across cycles
- **Performance Stability**: Compilation time variance analysis
- **Error Consistency**: Error handling behavior across stages

## Configuration

### VerificationConfig

```rust
pub struct VerificationConfig {
    pub work_dir: PathBuf,              // Working directory
    pub compilation_timeout: Duration,  // Compilation timeout
    pub execution_timeout: Duration,    // Execution timeout
    pub keep_intermediates: bool,       // Preserve debug files
    pub optimization_levels: Vec<String>, // Optimization levels to test
    pub bootstrap_cycles: usize,        // Number of bootstrap cycles
}
```

### Default Settings

- Working Directory: `./bootstrap_verification`
- Compilation Timeout: 300 seconds
- Execution Timeout: 60 seconds
- Bootstrap Cycles: 3
- Optimization Levels: `-O0`, `-O2`

## Usage

### Command Line Tool

```bash
# Basic verification
bootstrap-verify

# Quick verification (2 cycles)
bootstrap-verify --quick

# Verbose output
bootstrap-verify --verbose

# Preserve debugging files
bootstrap-verify --keep-intermediates

# Custom configuration
bootstrap-verify --work-dir ./custom_dir --cycles 5 --timeout 600
```

### Shell Script

```bash
# Basic verification
./run_bootstrap_verification.sh

# Quick verification
./run_bootstrap_verification.sh --quick

# Verbose output
./run_bootstrap_verification.sh --verbose

# Keep intermediate files
./run_bootstrap_verification.sh --keep
```

### Makefile Targets

```bash
# Complete verification
make bootstrap-verify

# Quick verification
make bootstrap-verify-quick

# Verbose verification
make bootstrap-verify-verbose

# Debug mode (preserve files)
make bootstrap-verify-debug

# Run test suite
make bootstrap-verify-test

# Build verification tool
make bootstrap-verify-build
```

## Output and Reporting

### Console Output

The verification system provides real-time progress updates:

```
🚀 CURSED Bootstrap Verification System
======================================
🔧 Building bootstrap verification tool...
✅ Bootstrap verification tool built successfully
🧹 Ensuring clean build of Stage 1 (Rust) compiler...
✅ Stage 1 compiler built successfully

🔍 Starting bootstrap verification process...
🔧 Stage 1: Building Rust-based CURSED compiler...
✅ Stage 1 completed successfully in 8.45s
🔧 Stage 2: Building CURSED-based compiler using Stage 1...
✅ Stage 2 completed successfully in 12.34s
🔍 Testing functional equivalence between compiler stages...
  ✅ Outputs match for program: arithmetic
  ✅ Outputs match for program: strings
  ✅ Outputs match for program: control_flow
🔄 Testing bootstrap convergence (3 cycles)...
  ✅ Binary convergence achieved at cycle 2

📊 Verification Summary
======================
✅ Bootstrap verification PASSED
📄 Full report available at: ./reports/bootstrap_verification_report.md
```

### Markdown Reports

Detailed reports are generated in Markdown format:

```markdown
# CURSED Bootstrap Verification Report

**Generated:** 2024-03-14 15:30:45 UTC
**Overall Success:** ✅ PASSED
**Verification Time:** 25.67 seconds
**Stages Completed:** 2

## Stage Results

### Stage 1 - ✅ SUCCESS
- **Compilation Time:** 8.45s
- **Execution Time:** 1.23s
- **Binary Checksum:** abc123def456

### Stage 2 - ✅ SUCCESS
- **Compilation Time:** 12.34s
- **Execution Time:** 2.45s
- **Binary Checksum:** def456abc123

## Performance Analysis
- **Average Compilation Time:** 10.40s
- **Average Binary Size:** 2,145,678 bytes

## Convergence Analysis
- **Binary Stability:** ✅ Achieved
- **Performance Stability:** ✅ Stable
- **Convergence Cycle:** 2

## Recommendations
✅ The bootstrap verification passed successfully.
```

## Error Handling

### Common Issues

1. **Stage 1 Compilation Failure**
   - Ensure Rust compiler is available
   - Check CURSED project build dependencies
   - Verify system requirements

2. **Stage 2 Not Implemented**
   - Currently simulated as CURSED-based compiler is in development
   - Future versions will use actual CURSED implementation

3. **Timeout Errors**
   - Increase timeout values with `--timeout` flag
   - Check system performance and available resources

4. **Permission Errors**
   - Ensure write permissions to working directory
   - Check file system space availability

### Debugging

Use the following options for debugging:

- `--verbose`: Detailed output and progress information
- `--keep-intermediates`: Preserve all intermediate files
- Working directory: `./bootstrap_verification` (or custom)

## Performance Characteristics

### Expected Performance

- **Stage 1 Compilation**: 5-15 seconds (depends on system)
- **Stage 2 Compilation**: 10-30 seconds (when implemented)
- **Total Verification**: 30-90 seconds for full process
- **Memory Usage**: ~100-500MB peak during compilation

### Optimization

The system is optimized for:

- **Minimal overhead**: Only essential operations performed
- **Parallel processing**: Where possible, operations run concurrently
- **Resource management**: Automatic cleanup of temporary files
- **Incremental processing**: Reuse of intermediate results where safe

## Integration with CI/CD

### Exit Codes

- `0`: Verification passed successfully
- `1`: Verification failed (issues found)
- `2`: System error (unable to run verification)

### CI Configuration Example

```yaml
name: Bootstrap Verification
on: [push, pull_request]
jobs:
  bootstrap:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install dependencies
        run: |
          # Install CURSED dependencies
      - name: Run bootstrap verification
        run: make bootstrap-verify
      - name: Upload report
        uses: actions/upload-artifact@v3
        with:
          name: bootstrap-report
          path: reports/bootstrap_verification_report.md
```

## Future Enhancements

### Planned Features

1. **Stage 2 Implementation**: Real CURSED-based compiler compilation
2. **Performance Benchmarks**: Detailed performance analysis and regression detection
3. **Cross-Platform Testing**: Validation across different operating systems
4. **Parallel Verification**: Concurrent stage testing for faster feedback
5. **Integration Testing**: Validation with real-world CURSED programs

### Extensibility

The verification system is designed to be extensible:

- **Custom Test Programs**: Add domain-specific test cases
- **Additional Metrics**: Extend performance and quality measurements
- **Plugin Architecture**: Support for custom verification steps
- **Report Formats**: Additional output formats (JSON, XML, etc.)

## Contributing

### Adding Test Cases

To add new test programs for equivalence testing:

1. Add test program creation in `create_test_programs()`
2. Ensure test covers specific language features
3. Add corresponding test cases in the test suite

### Extending Metrics

To add new performance or quality metrics:

1. Extend `PerformanceMetrics` structure
2. Add collection logic in `collect_performance_metrics()`
3. Update report generation to include new metrics

### Configuration Options

To add new configuration options:

1. Extend `VerificationConfig` structure
2. Add command-line argument parsing
3. Update documentation and help text

## Troubleshooting

### Common Solutions

1. **Build Failures**: Ensure all dependencies are installed and up-to-date
2. **Timeout Issues**: Increase timeout values or check system performance
3. **Permission Issues**: Verify write access to working directory
4. **Memory Issues**: Close other applications or increase available memory

### Getting Help

- Review this documentation for common issues
- Check the generated verification report for detailed error information
- Run with `--verbose` flag for additional debugging information
- Examine intermediate files when using `--keep-intermediates`

For additional support, consult the CURSED project documentation or file an issue in the project repository.
