#!/usr/bin/env python3
"""
Quick build fix script to resolve the most common compilation errors in CURSED.
This script targets the most frequently occurring error patterns.
"""

import re
import os
import glob

def fix_duplicate_imports_in_file(file_path):
    """Remove duplicate imports from a file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        # Track seen imports
        seen_imports = set()
        new_lines = []
        
        for line in lines:
            # Check if this is an import line
            if line.strip().startswith('pub use ') or line.strip().startswith('use '):
                # Extract the import content
                import_content = line.strip()
                if import_content not in seen_imports:
                    seen_imports.add(import_content)
                    new_lines.append(line)
                else:
                    print(f"Removing duplicate import: {import_content}")
            else:
                new_lines.append(line)
        
        # Write back if changed
        if len(new_lines) != len(lines):
            with open(file_path, 'w', encoding='utf-8') as f:
                f.writelines(new_lines)
            return True
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
    
    return False

def fix_common_import_errors(file_path):
    """Fix common import patterns"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original = content
        
        # Fix common crypto import errors
        content = re.sub(r'x25519_dalek::StaticSecret', 'x25519_dalek::EphemeralSecret', content)
        content = re.sub(r'rand::distributions::Normal', 'rand_distr::Normal', content)
        content = re.sub(r'p256::ecdh', 'p256::elliptic_curve::ecdh', content)
        content = re.sub(r'time::PrimitiveDateTime', 'time::OffsetDateTime', content)
        content = re.sub(r'pem::encode', 'pem::Pem::new', content)
        content = re.sub(r'pem::parse', 'pem::parse_many', content)
        
        # Comment out problematic imports temporarily
        content = re.sub(r'use oid_registry::OID_REGISTRY;', '// use oid_registry::OID_REGISTRY;', content)
        content = re.sub(r'use x509_parser::crl;', '// use x509_parser::crl;', content)
        content = re.sub(r'use webpki::certificate;', '// use webpki::certificate;', content)
        
        if content != original:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
    
    return False

def main():
    """Fix the most common build errors"""
    print("🔧 Running quick build fixes...")
    
    # Fix optimization module duplicates
    opt_mod_path = 'src/optimization/mod.rs'
    if os.path.exists(opt_mod_path):
        if fix_duplicate_imports_in_file(opt_mod_path):
            print(f"Fixed duplicate imports in {opt_mod_path}")
    
    # Fix common import errors across crypto files
    crypto_patterns = [
        'src/stdlib/packages/crypto_*/**/*.rs',
        'src/stdlib/web_vibez/*.rs'
    ]
    
    files_fixed = 0
    
    for pattern in crypto_patterns:
        for file_path in glob.glob(pattern, recursive=True):
            if os.path.isfile(file_path) and fix_common_import_errors(file_path):
                files_fixed += 1
                print(f"Fixed imports in {file_path}")
    
    print(f"\n✅ Fixed {files_fixed} files with import issues")
    print("Running cargo check to see remaining errors...")
    
    # Run cargo check to see current status
    os.system("./fix_linking.sh cargo check --message-format=short 2>&1 | grep 'error\\[' | wc -l")

if __name__ == "__main__":
    main()
