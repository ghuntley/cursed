#!/usr/bin/env python3
"""
Bootstrap Performance Monitoring System
Tracks performance metrics for self-hosting compiler validation
"""

import json
import time
import subprocess
import os
import sys
import argparse
from datetime import datetime
from typing import Dict, List, Optional, Tuple
import statistics

class PerformanceMonitor:
    def __init__(self, output_dir: str = "performance_data"):
        self.output_dir = output_dir
        os.makedirs(output_dir, exist_ok=True)
        self.results = {}
        
    def time_command(self, command: List[str], description: str) -> Tuple[float, int, str, str]:
        """Time a command execution and return (time_ms, exit_code, stdout, stderr)"""
        print(f"⏱️  Timing: {description}")
        print(f"   Command: {' '.join(command)}")
        
        start_time = time.time()
        try:
            result = subprocess.run(
                command,
                capture_output=True,
                text=True,
                timeout=300  # 5 minute timeout
            )
            end_time = time.time()
            
            execution_time = (end_time - start_time) * 1000  # Convert to milliseconds
            
            print(f"   Time: {execution_time:.2f}ms")
            print(f"   Exit code: {result.returncode}")
            
            return execution_time, result.returncode, result.stdout, result.stderr
            
        except subprocess.TimeoutExpired:
            print(f"   ❌ Command timed out after 300 seconds")
            return -1, 124, "", "Command timed out"
        except Exception as e:
            print(f"   ❌ Command failed: {e}")
            return -1, 1, "", str(e)
    
    def benchmark_compilation(self, compiler_path: str, test_file: str, compiler_name: str) -> Dict:
        """Benchmark a compiler with a test file"""
        print(f"\n🔧 Benchmarking {compiler_name}")
        
        # Test interpretation mode
        interp_time, interp_exit, interp_stdout, interp_stderr = self.time_command(
            [compiler_path, test_file],
            f"{compiler_name} interpretation"
        )
        
        # Test compilation mode
        compile_time, compile_exit, compile_stdout, compile_stderr = self.time_command(
            [compiler_path, "--", "compile", test_file],
            f"{compiler_name} compilation"
        )
        
        return {
            "compiler": compiler_name,
            "interpretation": {
                "time_ms": interp_time,
                "exit_code": interp_exit,
                "success": interp_exit == 0,
                "stdout": interp_stdout,
                "stderr": interp_stderr
            },
            "compilation": {
                "time_ms": compile_time,
                "exit_code": compile_exit,
                "success": compile_exit == 0,
                "stdout": compile_stdout,
                "stderr": compile_stderr
            }
        }
    
    def create_test_programs(self):
        """Create test programs for benchmarking"""
        test_programs = {
            "simple.csd": '''vibez.spill("Hello from CURSED!")''',
            
            "math.csd": '''
sus x normie = 42
sus y normie = 24
sus result normie = x + y * 2
vibez.spill("Math result: " + result.to_string())
''',
            
            "fibonacci.csd": '''
slay fibonacci(n normie) normie {
    lowkey (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

sus result normie = fibonacci(15)
vibez.spill("Fibonacci(15) = " + result.to_string())
''',
            
            "control_flow.csd": '''
sus count normie = 10
periodt (count > 0) {
    lowkey (count % 2 == 0) {
        vibez.spill("Even: " + count.to_string())
    } highkey {
        vibez.spill("Odd: " + count.to_string())
    }
    count = count - 1
}
''',
            
            "complex.csd": '''
slay factorial(n normie) normie {
    lowkey (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

bestie (i := 1; i <= 5; i++) {
    sus result normie = factorial(i)
    vibez.spill("factorial(" + i.to_string() + ") = " + result.to_string())
}
'''
        }
        
        os.makedirs("test_programs", exist_ok=True)
        for filename, content in test_programs.items():
            with open(f"test_programs/{filename}", "w") as f:
                f.write(content)
        
        return list(test_programs.keys())
    
    def run_comprehensive_benchmark(self) -> Dict:
        """Run comprehensive performance benchmarks"""
        print("🚀 Starting comprehensive performance benchmarking")
        
        # Create test programs
        test_files = self.create_test_programs()
        
        # Build original compiler
        print("\n🔨 Building original compiler...")
        build_time, build_exit, _, build_stderr = self.time_command(
            ["cargo", "build", "--release"],
            "Rust compiler build"
        )
        
        if build_exit != 0:
            print(f"❌ Failed to build original compiler: {build_stderr}")
            return {"error": "Failed to build original compiler"}
        
        original_compiler = "./target/release/cursed"
        
        # Try to compile Stage 2
        print("\n🔨 Compiling Stage 2 compiler...")
        stage2_compile_time, stage2_compile_exit, _, stage2_compile_stderr = self.time_command(
            [original_compiler, "--", "compile", "src/bootstrap/stage2/main.csd", "-o", "cursed_stage2"],
            "Stage 2 compilation"
        )
        
        benchmark_results = {
            "timestamp": datetime.now().isoformat(),
            "build_time_ms": build_time,
            "stage2_compile_time_ms": stage2_compile_time,
            "stage2_compile_success": stage2_compile_exit == 0,
            "test_results": {}
        }
        
        # Benchmark original compiler
        for test_file in test_files:
            test_path = f"test_programs/{test_file}"
            print(f"\n📊 Testing with {test_file}")
            
            original_results = self.benchmark_compilation(
                original_compiler, test_path, "original"
            )
            
            benchmark_results["test_results"][test_file] = {
                "original": original_results
            }
            
            # Benchmark Stage 2 if available
            if os.path.exists("cursed_stage2") and stage2_compile_exit == 0:
                stage2_results = self.benchmark_compilation(
                    "./cursed_stage2", test_path, "stage2"
                )
                benchmark_results["test_results"][test_file]["stage2"] = stage2_results
                
                # Calculate performance ratios
                if (original_results["interpretation"]["success"] and 
                    stage2_results["interpretation"]["success"] and
                    original_results["interpretation"]["time_ms"] > 0):
                    
                    ratio = (stage2_results["interpretation"]["time_ms"] / 
                            original_results["interpretation"]["time_ms"] * 100)
                    benchmark_results["test_results"][test_file]["performance_ratio"] = ratio
        
        return benchmark_results
    
    def analyze_performance_regression(self, current_results: Dict, baseline_file: str = None) -> Dict:
        """Analyze performance regression compared to baseline"""
        if baseline_file and os.path.exists(baseline_file):
            with open(baseline_file, 'r') as f:
                baseline_results = json.load(f)
            
            regression_analysis = {
                "has_baseline": True,
                "regressions": [],
                "improvements": []
            }
            
            for test_name, current_test in current_results.get("test_results", {}).items():
                if test_name in baseline_results.get("test_results", {}):
                    baseline_test = baseline_results["test_results"][test_name]
                    
                    # Compare original compiler performance
                    current_time = current_test["original"]["interpretation"]["time_ms"]
                    baseline_time = baseline_test["original"]["interpretation"]["time_ms"]
                    
                    if baseline_time > 0:
                        change_percent = ((current_time - baseline_time) / baseline_time) * 100
                        
                        if change_percent > 10:  # 10% slower is a regression
                            regression_analysis["regressions"].append({
                                "test": test_name,
                                "change_percent": change_percent,
                                "current_ms": current_time,
                                "baseline_ms": baseline_time
                            })
                        elif change_percent < -10:  # 10% faster is an improvement
                            regression_analysis["improvements"].append({
                                "test": test_name,
                                "change_percent": change_percent,
                                "current_ms": current_time,
                                "baseline_ms": baseline_time
                            })
            
            return regression_analysis
        else:
            return {"has_baseline": False}
    
    def generate_report(self, results: Dict, regression_analysis: Dict = None):
        """Generate comprehensive performance report"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        
        # Save raw results
        results_file = f"{self.output_dir}/benchmark_results_{timestamp}.json"
        with open(results_file, 'w') as f:
            json.dump(results, indent=2, fp=f)
        
        # Generate markdown report
        report_file = f"{self.output_dir}/performance_report_{timestamp}.md"
        with open(report_file, 'w') as f:
            f.write("# Bootstrap Performance Report\n\n")
            f.write(f"**Generated:** {datetime.now().isoformat()}\n\n")
            
            # Build information
            f.write("## Build Performance\n\n")
            f.write(f"- **Rust Compiler Build Time:** {results.get('build_time_ms', 0):.2f}ms\n")
            f.write(f"- **Stage 2 Compile Time:** {results.get('stage2_compile_time_ms', 0):.2f}ms\n")
            f.write(f"- **Stage 2 Compile Success:** {'✅' if results.get('stage2_compile_success') else '❌'}\n\n")
            
            # Test results
            f.write("## Test Performance Results\n\n")
            f.write("| Test | Original (ms) | Stage 2 (ms) | Ratio (%) | Status |\n")
            f.write("|------|---------------|---------------|-----------|--------|\n")
            
            for test_name, test_results in results.get("test_results", {}).items():
                original_time = test_results["original"]["interpretation"]["time_ms"]
                
                if "stage2" in test_results:
                    stage2_time = test_results["stage2"]["interpretation"]["time_ms"]
                    ratio = test_results.get("performance_ratio", 0)
                    status = "✅" if ratio <= 150 else "⚠️" if ratio <= 200 else "❌"
                    f.write(f"| {test_name} | {original_time:.2f} | {stage2_time:.2f} | {ratio:.1f} | {status} |\n")
                else:
                    f.write(f"| {test_name} | {original_time:.2f} | N/A | N/A | ⚠️ |\n")
            
            # Regression analysis
            if regression_analysis and regression_analysis.get("has_baseline"):
                f.write("\n## Performance Regression Analysis\n\n")
                
                if regression_analysis["regressions"]:
                    f.write("### 🔴 Performance Regressions\n\n")
                    for reg in regression_analysis["regressions"]:
                        f.write(f"- **{reg['test']}:** {reg['change_percent']:.1f}% slower "
                               f"({reg['current_ms']:.2f}ms vs {reg['baseline_ms']:.2f}ms)\n")
                
                if regression_analysis["improvements"]:
                    f.write("\n### 🟢 Performance Improvements\n\n")
                    for imp in regression_analysis["improvements"]:
                        f.write(f"- **{imp['test']}:** {abs(imp['change_percent']):.1f}% faster "
                               f"({imp['current_ms']:.2f}ms vs {imp['baseline_ms']:.2f}ms)\n")
                
                if not regression_analysis["regressions"] and not regression_analysis["improvements"]:
                    f.write("No significant performance changes detected.\n")
        
        print(f"\n📊 Performance report generated:")
        print(f"   Results: {results_file}")
        print(f"   Report: {report_file}")
        
        return results_file, report_file

def main():
    parser = argparse.ArgumentParser(description="Bootstrap Performance Monitoring")
    parser.add_argument("--baseline", help="Baseline results file for regression analysis")
    parser.add_argument("--output-dir", default="performance_data", help="Output directory")
    
    args = parser.parse_args()
    
    monitor = PerformanceMonitor(args.output_dir)
    
    # Run comprehensive benchmark
    results = monitor.run_comprehensive_benchmark()
    
    if "error" in results:
        print(f"❌ Benchmarking failed: {results['error']}")
        sys.exit(1)
    
    # Analyze regression if baseline provided
    regression_analysis = None
    if args.baseline:
        regression_analysis = monitor.analyze_performance_regression(results, args.baseline)
    
    # Generate report
    results_file, report_file = monitor.generate_report(results, regression_analysis)
    
    # Cleanup
    import shutil
    if os.path.exists("cursed_stage2"):
        os.remove("cursed_stage2")
    if os.path.exists("test_programs"):
        shutil.rmtree("test_programs")
    
    print("\n✅ Performance monitoring completed successfully!")

if __name__ == "__main__":
    main()
