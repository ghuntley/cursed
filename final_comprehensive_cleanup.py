#!/usr/bin/env python3
"""
FINAL comprehensive cleanup of ALL remaining .💀 references 
"""

import os
import sys
import re
import subprocess
from pathlib import Path

def get_all_files_with_csd():
    """Get all files containing .💀 references"""
    try:
        result = subprocess.run([
            'grep', '-r', '-l', r'\.💀', '.',
            '--exclude-dir=.git',
            '--exclude-dir=zig-out', 
            '--exclude-dir=.zig-cache',
            '--exclude-dir=__pycache__'
        ], capture_output=True, text=True, cwd='/home/ghuntley/cursed')
        
        if result.returncode == 0:
            files = [f.strip() for f in result.stdout.split('\n') if f.strip()]
            return files
        return []
    except Exception as e:
        print(f"Error getting files: {e}")
        return []

def fix_file_with_sed(filepath):
    """Use sed to replace all .💀 with .💀"""
    try:
        # Skip binary files and certain problematic files
        skip_patterns = [
            '/cross_compilation_results/',
            '.o', '.so', '.a', '.dll',
            '.exe', '.bin', '.img'
        ]
        
        if any(pattern in filepath for pattern in skip_patterns):
            return False, "Skipped binary/problematic file"
        
        # Use sed to replace .💀 with .💀
        result = subprocess.run([
            'sed', '-i', 's/\\.💀/\.💀/g', filepath
        ], capture_output=True, text=True)
        
        if result.returncode == 0:
            return True, "Fixed with sed"
        else:
            return False, f"sed error: {result.stderr}"
            
    except Exception as e:
        return False, f"Exception: {e}"

def main():
    print("🚀 FINAL COMPREHENSIVE CSD CLEANUP")
    print("💀 Converting ALL remaining .💀 to .💀")
    print("=" * 60)
    
    # Get all files with .💀 references
    files_with_csd = get_all_files_with_csd()
    
    print(f"Found {len(files_with_csd)} files with .💀 references")
    
    if not files_with_csd:
        print("✅ No .💀 references found!")
        return
    
    fixed_count = 0
    skipped_count = 0
    error_count = 0
    
    print("\nProcessing files...")
    
    for i, filepath in enumerate(files_with_csd, 1):
        if i % 100 == 0:
            print(f"Progress: {i}/{len(files_with_csd)}")
        
        success, message = fix_file_with_sed(filepath)
        
        if success:
            fixed_count += 1
        elif "Skipped" in message:
            skipped_count += 1
        else:
            error_count += 1
            print(f"❌ {filepath}: {message}")
    
    print("=" * 60)
    print(f"✅ Fixed: {fixed_count} files")
    print(f"⚪ Skipped: {skipped_count} files") 
    print(f"❌ Errors: {error_count} files")
    
    # Final verification
    print("\n🔍 Final verification...")
    remaining_files = get_all_files_with_csd()
    non_binary_remaining = [f for f in remaining_files if not any(pat in f for pat in ['/cross_compilation_results/', '.o', '.so', '.a', '.dll', '.exe', '.bin'])]
    
    print(f"📊 Remaining .💀 references: {len(remaining_files)} total, {len(non_binary_remaining)} non-binary")
    
    if len(non_binary_remaining) <= 10:
        print("🎯 Remaining non-binary files:")
        for f in non_binary_remaining:
            print(f"  - {f}")
    
    if len(non_binary_remaining) == 0:
        print("\n🎉 SUCCESS: All .💀 references eliminated!")
    else:
        print(f"\n⚠️  Still {len(non_binary_remaining)} non-binary files to fix")

if __name__ == "__main__":
    main()
