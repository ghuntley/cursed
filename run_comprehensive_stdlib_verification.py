#!/usr/bin/env python3
"""
Comprehensive Stdlib Verification Runner
Orchestrates all stdlib testing and verification processes.
"""

import os
import sys
import time
import json
import subprocess
from pathlib import Path
from typing import Dict, List
import logging

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class ComprehensiveStdlibVerifier:
    def __init__(self):
        self.results = {
            'timestamp': time.time(),
            'phases': {
                'audit': {'status': 'pending', 'results': {}},
                'dependency_analysis': {'status': 'pending', 'results': {}},
                'unit_tests': {'status': 'pending', 'results': {}},
                'integration_tests': {'status': 'pending', 'results': {}},
                'performance_tests': {'status': 'pending', 'results': {}},
                'ci_pipeline': {'status': 'pending', 'results': {}}
            },
            'summary': {
                'total_modules': 0,
                'modules_with_tests': 0,
                'test_coverage': 0.0,
                'performance_acceptable': False,
                'ci_passing': False,
                'ready_for_production': False
            }
        }
    
    def run_phase(self, phase_name: str, command: List[str], timeout: int = 300) -> Dict:
        """Run a verification phase and return results."""
        logger.info(f"Running phase: {phase_name}")
        
        try:
            start_time = time.time()
            
            result = subprocess.run(
                command,
                capture_output=True,
                text=True,
                timeout=timeout,
                cwd=Path.cwd()
            )
            
            execution_time = time.time() - start_time
            
            return {
                'status': 'success' if result.returncode == 0 else 'failed',
                'execution_time': execution_time,
                'return_code': result.returncode,
                'stdout': result.stdout,
                'stderr': result.stderr
            }
            
        except subprocess.TimeoutExpired:
            return {
                'status': 'timeout',
                'execution_time': timeout,
                'error': f"Phase {phase_name} timed out after {timeout} seconds"
            }
        except Exception as e:
            return {
                'status': 'error',
                'error': str(e)
            }
    
    def phase_1_audit(self) -> Dict:
        """Phase 1: Audit all stdlib modules."""
        logger.info("Phase 1: Auditing stdlib modules...")
        
        # Run stdlib audit
        result = self.run_phase('audit', ['python3', 'stdlib_test_audit.py'])
        
        # Load audit results
        try:
            with open('stdlib_audit_report.json', 'r') as f:
                audit_data = json.load(f)
                
            result['audit_summary'] = {
                'total_modules': audit_data.get('total_modules', 0),
                'modules_with_tests': audit_data.get('modules_with_tests', 0),
                'modules_without_tests': audit_data.get('modules_without_tests', 0),
                'missing_tests': audit_data.get('missing_tests', [])
            }
            
        except Exception as e:
            logger.error(f"Failed to load audit results: {e}")
            result['audit_summary'] = {}
        
        return result
    
    def phase_2_dependency_analysis(self) -> Dict:
        """Phase 2: Analyze module dependencies."""
        logger.info("Phase 2: Analyzing module dependencies...")
        
        result = self.run_phase('dependency_analysis', ['python3', 'stdlib_dependency_graph.py'])
        
        # Load dependency analysis results
        try:
            with open('stdlib_dependency_analysis.json', 'r') as f:
                dep_data = json.load(f)
                
            result['dependency_summary'] = {
                'total_modules': len(dep_data.get('modules', {})),
                'critical_modules': len(dep_data.get('critical_modules', [])),
                'circular_dependencies': len(dep_data.get('circular_dependencies', [])),
                'leaf_modules': len(dep_data.get('leaf_modules', [])),
                'test_order': dep_data.get('test_order', [])
            }
            
        except Exception as e:
            logger.error(f"Failed to load dependency analysis: {e}")
            result['dependency_summary'] = {}
        
        return result
    
    def phase_3_unit_tests(self) -> Dict:
        """Phase 3: Run unit tests for all modules."""
        logger.info("Phase 3: Running unit tests...")
        
        # First run the test runner
        result = self.run_phase('unit_tests', ['python3', 'stdlib_test_runner.py'], timeout=600)
        
        # Load test results
        try:
            with open('stdlib_test_results.json', 'r') as f:
                test_data = json.load(f)
                
            result['test_summary'] = {
                'total_tests': test_data.get('total_tests', 0),
                'passed_tests': test_data.get('passed_tests', 0),
                'pass_rate': test_data.get('pass_rate', 0.0)
            }
            
        except Exception as e:
            logger.error(f"Failed to load unit test results: {e}")
            result['test_summary'] = {}
        
        return result
    
    def phase_4_integration_tests(self) -> Dict:
        """Phase 4: Run integration tests."""
        logger.info("Phase 4: Running integration tests...")
        
        # Test integration test file
        result = self.run_phase('integration_tests', 
                               ['cargo', 'run', '--bin', 'cursed', 'stdlib_integration_tests.csd'],
                               timeout=300)
        
        # Also test compilation
        if result['status'] == 'success':
            compile_result = self.run_phase('integration_compile',
                                           ['cargo', 'run', '--bin', 'cursed', '--', 'compile', 'stdlib_integration_tests.csd'],
                                           timeout=300)
            result['compile_test'] = compile_result
        
        return result
    
    def phase_5_performance_tests(self) -> Dict:
        """Phase 5: Run performance benchmarks."""
        logger.info("Phase 5: Running performance benchmarks...")
        
        # Run performance benchmarks
        result = self.run_phase('performance_tests',
                               ['cargo', 'run', '--bin', 'cursed', 'stdlib_performance_benchmarks.csd'],
                               timeout=600)
        
        # Parse performance results from output
        if result['status'] == 'success':
            output = result.get('stdout', '')
            performance_metrics = self.parse_performance_output(output)
            result['performance_metrics'] = performance_metrics
        
        return result
    
    def parse_performance_output(self, output: str) -> Dict:
        """Parse performance metrics from test output."""
        metrics = {}
        
        # Look for performance indicators in output
        lines = output.split('\n')
        for line in lines:
            if 'ops/sec' in line:
                try:
                    # Extract module and performance
                    parts = line.split(':')
                    if len(parts) >= 2:
                        module = parts[0].strip()
                        perf_str = parts[1].strip()
                        # Extract numeric value
                        import re
                        match = re.search(r'([\d.]+)', perf_str)
                        if match:
                            metrics[module] = float(match.group(1))
                except:
                    pass
        
        return metrics
    
    def phase_6_ci_pipeline(self) -> Dict:
        """Phase 6: Run CI/CD pipeline."""
        logger.info("Phase 6: Running CI/CD pipeline...")
        
        # Run CI pipeline
        result = self.run_phase('ci_pipeline', ['python3', 'stdlib_ci_pipeline.py'], timeout=900)
        
        # Load CI results
        try:
            with open('stdlib_ci_results.json', 'r') as f:
                ci_data = json.load(f)
                
            result['ci_summary'] = {
                'total_tests': ci_data.get('tests_passed', 0) + ci_data.get('tests_failed', 0),
                'passed_tests': ci_data.get('tests_passed', 0),
                'failed_tests': ci_data.get('tests_failed', 0),
                'pass_rate': ci_data.get('coverage_percentage', 0.0)
            }
            
        except Exception as e:
            logger.error(f"Failed to load CI results: {e}")
            result['ci_summary'] = {}
        
        return result
    
    def generate_comprehensive_report(self) -> str:
        """Generate comprehensive verification report."""
        report = f"""# CURSED Stdlib Comprehensive Verification Report

## Executive Summary
- **Verification Date**: {time.strftime('%Y-%m-%d %H:%M:%S')}
- **Total Modules**: {self.results['summary']['total_modules']}
- **Test Coverage**: {self.results['summary']['test_coverage']:.1f}%
- **Production Ready**: {'✅ YES' if self.results['summary']['ready_for_production'] else '❌ NO'}

## Phase Results

### Phase 1: Module Audit
- **Status**: {self.results['phases']['audit']['status']}
- **Duration**: {self.results['phases']['audit']['results'].get('execution_time', 0):.1f}s
- **Modules with Tests**: {self.results['phases']['audit']['results'].get('audit_summary', {}).get('modules_with_tests', 0)}
- **Modules without Tests**: {self.results['phases']['audit']['results'].get('audit_summary', {}).get('modules_without_tests', 0)}

### Phase 2: Dependency Analysis
- **Status**: {self.results['phases']['dependency_analysis']['status']}
- **Duration**: {self.results['phases']['dependency_analysis']['results'].get('execution_time', 0):.1f}s
- **Critical Modules**: {self.results['phases']['dependency_analysis']['results'].get('dependency_summary', {}).get('critical_modules', 0)}
- **Circular Dependencies**: {self.results['phases']['dependency_analysis']['results'].get('dependency_summary', {}).get('circular_dependencies', 0)}

### Phase 3: Unit Tests
- **Status**: {self.results['phases']['unit_tests']['status']}
- **Duration**: {self.results['phases']['unit_tests']['results'].get('execution_time', 0):.1f}s
- **Tests Passed**: {self.results['phases']['unit_tests']['results'].get('test_summary', {}).get('passed_tests', 0)}
- **Pass Rate**: {self.results['phases']['unit_tests']['results'].get('test_summary', {}).get('pass_rate', 0.0):.1f}%

### Phase 4: Integration Tests
- **Status**: {self.results['phases']['integration_tests']['status']}
- **Duration**: {self.results['phases']['integration_tests']['results'].get('execution_time', 0):.1f}s
- **Compilation Test**: {self.results['phases']['integration_tests']['results'].get('compile_test', {}).get('status', 'N/A')}

### Phase 5: Performance Tests
- **Status**: {self.results['phases']['performance_tests']['status']}
- **Duration**: {self.results['phases']['performance_tests']['results'].get('execution_time', 0):.1f}s
- **Performance Acceptable**: {'✅ YES' if self.results['summary']['performance_acceptable'] else '❌ NO'}

### Phase 6: CI/CD Pipeline
- **Status**: {self.results['phases']['ci_pipeline']['status']}
- **Duration**: {self.results['phases']['ci_pipeline']['results'].get('execution_time', 0):.1f}s
- **CI Tests Passed**: {self.results['phases']['ci_pipeline']['results'].get('ci_summary', {}).get('passed_tests', 0)}
- **CI Pass Rate**: {self.results['phases']['ci_pipeline']['results'].get('ci_summary', {}).get('pass_rate', 0.0):.1f}%

## Recommendations

"""
        
        # Add recommendations based on results
        if self.results['summary']['test_coverage'] < 80:
            report += "- ❌ Improve test coverage (currently below 80%)\n"
        else:
            report += "- ✅ Test coverage is acceptable\n"
        
        if self.results['phases']['dependency_analysis']['results'].get('dependency_summary', {}).get('circular_dependencies', 0) > 0:
            report += "- ❌ Resolve circular dependencies\n"
        else:
            report += "- ✅ No circular dependencies found\n"
        
        if not self.results['summary']['performance_acceptable']:
            report += "- ❌ Performance benchmarks need improvement\n"
        else:
            report += "- ✅ Performance benchmarks are acceptable\n"
        
        if not self.results['summary']['ci_passing']:
            report += "- ❌ CI/CD pipeline needs fixes\n"
        else:
            report += "- ✅ CI/CD pipeline is passing\n"
        
        report += f"""
## Production Readiness Assessment

{'✅ READY FOR PRODUCTION' if self.results['summary']['ready_for_production'] else '❌ NOT READY FOR PRODUCTION'}

### Criteria Met:
- Test Coverage ≥ 80%: {'✅' if self.results['summary']['test_coverage'] >= 80 else '❌'}
- No Circular Dependencies: {'✅' if self.results['phases']['dependency_analysis']['results'].get('dependency_summary', {}).get('circular_dependencies', 0) == 0 else '❌'}
- Performance Acceptable: {'✅' if self.results['summary']['performance_acceptable'] else '❌'}
- CI/CD Passing: {'✅' if self.results['summary']['ci_passing'] else '❌'}
- Integration Tests Pass: {'✅' if self.results['phases']['integration_tests']['status'] == 'success' else '❌'}

## Files Generated
- stdlib_audit_report.json
- stdlib_dependency_report.md
- stdlib_test_results.json
- stdlib_coverage_report.md
- stdlib_ci_results.json
- comprehensive_verification_report.md
- comprehensive_verification_results.json
"""
        
        return report
    
    def run_comprehensive_verification(self) -> Dict:
        """Run complete verification process."""
        logger.info("Starting comprehensive stdlib verification...")
        
        # Phase 1: Audit
        self.results['phases']['audit']['results'] = self.phase_1_audit()
        self.results['phases']['audit']['status'] = self.results['phases']['audit']['results']['status']
        
        # Phase 2: Dependency Analysis
        self.results['phases']['dependency_analysis']['results'] = self.phase_2_dependency_analysis()
        self.results['phases']['dependency_analysis']['status'] = self.results['phases']['dependency_analysis']['results']['status']
        
        # Phase 3: Unit Tests
        self.results['phases']['unit_tests']['results'] = self.phase_3_unit_tests()
        self.results['phases']['unit_tests']['status'] = self.results['phases']['unit_tests']['results']['status']
        
        # Phase 4: Integration Tests
        self.results['phases']['integration_tests']['results'] = self.phase_4_integration_tests()
        self.results['phases']['integration_tests']['status'] = self.results['phases']['integration_tests']['results']['status']
        
        # Phase 5: Performance Tests
        self.results['phases']['performance_tests']['results'] = self.phase_5_performance_tests()
        self.results['phases']['performance_tests']['status'] = self.results['phases']['performance_tests']['results']['status']
        
        # Phase 6: CI Pipeline
        self.results['phases']['ci_pipeline']['results'] = self.phase_6_ci_pipeline()
        self.results['phases']['ci_pipeline']['status'] = self.results['phases']['ci_pipeline']['results']['status']
        
        # Calculate summary metrics
        self.calculate_summary_metrics()
        
        # Generate comprehensive report
        report = self.generate_comprehensive_report()
        
        # Save results
        with open('comprehensive_verification_results.json', 'w') as f:
            json.dump(self.results, f, indent=2, default=str)
        
        with open('comprehensive_verification_report.md', 'w') as f:
            f.write(report)
        
        logger.info("Comprehensive verification completed!")
        logger.info("Reports saved:")
        logger.info("- comprehensive_verification_results.json")
        logger.info("- comprehensive_verification_report.md")
        
        return self.results
    
    def calculate_summary_metrics(self):
        """Calculate summary metrics from all phases."""
        # Get metrics from audit
        audit_summary = self.results['phases']['audit']['results'].get('audit_summary', {})
        self.results['summary']['total_modules'] = audit_summary.get('total_modules', 0)
        self.results['summary']['modules_with_tests'] = audit_summary.get('modules_with_tests', 0)
        
        # Calculate test coverage
        if self.results['summary']['total_modules'] > 0:
            self.results['summary']['test_coverage'] = (
                self.results['summary']['modules_with_tests'] / 
                self.results['summary']['total_modules']
            ) * 100
        
        # Check performance
        perf_results = self.results['phases']['performance_tests']['results']
        self.results['summary']['performance_acceptable'] = (
            perf_results.get('status') == 'success' and
            len(perf_results.get('performance_metrics', {})) > 0
        )
        
        # Check CI
        ci_results = self.results['phases']['ci_pipeline']['results']
        ci_summary = ci_results.get('ci_summary', {})
        self.results['summary']['ci_passing'] = (
            ci_results.get('status') == 'success' and
            ci_summary.get('pass_rate', 0) >= 80
        )
        
        # Overall production readiness
        self.results['summary']['ready_for_production'] = (
            self.results['summary']['test_coverage'] >= 80 and
            self.results['summary']['performance_acceptable'] and
            self.results['summary']['ci_passing'] and
            self.results['phases']['integration_tests']['status'] == 'success' and
            self.results['phases']['dependency_analysis']['results'].get('dependency_summary', {}).get('circular_dependencies', 0) == 0
        )

def main():
    """Main entry point."""
    verifier = ComprehensiveStdlibVerifier()
    results = verifier.run_comprehensive_verification()
    
    # Print summary
    print("\n" + "="*60)
    print("COMPREHENSIVE STDLIB VERIFICATION SUMMARY")
    print("="*60)
    print(f"Total Modules: {results['summary']['total_modules']}")
    print(f"Test Coverage: {results['summary']['test_coverage']:.1f}%")
    print(f"Performance Acceptable: {'✅' if results['summary']['performance_acceptable'] else '❌'}")
    print(f"CI Passing: {'✅' if results['summary']['ci_passing'] else '❌'}")
    print(f"Production Ready: {'✅' if results['summary']['ready_for_production'] else '❌'}")
    print("="*60)
    
    # Exit with appropriate code
    exit_code = 0 if results['summary']['ready_for_production'] else 1
    sys.exit(exit_code)

if __name__ == "__main__":
    main()
