#!/usr/bin/env python3

import re
import os
import glob

def fix_crypto_zk_imports_in_file(file_path):
    """Fix crypto_zk import errors"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix import paths
        content = re.sub(r'use crate::stdlib::error::', r'use crate::error::', content)
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed crypto_zk imports in: {file_path}")
            return True
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False
    
    return False

def main():
    """Fix crypto_zk import issues"""
    
    rust_patterns = [
        'src/stdlib/packages/crypto_zk/*.rs',
    ]
    
    files_fixed = 0
    total_files = 0
    
    for pattern in rust_patterns:
        for file_path in glob.glob(pattern, recursive=True):
            if os.path.isfile(file_path):
                total_files += 1
                if fix_crypto_zk_imports_in_file(file_path):
                    files_fixed += 1
    
    print(f"\nSummary:")
    print(f"  Total crypto_zk files processed: {total_files}")
    print(f"  Files with import issues fixed: {files_fixed}")

if __name__ == "__main__":
    main()
