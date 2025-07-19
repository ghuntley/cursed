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
