#!/usr/bin/env python3

import os
import re

def fix_collections_error_returns():
    """Fix collections files to return IOError instead of CursedError"""
    
    collections_files = []
    
    # Find all collections files
    for root, dirs, files in os.walk('src/stdlib/collections'):
        for file in files:
            if file.endswith('.rs'):
                collections_files.append(os.path.join(root, file))
    
    total_fixes = 0
    
    for file_path in collections_files:
        if not os.path.exists(file_path):
            continue
            
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Fix CursedError::runtime_error back to IOError::Other for collections
            content = re.sub(
                r'return Err\(CursedError::runtime_error\(&"I/O test failed"\.to_string\(\)\)\);',
                'return Err(IOError::Other("I/O test failed".to_string()));',
                content
            )
            
            content = re.sub(
                r'return Err\(CursedError::runtime_error\(&"I/O string test failed"\.to_string\(\)\)\);',
                'return Err(IOError::Other("I/O string test failed".to_string()));',
                content
            )
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                print(f"Fixed collections errors in {file_path}")
                total_fixes += 1
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    return total_fixes

def main():
    print("Fixing collections error types...")
    total_fixes = fix_collections_error_returns()
    print(f"Fixed {total_fixes} collections files")

if __name__ == "__main__":
    main()
