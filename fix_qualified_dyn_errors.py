#!/usr/bin/env python3

import re
import os
import glob

def fix_qualified_dyn_errors_in_file(file_path):
    """Fix qualified trait object errors by adding 'dyn' keyword"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Pattern for fully qualified paths like &crate::ast::traits::Expression
        content = re.sub(r'&(crate::ast::traits::Expression)(?![a-zA-Z0-9_])', r'&dyn \1', content)
        content = re.sub(r'&(crate::ast::traits::Statement)(?![a-zA-Z0-9_])', r'&dyn \1', content)
        
        # Pattern for arrays with qualified paths
        content = re.sub(r'&\[&(crate::ast::traits::Expression)\]', r'&[&dyn \1]', content)
        content = re.sub(r'&\[&(crate::ast::traits::Statement)\]', r'&[&dyn \1]', content)
        
        # Pattern for Options with qualified paths
        content = re.sub(r'Option<&(crate::ast::traits::Expression)>', r'Option<&dyn \1>', content)
        content = re.sub(r'Option<&(crate::ast::traits::Statement)>', r'Option<&dyn \1>', content)
        
        # Fix duplicate dyn keywords
        content = re.sub(r'&dyn dyn ', r'&dyn ', content)
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed qualified dyn errors in: {file_path}")
            return True
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False
    
    return False

def main():
    """Fix all qualified dyn errors in the codebase"""
    
    rust_patterns = [
        'src/**/*.rs',
        'tests/**/*.rs',
    ]
    
    files_fixed = 0
    total_files = 0
    
    for pattern in rust_patterns:
        for file_path in glob.glob(pattern, recursive=True):
            if os.path.isfile(file_path):
                total_files += 1
                if fix_qualified_dyn_errors_in_file(file_path):
                    files_fixed += 1
    
    print(f"\nSummary:")
    print(f"  Total Rust files processed: {total_files}")
    print(f"  Files with qualified dyn errors fixed: {files_fixed}")

if __name__ == "__main__":
    main()
