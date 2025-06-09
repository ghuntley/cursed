#!/usr/bin/env python3
"""
Fix all brace issues in AST files.
"""

import os
import re

def fix_file(file_path):
    """Fix all brace issues in a single file."""
    print(f"Processing {file_path}...")
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Fix pattern where we have "    }\n    }\n}" 
    content = re.sub(r'(\s+})\s+}\s+}', r'\1', content)
    
    # Fix standalone "}\n}" patterns that shouldn't be there
    content = re.sub(r'}\s*\n\s*}\s*\n\s*pub fn', r'}\n    \n    pub fn', content)
    
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"  Updated {file_path}")
    else:
        print(f"  No changes needed in {file_path}")

def main():
    """Process all AST files."""
    ast_files = ['src/ast/statements.rs']
    
    for file_path in ast_files:
        if os.path.exists(file_path):
            fix_file(file_path)
        else:
            print(f"File not found: {file_path}")

if __name__ == '__main__':
    main()
