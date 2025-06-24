#!/usr/bin/env python3
"""
Surgical fix for E0252 duplicate Error import issues.
Only removes duplicate imports, preserves existing architecture.
"""
import os
import re
import subprocess

def get_e0252_files():
    """Get files with E0252 errors from cargo build output."""
    try:
        result = subprocess.run(
            ["./fix_linking.sh", "cargo", "build"],
            capture_output=True,
            text=True,
            cwd="."
        )
        
        files_with_errors = []
        lines = result.stderr.split('\n')
        
        for line in lines:
            if 'E0252' in line and '  --> ' in line:
                # Extract file path from error line
                # Format: "  --> src/path/file.rs:line:col"
                match = re.search(r'  --> (src/[^:]+\.rs):', line)
                if match:
                    files_with_errors.append(match.group(1))
        
        return list(set(files_with_errors))  # Remove duplicates
    except Exception as e:
        print(f"Error getting E0252 files: {e}")
        return []

def fix_duplicate_imports(file_path):
    """Fix duplicate Error imports in a single file."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        lines = content.split('\n')
        error_import_lines = []
        
        # Find all lines that import Error
        for i, line in enumerate(lines):
            if ('use crate::error::Error' in line or 
                'use crate::error::{Error' in line or
                'use crate::error::{CursedError, Error}' in line):
                error_import_lines.append(i)
        
        # If we have multiple Error imports, keep the most comprehensive one
        if len(error_import_lines) > 1:
            # Sort by comprehensiveness (keep the one with most imports)
            import_lines_with_content = [(i, lines[i]) for i in error_import_lines]
            import_lines_with_content.sort(key=lambda x: len(x[1]), reverse=True)
            
            # Keep the first (most comprehensive), remove the rest
            lines_to_remove = [i for i, _ in import_lines_with_content[1:]]
            
            # Remove duplicate import lines
            for line_num in sorted(lines_to_remove, reverse=True):
                lines.pop(line_num)
            
            # Write back the fixed content
            with open(file_path, 'w') as f:
                f.write('\n'.join(lines))
            
            return len(lines_to_remove)
        
        return 0
        
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return 0

def main():
    print("🔧 Starting surgical E0252 duplicate import fix...")
    
    # Get files with E0252 errors
    files_with_errors = get_e0252_files()
    print(f"📋 Found {len(files_with_errors)} files with E0252 errors")
    
    total_fixes = 0
    
    for file_path in files_with_errors:
        if os.path.exists(file_path):
            fixes = fix_duplicate_imports(file_path)
            if fixes > 0:
                print(f"✅ Fixed {fixes} duplicate imports in {file_path}")
                total_fixes += fixes
        else:
            print(f"❌ File not found: {file_path}")
    
    print(f"\n🎉 Surgical fix complete!")
    print(f"📊 Total duplicate imports removed: {total_fixes}")
    print(f"📁 Files processed: {len(files_with_errors)}")

if __name__ == "__main__":
    main()
