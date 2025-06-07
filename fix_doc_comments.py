#!/usr/bin/env python3

import os
import re
import glob

def fix_doc_comments(file_path):
    """Fix outer doc comments by converting //! to //"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Replace outer doc comments with regular comments
        # Only at start of lines (possibly with whitespace)
        content = re.sub(r'^(\s*)//!', r'\1//', content, flags=re.MULTILINE)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"Fixed doc comments in {file_path}")
        return True
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def main():
    test_files = glob.glob("tests/*.rs")
    fixed_count = 0
    
    for file_path in test_files:
        if fix_doc_comments(file_path):
            fixed_count += 1
    
    print(f"\nFixed doc comments in {fixed_count} files")

if __name__ == "__main__":
    main()
