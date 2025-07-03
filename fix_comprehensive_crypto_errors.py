#!/usr/bin/env python3

import os
import re
import subprocess

def fix_all_error_types():
    """
    Fix all error type mismatches in crypto and other files comprehensively.
    """
    crypto_files = []
    
    # Find all crypto-related files that should return various error types
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
            
            print(f"Processing {file_path}...")
            
            # Pattern 1: Functions that return IOResult should use IOError, not CursedError
            if "-> IOResult" in content:
                print(f"  Found IOResult functions, fixing IOError usage...")
                # CursedError::runtime_error(&"...") -> IOError::Other("...".to_string())
                content = re.sub(
                    r'CursedError::runtime_error\(&"([^"]+)"\)',
                    r'IOError::Other("\1".to_string())',
                    content
                )
                # CursedError::runtime_error("...") -> IOError::Other("...".to_string())
                content = re.sub(
                    r'CursedError::runtime_error\("([^"]+)"\)',
                    r'IOError::Other("\1".to_string())',
                    content
                )
            
            # Pattern 2: Functions that return CryptoResult should use CryptoError
            elif "-> CryptoResult" in content or "CryptoResult<" in content:
                print(f"  Found CryptoResult functions, fixing CryptoError usage...")
                # CursedError::runtime_error(&"...") -> CryptoError::Other("...".to_string())
                content = re.sub(
                    r'CursedError::runtime_error\(&"([^"]+)"\)',
                    r'CryptoError::Other("\1".to_string())',
                    content
                )
                # CursedError::runtime_error("...") -> CryptoError::Other("...".to_string())
                content = re.sub(
                    r'CursedError::runtime_error\("([^"]+)"\)',
                    r'CryptoError::Other("\1".to_string())',
                    content
                )
                
                # Fix specific error types for better semantics
                specific_replacements = [
                    # Key generation failures
                    (r'CryptoError::Other\("Crypto hash test failed".to_string\(\)\)', 'CryptoError::KeyGenerationFailed'),
                    (r'CryptoError::Other\("Key generation test failed".to_string\(\)\)', 'CryptoError::KeyGenerationFailed'),
                    (r'CryptoError::Other\("Key generation failed".to_string\(\)\)', 'CryptoError::KeyGenerationFailed'),
                    
                    # Validation issues
                    (r'CryptoError::Other\("Random byte generation test failed".to_string\(\)\)', 'CryptoError::Other("Random byte generation test failed".to_string())'),
                    (r'CryptoError::Other\("Random bytes appear to be all zeros - RNG may be broken".to_string\(\)\)', 'CryptoError::Other("Random bytes appear to be all zeros - RNG may be broken".to_string())'),
                    
                    # Empty/invalid cases
                    (r'CryptoError::Other\("Empty polynomial".to_string\(\)\)', 'CryptoError::InvalidInput'),
                    (r'CryptoError::Other\("Cannot build tree with no leaves".to_string\(\)\)', 'CryptoError::InvalidInput'),
                    (r'CryptoError::Other\("Index out of bounds".to_string\(\)\)', 'CryptoError::InvalidInput'),
                    (r'CryptoError::Other\("Cannot interpolate empty point set".to_string\(\)\)', 'CryptoError::InvalidInput'),
                    (r'CryptoError::Other\("Polynomial degree exceeds setup limit".to_string\(\)\)', 'CryptoError::InvalidInput'),
                ]
                
                for old_pattern, new_replacement in specific_replacements:
                    content = re.sub(old_pattern, new_replacement, content)
            
            # Pattern 3: Handle validation_error calls
            content = re.sub(
                r'CursedError::validation_error\("([^"]+)"\)',
                r'CryptoError::InvalidInput',
                content
            )
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                changes_made += 1
                print(f"  ✓ Updated {file_path}")
            else:
                print(f"  - No changes needed in {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    print(f"\nCompleted crypto files! Made changes to {changes_made} files.")
    
    # Now fix specific non-crypto files with issues
    specific_fixes = [
        # Fix test_vibes files that have mismatched CryptoError usage
        ("src/stdlib/packages/test_vibes/assertions.rs", "fix_test_vibes_assertions"),
        ("src/stdlib/packages/test_vibes/utilities.rs", "fix_test_vibes_utilities"),
        ("src/stdlib/packages/test_vibes/core.rs", "fix_test_vibes_core"),
        ("src/stdlib/packages/crypto_pki/pkcs.rs", "fix_pkcs_error"),
        ("src/stdlib/packages/crypto_random/mod.rs", "fix_crypto_random_mod"),
    ]
    
    for file_path, fix_type in specific_fixes:
        if os.path.exists(file_path):
            print(f"Applying specific fix to {file_path}...")
            try:
                if fix_specific_file(file_path, fix_type):
                    changes_made += 1
                    print(f"  ✓ Fixed {file_path}")
                else:
                    print(f"  - No changes needed in {file_path}")
            except Exception as e:
                print(f"Error fixing {file_path}: {e}")
    
    print(f"\nTotal files changed: {changes_made}")
    return changes_made > 0

def fix_specific_file(file_path, fix_type):
    """Apply specific fixes to known problematic files."""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original_content = content
    
    if fix_type == "fix_test_vibes_assertions":
        # Fix the &format! issues by removing the & reference
        content = re.sub(
            r'CryptoError::Other\(&format!\(',
            r'CryptoError::Other(format!(',
            content
        )
        
    elif fix_type == "fix_test_vibes_utilities":
        # Fix the format! with ? issues and function return types
        content = re.sub(
            r'format!\("([^"]+)", "placeholder"\)\?',
            r'format!("\1", "placeholder")',
            content
        )
        # Fix the Result wrapping issues
        content = re.sub(
            r'Ok\(fs::read_to_string\(&self\.path\)\s*\.map_err\([^)]+\)\)',
            r'fs::read_to_string(&self.path)\n            .map_err(|e| CryptoError::Other(format!("Failed to read temp file: {}", "placeholder")))',
            content
        )
        content = re.sub(
            r'Ok\(fs::write\(&self\.path, content\)\s*\.map_err\([^)]+\)\)',
            r'fs::write(&self.path, content)\n            .map_err(|e| CryptoError::Other(format!("Failed to write temp file: {}", "placeholder")))',
            content
        )
        
    elif fix_type == "fix_test_vibes_core":
        # Fix the format! return type
        content = re.sub(
            r'Err\(CryptoError::Other\(format!\("([^"]+)", "placeholder"\)\)\)',
            r'Err(CryptoError::Other("\1".to_string()).into())',
            content
        )
        
    elif fix_type == "fix_pkcs_error":
        # Fix the return type mismatch
        content = re.sub(
            r'Err\(CryptoError::EncryptionFailed\)',
            r'Err(CryptoError::EncryptionFailed.into())',
            content
        )
        
    elif fix_type == "fix_crypto_random_mod":
        # Fix the return type conversion
        content = re.sub(
            r'random_bytes\(size\)',
            r'random_bytes(size).map_err(|e| CursedError::runtime_error(&format!("Crypto error: {:?}", e)))',
            content
        )
    
    if content != original_content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    return False

if __name__ == "__main__":
    if fix_all_error_types():
        print("\nRunning cargo check to verify fixes...")
        try:
            result = subprocess.run(['cargo', 'check'], capture_output=True, text=True, timeout=120)
            if result.returncode == 0:
                print("✓ Cargo check passed!")
            else:
                print("❌ Cargo check failed. Some issues remain:")
                print(result.stderr)
        except subprocess.TimeoutExpired:
            print("❌ Cargo check timed out")
        except Exception as e:
            print(f"❌ Error running cargo check: {e}")
    else:
        print("No changes were made.")
