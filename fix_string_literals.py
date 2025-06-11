#!/usr/bin/env python3

import re
import glob
import os

def fix_unterminated_strings(content):
    """Fix unterminated string literals"""
    
    # Fix double quotes that are missing closing quotes
    # Pattern: ends with "" (likely missing closure)
    content = re.sub(r'(["\'])([^"\']*)\1\1$', r'\1\2\1;', content, flags=re.MULTILINE)
    
    # Fix strings that end with double quotes but missing semicolon
    content = re.sub(r'"([^"]*)""\s*$', r'"\1");', content, flags=re.MULTILINE)
    
    # Fix macro calls with unterminated strings
    content = re.sub(r'(info!|debug!|warn!|error!|println!|print!)\s*\("([^"]*)""\s*$', r'\1("\2");', content, flags=re.MULTILINE)
    
    # Fix expect calls with unterminated strings  
    content = re.sub(r'\.expect\("([^"]*)""\s*$', r'.expect("\1");', content, flags=re.MULTILINE)
    
    # Fix general unterminated strings at end of lines
    content = re.sub(r'"([^"]*)""\s*$', r'"\1";', content, flags=re.MULTILINE)
    
    return content

def fix_missing_delimiters(content):
    """Fix missing closing delimiters"""
    
    lines = content.split('\n')
    fixed_lines = []
    
    for line in lines:
        # Count opening and closing parens/braces
        open_parens = line.count('(') - line.count(')')
        open_braces = line.count('{') - line.count('}')
        
        # Add missing closing delimiters if line seems incomplete
        if line.strip() and not line.strip().endswith((';', '}', ')', ']', ',')):
            if open_parens > 0:
                line += ')' * open_parens
            if open_braces > 0:
                line += '}' * open_braces
            if not line.strip().endswith(';') and any(x in line for x in ['info!', 'debug!', 'expect']):
                line += ';'
        
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def fix_test_file(file_path):
    """Fix a single test file"""
    print(f"Fixing {file_path}...")
    
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Apply fixes
        fixed_content = fix_unterminated_strings(content)
        fixed_content = fix_missing_delimiters(fixed_content)
        
        # Only write if content changed
        if fixed_content != content:
            with open(file_path, 'w') as f:
                f.write(fixed_content)
            print(f"  ✓ Fixed {file_path}")
            return True
        else:
            print(f"  - No changes needed for {file_path}")
            return False
            
    except Exception as e:
        print(f"  ✗ Error fixing {file_path}: {e}")
        return False

def main():
    print("Fixing remaining string literal issues...")
    
    # Get list of test files that are known to have issues
    failing_tests = [
        'tests/gc_test.rs',
        'tests/interface_path_finder_test.rs', 
        'tests/core_stdlib_test.rs',
        'tests/simple_qualified_name_test.rs',
        'tests/math_constants_test.rs',
        'tests/interface_type_assertion_error_handling_test.rs',
        'tests/channel_llvm_basic_test.rs',
        'tests/gc_simple_test.rs'
    ]
    
    fixed_count = 0
    
    for test_file in failing_tests:
        if os.path.exists(test_file):
            if fix_test_file(test_file):
                fixed_count += 1
    
    print(f"\nFixed {fixed_count} files with string literal issues")

if __name__ == '__main__':
    main()
