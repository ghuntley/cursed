#!/usr/bin/env python3

import subprocess
import glob
import os
import sys

def run_test(test_name):
    """Run a single test and return True if it passes"""
    try:
        # Strip .rs extension and get base name
        test_name = os.path.basename(test_name).replace('.rs', '')
        
        result = subprocess.run(
            ['./fix_linking.sh', 'cargo', 'test', '--test', test_name],
            capture_output=True,
            text=True,
            timeout=30
        )
        return result.returncode == 0
    except Exception:
        return False

def main():
    print("Counting test results...")
    
    test_files = glob.glob('tests/*.rs')
    # Filter out helper files
    test_files = [f for f in test_files if not any(helper in f for helper in [
        'common.rs', 'tracing_setup.rs', 'ast_factory.rs', 'test_helpers.rs',
        'token_helper.rs', 'switch_test_helper.rs', 'llvm_test_helpers.rs',
        'database_test_utilities.rs', 'range_clause_test_helper.rs'
    ])]
    
    passed = 0
    failed = 0
    
    print(f"Found {len(test_files)} test files to check")
    
    for i, test_file in enumerate(test_files):
        test_name = os.path.basename(test_file).replace('.rs', '')
        print(f"[{i+1}/{len(test_files)}] Testing {test_name}...", end=' ')
        
        if run_test(test_file):
            print("✓ PASS")
            passed += 1
        else:
            print("✗ FAIL")
            failed += 1
    
    print(f"\n=== SUMMARY ===")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    print(f"Total:  {passed + failed}")
    print(f"Success rate: {passed/(passed+failed)*100:.1f}%")
    
    return 0 if failed == 0 else 1

if __name__ == '__main__':
    sys.exit(main())
