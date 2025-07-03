#!/usr/bin/env python3

"""
Comprehensive script to fix systematic error type mismatches in CURSED compiler.
Replaces CursedError::runtime_error() calls with appropriate specific error types.
"""

import os
import re
import glob
from typing import List, Tuple, Dict

def find_files_with_pattern(root_dir: str, pattern: str) -> List[str]:
    """Find all files containing the pattern."""
    result = []
    for root, dirs, files in os.walk(root_dir):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        content = f.read()
                        if pattern in content:
                            result.append(file_path)
                except Exception as e:
                    print(f"Error reading {file_path}: {e}")
    return result

def determine_error_type(file_path: str, content: str) -> str:
    """Determine the appropriate error type based on file path and content."""
    
    # Check return type from function signatures
    if 'CryptoResult<' in content:
        return 'CryptoError'
    elif 'IOResult<' in content:
        return 'IOError'
    elif 'ModuleResult<' in content:
        return 'ModuleError'
    elif 'PkiResult<' in content:
        return 'PkiError'
    
    # Check import statements
    if 'use crate::stdlib::packages::CryptoResult' in content:
        return 'CryptoError'
    elif 'use crate::stdlib::packages::IOResult' in content:
        return 'IOError'
    elif 'use crate::stdlib::packages::ModuleResult' in content:
        return 'ModuleError'
    elif 'use crate::stdlib::packages::PkiResult' in content:
        return 'PkiError'
    
    # Path-based detection
    if any(crypto_path in file_path for crypto_path in [
        'crypto_', 'crypto/', '/crypto', 'pqc', 'signatures', 'asymmetric'
    ]):
        return 'CryptoError'
    elif any(io_path in file_path for io_path in [
        'io/', '/io', 'database/', 'orm/', 'sql_', 'db_'
    ]):
        return 'IOError'
    elif 'collections/' in file_path:
        return 'IOError'  # Collections often involve I/O operations
    
    # Default fallback
    return 'CryptoError'

def get_error_constructor(error_type: str, message: str) -> str:
    """Get the appropriate error constructor."""
    if error_type == 'CryptoError':
        # Try to match to specific crypto error types
        msg_lower = message.lower()
        if 'key generation' in msg_lower or 'key' in msg_lower:
            return 'CryptoError::KeyGenerationFailed'
        elif 'encrypt' in msg_lower:
            return 'CryptoError::EncryptionFailed'
        elif 'decrypt' in msg_lower:
            return 'CryptoError::DecryptionFailed'
        elif 'signature' in msg_lower or 'sign' in msg_lower:
            return 'CryptoError::SignatureFailed'
        elif 'verif' in msg_lower:
            return 'CryptoError::VerificationFailed'
        elif 'input' in msg_lower:
            return 'CryptoError::InvalidInput'
        else:
            return f'CryptoError::Other({message})'
    elif error_type == 'IOError':
        msg_lower = message.lower()
        if 'read' in msg_lower:
            return f'IOError::ReadError({message})'
        elif 'write' in msg_lower:
            return 'IOError::WriteFailed'
        elif 'file not found' in msg_lower:
            return 'IOError::FileNotFound'
        elif 'permission' in msg_lower:
            return 'IOError::PermissionDenied'
        elif 'input' in msg_lower:
            return 'IOError::InvalidInput'
        else:
            return f'IOError::Other({message})'
    elif error_type == 'ModuleError':
        msg_lower = message.lower()
        if 'initialization' in msg_lower or 'init' in msg_lower:
            return 'ModuleError::InitializationFailed'
        elif 'process' in msg_lower:
            return 'ModuleError::ProcessingFailed'
        elif 'config' in msg_lower:
            return 'ModuleError::InvalidConfiguration'
        else:
            return f'ModuleError::Other({message})'
    elif error_type == 'PkiError':
        msg_lower = message.lower()
        if 'certificate' in msg_lower:
            return 'PkiError::CertificateInvalid'
        elif 'key' in msg_lower:
            return 'PkiError::KeyInvalid'
        elif 'sign' in msg_lower:
            return 'PkiError::SigningFailed'
        elif 'valid' in msg_lower:
            return 'PkiError::ValidationFailed'
        else:
            return f'PkiError::Other({message})'
    
    return f'{error_type}::Other({message})'

def fix_error_patterns(content: str, file_path: str) -> Tuple[str, int]:
    """Fix all error patterns in content."""
    fixes_made = 0
    
    # Determine the appropriate error type for this file
    error_type = determine_error_type(file_path, content)
    
    # Pattern 1: CursedError::runtime_error("message")
    pattern1 = r'CursedError::runtime_error\("([^"]+)"\)'
    def replace1(match):
        nonlocal fixes_made
        fixes_made += 1
        message = f'"{match.group(1)}"'
        return get_error_constructor(error_type, message)
    
    content = re.sub(pattern1, replace1, content)
    
    # Pattern 2: CursedError::runtime_error(&format!(...))
    pattern2 = r'CursedError::runtime_error\(&format!\("([^"]+)"[^)]*\)\)'
    def replace2(match):
        nonlocal fixes_made
        fixes_made += 1
        # For format! cases, we'll use the Other variant
        return f'{error_type}::Other(format!("{match.group(1)}"))'
    
    content = re.sub(pattern2, replace2, content)
    
    # Pattern 3: CursedError::runtime_error(&format!("Read error: {}", e))
    pattern3 = r'CursedError::runtime_error\(&format!\("Read error: \{\}", e\)\)'
    def replace3(match):
        nonlocal fixes_made
        fixes_made += 1
        if error_type == 'IOError':
            return 'IOError::ReadError(format!("Read error: {}", e))'
        else:
            return f'{error_type}::Other(format!("Read error: {{}}", e))'
    
    content = re.sub(pattern3, replace3, content)
    
    # Pattern 4: CursedError::runtime_error(&format!("Write error: {}", e))
    pattern4 = r'CursedError::runtime_error\(&format!\("Write error: \{\}", e\)\)'
    def replace4(match):
        nonlocal fixes_made
        fixes_made += 1
        if error_type == 'IOError':
            return 'IOError::Other(format!("Write error: {}", e))'
        else:
            return f'{error_type}::Other(format!("Write error: {{}}", e))'
    
    content = re.sub(pattern4, replace4, content)
    
    # Pattern 5: CursedError::runtime_error(&format!("UTF-8 decode error: {}", e))
    pattern5 = r'CursedError::runtime_error\(&format!\("UTF-8 decode error: \{\}", e\)\)'
    def replace5(match):
        nonlocal fixes_made
        fixes_made += 1
        if error_type == 'IOError':
            return 'IOError::Other(format!("UTF-8 decode error: {}", e))'
        else:
            return f'{error_type}::Other(format!("UTF-8 decode error: {{}}", e))'
    
    content = re.sub(pattern5, replace5, content)
    
    # Pattern 6: CursedError::runtime_error(&format!("Hex decode error: {}", e))
    pattern6 = r'CursedError::runtime_error\(&format!\("Hex decode error: \{\}", e\)\)'
    def replace6(match):
        nonlocal fixes_made
        fixes_made += 1
        return f'{error_type}::Other(format!("Hex decode error: {{}}", e))'
    
    content = re.sub(pattern6, replace6, content)
    
    # Pattern 7: Generic CursedError::runtime_error with variables
    pattern7 = r'CursedError::runtime_error\(([^)]+)\)'
    def replace7(match):
        nonlocal fixes_made
        arg = match.group(1)
        if '"' in arg and 'format!' not in arg:
            fixes_made += 1
            return get_error_constructor(error_type, arg)
        elif 'format!' in arg:
            fixes_made += 1
            return f'{error_type}::Other({arg})'
        return match.group(0)  # Don't change if we can't parse
    
    content = re.sub(pattern7, replace7, content)
    
    return content, fixes_made

def add_required_imports(content: str, error_types_used: set) -> str:
    """Add required imports for error types."""
    import_lines = []
    
    if 'CryptoError' in error_types_used:
        if 'use crate::stdlib::packages::CryptoError' not in content:
            import_lines.append('use crate::stdlib::packages::CryptoError;')
    
    if 'IOError' in error_types_used:
        if 'use crate::stdlib::packages::IOError' not in content:
            import_lines.append('use crate::stdlib::packages::IOError;')
    
    if 'ModuleError' in error_types_used:
        if 'use crate::stdlib::packages::ModuleError' not in content:
            import_lines.append('use crate::stdlib::packages::ModuleError;')
    
    if 'PkiError' in error_types_used:
        if 'use crate::stdlib::packages::PkiError' not in content:
            import_lines.append('use crate::stdlib::packages::PkiError;')
    
    if import_lines:
        # Find the last import line and add after it
        lines = content.split('\n')
        insert_pos = 0
        for i, line in enumerate(lines):
            if line.strip().startswith('use '):
                insert_pos = i + 1
        
        for import_line in reversed(import_lines):
            lines.insert(insert_pos, import_line)
        
        return '\n'.join(lines)
    
    return content

def process_file(file_path: str) -> Tuple[int, bool]:
    """Process a single file and return number of fixes made."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            original_content = f.read()
        
        if 'CursedError::runtime_error' not in original_content:
            return 0, False
        
        # Apply fixes
        fixed_content, fixes_made = fix_error_patterns(original_content, file_path)
        
        if fixes_made == 0:
            return 0, False
        
        # Determine which error types were used
        error_types_used = set()
        for error_type in ['CryptoError', 'IOError', 'ModuleError', 'PkiError']:
            if f'{error_type}::' in fixed_content:
                error_types_used.add(error_type)
        
        # Add required imports
        final_content = add_required_imports(fixed_content, error_types_used)
        
        # Write back to file
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(final_content)
        
        print(f"Fixed {fixes_made} errors in {file_path}")
        return fixes_made, True
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return 0, False

def main():
    """Main function to fix all error type mismatches."""
    print("🔧 Starting comprehensive error type fixes...")
    
    # Find all problematic files
    root_dirs = [
        'src/stdlib/packages',
        'src/stdlib/collections',
        'src/stdlib/crypto',
        'src/stdlib/async',
        'src/stdlib/database'
    ]
    
    all_files = []
    for root_dir in root_dirs:
        if os.path.exists(root_dir):
            files = find_files_with_pattern(root_dir, 'CursedError::runtime_error')
            all_files.extend(files)
    
    print(f"Found {len(all_files)} files with error type issues")
    
    total_fixes = 0
    files_modified = 0
    
    for file_path in all_files:
        fixes, modified = process_file(file_path)
        total_fixes += fixes
        if modified:
            files_modified += 1
    
    print(f"\n✅ Comprehensive fix complete!")
    print(f"📊 Files modified: {files_modified}")
    print(f"🔧 Total fixes applied: {total_fixes}")
    print(f"\nRunning cargo check to verify fixes...")
    
    # Run cargo check to verify
    os.system("cargo check 2>&1 | head -20")

if __name__ == "__main__":
    main()
