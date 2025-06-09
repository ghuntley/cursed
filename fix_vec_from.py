#!/usr/bin/env python3
import re
import os
import sys

def fix_vec_from_file(file_path):
    """Fix Vec::from([value; size]) to vec![value; size] in a file."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Pattern to match Vec::from([value; size]) where size might be a variable
        pattern = r'Vec::from\(\[([^;]+);\s*([^]]+)\]\)'
        replacement = r'vec![\1; \2]'
        
        new_content = re.sub(pattern, replacement, content)
        
        if new_content != content:
            with open(file_path, 'w') as f:
                f.write(new_content)
            print(f"Fixed: {file_path}")
            return True
        return False
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

# Files to fix
files_to_fix = [
    "src/stdlib/crypto/symmetric.rs",
    "src/stdlib/crypto/random.rs", 
    "src/stdlib/crypto/utils.rs",
    "src/stdlib/packages/crypto_hash_advanced/mod.rs",
    "src/stdlib/packages/web_vibez/server.rs",
    "src/stdlib/database/postgres/statement.rs",
    "src/stdlib/packages/crypto_advanced/aes_gcm.rs",
    "src/stdlib/packages/crypto_advanced/nonce_generator.rs"
]

fixed_count = 0
for file_path in files_to_fix:
    if os.path.exists(file_path):
        if fix_vec_from_file(file_path):
            fixed_count += 1
    else:
        print(f"File not found: {file_path}")

print(f"Fixed {fixed_count} files")
