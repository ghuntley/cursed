#!/usr/bin/env python3

import os
import re
import subprocess
from pathlib import Path

def get_files_with_error_issues():
    """Get files that have 'cannot find type Error' issues."""
    try:
        result = subprocess.run(
            ["cargo", "build"], 
            capture_output=True, 
            text=True, 
            cwd="/home/ghuntley/code/cursed"
        )
        
        files_with_errors = set()
        lines = result.stderr.split('\n')
        
        for line in lines:
            if 'cannot find type' in line and 'Error' in line:
                # Extract filename from error line
                if '-->' in line:
                    file_path = line.split('-->')[1].strip().split(':')[0].strip()
                    if file_path.startswith('src/'):
                        files_with_errors.add(file_path)
        
        return list(files_with_errors)
    except:
        return []

def fix_error_imports_in_file(filepath):
    """Fix Error import issues in a specific file."""
    if not os.path.exists(filepath):
        return False
        
    with open(filepath, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Check if file already has proper error import
    has_error_types_import = 'use crate::error_types::' in content or 'use crate::{' in content and 'Error' in content
    has_error_import = 'use crate::error::' in content
    
    if not has_error_types_import and not has_error_import:
        # Add the import at the top after other use statements
        lines = content.split('\n')
        insert_pos = 0
        
        # Find the position to insert import (after existing use statements)
        for i, line in enumerate(lines):
            if line.strip().startswith('use '):
                insert_pos = i + 1
            elif line.strip() and not line.strip().startswith('//') and not line.strip().startswith('use '):
                break
        
        # Insert the import
        lines.insert(insert_pos, 'use crate::error_types::Error;')
        content = '\n'.join(lines)
    
    # Fix incorrect imports
    content = re.sub(r'use crate::error::\{Error as CursedError[^}]*\}', 'use crate::error_types::Error', content)
    content = re.sub(r'use crate::error::Error as CursedError', 'use crate::error_types::Error', content)
    content = re.sub(r'use crate::error::Error', 'use crate::error_types::Error', content)
    
    # Fix CursedError references to Error
    content = re.sub(r'\bCursedError\b', 'Error', content)
    
    # Save if changed
    if content != original_content:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    
    return False

def main():
    print("🔍 Finding files with Error type issues...")
    
    # Get files with error issues
    files_with_errors = get_files_with_error_issues()
    
    if not files_with_errors:
        print("No files found with Error type issues. Checking common patterns...")
        # Fall back to searching for common error patterns
        src_path = Path("/home/ghuntley/code/cursed/src")
        files_with_errors = []
        
        for rust_file in src_path.rglob("*.rs"):
            try:
                with open(rust_file, 'r') as f:
                    content = f.read()
                    if ('Result<' in content or 'Error' in content) and 'use crate::error_types::Error' not in content:
                        files_with_errors.append(str(rust_file))
            except:
                continue
    
    print(f"📁 Found {len(files_with_errors)} files to process")
    
    fixed_count = 0
    for filepath in files_with_errors[:50]:  # Process first 50 files
        try:
            if fix_error_imports_in_file(filepath):
                print(f"✅ Fixed: {filepath}")
                fixed_count += 1
            else:
                print(f"⏭️  Skipped: {filepath}")
        except Exception as e:
            print(f"❌ Error fixing {filepath}: {e}")
    
    print(f"\n🎉 Fixed {fixed_count} files")
    
    # Check remaining errors
    print("\n🔍 Checking remaining Error type issues...")
    try:
        result = subprocess.run(
            ["cargo", "build", "--message-format=short"], 
            capture_output=True, 
            text=True, 
            cwd="/home/ghuntley/code/cursed"
        )
        
        error_lines = [line for line in result.stderr.split('\n') if 'cannot find type' in line and 'Error' in line]
        print(f"📊 Remaining Error type issues: {len(error_lines)}")
        
        if error_lines:
            print("🔍 Sample remaining errors:")
            for line in error_lines[:10]:
                print(f"   {line}")
                
    except Exception as e:
        print(f"❌ Error checking build status: {e}")

if __name__ == "__main__":
    main()
