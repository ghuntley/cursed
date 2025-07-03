#!/usr/bin/env python3

"""
Add CryptoError imports to files that use CryptoError but don't import it.
"""

import os
import glob

def add_crypto_error_import(file_path):
    """Add CryptoError import to a file if needed."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if file uses CryptoError but doesn't import it
        if 'CryptoError::' in content and 'use crate::stdlib::packages::CryptoError' not in content:
            lines = content.split('\n')
            
            # Find where to insert the import
            insert_pos = 0
            for i, line in enumerate(lines):
                if line.strip().startswith('use '):
                    insert_pos = i + 1
            
            # Insert the import
            lines.insert(insert_pos, 'use crate::stdlib::packages::CryptoError;')
            
            new_content = '\n'.join(lines)
            
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            
            print(f"Added CryptoError import to {file_path}")
            return True
        
        return False
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """Main function."""
    print("🔧 Adding CryptoError imports to files that need them...")
    
    # Find all .rs files that might need the import
    all_rs_files = []
    for root_dir in ['src/stdlib/packages', 'src/stdlib/collections', 'src/stdlib/crypto', 'src/stdlib/async', 'src/stdlib/database']:
        if os.path.exists(root_dir):
            all_rs_files.extend(glob.glob(f"{root_dir}/**/*.rs", recursive=True))
    
    fixed_count = 0
    for file_path in all_rs_files:
        if add_crypto_error_import(file_path):
            fixed_count += 1
    
    print(f"✅ Added CryptoError imports to {fixed_count} files")
    print("Running cargo check to see remaining errors...")
    os.system("cargo check 2>&1 | head -10")

if __name__ == "__main__":
    main()
