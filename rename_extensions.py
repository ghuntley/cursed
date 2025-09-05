#!/usr/bin/env python3
"""
Script to rename all .💀 files to .💀 files in the CURSED project
"""

import os
import sys
import glob
import shutil

def rename_file(old_path, new_path):
    """Rename a file and print the operation"""
    try:
        os.rename(old_path, new_path)
        print(f"✅ {old_path} → {new_path}")
        return True
    except Exception as e:
        print(f"❌ Failed to rename {old_path}: {e}")
        return False

def find_and_rename_files(root_dir):
    """Find all .💀 files and rename them to .💀"""
    renamed_count = 0
    failed_count = 0
    
    # Use glob to find all .💀 files recursively
    pattern = os.path.join(root_dir, "**", "*.💀")
    csd_files = glob.glob(pattern, recursive=True)
    
    print(f"Found {len(csd_files)} .💀 files to rename")
    print("=" * 60)
    
    for csd_file in sorted(csd_files):
        # Generate new filename with .💀 extension
        new_file = csd_file[:-4] + ".💀"  # Remove .💀 and add .💀
        
        if rename_file(csd_file, new_file):
            renamed_count += 1
        else:
            failed_count += 1
    
    print("=" * 60)
    print(f"✅ Successfully renamed: {renamed_count} files")
    if failed_count > 0:
        print(f"❌ Failed to rename: {failed_count} files")
    
    return renamed_count, failed_count

def main():
    # Get the current working directory (should be /home/ghuntley/cursed)
    root_dir = os.getcwd()
    
    print("🔄 CURSED File Extension Renaming Tool")
    print(f"📁 Working directory: {root_dir}")
    print("🎯 Converting .💀 → .💀")
    print()
    
    # Auto-proceed in non-interactive environment
    print("✅ Proceeding with automatic renaming...")
    
    # Perform the renaming
    renamed, failed = find_and_rename_files(root_dir)
    
    if failed == 0:
        print("\n🎉 All files renamed successfully!")
    else:
        print(f"\n⚠️  Completed with {failed} errors")
        sys.exit(1)

if __name__ == "__main__":
    main()
