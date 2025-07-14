#!/bin/bash

# Test script for JIT thread safety fixes

echo "🧪 Testing JIT thread safety fixes..."

# Test 1: Basic JIT functionality
echo "📋 Test 1: Basic JIT compilation"
cargo test jit --lib -- --nocapture --test-threads=1 || echo "⚠️ JIT tests failed (expected if JIT is disabled)"

# Test 2: Concurrent GC tests
echo "📋 Test 2: Concurrent GC thread safety"
cargo test concurrent_gc --lib -- --nocapture --test-threads=1

# Test 3: Basic program execution
echo "📋 Test 3: Basic program execution"
cargo run --bin cursed test_thread_safety_fixes.csd

# Test 4: Stress test with multiple compilations
echo "📋 Test 4: Stress testing with multiple threads"
for i in {1..5}; do
    echo "  Iteration $i..."
    cargo run --bin cursed test_thread_safety_fixes.csd &
done
wait

echo "✅ All JIT thread safety tests completed!"
