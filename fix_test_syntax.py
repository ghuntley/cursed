#!/usr/bin/env python3
"""
Script to systematically fix CURSED syntax issues in test files.

Issues to fix:
1. Entry-point mismatch: `vibe main` should be `vibe main_character` when function is `slay main_character()`
2. Variable declaration order: `sus <identifier> <type>` should be `sus <type> <identifier>`
"""

import os
import re
import subprocess
from pathlib import Path

def read_file(filepath):
    """Read file content safely."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            return f.read()
    except Exception as e:
        print(f"Error reading {filepath}: {e}")
        return None

def write_file(filepath, content):
    """Write file content safely."""
    try:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    except Exception as e:
        print(f"Error writing {filepath}: {e}")
        return False

def fix_entry_point_mismatch(content):
    """Fix vibe main -> vibe main_character when function is slay main_character()."""
    # Check if file has slay main_character() function
    has_main_character_function = re.search(r'slay main_character\s*\(', content)
    
    if has_main_character_function and content.startswith('vibe main'):
        # Replace vibe main with vibe main_character (but preserve any trailing content)
        content = re.sub(r'^vibe main(\s*;?\s*)$', r'vibe main_character\1', content, flags=re.MULTILINE)
    
    return content

def fix_variable_declarations(content):
    """Fix sus <identifier> <type> -> sus <type> <identifier>."""
    # Pattern to match: sus <identifier> <type> = <value>
    patterns = [
        (r'sus ([a-zA-Z_][a-zA-Z0-9_]*) (int|float|bool|str)\s*=', r'sus \2 \1 ='),
        (r'sus ([a-zA-Z_][a-zA-Z0-9_]*) (int|float|bool|str)\s*;', r'sus \2 \1;'),
    ]
    
    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content)
    
    return content

def fix_file(filepath):
    """Fix a single file."""
    content = read_file(filepath)
    if content is None:
        return False, "Could not read file"
    
    original_content = content
    
    # Apply fixes
    content = fix_entry_point_mismatch(content)
    content = fix_variable_declarations(content)
    
    # Check if any changes were made
    if content != original_content:
        if write_file(filepath, content):
            return True, "Fixed"
        else:
            return False, "Could not write file"
    else:
        return False, "No changes needed"

def find_cursed_files(root_dir):
    """Find all .💀 files in the directory tree."""
    cursed_files = []
    for root, dirs, files in os.walk(root_dir):
        for file in files:
            if file.endswith('.💀'):
                cursed_files.append(os.path.join(root, file))
    return sorted(cursed_files)

def main():
    test_programs_dir = "/home/ghuntley/cursed/test_suite/test_programs"
    
    if not os.path.exists(test_programs_dir):
        print(f"Directory {test_programs_dir} not found")
        return
    
    # Find all .💀 files
    cursed_files = find_cursed_files(test_programs_dir)
    print(f"Found {len(cursed_files)} .💀 files")
    
    # Fix each file
    fixed_count = 0
    errors = []
    
    for filepath in cursed_files:
        rel_path = os.path.relpath(filepath, test_programs_dir)
        success, message = fix_file(filepath)
        
        if success:
            print(f"✓ Fixed: {rel_path}")
            fixed_count += 1
        elif message != "No changes needed":
            print(f"✗ Error: {rel_path} - {message}")
            errors.append((rel_path, message))
    
    print(f"\nSummary:")
    print(f"- Total files scanned: {len(cursed_files)}")
    print(f"- Files fixed: {fixed_count}")
    print(f"- Files with errors: {len(errors)}")
    
    if errors:
        print(f"\nErrors encountered:")
        for rel_path, error in errors:
            print(f"  {rel_path}: {error}")

if __name__ == "__main__":
    main()
