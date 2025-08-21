#!/usr/bin/env python3
"""Fix print statements to include .{} format parameter."""

import os
import re
import subprocess

def fix_print_statements(file_path):
    """Fix print statements in a single file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Pattern to match print statements without .{} format
        # Match: print("text");, print("text\n"); but not print("text", .{});
        pattern = r'print\(\s*"([^"]*?)"\s*\);'
        
        def replace_print(match):
            text = match.group(1)
            return f'print("{text}", .{{}});'
        
        content = re.sub(pattern, replace_print, content)
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {file_path}")
            return True
        else:
            print(f"No changes: {file_path}")
            return False
    
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """Fix print statements in all .zig files."""
    src_dir = "src-zig"
    fixed_count = 0
    total_count = 0
    
    if not os.path.exists(src_dir):
        print(f"Source directory {src_dir} not found")
        return
    
    # Find all .zig files
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if file.endswith('.zig'):
                file_path = os.path.join(root, file)
                total_count += 1
                if fix_print_statements(file_path):
                    fixed_count += 1
    
    print(f"\nFixed {fixed_count} files out of {total_count} total files")

if __name__ == "__main__":
    main()
