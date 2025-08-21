#!/usr/bin/env python3
"""
Performance Regression Checker for CURSED v1.0
Compares current performance against baseline with ±5% threshold
"""

import json
import sys
import argparse
from typing import Dict, Any, List, Tuple
from pathlib import Path

def load_performance_data(filepath: str) -> Dict[str, Any]:
    """Load performance data from JSON file"""
    try:
        with open(filepath, 'r') as f:
            return json.load(f)
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"Error loading {filepath}: {e}")
        return {}

def calculate_regression(baseline_value: float, current_value: float) -> Tuple[float, bool]:
    """Calculate performance regression percentage"""
    if baseline_value == 0:
        return 0.0, current_value == 0
    
    regression_pct = ((current_value - baseline_value) / baseline_value) * 100
    return regression_pct, True

def check_benchmark_regression(baseline: Dict, current: Dict, threshold: float) -> Dict[str, Any]:
    """Check regression for a single benchmark"""
    benchmark_name = Path(baseline['file']).stem
    
    results = {
        'name': benchmark_name,
        'baseline_runtime_ms': baseline['total_runtime_ms']['mean'],
        'current_runtime_ms': current['total_runtime_ms']['mean'],
        'baseline_compile_ms': baseline['compile_time_ms']['mean'], 
        'current_compile_ms': current['compile_time_ms']['mean'],
        'runtime_regression_pct': 0.0,
        'compile_regression_pct': 0.0,
        'runtime_pass': True,
        'compile_pass': True,
        'overall_pass': True
    }
    
    # Check runtime regression
    if baseline['total_runtime_ms']['mean'] > 0:
        runtime_reg, _ = calculate_regression(
            baseline['total_runtime_ms']['mean'],
            current['total_runtime_ms']['mean']
        )
        results['runtime_regression_pct'] = runtime_reg
        results['runtime_pass'] = abs(runtime_reg) <= threshold
    
    # Check compile time regression  
    if baseline['compile_time_ms']['mean'] > 0:
        compile_reg, _ = calculate_regression(
            baseline['compile_time_ms']['mean'],
            current['compile_time_ms']['mean']
        )
        results['compile_regression_pct'] = compile_reg
        results['compile_pass'] = abs(compile_reg) <= threshold
    
    results['overall_pass'] = results['runtime_pass'] and results['compile_pass']
    
    return results

def generate_report(regression_results: List[Dict], threshold: float) -> str:
    """Generate performance regression report"""
    
    total_benchmarks = len(regression_results)
    passing_benchmarks = sum(1 for r in regression_results if r['overall_pass'])
    
    report = f"""# CURSED v1.0 Performance Regression Report

## Summary
- **Total Benchmarks**: {total_benchmarks}
- **Passing**: {passing_benchmarks}
- **Failing**: {total_benchmarks - passing_benchmarks}
- **Threshold**: ±{threshold}%

## Detailed Results

| Benchmark | Runtime (ms) | Change | Compile (ms) | Change | Status |
|-----------|--------------|--------|--------------|--------|--------|
"""
    
    for result in regression_results:
        name = result['name']
        
        # Runtime info
        baseline_rt = result['baseline_runtime_ms']
        current_rt = result['current_runtime_ms'] 
        rt_change = result['runtime_regression_pct']
        rt_status = "✅" if result['runtime_pass'] else "❌"
        
        # Compile info
        baseline_ct = result['baseline_compile_ms']
        current_ct = result['current_compile_ms']
        ct_change = result['compile_regression_pct'] 
        ct_status = "✅" if result['compile_pass'] else "❌"
        
        overall_status = "✅ PASS" if result['overall_pass'] else "❌ FAIL"
        
        report += f"| {name} | {current_rt:.2f} ({baseline_rt:.2f}) | {rt_change:+.1f}% {rt_status} | {current_ct:.2f} ({baseline_ct:.2f}) | {ct_change:+.1f}% {ct_status} | {overall_status} |\\n"
    
    # Performance insights
    avg_runtime_change = sum(r['runtime_regression_pct'] for r in regression_results) / len(regression_results)
    avg_compile_change = sum(r['compile_regression_pct'] for r in regression_results) / len(regression_results)
    
    report += f"""
## Performance Insights

- **Average Runtime Change**: {avg_runtime_change:+.1f}%
- **Average Compile Change**: {avg_compile_change:+.1f}%

### Regression Analysis
"""
    
    failing_benchmarks = [r for r in regression_results if not r['overall_pass']]
    if failing_benchmarks:
        report += "\\n**❌ Performance Regressions Detected:**\\n"
        for result in failing_benchmarks:
            report += f"- `{result['name']}`: "
            if not result['runtime_pass']:
                report += f"Runtime {result['runtime_regression_pct']:+.1f}% "
            if not result['compile_pass']:
                report += f"Compile {result['compile_regression_pct']:+.1f}% "
            report += "\\n"
    else:
        report += "\\n**✅ No performance regressions detected!**\\n"
    
    return report

def main():
    parser = argparse.ArgumentParser(description='Check CURSED performance regression')
    parser.add_argument('--baseline', required=True, help='Baseline performance JSON file')
    parser.add_argument('--current', required=True, help='Current performance JSON file') 
    parser.add_argument('--threshold', type=float, default=5.0, help='Regression threshold percentage')
    parser.add_argument('--output', default='performance_regression_report.md', help='Output report file')
    
    args = parser.parse_args()
    
    # Load performance data
    baseline_data = load_performance_data(args.baseline)
    current_data = load_performance_data(args.current)
    
    if not baseline_data or not current_data:
        print("Failed to load performance data")
        return 1
    
    # Check regressions for each benchmark
    regression_results = []
    
    baseline_benchmarks = {Path(b['file']).stem: b for b in baseline_data['benchmarks']}
    current_benchmarks = {Path(b['file']).stem: b for b in current_data['benchmarks']}
    
    common_benchmarks = set(baseline_benchmarks.keys()) & set(current_benchmarks.keys())
    
    for benchmark_name in sorted(common_benchmarks):
        baseline_bench = baseline_benchmarks[benchmark_name]
        current_bench = current_benchmarks[benchmark_name]
        
        result = check_benchmark_regression(baseline_bench, current_bench, args.threshold)
        regression_results.append(result)
    
    # Generate and save report
    report = generate_report(regression_results, args.threshold)
    
    with open(args.output, 'w') as f:
        f.write(report)
    
    print(f"Performance regression report saved to {args.output}")
    
    # Print summary to console
    passing = sum(1 for r in regression_results if r['overall_pass'])
    total = len(regression_results)
    
    print(f"\\n📊 Performance Summary: {passing}/{total} benchmarks passed")
    
    if passing == total:
        print("✅ All performance benchmarks passed!")
        return 0
    else:
        print(f"❌ {total - passing} benchmarks failed regression threshold")
        return 1

if __name__ == '__main__':
    sys.exit(main())
