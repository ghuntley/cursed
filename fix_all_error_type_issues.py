#!/usr/bin/env python3
"""
Comprehensive fix for all Error type issues in CURSED codebase.
"""

import os
import re
import subprocess

def get_error_files():
    """Get list of files with Error type issues from cargo check."""
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], 
                              capture_output=True, text=True, cwd='.')
        
        # Extract file paths with Error issues
        files = set()
        for line in result.stderr.split('\n'):
            if 'cannot find type' in line and 'Error' in line:
                match = re.search(r'src/[^:]+\.rs', line)
                if match:
                    files.add(match.group(0))
        
        return list(files)
    except Exception as e:
        print(f"Error getting files: {e}")
        return []

def fix_error_imports_in_file(file_path):
    """Add Error import and fix Error type usage in a file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changed = False
        
        # Add Error import if not present and file uses Error
        if 'use crate::error::Error' not in content and re.search(r'\bError\b', content):
            lines = content.split('\n')
            
            # Find where to insert the import
            insert_index = 0
            for i, line in enumerate(lines):
                if line.strip().startswith('use '):
                    insert_index = i + 1
                elif line.strip() and not line.strip().startswith('//') and not line.strip().startswith('use '):
                    break
            
            lines.insert(insert_index, 'use crate::error::Error;')
            content = '\n'.join(lines)
            changed = True
        
        # Fix common Error type issues
        replacements = [
            (r'Result<([^>]+),\s*IoError>', r'Result<\1, Error>'),
            (r'std::result::Result<([^>]+),\s*IoError>', r'std::result::Result<\1, Error>'),
            (r'Result<([^>]+),\s*CryptoError>', r'Result<\1, Error>'),
            (r'Result<([^>]+),\s*ParsingError>', r'Result<\1, Error>'),
            (r'Result<([^>]+),\s*SystemError>', r'Result<\1, Error>'),
            (r'Result<([^>]+),\s*DatabaseError>', r'Result<\1, Error>'),
            (r'Result<([^>]+),\s*NetworkError>', r'Result<\1, Error>'),
        ]
        
        for pattern, replacement in replacements:
            new_content = re.sub(pattern, replacement, content)
            if new_content != content:
                content = new_content
                changed = True
        
        # Save if changed
        if changed:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Fixed {file_path}")
            return True
        
        return False
        
    except Exception as e:
        print(f"❌ Error processing {file_path}: {e}")
        return False

def main():
    """Main function to fix all Error type issues."""
    print("🔍 Finding files with Error type issues...")
    
    error_files = get_error_files()
    print(f"Found {len(error_files)} files with Error issues")
    
    if not error_files:
        print("✨ No Error type issues found!")
        return
    
    fixed_count = 0
    for file_path in error_files:
        if os.path.exists(file_path):
            if fix_error_imports_in_file(file_path):
                fixed_count += 1
    
    print(f"\n🎉 Fixed Error types in {fixed_count} files")
    
    # Check results
    print("\n🔍 Checking remaining Error issues...")
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], 
                              capture_output=True, text=True, cwd='.')
        error_count = result.stderr.count('cannot find type') and result.stderr.count('Error')
        print(f"Remaining Error type issues: {error_count}")
    except:
        print("Could not check remaining issues")

if __name__ == '__main__':
    main()
