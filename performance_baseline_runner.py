#!/usr/bin/env python3
"""
CURSED v1.0 Performance Baseline Runner
Measures compile-time and runtime performance for regression testing
"""

import subprocess
import time
import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Any
from statistics import mean, stdev

def run_command_with_timing(cmd: List[str], cwd: str = None) -> Dict[str, Any]:
    """Run command and measure execution time"""
    start_time = time.perf_counter()
    try:
        result = subprocess.run(
            cmd, 
            cwd=cwd,
            capture_output=True, 
            text=True, 
            timeout=60
        )
        end_time = time.perf_counter()
        
        return {
            'success': result.returncode == 0,
            'execution_time_ms': (end_time - start_time) * 1000,
            'stdout': result.stdout,
            'stderr': result.stderr,
            'returncode': result.returncode
        }
    except subprocess.TimeoutExpired:
        return {
            'success': False,
            'execution_time_ms': 60000,  # Timeout
            'stdout': '',
            'stderr': 'Timeout after 60 seconds',
            'returncode': -1
        }

def extract_runtime_from_output(output: str) -> float:
    """Extract runtime in microseconds from benchmark output"""
    lines = output.strip().split('\n')
    for line in lines:
        if 'Execution time (μs):' in line:
            try:
                return float(line.split(':')[-1].strip())
            except ValueError:
                continue
    return 0.0

def benchmark_file(file_path: str, runs: int = 3) -> Dict[str, Any]:
    """Benchmark a single CURSED file"""
    print(f"Benchmarking {file_path}...")
    
    # Compile-time measurements
    compile_times = []
    runtime_times = []
    runtime_microseconds = []
    
    for run in range(runs):
        # Measure compilation time
        compile_result = run_command_with_timing([
            './zig-out/bin/cursed-zig', 
            '--compile', 
            file_path
        ])
        
        if compile_result['success']:
            compile_times.append(compile_result['execution_time_ms'])
            
            # Extract binary name
            binary_name = file_path.replace('.csd', '')
            if os.path.exists(binary_name):
                # Measure runtime
                runtime_result = run_command_with_timing([f'./{binary_name}'])
                if runtime_result['success']:
                    runtime_times.append(runtime_result['execution_time_ms'])
                    runtime_us = extract_runtime_from_output(runtime_result['stdout'])
                    if runtime_us > 0:
                        runtime_microseconds.append(runtime_us)
                
                # Cleanup binary
                os.remove(binary_name)
        
        # Also test interpreter mode
        interp_result = run_command_with_timing([
            './zig-out/bin/cursed-zig',
            file_path
        ])
        
        if interp_result['success']:
            runtime_us = extract_runtime_from_output(interp_result['stdout'])
            if runtime_us > 0:
                runtime_microseconds.append(runtime_us)
    
    return {
        'file': file_path,
        'compile_time_ms': {
            'mean': mean(compile_times) if compile_times else 0,
            'stdev': stdev(compile_times) if len(compile_times) > 1 else 0,
            'runs': compile_times
        },
        'runtime_ms': {
            'mean': mean(runtime_times) if runtime_times else 0,
            'stdev': stdev(runtime_times) if len(runtime_times) > 1 else 0,
            'runs': runtime_times
        },
        'benchmark_runtime_us': {
            'mean': mean(runtime_microseconds) if runtime_microseconds else 0,
            'stdev': stdev(runtime_microseconds) if len(runtime_microseconds) > 1 else 0,
            'runs': runtime_microseconds
        }
    }

def main():
    # Ensure we're in the right directory
    os.chdir('/home/ghuntley/cursed')
    
    # Build the compiler first
    print("Building CURSED compiler...")
    build_result = run_command_with_timing(['zig', 'build'])
    if not build_result['success']:
        print(f"Build failed: {build_result['stderr']}")
        return 1
    
    # Define benchmark files - using simple versions that work with current implementation
    benchmark_files = [
        'benchmarks/cursed/simple_arithmetic.csd',
        'benchmarks/cursed/simple_functions.csd', 
        'benchmarks/cursed/simple_arrays.csd',
        'benchmarks/cursed/simple_strings.csd',
        'benchmarks/cursed/simple_control_flow.csd'
    ]
    
    # Run benchmarks
    results = {
        'timestamp': time.time(),
        'cursed_version': '1.0.0',
        'system_info': {
            'platform': sys.platform,
            'python_version': sys.version
        },
        'benchmarks': []
    }
    
    for benchmark_path in benchmark_files:
        if os.path.exists(benchmark_path):
            result = benchmark_file(benchmark_path, runs=5)
            results['benchmarks'].append(result)
        else:
            print(f"Warning: {benchmark_path} not found")
    
    # Save results
    output_file = 'performance_baseline_v1.0.json'
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\nBaseline performance results saved to {output_file}")
    
    # Print summary
    print("\n=== CURSED v1.0 Performance Baseline Summary ===")
    for benchmark in results['benchmarks']:
        name = os.path.basename(benchmark['file']).replace('.csd', '')
        compile_time = benchmark['compile_time_ms']['mean']
        runtime = benchmark['benchmark_runtime_us']['mean']
        print(f"{name:20} | Compile: {compile_time:6.2f}ms | Runtime: {runtime:8.0f}μs")
    
    return 0

if __name__ == '__main__':
    sys.exit(main())
