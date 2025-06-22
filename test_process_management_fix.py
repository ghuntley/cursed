#!/usr/bin/env python3
"""
Test that the process management E0659 fixes work correctly.
"""

import subprocess
import re
from pathlib import Path

def test_e0659_process_conflicts():
    """Test that process-related E0659 conflicts are resolved"""
    
    print("Testing E0659 process management fixes...")
    
    # Run cargo check with linking fix
    try:
        result = subprocess.run(
            ['./fix_linking.sh', 'cargo', 'check'],
            capture_output=True,
            text=True,
            timeout=300
        )
        output = result.stderr
    except Exception as e:
        print(f"Error running cargo check: {e}")
        return False
    
    # Count E0659 errors
    total_e0659 = len(re.findall(r'error\[E0659\]', output))
    process_e0659 = len(re.findall(r'error\[E0659\].*(?:Process|Signal)', output))
    
    print(f"Total E0659 errors: {total_e0659}")
    print(f"Process-related E0659 errors: {process_e0659}")
    
    # Check specific types that were causing conflicts
    conflicting_types = ['Process', 'ProcessInfo', 'ProcessStatus', 'Signal', 'EnhancedProcess', 'ProcessGroup', 'ResourceLimits', 'SecurityContext']
    
    resolved_conflicts = []
    remaining_conflicts = []
    
    for conflict_type in conflicting_types:
        if re.search(rf'error\[E0659\].*`{conflict_type}` is ambiguous', output):
            remaining_conflicts.append(conflict_type)
        else:
            resolved_conflicts.append(conflict_type)
    
    print(f"\nResolved conflicts: {resolved_conflicts}")
    print(f"Remaining conflicts: {remaining_conflicts}")
    
    # Success if no process-related E0659 errors
    success = process_e0659 == 0
    
    if success:
        print("\n✅ SUCCESS: All process-related E0659 conflicts resolved!")
    else:
        print(f"\n❌ FAILURE: {process_e0659} process-related E0659 conflicts remain")
        
        # Show first few remaining conflicts
        process_errors = re.findall(r'error\[E0659\].*(?:Process|Signal).*', output)
        for i, error in enumerate(process_errors[:3]):
            print(f"  {i+1}. {error}")
    
    return success

def test_process_module_structure():
    """Test that the process module structure is correct"""
    
    print("\nTesting process module structure...")
    
    # Check that key files exist and have proper structure
    key_files = [
        'src/stdlib/process/mod.rs',
        'src/stdlib/process/type_disambiguation.rs',
        'src/runtime/process.rs',
        'src/stdlib/exec_slay/mod.rs',
    ]
    
    missing_files = []
    for file_path in key_files:
        if not Path(file_path).exists():
            missing_files.append(file_path)
    
    if missing_files:
        print(f"❌ Missing files: {missing_files}")
        return False
    
    # Check that type disambiguation file has proper content
    disambig_path = Path('src/stdlib/process/type_disambiguation.rs')
    with open(disambig_path, 'r') as f:
        disambig_content = f.read()
    
    required_aliases = ['RuntimeProcessInfo', 'StdProcessInfo', 'RuntimeProcessStatus', 'StdProcessState']
    missing_aliases = []
    
    for alias in required_aliases:
        if f'type {alias}' not in disambig_content:
            missing_aliases.append(alias)
    
    if missing_aliases:
        print(f"❌ Missing type aliases: {missing_aliases}")
        return False
    
    print("✅ Process module structure is correct")
    return True

def test_imports_are_explicit():
    """Test that wildcard imports have been replaced with explicit imports"""
    
    print("\nTesting explicit imports...")
    
    # Check key process files for wildcard imports
    process_files = [
        'src/stdlib/process/lifecycle.rs',
        'src/stdlib/process/integration.rs',
        'src/stdlib/process/unified_process_ipc.rs',
        'src/stdlib/process/unix_platform.rs',
    ]
    
    wildcard_issues = []
    
    for file_path_str in process_files:
        file_path = Path(file_path_str)
        if not file_path.exists():
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Check for problematic wildcard imports
        if 'use crate::stdlib::process::{' in content and '::*' in content:
            wildcard_issues.append(file_path_str)
    
    if wildcard_issues:
        print(f"❌ Files still using wildcard imports: {wildcard_issues}")
        return False
    
    print("✅ Wildcard imports have been replaced with explicit imports")
    return True

def main():
    print("=== Process Management E0659 Fix Validation ===\n")
    
    tests_passed = 0
    total_tests = 3
    
    # Run tests
    if test_e0659_process_conflicts():
        tests_passed += 1
    
    if test_process_module_structure():
        tests_passed += 1
    
    if test_imports_are_explicit():
        tests_passed += 1
    
    # Summary
    print(f"\n=== SUMMARY ===")
    print(f"Tests passed: {tests_passed}/{total_tests}")
    
    if tests_passed == total_tests:
        print("🎉 ALL TESTS PASSED - E0659 process management fixes are successful!")
        return True
    else:
        print("❌ Some tests failed - additional fixes may be needed")
        return False

if __name__ == '__main__':
    success = main()
    exit(0 if success else 1)
