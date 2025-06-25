#!/usr/bin/env python3

import os
import re

def fix_result_types():
    """Fix Result<(), CursedError> patterns to use proper Result<()>"""
    
    # Find all rust files
    rust_files = []
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(os.path.join(root, file))
    
    fixes_made = 0
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Fix Result patterns
            patterns = [
                # Result<(), CursedError> -> crate::error_types::Result<()>
                (r'Result<\(\), crate::error_types::CursedError>', 'crate::error_types::Result<()>'),
                (r'Result<\(\), CursedError>', 'crate::error_types::Result<()>'),
                # Result<T, CursedError> -> crate::error_types::Result<T>
                (r'Result<([^,]+), crate::error_types::CursedError>', r'crate::error_types::Result<\1>'),
                (r'Result<([^,]+), CursedError>', r'crate::error_types::Result<\1>'),
                # Fix std::result::Result patterns
                (r'std::result::Result<\(\), crate::error_types::CursedError>', 'crate::error_types::Result<()>'),
                (r'std::result::Result<([^,]+), crate::error_types::CursedError>', r'crate::error_types::Result<\1>'),
            ]
            
            for pattern, replacement in patterns:
                content = re.sub(pattern, replacement, content)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixes_made += 1
                print(f"Fixed Result types in {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    print(f"Fixed {fixes_made} files")

if __name__ == "__main__":
    fix_result_types()
