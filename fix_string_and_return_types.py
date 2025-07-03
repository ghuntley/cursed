#!/usr/bin/env python3

import os
import re
import sys

def fix_string_literal_errors():
    """Fix &str to String conversion issues in error constructors"""
    
    patterns_to_fix = [
        # IOError patterns
        (r'IOError::Other\("([^"]+)"\)', r'IOError::Other("\1".to_string())'),
        (r'IOError::ReadError\("([^"]+)"\)', r'IOError::ReadError("\1".to_string())'),
        (r'IOError::WriteFailed\("([^"]+)"\)', r'IOError::WriteFailed("\1".to_string())'),
        
        # CryptoError patterns  
        (r'CryptoError::Other\("([^"]+)"\)', r'CryptoError::Other("\1".to_string())'),
        (r'CryptoError::InvalidInput\("([^"]+)"\)', r'CryptoError::InvalidInput("\1".to_string())'),
        (r'CryptoError::EncryptionFailed\("([^"]+)"\)', r'CryptoError::EncryptionFailed("\1".to_string())'),
        (r'CryptoError::DecryptionFailed\("([^"]+)"\)', r'CryptoError::DecryptionFailed("\1".to_string())'),
        (r'CryptoError::KeyGenerationFailed\("([^"]+)"\)', r'CryptoError::KeyGenerationFailed("\1".to_string())'),
        (r'CryptoError::SignatureFailed\("([^"]+)"\)', r'CryptoError::SignatureFailed("\1".to_string())'),
        (r'CryptoError::VerificationFailed\("([^"]+)"\)', r'CryptoError::VerificationFailed("\1".to_string())'),
        
        # ModuleError patterns
        (r'ModuleError::Other\("([^"]+)"\)', r'ModuleError::Other("\1".to_string())'),
        (r'ModuleError::InitializationFailed\("([^"]+)"\)', r'ModuleError::InitializationFailed("\1".to_string())'),
        (r'ModuleError::ProcessingFailed\("([^"]+)"\)', r'ModuleError::ProcessingFailed("\1".to_string())'),
    ]
    
    files_to_fix = []
    
    # Find all .rs files in problematic directories
    for root_dir in ['src/stdlib/collections', 'src/stdlib/database', 'src/stdlib/packages']:
        for root, dirs, files in os.walk(root_dir):
            for file in files:
                if file.endswith('.rs'):
                    files_to_fix.append(os.path.join(root, file))
    
    total_fixes = 0
    files_modified = 0
    
    for file_path in files_to_fix:
        if not os.path.exists(file_path):
            continue
            
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            file_fixes = 0
            
            # Apply all string conversion patterns
            for pattern, replacement in patterns_to_fix:
                matches = re.findall(pattern, content)
                if matches:
                    content = re.sub(pattern, replacement, content)
                    file_fixes += len(matches)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                files_modified += 1
                total_fixes += file_fixes
                print(f"Fixed {file_fixes} string literals in {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    return files_modified, total_fixes

def fix_return_type_mismatches():
    """Fix return type mismatches where functions return wrong error types"""
    
    # Define patterns to convert specific errors to CursedError
    conversion_patterns = [
        # Convert IOError to CursedError
        (r'return Err\(IOError::([^)]+)\)', r'return Err(CursedError::runtime_error(&format!("IO Error: {:?}", IOError::\1)))'),
        
        # Convert ModuleError to CursedError
        (r'return Err\(ModuleError::([^)]+)\)', r'return Err(CursedError::runtime_error(&format!("Module Error: {:?}", ModuleError::\1)))'),
        
        # Convert CryptoError to CursedError
        (r'return Err\(CryptoError::([^)]+)\)', r'return Err(CursedError::runtime_error(&format!("Crypto Error: {:?}", CryptoError::\1)))'),
    ]
    
    # Alternative: Direct error conversion patterns
    simple_conversions = [
        # For simpler cases where we can directly convert
        (r'Err\(IOError::Other\(([^)]+)\)\)', r'Err(CursedError::runtime_error(&\1))'),
        (r'Err\(IOError::ReadError\(([^)]+)\)\)', r'Err(CursedError::runtime_error(&\1))'),
        (r'Err\(ModuleError::Other\(([^)]+)\)\)', r'Err(CursedError::runtime_error(&\1))'),
        (r'Err\(CryptoError::Other\(([^)]+)\)\)', r'Err(CursedError::runtime_error(&\1))'),
    ]
    
    files_to_fix = []
    
    # Target files with return type issues
    problematic_dirs = [
        'src/stdlib/collections', 
        'src/stdlib/database',
        'src/stdlib/packages'
    ]
    
    for root_dir in problematic_dirs:
        for root, dirs, files in os.walk(root_dir):
            for file in files:
                if file.endswith('.rs'):
                    files_to_fix.append(os.path.join(root, file))
    
    total_fixes = 0
    files_modified = 0
    
    for file_path in files_to_fix:
        if not os.path.exists(file_path):
            continue
            
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            file_fixes = 0
            
            # Apply simple conversion patterns first
            for pattern, replacement in simple_conversions:
                matches = re.findall(pattern, content)
                if matches:
                    content = re.sub(pattern, replacement, content)
                    file_fixes += len(matches)
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                files_modified += 1
                total_fixes += file_fixes
                print(f"Fixed {file_fixes} return type issues in {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    return files_modified, total_fixes

def main():
    print("Starting comprehensive error handling fixes...")
    
    # Fix string literal conversions
    print("\n=== Fixing String Literal Conversions ===")
    str_files, str_fixes = fix_string_literal_errors()
    print(f"String conversion fixes: {str_fixes} fixes in {str_files} files")
    
    # Fix return type mismatches
    print("\n=== Fixing Return Type Mismatches ===")
    ret_files, ret_fixes = fix_return_type_mismatches()
    print(f"Return type fixes: {ret_fixes} fixes in {ret_files} files")
    
    print(f"\n=== SUMMARY ===")
    print(f"Total files modified: {str_files + ret_files}")
    print(f"Total fixes applied: {str_fixes + ret_fixes}")
    print(f"String literal fixes: {str_fixes}")
    print(f"Return type fixes: {ret_fixes}")

if __name__ == "__main__":
    main()
