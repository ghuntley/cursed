# CURSED Platform Abstraction Layer (PAL) Testing Plan

## 📋 Overview

The **Platform Abstraction Layer (PAL)** is CURSED's cross-platform runtime optimization system that provides platform-specific implementations for memory management, goroutine scheduling, and hardware feature detection. This document outlines comprehensive testing procedures to validate PAL functionality across all supported platforms.

## 🏗️ PAL Architecture

### Core Components

- **PAL Factory** (`src/runtime/pal/mod.rs`) - Automatic platform detection and component creation
- **Platform Implementations** - ARM64, x86_64, and WebAssembly specific optimizations
- **Memory Managers** - Platform-optimized memory allocation and management
- **Schedulers** - Platform-specific goroutine scheduling with hardware awareness
- **Feature Detection** - Runtime capability detection (SIMD, crypto, memory features)

### Supported Platforms

| Architecture | Operating System | Features |
|-------------|------------------|----------|
| **ARM64** | macOS (Apple Silicon) | 16KB pages, P+E core scheduling, Metal acceleration |
| **ARM64** | Linux | 4KB pages, NUMA awareness, MTE detection |
| **x86_64** | macOS | AVX detection, standard pages, Homebrew integration |
| **x86_64** | Linux | Transparent huge pages, NUMA policy, advanced SIMD |
| **x86_64** | Windows | SEH handling, Windows heap optimization, MSVC compatibility |
| **WASM32** | Browser/Runtime | 64KB linear memory, cooperative scheduling, SIMD.js |

## 🧪 Testing Strategy

### Phase 1: Platform Detection Tests

#### 1.1 Automatic Platform Detection
```bash
# Test PAL factory creation on current platform
cargo run --bin cursed -- --version --verbose

# Expected output should include:
# - Detected architecture (arm64/x86_64/wasm32)
# - Operating system identification
# - Platform-specific optimizations enabled
# - Hardware feature detection results
```

#### 1.2 Cross-Platform Compilation
```bash
# Test compilation for all supported targets
./test_cross_compilation.sh

# Individual target testing:
cargo check --target aarch64-apple-darwin
cargo check --target aarch64-unknown-linux-gnu
cargo check --target x86_64-apple-darwin
cargo check --target x86_64-unknown-linux-gnu
cargo check --target x86_64-pc-windows-msvc
cargo check --target wasm32-unknown-unknown
cargo check --target wasm32-wasi
```

#### 1.3 Feature Detection Validation
```bash
# Create test program for feature detection
cat > test_feature_detection.csd << 'EOF'
yeet "testz"

test_start("Platform Feature Detection")

// Test should automatically detect and report platform capabilities
vibez.spill("Testing PAL feature detection...")

// This will be implemented by the runtime
damn based
EOF

cargo run --bin cursed test_feature_detection.csd
```

### Phase 2: Memory Management Tests

#### 2.1 Platform-Specific Memory Allocation
```bash
# Test memory allocation with platform optimizations
cat > test_memory_pal.csd << 'EOF'
yeet "testz"

test_start("PAL Memory Management")

// Test large allocations that should use platform-specific optimizations
sus large_buffer drip = allocate_memory(1024 * 1024) // 1MB
sus small_buffer drip = allocate_memory(64) // 64 bytes

assert_true(large_buffer != 0)
assert_true(small_buffer != 0)

deallocate_memory(large_buffer, 1024 * 1024)
deallocate_memory(small_buffer, 64)

print_test_summary()
EOF

cargo run --bin cursed test_memory_pal.csd
```

#### 2.2 Memory Alignment Testing
```bash
# Test platform-specific memory alignment
cat > test_memory_alignment.csd << 'EOF'
yeet "testz"

test_start("Memory Alignment Tests")

// ARM64 requires 16-byte alignment, x86_64 requires 16-byte for SIMD
sus aligned_ptr drip = allocate_aligned_memory(1024, 16)
assert_true(aligned_ptr % 16 == 0)

deallocate_memory(aligned_ptr, 1024)

print_test_summary()
EOF

cargo run --bin cursed test_memory_alignment.csd
```

#### 2.3 Large Page Support Testing
```bash
# Test large page allocation where supported
cat > test_large_pages.csd << 'EOF'
yeet "testz"

test_start("Large Page Support")

// Should automatically use large pages on supported platforms
sus large_allocation drip = allocate_memory(2 * 1024 * 1024) // 2MB
assert_true(large_allocation != 0)

// Verify alignment for large pages
assert_true(large_allocation % page_size() == 0)

deallocate_memory(large_allocation, 2 * 1024 * 1024)

print_test_summary()
EOF

cargo run --bin cursed test_large_pages.csd
```

### Phase 3: Scheduler Optimization Tests

#### 3.1 Goroutine Spawning Performance
```bash
# Test platform-optimized goroutine creation
cat > test_scheduler_pal.csd << 'EOF'
yeet "testz"

test_start("PAL Scheduler Performance")

sus start_time drip = current_time_nanos()

// Spawn many goroutines to test scheduler efficiency
periodt i := 0; i < 1000; i++ {
    stan {
        // Simple computation
        sus result drip = i * i
        vibez.spill("Goroutine " + str(i) + " result: " + str(result))
    }
}

// Wait for completion
wait_for_all_goroutines()

sus end_time drip = current_time_nanos()
sus duration drip = end_time - start_time

vibez.spill("Spawned 1000 goroutines in " + str(duration) + " nanoseconds")

print_test_summary()
EOF

cargo run --bin cursed test_scheduler_pal.csd
```

#### 3.2 Apple Silicon P+E Core Testing
```bash
# Test performance vs efficiency core scheduling (macOS ARM64 only)
if [[ $(uname -m) == "arm64" && $(uname -s) == "Darwin" ]]; then
    cat > test_apple_silicon_cores.csd << 'EOF'
yeet "testz"

test_start("Apple Silicon Core Scheduling")

// CPU-intensive task should prefer P-cores
stan {
    periodt i := 0; i < 1000000; i++ {
        sus heavy_computation drip = fibonacci(30)
    }
    vibez.spill("CPU-intensive task completed")
}

// I/O task should prefer E-cores  
stan {
    periodt i := 0; i < 1000; i++ {
        read_file("test.txt")
        yield_goroutine()
    }
    vibez.spill("I/O task completed")
}

wait_for_all_goroutines()

print_test_summary()
EOF

    cargo run --bin cursed test_apple_silicon_cores.csd
fi
```

#### 3.3 NUMA Awareness Testing
```bash
# Test NUMA-aware scheduling on multi-socket systems
cat > test_numa_scheduling.csd << 'EOF'
yeet "testz"

test_start("NUMA-Aware Scheduling")

// Test memory locality with goroutines
periodt node := 0; node < numa_node_count(); node++ {
    stan {
        // Allocate memory local to this NUMA node
        sus local_memory drip = allocate_on_node(1024 * 1024, node)
        
        // Perform computation on local memory
        periodt i := 0; i < 1000; i++ {
            process_memory_block(local_memory, 1024)
        }
        
        deallocate_memory(local_memory, 1024 * 1024)
        vibez.spill("NUMA node " + str(node) + " processing complete")
    }
}

wait_for_all_goroutines()

print_test_summary()
EOF

cargo run --bin cursed test_numa_scheduling.csd
```

### Phase 4: WebAssembly Specific Tests

#### 4.1 WASM Memory Management
```bash
# Test WebAssembly linear memory management
if command -v wasmtime &> /dev/null; then
    cargo build --target wasm32-wasi
    
    cat > test_wasm_memory.csd << 'EOF'
yeet "testz"

test_start("WASM Memory Management")

// Test linear memory allocation
sus mem1 drip = allocate_memory(64 * 1024) // 64KB page
sus mem2 drip = allocate_memory(32 * 1024) // 32KB

assert_true(mem1 != 0)
assert_true(mem2 != 0)
assert_true(mem2 > mem1) // Should be allocated after mem1

deallocate_memory(mem1, 64 * 1024)
deallocate_memory(mem2, 32 * 1024)

print_test_summary()
EOF

    wasmtime target/wasm32-wasi/debug/cursed.wasm test_wasm_memory.csd
fi
```

#### 4.2 WASM Cooperative Scheduling
```bash
# Test cooperative scheduling in WebAssembly
cat > test_wasm_scheduling.csd << 'EOF'
yeet "testz"

test_start("WASM Cooperative Scheduling")

sus counter drip = 0

// Multiple tasks that yield control
periodt i := 0; i < 5; i++ {
    stan {
        periodt j := 0; j < 100; j++ {
            counter++
            yield_goroutine() // Cooperative yield
        }
        vibez.spill("Task " + str(i) + " completed")
    }
}

wait_for_all_goroutines()

assert_eq_int(counter, 500)

print_test_summary()
EOF

if command -v wasmtime &> /dev/null; then
    wasmtime target/wasm32-wasi/debug/cursed.wasm test_wasm_scheduling.csd
fi
```

### Phase 5: Hardware Feature Utilization Tests

#### 5.1 SIMD Instruction Testing
```bash
# Test SIMD instruction utilization
cat > test_simd_features.csd << 'EOF'
yeet "testz"

test_start("SIMD Feature Utilization")

// Test vector operations that should use SIMD when available
sus vector_a drip = [1.0, 2.0, 3.0, 4.0]
sus vector_b drip = [5.0, 6.0, 7.0, 8.0]

// Should automatically use AVX/NEON/WASM SIMD when available
sus result drip = vector_multiply(vector_a, vector_b)

assert_eq_float(result[0], 5.0)
assert_eq_float(result[1], 12.0)
assert_eq_float(result[2], 21.0)
assert_eq_float(result[3], 32.0)

print_test_summary()
EOF

cargo run --bin cursed test_simd_features.csd
```

#### 5.2 Crypto Acceleration Testing
```bash
# Test hardware crypto acceleration
cat > test_crypto_acceleration.csd << 'EOF'
yeet "testz"

test_start("Crypto Hardware Acceleration")

sus data tea = "Hello, CURSED!"
sus key tea = "secret_key_12345"

// Should use AES-NI on x86_64 or hardware AES on ARM64
sus encrypted tea = aes_encrypt(data, key)
sus decrypted tea = aes_decrypt(encrypted, key)

assert_eq_string(data, decrypted)

// Test SHA acceleration
sus hash tea = sha256(data)
assert_true(length(hash) == 64) // 32 bytes * 2 hex chars

print_test_summary()
EOF

cargo run --bin cursed test_crypto_acceleration.csd
```

### Phase 6: Stress Testing

#### 6.1 Memory Stress Test
```bash
cat > test_memory_stress.csd << 'EOF'
yeet "testz"

test_start("Memory Stress Test")

sus allocations drip = []

// Allocate memory in various sizes to test platform optimization
periodt i := 0; i < 1000; i++ {
    sus size drip = (i % 10 + 1) * 1024 // 1KB to 10KB
    sus ptr drip = allocate_memory(size)
    assert_true(ptr != 0)
    allocations.append([ptr, size])
}

// Deallocate in reverse order
periodt i := length(allocations) - 1; i >= 0; i-- {
    sus allocation drip = allocations[i]
    deallocate_memory(allocation[0], allocation[1])
}

print_test_summary()
EOF

cargo run --bin cursed test_memory_stress.csd
```

#### 6.2 Scheduler Stress Test
```bash
cat > test_scheduler_stress.csd << 'EOF'
yeet "testz"

test_start("Scheduler Stress Test")

sus completed_tasks drip = 0

// Spawn many short-lived goroutines
periodt i := 0; i < 10000; i++ {
    stan {
        // Short computation
        sus result drip = i * 2 + 1
        atomic_increment(&completed_tasks)
        
        lowkey i % 100 == 0 {
            yield_goroutine()
        }
    }
}

wait_for_all_goroutines()

assert_eq_int(completed_tasks, 10000)

print_test_summary()
EOF

cargo run --bin cursed test_scheduler_stress.csd
```

### Phase 7: Performance Benchmarking

#### 7.1 Platform Comparison Benchmark
```bash
# Create comprehensive performance benchmark
./create_platform_benchmark.sh

cat > benchmark_pal_performance.csd << 'EOF'
yeet "testz"

test_start("PAL Performance Benchmark")

sus iterations drip = 1000000

// Memory allocation benchmark
sus start_time drip = current_time_nanos()
periodt i := 0; i < iterations; i++ {
    sus ptr drip = allocate_memory(64)
    deallocate_memory(ptr, 64)
}
sus memory_time drip = current_time_nanos() - start_time

// Goroutine spawning benchmark
start_time = current_time_nanos()
periodt i := 0; i < 1000; i++ {
    stan {
        sus dummy drip = i * i
    }
}
wait_for_all_goroutines()
sus goroutine_time drip = current_time_nanos() - start_time

vibez.spill("Memory operations: " + str(memory_time) + " ns")
vibez.spill("Goroutine operations: " + str(goroutine_time) + " ns")

print_test_summary()
EOF

cargo run --bin cursed benchmark_pal_performance.csd
```

## 🔧 Test Execution Scripts

### Automated Test Runner
```bash
#!/bin/bash
# run_pal_tests.sh

echo "🧪 CURSED PAL Testing Suite"
echo "=========================="

# Set up test environment
export RUST_LOG=debug
export CURSED_TEST_MODE=1

# Phase 1: Platform Detection
echo "📍 Phase 1: Platform Detection Tests"
cargo run --bin cursed -- --version --verbose
./test_cross_compilation.sh

# Phase 2: Memory Management
echo "🧠 Phase 2: Memory Management Tests"
cargo run --bin cursed test_memory_pal.csd
cargo run --bin cursed test_memory_alignment.csd
cargo run --bin cursed test_large_pages.csd

# Phase 3: Scheduler Tests
echo "⚡ Phase 3: Scheduler Optimization Tests"
cargo run --bin cursed test_scheduler_pal.csd

# Platform-specific tests
if [[ $(uname -m) == "arm64" && $(uname -s) == "Darwin" ]]; then
    echo "🍎 Apple Silicon specific tests"
    cargo run --bin cursed test_apple_silicon_cores.csd
fi

# Phase 4: WebAssembly Tests
if command -v wasmtime &> /dev/null; then
    echo "🕸️ Phase 4: WebAssembly Tests"
    cargo build --target wasm32-wasi
    wasmtime target/wasm32-wasi/debug/cursed.wasm test_wasm_memory.csd
    wasmtime target/wasm32-wasi/debug/cursed.wasm test_wasm_scheduling.csd
fi

# Phase 5: Hardware Features
echo "🔧 Phase 5: Hardware Feature Tests"
cargo run --bin cursed test_simd_features.csd
cargo run --bin cursed test_crypto_acceleration.csd

# Phase 6: Stress Testing
echo "💪 Phase 6: Stress Tests"
cargo run --bin cursed test_memory_stress.csd
cargo run --bin cursed test_scheduler_stress.csd

# Phase 7: Performance Benchmarking
echo "📊 Phase 7: Performance Benchmarks"
cargo run --bin cursed benchmark_pal_performance.csd

echo "✅ All PAL tests completed!"
```

### Cross-Compilation Test Script
```bash
#!/bin/bash
# test_cross_compilation.sh

echo "🔀 Testing cross-platform compilation..."

TARGETS=(
    "aarch64-apple-darwin"
    "aarch64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "x86_64-unknown-linux-gnu"
    "x86_64-pc-windows-msvc"
    "wasm32-unknown-unknown"
    "wasm32-wasi"
)

for target in "${TARGETS[@]}"; do
    echo "Checking target: $target"
    if rustup target list --installed | grep -q "$target"; then
        if cargo check --target "$target" --quiet; then
            echo "✅ $target: Compilation successful"
        else
            echo "❌ $target: Compilation failed"
        fi
    else
        echo "⚠️  $target: Target not installed"
        echo "   Install with: rustup target add $target"
    fi
done
```

## 📊 Expected Results

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
❌ Memory tagging (MTE) - not exposed on macOS
```

### Performance Baseline Targets

| Platform | Memory Ops/sec | Goroutine Spawn/sec | Notes |
|----------|---------------|-------------------|-------|
| ARM64 macOS | > 1M | > 100K | Apple Silicon optimized |
| ARM64 Linux | > 800K | > 80K | Standard ARM64 |
| x86_64 macOS | > 1.2M | > 120K | AVX optimizations |
| x86_64 Linux | > 1.5M | > 150K | Huge pages + NUMA |
| x86_64 Windows | > 1M | > 100K | Windows heap optimized |
| WASM32 | > 100K | > 10K | Limited by WASM constraints |

## 🐛 Troubleshooting

### Common Issues

1. **Target not installed**: Run `rustup target add <target>`
2. **WASM tests failing**: Install `wasmtime` with `cargo install wasmtime-cli`
3. **Feature detection issues**: Check hardware support and OS version
4. **Permission denied**: Some tests may require elevated privileges for large pages

### Debug Mode
```bash
# Enable detailed PAL logging
export RUST_LOG=cursed::runtime::pal=debug
cargo run --bin cursed test_program.csd
```

### Platform-Specific Notes

- **Apple Silicon**: Requires macOS 11+ for full optimization
- **Linux ARM64**: MTE requires Linux 5.4+ and compatible hardware
- **Windows x86_64**: Large page support requires "Lock pages in memory" privilege
- **WebAssembly**: SIMD requires browser with WASM SIMD support

## 📈 Success Criteria

- ✅ All platform detection tests pass
- ✅ Memory allocation tests show platform-appropriate page sizes
- ✅ Scheduler tests demonstrate hardware-aware optimization
- ✅ Cross-compilation succeeds for all targets
- ✅ Performance benchmarks meet baseline targets
- ✅ Stress tests complete without memory leaks
- ✅ Hardware features are properly detected and utilized

This comprehensive testing plan ensures the PAL implementation provides robust, optimized, cross-platform runtime support for the CURSED programming language.
