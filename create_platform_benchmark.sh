#!/bin/bash
# create_platform_benchmark.sh

echo "📊 Creating Platform Performance Benchmark"
echo "========================================="

# Detect current platform
ARCH=$(uname -m)
OS=$(uname -s)
DATE=$(date '+%Y-%m-%d_%H-%M-%S')

BENCHMARK_DIR="benchmark_results_${ARCH}_${OS}_${DATE}"
mkdir -p "$BENCHMARK_DIR"

echo "Creating benchmark suite for $ARCH $OS..."

# Create platform-specific benchmark test
cat > "${BENCHMARK_DIR}/platform_benchmark.csd" << 'EOF'
yeet "testz"

test_start("Comprehensive Platform Benchmark")

sus platform_info tea = detect_platform()
vibez.spill("Benchmarking platform: " + platform_info)

// Memory benchmark suite
sus memory_results drip = benchmark_memory_operations()
vibez.spill("Memory benchmark results: " + str(memory_results))

// Scheduler benchmark suite
sus scheduler_results drip = benchmark_scheduler_performance()
vibez.spill("Scheduler benchmark results: " + str(scheduler_results))

// Hardware feature benchmarks
sus simd_results drip = benchmark_simd_operations()
vibez.spill("SIMD benchmark results: " + str(simd_results))

sus crypto_results drip = benchmark_crypto_operations()
vibez.spill("Crypto benchmark results: " + str(crypto_results))

// Generate performance report
generate_performance_report(memory_results, scheduler_results, simd_results, crypto_results)

print_test_summary()
EOF

# Create benchmark execution script
cat > "${BENCHMARK_DIR}/run_benchmark.sh" << 'EOF'
#!/bin/bash

echo "Running platform benchmark..."
cargo run --bin cursed platform_benchmark.csd 2>&1 | tee benchmark_output.log

# Parse results and create summary
echo "Platform: $(uname -m) $(uname -s)" > benchmark_summary.txt
echo "Date: $(date)" >> benchmark_summary.txt
echo "" >> benchmark_summary.txt
grep -E "(Memory|Scheduler|SIMD|Crypto) benchmark results" benchmark_output.log >> benchmark_summary.txt

echo "Benchmark completed. Results saved in benchmark_summary.txt"
EOF

chmod +x "${BENCHMARK_DIR}/run_benchmark.sh"

# Create comparison script
cat > "${BENCHMARK_DIR}/compare_platforms.py" << 'EOF'
#!/usr/bin/env python3
"""
Platform Performance Comparison Tool
Compares benchmark results across different platforms
"""

import json
import sys
from pathlib import Path

def parse_benchmark_results(log_file):
    """Parse benchmark log file and extract performance metrics"""
    results = {}
    with open(log_file, 'r') as f:
        for line in f:
            if "benchmark results:" in line:
                parts = line.split(":")
                metric_name = parts[0].split()[-2]  # Get metric type
                metric_value = parts[1].strip()
                results[metric_name] = metric_value
    return results

def compare_results(baseline_file, comparison_file):
    """Compare two benchmark result files"""
    baseline = parse_benchmark_results(baseline_file)
    comparison = parse_benchmark_results(comparison_file)
    
    print("Performance Comparison")
    print("=====================")
    print(f"Baseline: {baseline_file}")
    print(f"Comparison: {comparison_file}")
    print()
    
    for metric in baseline:
        if metric in comparison:
            baseline_val = float(baseline[metric])
            comparison_val = float(comparison[metric])
            improvement = ((comparison_val - baseline_val) / baseline_val) * 100
            
            print(f"{metric}:")
            print(f"  Baseline: {baseline_val}")
            print(f"  Comparison: {comparison_val}")
            print(f"  Change: {improvement:+.2f}%")
            print()

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python compare_platforms.py <baseline_log> <comparison_log>")
        sys.exit(1)
    
    compare_results(sys.argv[1], sys.argv[2])
EOF

chmod +x "${BENCHMARK_DIR}/compare_platforms.py"

echo "✅ Platform benchmark suite created in: $BENCHMARK_DIR"
echo "📊 Run with: cd $BENCHMARK_DIR && ./run_benchmark.sh"
echo "🔍 Compare results with: ./compare_platforms.py baseline.log comparison.log"
