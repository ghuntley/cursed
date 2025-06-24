#!/usr/bin/env python3

import subprocess
import re
import os

def get_e0753_files():
    """Extract all files with E0753 errors and their line numbers."""
    result = subprocess.run(['cargo', 'build'], capture_output=True, text=True, cwd='.')
    errors = {}
    
    lines = result.stderr.split('\n')
    for i, line in enumerate(lines):
        if 'E0753' in line and 'expected outer doc comment' in line:
            # Look for the next line with file location
            j = i + 1
            while j < len(lines) and '-->' not in lines[j]:
                j += 1
            if j < len(lines):
                match = re.search(r'--> ([^:]+):(\d+):\d+', lines[j])
                if match:
                    file_path = match.group(1)
                    line_num = int(match.group(2))
                    if file_path not in errors:
                        errors[file_path] = []
                    errors[file_path].append(line_num)
    
    return errors

def fix_doc_comments_in_file(file_path, error_lines):
    """Fix doc comment issues in a specific file."""
    if not os.path.exists(file_path):
        print(f"File not found: {file_path}")
        return 0
    
    with open(file_path, 'r') as f:
        lines = f.readlines()
    
    fixes_made = 0
    modified_lines = set()
    
    # Convert error lines to 0-based indexing
    error_lines_0based = [line - 1 for line in error_lines]
    
    for i in error_lines_0based:
        if i < len(lines):
            line = lines[i]
            # Check if this line has a misplaced inner doc comment
            if line.strip().startswith('//!'):
                # Check if this is at the very beginning of the file (before any code)
                has_code_before = False
                for j in range(i):
                    prev_line = lines[j].strip()
                    if prev_line and not prev_line.startswith('//') and not prev_line.startswith('#'):
                        has_code_before = True
                        break
                
                if has_code_before:
                    # Convert //! to /// for outer doc comment
                    lines[i] = line.replace('//!', '///', 1)
                    fixes_made += 1
                    modified_lines.add(i + 1)
                else:
                    # Move inner doc comments to the top of the file
                    # For now, just convert to outer doc comments
                    lines[i] = line.replace('//!', '///', 1)
                    fixes_made += 1
                    modified_lines.add(i + 1)
    
    if fixes_made > 0:
        with open(file_path, 'w') as f:
            f.writelines(lines)
        print(f"Fixed {fixes_made} doc comment issues in {file_path}")
        if modified_lines:
            print(f"  Modified lines: {sorted(modified_lines)}")
    
    return fixes_made

def main():
    print("Analyzing E0753 doc comment errors...")
    
    error_files = get_e0753_files()
    if not error_files:
        print("No E0753 errors found!")
        return
    
    print(f"Found E0753 errors in {len(error_files)} files")
    
    total_fixes = 0
    for file_path, error_lines in error_files.items():
        print(f"\nProcessing {file_path} ({len(error_lines)} errors)...")
        fixes = fix_doc_comments_in_file(file_path, error_lines)
        total_fixes += fixes
    
    print(f"\nTotal fixes applied: {total_fixes}")
    
    # Test compilation
    print("\nTesting compilation after fixes...")
    result = subprocess.run(['cargo', 'build'], capture_output=True, text=True)
    new_e0753_count = result.stderr.count('E0753')
    
    print(f"E0753 errors after fixes: {new_e0753_count}")
    print(f"E0753 errors resolved: {412 - new_e0753_count}")

if __name__ == "__main__":
    main()
