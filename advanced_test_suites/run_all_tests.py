#!/usr/bin/env python3
"""
Advanced Test Suite Runner for CURSED Language
Comprehensive test execution with detailed reporting and CI/CD integration
"""

import os
import sys
import subprocess
import time
import json
import xml.etree.ElementTree as ET
from pathlib import Path
from typing import Dict, List, Optional, Tuple
import argparse
import concurrent.futures
from dataclasses import dataclass
from datetime import datetime
import platform
# import psutil  # Optional dependency for system monitoring

@dataclass
class TestResult:
    name: str
    category: str
    status: str  # "pass", "fail", "skip", "error"
    duration: float
    output: str
    error_message: Optional[str] = None
    memory_usage: Optional[int] = None
    cpu_usage: Optional[float] = None

class TestRunner:
    def __init__(self, cursed_executable: str = "./zig-out/bin/cursed-zig"):
        self.cursed_executable = cursed_executable
        self.test_results: List[TestResult] = []
        self.start_time = time.time()
        
        # Test categories and their files
        self.test_suites = {
            "edge_cases": [
                "advanced_test_suites/edge_cases/boundary_conditions.csd",
                "advanced_test_suites/edge_cases/malformed_input.csd"
            ],
            "performance": [
                "advanced_test_suites/performance/benchmark_suite.csd"
            ],
            "integration": [
                "advanced_test_suites/integration/module_integration.csd"
            ],
            "stress": [
                "advanced_test_suites/stress/resource_exhaustion.csd"
            ],
            "security": [
                "advanced_test_suites/security/injection_prevention.csd"
            ],
            "cross_platform": [
                "advanced_test_suites/cross_platform/platform_validation.csd"
            ]
        }
        
        # System information
        try:
            import psutil
            cpu_cores = psutil.cpu_count()
            memory_total = psutil.virtual_memory().total
        except ImportError:
            cpu_cores = os.cpu_count() or 1
            memory_total = 0
        
        self.system_info = {
            "platform": platform.system(),
            "architecture": platform.machine(),
            "python_version": platform.python_version(),
            "cpu_cores": cpu_cores,
            "memory_total": memory_total,
            "timestamp": datetime.now().isoformat()
        }

    def run_single_test(self, test_file: str, category: str, timeout: int = 300) -> TestResult:
        """Run a single test file and collect results"""
        test_name = os.path.basename(test_file)
        print(f"Running {category}/{test_name}...")
        
        start_time = time.time()
        process = None
        
        try:
            # Monitor system resources before test (if psutil available)
            try:
                import psutil
                memory_before = psutil.virtual_memory().used
            except ImportError:
                memory_before = 0
            
            # Run the test
            process = subprocess.Popen(
                [self.cursed_executable, test_file],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True
            )
            
            stdout, stderr = process.communicate(timeout=timeout)
            duration = time.time() - start_time
            
            # Monitor system resources after test (if psutil available)
            try:
                import psutil
                memory_after = psutil.virtual_memory().used
                memory_usage = memory_after - memory_before
            except ImportError:
                memory_usage = None
            
            # Determine test status based on output and return code
            if process.returncode == 0:
                if "FAIL" in stdout or "ERROR" in stdout:
                    status = "fail"
                    error_message = "Test failures detected in output"
                elif "PASS" in stdout or "All tests passed" in stdout:
                    status = "pass"
                    error_message = None
                else:
                    status = "pass"  # Assume pass if no explicit failures
                    error_message = None
            else:
                status = "error"
                error_message = f"Process exited with code {process.returncode}"
                if stderr:
                    error_message += f": {stderr[:500]}"
            
            return TestResult(
                name=test_name,
                category=category,
                status=status,
                duration=duration,
                output=stdout,
                error_message=error_message,
                memory_usage=memory_usage
            )
            
        except subprocess.TimeoutExpired:
            if process:
                process.kill()
            duration = time.time() - start_time
            return TestResult(
                name=test_name,
                category=category,
                status="error",
                duration=duration,
                output="",
                error_message=f"Test timed out after {timeout} seconds"
            )
        except Exception as e:
            duration = time.time() - start_time
            return TestResult(
                name=test_name,
                category=category,
                status="error",
                duration=duration,
                output="",
                error_message=f"Exception: {str(e)}"
            )

    def run_category(self, category: str, parallel: bool = False) -> List[TestResult]:
        """Run all tests in a category"""
        if category not in self.test_suites:
            print(f"Unknown test category: {category}")
            return []
        
        test_files = self.test_suites[category]
        results = []
        
        if parallel and len(test_files) > 1:
            print(f"Running {len(test_files)} tests in {category} category in parallel...")
            with concurrent.futures.ThreadPoolExecutor(max_workers=4) as executor:
                future_to_file = {
                    executor.submit(self.run_single_test, test_file, category): test_file 
                    for test_file in test_files
                }
                
                for future in concurrent.futures.as_completed(future_to_file):
                    test_file = future_to_file[future]
                    try:
                        result = future.result()
                        results.append(result)
                    except Exception as e:
                        print(f"Error running {test_file}: {e}")
                        results.append(TestResult(
                            name=os.path.basename(test_file),
                            category=category,
                            status="error",
                            duration=0.0,
                            output="",
                            error_message=f"Execution error: {str(e)}"
                        ))
        else:
            for test_file in test_files:
                result = self.run_single_test(test_file, category)
                results.append(result)
        
        return results

    def run_all_tests(self, categories: Optional[List[str]] = None, parallel: bool = False) -> None:
        """Run all test categories"""
        if categories is None:
            categories = list(self.test_suites.keys())
        
        print("="*60)
        print("CURSED Advanced Test Suite Runner")
        print("="*60)
        print(f"System: {self.system_info['platform']} {self.system_info['architecture']}")
        print(f"CPU Cores: {self.system_info['cpu_cores']}")
        memory_gb = self.system_info['memory_total'] // (1024**3) if self.system_info['memory_total'] > 0 else 0
        print(f"Memory: {memory_gb} GB")
        print(f"Executable: {self.cursed_executable}")
        print("="*60)
        
        # Verify executable exists
        if not os.path.exists(self.cursed_executable):
            print(f"ERROR: CURSED executable not found at {self.cursed_executable}")
            print("Please build the project first with 'zig build'")
            sys.exit(1)
        
        for category in categories:
            print(f"\n{'='*20} {category.upper()} TESTS {'='*20}")
            results = self.run_category(category, parallel)
            self.test_results.extend(results)
            
            # Print immediate results for this category
            passed = sum(1 for r in results if r.status == "pass")
            failed = sum(1 for r in results if r.status == "fail")
            errors = sum(1 for r in results if r.status == "error")
            
            print(f"\n{category.upper()} Results: {passed} passed, {failed} failed, {errors} errors")
            
            # Show failed tests
            failed_tests = [r for r in results if r.status in ["fail", "error"]]
            if failed_tests:
                print("\nFailed tests:")
                for result in failed_tests:
                    print(f"  - {result.name}: {result.error_message}")
        
        self.print_summary()

    def print_summary(self) -> None:
        """Print comprehensive test summary"""
        total_duration = time.time() - self.start_time
        
        # Calculate statistics
        total_tests = len(self.test_results)
        passed = sum(1 for r in self.test_results if r.status == "pass")
        failed = sum(1 for r in self.test_results if r.status == "fail")
        errors = sum(1 for r in self.test_results if r.status == "error")
        
        # Performance statistics
        if self.test_results:
            avg_duration = sum(r.duration for r in self.test_results) / len(self.test_results)
            slowest_test = max(self.test_results, key=lambda x: x.duration)
            fastest_test = min(self.test_results, key=lambda x: x.duration)
        else:
            avg_duration = 0
            slowest_test = None
            fastest_test = None
        
        print("\n" + "="*60)
        print("COMPREHENSIVE TEST SUMMARY")
        print("="*60)
        print(f"Total Tests: {total_tests}")
        print(f"Passed: {passed} ({passed/total_tests*100:.1f}%)" if total_tests > 0 else "Passed: 0")
        print(f"Failed: {failed} ({failed/total_tests*100:.1f}%)" if total_tests > 0 else "Failed: 0")
        print(f"Errors: {errors} ({errors/total_tests*100:.1f}%)" if total_tests > 0 else "Errors: 0")
        print(f"Total Duration: {total_duration:.2f}s")
        print(f"Average Test Duration: {avg_duration:.2f}s")
        
        if slowest_test:
            print(f"Slowest Test: {slowest_test.name} ({slowest_test.duration:.2f}s)")
        if fastest_test:
            print(f"Fastest Test: {fastest_test.name} ({fastest_test.duration:.2f}s)")
        
        # Category breakdown
        print("\nResults by Category:")
        for category in self.test_suites.keys():
            category_results = [r for r in self.test_results if r.category == category]
            if category_results:
                cat_passed = sum(1 for r in category_results if r.status == "pass")
                cat_total = len(category_results)
                print(f"  {category}: {cat_passed}/{cat_total} passed")
        
        # Overall status
        success_rate = passed / total_tests * 100 if total_tests > 0 else 0
        print(f"\nOverall Success Rate: {success_rate:.1f}%")
        
        if failed + errors == 0:
            print("🎉 ALL TESTS PASSED! 🎉")
            exit_code = 0
        else:
            print("❌ Some tests failed or had errors")
            exit_code = 1
        
        print("="*60)
        return exit_code

    def generate_json_report(self, output_file: str) -> None:
        """Generate JSON test report"""
        report = {
            "system_info": self.system_info,
            "summary": {
                "total_tests": len(self.test_results),
                "passed": sum(1 for r in self.test_results if r.status == "pass"),
                "failed": sum(1 for r in self.test_results if r.status == "fail"),
                "errors": sum(1 for r in self.test_results if r.status == "error"),
                "total_duration": time.time() - self.start_time,
            },
            "tests": [
                {
                    "name": r.name,
                    "category": r.category,
                    "status": r.status,
                    "duration": r.duration,
                    "error_message": r.error_message,
                    "memory_usage": r.memory_usage
                }
                for r in self.test_results
            ]
        }
        
        with open(output_file, 'w') as f:
            json.dump(report, f, indent=2)
        
        print(f"JSON report generated: {output_file}")

    def generate_junit_xml(self, output_file: str) -> None:
        """Generate JUnit XML report for CI/CD integration"""
        testsuites = ET.Element("testsuites")
        
        for category in self.test_suites.keys():
            category_results = [r for r in self.test_results if r.category == category]
            if not category_results:
                continue
            
            testsuite = ET.SubElement(testsuites, "testsuite")
            testsuite.set("name", category)
            testsuite.set("tests", str(len(category_results)))
            testsuite.set("failures", str(sum(1 for r in category_results if r.status == "fail")))
            testsuite.set("errors", str(sum(1 for r in category_results if r.status == "error")))
            testsuite.set("time", str(sum(r.duration for r in category_results)))
            
            for result in category_results:
                testcase = ET.SubElement(testsuite, "testcase")
                testcase.set("name", result.name)
                testcase.set("classname", result.category)
                testcase.set("time", str(result.duration))
                
                if result.status == "fail":
                    failure = ET.SubElement(testcase, "failure")
                    failure.set("message", result.error_message or "Test failed")
                    failure.text = result.output
                elif result.status == "error":
                    error = ET.SubElement(testcase, "error")
                    error.set("message", result.error_message or "Test error")
                    error.text = result.output
        
        tree = ET.ElementTree(testsuites)
        tree.write(output_file, encoding="utf-8", xml_declaration=True)
        print(f"JUnit XML report generated: {output_file}")

    def generate_html_report(self, output_file: str) -> None:
        """Generate HTML test report"""
        html = f"""
<!DOCTYPE html>
<html>
<head>
    <title>CURSED Advanced Test Suite Results</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        .header {{ background: #2c3e50; color: white; padding: 20px; border-radius: 5px; }}
        .summary {{ background: #ecf0f1; padding: 15px; margin: 20px 0; border-radius: 5px; }}
        .category {{ margin: 20px 0; }}
        .test-pass {{ color: #27ae60; }}
        .test-fail {{ color: #e74c3c; }}
        .test-error {{ color: #f39c12; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #f2f2f2; }}
        .duration {{ text-align: right; }}
        .status-pass {{ background-color: #d5f4e6; }}
        .status-fail {{ background-color: #ffeaa7; }}
        .status-error {{ background-color: #fab1a0; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>CURSED Advanced Test Suite Results</h1>
        <p>Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}</p>
        <p>System: {self.system_info['platform']} {self.system_info['architecture']}</p>
    </div>
    
    <div class="summary">
        <h2>Summary</h2>
        <p><strong>Total Tests:</strong> {len(self.test_results)}</p>
        <p><strong>Passed:</strong> <span class="test-pass">{sum(1 for r in self.test_results if r.status == "pass")}</span></p>
        <p><strong>Failed:</strong> <span class="test-fail">{sum(1 for r in self.test_results if r.status == "fail")}</span></p>
        <p><strong>Errors:</strong> <span class="test-error">{sum(1 for r in self.test_results if r.status == "error")}</span></p>
        <p><strong>Total Duration:</strong> {time.time() - self.start_time:.2f}s</p>
    </div>
    
    <h2>Test Results</h2>
    <table>
        <tr>
            <th>Category</th>
            <th>Test Name</th>
            <th>Status</th>
            <th>Duration (s)</th>
            <th>Error Message</th>
        </tr>
"""
        
        for result in self.test_results:
            status_class = f"status-{result.status}"
            html += f"""
        <tr class="{status_class}">
            <td>{result.category}</td>
            <td>{result.name}</td>
            <td>{result.status.upper()}</td>
            <td class="duration">{result.duration:.3f}</td>
            <td>{result.error_message or ''}</td>
        </tr>"""
        
        html += """
    </table>
</body>
</html>"""
        
        with open(output_file, 'w') as f:
            f.write(html)
        
        print(f"HTML report generated: {output_file}")

def main():
    parser = argparse.ArgumentParser(description="Run CURSED advanced test suites")
    parser.add_argument("--executable", "-e", default="./zig-out/bin/cursed-zig",
                        help="Path to CURSED executable")
    parser.add_argument("--categories", "-c", nargs="*", 
                        choices=["edge_cases", "performance", "integration", "stress", "security", "cross_platform"],
                        help="Test categories to run (default: all)")
    parser.add_argument("--parallel", "-p", action="store_true",
                        help="Run tests in parallel where possible")
    parser.add_argument("--json-report", help="Generate JSON report file")
    parser.add_argument("--junit-xml", help="Generate JUnit XML report file")
    parser.add_argument("--html-report", help="Generate HTML report file")
    parser.add_argument("--output-dir", "-o", default="test_reports",
                        help="Output directory for reports")
    
    args = parser.parse_args()
    
    # Create output directory
    os.makedirs(args.output_dir, exist_ok=True)
    
    # Initialize test runner
    runner = TestRunner(args.executable)
    
    # Run tests
    exit_code = runner.run_all_tests(args.categories, args.parallel)
    
    # Generate reports
    timestamp = datetime.now().strftime('%Y%m%d_%H%M%S')
    
    if args.json_report or not any([args.json_report, args.junit_xml, args.html_report]):
        json_file = args.json_report or f"{args.output_dir}/test_results_{timestamp}.json"
        runner.generate_json_report(json_file)
    
    if args.junit_xml:
        runner.generate_junit_xml(args.junit_xml)
    
    if args.html_report:
        runner.generate_html_report(args.html_report)
    
    # Generate default reports
    runner.generate_junit_xml(f"{args.output_dir}/junit_results_{timestamp}.xml")
    runner.generate_html_report(f"{args.output_dir}/test_results_{timestamp}.html")
    
    sys.exit(exit_code)

if __name__ == "__main__":
    main()
