#!/usr/bin/env python3

import os
import re
import subprocess
from pathlib import Path
from typing import List, Set

def get_files_with_error_issues():
    """Get all files that have Error type issues"""
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True)
        
        files = set()
        lines = result.stderr.split('\n')
        
        for line in lines:
            if '-->' in line and 'cannot find type `Error` in this scope' in result.stderr:
                # Extract file paths from error location lines
                match = re.search(r'-->\s*([^:]+\.rs):', line)
                if match:
                    file_path = match.group(1)
                    if os.path.exists(file_path):
                        files.add(file_path)
        
        return list(files)
    except Exception as e:
        print(f"Error getting files: {e}")
        return []

def add_error_import(file_path: str) -> bool:
    """Add appropriate Error import to a file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Skip if already has Error import
        if any(pattern in content for pattern in [
            'use crate::error::Error',
            'use super::Error', 
            'use anyhow::Error',
            'use std::error::Error',
            'pub enum Error',
            'pub type Error'
        ]):
            return False
        
        # Determine appropriate import based on file location
        if '/stdlib/crypto/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/stdlib/collections/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/stdlib/fs/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/stdlib/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/src/core/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/src/runtime/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/src/parser/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/src/codegen/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/tests/' in file_path:
            import_stmt = 'use crate::error::Error;'
        else:
            import_stmt = 'use crate::error::Error;'
        
        # Find insertion point for import
        lines = content.split('\n')
        insert_index = 0
        
        # Find the best place to insert the import
        for i, line in enumerate(lines):
            stripped = line.strip()
            if stripped.startswith('use '):
                insert_index = i + 1
            elif stripped.startswith('mod ') or stripped.startswith('pub mod '):
                insert_index = i + 1
            elif stripped and not stripped.startswith('//') and not stripped.startswith('#['):
                break
        
        # Insert the import
        lines.insert(insert_index, import_stmt)
        
        # Write back to file
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        
        return True
        
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def main():
    print("🔍 Finding all files with Error type issues...")
    
    # Get specific files mentioned in compilation errors
    specific_files = [
        "src/stdlib/collections/sorta_fresh/mod.rs",
        "src/stdlib/fs/watcher.rs",
        "src/stdlib/crypto/certificates.rs",
        "src/stdlib/crypto/protocols.rs", 
        "src/stdlib/crypto/protocols_advanced.rs",
        "src/stdlib/crypto/types.rs",
        "src/stdlib/crypto/unified_api.rs",
        "src/stdlib/crypto/package_manager.rs",
        "src/stdlib/crypto/random.rs",
        "src/stdlib/crypto/encoding.rs",
        "src/stdlib/crypto/llvm_integration.rs",
        "src/stdlib/crypto/zk_enhanced.rs",
        "src/stdlib/crypto/format_conversions.rs",
        "src/stdlib/crypto/x448_implementation.rs",
        "src/stdlib/crypto/crypto_advanced/xchacha20_poly1305.rs"
    ]
    
    # Also get files from compilation output
    compilation_files = get_files_with_error_issues()
    
    all_files = list(set(specific_files + compilation_files))
    existing_files = [f for f in all_files if os.path.exists(f)]
    
    print(f"Found {len(existing_files)} files to process")
    
    if not existing_files:
        print("🎉 No files need Error imports!")
        return
    
    fixed = 0
    skipped = 0
    
    for file_path in existing_files:
        if add_error_import(file_path):
            print(f"✓ Fixed: {file_path}")
            fixed += 1
        else:
            print(f"- Skipped: {file_path}")
            skipped += 1
    
    print(f"\n📊 Summary:")
    print(f"   Files processed: {len(existing_files)}")
    print(f"   Files fixed: {fixed}")
    print(f"   Files skipped: {skipped}")
    
    # Check compilation status
    print(f"\n🧪 Checking compilation status...")
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True, timeout=60)
        remaining_errors = result.stderr.count("cannot find type `Error` in this scope")
        
        if remaining_errors == 0:
            print("🎉 All Error type issues resolved!")
        else:
            print(f"⚠️  {remaining_errors} Error type issues remaining")
            
        total_errors = result.stderr.count("error:")
        print(f"   Total compilation errors: {total_errors}")
        
        # Show top remaining error types
        if remaining_errors > 0:
            print(f"\n🔍 Remaining Error patterns:")
            error_output = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True)
            error_types = {}
            for line in error_output.stderr.split('\n'):
                if 'cannot find type' in line and 'Error' in line:
                    match = re.search(r'cannot find type `([^`]*Error[^`]*)`', line)
                    if match:
                        error_type = match.group(1)
                        error_types[error_type] = error_types.get(error_type, 0) + 1
            
            for error_type, count in sorted(error_types.items(), key=lambda x: x[1], reverse=True)[:10]:
                print(f"     {count}x {error_type}")
        
    except subprocess.TimeoutExpired:
        print("⏰ Compilation check timed out")
    except Exception as e:
        print(f"❌ Error checking compilation: {e}")

if __name__ == "__main__":
    main()
