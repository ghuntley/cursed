#!/usr/bin/env python3

import re
import os
import glob

def fix_missing_semicolons(file_path):
    """Fix missing semicolons after LlvmCodeGenerator::new calls"""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Fix missing semicolon after LlvmCodeGenerator::new calls
    content = re.sub(r'(LlvmCodeGenerator::new\([^)]+PathBuf::from\([^)]+\)\))$', r'\1;', content, flags=re.MULTILINE)
    
    # Fix any other missing semicolons patterns
    content = re.sub(r'(let [^=]+ = [^;]+)$\n\s*$\n\s*//[^\n]*$', r'\1;\n\n    //', content, flags=re.MULTILINE)
    
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"Fixed missing semicolons in {file_path}")
        return True
    return False

def main():
    # Find all test files
    test_files = glob.glob('tests/*.rs')
    
    fixed_count = 0
    for test_file in test_files:
        if fix_missing_semicolons(test_file):
            fixed_count += 1
    
    print(f"Fixed missing semicolons in {fixed_count} files")

if __name__ == "__main__":
    main()
