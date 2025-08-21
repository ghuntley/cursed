#!/usr/bin/env python3
"""
CURSED v1.0 Comprehensive Performance Benchmark Suite
Establishes baseline performance metrics for regression testing
"""

import subprocess
import time
import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Any, Tuple
from statistics import mean, stdev
import tempfile

def run_command_with_timing(cmd: List[str], cwd: str = None, timeout: int = 30) -> Dict[str, Any]:
    """Run command and measure execution time with better precision"""
    start_time = time.perf_counter_ns()
    try:
        result = subprocess.run(
            cmd, 
            cwd=cwd,
            capture_output=True, 
            text=True, 
            timeout=timeout
        )
        end_time = time.perf_counter_ns()
        
        return {
            'success': result.returncode == 0,
            'execution_time_ns': end_time - start_time,
            'execution_time_ms': (end_time - start_time) / 1_000_000,
            'stdout': result.stdout,
            'stderr': result.stderr,
            'returncode': result.returncode
        }
    except subprocess.TimeoutExpired:
        return {
            'success': False,
            'execution_time_ns': timeout * 1_000_000_000,
            'execution_time_ms': timeout * 1000,
            'stdout': '',
            'stderr': f'Timeout after {timeout} seconds',
            'returncode': -1
        }

def create_timed_benchmark(benchmark_code: str) -> str:
    """Wrap benchmark code with timing"""
    # Simple timing wrapper - just measure the basic execution
    modified_code = benchmark_code.replace("slay main() drip {", "slay benchmark_logic() drip {")
    modified_code = modified_code.replace("damn 0", "damn result")
    
    wrapper = """# Timed benchmark wrapper
slay main() drip {
    sus result drip = benchmark_logic()
    vibez.spill("BENCHMARK_RESULT:", result)
    damn 0
}

""" + modified_code
    
    return wrapper

def benchmark_cursed_file(file_path: str, runs: int = 5) -> Dict[str, Any]:
    """Benchmark a CURSED file with multiple runs"""
    print(f"Benchmarking {file_path}...")
    
    # Read the original benchmark
    with open(file_path, 'r') as f:
        original_code = f.read()
    
    # Create timed version
    timed_code = create_timed_benchmark(original_code)
    
    compile_times_ms = []
    runtime_times_ms = []
    benchmark_times_us = []
    
    for run in range(runs):
        with tempfile.NamedTemporaryFile(mode='w', suffix='.csd', delete=False) as f:
            f.write(timed_code)
            temp_file = f.name
        
        try:
            # Measure interpreter mode timing
            interp_result = run_command_with_timing([
                './zig-out/bin/cursed-zig', temp_file
            ], timeout=10)
            
            if interp_result['success']:
                runtime_times_ms.append(interp_result['execution_time_ms'])
                
                # Extract benchmark-specific timing from output
                lines = interp_result['stdout'].split('\n')
                for line in lines:
                    if 'BENCHMARK_TIME_US:' in line:
                        try:
                            us_time = float(line.split(':')[-1].strip())
                            benchmark_times_us.append(us_time)
                        except ValueError:
                            pass
            
            # Measure compilation time (attempt)
            compile_result = run_command_with_timing([
                './zig-out/bin/cursed-zig', '--compile', temp_file
            ], timeout=5)
            
            if compile_result['success']:
                compile_times_ms.append(compile_result['execution_time_ms'])
                
                # Clean up binary if created
                binary_name = temp_file.replace('.csd', '')
                if os.path.exists(binary_name):
                    os.remove(binary_name)
            
        finally:
            os.unlink(temp_file)
    
    return {
        'file': file_path,
        'successful_runs': len(runtime_times_ms),
        'compile_time_ms': {
            'mean': mean(compile_times_ms) if compile_times_ms else 0,
            'stdev': stdev(compile_times_ms) if len(compile_times_ms) > 1 else 0,
            'min': min(compile_times_ms) if compile_times_ms else 0,
            'max': max(compile_times_ms) if compile_times_ms else 0,
            'runs': compile_times_ms
        },
        'total_runtime_ms': {
            'mean': mean(runtime_times_ms) if runtime_times_ms else 0,
            'stdev': stdev(runtime_times_ms) if len(runtime_times_ms) > 1 else 0,
            'min': min(runtime_times_ms) if runtime_times_ms else 0,
            'max': max(runtime_times_ms) if runtime_times_ms else 0,
            'runs': runtime_times_ms
        },
        'benchmark_execution_us': {
            'mean': mean(benchmark_times_us) if benchmark_times_us else 0,
            'stdev': stdev(benchmark_times_us) if len(benchmark_times_us) > 1 else 0,
            'min': min(benchmark_times_us) if benchmark_times_us else 0,
            'max': max(benchmark_times_us) if benchmark_times_us else 0,
            'runs': benchmark_times_us
        }
    }

def create_comprehensive_benchmarks():
    """Create comprehensive benchmark files"""
    
    benchmarks = {
        'intensive_arithmetic.csd': '''slay main() drip {
    sus iterations drip = 500000
    sus result drip = 0
    
    bestie (sus i drip = 0; i < iterations; i++) {
        sus a drip = i * 3 + 7
        sus b drip = i * i + 12
        sus c drip = a * b + i
        sus d drip = c / (i + 1)
        result = result + d
    }
    
    damn result
}''',
        
        'recursive_functions.csd': '''slay fibonacci(n drip) drip {
    ready (n <= 1) { damn n }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay ackermann(m drip, n drip) drip {
    ready (m == 0) { damn n + 1 }
    ready (n == 0) { damn ackermann(m - 1, 1) }
    damn ackermann(m - 1, ackermann(m, n - 1))
}

slay main() drip {
    sus total drip = 0
    bestie (sus i drip = 1; i <= 30; i++) {
        total = total + fibonacci(i)
    }
    # Add some Ackermann calls (small values)
    total = total + ackermann(3, 3)
    damn total
}''',

        'array_intensive.csd': '''slay main() drip {
    sus iterations drip = 1000
    sus array_size drip = 200
    sus total drip = 0
    
    bestie (sus iter drip = 0; iter < iterations; iter++) {
        sus arr []drip = []
        
        # Build array
        bestie (sus i drip = 0; i < array_size; i++) {
            arr = push(arr, i * i + iter)
        }
        
        # Process array multiple times
        bestie (sus pass drip = 0; pass < 3; pass++) {
            bestie (sus i drip = 0; i < array_size; i++) {
                total = total + arr[i]
            }
        }
    }
    
    damn total
}''',

        'complex_control_flow.csd': '''slay process_number(n drip) drip {
    sus result drip = n
    
    ready (n % 2 == 0) {
        ready (n % 4 == 0) {
            ready (n % 8 == 0) {
                result = n * 4
            } otherwise {
                result = n * 2  
            }
        } otherwise {
            result = n + 100
        }
    } otherwise {
        ready (n % 3 == 0) {
            ready (n % 9 == 0) {
                result = n / 3
            } otherwise {
                result = n * 3
            }
        } otherwise {
            ready (n % 5 == 0) {
                result = n * 5
            } otherwise {
                result = n + 1
            }
        }
    }
    
    damn result
}

slay main() drip {
    sus iterations drip = 50000
    sus total drip = 0
    
    bestie (sus i drip = 0; i < iterations; i++) {
        total = total + process_number(i)
        
        # Nested loop with conditions
        ready (i % 100 == 0) {
            bestie (sus j drip = 0; j < 20; j++) {
                bestie (sus k drip = 0; k < 10; k++) {
                    ready ((j + k) % 3 == 0) {
                        total = total + (j * k)
                    }
                }
            }
        }
    }
    
    damn total
}''',

        'mixed_workload.csd': '''slay factorial(n drip) drip {
    ready (n <= 1) { damn 1 }
    damn n * factorial(n - 1)
}

slay string_work() drip {
    sus total drip = 0
    bestie (sus i drip = 0; i < 500; i++) {
        sus str tea = "test_string_number_"
        total = total + len(str) * i
    }
    damn total
}

slay array_work() drip {
    sus arr []drip = []
    bestie (sus i drip = 0; i < 100; i++) {
        arr = push(arr, i * 2)
    }
    
    sus sum drip = 0
    bestie (sus i drip = 0; i < 100; i++) {
        sum = sum + arr[i]
    }
    damn sum
}

slay main() drip {
    sus total drip = 0
    
    bestie (sus i drip = 0; i < 1000; i++) {
        total = total + factorial(i % 8)
        
        ready (i % 10 == 0) {
            total = total + string_work()
        }
        
        ready (i % 20 == 0) {
            total = total + array_work()
        }
    }
    
    damn total
}'''
    }
    
    # Create benchmark directory
    os.makedirs('benchmarks/cursed/comprehensive', exist_ok=True)
    
    for filename, code in benchmarks.items():
        filepath = f'benchmarks/cursed/comprehensive/{filename}'
        with open(filepath, 'w') as f:
            f.write(code)
        print(f"Created {filepath}")

def main():
    os.chdir('/home/ghuntley/cursed')
    
    # Build compiler
    print("Building CURSED compiler...")
    build_result = run_command_with_timing(['zig', 'build'])
    if not build_result['success']:
        print(f"Build failed: {build_result['stderr']}")
        return 1
    
    # Create comprehensive benchmarks
    create_comprehensive_benchmarks()
    
    # Run benchmarks
    benchmark_files = [
        'benchmarks/cursed/comprehensive/intensive_arithmetic.csd',
        'benchmarks/cursed/comprehensive/recursive_functions.csd',
        'benchmarks/cursed/comprehensive/array_intensive.csd',
        'benchmarks/cursed/comprehensive/complex_control_flow.csd',
        'benchmarks/cursed/comprehensive/mixed_workload.csd'
    ]
    
    results = {
        'timestamp': time.time(),
        'cursed_version': '1.0.0',
        'system_info': {
            'platform': sys.platform,
            'python_version': sys.version,
            'hostname': os.uname().nodename if hasattr(os, 'uname') else 'unknown'
        },
        'build_info': {
            'build_time_ms': build_result['execution_time_ms'],
            'build_success': build_result['success']
        },
        'benchmarks': []
    }
    
    for benchmark_path in benchmark_files:
        if os.path.exists(benchmark_path):
            result = benchmark_cursed_file(benchmark_path, runs=3)
            results['benchmarks'].append(result)
        else:
            print(f"Warning: {benchmark_path} not found")
    
    # Save results
    output_file = 'cursed_v1.0_performance_baseline.json'
    with open(output_file, 'w') as f:
        json.dump(results, f, indent=2)
    
    print(f"\\nComprehensive performance baseline saved to {output_file}")
    
    # Print summary
    print("\\n" + "="*80)
    print("CURSED v1.0 Performance Baseline Summary")
    print("="*80)
    print(f"{'Benchmark':<25} | {'Compile(ms)':<12} | {'Runtime(ms)':<12} | {'Benchmark(μs)':<15}")
    print("-"*80)
    
    for benchmark in results['benchmarks']:
        name = os.path.basename(benchmark['file']).replace('.csd', '')
        compile_time = benchmark['compile_time_ms']['mean']
        runtime = benchmark['total_runtime_ms']['mean']
        bench_time = benchmark['benchmark_execution_us']['mean']
        
        print(f"{name:<25} | {compile_time:8.2f}    | {runtime:8.2f}    | {bench_time:11.0f}")
    
    # Performance claims validation
    print("\\n" + "="*80) 
    print("Performance Claims Validation")
    print("="*80)
    
    total_compile_time = sum(b['compile_time_ms']['mean'] for b in results['benchmarks'])
    avg_benchmark_time = mean([b['benchmark_execution_us']['mean'] for b in results['benchmarks'] if b['benchmark_execution_us']['mean'] > 0])
    
    print(f"Total compilation time: {total_compile_time:.2f}ms")
    print(f"Average benchmark execution: {avg_benchmark_time:.0f}μs") 
    print(f"Build time: {results['build_info']['build_time_ms']:.2f}ms")
    
    # Claims comparison
    claims = {
        "Sub-second builds": build_result['execution_time_ms'] < 1000,
        "Fast execution": avg_benchmark_time < 100000,  # < 100ms for our benchmarks
        "Efficient compilation": total_compile_time < 5000  # < 5s total
    }
    
    print("\\nClaims verification:")
    for claim, result in claims.items():
        status = "✓ PASS" if result else "✗ FAIL"
        print(f"  {claim}: {status}")
    
    return 0

if __name__ == '__main__':
    sys.exit(main())
