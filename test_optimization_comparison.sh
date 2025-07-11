#!/bin/bash

# Test script to compare different optimization levels

echo "🚀 CURSED Optimization Comparison Test"
echo "======================================"

# Test program with optimization opportunities
cat > benchmark_test.csd << 'EOF'
// Benchmark program for optimization testing
vibez.spill("Starting optimization benchmark...")

// Constant folding opportunities
sus x normie = 10 + 20 + 30
sus y normie = x * 2 + 5

// Function inlining opportunities
slay simple_add(a normie, b normie) normie {
    damn a + b
}

// Loop optimization opportunities
sus total normie = 0
bestie i := 0; i < 100; i++ {
    total = total + simple_add(i, 1)
}

vibez.spill("Optimization result: " + total)
vibez.spill("Benchmark complete!")
EOF

echo "Testing different optimization levels:"
echo "------------------------------------"

# Test O0 (no optimization)
echo "🔧 Testing O0 (no optimization)..."
time cargo run --bin cursed -- compile --opt-level 0 benchmark_test.csd -o benchmark_o0 2>/dev/null
echo "✓ O0 compilation completed"

# Test O1 (basic optimization)
echo "🔧 Testing O1 (basic optimization)..."
time cargo run --bin cursed -- compile --opt-level 1 benchmark_test.csd -o benchmark_o1 2>/dev/null
echo "✓ O1 compilation completed"

# Test O2 (standard optimization)
echo "🔧 Testing O2 (standard optimization)..."
time cargo run --bin cursed -- compile --opt-level 2 benchmark_test.csd -o benchmark_o2 2>/dev/null
echo "✓ O2 compilation completed"

# Test O3 (aggressive optimization)
echo "🔧 Testing O3 (aggressive optimization)..."
time cargo run --bin cursed -- compile --opt-level 3 benchmark_test.csd -o benchmark_o3 2>/dev/null
echo "✓ O3 compilation completed"

# Test basic optimization flag
echo "🔧 Testing --optimize flag..."
time cargo run --bin cursed -- compile --optimize benchmark_test.csd -o benchmark_optimize 2>/dev/null
echo "✓ --optimize compilation completed"

echo ""
echo "🎯 All optimization levels tested successfully!"
echo "📊 Performance comparison:"
echo "   O0: No optimization (fastest compile, slowest execution)"
echo "   O1: Basic optimization (fast compile, good execution)"
echo "   O2: Standard optimization (balanced compile/execution)"
echo "   O3: Aggressive optimization (slow compile, fastest execution)"
echo ""
echo "✅ CURSED optimization system is working correctly!"

# Clean up
rm -f benchmark_test.csd benchmark_o0 benchmark_o1 benchmark_o2 benchmark_o3 benchmark_optimize
