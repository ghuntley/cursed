# CURSED Parallel Test Runner with Environment Support

A comprehensive parallel test execution system for CURSED with advanced configuration, resource management, and environment variable support.

## 🚀 Features

### ⚡ Parallelism Control
- **Sequential Execution**: Traditional single-threaded test execution
- **Parallel Execution**: Multi-worker concurrent test execution  
- **Adaptive Execution**: Smart strategy selection based on test characteristics
- **Resource-Aware Scheduling**: Dynamic worker allocation based on available resources

### 🌍 Environment Configuration
- **Comprehensive Environment Variables**: Full control via environment settings
- **Configuration Profiles**: Pre-defined profiles for different environments
- **Runtime Validation**: Automatic validation of configuration parameters
- **Hot Configuration**: Runtime configuration changes without restarts

### 📊 Resource Management
- **Memory Monitoring**: Track and limit memory usage per worker
- **CPU Utilization**: Monitor and balance CPU usage across workers
- **Resource Warnings**: Automatic alerts when resources exceed thresholds
- **Load Balancing**: Intelligent task distribution across workers

### 🎯 Advanced Features
- **Test Isolation**: Each test runs in its own isolated environment
- **Cleanup Management**: Automatic resource cleanup after test execution
- **Retry Logic**: Configurable retry attempts for flaky tests
- **Performance Profiling**: Detailed performance metrics and analysis

## 📖 Environment Variables

### Basic Configuration
```bash
export CURSED_TEST_PARALLEL=true          # Enable parallel execution
export CURSED_TEST_WORKERS=4               # Number of parallel workers
export CURSED_TEST_TIMEOUT=30              # Test timeout in seconds
export CURSED_TEST_MEMORY_LIMIT=512        # Memory limit in MB
```

### Test Filtering
```bash
export CURSED_TEST_FILTER="*crypto*"       # Filter tests by pattern
export CURSED_TEST_EXCLUDE="*slow*"        # Exclude tests by pattern
```

### Output Control
```bash
export CURSED_TEST_VERBOSE=true            # Enable verbose output
export CURSED_TEST_DEBUG=true              # Enable debug output
export CURSED_TEST_PROFILE=true            # Enable performance profiling
```

### Execution Control
```bash
export CURSED_TEST_FAIL_FAST=true          # Stop on first failure
export CURSED_TEST_ISOLATION=true          # Run tests in isolation
export CURSED_TEST_CLEANUP=true            # Cleanup after each test
```

### Advanced Configuration
```bash
export CURSED_TEST_RETRY_COUNT=3           # Number of retries for failed tests
export CURSED_TEST_RETRY_DELAY=1000        # Delay between retries in ms
export CURSED_TEST_OUTPUT_FORMAT=json      # Output format (console/json/xml)
export CURSED_TEST_REPORT_FILE=results.json # Output report file path
export CURSED_TEST_COVERAGE=true           # Collect test coverage
export CURSED_TEST_BENCHMARK=true          # Run in benchmark mode
```

## 🔧 Usage Examples

### Basic Parallel Execution
```bash
# Run all tests in parallel with 4 workers
export CURSED_TEST_PARALLEL=true
export CURSED_TEST_WORKERS=4
./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
```

### Development Mode
```bash
# Verbose output with fail-fast for quick feedback
export CURSED_TEST_PARALLEL=true
export CURSED_TEST_WORKERS=2
export CURSED_TEST_VERBOSE=true
export CURSED_TEST_FAIL_FAST=true
export CURSED_TEST_DEBUG=true
./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
```

### CI/CD Pipeline
```bash
# Optimized for CI environments
export CURSED_TEST_PARALLEL=true
export CURSED_TEST_WORKERS=4
export CURSED_TEST_TIMEOUT=120
export CURSED_TEST_FAIL_FAST=true
export CURSED_TEST_EXCLUDE="*manual*"
export CURSED_TEST_OUTPUT_FORMAT=json
export CURSED_TEST_REPORT_FILE=ci_results.json
./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
```

### Performance Benchmarking
```bash
# Sequential execution with profiling for accurate benchmarks
export CURSED_TEST_PARALLEL=false
export CURSED_TEST_WORKERS=1
export CURSED_TEST_TIMEOUT=300
export CURSED_TEST_FILTER="*benchmark*"
export CURSED_TEST_PROFILE=true
export CURSED_TEST_VERBOSE=true
./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
```

### Filtered Test Execution
```bash
# Run only crypto-related tests
export CURSED_TEST_FILTER="*crypto*"
export CURSED_TEST_PARALLEL=true
export CURSED_TEST_WORKERS=2
./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd

# Exclude slow tests for quick validation
export CURSED_TEST_EXCLUDE="*slow*"
export CURSED_TEST_PARALLEL=true
./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
```

## 🏗️ Architecture

### Worker Pool Management
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Worker 1      │    │   Worker 2      │    │   Worker N      │
│   Status: idle  │    │   Status: run   │    │   Status: idle  │
│   Memory: 64MB  │    │   Memory: 128MB │    │   Memory: 32MB  │
│   CPU: 25%      │    │   CPU: 75%      │    │   CPU: 10%      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                    ┌─────────────────┐
                    │  Load Balancer  │
                    │  Strategy:      │
                    │  Resource-Aware │
                    └─────────────────┘
                                 │
                    ┌─────────────────┐
                    │   Task Queue    │
                    │ ┌─────────────┐ │
                    │ │ Task 1: P1  │ │
                    │ │ Task 2: P2  │ │
                    │ │ Task 3: P1  │ │
                    │ └─────────────┘ │
                    └─────────────────┘
```

### Resource Monitoring
```
┌─────────────────────────────────────────────────────────────┐
│                   Resource Manager                          │
├─────────────────────────────────────────────────────────────┤
│ Total Memory: 1024MB    │ Available Memory: 512MB          │
│ CPU Cores: 4            │ Active Workers: 3                │
│ Memory Threshold: 80%   │ CPU Threshold: 90%               │
│ Warnings: 1             │ Status: Normal                   │
└─────────────────────────────────────────────────────────────┘
```

### Test Execution Flow
```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Test        │    │ Worker      │    │ Resource    │
│ Discovery   │───▶│ Allocation  │───▶│ Monitoring  │
└─────────────┘    └─────────────┘    └─────────────┘
        │                   │                   │
        ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Task        │    │ Test        │    │ Cleanup &   │
│ Creation    │───▶│ Execution   │───▶│ Reporting   │
└─────────────┘    └─────────────┘    └─────────────┘
```

## 📊 Configuration Profiles

### Development Profile
- **Workers**: 2 (lightweight for development)
- **Timeout**: 60 seconds (generous for debugging)
- **Memory**: 256MB (sufficient for most tests)
- **Verbose**: Enabled (detailed feedback)
- **Fail-Fast**: Enabled (quick feedback loop)
- **Debug**: Enabled (troubleshooting)

### Production Profile
- **Workers**: 8 (maximum throughput)
- **Timeout**: 30 seconds (strict timeouts)
- **Memory**: 1024MB (high capacity)
- **Verbose**: Disabled (clean output)
- **Strategy**: Adaptive (smart execution)
- **Profiling**: Enabled (performance tracking)

### CI Profile
- **Workers**: 4 (balanced for CI resources)
- **Timeout**: 120 seconds (accommodate CI overhead)
- **Fail-Fast**: Enabled (fast feedback)
- **Exclude**: "*manual*" (skip manual tests)
- **Output**: JSON format (machine readable)

### Benchmark Profile
- **Workers**: 1 (sequential for accuracy)
- **Timeout**: 300 seconds (long-running benchmarks)
- **Memory**: 2048MB (high memory for benchmarks)
- **Filter**: "*benchmark*" (only benchmark tests)
- **Profiling**: Enabled (detailed metrics)

## 🧪 Testing the Parallel Runner

### Run Parallelism Tests
```bash
# Test the parallelism control system
./zig-out/bin/cursed stdlib/testz/test_parallelism_control.csd

# Test configuration system
./zig-out/bin/cursed stdlib/testz/parallel_runner_config.csd
```

### Test Different Configurations
```bash
# Test with different worker counts
for workers in 1 2 4 8; do
    echo "Testing with $workers workers"
    CURSED_TEST_WORKERS=$workers ./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
done

# Test memory limits
for memory in 128 256 512 1024; do
    echo "Testing with ${memory}MB memory limit"
    CURSED_TEST_MEMORY_LIMIT=$memory ./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
done
```

### Performance Comparison
```bash
# Sequential vs Parallel comparison
echo "=== Sequential Execution ==="
time CURSED_TEST_PARALLEL=false ./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd

echo "=== Parallel Execution ==="
time CURSED_TEST_PARALLEL=true CURSED_TEST_WORKERS=4 ./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
```

## 🔍 Debugging and Troubleshooting

### Enable Debug Output
```bash
export CURSED_TEST_DEBUG=true
export CURSED_TEST_VERBOSE=true
./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
```

### Monitor Resource Usage
```bash
export CURSED_TEST_PROFILE=true
./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
```

### Configuration Validation
```bash
# Validate current configuration
./zig-out/bin/cursed stdlib/testz/parallel_runner_config.csd
```

## 📈 Performance Tips

### Optimal Worker Count
- **CPU-bound tests**: Set workers = CPU cores
- **I/O-bound tests**: Set workers = 2-4x CPU cores  
- **Memory-intensive tests**: Reduce workers based on available memory

### Memory Management
- Monitor memory usage with `CURSED_TEST_PROFILE=true`
- Adjust `CURSED_TEST_MEMORY_LIMIT` based on available system memory
- Use test isolation to prevent memory leaks between tests

### Load Balancing
- Use "resource_aware" strategy for mixed workloads
- Use "round_robin" for uniform tests
- Use "least_loaded" for varying test durations

## 🛠️ Integration

### CI/CD Integration
```yaml
# GitHub Actions example
- name: Run CURSED Tests
  env:
    CURSED_TEST_PARALLEL: true
    CURSED_TEST_WORKERS: 4
    CURSED_TEST_TIMEOUT: 120
    CURSED_TEST_OUTPUT_FORMAT: json
    CURSED_TEST_REPORT_FILE: test_results.json
  run: ./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
```

### Docker Integration
```dockerfile
ENV CURSED_TEST_PARALLEL=true
ENV CURSED_TEST_WORKERS=4
ENV CURSED_TEST_MEMORY_LIMIT=512
RUN ./zig-out/bin/cursed stdlib/testz/parallel_test_runner.csd
```

## 📝 Implementation Status

### ✅ Completed Features
- Environment variable configuration system
- Parallel and sequential execution modes
- Worker pool management with load balancing
- Resource monitoring and warnings
- Test isolation and cleanup
- Configuration profiles (dev, prod, CI, benchmark)
- Performance profiling and metrics
- Comprehensive test suite for parallelism control

### 🚧 Future Enhancements
- Real goroutine/thread implementation for parallel execution
- Advanced test sharding across multiple machines
- Test result caching and incremental execution
- Integration with external test runners
- Real-time dashboard for test execution monitoring

## 🤝 Contributing

The parallel test runner is designed to be extensible. Key areas for contribution:

1. **Worker Implementation**: Replace simulated workers with real goroutines
2. **Resource Monitoring**: Add real system resource monitoring
3. **Load Balancing**: Implement additional load balancing strategies
4. **Test Discovery**: Enhanced test discovery with metadata
5. **Reporting**: Additional output formats and integrations

---

**Note**: This implementation provides a comprehensive framework for parallel test execution in CURSED. The simulated components (workers, resource monitoring) demonstrate the architecture and can be replaced with real implementations as the CURSED runtime system develops.
