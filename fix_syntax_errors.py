#!/usr/bin/env python3

import os
import re
import glob

def fix_syntax_errors(file_path):
    """Fix common syntax errors in test files"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix collect() placement in iterator chains
        content = re.sub(r'\.map\(\|e\| e\.to_string\(\)\.collect::<Vec<_>>\(\)\.join\(', 
                        r'.map(|e| e.to_string()).collect::<Vec<_>>().join(', content)
        
        # Fix misplaced .? operator
        content = re.sub(r'\.map_err\(\|e\| e\.to_string\(\)\?\;', 
                        r'.map_err(|e| e.to_string())?;', content)
        
        # Fix double closing brackets/parentheses
        content = re.sub(r'\)\)\;', r');', content)
        content = re.sub(r'\]\]\;', r'];', content)
        content = re.sub(r'\}\}\;', r'};', content)
        
        # Fix missing closing brackets in method chains
        content = re.sub(r'\.join\("\n"\)', r'.join("\n");', content)
        
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed syntax errors in {file_path}")
            return True
        return False
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def main():
    test_files = glob.glob("tests/*.rs")
    fixed_count = 0
    
    for file_path in test_files:
        if fix_syntax_errors(file_path):
            fixed_count += 1
    
    print(f"\nFixed syntax errors in {fixed_count} files")

if __name__ == "__main__":
    main()
