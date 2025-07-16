#!/usr/bin/env python3
"""
Fix testz import system issues across stdlib modules.
Standardizes all testz imports to use 'yeet "testz"' format.
"""

import os
import re
import glob

def fix_testz_imports():
    """Fix all testz import variations in stdlib modules"""
    stdlib_path = "stdlib"
    
    # Patterns to find and replace
    patterns = [
        (r'yeet\s+"\.\.\/testz\/mod"', 'yeet "testz"'),
        (r'yeet\s+"testz\/mod"', 'yeet "testz"'),
        (r'yeet\s+"testz\/mod_enhanced"', 'yeet "testz"'),
        (r'yeet\s+"testz\/mod_enhanced_simple"', 'yeet "testz"'),
        (r'yeet\s+"testz\/mod_enhanced_production"', 'yeet "testz"'),
        (r'yeet\s+"enhanced_testz"', 'yeet "testz"'),
        (r'yeet\s+"testz_simple"', 'yeet "testz"'),
    ]
    
    total_files = 0
    fixed_files = 0
    
    # Find all .csd files in stdlib
    for root, dirs, files in os.walk(stdlib_path):
        for file in files:
            if file.endswith('.csd'):
                filepath = os.path.join(root, file)
                total_files += 1
                
                try:
                    with open(filepath, 'r', encoding='utf-8') as f:
                        content = f.read()
                    
                    original_content = content
                    
                    # Apply all patterns
                    for pattern, replacement in patterns:
                        content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
                    
                    # Write back if changed
                    if content != original_content:
                        with open(filepath, 'w', encoding='utf-8') as f:
                            f.write(content)
                        fixed_files += 1
                        print(f"Fixed: {filepath}")
                
                except Exception as e:
                    print(f"Error processing {filepath}: {e}")
    
    print(f"\nSummary:")
    print(f"Total files processed: {total_files}")
    print(f"Files fixed: {fixed_files}")

if __name__ == "__main__":
    fix_testz_imports()
