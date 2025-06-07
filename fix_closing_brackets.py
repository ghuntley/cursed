#!/usr/bin/env python3

import os
import re
import glob

def fix_closing_brackets(file_path):
    """Fix missing closing brackets and parentheses in test files"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix missing closing parentheses in common patterns
        content = re.sub(r'assert_eq!\(([^)]+), Some\(([^)]+)\);', r'assert_eq!(\1, Some(\2));', content)
        content = re.sub(r'assert!\(([^)]+), "([^"]+)", ([^)]+)\.err\(\);', r'assert!(\1, "\2", \3.err());', content)
        content = re.sub(r'println!\("([^"]+)", ([^)]+);', r'println!("\1", \2);', content)
        
        # Fix missing closing bracket in to_string() calls
        content = re.sub(r'\.to_string\(\);(?=\s*$)', r'.to_string());', content, flags=re.MULTILINE)
        
        # Fix method chains with missing closing brackets
        content = re.sub(r'(\w+\(.*\))\.(\w+)\(\);', r'\1.\2());', content)
        
        # Fix assert statements with missing closing parentheses
        content = re.sub(r'assert!\(([^)]+\.is_empty\(\));', r'assert!(\1.is_empty());', content)
        
        # Fix result.err() calls with missing parentheses
        content = re.sub(r'result\.err\(\);', r'result.err());', content)
        
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed closing brackets in {file_path}")
            return True
        return False
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def main():
    test_files = glob.glob("tests/*.rs")
    fixed_count = 0
    
    for file_path in test_files:
        if fix_closing_brackets(file_path):
            fixed_count += 1
    
    print(f"\nFixed closing brackets in {fixed_count} files")

if __name__ == "__main__":
    main()
