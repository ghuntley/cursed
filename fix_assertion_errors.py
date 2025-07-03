#!/usr/bin/env python3

"""
Fix assertion format string errors in test_vibes modules.
"""

import os
import re
import glob

def fix_assertion_format_errors():
    """Fix assertion format string errors."""
    
    # Find all test_vibes files
    test_files = glob.glob('src/stdlib/packages/test_vibes/*.rs')
    
    for file_path in test_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Fix assertion failed patterns
            # Pattern 1: format!("Assertion failed: {:?} != {:?}")
            content = re.sub(
                r'format!\("Assertion failed: \{\:?\?\} != \{\:?\?\}"\)',
                r'format!("Assertion failed: {:?} != {:?}", left, right)',
                content
            )
            
            # Pattern 2: format!("Assertion failed: {:?} == {:?}")
            content = re.sub(
                r'format!\("Assertion failed: \{\:?\?\} == \{\:?\?\}"\)',
                r'format!("Assertion failed: {:?} == {:?}", left, right)',
                content
            )
            
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                print(f"Fixed assertion format errors in {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")

def main():
    """Main function."""
    print("🔧 Fixing assertion format string errors...")
    fix_assertion_format_errors()
    print("✅ Done! Running cargo check...")
    os.system("cargo check 2>&1 | head -20")

if __name__ == "__main__":
    main()
