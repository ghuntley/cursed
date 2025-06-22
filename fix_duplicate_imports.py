#!/usr/bin/env python3

import re
import os
from pathlib import Path

def fix_crypto_mod_duplicates():
    """Fix duplicate imports in crypto/mod.rs"""
    file_path = "src/stdlib/crypto/mod.rs"
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix Ed25519 types - remove from asymmetric import, keep in types import
    content = re.sub(
        r'(\s+Ed25519KeyPair,) Ed25519PublicKey, Ed25519PrivateKey, Ed25519Signature,',
        r'\1',
        content
    )
    
    # Remove SecurityLevel from pqc imports - keep in types import
    content = re.sub(
        r'PqcError, PqcResult, SecurityLevel, AlgorithmType,',
        r'PqcError, PqcResult, AlgorithmType,',
        content
    )
    
    # Remove SecurityLevel alias from pqc_hybrid - keep main one
    content = re.sub(
        r'SecurityLevel as ProductionSecurityLevel, AlgorithmType as ProductionAlgorithmType,',
        r'AlgorithmType as ProductionAlgorithmType,',
        content
    )
    
    # Remove SecurityLevel alias from protocols - keep main one
    content = re.sub(
        r'ProtocolSuite, ProtocolBuilder, SecurityLevel as ProtocolSecurityLevel, ProtocolConfig,',
        r'ProtocolSuite, ProtocolBuilder, ProtocolConfig,',
        content
    )
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed duplicate imports in {file_path}")

def find_other_duplicate_imports():
    """Search for other E0252 errors in the codebase"""
    result = os.popen('./fix_linking.sh cargo check 2>&1 | grep -A 3 "E0252"').read()
    
    lines = result.split('\n')
    files_with_duplicates = set()
    
    for line in lines:
        if '-->' in line and '.rs:' in line:
            file_path = line.split('-->')[1].strip().split(':')[0]
            files_with_duplicates.add(file_path)
    
    return files_with_duplicates

def fix_specific_duplicates_in_file(file_path):
    """Fix common duplicate patterns in a specific file"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Common patterns to consolidate
        patterns = [
            # Consolidate multiple use statements from same module
            (r'use ([^;]+::\w+)::\{([^}]+)\};\s*use \1::\{([^}]+)\};', r'use \1::{\2, \3};'),
            
            # Remove duplicate individual imports
            (r'use ([^;]+);\s*use \1;', r'use \1;'),
            
            # Consolidate Error types
            (r'use ([^;]+::)Error;\s*use \1Error;', r'use \1Error;'),
        ]
        
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
        
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed duplicates in {file_path}")
            
    except Exception as e:
        print(f"Error processing {file_path}: {e}")

def main():
    print("Fixing E0252 duplicate import errors...")
    
    # Fix the main crypto module duplicates
    fix_crypto_mod_duplicates()
    
    # Find other files with duplicates
    duplicate_files = find_other_duplicate_imports()
    print(f"Found duplicate imports in: {duplicate_files}")
    
    # Fix other files
    for file_path in duplicate_files:
        if file_path and os.path.exists(file_path):
            fix_specific_duplicates_in_file(file_path)
    
    print("Duplicate import fixes completed!")

if __name__ == "__main__":
    main()
