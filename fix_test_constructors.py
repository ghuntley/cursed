#!/usr/bin/env python3

import os
import re

def fix_constructor_calls(file_path):
    """Fix LlvmCodeGenerator constructor calls in test files"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Pattern to match LlvmCodeGenerator::new(...) calls with arguments
        pattern = r'LlvmCodeGenerator::new\([^)]+\)'
        replacement = 'LlvmCodeGenerator::new().unwrap()'
        
        # Apply the fix
        new_content = re.sub(pattern, replacement, content)
        
        if new_content != content:
            with open(file_path, 'w') as f:
                f.write(new_content)
            print(f"Fixed constructor calls in {file_path}")
            return True
        else:
            print(f"No constructor issues found in {file_path}")
            return False
            
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """Fix constructor calls in all test files"""
    test_files = []
    
    # Find all test files in tests/ directory
    for root, dirs, files in os.walk('tests'):
        for file in files:
            if file.endswith('.rs'):
                test_files.append(os.path.join(root, file))
    
    print(f"Found {len(test_files)} test files")
    fixed_count = 0
    
    for file_path in test_files:
        if fix_constructor_calls(file_path):
            fixed_count += 1
    
    print(f"Fixed constructor calls in {fixed_count} files")

if __name__ == "__main__":
    main()
