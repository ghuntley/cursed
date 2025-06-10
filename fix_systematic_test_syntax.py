#!/usr/bin/env python3
"""
Systematic fix for CURSED test syntax errors.
Fixes common patterns like mismatched delimiters, corrupted strings, etc.
"""

import os
import re
import glob
import sys

def fix_basic_syntax_errors(content):
    """Fix basic syntax errors in Rust test files."""
    
    # Fix mismatched delimiters at line endings
    content = re.sub(r'\#\[cfg\(test\)\]\]', '#[cfg(test)]', content)
    content = re.sub(r'\#\[test\]\)', '#[test]', content)
    content = re.sub(r'fn test_[^{(]*\([^)]*\}', lambda m: m.group(0).replace('}', ')'), content)
    
    # Fix corrupted string patterns
    content = re.sub(r'\.to_string\(\),\s*,\s*"\)+"', '.to_string()', content)
    content = re.sub(r'",\s*"\)+', '"', content)
    content = re.sub(r'"\s*,\s*"\)', '"', content)
    content = re.sub(r'",\s*"\s*\)', '"', content)
    
    # Fix assert patterns
    content = re.sub(r'assert_eq!\([^,]+,\s*,\s*"[^"]*\)+"[^)]*\)', 
                    lambda m: 'assert_eq!({}, "expected")'.format(m.group(0).split(',')[0].split('(')[1]), content)
    
    # Fix identifier patterns
    content = re.sub(r'Identifier::new\([^)]*,\s*[",)]+\s*"\)+"', 'Identifier::new("test")', content)
    content = re.sub(r'Identifier::new\(",\s*"\)', 'Identifier::new("test")', content)
    
    # Fix use statements with malformed braces
    content = re.sub(r'use [^;]+::\{\}', lambda m: m.group(0).replace('{}', '{*}'), content)
    
    # Fix path attributes
    content = re.sub(r'#\[path = ([^]]+)\]', r'#[path = "\1"]', content)
    
    # Fix missing semicolons after statements
    content = re.sub(r'assert!\(true\)\s*\}', 'assert!(true);\n}', content)
    
    # Fix corrupted line endings with syntax artifacts
    content = re.sub(r';\s*"\)+".*$', ';', content, flags=re.MULTILINE)
    content = re.sub(r';\s*fixed"".*$', ';', content, flags=re.MULTILINE)
    
    return content

def fix_specific_patterns(content):
    """Fix specific corrupted patterns found in files."""
    
    # Fix corrupted mod declarations
    content = re.sub(r'mod integration_tests \{\}\s*use super::\*;', 
                    'mod integration_tests {\n    use super::*;', content)
    
    # Fix corrupted function signatures
    content = re.sub(r'fn test_[^{(]*\([^)]*\}([^{]*)\{', 
                    lambda m: f"fn {m.group(0).split('fn ')[1].split('(')[0]}() {{\n        {m.group(1).strip()}", content)
    
    # Fix debug/info statements
    content = re.sub(r'debug!\([^)]*;"[^)]*\)', 'debug!("Debug message");', content)
    content = re.sub(r'info!\([^)]*\)"[^)]*"', 'info!("Info message");', content)
    
    # Fix expect patterns
    content = re.sub(r'\.expect\(",\s*"\s*[^")]*\)', '.expect("operation failed")', content)
    
    return content

def fix_imports_and_modules(content):
    """Fix import statements and module paths."""
    
    # Fix malformed use statements
    lines = content.split('\n')
    fixed_lines = []
    
    for line in lines:
        # Fix use statements with malformed braces
        if 'use ' in line and '::{}' in line:
            # Extract the module path and fix it
            match = re.match(r'(\s*use\s+[^:]+)::\{\}', line)
            if match:
                line = f"{match.group(1)}::*;"
        
        # Fix use statements with broken closing braces
        if 'use ' in line and line.count('{') != line.count('}'):
            if '{' in line and '}' not in line:
                line = line.replace('{', '{*}')
        
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def fix_test_file(filepath):
    """Fix a single test file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_basic_syntax_errors(content)
        content = fix_specific_patterns(content)
        content = fix_imports_and_modules(content)
        
        # Only write if we made changes
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
            
        return False
        
    except Exception as e:
        print(f"Error fixing {filepath}: {e}")
        return False

def main():
    """Fix all test files with syntax errors."""
    test_files = glob.glob('tests/*.rs')
    
    if len(sys.argv) > 1:
        # Fix specific files
        test_files = [f for f in test_files if any(pattern in f for pattern in sys.argv[1:])]
    
    fixed_count = 0
    total_count = len(test_files)
    
    print(f"Analyzing {total_count} test files...")
    
    for filepath in test_files:
        if fix_test_file(filepath):
            fixed_count += 1
            print(f"Fixed: {os.path.basename(filepath)}")
    
    print(f"\nFixed {fixed_count} out of {total_count} test files")

if __name__ == "__main__":
    main()
