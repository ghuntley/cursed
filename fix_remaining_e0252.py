#!/usr/bin/env python3
"""
Fix remaining E0252 errors - specific pattern detection and fix
"""

import re
import subprocess
from pathlib import Path

def fix_specific_pattern(file_path: Path) -> bool:
    """Fix the specific pattern: use crate::{Error, ...} + use crate::error::Error"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except (UnicodeDecodeError, IOError):
        return False
    
    lines = content.split('\n')
    modified = False
    
    for i, line in enumerate(lines):
        # Look for the redundant import pattern
        if line.strip() == 'use crate::error::Error;':
            # Check if there's already a crate::{Error, ...} import
            for j, other_line in enumerate(lines):
                if re.search(r'use crate::\{.*Error.*\}', other_line):
                    # Remove the redundant single import
                    lines[i] = ''
                    modified = True
                    print(f"Removed redundant import from {file_path}")
                    break
    
    if modified:
        # Clean up empty lines
        cleaned_lines = []
        for line in lines:
            if line.strip() == '' and len(cleaned_lines) > 0 and cleaned_lines[-1].strip() == '':
                continue  # Skip consecutive empty lines
            cleaned_lines.append(line)
        
        new_content = '\n'.join(cleaned_lines)
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            return True
        except IOError:
            return False
    
    return False

def get_e0252_files() -> list:
    """Get list of files with E0252 errors"""
    try:
        result = subprocess.run(
            ['cargo', 'check', '--message-format=short'],
            capture_output=True,
            text=True,
            timeout=300
        )
        
        files_with_errors = []
        lines = result.stderr.split('\n')
        
        for line in lines:
            if 'E0252' in line and 'Error' in line:
                # Extract file path from error message
                match = re.search(r'--> ([^:]+):', line)
                if match:
                    file_path = Path(match.group(1))
                    if file_path not in files_with_errors:
                        files_with_errors.append(file_path)
        
        return files_with_errors
    except:
        return []

def main():
    print("🎯 Fixing remaining E0252 errors...")
    
    # Get files with E0252 errors
    error_files = get_e0252_files()
    print(f"Found {len(error_files)} files with E0252 errors")
    
    fixed_count = 0
    for file_path in error_files:
        if fix_specific_pattern(file_path):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files")
    
    # Validate
    try:
        result = subprocess.run(
            ['cargo', 'check', '--message-format=short'],
            capture_output=True,
            text=True,
            timeout=300
        )
        remaining_e0252 = result.stderr.count('E0252')
        print(f"Remaining E0252 errors: {remaining_e0252}")
        
        if remaining_e0252 == 0:
            print("🎉 All E0252 errors fixed!")
        else:
            print(f"⚠️ {remaining_e0252} E0252 errors still remain")
            
    except Exception as e:
        print(f"Error validating: {e}")

if __name__ == "__main__":
    main()
