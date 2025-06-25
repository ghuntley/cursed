#!/usr/bin/env python3

import os
import re
import subprocess
from pathlib import Path

def fix_file_comprehensively(filepath):
    """Comprehensively fix Error import and usage issues in a file."""
    if not os.path.exists(filepath):
        return False
        
    with open(filepath, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Skip if file is already properly configured
    if 'use crate::error_types::Error' in content:
        return False
    
    # Fix common error import patterns
    content = re.sub(r'use crate::error::\{[^}]*Error[^}]*\}[^;]*;', 'use crate::error_types::Error;', content)
    content = re.sub(r'use crate::error::Error[^;]*;', 'use crate::error_types::Error;', content)
    content = re.sub(r'use crate::error::\{[^}]*CursedError[^}]*\}[^;]*;', 'use crate::error_types::Error;', content)
    
    # Add import if not present and file uses Error types
    has_error_types_import = 'use crate::error_types::Error' in content
    uses_error = re.search(r'\bError\b', content) or re.search(r'Result<[^,>]*>', content)
    
    if not has_error_types_import and uses_error:
        # Find position to insert import
        lines = content.split('\n')
        insert_pos = 0
        
        for i, line in enumerate(lines):
            if line.strip().startswith('use ') and 'crate::' in line:
                insert_pos = i + 1
            elif line.strip().startswith('use '):
                insert_pos = i + 1
            elif line.strip() and not line.strip().startswith('//') and not line.strip().startswith('use '):
                break
        
        # Insert import
        lines.insert(insert_pos, 'use crate::error_types::Error;')
        content = '\n'.join(lines)
    
    # Fix specific error patterns
    content = re.sub(r'\bCursedError\b', 'Error', content)
    content = re.sub(r'crate::error::Error', 'crate::error_types::Error', content)
    
    # Save if changed
    if content != original_content:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    
    return False

def get_error_files():
    """Get files with specific Error type compilation errors."""
    try:
        result = subprocess.run(
            ["cargo", "build", "--message-format=short"], 
            capture_output=True, 
            text=True, 
            cwd="/home/ghuntley/code/cursed"
        )
        
        error_files = set()
        for line in result.stderr.split('\n'):
            if 'cannot find type' in line and 'Error' in line and 'src/' in line:
                # Extract filename
                parts = line.split(':')
                if len(parts) >= 2:
                    file_path = parts[0].strip()
                    if file_path.startswith('src/'):
                        error_files.add(file_path)
        
        return list(error_files)
    except:
        return []

def main():
    print("🔍 Finding files with Error type compilation errors...")
    
    error_files = get_error_files()
    print(f"📁 Found {len(error_files)} files with Error type issues")
    
    if not error_files:
        print("No specific error files found. Applying to common problem areas...")
        # Apply to known problem areas
        problem_dirs = ['src/runtime', 'src/imports', 'src/package_manager']
        error_files = []
        for dir_path in problem_dirs:
            if os.path.exists(dir_path):
                for rs_file in Path(dir_path).rglob("*.rs"):
                    error_files.append(str(rs_file))
    
    fixed_count = 0
    for filepath in error_files[:30]:  # Process first 30 files
        try:
            if fix_file_comprehensively(filepath):
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
        
        error_lines = [line for line in result.stderr.split('\n') 
                      if 'cannot find type' in line and 'Error' in line]
        print(f"📊 Remaining Error type issues: {len(error_lines)}")
        
        if error_lines:
            print("🔍 Sample remaining errors:")
            for line in error_lines[:8]:
                print(f"   {line}")
                
    except Exception as e:
        print(f"❌ Error checking build status: {e}")

if __name__ == "__main__":
    main()
