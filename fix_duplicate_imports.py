#!/usr/bin/env python3
"""
Script to fix duplicate import errors in test files.
"""

import os
import re
from pathlib import Path

def fix_duplicate_imports(filepath):
    """Fix duplicate imports in a single file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Find all use statements
        use_pattern = r'^use\s+[^;]+;$'
        use_lines = []
        other_lines = []
        
        lines = content.split('\n')
        in_use_section = True
        
        for line in lines:
            stripped = line.strip()
            if re.match(use_pattern, stripped):
                if stripped not in use_lines:  # Remove exact duplicates
                    use_lines.append(stripped)
            elif stripped.startswith('use ') and stripped.endswith(';'):
                if stripped not in use_lines:  # Remove exact duplicates
                    use_lines.append(stripped)
            else:
                if stripped and not stripped.startswith('//') and in_use_section:
                    in_use_section = False
                other_lines.append(line)
        
        # Reconstruct content
        if use_lines:
            new_content = '\n'.join(use_lines) + '\n\n' + '\n'.join(other_lines)
        else:
            new_content = '\n'.join(other_lines)
        
        # Further remove semantic duplicates
        # Handle specific known duplicates
        patterns = [
            (r'use std::sync::Arc;\s*use std::sync::Arc;', 'use std::sync::Arc;'),
            (r'use cursed::lexer::Token;\s*use cursed::lexer::Token;', 'use cursed::lexer::Token;'),
            (r'use cursed::lexer::token::Token;\s*use cursed::lexer::Token;', 'use cursed::lexer::Token;'),
        ]
        
        for pattern, replacement in patterns:
            new_content = re.sub(pattern, replacement, new_content, flags=re.MULTILINE)
        
        # Write back if changed
        if new_content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
            print(f"Fixed: {filepath}")
            return True
        else:
            return False
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    """Main function to fix all test files."""
    test_dir = Path("tests")
    
    if not test_dir.exists():
        print("Tests directory not found!")
        return
    
    # Find all Rust test files
    test_files = list(test_dir.glob("**/*.rs"))
    
    print(f"Found {len(test_files)} test files")
    
    fixed_count = 0
    for test_file in test_files:
        if fix_duplicate_imports(test_file):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files")

if __name__ == "__main__":
    main()
