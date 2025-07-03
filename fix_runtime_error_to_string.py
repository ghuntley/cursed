#!/usr/bin/env python3
import os
import re
import glob

def fix_runtime_error_to_string():
    """Fix all instances of runtime_error calls with .to_string()"""
    
    # Pattern to match runtime_error calls with .to_string()
    pattern = r'(CursedError::runtime_error\(.*?)\.to_string\(\)(\))'
    
    fixed_count = 0
    
    # Find all .rs files recursively
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                filepath = os.path.join(root, file)
                
                try:
                    with open(filepath, 'r', encoding='utf-8') as f:
                        content = f.read()
                    
                    original_content = content
                    
                    # Fix the pattern by removing .to_string()
                    content = re.sub(pattern, r'\1\2', content)
                    
                    if content != original_content:
                        # Count the number of fixes in this file
                        file_fixes = len(re.findall(pattern, original_content))
                        fixed_count += file_fixes
                        
                        with open(filepath, 'w', encoding='utf-8') as f:
                            f.write(content)
                        
                        print(f"Fixed {file_fixes} occurrences in {filepath}")
                
                except Exception as e:
                    print(f"Error processing {filepath}: {e}")
    
    return fixed_count

if __name__ == "__main__":
    total_fixed = fix_runtime_error_to_string()
    print(f"\nTotal occurrences fixed: {total_fixed}")
