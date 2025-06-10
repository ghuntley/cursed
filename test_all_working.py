#!/usr/bin/env python3
"""
Test all individual test files to see which ones pass/fail
"""

import subprocess
import glob
import os
from pathlib import Path

def run_test(test_file):
    """Run a single test file and return (test_name, success, output)"""
    test_name = Path(test_file).stem
    
    try:
        # Run with linking fix
        result = subprocess.run([
            './fix_linking.sh', 'cargo', 'test', '--test', test_name
        ], capture_output=True, text=True, timeout=60)
        
        success = result.returncode == 0
        return test_name, success, result.stderr + result.stdout
    except subprocess.TimeoutExpired:
        return test_name, False, "Test timed out"
    except Exception as e:
        return test_name, False, f"Error running test: {e}"

def main():
    # Get all test files
    test_files = glob.glob("tests/*.rs")
    
    # Filter out some known problematic patterns
    excluded_patterns = [
        'common.rs',
        'tracing_setup.rs', 
        'test_utilities.rs',
        'database_test_utilities.rs'
    ]
    
    test_files = [f for f in test_files if not any(pattern in f for pattern in excluded_patterns)]
    
    print(f"Found {len(test_files)} test files to check")
    
    working_tests = []
    failing_tests = []
    
    for i, test_file in enumerate(test_files, 1):
        test_name = Path(test_file).stem
        print(f"[{i}/{len(test_files)}] Testing {test_name}...", end=" ")
        
        name, success, output = run_test(test_file)
        
        if success:
            print("✅ PASS")
            working_tests.append(name)
        else:
            print("❌ FAIL")
            failing_tests.append((name, output))
    
    print(f"\n=== SUMMARY ===")
    print(f"✅ Working tests: {len(working_tests)}")
    print(f"❌ Failing tests: {len(failing_tests)}")
    print(f"📊 Success rate: {len(working_tests) / len(test_files) * 100:.1f}%")
    
    print(f"\n=== WORKING TESTS ===")
    for test in sorted(working_tests):
        print(f"  ✅ {test}")
    
    print(f"\n=== FAILING TESTS ===")
    for test, _ in sorted(failing_tests):
        print(f"  ❌ {test}")
    
    # Write detailed failure info to file
    with open("test_failures.log", "w") as f:
        f.write("=== DETAILED FAILURE OUTPUT ===\n\n")
        for test, output in failing_tests:
            f.write(f"=== {test} ===\n")
            f.write(output[:2000])  # Limit output size
            f.write("\n\n")
    
    print(f"\nDetailed failure output written to test_failures.log")

if __name__ == "__main__":
    main()
