#!/usr/bin/env python3

import os
import re

def fix_file(filepath):
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix corrupted std..empty patterns
        content = content.replace('std..empty', '.{}')
        
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
        
    except Exception as e:
        print(f"Error processing {filepath}: {e}")

# Process all Zig files
for root, dirs, files in os.walk('src-zig/'):
    for file in files:
        if file.endswith('.zig'):
            fix_file(os.path.join(root, file))

print("Fixed all std..empty patterns!")
