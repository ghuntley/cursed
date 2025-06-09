#!/usr/bin/env python3
import re
import os

def fix_crypto_mod_file(file_path):
    """Remove inline module definitions that conflict with file-based modules."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Find where stub implementations start
        stub_marker = "// Stub implementations for the imported modules"
        alt_marker = "pub mod "
        
        lines = content.split('\n')
        keep_lines = []
        in_stub_section = False
        
        for i, line in enumerate(lines):
            if stub_marker in line:
                in_stub_section = True
                break
            elif line.strip().startswith("pub mod ") and "{" in line:
                # Found inline module definition, stop here
                break
            keep_lines.append(line)
        
        if len(keep_lines) < len(lines):
            # Remove trailing empty lines
            while keep_lines and not keep_lines[-1].strip():
                keep_lines.pop()
            
            new_content = '\n'.join(keep_lines) + '\n'
            
            with open(file_path, 'w') as f:
                f.write(new_content)
            
            print(f"Cleaned up: {file_path}")
            return True
        return False
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

# Files to fix
crypto_mod_files = [
    "src/stdlib/packages/crypto_advanced/mod.rs",
    "src/stdlib/packages/crypto_asymmetric/mod.rs", 
    "src/stdlib/packages/crypto_kdf/mod.rs",
    "src/stdlib/packages/crypto_pki/mod.rs",
    "src/stdlib/packages/crypto_pqc/mod.rs",
    "src/stdlib/packages/crypto_protocols/mod.rs",
    "src/stdlib/packages/crypto_random/mod.rs",
    "src/stdlib/packages/crypto_signatures/mod.rs",
    "src/stdlib/packages/crypto_zk/mod.rs",
]

fixed_count = 0
for file_path in crypto_mod_files:
    if os.path.exists(file_path):
        if fix_crypto_mod_file(file_path):
            fixed_count += 1
    else:
        print(f"File not found: {file_path}")

print(f"Cleaned up {fixed_count} crypto module files")
