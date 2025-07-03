#!/usr/bin/env python3

import subprocess
import re
import os

def get_files_with_crypto_errors():
    """Get all files that have CryptoError import errors"""
    result = subprocess.run(
        ["cargo", "check"], 
        capture_output=True, 
        text=True, 
        cwd="."
    )
    
    files_needing_fixes = set()
    
    # Parse cargo check output to find files with CryptoError errors
    lines = result.stderr.split('\n')
    for i, line in enumerate(lines):
        if 'use of undeclared type `CryptoError`' in line:
            # Look for the file path in previous lines
            for j in range(i-5, i):
                if j >= 0 and '-->' in lines[j]:
                    file_path = lines[j].split('-->')[1].strip().split(':')[0]
                    if file_path.startswith('src/'):
                        files_needing_fixes.add(file_path)
                    break
    
    return list(files_needing_fixes)

def fix_crypto_import_in_file(file_path):
    """Add CryptoError import to a file if it doesn't already have it"""
    if not os.path.exists(file_path):
        print(f"File not found: {file_path}")
        return False
        
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Check if CryptoError import already exists
    if 'use crate::stdlib::packages::CryptoError' in content or 'use super::CryptoError' in content:
        print(f"File {file_path} already has CryptoError import")
        return False
    
    lines = content.split('\n')
    
    # Find the position to insert the import (after existing use statements)
    insert_position = 0
    in_use_block = False
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith('use '):
            in_use_block = True
            insert_position = i + 1
        elif in_use_block and not stripped.startswith('use ') and stripped != '':
            break
    
    # Insert the import
    import_line = "use crate::stdlib::packages::CryptoError;"
    lines.insert(insert_position, import_line)
    
    # Write back to file
    with open(file_path, 'w') as f:
        f.write('\n'.join(lines))
    
    print(f"Added CryptoError import to {file_path}")
    return True

def main():
    print("Finding files with CryptoError import errors...")
    files_to_fix = get_files_with_crypto_errors()
    
    print(f"Found {len(files_to_fix)} files needing CryptoError imports:")
    for file_path in sorted(files_to_fix):
        print(f"  {file_path}")
    
    print("\nFixing imports...")
    fixed_count = 0
    
    for file_path in files_to_fix:
        if fix_crypto_import_in_file(file_path):
            fixed_count += 1
    
    print(f"\nFixed {fixed_count} files")
    
    # Run cargo check again to see results
    print("\nRunning cargo check to verify fixes...")
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    
    # Count remaining CryptoError errors
    remaining_crypto_errors = result.stderr.count('use of undeclared type `CryptoError`')
    print(f"Remaining CryptoError import errors: {remaining_crypto_errors}")
    
    if remaining_crypto_errors == 0:
        print("✅ All CryptoError import errors fixed!")
    else:
        print(f"❌ Still have {remaining_crypto_errors} CryptoError import errors to fix")

if __name__ == "__main__":
    main()
