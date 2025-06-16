#!/usr/bin/env python3

import re
import os
import subprocess

def find_duplicate_export_errors():
    """Run cargo check and parse duplicate export errors"""
    result = subprocess.run(['cargo', 'check'], capture_output=True, text=True, cwd='.')
    
    duplicates = []
    lines = result.stderr.split('\n')
    
    for i, line in enumerate(lines):
        if 'is defined multiple times' in line:
            # Extract file path and duplicate name
            if i > 0 and '-->' in lines[i-1]:
                file_line = lines[i-1].strip()
                file_match = re.search(r'--> (.+?):(\d+):(\d+)', file_line)
                if file_match:
                    file_path = file_match.group(1)
                    
                    # Extract the duplicate name
                    name_match = re.search(r'the name `(.+?)` is defined multiple times', line)
                    if name_match:
                        duplicate_name = name_match.group(1)
                        duplicates.append((file_path, duplicate_name))
                        print(f"Found duplicate: {duplicate_name} in {file_path}")
    
    return duplicates

def fix_duplicate_exports(file_path, duplicate_name):
    """Remove duplicate export from pub use statements"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Pattern to match pub use blocks that contain the duplicate
        pub_use_pattern = r'pub use[^{]*\{[^}]*\}'
        
        def remove_duplicate_from_use(match):
            use_block = match.group(0)
            lines = use_block.split('\n')
            
            # Find and remove lines containing the duplicate name
            filtered_lines = []
            for line in lines:
                if duplicate_name in line and not line.strip().startswith('//'):
                    # Check if this is just the duplicate name (not part of a longer name)
                    if re.search(rf'\b{re.escape(duplicate_name)}\b', line):
                        # Remove or comment out this line
                        if ',' in line:
                            # Remove the name but keep the comma structure
                            line = re.sub(rf'\s*{re.escape(duplicate_name)}\s*,?\s*', '', line)
                            if line.strip() == ',':
                                continue
                        else:
                            continue
                
                if line.strip():  # Keep non-empty lines
                    filtered_lines.append(line)
            
            return '\n'.join(filtered_lines)
        
        # Apply the fix
        new_content = re.sub(pub_use_pattern, remove_duplicate_from_use, content, flags=re.DOTALL)
        
        if new_content != content:
            with open(file_path, 'w') as f:
                f.write(new_content)
            print(f"Fixed duplicate export of {duplicate_name} in {file_path}")
            return True
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
    return False

def main():
    """Main function to find and fix duplicate exports"""
    duplicates = find_duplicate_export_errors()
    
    fixed_count = 0
    for file_path, duplicate_name in duplicates:
        if fix_duplicate_exports(file_path, duplicate_name):
            fixed_count += 1
    
    print(f"\nFixed {fixed_count} duplicate export errors")
    
    # Check if there are still errors
    result = subprocess.run(['cargo', 'check'], capture_output=True, text=True, cwd='.')
    remaining_errors = result.stderr.count('error[E0255]') + result.stderr.count('error[E0252]')
    print(f"Remaining duplicate definition errors: {remaining_errors}")

if __name__ == "__main__":
    main()
