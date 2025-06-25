#!/usr/bin/env python3

import os
import re

def fix_duplicate_cursederror_imports():
    """Remove duplicate CursedError imports from all files"""
    
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                
                with open(file_path, 'r') as f:
                    content = f.read()
                
                original_content = content
                
                # Find all CursedError import lines
                lines = content.split('\n')
                cursed_error_imports = []
                other_lines = []
                
                for i, line in enumerate(lines):
                    if 'use' in line and 'CursedError' in line and not line.strip().startswith('//'):
                        cursed_error_imports.append((i, line))
                    else:
                        other_lines.append((i, line))
                
                # If multiple CursedError imports found, keep only the first one
                if len(cursed_error_imports) > 1:
                    print(f"Fixing {file_path}: {len(cursed_error_imports)} CursedError imports")
                    
                    # Reconstruct content with only the first CursedError import
                    new_lines = []
                    kept_first = False
                    
                    for i, line in enumerate(lines):
                        if 'use' in line and 'CursedError' in line and not line.strip().startswith('//'):
                            if not kept_first:
                                new_lines.append(line)
                                kept_first = True
                            # Skip duplicate imports
                        else:
                            new_lines.append(line)
                    
                    new_content = '\n'.join(new_lines)
                    
                    if new_content != original_content:
                        with open(file_path, 'w') as f:
                            f.write(new_content)
                        print(f"  Removed {len(cursed_error_imports) - 1} duplicate imports")

if __name__ == '__main__':
    fix_duplicate_cursederror_imports()
