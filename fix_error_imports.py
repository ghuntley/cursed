#!/usr/bin/env python3
"""
Fix Error type import issues across the CURSED codebase.
Adds `use crate::error::Error;` where needed.
"""

import os
import re

def add_error_import(file_path):
    """Add Error import to a file if needed and not already present."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Skip if Error is already imported from crate::error
        if 'use crate::error::Error' in content or 'use crate::error::{' in content:
            return False
        
        # Skip if file doesn't seem to use Error type
        if not re.search(r'\bError\b', content):
            return False
        
        # Find the last use statement to insert after it
        lines = content.split('\n')
        last_use_index = -1
        
        for i, line in enumerate(lines):
            if line.strip().startswith('use ') and not line.strip().startswith('use std::'):
                last_use_index = i
        
        if last_use_index == -1:
            # Find first use statement
            for i, line in enumerate(lines):
                if line.strip().startswith('use '):
                    last_use_index = i
                    break
        
        if last_use_index >= 0:
            # Insert after the last use statement
            lines.insert(last_use_index + 1, 'use crate::error::Error;')
            
            new_content = '\n'.join(lines)
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            
            print(f"✅ Added Error import to {file_path}")
            return True
    
    except Exception as e:
        print(f"❌ Error processing {file_path}: {e}")
        return False
    
    return False

def fix_error_imports():
    """Fix Error imports in key files that need them."""
    
    # Files that definitely need Error import based on compilation errors
    target_files = [
        'src/stdlib/collections/heap_slay/mod.rs',
        'src/stdlib/collections/sorta_fresh/mod.rs', 
        'src/stdlib/fs/watcher.rs',
        'src/error/error_propagation.rs',
    ]
    
    # Also check all stdlib files for potential Error usage
    stdlib_dirs = [
        'src/stdlib/crypto',
        'src/stdlib/math',
        'src/stdlib/io',
        'src/stdlib/fs',
        'src/stdlib/collections',
        'src/stdlib/system',
        'src/stdlib/database',
    ]
    
    fixed_count = 0
    
    # Fix specific target files
    for file_path in target_files:
        if os.path.exists(file_path):
            if add_error_import(file_path):
                fixed_count += 1
    
    # Scan stdlib directories
    for dir_path in stdlib_dirs:
        if os.path.exists(dir_path):
            for root, dirs, files in os.walk(dir_path):
                for file in files:
                    if file.endswith('.rs'):
                        file_path = os.path.join(root, file)
                        if add_error_import(file_path):
                            fixed_count += 1
    
    print(f"\n🎉 Fixed Error imports in {fixed_count} files")
    return fixed_count

if __name__ == '__main__':
    fix_error_imports()
