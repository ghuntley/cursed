#!/usr/bin/env python3
"""
Generate comprehensive test status report for CURSED codebase.
"""

import subprocess
import re
import glob

def check_compilation_status():
    """Check which tests compile successfully."""
    
    # Get all test files
    test_files = glob.glob("tests/*.rs")
    
    print("=== CURSED Test Compilation Status Report ===\n")
    
    passing_tests = []
    failing_tests = []
    
    for test_file in sorted(test_files):
        test_name = test_file.replace("tests/", "").replace(".rs", "")
        
        try:
            # Try to compile the test without running it
            result = subprocess.run(
                ["./fix_linking.sh", "cargo", "test", "--test", test_name, "--no-run"],
                capture_output=True,
                text=True,
                timeout=30
            )
            
            if result.returncode == 0:
                passing_tests.append(test_name)
                print(f"✓ {test_name}")
            else:
                failing_tests.append(test_name)
                # Extract main error from stderr
                error_lines = result.stderr.split('\n')
                main_errors = [line for line in error_lines if 'error:' in line or 'error[' in line]
                if main_errors:
                    print(f"✗ {test_name} - {main_errors[0][:80]}")
                else:
                    print(f"✗ {test_name} - compilation failed")
                    
        except subprocess.TimeoutExpired:
            failing_tests.append(test_name)
            print(f"✗ {test_name} - timeout")
        except Exception as e:
            failing_tests.append(test_name)
            print(f"✗ {test_name} - exception: {e}")
    
    print(f"\n=== Summary ===")
    print(f"Total tests: {len(test_files)}")
    print(f"Compiling: {len(passing_tests)}")
    print(f"Failing: {len(failing_tests)}")
    print(f"Success rate: {len(passing_tests)/len(test_files)*100:.1f}%")
    
    if passing_tests:
        print(f"\n=== Working Tests ===")
        for test in passing_tests:
            print(f"  {test}")
    
    return passing_tests, failing_tests

def run_working_tests(passing_tests):
    """Run the tests that compile to see which actually pass."""
    
    print(f"\n=== Running Working Tests ===")
    
    actually_passing = []
    runtime_failing = []
    
    for test_name in passing_tests:
        try:
            result = subprocess.run(
                ["./fix_linking.sh", "cargo", "test", "--test", test_name],
                capture_output=True,
                text=True,
                timeout=30
            )
            
            if result.returncode == 0:
                # Count passed tests
                output = result.stdout
                if "test result: ok" in output:
                    passed_match = re.search(r'(\d+) passed', output)
                    if passed_match:
                        count = passed_match.group(1)
                        actually_passing.append(f"{test_name} ({count} tests)")
                        print(f"✓ {test_name} - {count} tests passed")
                    else:
                        actually_passing.append(test_name)
                        print(f"✓ {test_name}")
                else:
                    runtime_failing.append(test_name)
                    print(f"✗ {test_name} - runtime failure")
            else:
                runtime_failing.append(test_name)
                print(f"✗ {test_name} - execution failed")
                
        except subprocess.TimeoutExpired:
            runtime_failing.append(test_name)
            print(f"✗ {test_name} - timeout during execution")
        except Exception as e:
            runtime_failing.append(test_name)
            print(f"✗ {test_name} - exception: {e}")
    
    print(f"\n=== Execution Summary ===")
    print(f"Tests that compile: {len(passing_tests)}")
    print(f"Tests that pass: {len(actually_passing)}")
    print(f"Tests with runtime issues: {len(runtime_failing)}")
    
    return actually_passing

def main():
    print("Checking compilation status of all tests...")
    passing_tests, failing_tests = check_compilation_status()
    
    if passing_tests:
        print(f"\nRunning {len(passing_tests)} tests that compile...")
        actually_passing = run_working_tests(passing_tests)
        
        print(f"\n=== Final Status ===")
        print(f"✅ Tests working end-to-end: {len(actually_passing)}")
        if actually_passing:
            for test in actually_passing:
                print(f"   {test}")
    
    print(f"\n=== Next Steps ===")
    print("1. Fix compilation errors in failing tests")
    print("2. Address runtime issues in tests that compile but fail")
    print("3. Add any missing test dependencies or imports")
    print("4. Run full test suite with `make test`")

if __name__ == "__main__":
    main()
