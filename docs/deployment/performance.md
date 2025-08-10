# CURSED Performance Guide

Comprehensive guide to optimizing CURSED compiler and runtime performance.

## Performance Overview

CURSED is designed for high performance across all dimensions:
- **Compilation Speed**: 0.1-0.2s for typical programs
- **Runtime Performance**: LLVM-optimized native code
- **Memory Efficiency**: Minimal runtime overhead
- **Binary Size**: Compact executables (2-5MB typical)

## Compilation Performance

### Build Time Optimization

#### Parallel Compilation
```bash
# Use all CPU cores
export CURSED_PARALLEL_JOBS=$(nproc)
zig build -j$(nproc)

# Or specify job count
export CURSED_PARALLEL_JOBS=8
zig build -j8
```

#### Compilation Cache
```bash
# Enable persistent compilation cache
export CURSED_ENABLE_CACHE=true
export CURSED_CACHE_DIR="$HOME/.cursed/cache"
mkdir -p ~/.cursed/cache

# Cache configuration
export CURSED_CACHE_SIZE_LIMIT="2GB"
export CURSED_CACHE_TTL="7d"  # 7 days
```

#### Incremental Compilation
```bash
# Only recompile changed modules
cursed-zig --incremental src/main.csd

# Watch mode for development
cursed-zig --watch src/main.csd
```

#### Build Optimization Levels
```bash
# Development builds (fastest compilation)
zig build

# Release builds (optimized for speed)
zig build -Doptimize=ReleaseFast

# Size-optimized builds
zig build -Doptimize=ReleaseSmall

# Safe release builds (with runtime checks)
zig build -Doptimize=ReleaseSafe
```

### Compilation Benchmarks

**Small Program (100 lines):**
```bash
# Benchmark script
#!/bin/bash
echo 'Creating test program...'
cat > perf_test.csd << 'EOF'
yeet "mathz"
yeet "stringz"

slay fibonacci(n drip) drip {
    ready (n <= 1) { damn n }
    damn fibonacci(n-1) + fibonacci(n-2)
}

slay main() {
    for i in 0..10 {
        sus result = fibonacci(i)
        vibez.spill("fib(", i, ") =", result)
    }
}

main()
EOF

echo 'Benchmarking compilation...'
time cursed-zig perf_test.csd
# Expected: ~0.1-0.2 seconds
```

**Medium Project (1000 lines, 10 modules):**
```bash
# Expected compilation time: 1-3 seconds
time zig build
```

**Large Project (10000+ lines, 100+ modules):**
```bash
# Expected compilation time: 5-10 seconds
time zig build -j$(nproc)
```

### Development Environment Optimization

#### VS Code Setup
```json
{
  "cursed.compiler.path": "/usr/local/bin/cursed-zig",
  "cursed.build.parallel": true,
  "cursed.build.cache": true,
  "cursed.build.incremental": true,
  "cursed.lsp.responsiveness": "fast"
}
```

#### Vim/Neovim Setup
```lua
-- lua/cursed.lua
require('lspconfig').cursed_lsp.setup{
  cmd = { "cursed-lsp" },
  settings = {
    cursed = {
      build = {
        parallel = true,
        cache = true,
        fast_compile = true
      }
    }
  }
}
```

## Runtime Performance

### LLVM Compilation Optimization

#### Basic Optimization
```bash
# Compile with LLVM optimizations
cursed-zig --compile program.csd

# With link-time optimization
cursed-zig --compile --lto program.csd

# Maximum optimization
cursed-zig --compile --lto --optimize=speed program.csd
```

#### Advanced LLVM Options
```bash
# Profile-guided optimization
cursed-zig --compile --profile-generate program.csd
./program  # Run with representative workload
cursed-zig --compile --profile-use program.csd

# Target-specific optimizations
cursed-zig --compile --target=native --cpu=native program.csd

# Vector optimization
cursed-zig --compile --enable-vectorization program.csd
```

### Performance Profiling

#### CPU Profiling
```bash
# Compile with profiling support
cursed-zig --compile --profile program.csd

# Profile with perf
perf record -g ./program
perf report

# Profile with gprof
cursed-zig --compile --gprof program.csd
./program
gprof ./program gmon.out > analysis.txt
```

#### Memory Profiling
```bash
# Compile with memory profiling
cursed-zig --compile --memory-profile program.csd

# Profile with Valgrind
valgrind --tool=callgrind ./program
kcachegrind callgrind.out.*

# Memory usage analysis
valgrind --tool=massif ./program
ms_print massif.out.*
```

### Benchmark Suite

#### CPU-Intensive Benchmarks
```cursed
# cpu_benchmark.csd
yeet "timez"
yeet "mathz"

slay benchmark_fibonacci(n drip) {
    sus start_time = timestamp()
    sus result = fibonacci(n)
    sus end_time = timestamp()
    
    vibez.spill("Fibonacci(", n, ") =", result)
    vibez.spill("Time:", end_time - start_time, "ms")
}

slay fibonacci(n drip) drip {
    ready (n <= 1) { damn n }
    damn fibonacci(n-1) + fibonacci(n-2)
}

# Benchmark matrix multiplication
slay matrix_multiply_benchmark() {
    sus size = 500
    sus a = create_matrix(size, size)
    sus b = create_matrix(size, size)
    
    sus start_time = timestamp()
    sus c = matrix_multiply(a, b)
    sus end_time = timestamp()
    
    vibez.spill("Matrix multiplication (", size, "x", size, "):", end_time - start_time, "ms")
}

benchmark_fibonacci(35)
matrix_multiply_benchmark()
```

Expected Results:
- **Fibonacci(35)**: ~300ms (vs 2.5s in interpreted languages)
- **Matrix multiplication (500x500)**: ~200ms

#### Memory-Intensive Benchmarks
```cursed
# memory_benchmark.csd
yeet "timez"
yeet "arrayz"

slay memory_allocation_benchmark() {
    sus start_time = timestamp()
    
    # Allocate large arrays
    sus arrays [][]drip = []
    for i in 0..1000 {
        sus large_array = create_array(10000)
        arrays = append(arrays, large_array)
    }
    
    sus end_time = timestamp()
    vibez.spill("Memory allocation benchmark:", end_time - start_time, "ms")
    vibez.spill("Total allocated:", len(arrays) * 10000 * 4, "bytes")
}

memory_allocation_benchmark()
```

Expected Results:
- **Memory allocation**: <50ms for 40MB allocation
- **Memory usage**: <100MB peak RSS

#### Concurrency Benchmarks
```cursed
# concurrency_benchmark.csd
yeet "concurrenz"
yeet "timez"

slay goroutine_benchmark() {
    sus start_time = timestamp()
    sus workers = 1000
    sus work_channel chan<drip> = make_channel_buffered(100)
    sus result_channel chan<drip> = make_channel_buffered(workers)
    
    # Spawn workers
    for i in 0..workers {
        vibe worker(i, work_channel, result_channel)
    }
    
    # Send work
    for i in 0..workers {
        work_channel <- i
    }
    close(work_channel)
    
    # Collect results
    sus total = 0
    for i in 0..workers {
        total = total + <-result_channel
    }
    
    sus end_time = timestamp()
    vibez.spill("Goroutine benchmark (", workers, " workers):", end_time - start_time, "ms")
    vibez.spill("Total result:", total)
}

slay worker(id drip, work chan<drip>, result chan<drip>) {
    bestie (based) {
        sus work_item = <-work
        ready (work_item == -1) { break }  # Termination signal
        
        # Simulate work
        sus computed = fibonacci_fast(work_item % 20)
        result <- computed
    }
}

goroutine_benchmark()
```

Expected Results:
- **1000 goroutines**: <100ms spawning and coordination
- **Channel operations**: <1µs per operation

### Performance Tuning Patterns

#### Memory Optimization
```cursed
# Use arena allocators for temporary data
slay process_large_dataset(data []drip) {
    # Arena automatically freed at function exit
    sus temp_results = allocate_temp_array(len(data))
    
    for i, item in data {
        temp_results[i] = expensive_computation(item)
    }
    
    damn aggregate_results(temp_results)
}

# Prefer stack allocation for small arrays
slay fast_small_computation() {
    sus local_buffer [1024]drip  # Stack allocated
    # Process buffer...
}

# Use object pooling for frequent allocations
sus connection_pool = create_pool(100)

slay get_connection() Connection {
    damn connection_pool.get()
}

slay return_connection(conn Connection) {
    connection_pool.put(conn)
}
```

#### CPU Optimization
```cursed
# Use SIMD operations for numeric computations
slay vector_add_optimized(a []f32, b []f32) []f32 {
    # Compiler automatically vectorizes simple loops
    sus result = make_array(len(a))
    for i in 0..len(a) {
        result[i] = a[i] + b[i]  # SIMD optimized
    }
    damn result
}

# Minimize function call overhead in hot paths
slay hot_path_computation(data []drip) drip {
    sus total = 0
    # Inline simple operations
    for item in data {
        total = total + (item * 2 + 1)  # Inlined
    }
    damn total
}

# Use compile-time computation when possible
const LOOKUP_TABLE = compute_lookup_table()  # Computed at compile time

slay fast_lookup(index drip) drip {
    damn LOOKUP_TABLE[index]  # No runtime computation
}
```

#### I/O Optimization
```cursed
yeet "filez"
yeet "concurrenz"

# Async I/O with goroutines
slay parallel_file_processing(filenames []tea) {
    sus workers = min(len(filenames), 10)  # Limit concurrent I/O
    sus work_channel chan<tea> = make_channel_buffered(len(filenames))
    sus result_channel chan<tea> = make_channel_buffered(len(filenames))
    
    # Spawn workers
    for i in 0..workers {
        vibe file_worker(work_channel, result_channel)
    }
    
    # Send work
    for filename in filenames {
        work_channel <- filename
    }
    close(work_channel)
    
    # Collect results
    for i in 0..len(filenames) {
        sus result = <-result_channel
        vibez.spill("Processed:", result)
    }
}

slay file_worker(work chan<tea>, result chan<tea>) {
    bestie (based) {
        sus filename = <-work
        ready (filename == "") { break }
        
        sus content = read_file(filename)
        sus processed = process_content(content)
        result <- processed
    }
}

# Buffered I/O for better performance
slay efficient_file_write(filename tea, data []tea) {
    sus writer = create_buffered_writer(filename, 64*1024)  # 64KB buffer
    defer writer.close()
    
    for line in data {
        writer.write(line + "\n")
    }
    # Buffer automatically flushed on close
}
```

## Memory Performance

### Memory Management Strategies

#### Arena Allocation
```cursed
# Automatic arena allocation for temporary data
slay process_request(request Request) Response {
    # All allocations in this scope use arena
    sus parsed_data = parse_complex_data(request.body)
    sus processed = transform_data(parsed_data)
    sus response = format_response(processed)
    
    # Arena automatically freed here
    damn response
}
```

#### Reference Counting
```cursed
# Shared ownership with reference counting
squad SharedResource {
    spill data tea
    spill ref_count drip
}

slay create_shared(data tea) Rc<SharedResource> {
    damn Rc.new(SharedResource{
        data: data,
        ref_count: 1
    })
}

slay clone_shared(resource Rc<SharedResource>) Rc<SharedResource> {
    damn resource.clone()  # Increment ref count
}
```

#### Memory Pools
```cursed
# Object pooling for frequent allocations
sus buffer_pool = Pool.new(1024, 100)  # 100 buffers of 1KB each

slay get_buffer() []u8 {
    damn buffer_pool.get()
}

slay return_buffer(buffer []u8) {
    buffer_pool.put(buffer)
}

# Use in hot path
slay process_many_requests(requests []Request) {
    for request in requests {
        sus buffer = get_buffer()
        defer return_buffer(buffer)
        
        # Use buffer for processing
        process_request_with_buffer(request, buffer)
    }
}
```

### Memory Usage Monitoring

#### Built-in Memory Profiling
```cursed
# Enable memory profiling
@compile_flag("enable_memory_profiling")

slay memory_intensive_function() {
    # Memory usage automatically tracked
    sus large_data = allocate_large_array(1000000)
    
    # Check current memory usage
    ready (get_memory_usage() > MEMORY_LIMIT) {
        trigger_gc()
    }
    
    process_data(large_data)
}
```

#### Memory Debugging
```bash
# Compile with memory debugging
cursed-zig --compile --debug-memory program.csd

# Run with memory monitoring
CURSED_MEMORY_DEBUG=1 ./program

# Memory leak detection
valgrind --leak-check=full ./program

# Memory usage limits
CURSED_MEMORY_LIMIT=100MB ./program
```

## Performance Monitoring

### Built-in Performance Metrics

```cursed
# performance_monitor.csd
yeet "timez"

@compile_flag("enable_performance_monitoring")

slay monitored_function() {
    sus start_time = timestamp()
    defer {
        sus end_time = timestamp()
        sus duration = end_time - start_time
        ready (duration > PERFORMANCE_THRESHOLD) {
            vibez.spill("WARNING: Slow function execution:", duration, "ms")
        }
    }
    
    # Function implementation
    expensive_computation()
}

# Automatic performance tracking
@performance_track
slay critical_path() {
    # Automatically tracked for performance regressions
    important_computation()
}
```

### Continuous Performance Testing

```bash
#!/bin/bash
# continuous_perf_test.sh

echo "=== CURSED Performance Regression Tests ==="

# Compile benchmarks
cursed-zig --compile --lto benchmarks/cpu_benchmark.csd
cursed-zig --compile --lto benchmarks/memory_benchmark.csd
cursed-zig --compile --lto benchmarks/concurrency_benchmark.csd

# Run CPU benchmarks
echo "Running CPU benchmarks..."
./cpu_benchmark | tee cpu_results.txt

# Run memory benchmarks
echo "Running memory benchmarks..."
/usr/bin/time -v ./memory_benchmark 2>&1 | tee memory_results.txt

# Run concurrency benchmarks
echo "Running concurrency benchmarks..."
./concurrency_benchmark | tee concurrency_results.txt

# Check for regressions
python3 check_performance_regression.py cpu_results.txt memory_results.txt concurrency_results.txt
```

### Performance Baseline

**Reference Performance (Apple M2, 16GB RAM):**

| Benchmark | Expected Performance | Tolerance |
|-----------|---------------------|-----------|
| Fibonacci(35) | 250-350ms | ±20% |
| Matrix Multiply (500x500) | 150-250ms | ±30% |
| 1000 Goroutines | <100ms | ±50% |
| File I/O (100MB) | <200ms | ±40% |
| JSON Parse (1MB) | <50ms | ±30% |
| Memory Allocation (100MB) | <100ms | ±50% |

## Production Performance Tips

### Deployment Optimization

```bash
# Production build with all optimizations
cursed-zig --compile \
  --lto \
  --optimize=speed \
  --target=native \
  --cpu=native \
  --profile-guided-optimization \
  --strip-debug \
  program.csd

# Container optimization
FROM ubuntu:22.04 AS builder
RUN apt-get update && apt-get install -y cursed-zig
COPY . /src
WORKDIR /src
RUN cursed-zig --compile --lto --optimize=size main.csd

FROM ubuntu:22.04
RUN apt-get update && apt-get install -y libllvm15
COPY --from=builder /src/main /usr/local/bin/app
CMD ["app"]
```

### Configuration Tuning

```bash
# Environment variables for production
export CURSED_GC_STRATEGY="incremental"
export CURSED_THREAD_POOL_SIZE="$(nproc)"
export CURSED_MEMORY_POOL_SIZE="512MB"
export CURSED_ENABLE_SIMD="true"
export CURSED_CPU_AFFINITY="true"

# Kernel parameters for high-performance apps
echo 'vm.swappiness=1' | sudo tee -a /etc/sysctl.conf
echo 'net.core.rmem_max=134217728' | sudo tee -a /etc/sysctl.conf
echo 'net.core.wmem_max=134217728' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

### Monitoring and Alerting

```bash
# Performance monitoring script
#!/bin/bash
# monitor_performance.sh

while true; do
    # CPU usage
    CPU_USAGE=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
    
    # Memory usage
    MEMORY_USAGE=$(free | grep Mem | awk '{printf("%.1f", $3/$2 * 100.0)}')
    
    # Response time
    RESPONSE_TIME=$(curl -w "%{time_total}" -s -o /dev/null localhost:8080/health)
    
    # Log metrics
    echo "$(date): CPU=${CPU_USAGE}% MEM=${MEMORY_USAGE}% RT=${RESPONSE_TIME}s"
    
    # Alert on performance degradation
    if (( $(echo "$RESPONSE_TIME > 1.0" | bc -l) )); then
        echo "ALERT: High response time: ${RESPONSE_TIME}s"
        # Send alert to monitoring system
    fi
    
    sleep 10
done
```

## Performance Best Practices

### Do's ✅

1. **Use LLVM compilation** for production deployments
2. **Enable LTO** for better optimization across modules  
3. **Profile before optimizing** to identify actual bottlenecks
4. **Use arena allocation** for temporary data
5. **Prefer stack allocation** for small, short-lived data
6. **Minimize memory allocations** in hot paths
7. **Use goroutines** for I/O-bound concurrency
8. **Cache compilation results** for faster development
9. **Set appropriate buffer sizes** for I/O operations
10. **Monitor performance metrics** in production

### Don'ts ❌

1. **Don't optimize prematurely** without profiling
2. **Don't use interpreted mode** in production
3. **Don't ignore memory leak warnings** from valgrind
4. **Don't create excessive goroutines** (>10k without pooling)
5. **Don't block goroutines** with CPU-intensive work
6. **Don't use debug builds** in production
7. **Don't skip performance testing** before deployment
8. **Don't ignore compiler optimization warnings**
9. **Don't over-allocate** buffers and pools
10. **Don't forget to** profile memory usage patterns

CURSED's performance is designed to rival C and Rust while maintaining memory safety and developer productivity. With proper optimization techniques, CURSED applications can achieve excellent performance across all metrics.

For more specific performance questions, see the [FAQ](../support/faq.md) or join the [Discord community](https://discord.gg/cursed-lang).
