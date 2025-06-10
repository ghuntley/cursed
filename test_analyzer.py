#!/usr/bin/env python3
"""
Analyze CURSED test compilation issues and generate fixes
"""
import subprocess
import os
import glob
import re
from pathlib import Path

def run_test(test_name):
    """Run a single test and return results"""
    try:
        cmd = ["./fix_linking.sh", "cargo", "test", "--test", test_name]
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
        return {
            'name': test_name,
            'success': result.returncode == 0,
            'stdout': result.stdout,
            'stderr': result.stderr,
            'returncode': result.returncode
        }
    except subprocess.TimeoutExpired:
        return {
            'name': test_name,
            'success': False,
            'stdout': '',
            'stderr': 'Test timed out',
            'returncode': -1
        }
    except Exception as e:
        return {
            'name': test_name,
            'success': False,
            'stdout': '',
            'stderr': str(e),
            'returncode': -2
        }

def get_test_files():
    """Get all test files"""
    test_files = []
    for file_path in glob.glob("tests/*.rs"):
        if os.path.isfile(file_path):
            test_name = os.path.basename(file_path).replace('.rs', '')
            test_files.append(test_name)
    return sorted(test_files)

def analyze_errors(stderr):
    """Analyze error patterns in test output"""
    error_patterns = []
    
    # Common error patterns
    patterns = [
        (r'unresolved import `([^`]+)`', 'unresolved_import'),
        (r'no field `([^`]+)` on type', 'missing_field'),
        (r'no variant or associated item named `([^`]+)`', 'missing_variant'),
        (r'no method named `([^`]+)` found', 'missing_method'),
        (r'mismatched types', 'type_mismatch'),
        (r'expected tuple struct or tuple variant, found associated function', 'token_pattern_issue'),
        (r'prefix `([^`]+)` is unknown', 'unknown_prefix'),
        (r'unknown start of token: \\', 'escape_sequence_issue'),
    ]
    
    for pattern, error_type in patterns:
        matches = re.findall(pattern, stderr)
        if matches:
            for match in matches:
                error_patterns.append((error_type, match if isinstance(match, str) else match))
    
    return error_patterns

def main():
    """Main analysis function"""
    print("Analyzing CURSED test suite...")
    
    # Get working tests from AGENT.md
    known_working = [
        'very_simple_test',
        'simple_core_test', 
        'simple_lexer_test',
        'simple_llvm_test',
        'simple_jit_test',
        'minimal_interface_test'
    ]
    
    print("Testing known working tests...")
    working_tests = []
    failing_tests = []
    
    for test_name in known_working:
        print(f"Testing {test_name}...")
        result = run_test(test_name)
        if result['success']:
            working_tests.append(test_name)
            print(f"  ✅ {test_name} - PASS")
        else:
            failing_tests.append(test_name)
            print(f"  ❌ {test_name} - FAIL")
            # Print first few lines of error for debugging
            if result['stderr']:
                lines = result['stderr'].split('\n')[:5]
                for line in lines:
                    if line.strip():
                        print(f"     {line}")
    
    print(f"\nWorking tests: {len(working_tests)}")
    print(f"Failing known tests: {len(failing_tests)}")
    
    # Test a few more to get error patterns
    print("\nTesting additional tests for error patterns...")
    test_files = get_test_files()
    error_summary = {}
    
    # Test first 10 not in known working
    additional_tests = [t for t in test_files[:10] if t not in known_working]
    
    for test_name in additional_tests[:5]:  # Test just 5 more for now
        print(f"Testing {test_name}...")
        result = run_test(test_name)
        if result['success']:
            working_tests.append(test_name)
            print(f"  ✅ {test_name} - PASS")
        else:
            patterns = analyze_errors(result['stderr'])
            for error_type, detail in patterns:
                if error_type not in error_summary:
                    error_summary[error_type] = []
                error_summary[error_type].append((test_name, detail))
            print(f"  ❌ {test_name} - FAIL ({len(patterns)} error types)")
    
    print("\n" + "="*50)
    print("ERROR PATTERN SUMMARY")
    print("="*50)
    
    for error_type, occurrences in error_summary.items():
        print(f"\n{error_type.upper()}:")
        unique_details = list(set([detail for _, detail in occurrences]))
        for detail in unique_details[:5]:  # Show first 5 unique examples
            print(f"  - {detail}")
        if len(unique_details) > 5:
            print(f"  ... and {len(unique_details) - 5} more")
    
    print(f"\nFinal Summary:")
    print(f"  Working tests: {len(working_tests)}")
    print(f"  Error types found: {len(error_summary)}")
    print(f"  Total tests analyzed: {len(known_working) + len(additional_tests)}")

if __name__ == "__main__":
    main()
