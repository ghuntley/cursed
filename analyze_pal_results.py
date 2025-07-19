#!/usr/bin/env python3
"""
CURSED PAL Test Results Analyzer
Analyzes test results and validates against performance targets
"""

import re
import sys
import json
from pathlib import Path
from datetime import datetime

# Performance baseline targets from TEST_PAL.md
PERFORMANCE_TARGETS = {
    'arm64_macos': {
        'memory_ops_per_sec': 1_000_000,
        'goroutine_spawn_per_sec': 100_000,
        'notes': 'Apple Silicon optimized'
    },
    'arm64_linux': {
        'memory_ops_per_sec': 800_000,
        'goroutine_spawn_per_sec': 80_000,
        'notes': 'Standard ARM64'
    },
    'x86_64_macos': {
        'memory_ops_per_sec': 1_200_000,
        'goroutine_spawn_per_sec': 120_000,
        'notes': 'AVX optimizations'
    },
    'x86_64_linux': {
        'memory_ops_per_sec': 1_500_000,
        'goroutine_spawn_per_sec': 150_000,
        'notes': 'Huge pages + NUMA'
    },
    'x86_64_windows': {
        'memory_ops_per_sec': 1_000_000,
        'goroutine_spawn_per_sec': 100_000,
        'notes': 'Windows heap optimized'
    },
    'wasm32': {
        'memory_ops_per_sec': 100_000,
        'goroutine_spawn_per_sec': 10_000,
        'notes': 'Limited by WASM constraints'
    }
}

def detect_platform():
    """Detect current platform for target comparison"""
    import platform
    arch = platform.machine().lower()
    system = platform.system().lower()
    
    if arch in ['arm64', 'aarch64']:
        if system == 'darwin':
            return 'arm64_macos'
        elif system == 'linux':
            return 'arm64_linux'
    elif arch in ['x86_64', 'amd64']:
        if system == 'darwin':
            return 'x86_64_macos'
        elif system == 'linux':
            return 'x86_64_linux'
        elif system == 'windows':
            return 'x86_64_windows'
    elif arch.startswith('wasm'):
        return 'wasm32'
    
    return 'unknown'

def parse_test_output(log_file):
    """Parse test output and extract performance metrics"""
    results = {
        'platform': detect_platform(),
        'timestamp': datetime.now().isoformat(),
        'tests_passed': 0,
        'tests_failed': 0,
        'performance_metrics': {},
        'errors': []
    }
    
    try:
        with open(log_file, 'r') as f:
            content = f.read()
            
        # Count test results
        passed_tests = re.findall(r'✅.*test.*passed', content, re.IGNORECASE)
        failed_tests = re.findall(r'❌.*test.*failed', content, re.IGNORECASE)
        
        results['tests_passed'] = len(passed_tests)
        results['tests_failed'] = len(failed_tests)
        
        # Extract performance metrics
        memory_matches = re.findall(r'Memory operations:\s*(\d+)\s*ns', content)
        if memory_matches:
            memory_time_ns = int(memory_matches[0])
            # Calculate ops per second (assuming 1M iterations from benchmark)
            results['performance_metrics']['memory_ops_per_sec'] = int(1_000_000 / (memory_time_ns / 1_000_000_000))
        
        goroutine_matches = re.findall(r'Goroutine operations:\s*(\d+)\s*ns', content)
        if goroutine_matches:
            goroutine_time_ns = int(goroutine_matches[0])
            # Calculate spawns per second (assuming 1000 goroutines from benchmark)
            results['performance_metrics']['goroutine_spawn_per_sec'] = int(1000 / (goroutine_time_ns / 1_000_000_000))
        
        # Extract goroutine spawn time
        spawn_matches = re.findall(r'Spawned \d+ goroutines in (\d+) nanoseconds', content)
        if spawn_matches:
            spawn_time_ns = int(spawn_matches[0])
            results['performance_metrics']['goroutine_spawn_time_ns'] = spawn_time_ns
        
        # Extract errors
        error_matches = re.findall(r'ERROR:.*', content)
        results['errors'] = error_matches
        
    except Exception as e:
        results['errors'].append(f"Failed to parse log file: {e}")
    
    return results

def validate_performance(results):
    """Validate performance against targets"""
    platform = results['platform']
    if platform not in PERFORMANCE_TARGETS:
        return {
            'status': 'unknown_platform',
            'message': f"No performance targets defined for platform: {platform}"
        }
    
    targets = PERFORMANCE_TARGETS[platform]
    metrics = results['performance_metrics']
    validation = {
        'status': 'pass',
        'details': {},
        'overall_score': 0
    }
    
    total_checks = 0
    passed_checks = 0
    
    # Check memory operations per second
    if 'memory_ops_per_sec' in metrics:
        target = targets['memory_ops_per_sec']
        actual = metrics['memory_ops_per_sec']
        passed = actual >= target
        
        validation['details']['memory_performance'] = {
            'target': target,
            'actual': actual,
            'passed': passed,
            'percentage': (actual / target) * 100
        }
        
        total_checks += 1
        if passed:
            passed_checks += 1
    
    # Check goroutine spawn performance
    if 'goroutine_spawn_per_sec' in metrics:
        target = targets['goroutine_spawn_per_sec']
        actual = metrics['goroutine_spawn_per_sec']
        passed = actual >= target
        
        validation['details']['goroutine_performance'] = {
            'target': target,
            'actual': actual,
            'passed': passed,
            'percentage': (actual / target) * 100
        }
        
        total_checks += 1
        if passed:
            passed_checks += 1
    
    # Calculate overall score
    if total_checks > 0:
        validation['overall_score'] = (passed_checks / total_checks) * 100
        if validation['overall_score'] < 100:
            validation['status'] = 'partial_pass'
        if validation['overall_score'] < 50:
            validation['status'] = 'fail'
    
    return validation

def generate_report(results, validation):
    """Generate comprehensive test report"""
    report = []
    report.append("=" * 60)
    report.append("CURSED PAL Test Results Report")
    report.append("=" * 60)
    report.append(f"Platform: {results['platform']}")
    report.append(f"Timestamp: {results['timestamp']}")
    report.append(f"Tests Passed: {results['tests_passed']}")
    report.append(f"Tests Failed: {results['tests_failed']}")
    report.append("")
    
    # Performance section
    report.append("Performance Results:")
    report.append("-" * 20)
    
    if validation['status'] == 'unknown_platform':
        report.append(validation['message'])
    else:
        report.append(f"Overall Score: {validation['overall_score']:.1f}%")
        report.append(f"Status: {validation['status'].upper()}")
        report.append("")
        
        for metric, details in validation['details'].items():
            report.append(f"{metric.replace('_', ' ').title()}:")
            report.append(f"  Target: {details['target']:,}")
            report.append(f"  Actual: {details['actual']:,}")
            report.append(f"  Achievement: {details['percentage']:.1f}%")
            report.append(f"  Status: {'✅ PASS' if details['passed'] else '❌ FAIL'}")
            report.append("")
    
    # Raw metrics
    if results['performance_metrics']:
        report.append("Raw Performance Metrics:")
        report.append("-" * 25)
        for metric, value in results['performance_metrics'].items():
            report.append(f"  {metric}: {value:,}")
        report.append("")
    
    # Errors section
    if results['errors']:
        report.append("Errors Detected:")
        report.append("-" * 16)
        for error in results['errors']:
            report.append(f"  • {error}")
        report.append("")
    
    # Recommendations
    report.append("Recommendations:")
    report.append("-" * 15)
    if validation['status'] == 'fail':
        report.append("  • Performance is significantly below targets")
        report.append("  • Review PAL platform-specific optimizations")
        report.append("  • Check for memory leaks or inefficient algorithms")
    elif validation['status'] == 'partial_pass':
        report.append("  • Some performance targets not met")
        report.append("  • Consider platform-specific tuning")
        report.append("  • Profile hotspots for optimization opportunities")
    else:
        report.append("  • All performance targets met ✅")
        report.append("  • PAL system performing as expected")
    
    return "\n".join(report)

def main():
    if len(sys.argv) != 2:
        print("Usage: python analyze_pal_results.py <test_output_log>")
        print("Example: python analyze_pal_results.py pal_test_results.log")
        sys.exit(1)
    
    log_file = sys.argv[1]
    if not Path(log_file).exists():
        print(f"Error: Log file '{log_file}' not found")
        sys.exit(1)
    
    print("🔍 Analyzing PAL test results...")
    
    # Parse results
    results = parse_test_output(log_file)
    
    # Validate performance
    validation = validate_performance(results)
    
    # Generate report
    report = generate_report(results, validation)
    
    # Save results
    results_file = f"pal_analysis_{results['platform']}_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
    with open(results_file, 'w') as f:
        json.dump({
            'results': results,
            'validation': validation
        }, f, indent=2)
    
    # Display report
    print(report)
    print(f"\n📊 Detailed results saved to: {results_file}")
    
    # Exit code based on validation status
    if validation['status'] == 'fail':
        sys.exit(1)
    elif validation['status'] == 'partial_pass':
        sys.exit(2)
    else:
        sys.exit(0)

if __name__ == "__main__":
    main()
