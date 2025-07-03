#!/usr/bin/env python3

import os
import re
import subprocess

def fix_crypto_error_files():
    """
    Fix all occurrences of CursedError::runtime_error in crypto files that should return CryptoError.
    """
    # List all crypto-related files that should return CryptoError
    crypto_files = []
    
    # Find all crypto_ directories in src/stdlib/packages/
    crypto_dirs = [
        "crypto_asymmetric",
        "crypto_pqc", 
        "crypto_signatures",
        "crypto_kdf",
        "crypto_zk",
        "crypto_random",
        "crypto_hash_advanced",
        "crypto_pki"
    ]
    
    base_path = "src/stdlib/packages"
    
    for crypto_dir in crypto_dirs:
        dir_path = os.path.join(base_path, crypto_dir)
        if os.path.exists(dir_path):
            for root, dirs, files in os.walk(dir_path):
                for file in files:
                    if file.endswith('.rs'):
                        crypto_files.append(os.path.join(root, file))
    
    # Also include specific crypto files in the packages root
    crypto_root_files = [
        os.path.join(base_path, "crypto_advanced.rs"),
        os.path.join(base_path, "crypto_security_warnings.rs"),
    ]
    
    for file in crypto_root_files:
        if os.path.exists(file):
            crypto_files.append(file)
    
    print(f"Found {len(crypto_files)} crypto-related files to process")
    
    changes_made = 0
    
    for file_path in crypto_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Check if this file has functions that return CryptoResult
            if "-> CryptoResult" in content or "CryptoResult<" in content:
                print(f"Processing {file_path}...")
                
                # Pattern 1: CursedError::runtime_error(&"...")
                content = re.sub(
                    r'CursedError::runtime_error\(&"([^"]+)"\)',
                    r'CryptoError::Other("\1".to_string())',
                    content
                )
                
                # Pattern 2: CursedError::runtime_error("...")
                content = re.sub(
                    r'CursedError::runtime_error\("([^"]+)"\)',
                    r'CryptoError::Other("\1".to_string())',
                    content
                )
                
                # Pattern 3: CursedError::runtime_error(&format!(...))
                content = re.sub(
                    r'CursedError::runtime_error\(&format!\(([^)]+)\)\)',
                    r'CryptoError::Other(format!(\1))',
                    content
                )
                
                # Pattern 4: CursedError::runtime_error(format!(...))
                content = re.sub(
                    r'CursedError::runtime_error\(format!\(([^)]+)\)\)',
                    r'CryptoError::Other(format!(\1))',
                    content
                )
                
                # Pattern 5: CursedError::runtime_error(&&format!(...))
                content = re.sub(
                    r'CursedError::runtime_error\(&&format!\(([^)]+)\)\)',
                    r'CryptoError::Other(format!(\1))',
                    content
                )
                
                # Specific cases for meaningful error types
                specific_replacements = [
                    # Key generation failures
                    (r'CryptoError::Other\("Key generation test failed".to_string\(\)\)', 'CryptoError::KeyGenerationFailed'),
                    (r'CryptoError::Other\("Key generation failed".to_string\(\)\)', 'CryptoError::KeyGenerationFailed'),
                    
                    # Encryption/Decryption failures  
                    (r'CryptoError::Other\("Encryption failed".to_string\(\)\)', 'CryptoError::EncryptionFailed'),
                    (r'CryptoError::Other\("Decryption failed".to_string\(\)\)', 'CryptoError::DecryptionFailed'),
                    
                    # Signature failures
                    (r'CryptoError::Other\("Signature failed".to_string\(\)\)', 'CryptoError::SignatureFailed'),
                    (r'CryptoError::Other\("Verification failed".to_string\(\)\)', 'CryptoError::VerificationFailed'),
                    
                    # Invalid input
                    (r'CryptoError::Other\("Invalid input".to_string\(\)\)', 'CryptoError::InvalidInput'),
                    (r'CryptoError::Other\("Invalid key".to_string\(\)\)', 'CryptoError::InvalidInput'),
                    (r'CryptoError::Other\("Invalid data".to_string\(\)\)', 'CryptoError::InvalidInput'),
                ]
                
                for old_pattern, new_replacement in specific_replacements:
                    content = re.sub(old_pattern, new_replacement, content)
                
                if content != original_content:
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(content)
                    changes_made += 1
                    print(f"  ✓ Updated {file_path}")
                else:
                    print(f"  - No changes needed in {file_path}")
            else:
                print(f"  - Skipping {file_path} (no CryptoResult functions)")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    print(f"\nCompleted! Made changes to {changes_made} files.")
    return changes_made > 0

if __name__ == "__main__":
    if fix_crypto_error_files():
        print("\nRunning cargo check to verify fixes...")
        try:
            result = subprocess.run(['cargo', 'check'], capture_output=True, text=True, timeout=120)
            if result.returncode == 0:
                print("✓ Cargo check passed!")
            else:
                print("❌ Cargo check failed:")
                print(result.stderr)
        except subprocess.TimeoutExpired:
            print("❌ Cargo check timed out")
        except Exception as e:
            print(f"❌ Error running cargo check: {e}")
    else:
        print("No changes were made.")
