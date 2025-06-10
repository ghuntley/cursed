#!/usr/bin/env python3
"""
Fix systematic syntax errors in CURSED test files
"""

import os
import re
import glob

def fix_unclosed_delimiters(content):
    """Fix common unclosed delimiter patterns"""
    # Fix malformed string literals with missing quotes
    content = re.sub(r'(["\'])([^"\']*)\1([^"\']*?)(["\'])', r'\1\2\4', content)
    
    # Fix mismatched parentheses in function calls
    content = re.sub(r'(\w+)\s*\(\s*([^)]*?);\s*$', r'\1(\2)', content, flags=re.MULTILINE)
    
    # Fix mismatched brackets in vector/array literals
    content = re.sub(r'vec!\[\s*([^\]]*?)\]\s*([^\]]*?)\]', r'vec![\1]', content)
    
    # Fix unclosed string literals at end of lines
    content = re.sub(r'("[^"]*$)', r'\1"', content, flags=re.MULTILINE)
    
    return content

def fix_malformed_strings(content):
    """Fix malformed string literals"""
    # Fix unterminated raw strings
    content = re.sub(r'r#"([^"]*?)$', r'r#"\1"#', content, flags=re.MULTILINE)
    
    # Fix strings with missing closing quotes
    content = re.sub(r'"([^"]*?)([^"])$', r'"\1\2"', content, flags=re.MULTILINE)
    
    # Fix escaped quotes that break strings
    content = re.sub(r'([^\\])"([^"]*?)([^\\])"', r'\1"\2\3"', content)
    
    return content

def fix_specific_patterns(content):
    """Fix specific error patterns found in the test files"""
    
    # Fix mismatched closing delimiters in function calls
    content = re.sub(r'\)\s*;\s*"\s*\}', r')";}', content)
    content = re.sub(r'\)\s*;\s*\}', r');}', content)
    
    # Fix assert! macro issues
    content = re.sub(r'assert!\s*\(([^)]*?)\s*;\s*$', r'assert!(\1)', content, flags=re.MULTILINE)
    
    # Fix println! macro issues  
    content = re.sub(r'println!\s*\(([^)]*?)\s*;\s*"\s*\}', r'println!(\1);}', content)
    
    # Fix debug! macro issues
    content = re.sub(r'debug!\s*\(([^)]*?)\s*;\s*"\s*\}', r'debug!(\1);}', content)
    content = re.sub(r'debug!\s*\(([^)]*?)\s*"\s*\)\s*"', r'debug!(\1)', content)
    
    # Fix info! macro issues
    content = re.sub(r'info!\s*\(([^)]*?)\s*;\s*"\s*\}', r'info!(\1);}', content)
    content = re.sub(r'info!\s*\(([^)]*?)\s*"\s*\)\s*"', r'info!(\1)', content)
    
    return content

def fix_struct_literals(content):
    """Fix malformed struct literal syntax"""
    # Fix missing closing braces in struct literals
    content = re.sub(r'\{\s*([^}]*?)\s*;\s*\}\s*\)', r'{\1})', content)
    
    # Fix token field assignments
    content = re.sub(r'token:\s*([^,}]*?)\s*,?\s*value:\s*([^}]*?)\s*\}\s*\}', r'token: \1, value: \2}', content)
    
    return content

def fix_test_file(filepath):
    """Fix a single test file"""
    print(f"Fixing {filepath}")
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
    except UnicodeDecodeError:
        print(f"  Warning: Could not read {filepath} as UTF-8, skipping")
        return False
    
    original_content = content
    
    # Apply fixes
    content = fix_unclosed_delimiters(content)
    content = fix_malformed_strings(content)
    content = fix_specific_patterns(content)
    content = fix_struct_literals(content)
    
    # Only write if changes were made
    if content != original_content:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"  Fixed syntax errors in {filepath}")
        return True
    else:
        print(f"  No changes needed for {filepath}")
        return False

def main():
    """Main function to fix all test files"""
    test_files = glob.glob('tests/**/*.rs', recursive=True)
    
    fixed_count = 0
    total_count = len(test_files)
    
    for test_file in test_files:
        if fix_test_file(test_file):
            fixed_count += 1
    
    print(f"\nFixed {fixed_count} out of {total_count} test files")

if __name__ == '__main__':
    main()
