#!/usr/bin/env python3

import subprocess
import re
import os

def fix_redis_errors():
    """Fix the specific redis.rs error type issues"""
    file_path = "src/stdlib/packages/db_nosql/redis.rs"
    
    if not os.path.exists(file_path):
        return False
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Fix IOError returns in ModuleResult functions
    content = content.replace(
        'IOError::Other("Connection is closed".to_string())',
        'ModuleError::Other("Connection is closed".to_string())'
    )
    
    # Fix any other IOError in this context
    content = content.replace(
        'IOError::Other(',
        'ModuleError::Other('
    )
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"Fixed error types in {file_path}")
    return True

def fix_mongodb_errors():
    """Fix the specific mongodb.rs error type issues"""
    file_path = "src/stdlib/packages/db_nosql/mongodb.rs"
    
    if not os.path.exists(file_path):
        return False
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Fix IOError returns in ModuleResult functions
    content = content.replace(
        'IOError::Other("Connection is closed".to_string())',
        'ModuleError::Other("Connection is closed".to_string())'
    )
    
    # Fix any other IOError in this context
    content = content.replace(
        'IOError::Other(',
        'ModuleError::Other('
    )
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"Fixed error types in {file_path}")
    return True

def fix_function_signatures():
    """Fix function signature mismatches where the error type doesn't match the return type"""
    
    # List of files and their expected error types based on the module
    files_to_fix = {
        'src/stdlib/packages/db_nosql/redis.rs': 'ModuleError',
        'src/stdlib/packages/db_nosql/mongodb.rs': 'ModuleError',
        'src/stdlib/packages/db_sql/connection.rs': 'IOError',
        'src/stdlib/packages/db_sql/migration.rs': 'IOError',
    }
    
    fixed_count = 0
    
    for file_path, expected_error in files_to_fix.items():
        if not os.path.exists(file_path):
            continue
        
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix return error types to match function signatures
        if 'ModuleResult' in content and expected_error == 'ModuleError':
            # Functions returning ModuleResult should return ModuleError
            content = content.replace('IOError::Other(', 'ModuleError::Other(')
            content = content.replace('IOError::ReadFailed', 'ModuleError::ProcessingFailed')
            content = content.replace('IOError::WriteFailed', 'ModuleError::ProcessingFailed')
        elif 'IOResult' in content and expected_error == 'IOError':
            # Functions returning IOResult should return IOError - these are correct
            pass
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed function signatures in {file_path}")
            fixed_count += 1
    
    return fixed_count

def main():
    print("Fixing return type consistency...")
    
    # Get initial count
    result = subprocess.run(["cargo", "check"], capture_output=True, text=True)
    initial_errors = result.stderr.count('error[E0308]')
    print(f"Initial type mismatch errors: {initial_errors}")
    
    # Fix specific errors
    fixed_count = 0
    
    if fix_redis_errors():
        fixed_count += 1
    
    if fix_mongodb_errors():
        fixed_count += 1
    
    fixed_count += fix_function_signatures()
    
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
        for line in lines:
            if 'expected `' in line and 'found `' in line:
                print(f"  {line.strip()}")
                break

if __name__ == "__main__":
    main()
