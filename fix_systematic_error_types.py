#!/usr/bin/env python3
"""
Systematic fix for error type mismatches in CURSED compiler codebase.
Phase 2.25: Error type consistency work.
"""

import os
import re
import glob
import subprocess

def count_cargo_errors():
    """Count the current number of cargo check errors."""
    try:
        result = subprocess.run(['cargo', 'check'], capture_output=True, text=True, cwd='.')
        error_lines = [line for line in result.stderr.split('\n') if line.startswith('error[')]
        return len(error_lines)
    except:
        return -1

def fix_crypto_return_type_mismatches(file_path):
    """Fix CryptoError -> CursedError conversion mismatches."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        changes_made = 0
        
        # Pattern 1: Expected CursedError, found CryptoError - add .into()
        # Err(CryptoError::Other(...)) -> Err(CryptoError::Other(...).into())
        pattern1 = r'Err\(CryptoError::[^)]+\([^)]+\)\)'
        def replace_crypto_error(match):
            nonlocal changes_made
            changes_made += 1
            return match.group(0)[:-1] + '.into())'
        
        content = re.sub(pattern1, replace_crypto_error, content)
        
        # Pattern 2: Remove & from &format!() in error constructors
        # CryptoError::Other(&format!(...)) -> CryptoError::Other(format!(...))
        pattern2 = r'CryptoError::Other\(&format!\('
        if pattern2 in content:
            content = content.replace('CryptoError::Other(&format!(', 'CryptoError::Other(format!(')
            changes_made += 1
        
        # Pattern 3: IOError::Other(&format!(...)) -> IOError::Other(format!(...))
        pattern3 = r'IOError::Other\(&format!\('
        if pattern3 in content:
            content = content.replace('IOError::Other(&format!(', 'IOError::Other(format!(')
            changes_made += 1
        
        # Pattern 4: Fix string type mismatches in error messages
        pattern4 = r'&"([^"]+)"\.to_string\(\)'
        content = re.sub(pattern4, r'"\1".to_string()', content)
        if re.search(pattern4, original_content):
            changes_made += 1
        
        # Pattern 5: Functions returning wrong error types - add Ok() wrapper and ? operator
        # Fix read_string functions that expect CursedError but return IOError
        pattern5 = r'(String::from_utf8\(bytes\)\s*\.map_err\([^)]+\))'
        def fix_read_string(match):
            nonlocal changes_made
            changes_made += 1
            return f'Ok({match.group(1)}?)'
        
        if 'read_string' in content and 'IOResult<String>' in content:
            content = re.sub(pattern5, fix_read_string, content)
        
        # Pattern 6: Functions that need Ok() wrapper for conversion
        pattern6 = r'(self\.last_insert_id\s*\.ok_or_else\([^)]+\))'
        def fix_last_insert_id(match):
            nonlocal changes_made  
            changes_made += 1
            return f'Ok({match.group(1)}?)'
        
        if 'last_insert_id' in content and 'DatabaseResult<i64>' in content:
            content = re.sub(pattern6, fix_last_insert_id, content)
        
        # Pattern 7: fs operations that need Ok() wrapper
        pattern7 = r'(fs::(read_to_string|write)\([^)]+\)\s*\.map_err\([^)]+\))'
        def fix_fs_ops(match):
            nonlocal changes_made
            changes_made += 1
            return f'Ok({match.group(1)}?)'
        
        if ('TestResult<' in content or 'IOResult<' in content) and 'fs::' in content:
            content = re.sub(pattern7, fix_fs_ops, content)
        
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            return changes_made
        
        return 0
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return 0

def fix_specific_crypto_pki_files():
    """Fix the crypto_pki files that have CryptoError but expect CursedError."""
    crypto_pki_files = glob.glob('src/stdlib/packages/crypto_pki/*.rs')
    crypto_protocols_files = glob.glob('src/stdlib/packages/crypto_protocols/*.rs')
    
    changes = 0
    for file_path in crypto_pki_files + crypto_protocols_files:
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            original_content = content
            
            # These files return CryptoError but the function expects CursedError
            # Replace CursedError::runtime_error with CryptoError::Other and add .into()
            if 'CursedError::runtime_error' in content:
                if 'expected `CryptoError`' in subprocess.run(['cargo', 'check'], capture_output=True, text=True).stderr:
                    # Change back to CryptoError and add .into()
                    content = content.replace(
                        'CursedError::runtime_error(&"Crypto hash test failed".to_string())',
                        'CryptoError::Other("Crypto hash test failed".to_string()).into()'
                    )
                    content = content.replace(
                        'CursedError::runtime_error(&"I/O test failed".to_string())',
                        'IOError::Other("I/O test failed".to_string()).into()'
                    )
                    content = content.replace(
                        'CursedError::runtime_error(&"I/O string test failed".to_string())',
                        'IOError::Other("I/O string test failed".to_string()).into()'
                    )
                    changes += 1
            
            if content != original_content:
                with open(file_path, 'w') as f:
                    f.write(content)
                    
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    return changes

def main():
    print("=== CURSED Compiler Error Type Fixes - Phase 2.25 ===")
    
    # Count initial errors
    initial_errors = count_cargo_errors()
    print(f"Initial errors: {initial_errors}")
    
    # Get list of files to fix
    files_to_fix = []
    
    # Focus on stdlib files that have type mismatches
    files_to_fix.extend(glob.glob('src/stdlib/packages/**/*.rs', recursive=True))
    files_to_fix.extend(glob.glob('src/stdlib/database/**/*.rs', recursive=True))
    files_to_fix.extend(glob.glob('src/stdlib/**/*.rs', recursive=True))
    
    files_modified = 0
    total_changes = 0
    
    # Apply systematic fixes
    print("\n=== Applying systematic fixes ===")
    
    # Fix specific crypto_pki files first
    print("Fixing crypto_pki and crypto_protocols files...")
    pki_changes = fix_specific_crypto_pki_files()
    if pki_changes > 0:
        total_changes += pki_changes
        print(f"Applied {pki_changes} fixes to crypto_pki/crypto_protocols files")
    
    # Apply general fixes
    for file_path in files_to_fix:
        if os.path.isfile(file_path):
            changes = fix_crypto_return_type_mismatches(file_path)
            if changes > 0:
                files_modified += 1
                total_changes += changes
                print(f"Fixed {changes} issues in {file_path}")
    
    print(f"\n=== Summary ===")
    print(f"Files modified: {files_modified}")
    print(f"Total changes applied: {total_changes}")
    
    # Count final errors
    print("\nRechecking compilation...")
    final_errors = count_cargo_errors()
    print(f"Final errors: {final_errors}")
    
    if initial_errors > 0:
        reduction = initial_errors - final_errors
        print(f"Error reduction: {reduction} (from {initial_errors} to {final_errors})")
        print(f"Progress: {(reduction/initial_errors)*100:.1f}% reduction")
    
    return final_errors < initial_errors

if __name__ == "__main__":
    main()
