#!/usr/bin/env python3

import os
import re
import glob

def final_cleanup_cursed_file(filepath):
    """Final cleanup of CURSED file to properly structure it"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        # Remove all existing vibe and yeet lines and empty lines at start
        lines = content.split('\n')
        
        # Find the first actual content (comments or code)
        content_start = 0
        for i, line in enumerate(lines):
            stripped = line.strip()
            if stripped and not stripped.startswith('vibe ') and not stripped.startswith('yeet '):
                content_start = i
                break
        
        # Get the remaining content
        remaining_content = '\n'.join(lines[content_start:])
        
        # Determine what imports are needed
        imports = set()
        if 'vibez.spill' in remaining_content:
            imports.add('yeet "vibez"')
        if 'mathz.' in remaining_content:
            imports.add('yeet "mathz"')
        if 'stringz.' in remaining_content:
            imports.add('yeet "stringz"')
        if 'collections.' in remaining_content:
            imports.add('yeet "collections"')
        
        # Build new content
        new_content = "vibe main\n\n"
        
        # Add imports if needed
        if imports:
            for imp in sorted(imports):
                new_content += imp + '\n'
            new_content += '\n'
        
        # Add remaining content
        new_content += remaining_content
        
        # Clean up extra newlines
        new_content = re.sub(r'\n{3,}', '\n\n', new_content)
        new_content = new_content.strip() + '\n'
        
        # Write back
        with open(filepath, 'w') as f:
            f.write(new_content)
            
        print(f"FINAL CLEANUP: {filepath}")
        return True
        
    except Exception as e:
        print(f"ERROR in final cleanup {filepath}: {e}")
        return False

def main():
    # Find all .💀 files in test_programs
    pattern = "/home/ghuntley/cursed/test_suite/test_programs/**/*.💀"
    files = glob.glob(pattern, recursive=True)
    
    print(f"Found {len(files)} CURSED test files for final cleanup...")
    
    cleaned_count = 0
    for filepath in sorted(files):
        if final_cleanup_cursed_file(filepath):
            cleaned_count += 1
    
    print(f"\nFinal cleanup: processed {len(files)} files, cleaned {cleaned_count} files")

if __name__ == "__main__":
    main()
