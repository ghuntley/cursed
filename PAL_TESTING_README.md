# CURSED PAL Testing Suite

This directory contains the comprehensive testing suite for the CURSED Platform Abstraction Layer (PAL) system. The PAL provides cross-platform runtime optimizations for memory management, goroutine scheduling, and hardware feature detection.

## 📁 Test Files

### Core Test Files

| Test File | Purpose | Platform |
|-----------|---------|----------|
| `test_feature_detection.csd` | Platform capability detection | All |
| `test_memory_pal.csd` | Memory allocation optimizations | All |
| `test_memory_alignment.csd` | Platform-specific alignment | All |
| `test_large_pages.csd` | Large page support | Linux/Windows |
| `test_scheduler_pal.csd` | Goroutine scheduler performance | All |
| `test_apple_silicon_cores.csd` | P+E core scheduling | macOS ARM64 |
| `test_numa_scheduling.csd` | NUMA-aware scheduling | Linux multi-socket |
| `test_wasm_memory.csd` | WebAssembly linear memory | WASM |
| `test_wasm_scheduling.csd` | Cooperative scheduling | WASM |
| `test_simd_features.csd` | SIMD instruction utilization | All |
| `test_crypto_acceleration.csd` | Hardware crypto acceleration | All |
| `test_memory_stress.csd` | Memory allocation stress test | All |
| `test_scheduler_stress.csd` | Scheduler stress test | All |
| `benchmark_pal_performance.csd` | Performance benchmarking | All |

### Shell Scripts

| Script | Purpose |
|--------|---------|
| `run_pal_tests.sh` | Main test runner |
| `test_cross_compilation.sh` | Cross-platform compilation tests |
| `create_platform_benchmark.sh` | Platform benchmark creation |
| `validate_pal_integration.sh` | PAL system integration validation |
| `run_comprehensive_pal_tests.sh` | Complete testing pipeline |

### Analysis Tools

| Tool | Purpose |
|------|---------|
| `analyze_pal_results.py` | Test results analysis and validation |

## 🚀 Quick Start

### Run All Tests
```bash
./run_comprehensive_pal_tests.sh
```

### Run Specific Test Phases
```bash
# Platform detection only
cargo run --bin cursed test_feature_detection.csd

# Memory management tests
./run_pal_tests.sh

# Cross-compilation validation
./test_cross_compilation.sh
```

### Create Platform Benchmark
```bash
./create_platform_benchmark.sh
```

## 📊 Performance Targets

The test suite validates against these performance baselines:

| Platform | Memory Ops/sec | Goroutine Spawn/sec | Notes |
|----------|---------------|-------------------|-------|
| ARM64 macOS | > 1M | > 100K | Apple Silicon optimized |
| ARM64 Linux | > 800K | > 80K | Standard ARM64 |
| x86_64 macOS | > 1.2M | > 120K | AVX optimizations |
| x86_64 Linux | > 1.5M | > 150K | Huge pages + NUMA |
| x86_64 Windows | > 1M | > 100K | Windows heap optimized |
| WASM32 | > 100K | > 10K | Limited by WASM constraints |

## 🔧 Requirements

### System Dependencies
- Rust toolchain with cross-compilation targets
- Python 3.6+ (for analysis tools)
- `wasmtime` (for WebAssembly tests)

### Install Cross-Compilation Targets
```bash
rustup target add aarch64-apple-darwin
rustup target add aarch64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-msvc
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
```

### Install WebAssembly Runtime
```bash
cargo install wasmtime-cli
```

## 📋 Test Framework Integration

All tests use the CURSED `testz` framework:

```cursed
yeet "testz"

test_start("Test Name")

// Test implementation
assert_true(condition)
assert_eq_int(actual, expected)
assert_eq_string(actual, expected)

print_test_summary()
```

## 🎯 Expected Results

### Successful Test Run Output
```
🧪 CURSED PAL Testing Suite
==========================
📍 Phase 1: Platform Detection Tests
✅ Feature detection test passed
🧠 Phase 2: Memory Management Tests
✅ test_memory_pal.csd passed
✅ test_memory_alignment.csd passed
⚡ Phase 3: Scheduler Optimization Tests
✅ test_scheduler_pal.csd passed
🔧 Phase 5: Hardware Feature Tests
✅ test_simd_features.csd passed
💪 Phase 6: Stress Tests
✅ test_memory_stress.csd passed
📊 Phase 7: Performance Benchmarks
✅ Performance benchmark completed
🎉 All tests passed!
```

### Platform Detection Output
```
CURSED v0.1.0 - Platform Information
====================================
Architecture: arm64
Operating System: macOS
Platform: ARM64 macOS (Apple Silicon)
Hardware Concurrency: 8 cores (4P+4E)
Page Size: 16KB
Default Stack Size: 1MB
Memory Alignment: 16 bytes

Hardware Features Detected:
✅ NEON SIMD instructions
✅ AES hardware acceleration
✅ SHA hardware acceleration  
✅ Large page support
```

## 🐛 Troubleshooting

### Common Issues

1. **Target not installed**
   ```bash
   rustup target add <target>
   ```

2. **WASM tests failing**
   ```bash
   cargo install wasmtime-cli
   ```

3. **Permission denied for large pages**
   - Linux: `sudo sysctl vm.nr_hugepages=128`
   - Windows: Enable "Lock pages in memory" privilege

4. **Feature detection issues**
   - Check hardware support and OS version
   - Enable debug logging: `export RUST_LOG=debug`

### Debug Mode
```bash
export RUST_LOG=cursed::runtime::pal=debug
cargo run --bin cursed test_program.csd
```

## 📈 Results Analysis

The test suite automatically analyzes results and validates performance:

```bash
python3 analyze_pal_results.py pal_test_results.log
```

This generates:
- Performance comparison against targets
- Platform-specific optimization recommendations
- Detailed metrics and error analysis
- JSON results for automated processing

## 🔍 Continuous Integration

For CI/CD integration:

```bash
# Exit codes:
# 0 = All tests passed, performance targets met
# 1 = Some tests failed, but majority passed
# 2 = Significant failures detected

./run_comprehensive_pal_tests.sh
exit_code=$?

if [ $exit_code -eq 0 ]; then
    echo "✅ PAL system validated - ready for production"
elif [ $exit_code -eq 1 ]; then
    echo "⚠️  PAL system has minor issues - review recommended"
else
    echo "❌ PAL system has critical issues - fixes required"
    exit 1
fi
```

## 📚 Additional Resources

- [TEST_PAL.md](TEST_PAL.md) - Detailed testing specification
- [src/runtime/pal/mod.rs](src/runtime/pal/mod.rs) - PAL implementation
- [stdlib/testz/mod.csd](stdlib/testz/mod.csd) - Testing framework
- [AGENT.md](AGENT.md) - Development commands and workflow

## 🤝 Contributing

When adding new PAL features or platforms:

1. Add corresponding test files following the naming convention
2. Update performance targets in `analyze_pal_results.py`
3. Add platform-specific tests to the appropriate scripts
4. Update this README with new test documentation
5. Validate all tests pass on the new platform

## ⚡ Performance Tips

- Run tests on dedicated hardware for accurate benchmarking
- Disable other applications during performance testing
- Use consistent compiler flags across test runs
- Monitor system resources during stress tests
- Compare results across multiple runs for stability
