#!/usr/bin/env python3

"""
Fix missing CryptoError imports in files that use CryptoError but don't import it.
"""

import os
import re
import subprocess

def get_files_with_crypto_error_missing():
    """Get files that have CryptoError usage but missing import."""
    result = subprocess.run(['cargo', 'check'], capture_output=True, text=True)
    error_output = result.stderr
    
    files_to_fix = set()
    current_file = None
    
    for line in error_output.split('\n'):
        # Match file references
        file_match = re.match(r'\s*--> (.+):\d+:\d+', line)
        if file_match:
            current_file = file_match.group(1)
        
        # Check for CryptoError undeclared errors
        if 'failed to resolve: use of undeclared type `CryptoError`' in line and current_file:
            files_to_fix.add(current_file)
    
    return list(files_to_fix)

def fix_crypto_error_imports(file_path):
    """Add CryptoError import to a file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if CryptoError import already exists
        if 'use crate::stdlib::packages::CryptoError' in content:
            return False
        
        # Check if file uses CryptoError
        if 'CryptoError::' not in content:
            return False
        
        lines = content.split('\n')
        
        # Find where to insert the import
        insert_pos = 0
        has_crypto_result = False
        
        for i, line in enumerate(lines):
            if line.strip().startswith('use crate::stdlib::packages::CryptoResult'):
                # Replace this line to include CryptoError
                lines[i] = line.replace(
                    'use crate::stdlib::packages::CryptoResult',
                    'use crate::stdlib::packages::{CryptoResult, CryptoError}'
                )
                has_crypto_result = True
                break
            elif line.strip().startswith('use crate::stdlib::packages::') and 'CryptoResult' in line:
                # Add CryptoError to existing import
                if not 'CryptoError' in line:
                    lines[i] = line.replace('{', '{CryptoError, ').replace('CryptoResult', 'CryptoResult')
                has_crypto_result = True
                break
            elif line.strip().startswith('use '):
                insert_pos = i + 1
        
        if not has_crypto_result:
            # Insert new import after other use statements
            lines.insert(insert_pos, 'use crate::stdlib::packages::CryptoError;')
        
        new_content = '\n'.join(lines)
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        
        print(f"Added CryptoError import to {file_path}")
        return True
        
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def main():
    """Main function."""
    print("🔧 Fixing missing CryptoError imports...")
    
    files_to_fix = get_files_with_crypto_error_missing()
    print(f"Found {len(files_to_fix)} files needing CryptoError import")
    
    fixed_count = 0
    for file_path in files_to_fix:
        if fix_crypto_error_imports(file_path):
            fixed_count += 1
    
    print(f"✅ Fixed imports in {fixed_count} files")
    print("Running final cargo check...")
    os.system("cargo check 2>&1 | head -20")

if __name__ == "__main__":
    main()
