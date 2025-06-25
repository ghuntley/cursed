#!/usr/bin/env python3

import os
import re
import subprocess
from pathlib import Path

def fix_import_positioning(filepath):
    """Fix Error import positioning issues in a file."""
    if not os.path.exists(filepath):
        return False
        
    with open(filepath, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Find all error_types imports
    error_imports = list(re.finditer(r'^use crate::error_types::Error;', content, re.MULTILINE))
    
    if len(error_imports) <= 1:
        return False  # No duplicates to fix
    
    # Remove all error_types imports
    content = re.sub(r'^use crate::error_types::Error;\n?', '', content, flags=re.MULTILINE)
    
    # Find the proper position to insert import (after first use statements)
    lines = content.split('\n')
    insert_pos = 0
    
    for i, line in enumerate(lines):
        if line.strip().startswith('use ') and 'std::' in line:
            insert_pos = i + 1
        elif line.strip().startswith('use ') and 'tracing::' in line:
            insert_pos = i + 1
        elif line.strip().startswith('use ') and ('crate::' in line or 'super::' in line):
            insert_pos = i
            break
        elif line.strip() and not line.strip().startswith('//') and not line.strip().startswith('use '):
            break
    
    # Insert the import at the proper position
    lines.insert(insert_pos, 'use crate::error_types::Error;')
    content = '\n'.join(lines)
    
    # Save if changed
    if content != original_content:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    
    return False

def get_files_with_import_issues():
    """Get files that have duplicate or misplaced Error imports."""
    files_with_issues = []
    
    # Search for files with duplicate error_types imports
    src_path = Path("/home/ghuntley/code/cursed/src")
    for rust_file in src_path.rglob("*.rs"):
        try:
            with open(rust_file, 'r') as f:
                content = f.read()
                
            # Count error_types imports
            error_imports = re.findall(r'^use crate::error_types::Error;', content, re.MULTILINE)
            if len(error_imports) > 1:
                files_with_issues.append(str(rust_file))
                
        except:
            continue
    
    return files_with_issues

def main():
    print("🔍 Finding files with import positioning issues...")
    
    files_with_issues = get_files_with_import_issues()
    print(f"📁 Found {len(files_with_issues)} files with import issues")
    
    fixed_count = 0
    for filepath in files_with_issues:
        try:
            if fix_import_positioning(filepath):
                print(f"✅ Fixed: {filepath}")
                fixed_count += 1
            else:
                print(f"⏭️  Skipped: {filepath}")
        except Exception as e:
            print(f"❌ Error fixing {filepath}: {e}")
    
    print(f"\n🎉 Fixed {fixed_count} files")
    
    # Quick build check to see remaining errors
    print("\n🔍 Quick Error type check...")
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
        
    except Exception as e:
        print(f"❌ Error checking build status: {e}")

if __name__ == "__main__":
    main()
