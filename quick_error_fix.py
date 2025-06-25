#!/usr/bin/env python3

import os
import re

def fix_duplicate_error_imports(file_path):
    """Fix duplicate Error imports in a single file"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Remove duplicate CursedError imports
        lines = content.split('\n')
        seen_cursederror_import = False
        fixed_lines = []
        
        for line in lines:
            # Skip duplicate CursedError imports  
            if 'use crate::error_types::CursedError;' in line and seen_cursederror_import:
                print(f"  Removing duplicate import: {line.strip()}")
                continue
            elif 'use crate::error_types::CursedError;' in line:
                seen_cursederror_import = True
            
            # Fix conflicting imports
            if 'use crate::{Error,' in line:
                line = line.replace('Error,', 'CursedError,')
                print(f"  Fixed import: {line.strip()}")
            elif 'use crate::{CursedError,' in line and 'use crate::error_types::CursedError;' in content:
                # Replace with unified import
                line = line.replace('use crate::{CursedError,', 'use crate::error_types::{CursedError,')
                print(f"  Unified import: {line.strip()}")
            
            fixed_lines.append(line)
        
        new_content = '\n'.join(fixed_lines)
        
        if new_content != original_content:
            with open(file_path, 'w') as f:
                f.write(new_content)
            return True
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
    
    return False

def main():
    print("Fixing duplicate Error imports in key files...")
    
    # Focus on files that most likely have the E0252 errors
    target_files = [
        'src/ast/expressions/error_propagation_enhanced.rs',
        'src/ast/documentation.rs', 
        'src/parser/async_await.rs',
        'src/lexer.rs',
        'src/execution/runtime_functions.rs',
        'src/repl/build_integration.rs',
        'src/repl/interface.rs'
    ]
    
    fixed_count = 0
    
    for file_path in target_files:
        if os.path.exists(file_path):
            print(f"Processing {file_path}...")
            if fix_duplicate_error_imports(file_path):
                fixed_count += 1
        else:
            print(f"Skipping {file_path} (not found)")
    
    print(f"\nFixed imports in {fixed_count} files")

if __name__ == "__main__":
    main()
