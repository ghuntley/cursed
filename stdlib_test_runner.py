#!/usr/bin/env python3
"""
Comprehensive stdlib test runner for CURSED language.
Executes all stdlib module tests with performance monitoring and coverage reporting.
"""

import os
import subprocess
import time
import json
from pathlib import Path
from typing import Dict, List, Tuple

class StdlibTestRunner:
    def __init__(self):
        self.test_results = {}
        self.performance_metrics = {}
        self.coverage_data = {}
        
    def run_module_test(self, module_name: str, test_file: Path) -> Tuple[bool, float, str]:
        """Run a single module test and return results."""
        start_time = time.time()
        
        try:
            # Test interpretation mode
            interp_result = subprocess.run(
                ['cargo', 'run', '--bin', 'cursed', str(test_file)],
                capture_output=True,
                text=True,
                timeout=30
            )
            
            # Test compilation mode
            comp_result = subprocess.run(
                ['cargo', 'run', '--bin', 'cursed', '--', 'compile', str(test_file)],
                capture_output=True,
                text=True,
                timeout=30
            )
            
            executable = test_file.parent / test_file.stem
            if executable.exists():
                exec_result = subprocess.run(
                    [str(executable)],
                    capture_output=True,
                    text=True,
                    timeout=30
                )
                os.remove(executable)
            else:
                exec_result = None
                
            execution_time = time.time() - start_time
            
            # Check results
            success = (interp_result.returncode == 0 and 
                      comp_result.returncode == 0 and
                      (exec_result is None or exec_result.returncode == 0))
            
            output = f"Interp: {interp_result.stdout}\nComp: {comp_result.stdout}"
            if exec_result:
                output += f"\nExec: {exec_result.stdout}"
                
            return success, execution_time, output
            
        except subprocess.TimeoutExpired:
            return False, time.time() - start_time, "Test timed out"
        except Exception as e:
            return False, time.time() - start_time, f"Error: {str(e)}"
    
    def run_all_tests(self) -> Dict:
        """Run all stdlib module tests."""
        stdlib_dir = Path("stdlib")
        total_tests = 0
        passed_tests = 0
        
        print("Running comprehensive stdlib test suite...")
        print("=" * 60)
        
        for module_dir in stdlib_dir.iterdir():
            if module_dir.is_dir() and not module_dir.name.startswith('.'):
                module_name = module_dir.name
                test_files = list(module_dir.glob('test_*.csd')) + list(module_dir.glob('*_test.csd'))
                
                if not test_files:
                    print(f"❌ {module_name}: No test files found")
                    continue
                    
                module_results = []
                for test_file in test_files:
                    total_tests += 1
                    success, exec_time, output = self.run_module_test(module_name, test_file)
                    
                    if success:
                        passed_tests += 1
                        print(f"✅ {module_name}/{test_file.name}: PASSED ({exec_time:.2f}s)")
                    else:
                        print(f"❌ {module_name}/{test_file.name}: FAILED ({exec_time:.2f}s)")
                        if output:
                            print(f"   Output: {output[:100]}...")
                    
                    module_results.append({
                        'test_file': test_file.name,
                        'success': success,
                        'execution_time': exec_time,
                        'output': output
                    })
                
                self.test_results[module_name] = module_results
        
        print("=" * 60)
        print(f"Test Summary: {passed_tests}/{total_tests} tests passed ({(passed_tests/total_tests)*100:.1f}%)")
        
        return {
            'total_tests': total_tests,
            'passed_tests': passed_tests,
            'pass_rate': (passed_tests/total_tests)*100 if total_tests > 0 else 0,
            'results': self.test_results
        }
    
    def generate_coverage_report(self) -> str:
        """Generate detailed coverage report."""
        report = "# Stdlib Test Coverage Report\n\n"
        
        for module_name, results in self.test_results.items():
            total_tests = len(results)
            passed_tests = sum(1 for r in results if r['success'])
            pass_rate = (passed_tests / total_tests) * 100 if total_tests > 0 else 0
            
            status = "✅" if pass_rate >= 80 else "⚠️" if pass_rate >= 50 else "❌"
            report += f"{status} **{module_name}**: {passed_tests}/{total_tests} ({pass_rate:.1f}%)\n"
        
        return report
    
    def run_performance_benchmarks(self):
        """Run performance benchmarks for critical modules."""
        critical_modules = ['math', 'string', 'crypto', 'collections', 'io', 'json']
        
        print("Running performance benchmarks...")
        for module in critical_modules:
            module_dir = Path(f"stdlib/{module}")
            if module_dir.exists():
                # Run benchmark tests
                benchmark_files = list(module_dir.glob('benchmark_*.csd'))
                for benchmark_file in benchmark_files:
                    print(f"📊 Benchmarking {module}/{benchmark_file.name}")
                    # TODO: Implement benchmark execution

if __name__ == "__main__":
    runner = StdlibTestRunner()
    results = runner.run_all_tests()
    
    # Generate reports
    with open("stdlib_test_results.json", "w") as f:
        json.dump(results, f, indent=2)
    
    coverage_report = runner.generate_coverage_report()
    with open("stdlib_coverage_report.md", "w") as f:
        f.write(coverage_report)
    
    print("\nReports generated:")
    print("- stdlib_test_results.json")
    print("- stdlib_coverage_report.md")
