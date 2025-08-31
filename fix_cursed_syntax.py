#!/usr/bin/env python3

import os
import re
import glob

def fix_cursed_syntax_in_file(filepath):
    """Fix CURSED syntax issues in a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix main() function to main_character()
        content = re.sub(r'slay main\(\)', r'slay main_character()', content)
        
        # Fix variable declarations - replace normie/snack/cap/periodt with drip
        content = re.sub(r'sus\s+(\w+)\s+(normie|snack|cap|periodt)\s*=', r'sus \1 drip =', content)
        
        # Fix function parameter types - replace normie/snack/cap/periodt with drip
        content = re.sub(r'(\w+)\s+(normie|snack|cap|periodt)(?=\s*[,)])', r'\1 drip', content)
        
        # Fix string variable type declarations if they exist
        content = re.sub(r'sus\s+(\w+)\s+tea\s*=', r'sus \1 drip =', content)
        
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
            return True
        else:
            return False
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    """Fix all CURSED files in test suite"""
    test_dir = "/home/ghuntley/cursed/test_suite/test_programs"
    
    # Find all .csd files
    csd_files = []
    for root, dirs, files in os.walk(test_dir):
        for file in files:
            if file.endswith('.csd'):
                csd_files.append(os.path.join(root, file))
    
    print(f"Found {len(csd_files)} CURSED files to check")
    
    fixed_count = 0
    for filepath in csd_files:
        if fix_cursed_syntax_in_file(filepath):
            fixed_count += 1
    
    print(f"\nFixed {fixed_count} files")

if __name__ == "__main__":
    main()
