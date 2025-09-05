#!/usr/bin/env python3

import os
import re
import glob

def cleanup_cursed_file(filepath):
    """Clean up a CURSED file to remove duplicate imports"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        lines = content.split('\n')
        
        # Extract package declaration
        package_line = ""
        content_start = 0
        
        for i, line in enumerate(lines):
            if line.strip().startswith('vibe '):
                package_line = line.strip()
                content_start = i + 1
                break
        
        if not package_line:
            print(f"SKIP (no package): {filepath}")
            return False
        
        # Extract comments
        comment_lines = []
        while content_start < len(lines) and (lines[content_start].strip().startswith('//') or lines[content_start].strip() == ''):
            if lines[content_start].strip():  # Non-empty comment
                comment_lines.append(lines[content_start])
            content_start += 1
        
        # Extract imports and deduplicate
        imports = set()
        while content_start < len(lines) and lines[content_start].strip().startswith('yeet '):
            imports.add(lines[content_start].strip())
            content_start += 1
            
        # Skip empty lines after imports
        while content_start < len(lines) and lines[content_start].strip() == '':
            content_start += 1
        
        # Get remaining content
        remaining_lines = lines[content_start:]
        
        # Rebuild content
        new_content = package_line + '\n\n'
        
        # Add comments
        for comment in comment_lines:
            new_content += comment + '\n'
        
        new_content += '\n'
        
        # Add deduplicated imports
        if imports:
            for imp in sorted(imports):
                new_content += imp + '\n'
            new_content += '\n'
        
        # Add remaining content
        for line in remaining_lines:
            new_content += line + '\n'
        
        # Clean up extra newlines
        new_content = re.sub(r'\n{3,}', '\n\n', new_content)
        new_content = new_content.strip() + '\n'
        
        # Write back
        with open(filepath, 'w') as f:
            f.write(new_content)
            
        print(f"CLEANED: {filepath}")
        return True
        
    except Exception as e:
        print(f"ERROR cleaning {filepath}: {e}")
        return False

def main():
    # Find all .💀 files in test_programs
    pattern = "/home/ghuntley/cursed/test_suite/test_programs/**/*.💀"
    files = glob.glob(pattern, recursive=True)
    
    print(f"Found {len(files)} CURSED test files to clean...")
    
    cleaned_count = 0
    for filepath in sorted(files):
        if cleanup_cursed_file(filepath):
            cleaned_count += 1
    
    print(f"\nProcessed {len(files)} files, cleaned {cleaned_count} files")

if __name__ == "__main__":
    main()
