#!/usr/bin/env python3

import subprocess
import re
import os

def get_actual_error_files():
    """Get actual source files with error type mismatches"""
    result = subprocess.run(
        ["cargo", "check"], 
        capture_output=True, 
        text=True, 
        cwd="."
    )
    
    mismatches = []
    lines = result.stderr.split('\n')
    current_file = None
    current_line = None
    
    for i, line in enumerate(lines):
        # Look for file references
        if '-->' in line and '.rs:' in line and line.strip().startswith('-->'):
            file_info = line.split('-->')[1].strip()
            if file_info.startswith('src/'):  # Only our source files
                parts = file_info.split(':')
                current_file = parts[0]
                current_line = int(parts[1]) if len(parts) > 1 else None
        
        # Look for type mismatch errors
        elif 'expected `' in line and 'found `CursedError`' in line and current_file:
            expected_match = re.search(r'expected `(\w+Error)`', line)
            if expected_match and current_line:
                expected_type = expected_match.group(1)
                mismatches.append({
                    'file': current_file,
                    'line': current_line,
                    'expected': expected_type
                })
    
    return mismatches

def fix_specific_files():
    """Fix specific files we know have issues based on earlier analysis"""
    files_to_fix = [
        'src/stdlib/packages/db_sql/connection.rs',
        'src/stdlib/packages/db_sql/migration.rs',
        'src/stdlib/packages/db_migrate/migration.rs',
        'src/stdlib/packages/db_nosql/redis.rs',
        'src/stdlib/packages/db_nosql/mongodb.rs'
    ]
    
    fixed_count = 0
    
    for file_path in files_to_fix:
        if not os.path.exists(file_path):
            continue
            
        print(f"Processing {file_path}...")
        
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Determine expected error type based on module
        if 'db_' in file_path or '/io/' in file_path:
            expected_error = 'IOError'
        elif 'crypto' in file_path:
            expected_error = 'CryptoError'
        elif 'module' in file_path:
            expected_error = 'ModuleError'
        elif 'pki' in file_path:
            expected_error = 'PkiError'
        else:
            expected_error = 'IOError'  # Default for most cases
        
        # Replace CursedError::runtime_error with appropriate error
        content = content.replace(
            'CursedError::runtime_error(',
            f'{expected_error}::Other('
        )
        
        # Fix string literal issues
        content = content.replace('(&"', '("')
        
        # Ensure import is present
        if expected_error in content and f'use crate::stdlib::packages::{expected_error}' not in content:
            lines = content.split('\n')
            
            # Find import section
            import_inserted = False
            for i, line in enumerate(lines):
                if 'use crate::stdlib::packages::' in line and '{' in line:
                    # Modify existing import
                    import_part = line.split('{')[1].split('}')[0]
                    imports = [imp.strip() for imp in import_part.split(',')]
                    if expected_error not in imports:
                        imports.append(expected_error)
                        new_import = f"use crate::stdlib::packages::{{{', '.join(sorted(imports))}}};"
                        lines[i] = new_import
                        import_inserted = True
                        break
            
            if not import_inserted:
                # Add new import
                insert_pos = 0
                for i, line in enumerate(lines):
                    if line.strip().startswith('use '):
                        insert_pos = i + 1
                
                lines.insert(insert_pos, f"use crate::stdlib::packages::{expected_error};")
            
            content = '\n'.join(lines)
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed {file_path}")
            fixed_count += 1
    
    return fixed_count

def main():
    print("Fixing error type mismatches in specific files...")
    
    # Get initial count
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    initial_errors = result.stderr.count('error[E0308]')
    print(f"Initial type mismatch errors: {initial_errors}")
    
    # Fix specific files
    fixed_count = fix_specific_files()
    print(f"Fixed {fixed_count} files")
    
    # Check results
    print("\nRunning cargo check to verify fixes...")
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    
    final_errors = result.stderr.count('error[E0308]')
    print(f"Remaining type mismatch errors: {final_errors}")
    print(f"Reduced type mismatch errors by: {initial_errors - final_errors}")
    
    # Show sample of remaining errors
    if final_errors > 0:
        print("\nSample remaining errors:")
        lines = result.stderr.split('\n')
        error_count = 0
        for line in lines:
            if 'expected `' in line and 'found `' in line:
                print(f"  {line.strip()}")
                error_count += 1
                if error_count >= 5:
                    break

if __name__ == "__main__":
    main()
