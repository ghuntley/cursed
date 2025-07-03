#!/usr/bin/env python3

import subprocess
import re
import os

def get_all_files_with_crypto_errors():
    """Get all files that have CryptoError import errors by parsing cargo output"""
    result = subprocess.run(
        ["cargo", "check"], 
        capture_output=True, 
        text=True, 
        cwd="."
    )
    
    files_needing_fixes = set()
    
    # Parse cargo check output line by line
    lines = result.stderr.split('\n')
    for line in lines:
        if '-->' in line and '.rs:' in line and 'src/' in line:
            # Extract file path from lines like "   --> src/path/file.rs:123:45"
            file_path = line.split('-->')[1].strip().split(':')[0].strip()
            if file_path.startswith('src/') and file_path.endswith('.rs'):
                # Check if this is related to CryptoError by looking at next few lines
                files_needing_fixes.add(file_path)
    
    # Now filter to only files that actually have CryptoError issues
    crypto_error_files = set()
    for line in lines:
        if 'use of undeclared type `CryptoError`' in line:
            # Look backwards for the file path
            for prev_line in reversed(lines[:lines.index(line)]):
                if '-->' in prev_line and '.rs:' in prev_line and 'src/' in prev_line:
                    file_path = prev_line.split('-->')[1].strip().split(':')[0].strip()
                    if file_path.startswith('src/'):
                        crypto_error_files.add(file_path)
                    break
    
    return list(crypto_error_files)

def fix_crypto_import_in_file(file_path):
    """Add CryptoError import to a file if it doesn't already have it"""
    if not os.path.exists(file_path):
        print(f"File not found: {file_path}")
        return False
        
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Check if CryptoError import already exists
    if ('use crate::stdlib::packages::CryptoError' in content or 
        'use super::CryptoError' in content or
        'CryptoError,' in content):
        return False
    
    lines = content.split('\n')
    
    # Find existing stdlib packages import line to modify it
    for i, line in enumerate(lines):
        stripped = line.strip()
        if 'use crate::stdlib::packages::' in stripped and 'CryptoResult' in stripped:
            # Modify this line to include CryptoError
            if '{' in stripped and '}' in stripped:
                # Multi-import line like: use crate::stdlib::packages::{CryptoResult, CryptoHandler};
                import_part = stripped.split('{')[1].split('}')[0]
                imports = [imp.strip() for imp in import_part.split(',')]
                if 'CryptoError' not in imports:
                    imports.append('CryptoError')
                    new_import = f"use crate::stdlib::packages::{{{', '.join(imports)}}};"
                    lines[i] = new_import
                    print(f"Modified existing import in {file_path}")
                    
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write('\n'.join(lines))
                    return True
            else:
                # Single import line, add a new one
                lines.insert(i + 1, "use crate::stdlib::packages::CryptoError;")
                print(f"Added CryptoError import after existing import in {file_path}")
                
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write('\n'.join(lines))
                return True
    
    # If no existing stdlib packages import, add new import at appropriate position
    insert_position = 0
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith('use '):
            insert_position = i + 1
        elif stripped == '' or not stripped.startswith('use '):
            break
    
    # Insert the import
    lines.insert(insert_position, "use crate::stdlib::packages::CryptoError;")
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(lines))
    
    print(f"Added CryptoError import to {file_path}")
    return True

def main():
    print("Finding files with CryptoError import errors...")
    
    # Get initial count of CryptoError errors
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    initial_errors = result.stderr.count('use of undeclared type `CryptoError`')
    print(f"Initial CryptoError import errors: {initial_errors}")
    
    files_to_fix = get_all_files_with_crypto_errors()
    
    print(f"Found {len(files_to_fix)} files potentially needing CryptoError imports")
    
    print("\nFixing imports...")
    fixed_count = 0
    
    for file_path in sorted(files_to_fix):
        if fix_crypto_import_in_file(file_path):
            fixed_count += 1
    
    print(f"\nAttempted fixes on {fixed_count} files")
    
    # Run cargo check again to see results
    print("\nRunning cargo check to verify fixes...")
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    
    # Count remaining CryptoError errors
    remaining_crypto_errors = result.stderr.count('use of undeclared type `CryptoError`')
    print(f"Remaining CryptoError import errors: {remaining_crypto_errors}")
    print(f"Reduced CryptoError errors by: {initial_errors - remaining_crypto_errors}")
    
    if remaining_crypto_errors == 0:
        print("✅ All CryptoError import errors fixed!")
    else:
        print(f"❌ Still have {remaining_crypto_errors} CryptoError import errors to fix")
        
        # Show a few remaining errors for debugging
        print("\nSample remaining errors:")
        error_lines = [line for line in result.stderr.split('\n') if 'CryptoError' in line and 'undeclared' in line][:3]
        for line in error_lines:
            print(f"  {line}")

if __name__ == "__main__":
    main()
