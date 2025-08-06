#!/usr/bin/env python3

"""
CURSED Performance Profiler
Comprehensive performance analysis and benchmarking for production deployment
"""

import os
import sys
import json
import time
import psutil
import subprocess
import statistics
from pathlib import Path
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, asdict
from datetime import datetime
import matplotlib.pyplot as plt
import numpy as np

@dataclass
class BenchmarkResult:
    """Result of a single benchmark run"""
    name: str
    duration_ms: float
    memory_peak_mb: float
    cpu_percent: float
    exit_code: int
    timestamp: str

@dataclass 
class PerformanceMetrics:
    """Complete performance metrics for a test"""
    test_name: str
    runs: List[BenchmarkResult]
    avg_duration_ms: float
    min_duration_ms: float
    max_duration_ms: float
    std_deviation_ms: float
    avg_memory_mb: float
    avg_cpu_percent: float
    success_rate: float

class PerformanceProfiler:
    """Comprehensive performance profiler for CURSED"""
    
    def __init__(self, project_root: str):
        self.project_root = Path(project_root)
        self.cursed_binary = self.project_root / "zig-out" / "bin" / "cursed"
        self.results_dir = self.project_root / "performance_results"
        self.results_dir.mkdir(exist_ok=True)
        
        # Test configurations
        self.test_programs = {
            "hello_world": 'vibez.spill("Hello, World!")',
            "arithmetic": '''
                sus a drip = 42
                sus b drip = 24
                sus result drip = (a + b) * 2
                vibez.spill(result.(tea))
            ''',
            "fibonacci": '''
                slay fibonacci(n drip) drip {
                    ready n <= 1 {
                        damn 1
                    }
                    damn fibonacci(n-1) + fibonacci(n-2)
                }
                vibez.spill(fibonacci(25).(tea))
            ''',
            "loop_intensive": '''
                sus total drip = 0
                bestie i drip = 0; i < 10000; i = i + 1 {
                    total = total + i
                }
                vibez.spill(total.(tea))
            ''',
            "string_processing": '''
                sus message tea = "CURSED is fast"
                bestie i drip = 0; i < 1000; i = i + 1 {
                    message = message + " and efficient"
                }
                vibez.spill(message)
            ''',
            "memory_allocation": '''
                slay create_array(size drip) [] drip {
                    sus arr [] drip = []
                    bestie i drip = 0; i < size; i = i + 1 {
                        arr.append(i)
                    }
                    damn arr
                }
                sus large_array [] drip = create_array(10000)
                vibez.spill(large_array.length.(tea))
            '''
        }
        
        # Compilation benchmarks
        self.compilation_tests = {
            "small_file": "basic_test.csd",
            "medium_file": "comprehensive_stdlib_test.csd", 
            "large_file": "comprehensive_advanced_test.csd"
        }
    
    def check_prerequisites(self) -> bool:
        """Check if all required tools are available"""
        if not self.cursed_binary.exists():
            print(f"❌ CURSED binary not found at {self.cursed_binary}")
            return False
        
        # Check for optional tools
        tools = ["hyperfine", "valgrind", "perf"]
        available_tools = []
        
        for tool in tools:
            if subprocess.run(["which", tool], capture_output=True).returncode == 0:
                available_tools.append(tool)
            else:
                print(f"⚠️  {tool} not available - some benchmarks will be skipped")
        
        return True
    
    def run_single_benchmark(self, test_name: str, program_code: str, 
                           timeout: int = 30) -> BenchmarkResult:
        """Run a single benchmark and collect metrics"""
        # Create temporary test file
        test_file = self.results_dir / f"{test_name}_temp.csd"
        test_file.write_text(program_code)
        
        try:
            # Start monitoring
            start_time = time.time()
            process = psutil.Popen([str(self.cursed_binary), str(test_file)],
                                 stdout=subprocess.PIPE,
                                 stderr=subprocess.PIPE)
            
            # Monitor resource usage
            memory_samples = []
            cpu_samples = []
            
            while process.is_running():
                try:
                    memory_mb = process.memory_info().rss / 1024 / 1024
                    cpu_percent = process.cpu_percent()
                    memory_samples.append(memory_mb)
                    cpu_samples.append(cpu_percent)
                    time.sleep(0.01)  # Sample every 10ms
                except (psutil.NoSuchProcess, psutil.AccessDenied):
                    break
            
            # Wait for completion
            exit_code = process.wait(timeout=timeout)
            end_time = time.time()
            
            duration_ms = (end_time - start_time) * 1000
            memory_peak_mb = max(memory_samples) if memory_samples else 0
            avg_cpu_percent = statistics.mean(cpu_samples) if cpu_samples else 0
            
            return BenchmarkResult(
                name=test_name,
                duration_ms=duration_ms,
                memory_peak_mb=memory_peak_mb,
                cpu_percent=avg_cpu_percent,
                exit_code=exit_code,
                timestamp=datetime.now().isoformat()
            )
            
        except subprocess.TimeoutExpired:
            process.kill()
            return BenchmarkResult(
                name=test_name,
                duration_ms=timeout * 1000,
                memory_peak_mb=0,
                cpu_percent=0,
                exit_code=-1,
                timestamp=datetime.now().isoformat()
            )
        finally:
            # Clean up
            test_file.unlink(missing_ok=True)
    
    def run_execution_benchmarks(self, num_runs: int = 10) -> Dict[str, PerformanceMetrics]:
        """Run execution benchmarks for all test programs"""
        print("🚀 Running execution benchmarks...")
        results = {}
        
        for test_name, program_code in self.test_programs.items():
            print(f"  📊 Benchmarking {test_name}...")
            
            runs = []
            for i in range(num_runs):
                print(f"    Run {i+1}/{num_runs}", end="\r")
                result = self.run_single_benchmark(test_name, program_code)
                runs.append(result)
            
            # Calculate metrics
            successful_runs = [r for r in runs if r.exit_code == 0]
            
            if successful_runs:
                durations = [r.duration_ms for r in successful_runs]
                memories = [r.memory_peak_mb for r in successful_runs]
                cpus = [r.cpu_percent for r in successful_runs]
                
                metrics = PerformanceMetrics(
                    test_name=test_name,
                    runs=runs,
                    avg_duration_ms=statistics.mean(durations),
                    min_duration_ms=min(durations),
                    max_duration_ms=max(durations),
                    std_deviation_ms=statistics.stdev(durations) if len(durations) > 1 else 0,
                    avg_memory_mb=statistics.mean(memories),
                    avg_cpu_percent=statistics.mean(cpus),
                    success_rate=len(successful_runs) / len(runs)
                )
            else:
                # All runs failed
                metrics = PerformanceMetrics(
                    test_name=test_name,
                    runs=runs,
                    avg_duration_ms=0,
                    min_duration_ms=0,
                    max_duration_ms=0,
                    std_deviation_ms=0,
                    avg_memory_mb=0,
                    avg_cpu_percent=0,
                    success_rate=0
                )
            
            results[test_name] = metrics
            print(f"    ✅ {test_name}: {metrics.avg_duration_ms:.1f}ms avg")
        
        return results
    
    def run_compilation_benchmarks(self) -> Dict[str, PerformanceMetrics]:
        """Run compilation benchmarks"""
        print("🔧 Running compilation benchmarks...")
        results = {}
        
        for test_name, file_name in self.compilation_tests.items():
            test_file = self.project_root / file_name
            
            if not test_file.exists():
                print(f"⚠️  Test file {file_name} not found, skipping")
                continue
            
            print(f"  📊 Benchmarking compilation of {test_name}...")
            
            runs = []
            for i in range(5):  # Fewer runs for compilation tests
                print(f"    Run {i+1}/5", end="\r")
                
                start_time = time.time()
                result = subprocess.run([
                    str(self.cursed_binary), "--compile", str(test_file)
                ], capture_output=True)
                end_time = time.time()
                
                duration_ms = (end_time - start_time) * 1000
                
                benchmark_result = BenchmarkResult(
                    name=f"{test_name}_compilation",
                    duration_ms=duration_ms,
                    memory_peak_mb=0,  # Would need more complex monitoring
                    cpu_percent=0,
                    exit_code=result.returncode,
                    timestamp=datetime.now().isoformat()
                )
                runs.append(benchmark_result)
            
            # Calculate metrics
            successful_runs = [r for r in runs if r.exit_code == 0]
            
            if successful_runs:
                durations = [r.duration_ms for r in successful_runs]
                
                metrics = PerformanceMetrics(
                    test_name=f"{test_name}_compilation",
                    runs=runs,
                    avg_duration_ms=statistics.mean(durations),
                    min_duration_ms=min(durations),
                    max_duration_ms=max(durations),
                    std_deviation_ms=statistics.stdev(durations) if len(durations) > 1 else 0,
                    avg_memory_mb=0,
                    avg_cpu_percent=0,
                    success_rate=len(successful_runs) / len(runs)
                )
            else:
                metrics = PerformanceMetrics(
                    test_name=f"{test_name}_compilation",
                    runs=runs,
                    avg_duration_ms=0,
                    min_duration_ms=0,
                    max_duration_ms=0,
                    std_deviation_ms=0,
                    avg_memory_mb=0,
                    avg_cpu_percent=0,
                    success_rate=0
                )
            
            results[f"{test_name}_compilation"] = metrics
            print(f"    ✅ {test_name}: {metrics.avg_duration_ms:.1f}ms avg")
        
        return results
    
    def run_hyperfine_benchmarks(self) -> Dict[str, Dict]:
        """Run benchmarks using hyperfine if available"""
        if subprocess.run(["which", "hyperfine"], capture_output=True).returncode != 0:
            print("⚠️  hyperfine not available, skipping precision benchmarks")
            return {}
        
        print("⚡ Running precision benchmarks with hyperfine...")
        results = {}
        
        for test_name, program_code in self.test_programs.items():
            # Create test file
            test_file = self.results_dir / f"{test_name}_hyperfine.csd"
            test_file.write_text(program_code)
            
            try:
                # Run hyperfine
                result = subprocess.run([
                    "hyperfine",
                    "--json",
                    "--warmup", "3",
                    "--min-runs", "10",
                    f"{self.cursed_binary} {test_file}"
                ], capture_output=True, text=True)
                
                if result.returncode == 0:
                    data = json.loads(result.stdout)
                    results[test_name] = data
                    
                    # Extract key metrics
                    if data.get("results"):
                        benchmark = data["results"][0]
                        mean_ms = benchmark["mean"] * 1000
                        print(f"  ✅ {test_name}: {mean_ms:.1f}ms (hyperfine)")
                
            except (subprocess.CalledProcessError, json.JSONDecodeError):
                print(f"  ⚠️  hyperfine failed for {test_name}")
            finally:
                test_file.unlink(missing_ok=True)
        
        return results
    
    def run_memory_profiling(self) -> Dict[str, Dict]:
        """Run memory profiling with valgrind if available"""
        if subprocess.run(["which", "valgrind"], capture_output=True).returncode != 0:
            print("⚠️  valgrind not available, skipping memory profiling")
            return {}
        
        print("🧠 Running memory profiling with valgrind...")
        results = {}
        
        # Test with a subset of programs (valgrind is slow)
        memory_test_programs = {
            "hello_world": self.test_programs["hello_world"],
            "memory_allocation": self.test_programs["memory_allocation"]
        }
        
        for test_name, program_code in memory_test_programs.items():
            test_file = self.results_dir / f"{test_name}_valgrind.csd"
            test_file.write_text(program_code)
            
            try:
                # Run valgrind
                result = subprocess.run([
                    "valgrind",
                    "--tool=massif",
                    "--stacks=yes",
                    f"--massif-out-file={self.results_dir}/{test_name}_massif.out",
                    str(self.cursed_binary), str(test_file)
                ], capture_output=True, text=True, timeout=60)
                
                # Parse massif output
                massif_file = self.results_dir / f"{test_name}_massif.out"
                if massif_file.exists():
                    memory_data = self._parse_massif_output(massif_file)
                    results[test_name] = memory_data
                    print(f"  ✅ {test_name}: {memory_data.get('peak_mb', 0):.1f}MB peak")
                
            except subprocess.TimeoutExpired:
                print(f"  ⚠️  valgrind timeout for {test_name}")
            finally:
                test_file.unlink(missing_ok=True)
        
        return results
    
    def _parse_massif_output(self, massif_file: Path) -> Dict:
        """Parse valgrind massif output"""
        try:
            content = massif_file.read_text()
            lines = content.split('\n')
            
            peak_bytes = 0
            for line in lines:
                if line.startswith('mem_heap_B='):
                    bytes_val = int(line.split('=')[1])
                    peak_bytes = max(peak_bytes, bytes_val)
            
            return {
                'peak_bytes': peak_bytes,
                'peak_mb': peak_bytes / 1024 / 1024
            }
        except Exception:
            return {'peak_bytes': 0, 'peak_mb': 0}
    
    def generate_performance_graphs(self, execution_results: Dict[str, PerformanceMetrics]):
        """Generate performance visualization graphs"""
        print("📊 Generating performance graphs...")
        
        try:
            # Performance comparison bar chart
            test_names = list(execution_results.keys())
            avg_durations = [metrics.avg_duration_ms for metrics in execution_results.values()]
            
            plt.figure(figsize=(12, 6))
            bars = plt.bar(test_names, avg_durations, color='steelblue', alpha=0.7)
            plt.xlabel('Test')
            plt.ylabel('Average Duration (ms)')
            plt.title('CURSED Performance Benchmarks')
            plt.xticks(rotation=45)
            
            # Add value labels on bars
            for bar, duration in zip(bars, avg_durations):
                plt.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.5,
                        f'{duration:.1f}ms', ha='center', va='bottom')
            
            plt.tight_layout()
            plt.savefig(self.results_dir / 'performance_comparison.png', dpi=300, bbox_inches='tight')
            plt.close()
            
            # Memory usage chart
            memory_usage = [metrics.avg_memory_mb for metrics in execution_results.values()]
            
            plt.figure(figsize=(12, 6))
            bars = plt.bar(test_names, memory_usage, color='green', alpha=0.7)
            plt.xlabel('Test')
            plt.ylabel('Average Memory Usage (MB)')
            plt.title('CURSED Memory Usage')
            plt.xticks(rotation=45)
            
            # Add value labels
            for bar, memory in zip(bars, memory_usage):
                plt.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.1,
                        f'{memory:.1f}MB', ha='center', va='bottom')
            
            plt.tight_layout()
            plt.savefig(self.results_dir / 'memory_usage.png', dpi=300, bbox_inches='tight')
            plt.close()
            
            print("  ✅ Graphs saved to performance_results/")
            
        except ImportError:
            print("  ⚠️  matplotlib not available, skipping graph generation")
        except Exception as e:
            print(f"  ⚠️  Graph generation failed: {e}")
    
    def generate_report(self, execution_results: Dict[str, PerformanceMetrics],
                       compilation_results: Dict[str, PerformanceMetrics],
                       hyperfine_results: Dict[str, Dict],
                       memory_results: Dict[str, Dict]) -> Dict:
        """Generate comprehensive performance report"""
        report = {
            'timestamp': datetime.now().isoformat(),
            'system_info': {
                'cpu_count': psutil.cpu_count(),
                'memory_gb': psutil.virtual_memory().total / 1024 / 1024 / 1024,
                'platform': sys.platform
            },
            'execution_benchmarks': {
                test_name: asdict(metrics) for test_name, metrics in execution_results.items()
            },
            'compilation_benchmarks': {
                test_name: asdict(metrics) for test_name, metrics in compilation_results.items()
            },
            'hyperfine_results': hyperfine_results,
            'memory_profiling': memory_results,
            'summary': {
                'fastest_execution_ms': min(m.avg_duration_ms for m in execution_results.values()) if execution_results else 0,
                'slowest_execution_ms': max(m.avg_duration_ms for m in execution_results.values()) if execution_results else 0,
                'avg_memory_usage_mb': statistics.mean(m.avg_memory_mb for m in execution_results.values()) if execution_results else 0,
                'success_rate': statistics.mean(m.success_rate for m in execution_results.values()) if execution_results else 0
            }
        }
        
        return report
    
    def save_report(self, report: Dict, output_file: str):
        """Save performance report to file"""
        output_path = self.results_dir / output_file
        
        with open(output_path, 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"📄 Performance report saved to {output_path}")
    
    def print_summary(self, execution_results: Dict[str, PerformanceMetrics]):
        """Print performance summary"""
        print("\n🏁 Performance Summary")
        print("=" * 50)
        
        if execution_results:
            fastest = min(execution_results.values(), key=lambda x: x.avg_duration_ms)
            slowest = max(execution_results.values(), key=lambda x: x.avg_duration_ms)
            
            print(f"Fastest Test: {fastest.test_name} ({fastest.avg_duration_ms:.1f}ms)")
            print(f"Slowest Test: {slowest.test_name} ({slowest.avg_duration_ms:.1f}ms)")
            
            avg_memory = statistics.mean(m.avg_memory_mb for m in execution_results.values())
            print(f"Average Memory: {avg_memory:.1f}MB")
            
            success_rate = statistics.mean(m.success_rate for m in execution_results.values())
            print(f"Overall Success Rate: {success_rate:.1%}")
            
            if success_rate < 1.0:
                print("⚠️  Some tests failed - check individual results")
            else:
                print("✅ All tests passed")
        else:
            print("❌ No benchmark results available")

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="CURSED Performance Profiler")
    parser.add_argument("--project-root", default=".", help="Project root directory")
    parser.add_argument("--runs", type=int, default=10, help="Number of runs per benchmark")
    parser.add_argument("--output", default="performance_report.json", help="Output report file")
    parser.add_argument("--skip-hyperfine", action="store_true", help="Skip hyperfine benchmarks")
    parser.add_argument("--skip-valgrind", action="store_true", help="Skip valgrind profiling")
    parser.add_argument("--graphs", action="store_true", help="Generate performance graphs")
    
    args = parser.parse_args()
    
    profiler = PerformanceProfiler(args.project_root)
    
    if not profiler.check_prerequisites():
        print("❌ Prerequisites check failed")
        sys.exit(1)
    
    print("🚀 Starting CURSED performance profiling...")
    
    # Run benchmarks
    execution_results = profiler.run_execution_benchmarks(args.runs)
    compilation_results = profiler.run_compilation_benchmarks()
    
    hyperfine_results = {}
    if not args.skip_hyperfine:
        hyperfine_results = profiler.run_hyperfine_benchmarks()
    
    memory_results = {}
    if not args.skip_valgrind:
        memory_results = profiler.run_memory_profiling()
    
    # Generate graphs if requested
    if args.graphs:
        profiler.generate_performance_graphs(execution_results)
    
    # Generate and save report
    report = profiler.generate_report(execution_results, compilation_results, 
                                     hyperfine_results, memory_results)
    profiler.save_report(report, args.output)
    
    # Print summary
    profiler.print_summary(execution_results)
    
    print("\n✅ Performance profiling completed")

if __name__ == "__main__":
    main()
