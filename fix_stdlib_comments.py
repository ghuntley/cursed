#!/usr/bin/env python3

import os
import re

def fix_comments_in_file(filepath):
    """Fix # comments to fr fr comments in a CURSED file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Replace line-starting # comments with fr fr
        content = re.sub(r'^#\s*', 'fr fr ', content, flags=re.MULTILINE)
        
        # Replace mid-line # comments with fr fr 
        content = re.sub(r'\s+#\s*', ' fr fr ', content)
        
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f"Fixed: {filepath}")
        return True
    except Exception as e:
        print(f"Error fixing {filepath}: {e}")
        return False

def main():
    stdlib_dir = "stdlib"
    fixed_count = 0
    
    for root, dirs, files in os.walk(stdlib_dir):
        for file in files:
            if file.endswith('.csd'):
                filepath = os.path.join(root, file)
                if fix_comments_in_file(filepath):
                    fixed_count += 1
    
    print(f"\nFixed {fixed_count} files")

if __name__ == "__main__":
    main()
