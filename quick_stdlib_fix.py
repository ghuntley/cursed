#!/usr/bin/env python3

import os

def comment_problematic_stdlib_refs():
    """Comment out references to stdlib module that doesn't exist"""
    
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                
                with open(file_path, 'r') as f:
                    content = f.read()
                
                original_content = content
                lines = content.split('\n')
                modified = False
                
                for i, line in enumerate(lines):
                    # Comment out use statements referencing crate::stdlib
                    if ('crate::stdlib' in line or 'use stdlib::' in line) and not line.strip().startswith('//'):
                        lines[i] = '// ' + line
                        modified = True
                
                if modified:
                    new_content = '\n'.join(lines)
                    with open(file_path, 'w') as f:
                        f.write(new_content)
                    print(f"Fixed stdlib references in {file_path}")

if __name__ == '__main__':
    comment_problematic_stdlib_refs()
