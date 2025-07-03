#!/usr/bin/env python3

import subprocess
import re
import os

def fix_error_conversions():
    """Fix error type conversions where functions return specific errors but need CursedError"""
    
    # Files that need error conversions
    files_to_fix = [
        'src/stdlib/packages/db_nosql/redis.rs',
        'src/stdlib/packages/db_nosql/mongodb.rs',
    ]
    
    fixed_count = 0
    
    for file_path in files_to_fix:
        if not os.path.exists(file_path):
            continue
        
        print(f"Processing {file_path}...")
        
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Change function signatures from ModuleResult to return Result<T, CursedError>
        # and convert the error returns
        
        # First, change the imports to include CursedError
        if 'use crate::error::CursedError;' not in content:
            lines = content.split('\n')
            
            # Find where to insert the import
            insert_pos = 0
            for i, line in enumerate(lines):
                if line.strip().startswith('use '):
                    insert_pos = i + 1
            
            lines.insert(insert_pos, "use crate::error::CursedError;")
            content = '\n'.join(lines)
        
        # Change ModuleResult<T> to Result<T, CursedError>
        content = content.replace('ModuleResult<()', 'Result<(), CursedError')
        content = content.replace('ModuleResult<Option<String>>', 'Result<Option<String>, CursedError>')
        content = content.replace('ModuleResult<String>', 'Result<String, CursedError>')
        content = content.replace('ModuleResult<Vec<String>>', 'Result<Vec<String>, CursedError>')
        content = content.replace('ModuleResult<bool>', 'Result<bool, CursedError>')
        
        # Convert ModuleError returns to use From trait
        content = content.replace(
            'ModuleError::Other(',
            'CursedError::from(ModuleError::Other('
        )
        
        # Fix the return statements to close the from conversion
        content = content.replace(
            'CursedError::from(ModuleError::Other("Connection is closed".to_string()));',
            'CursedError::from(ModuleError::Other("Connection is closed".to_string()))'
        )
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed error conversions in {file_path}")
            fixed_count += 1
    
    return fixed_count

def main():
    print("Fixing error type conversions...")
    
    # Get initial count
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    initial_errors = result.stderr.count('error[E0308]')
    print(f"Initial type mismatch errors: {initial_errors}")
    
    # Fix conversions
    fixed_count = fix_error_conversions()
    print(f"Fixed {fixed_count} files")
    
    # Check results
    print("\nRunning cargo check to verify fixes...")
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    
    final_errors = result.stderr.count('error[E0308]')
    print(f"Remaining type mismatch errors: {final_errors}")
    print(f"Reduced type mismatch errors by: {initial_errors - final_errors}")
    
    if final_errors > 0:
        print("\nSample remaining errors:")
        lines = result.stderr.split('\n')
        for i, line in enumerate(lines):
            if 'expected `' in line and 'found `' in line:
                print(f"  {line.strip()}")
                # Also show context
                if i > 0:
                    prev_line = lines[i-1].strip()
                    if '-->' in prev_line:
                        print(f"    Context: {prev_line}")
                break

if __name__ == "__main__":
    main()
