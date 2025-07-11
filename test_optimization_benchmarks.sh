#!/bin/bash

# Test script for CURSED advanced optimization passes
# This script demonstrates the new optimization features

echo "🚀 Testing CURSED Advanced Optimization Passes"
echo "=============================================="

# Test file
TEST_FILE="test_advanced_optimization.csd"

# Clean up any existing files
rm -f test_advanced_optimization test_advanced_optimization.ll test_advanced_optimization.s

echo ""
echo "📊 1. Testing basic optimization (O2)"
echo "--------------------------------------"
cargo run --bin cursed compile --opt-level 2 --benchmark "$TEST_FILE"

echo ""
echo "🎯 2. Testing Profile-Guided Optimization (PGO)"
echo "-----------------------------------------------"
# First generate profile data
cargo run --bin cursed compile --pgo-generate --benchmark "$TEST_FILE"
# Then use the profile data for optimization
cargo run --bin cursed compile --enable-pgo --pgo-profile target/pgo-profile.profdata --benchmark "$TEST_FILE"

echo ""
echo "🔗 3. Testing Link-Time Optimization (LTO)"
echo "------------------------------------------"
cargo run --bin cursed compile --enable-lto --lto-level full --benchmark "$TEST_FILE"

echo ""
echo "📦 4. Testing Size Optimization"
echo "-------------------------------"
cargo run --bin cursed compile --size-opt --size-level z --benchmark "$TEST_FILE"

echo ""
echo "🏭 5. Testing Production Pipeline"
echo "--------------------------------"
cargo run --bin cursed compile --pass-pipeline production --enable-lto --benchmark "$TEST_FILE"

echo ""
echo "⚡ 6. Testing Combined Advanced Optimization"
echo "-------------------------------------------"
cargo run --bin cursed compile \
    --enable-pgo --pgo-profile target/pgo-profile.profdata \
    --enable-lto --lto-level full \
    --pass-pipeline production \
    --benchmark \
    "$TEST_FILE"

echo ""
echo "🔬 7. Testing LLVM IR Generation with Advanced Optimization"
echo "----------------------------------------------------------"
cargo run --bin cursed compile --emit-ir --enable-pgo --enable-lto --benchmark "$TEST_FILE"

echo ""
echo "⚙️ 8. Testing Assembly Generation with Advanced Optimization"
echo "------------------------------------------------------------"
cargo run --bin cursed compile --emit-asm --size-opt --benchmark "$TEST_FILE"

echo ""
echo "✅ Advanced optimization testing completed!"
echo "Check the generated files and benchmark reports above."
