#!/usr/bin/env python3

import os
import re
import glob

def fix_malformed_use_statements(file_path):
    """Fix malformed use statements in a Rust file."""
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Track if we made changes
    original_content = content
    
    # Pattern 1: Remove "use crate::error::Error;" that appears inside use blocks
    # First, find all use blocks and remove the error import from inside them
    def fix_use_block(match):
        use_block = match.group(0)
        # Remove the malformed line from inside the block
        fixed_block = re.sub(r'\nuse crate::error::Error;\n', '\n', use_block)
        fixed_block = re.sub(r'\nuse crate::error::Error;', '', fixed_block)
        return fixed_block
    
    # Match use blocks that span multiple lines
    content = re.sub(r'use [^;]+\{[^}]*\};', fix_use_block, content, flags=re.DOTALL)
    
    # Pattern 2: Remove duplicate error imports
    lines = content.split('\n')
    error_import_seen = False
    fixed_lines = []
    
    for line in lines:
        # If it's an error import line
        if line.strip() == 'use crate::error::Error;':
            if not error_import_seen:
                fixed_lines.append(line)
                error_import_seen = True
            # Otherwise skip duplicate
        else:
            fixed_lines.append(line)
    
    content = '\n'.join(fixed_lines)
    
    # If we removed all error imports but the file still references Error, add it back
    if 'use crate::error::Error;' not in content and 'Error' in content:
        # Find a good place to insert it (after other crate imports)
        lines = content.split('\n')
        insert_index = 0
        for i, line in enumerate(lines):
            if line.startswith('use crate::'):
                insert_index = i + 1
            elif line.startswith('use ') and 'crate::' not in line:
                break
        
        lines.insert(insert_index, 'use crate::error::Error;')
        content = '\n'.join(lines)
    
    # Write back if changed
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        return True
    return False

def main():
    print("🔧 Comprehensively fixing all malformed use statements...")
    
    # Find all Rust files
    rust_files = glob.glob('src/**/*.rs', recursive=True)
    
    fixed_count = 0
    for file_path in rust_files:
        if fix_malformed_use_statements(file_path):
            print(f"📝 Fixed {file_path}")
            fixed_count += 1
    
    print(f"✅ Fixed {fixed_count} files with malformed use statements!")

if __name__ == "__main__":
    main()
