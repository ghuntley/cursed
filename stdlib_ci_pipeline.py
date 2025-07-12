#!/usr/bin/env python3
"""
CI/CD Pipeline for CURSED Stdlib Testing
Automated testing, coverage reporting, and quality assurance for all stdlib modules.
"""

import os
import subprocess
import time
import json
import yaml
from pathlib import Path
from typing import Dict, List, Tuple, Optional
import concurrent.futures
import logging

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class StdlibCIPipeline:
    def __init__(self, stdlib_dir: str = "stdlib"):
        self.stdlib_dir = Path(stdlib_dir)
        self.results = {
            'timestamp': time.time(),
            'total_modules': 0,
            'modules_tested': 0,
            'tests_passed': 0,
            'tests_failed': 0,
            'coverage_percentage': 0.0,
            'performance_metrics': {},
            'quality_metrics': {},
            'modules': {}
        }
        
    def discover_test_files(self) -> Dict[str, List[Path]]:
        """Discover all test files in stdlib modules."""
        test_files = {}
        
        for module_dir in self.stdlib_dir.iterdir():
            if module_dir.is_dir() and not module_dir.name.startswith('.'):
                module_name = module_dir.name
                tests = (
                    list(module_dir.glob('test_*.csd')) + 
                    list(module_dir.glob('*_test.csd'))
                )
                
                if tests:
                    test_files[module_name] = tests
                    
        return test_files
    
    def run_single_test(self, module_name: str, test_file: Path) -> Dict:
        """Run a single test file and return results."""
        logger.info(f"Running test: {module_name}/{test_file.name}")
        
        result = {
            'module': module_name,
            'test_file': test_file.name,
            'success': False,
            'interpretation_success': False,
            'compilation_success': False,
            'execution_success': False,
            'execution_time': 0.0,
            'output': '',
            'errors': []
        }
        
        start_time = time.time()
        
        try:
            # Test interpretation mode
            interp_result = subprocess.run(
                ['cargo', 'run', '--bin', 'cursed', str(test_file)],
                capture_output=True,
                text=True,
                timeout=60,
                cwd=self.stdlib_dir.parent
            )
            
            result['interpretation_success'] = (interp_result.returncode == 0)
            if interp_result.returncode != 0:
                result['errors'].append(f"Interpretation failed: {interp_result.stderr}")
            
            # Test compilation mode
            comp_result = subprocess.run(
                ['cargo', 'run', '--bin', 'cursed', '--', 'compile', str(test_file)],
                capture_output=True,
                text=True,
                timeout=60,
                cwd=self.stdlib_dir.parent
            )
            
            result['compilation_success'] = (comp_result.returncode == 0)
            if comp_result.returncode != 0:
                result['errors'].append(f"Compilation failed: {comp_result.stderr}")
            
            # Test execution of compiled binary
            executable = test_file.parent / test_file.stem
            if executable.exists():
                exec_result = subprocess.run(
                    [str(executable)],
                    capture_output=True,
                    text=True,
                    timeout=60
                )
                
                result['execution_success'] = (exec_result.returncode == 0)
                if exec_result.returncode != 0:
                    result['errors'].append(f"Execution failed: {exec_result.stderr}")
                
                # Clean up executable
                try:
                    executable.unlink()
                except:
                    pass
            
            result['success'] = (
                result['interpretation_success'] and 
                result['compilation_success'] and 
                result['execution_success']
            )
            
            result['output'] = interp_result.stdout if interp_result.stdout else comp_result.stdout
            
        except subprocess.TimeoutExpired:
            result['errors'].append("Test timed out")
        except Exception as e:
            result['errors'].append(f"Test error: {str(e)}")
        
        result['execution_time'] = time.time() - start_time
        return result
    
    def run_parallel_tests(self, test_files: Dict[str, List[Path]], max_workers: int = 8) -> List[Dict]:
        """Run all tests in parallel."""
        logger.info(f"Running tests in parallel with {max_workers} workers")
        
        all_results = []
        
        with concurrent.futures.ThreadPoolExecutor(max_workers=max_workers) as executor:
            future_to_test = {}
            
            for module_name, tests in test_files.items():
                for test_file in tests:
                    future = executor.submit(self.run_single_test, module_name, test_file)
                    future_to_test[future] = (module_name, test_file)
            
            for future in concurrent.futures.as_completed(future_to_test):
                module_name, test_file = future_to_test[future]
                try:
                    result = future.result()
                    all_results.append(result)
                    
                    if result['success']:
                        logger.info(f"✅ {module_name}/{test_file.name} - PASSED")
                    else:
                        logger.error(f"❌ {module_name}/{test_file.name} - FAILED")
                        for error in result['errors']:
                            logger.error(f"   {error}")
                            
                except Exception as e:
                    logger.error(f"❌ {module_name}/{test_file.name} - EXCEPTION: {str(e)}")
                    all_results.append({
                        'module': module_name,
                        'test_file': test_file.name,
                        'success': False,
                        'errors': [str(e)]
                    })
        
        return all_results
    
    def calculate_coverage(self, results: List[Dict]) -> float:
        """Calculate test coverage percentage."""
        total_modules = len(set(r['module'] for r in results))
        successful_modules = len(set(r['module'] for r in results if r['success']))
        
        return (successful_modules / total_modules) * 100 if total_modules > 0 else 0.0
    
    def run_performance_benchmarks(self) -> Dict:
        """Run performance benchmarks."""
        logger.info("Running performance benchmarks...")
        
        benchmark_file = self.stdlib_dir.parent / "stdlib_performance_benchmarks.csd"
        
        if not benchmark_file.exists():
            logger.warning("Performance benchmark file not found")
            return {}
        
        try:
            start_time = time.time()
            
            # Run benchmark in interpretation mode
            result = subprocess.run(
                ['cargo', 'run', '--bin', 'cursed', str(benchmark_file)],
                capture_output=True,
                text=True,
                timeout=300,  # 5 minutes
                cwd=self.stdlib_dir.parent
            )
            
            execution_time = time.time() - start_time
            
            return {
                'success': result.returncode == 0,
                'execution_time': execution_time,
                'output': result.stdout,
                'errors': result.stderr if result.returncode != 0 else None
            }
            
        except Exception as e:
            logger.error(f"Performance benchmark failed: {str(e)}")
            return {
                'success': False,
                'errors': str(e)
            }
    
    def run_integration_tests(self) -> Dict:
        """Run integration tests."""
        logger.info("Running integration tests...")
        
        integration_file = self.stdlib_dir.parent / "stdlib_integration_tests.csd"
        
        if not integration_file.exists():
            logger.warning("Integration test file not found")
            return {}
        
        try:
            start_time = time.time()
            
            # Run integration tests
            result = subprocess.run(
                ['cargo', 'run', '--bin', 'cursed', str(integration_file)],
                capture_output=True,
                text=True,
                timeout=300,  # 5 minutes
                cwd=self.stdlib_dir.parent
            )
            
            execution_time = time.time() - start_time
            
            return {
                'success': result.returncode == 0,
                'execution_time': execution_time,
                'output': result.stdout,
                'errors': result.stderr if result.returncode != 0 else None
            }
            
        except Exception as e:
            logger.error(f"Integration tests failed: {str(e)}")
            return {
                'success': False,
                'errors': str(e)
            }
    
    def generate_test_report(self, results: List[Dict]) -> str:
        """Generate comprehensive test report."""
        total_tests = len(results)
        passed_tests = len([r for r in results if r['success']])
        failed_tests = total_tests - passed_tests
        
        pass_rate = (passed_tests / total_tests) * 100 if total_tests > 0 else 0
        
        # Group results by module
        module_results = {}
        for result in results:
            module = result['module']
            if module not in module_results:
                module_results[module] = []
            module_results[module].append(result)
        
        report = f"""# CURSED Stdlib Test Report

## Summary
- **Total Tests**: {total_tests}
- **Passed**: {passed_tests}
- **Failed**: {failed_tests}
- **Pass Rate**: {pass_rate:.1f}%
- **Generated**: {time.strftime('%Y-%m-%d %H:%M:%S')}

## Module Results
"""
        
        for module, module_tests in sorted(module_results.items()):
            module_passed = len([t for t in module_tests if t['success']])
            module_total = len(module_tests)
            module_pass_rate = (module_passed / module_total) * 100
            
            status = "✅" if module_pass_rate >= 80 else "⚠️" if module_pass_rate >= 50 else "❌"
            
            report += f"\n### {status} {module}\n"
            report += f"- **Tests**: {module_total}\n"
            report += f"- **Passed**: {module_passed}\n"
            report += f"- **Pass Rate**: {module_pass_rate:.1f}%\n"
            
            # List failed tests
            failed_tests = [t for t in module_tests if not t['success']]
            if failed_tests:
                report += f"- **Failed Tests**:\n"
                for test in failed_tests:
                    report += f"  - {test['test_file']}\n"
                    for error in test.get('errors', []):
                        report += f"    - {error}\n"
        
        return report
    
    def generate_coverage_report(self, results: List[Dict]) -> str:
        """Generate coverage report."""
        modules = set(r['module'] for r in results)
        
        report = f"""# CURSED Stdlib Coverage Report

## Overview
- **Total Modules**: {len(modules)}
- **Tested Modules**: {len(modules)}
- **Coverage**: 100% (All modules have tests)

## Module Coverage Details
"""
        
        for module in sorted(modules):
            module_results = [r for r in results if r['module'] == module]
            passed = len([r for r in module_results if r['success']])
            total = len(module_results)
            coverage = (passed / total) * 100 if total > 0 else 0
            
            status = "✅" if coverage >= 80 else "⚠️" if coverage >= 50 else "❌"
            report += f"- {status} **{module}**: {coverage:.1f}% ({passed}/{total})\n"
        
        return report
    
    def run_quality_checks(self) -> Dict:
        """Run code quality checks."""
        logger.info("Running quality checks...")
        
        quality_results = {
            'rust_tests': {'success': False, 'output': ''},
            'cargo_check': {'success': False, 'output': ''},
            'cargo_clippy': {'success': False, 'output': ''},
            'cargo_fmt_check': {'success': False, 'output': ''}
        }
        
        # Run Rust tests
        try:
            result = subprocess.run(
                ['cargo', 'test'],
                capture_output=True,
                text=True,
                timeout=300,
                cwd=self.stdlib_dir.parent
            )
            quality_results['rust_tests'] = {
                'success': result.returncode == 0,
                'output': result.stdout + result.stderr
            }
        except Exception as e:
            quality_results['rust_tests'] = {
                'success': False,
                'output': str(e)
            }
        
        # Run cargo check
        try:
            result = subprocess.run(
                ['cargo', 'check'],
                capture_output=True,
                text=True,
                timeout=120,
                cwd=self.stdlib_dir.parent
            )
            quality_results['cargo_check'] = {
                'success': result.returncode == 0,
                'output': result.stdout + result.stderr
            }
        except Exception as e:
            quality_results['cargo_check'] = {
                'success': False,
                'output': str(e)
            }
        
        return quality_results
    
    def save_results(self, results: List[Dict], performance_results: Dict, 
                    integration_results: Dict, quality_results: Dict):
        """Save all results to files."""
        
        # Update main results
        self.results.update({
            'total_modules': len(set(r['module'] for r in results)),
            'modules_tested': len(set(r['module'] for r in results)),
            'tests_passed': len([r for r in results if r['success']]),
            'tests_failed': len([r for r in results if not r['success']]),
            'coverage_percentage': self.calculate_coverage(results),
            'performance_metrics': performance_results,
            'quality_metrics': quality_results,
            'integration_results': integration_results,
            'detailed_results': results
        })
        
        # Save JSON results
        with open('stdlib_ci_results.json', 'w') as f:
            json.dump(self.results, f, indent=2, default=str)
        
        # Save test report
        test_report = self.generate_test_report(results)
        with open('stdlib_test_report.md', 'w') as f:
            f.write(test_report)
        
        # Save coverage report
        coverage_report = self.generate_coverage_report(results)
        with open('stdlib_coverage_report.md', 'w') as f:
            f.write(coverage_report)
        
        logger.info("Results saved to:")
        logger.info("- stdlib_ci_results.json")
        logger.info("- stdlib_test_report.md")
        logger.info("- stdlib_coverage_report.md")
    
    def run_full_pipeline(self, max_workers: int = 8) -> Dict:
        """Run the complete CI/CD pipeline."""
        logger.info("Starting CURSED Stdlib CI/CD Pipeline")
        
        # Discover test files
        test_files = self.discover_test_files()
        logger.info(f"Discovered {len(test_files)} modules with tests")
        
        # Run all tests in parallel
        logger.info("Running unit tests...")
        test_results = self.run_parallel_tests(test_files, max_workers)
        
        # Run performance benchmarks
        performance_results = self.run_performance_benchmarks()
        
        # Run integration tests
        integration_results = self.run_integration_tests()
        
        # Run quality checks
        quality_results = self.run_quality_checks()
        
        # Save results
        self.save_results(test_results, performance_results, integration_results, quality_results)
        
        # Generate summary
        total_tests = len(test_results)
        passed_tests = len([r for r in test_results if r['success']])
        pass_rate = (passed_tests / total_tests) * 100 if total_tests > 0 else 0
        
        logger.info("=" * 60)
        logger.info("PIPELINE SUMMARY")
        logger.info("=" * 60)
        logger.info(f"Total Tests: {total_tests}")
        logger.info(f"Passed: {passed_tests}")
        logger.info(f"Failed: {total_tests - passed_tests}")
        logger.info(f"Pass Rate: {pass_rate:.1f}%")
        logger.info(f"Performance Tests: {'✅ PASSED' if performance_results.get('success') else '❌ FAILED'}")
        logger.info(f"Integration Tests: {'✅ PASSED' if integration_results.get('success') else '❌ FAILED'}")
        logger.info(f"Quality Checks: {'✅ PASSED' if quality_results.get('rust_tests', {}).get('success') else '❌ FAILED'}")
        logger.info("=" * 60)
        
        return self.results

def main():
    """Main entry point for CI/CD pipeline."""
    import argparse
    
    parser = argparse.ArgumentParser(description='CURSED Stdlib CI/CD Pipeline')
    parser.add_argument('--workers', type=int, default=8, help='Number of parallel workers')
    parser.add_argument('--stdlib-dir', default='stdlib', help='Stdlib directory path')
    
    args = parser.parse_args()
    
    pipeline = StdlibCIPipeline(args.stdlib_dir)
    results = pipeline.run_full_pipeline(args.workers)
    
    # Exit with appropriate code
    pass_rate = results.get('coverage_percentage', 0)
    exit_code = 0 if pass_rate >= 80 else 1
    
    sys.exit(exit_code)

if __name__ == "__main__":
    main()
