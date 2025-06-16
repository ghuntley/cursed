#!/usr/bin/env python3

import re
import os
import glob
from pathlib import Path

def fix_dyn_errors_in_file(file_path):
    """Fix trait object errors by adding 'dyn' keyword"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Pattern 1: &Expression -> &dyn Expression (most common)
        content = re.sub(r'&(Expression)(?![a-zA-Z0-9_])', r'&dyn \1', content)
        
        # Pattern 2: &Statement -> &dyn Statement
        content = re.sub(r'&(Statement)(?![a-zA-Z0-9_])', r'&dyn \1', content)
        
        # Pattern 3: &[&Expression] -> &[&dyn Expression]
        content = re.sub(r'&\[&(Expression)\]', r'&[&dyn \1]', content)
        
        # Pattern 4: &[&Statement] -> &[&dyn Statement]
        content = re.sub(r'&\[&(Statement)\]', r'&[&dyn \1]', content)
        
        # Pattern 5: &[Expression] -> &[dyn Expression]
        content = re.sub(r'&\[(Expression)\]', r'&[dyn \1]', content)
        
        # Pattern 6: &[Statement] -> &[dyn Statement]
        content = re.sub(r'&\[(Statement)\]', r'&[dyn \1]', content)
        
        # Pattern 7: Option<&Expression> -> Option<&dyn Expression>
        content = re.sub(r'Option<&(Expression)>', r'Option<&dyn \1>', content)
        
        # Pattern 8: Option<&Statement> -> Option<&dyn Statement>
        content = re.sub(r'Option<&(Statement)>', r'Option<&dyn \1>', content)
        
        # Fix duplicate dyn keywords that might be introduced
        content = re.sub(r'&dyn dyn ', r'&dyn ', content)
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed dyn errors in: {file_path}")
            return True
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False
    
    return False

def main():
    """Fix all dyn errors in the codebase"""
    
    # List of patterns to search for Rust files
    rust_patterns = [
        'src/**/*.rs',
        'tests/**/*.rs',
        'benches/**/*.rs',
        'examples/**/*.rs',
    ]
    
    files_fixed = 0
    total_files = 0
    
    for pattern in rust_patterns:
        for file_path in glob.glob(pattern, recursive=True):
            if os.path.isfile(file_path):
                total_files += 1
                if fix_dyn_errors_in_file(file_path):
                    files_fixed += 1
    
    print(f"\nSummary:")
    print(f"  Total Rust files processed: {total_files}")
    print(f"  Files with dyn errors fixed: {files_fixed}")

if __name__ == "__main__":
    main()
