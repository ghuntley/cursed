#!/usr/bin/env python3
"""
Script to update CURSED standard library array syntax from []type to type[value]
"""

import os
import re
import glob
from pathlib import Path

def fix_array_syntax(content):
    """Fix array syntax in the content."""
    changes = 0
    
    # Pattern 1: []type -> type[value] (simple slice)
    # Look for []followed by identifier, but not in array literals like []tea{}
    pattern1 = r'\[\]([a-zA-Z_][a-zA-Z0-9_]*)\b(?!\s*\{)'
    def replace1(match):
        nonlocal changes
        changes += 1
        return f"{match.group(1)}[value]"
    content = re.sub(pattern1, replace1, content)
    
    # Pattern 2: [n]type -> type[n] (fixed size array)
    pattern2 = r'\[([0-9]+)\]([a-zA-Z_][a-zA-Z0-9_]*)\b(?!\s*\{)'
    def replace2(match):
        nonlocal changes
        changes += 1
        return f"{match.group(2)}[{match.group(1)}]"
    content = re.sub(pattern2, replace2, content)
    
    # Pattern 3: Multi-dimensional arrays like [][]type -> type[value][value]
    # This is more complex - we need to handle multiple levels
    pattern3 = r'(\[\])+([a-zA-Z_][a-zA-Z0-9_]*)\b(?!\s*\{)'
    def replace3(match):
        nonlocal changes
        brackets_count = match.group(0).count('[]')
        type_name = match.group(2)
        changes += 1
        return f"{type_name}{'[value]' * brackets_count}"
    content = re.sub(pattern3, replace3, content)
    
    # Pattern 4: Mixed arrays like [n][]type or [][n]type
    # Handle patterns like [5][]normie or [][10]normie
    pattern4 = r'(\[(?:[0-9]+|\])+)+([a-zA-Z_][a-zA-Z0-9_]*)\b(?!\s*\{)'
    def replace4(match):
        nonlocal changes
        full_match = match.group(0)
        type_name = match.group(2)
        
        # Extract all bracket pairs
        bracket_pattern = r'\[([^\]]*)\]'
        brackets = re.findall(bracket_pattern, full_match)
        
        # Convert each bracket
        result_brackets = []
        for bracket in brackets:
            if bracket == '':  # Empty bracket []
                result_brackets.append('[value]')
            elif bracket.isdigit():  # Numeric bracket [n]
                result_brackets.append(f'[{bracket}]')
            else:
                result_brackets.append(f'[{bracket}]')
        
        if result_brackets:  # Only make changes if we found brackets
            changes += 1
            return f"{type_name}{''.join(result_brackets)}"
        return full_match
    
    # Apply pattern 4 after the others to catch any remaining cases
    content = re.sub(pattern4, replace4, content)
    
    return content, changes

def process_file(file_path):
    """Process a single .💀 file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            original_content = f.read()
        
        fixed_content, changes = fix_array_syntax(original_content)
        
        if changes > 0:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
            return changes
        
        return 0
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return 0

def main():
    stdlib_dir = "/home/ghuntley/cursed/stdlib"
    total_files = 0
    total_changes = 0
    files_changed = 0
    
    print("Searching for .💀 files in stdlib directory...")
    
    # Find all .💀 files recursively
    csd_files = []
    for root, dirs, files in os.walk(stdlib_dir):
        for file in files:
            if file.endswith('.💀'):
                csd_files.append(os.path.join(root, file))
    
    print(f"Found {len(csd_files)} .💀 files")
    
    for file_path in csd_files:
        changes = process_file(file_path)
        if changes > 0:
            files_changed += 1
            total_changes += changes
            rel_path = os.path.relpath(file_path, stdlib_dir)
            print(f"Updated {rel_path}: {changes} changes")
        total_files += 1
    
    print(f"\nSummary:")
    print(f"- Total files processed: {total_files}")
    print(f"- Files modified: {files_changed}")
    print(f"- Total syntax changes: {total_changes}")

if __name__ == "__main__":
    main()
