#!/usr/bin/env python3

import os
import re
import subprocess

def fix_errno_in_file(filepath):
    """Fix errno handling in a single file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Pattern 1: let errno = unsafe { *libc::__errno_location() };
        pattern1 = r'let errno = unsafe \{ \*libc::__errno_location\(\) \};'
        replacement1 = 'let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);'
        content = re.sub(pattern1, replacement1, content)
        
        # Pattern 2: let errno = *libc::__errno_location();
        pattern2 = r'let errno = \*libc::__errno_location\(\);'
        replacement2 = 'let errno = std::io::Error::last_os_error().raw_os_error().unwrap_or(-1);'
        content = re.sub(pattern2, replacement2, content)
        
        # Pattern 3: unsafe { *libc::__errno_location() = 0; }
        pattern3 = r'unsafe \{ \*libc::__errno_location\(\) = 0; \}'
        replacement3 = '// errno cleared - not needed with std::io::Error'
        content = re.sub(pattern3, replacement3, content)
        
        # Pattern 4: unsafe { *libc::__errno_location() },
        pattern4 = r'unsafe \{ \*libc::__errno_location\(\) \},'
        replacement4 = 'std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),'
        content = re.sub(pattern4, replacement4, content)
        
        # Pattern 5: unsafe { *libc::__errno_location() }
        pattern5 = r'unsafe \{ \*libc::__errno_location\(\) \}'
        replacement5 = 'std::io::Error::last_os_error().raw_os_error().unwrap_or(-1)'
        content = re.sub(pattern5, replacement5, content)
        
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        return False
        
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def find_errno_files():
    """Find all files containing __errno_location."""
    result = subprocess.run(['grep', '-r', '-l', '__errno_location', 'src/'], 
                          capture_output=True, text=True)
    if result.returncode == 0:
        return result.stdout.strip().split('\n')
    return []

def main():
    """Main function to fix all errno issues."""
    files = find_errno_files()
    if not files:
        print("No files with __errno_location found.")
        return
    
    print(f"Found {len(files)} files with __errno_location usage:")
    fixed_count = 0
    
    for filepath in files:
        if filepath.strip() and os.path.exists(filepath):
            print(f"Fixing {filepath}...")
            if fix_errno_in_file(filepath):
                fixed_count += 1
                print(f"  ✓ Fixed {filepath}")
            else:
                print(f"  - No changes needed in {filepath}")
    
    print(f"\nFixed errno handling in {fixed_count} files.")

if __name__ == "__main__":
    main()
