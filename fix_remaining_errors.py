#!/usr/bin/env python3

import os
import re
import subprocess
from pathlib import Path

def get_specific_error_files():
    """Get files with specific Error type issues from compilation output."""
    try:
        result = subprocess.run(
            ["cargo", "build", "--message-format=short"], 
            capture_output=True, 
            text=True, 
            cwd="/home/ghuntley/code/cursed"
        )
        
        error_files = []
        current_file = None
        
        for line in result.stderr.split('\n'):
            # Look for file location lines
            if '.rs:' in line and 'error[E0412]' in line and 'cannot find type' in line and 'Error' in line:
                parts = line.split(':')
                if len(parts) >= 3:
                    file_path = parts[0].strip()
                    if file_path.startswith('src/'):
                        error_files.append(file_path)
        
        return list(set(error_files))  # Remove duplicates
    except:
        return []

def fix_error_imports_targeted(filepath):
    """Fix Error import issues in a specific file with targeted approach."""
    if not os.path.exists(filepath):
        return False
        
    with open(filepath, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Check if already has error import
    if 'use crate::error_types::Error' in content:
        return False
    
    # Check if the file actually uses Error types
    if not (re.search(r'\bError\b', content) or re.search(r'Result<[^,>]*>', content)):
        return False
    
    # Find the best position to add import
    lines = content.split('\n')
    import_pos = 0
    
    # Look for existing use statements
    for i, line in enumerate(lines):
        if line.strip().startswith('use std::'):
            import_pos = i + 1
        elif line.strip().startswith('use tracing::'):
            import_pos = i + 1
        elif line.strip().startswith('use crate::') or line.strip().startswith('use super::'):
            import_pos = i
            break
        elif line.strip() and not line.strip().startswith('//') and not line.strip().startswith('use '):
            break
    
    # Insert the import
    lines.insert(import_pos, 'use crate::error_types::Error;')
    content = '\n'.join(lines)
    
    # Save if changed
    if content != original_content:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    
    return False

def fix_specific_error_types(filepath):
    """Fix specific error type imports like CryptoError, NetError, etc."""
    if not os.path.exists(filepath):
        return False
        
    with open(filepath, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Add specific error type imports based on usage
    needs_crypto_error = 'CryptoError' in content and 'use crate::stdlib::crypto::types::CryptoError' not in content
    needs_net_error = 'NetError' in content and 'use crate::stdlib::vibe_net::error::NetError' not in content  
    needs_channel_error = 'ChannelError' in content and 'use crate::runtime::channels::ChannelError' not in content
    
    if needs_crypto_error or needs_net_error or needs_channel_error:
        lines = content.split('\n')
        import_pos = 0
        
        # Find import position
        for i, line in enumerate(lines):
            if line.strip().startswith('use crate::'):
                import_pos = i + 1
            elif line.strip() and not line.strip().startswith('//') and not line.strip().startswith('use '):
                break
        
        # Add needed imports
        if needs_crypto_error:
            lines.insert(import_pos, 'use crate::stdlib::crypto::types::CryptoError;')
            import_pos += 1
        if needs_net_error:
            lines.insert(import_pos, 'use crate::stdlib::vibe_net::error::NetError;')
            import_pos += 1
        if needs_channel_error:
            lines.insert(import_pos, 'use crate::runtime::channels::ChannelError;')
        
        content = '\n'.join(lines)
    
    # Save if changed
    if content != original_content:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    
    return False

def main():
    print("🔍 Finding files with specific Error type compilation errors...")
    
    error_files = get_specific_error_files()
    print(f"📁 Found {len(error_files)} files with Error type issues")
    
    if error_files:
        print("Files with errors:")
        for f in error_files[:10]:  # Show first 10
            print(f"  - {f}")
    
    fixed_count = 0
    
    # Fix Error imports first
    for filepath in error_files[:40]:  # Process first 40 files
        try:
            if fix_error_imports_targeted(filepath):
                print(f"✅ Fixed Error import: {filepath}")
                fixed_count += 1
        except Exception as e:
            print(f"❌ Error fixing {filepath}: {e}")
    
    # Fix specific error types
    for filepath in error_files[:40]:
        try:
            if fix_specific_error_types(filepath):
                print(f"✅ Fixed specific errors: {filepath}")
                fixed_count += 1
        except Exception as e:
            print(f"❌ Error fixing specific types in {filepath}: {e}")
    
    print(f"\n🎉 Fixed {fixed_count} files")
    
    # Final check
    print("\n🔍 Final Error type check...")
    try:
        result = subprocess.run(
            ["cargo", "build", "--message-format=short"], 
            capture_output=True, 
            text=True, 
            cwd="/home/ghuntley/code/cursed"
        )
        
        error_lines = [line for line in result.stderr.split('\n') 
                      if 'cannot find type' in line and 'Error' in line]
        print(f"📊 Remaining Error type issues: {len(error_lines)}")
        
        if error_lines:
            print("🔍 Sample remaining errors:")
            for line in error_lines[:5]:
                print(f"   {line}")
                
    except Exception as e:
        print(f"❌ Error checking build status: {e}")

if __name__ == "__main__":
    main()
