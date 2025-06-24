#!/usr/bin/env python3

import os
import re
import subprocess

def get_exact_error_files():
    """Get exact files that still have Error type issues"""
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True)
        
        error_files = set()
        lines = result.stderr.split('\n')
        
        for line in lines:
            if '-->' in line and '.rs:' in line and 'cannot find type `Error` in this scope' in result.stderr:
                match = re.search(r'-->\s*([^:]+\.rs)', line)
                if match:
                    file_path = match.group(1)
                    if os.path.exists(file_path) and not file_path.startswith('/home/ghuntley/.cargo/'):
                        error_files.add(file_path)
        
        return list(error_files)
    except Exception as e:
        print(f"Error: {e}")
        return []

def force_add_error_import(file_path: str) -> bool:
    """Force add Error import to a file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check current imports
        has_error_import = any(pattern in content for pattern in [
            'use crate::error::Error',
            'use super::Error',
            'pub enum Error',
            'pub type Error',
            'use anyhow::Error'
        ])
        
        if has_error_import:
            print(f"  - Already has Error import: {file_path}")
            return False
        
        lines = content.split('\n')
        
        # Find best insertion point
        insert_index = 0
        found_use = False
        
        for i, line in enumerate(lines):
            stripped = line.strip()
            if stripped.startswith('use '):
                insert_index = i + 1
                found_use = True
            elif stripped.startswith('extern crate'):
                insert_index = i + 1
            elif found_use and (stripped.startswith('mod ') or stripped.startswith('pub mod ')):
                break
            elif found_use and stripped and not stripped.startswith('//') and not stripped.startswith('#'):
                break
        
        # Add the import
        import_line = 'use crate::error::Error;'
        lines.insert(insert_index, import_line)
        
        # Write back
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        
        print(f"  ✓ Added Error import: {file_path}")
        return True
        
    except Exception as e:
        print(f"  ✗ Error with {file_path}: {e}")
        return False

def main():
    print("🔍 Getting exact files with remaining Error issues...")
    
    error_files = get_exact_error_files()
    
    if not error_files:
        print("🎉 No Error type issues found!")
        return
    
    print(f"Found {len(error_files)} files with Error issues")
    
    # List of known problematic files
    known_files = [
        "src/stdlib/fs/watcher.rs",
        "src/stdlib/crypto/certificates.rs", 
        "src/stdlib/crypto/protocols.rs",
        "src/stdlib/crypto/types.rs",
        "src/stdlib/crypto/unified_api.rs",
        "src/stdlib/crypto/package_manager.rs",
        "src/stdlib/crypto/random.rs",
        "src/stdlib/crypto/encoding.rs",
        "src/stdlib/crypto/llvm_integration.rs",
        "src/stdlib/crypto/zk_enhanced.rs",
        "src/stdlib/crypto/format_conversions.rs",
        "src/stdlib/crypto/x448_implementation.rs",
        "src/stdlib/crypto/crypto_advanced/xchacha20_poly1305.rs",
        "src/stdlib/crypto/protocols_advanced.rs",
        "src/stdlib/crypto/protocols_enhanced.rs",
        "src/stdlib/crypto/protocols_production.rs",
        "src/stdlib/crypto/pqc.rs",
        "src/stdlib/crypto/pqc_production.rs"
    ]
    
    # Combine discovered and known files
    all_files = list(set(error_files + known_files))
    existing_files = [f for f in all_files if os.path.exists(f)]
    
    print(f"Processing {len(existing_files)} files...")
    
    fixed = 0
    for file_path in existing_files:
        if force_add_error_import(file_path):
            fixed += 1
    
    print(f"\n📊 Results:")
    print(f"   Files processed: {len(existing_files)}")
    print(f"   Files fixed: {fixed}")
    
    # Final check
    print(f"\n🧪 Final check...")
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True, timeout=60)
        
        error_count = result.stderr.count("cannot find type `Error` in this scope")
        total_errors = result.stderr.count("error:")
        
        print(f"   Error type issues: {error_count}")
        print(f"   Total compilation errors: {total_errors}")
        
        if error_count == 0:
            print("🎉 All Error type issues resolved!")
        else:
            print(f"⚠️  {error_count} Error type issues remaining")
        
    except Exception as e:
        print(f"Error in final check: {e}")

if __name__ == "__main__":
    main()
