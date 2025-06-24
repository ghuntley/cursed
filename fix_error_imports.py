#!/usr/bin/env python3
"""
Fix Error type import issues systematically.
Add 'use crate::error::Error;' to files that need it.
"""
import subprocess
import re
import os

def get_files_needing_error_import():
    """Get files that have 'cannot find type Error' errors."""
    try:
        result = subprocess.run(
            ["./fix_linking.sh", "cargo", "build"],
            capture_output=True,
            text=True,
            cwd="."
        )
        
        files_needing_error = set()
        lines = result.stderr.split('\n')
        
        for i, line in enumerate(lines):
            if "cannot find type `Error`" in line:
                # Look for file path in the next few lines
                for j in range(max(0, i-3), min(len(lines), i+3)):
                    match = re.search(r'--> (src/[^:]+\.rs):', lines[j])
                    if match:
                        files_needing_error.add(match.group(1))
                        break
        
        return list(files_needing_error)
        
    except Exception as e:
        print(f"Error getting files: {e}")
        return []

def add_error_import(file_path):
    """Add Error import to a file if it doesn't already have it."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Check if Error is already imported
        if 'use crate::error::Error' in content:
            return False
            
        lines = content.split('\n')
        
        # Find the best place to add the import (after other use statements)
        insert_line = 0
        for i, line in enumerate(lines):
            if line.strip().startswith('use '):
                insert_line = i + 1
            elif line.strip() and not line.strip().startswith('//') and not line.strip().startswith('///'):
                break
        
        # Insert the import
        lines.insert(insert_line, 'use crate::error::Error;')
        
        # Write back the file
        with open(file_path, 'w') as f:
            f.write('\n'.join(lines))
        
        return True
        
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def main():
    print("🔧 Fixing Error type import issues...")
    
    # Get files that need Error imports
    files_needing_error = get_files_needing_error_import()
    print(f"📋 Found {len(files_needing_error)} files needing Error imports")
    
    total_fixed = 0
    
    for file_path in files_needing_error:
        if os.path.exists(file_path):
            if add_error_import(file_path):
                print(f"✅ Added Error import to {file_path}")
                total_fixed += 1
        else:
            print(f"❌ File not found: {file_path}")
    
    print(f"\n🎉 Error import fix complete!")
    print(f"📊 Total files fixed: {total_fixed}")
    print(f"📁 Files processed: {len(files_needing_error)}")

if __name__ == "__main__":
    main()
