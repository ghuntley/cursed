#!/usr/bin/env python3
"""
Bulk update all remaining .💀 references to .💀 in documentation and other files
"""

import os
import sys
import re
import glob
from typing import List, Tuple

def update_file_references(file_path: str) -> Tuple[bool, int]:
    """Update .💀 references to .💀 in a single file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Replace .💀 with .💀 in various contexts
        patterns = [
            (r'\.💀\b', '.💀'),  # Basic .💀 -> .💀
            (r'/mod\.💀', '/mod.💀'),  # mod.💀 -> mod.💀  
            (r'_\.💀', '_.💀'),  # file_.💀 -> file_.💀
            (r'file\.💀', 'file.💀'),  # file.💀 -> file.💀
            (r'main\.💀', 'main.💀'),  # main.💀 -> main.💀
            (r'lib\.💀', 'lib.💀'),  # lib.💀 -> lib.💀
            (r'test.*?\.💀', lambda m: m.group(0).replace('.💀', '.💀')),  # test*.💀
        ]
        
        updates = 0
        for pattern, replacement in patterns:
            if callable(replacement):
                new_content = re.sub(pattern, replacement, content)
            else:
                new_content = re.sub(pattern, replacement, content)
            
            if new_content != content:
                updates += len(re.findall(pattern, content))
                content = new_content
        
        # Only write if changes were made
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True, updates
        
        return False, 0
        
    except Exception as e:
        print(f"❌ Error processing {file_path}: {e}")
        return False, 0

def main():
    print("🔄 CURSED Bulk Extension Update Tool")
    print("📁 Updating all .💀 references to .💀")
    print("=" * 60)
    
    # Get all markdown files with .💀 references
    md_files = []
    
    # Find markdown files
    for root, dirs, files in os.walk('.'):
        # Skip certain directories
        skip_dirs = {'.git', '.zig-cache', 'zig-out', '__pycache__', '.vscode'}
        dirs[:] = [d for d in dirs if d not in skip_dirs]
        
        for file in files:
            if file.endswith('.md'):
                file_path = os.path.join(root, file)
                # Check if file contains .💀 references
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        if '.💀' in f.read():
                            md_files.append(file_path)
                except:
                    continue
    
    print(f"Found {len(md_files)} files with .💀 references")
    print()
    
    total_files_updated = 0
    total_updates = 0
    
    for file_path in sorted(md_files):
        updated, count = update_file_references(file_path)
        if updated:
            print(f"✅ {file_path}: {count} updates")
            total_files_updated += 1
            total_updates += count
        else:
            print(f"⚪ {file_path}: no changes needed")
    
    print("=" * 60)
    print(f"✅ Updated {total_files_updated} files")
    print(f"🔧 Made {total_updates} total replacements")
    print("🎉 Bulk update complete!")

if __name__ == "__main__":
    main()
