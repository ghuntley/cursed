#!/usr/bin/env python3

import subprocess
import re
import os

def get_error_type_mismatches():
    """Parse cargo check output to find error type mismatches"""
    result = subprocess.run(
        ["cargo", "check"], 
        capture_output=True, 
        text=True, 
        cwd="."
    )
    
    mismatches = []
    lines = result.stderr.split('\n')
    
    for i, line in enumerate(lines):
        if 'expected `' in line and 'found `CursedError`' in line:
            # Extract expected type and file info
            expected_match = re.search(r'expected `(\w+Error)`', line)
            if expected_match:
                expected_type = expected_match.group(1)
                
                # Look for file path in previous lines
                for j in range(i-10, i):
                    if j >= 0 and '-->' in lines[j] and '.rs:' in lines[j]:
                        file_info = lines[j].split('-->')[1].strip()
                        file_path = file_info.split(':')[0]
                        line_num = file_info.split(':')[1]
                        
                        mismatches.append({
                            'file': file_path,
                            'line': int(line_num),
                            'expected': expected_type,
                            'found': 'CursedError'
                        })
                        break
    
    return mismatches

def fix_error_in_file(file_path, line_num, expected_type):
    """Fix a specific error type mismatch in a file"""
    if not os.path.exists(file_path):
        print(f"File not found: {file_path}")
        return False
        
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    if line_num > len(lines):
        print(f"Line {line_num} not found in {file_path}")
        return False
    
    original_line = lines[line_num - 1]  # line_num is 1-based
    
    # Check if this line contains CursedError::runtime_error
    if 'CursedError::runtime_error(' in original_line:
        # Replace with appropriate error type
        if expected_type == 'IOError':
            new_line = original_line.replace(
                'CursedError::runtime_error(',
                'IOError::Other('
            )
        elif expected_type == 'CryptoError':
            new_line = original_line.replace(
                'CursedError::runtime_error(',
                'CryptoError::Other('
            )
        elif expected_type == 'ModuleError':
            new_line = original_line.replace(
                'CursedError::runtime_error(',
                'ModuleError::Other('
            )
        elif expected_type == 'PkiError':
            new_line = original_line.replace(
                'CursedError::runtime_error(',
                'PkiError::Other('
            )
        else:
            print(f"Unknown expected type: {expected_type}")
            return False
        
        # Also fix string literal issues - remove & for String parameters
        new_line = new_line.replace('(&"', '("')
        new_line = new_line.replace('".to_string()', '".to_string()')
        
        lines[line_num - 1] = new_line
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.writelines(lines)
        
        print(f"Fixed {file_path}:{line_num} - Changed CursedError to {expected_type}")
        return True
    
    return False

def ensure_error_imports(file_path, error_types):
    """Ensure necessary error type imports are present in the file"""
    if not os.path.exists(file_path):
        return False
        
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Check what imports are needed and missing
    needed_imports = []
    for error_type in error_types:
        if f'{error_type}' in content and f'use crate::stdlib::packages::{error_type}' not in content:
            needed_imports.append(error_type)
    
    if not needed_imports:
        return False
    
    lines = content.split('\n')
    
    # Find existing import line to modify
    for i, line in enumerate(lines):
        if 'use crate::stdlib::packages::' in line and '{' in line:
            # Modify existing import
            import_part = line.split('{')[1].split('}')[0]
            imports = [imp.strip() for imp in import_part.split(',')]
            
            for error_type in needed_imports:
                if error_type not in imports:
                    imports.append(error_type)
            
            new_import = f"use crate::stdlib::packages::{{{', '.join(sorted(imports))}}};"
            lines[i] = new_import
            
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write('\n'.join(lines))
            
            print(f"Updated imports in {file_path}: {needed_imports}")
            return True
    
    # Add new import if no existing one found
    insert_pos = 0
    for i, line in enumerate(lines):
        if line.strip().startswith('use '):
            insert_pos = i + 1
    
    import_line = f"use crate::stdlib::packages::{{{', '.join(sorted(needed_imports))}}};"
    lines.insert(insert_pos, import_line)
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(lines))
    
    print(f"Added imports to {file_path}: {needed_imports}")
    return True

def main():
    print("Finding error type mismatches...")
    
    # Get initial count
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    initial_errors = result.stderr.count('error[E0308]')
    print(f"Initial type mismatch errors: {initial_errors}")
    
    mismatches = get_error_type_mismatches()
    print(f"Found {len(mismatches)} CursedError -> specific error type mismatches")
    
    # Group by file to handle imports efficiently
    files_to_fix = {}
    for mismatch in mismatches:
        file_path = mismatch['file']
        if file_path not in files_to_fix:
            files_to_fix[file_path] = []
        files_to_fix[file_path].append(mismatch)
    
    print(f"Affecting {len(files_to_fix)} files")
    
    fixed_count = 0
    
    for file_path, file_mismatches in files_to_fix.items():
        print(f"\nFixing {file_path}...")
        
        # Ensure imports are present
        error_types = list(set(m['expected'] for m in file_mismatches))
        ensure_error_imports(file_path, error_types)
        
        # Fix each mismatch
        for mismatch in file_mismatches:
            if fix_error_in_file(mismatch['file'], mismatch['line'], mismatch['expected']):
                fixed_count += 1
    
    print(f"\nFixed {fixed_count} error type mismatches")
    
    # Check results
    print("\nRunning cargo check to verify fixes...")
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    
    final_errors = result.stderr.count('error[E0308]')
    print(f"Remaining type mismatch errors: {final_errors}")
    print(f"Reduced type mismatch errors by: {initial_errors - final_errors}")
    
    if final_errors == 0:
        print("✅ All type mismatch errors fixed!")
    else:
        print(f"❌ Still have {final_errors} type mismatch errors")

if __name__ == "__main__":
    main()
