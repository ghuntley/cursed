#!/usr/bin/env python3

import os
import re

# Fix misplaced inner doc comments that appear after imports/code
def fix_doc_comments_in_file(file_path):
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        lines = content.split('\n')
        modified = False
        
        # Look for inner doc comments that appear after non-comment lines
        for i, line in enumerate(lines):
            if line.strip().startswith('//!'):
                # Check if there's any non-comment, non-empty line before this
                found_code = False
                for j in range(i):
                    prev_line = lines[j].strip()
                    if prev_line and not prev_line.startswith('//') and not prev_line.startswith('#'):
                        found_code = True
                        break
                
                if found_code:
                    # Convert inner doc comment to regular comment
                    lines[i] = line.replace('//!', '//', 1)
                    modified = True
        
        if modified:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write('\n'.join(lines))
            print(f"Fixed doc comments in {file_path}")
    
    except Exception as e:
        print(f"Error processing {file_path}: {e}")

# Process all Rust files in stdlib
for root, dirs, files in os.walk('src/stdlib'):
    for file in files:
        if file.endswith('.rs'):
            fix_doc_comments_in_file(os.path.join(root, file))
