#!/usr/bin/env python3

import os
import re
import sys
from pathlib import Path

def fix_file(file_path):
    """Fix double >> in function return types in a single file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Pattern to match function return types with malformed double >>
        # This pattern looks for: ") -> Result<TYPE, Error>>" where there's an extra >
        patterns = [
            (r'(\) -> Result<[^>]*?, Error>)>', r'\1'),
            (r'(\) -> Result<\(\), Error>)>', r'\1'),
            (r'(\) -> Result<\(\), Box<dyn std::error::Error>)>', r'\1'),
            (r'(async fn [^(]*\([^)]*\) -> Result<[^>]*?, Error>)>', r'\1'),
            (r'(pub fn [^(]*\([^)]*\) -> Result<[^>]*?, Error>)>', r'\1'),
            (r'(fn [^(]*\([^)]*\) -> Result<[^>]*?, Error>)>', r'\1'),
        ]
        
        fixes_made = 0
        for pattern, replacement in patterns:
            new_content = re.sub(pattern, replacement, content)
            if new_content != content:
                matches = len(re.findall(pattern, content))
                fixes_made += matches
                content = new_content
        
        # Additional pattern for trait definitions
        trait_pattern = r'(async fn [^(]*\([^)]*\) -> Result<[^>]*?, Error>);'
        trait_replacement = r'\1;'
        new_content = re.sub(trait_pattern, trait_replacement, content)
        if new_content != content:
            matches = len(re.findall(r'(async fn [^(]*\([^)]*\) -> Result<[^>]*?, Error>);', content))
            fixes_made += matches
            content = new_content
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return fixes_made
        
        return 0
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return 0

def find_rust_files():
    """Find all Rust source files in the src/ directory."""
    rust_files = []
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(os.path.join(root, file))
    return rust_files

def main():
    rust_files = find_rust_files()
    total_fixes = 0
    files_fixed = 0
    
    print(f"Checking {len(rust_files)} Rust files for double >> syntax errors...")
    
    for file_path in rust_files:
        fixes = fix_file(file_path)
        if fixes > 0:
            print(f"Fixed {fixes} issues in {file_path}")
            total_fixes += fixes
            files_fixed += 1
    
    print(f"\nSummary:")
    print(f"Files fixed: {files_fixed}")
    print(f"Total fixes: {total_fixes}")
    
    return 0 if total_fixes > 0 else 1

if __name__ == "__main__":
    sys.exit(main())
