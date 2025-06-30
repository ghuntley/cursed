#!/usr/bin/env python3

import os
import re

def fix_syntax_errors():
    """Fix syntax errors introduced by regex replacements"""
    
    files_to_fix = [
        "src/stdlib/packages/crypto_kdf/mod.rs",
        "src/stdlib/packages/crypto_hash_advanced/mod.rs"
    ]
    
    for file_path in files_to_fix:
        if os.path.exists(file_path):
            with open(file_path, "r") as f:
                content = f.read()
                
            # Fix duplicate pub keywords
            content = re.sub(r'pub\s+pub\s+', 'pub ', content)
            
            # Fix orphaned pub keywords
            content = re.sub(r'^pub\s*$', '', content, flags=re.MULTILINE)
            
            # Fix empty use statements
            lines = content.split('\n')
            fixed_lines = []
            for line in lines:
                if line.strip() == 'pub' or line.strip() == 'use ;':
                    continue
                if 'pub use *;' in line:
                    continue
                fixed_lines.append(line)
                
            content = '\n'.join(fixed_lines)
            
            # Remove empty lines that were left behind
            content = re.sub(r'\n\s*\n\s*\n', '\n\n', content)
            
            with open(file_path, "w") as f:
                f.write(content)
                
            print(f"✓ Fixed syntax errors in {file_path}")

def main():
    print("Fixing syntax errors...")
    fix_syntax_errors()
    print("Done!")

if __name__ == "__main__":
    main()
