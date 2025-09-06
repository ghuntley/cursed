#!/usr/bin/env python3

import os
import re
import glob

def fix_collections_usage():
    # Find all .💀 files in leetcode_comprehensive_suite
    files_pattern = "/home/ghuntley/cursed/leetcode_comprehensive_suite/**/*.💀"
    cursed_files = glob.glob(files_pattern, recursive=True)
    
    modified_files = []
    
    for file_path in cursed_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Replace collections.len( with len(
            content = re.sub(r'collections\.len\(', 'len(', content)
            
            # Replace collections.append( with append(
            content = re.sub(r'collections\.append\(', 'append(', content)
            
            # Check if content changed
            if content != original_content:
                # Check if collections import is still needed
                # If content still contains "collections." then keep the import
                if "collections." not in content:
                    # Remove unused collections import
                    content = re.sub(r'yeet "collections"\s*\n', '', content)
                
                # Write back the fixed content
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                
                modified_files.append(file_path)
                print(f"Fixed: {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    return modified_files

if __name__ == "__main__":
    modified = fix_collections_usage()
    print(f"\nTotal files modified: {len(modified)}")
    for f in modified:
        print(f"  - {f}")
