#!/usr/bin/env python3

import os
import re
import subprocess

def fix_base64_in_file(filepath):
    """Fix base64 API usage in a single file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Ensure the proper import is present
        if 'use base64::{Engine as _, engine::general_purpose};' not in content:
            # Add the import after other base64 imports or at appropriate location
            if 'use base64::' in content:
                content = re.sub(
                    r'(use base64::.*?;)',
                    r'\1\nuse base64::{Engine as _, engine::general_purpose};',
                    content,
                    count=1
                )
            else:
                # Add after other use statements
                content = re.sub(
                    r'(use [^;]+;)',
                    r'\1\nuse base64::{Engine as _, engine::general_purpose};',
                    content,
                    count=1
                )
        
        # Replace base64::encode_config(data, base64::URL_SAFE_NO_PAD)
        pattern1 = r'base64::encode_config\(([^,]+),\s*base64::URL_SAFE_NO_PAD\)'
        replacement1 = r'general_purpose::URL_SAFE_NO_PAD.encode(\1)'
        content = re.sub(pattern1, replacement1, content)
        
        # Replace base64::decode_config(data, base64::URL_SAFE_NO_PAD)  
        pattern2 = r'base64::decode_config\(([^,]+),\s*base64::URL_SAFE_NO_PAD\)'
        replacement2 = r'general_purpose::URL_SAFE_NO_PAD.decode(\1)'
        content = re.sub(pattern2, replacement2, content)
        
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        return False
        
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def find_base64_files():
    """Find all files containing base64 API calls."""
    result = subprocess.run(['grep', '-r', '-l', 'encode_config\\|URL_SAFE_NO_PAD', 'src/'], 
                          capture_output=True, text=True)
    if result.returncode == 0:
        return result.stdout.strip().split('\n')
    return []

def main():
    """Main function to fix all base64 API issues."""
    files = find_base64_files()
    if not files:
        print("No files with base64 API calls found.")
        return
    
    print(f"Found {len(files)} files with base64 API usage:")
    fixed_count = 0
    
    for filepath in files:
        if filepath.strip() and os.path.exists(filepath):
            print(f"Fixing {filepath}...")
            if fix_base64_in_file(filepath):
                fixed_count += 1
                print(f"  ✓ Fixed {filepath}")
            else:
                print(f"  - No changes needed in {filepath}")
    
    print(f"\nFixed base64 API in {fixed_count} files.")

if __name__ == "__main__":
    main()
