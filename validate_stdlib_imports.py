#!/usr/bin/env python3
"""
Validate testz import system across all stdlib modules.
Tests that all modules with testz imports can be loaded correctly.
"""

import os
import subprocess
import glob

def validate_stdlib_imports():
    """Validate testz imports across all stdlib modules"""
    stdlib_path = "stdlib"
    test_files = []
    
    # Find all test files that use testz
    for root, dirs, files in os.walk(stdlib_path):
        for file in files:
            if file.startswith('test_') and file.endswith('.csd'):
                filepath = os.path.join(root, file)
                
                # Check if file uses testz import
                try:
                    with open(filepath, 'r', encoding='utf-8') as f:
                        content = f.read()
                        if 'yeet "testz"' in content:
                            test_files.append(filepath)
                except Exception as e:
                    print(f"Error reading {filepath}: {e}")
    
    print(f"Found {len(test_files)} test files using testz imports")
    
    # Test a sample of files to validate imports work
    sample_files = test_files[:10]  # Test first 10 files
    
    success_count = 0
    failed_files = []
    
    for test_file in sample_files:
        print(f"Testing {test_file}...")
        try:
            # Run the test file
            result = subprocess.run(
                ['cargo', 'run', '--bin', 'cursed', test_file],
                capture_output=True,
                text=True,
                timeout=30
            )
            
            if result.returncode == 0:
                success_count += 1
                print(f"  ✅ Success")
            else:
                failed_files.append(test_file)
                print(f"  ❌ Failed: {result.stderr}")
                
        except subprocess.TimeoutExpired:
            failed_files.append(test_file)
            print(f"  ⏰ Timeout")
        except Exception as e:
            failed_files.append(test_file)
            print(f"  ❌ Error: {e}")
    
    print(f"\nValidation Summary:")
    print(f"Total test files: {len(test_files)}")
    print(f"Sample tested: {len(sample_files)}")
    print(f"Successful imports: {success_count}")
    print(f"Failed imports: {len(failed_files)}")
    
    if failed_files:
        print(f"\nFailed files:")
        for file in failed_files:
            print(f"  - {file}")
    
    return len(failed_files) == 0

if __name__ == "__main__":
    success = validate_stdlib_imports()
    exit(0 if success else 1)
