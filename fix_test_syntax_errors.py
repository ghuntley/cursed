#!/usr/bin/env python3

import os
import re
import glob

def fix_syntax_errors(content):
    """Fix common syntax errors in test files"""
    
    # Fix init_tracing!() calls to include common::tracing::
    content = re.sub(r'\binit_tracing!\(\)', 'common::tracing::init_tracing!()', content)
    
    # Fix missing semicolons after use statements
    content = re.sub(r'(use [^;]+)(?<!;)(\n)', r'\1;\2', content)
    
    # Fix missing closing parentheses in function calls
    content = re.sub(r'\.unwrap\(\)(?!;)', '.unwrap()', content)
    
    # Fix missing closing parentheses in assertions with string literals
    content = re.sub(r'assert!\(([^)]+)"([^"]*)"([^)]*)$', r'assert!(\1"\2"\3)', content, flags=re.MULTILINE)
    
    # Fix missing closing parentheses in assertions  
    content = re.sub(r'assert_eq!\(([^)]+)(?<!;)(\n)', r'assert_eq!(\1);\2', content)
    content = re.sub(r'assert!\(([^)]+)(?<!;)(\n)', r'assert!(\1);\2', content)
    
    # Fix malformed string literals with escapes
    content = re.sub(r'""([^"]*)"', r'"\1"', content)
    content = re.sub(r'"([^"]*)"([^"]*)"', r'"\1\2"', content)
    
    # Fix invalid token prefixes like squad", use", etc
    content = re.sub(r'([a-zA-Z_]+)"([^"]*)"', r'\1 "\2"', content)
    
    # Fix missing closing braces in structs
    content = re.sub(r'(\w+)\s*{([^}]*)(?<!})(\n)', r'\1 {\2}\3', content)
    
    # Fix missing closing parentheses in generic calls
    content = re.sub(r'new_container\[([^\]]+)\]\(([^)]+)(?!\))', r'new_container[\1](\2)', content)
    
    # Fix trim() calls with wrong syntax
    content = re.sub(r'\.trim\(\)";', '.trim();', content)
    
    # Fix missing semicolons in let statements
    content = re.sub(r'(let [^=]+ = [^;]+)(?<!;)(\n)', r'\1;\2', content)
    
    # Fix string concatenation issues
    content = re.sub(r'\+ "([^"]*)"([^"]*)"', r'+ "\1\2"', content)
    
    # Fix format! macro usage
    content = re.sub(r'format!\(\)([^)]*)', r'format!(\1)', content)
    
    # Fix missing closing delimiters in vec! macros
    content = re.sub(r'vec!\[([^\]]+)(?!\])', r'vec![\1]', content)
    
    # Fix string literal issues with backslashes
    content = re.sub(r'\\"([^"]*)\\"', r'"\1"', content)
    
    # Fix missing closing parentheses in function definitions
    content = re.sub(r'fn ([^(]+)\(([^)]+)(?!\))\s*{', r'fn \1(\2) {', content)
    
    # Fix format string issues
    content = re.sub(r'format!\("([^"]*)", ([^,)]+),([^)]*)\)', r'format!("\1", \2\3)', content)
    
    return content

def fix_file(filepath):
    """Fix syntax errors in a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            original_content = f.read()
        
        fixed_content = fix_syntax_errors(original_content)
        
        if fixed_content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
            print(f"Fixed: {filepath}")
            return True
        else:
            print(f"No changes: {filepath}")
            return False
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    """Fix syntax errors in all test files"""
    
    test_files = glob.glob("tests/**/*.rs", recursive=True)
    
    fixed_count = 0
    total_count = len(test_files)
    
    print(f"Processing {total_count} test files...")
    
    for filepath in test_files:
        if fix_file(filepath):
            fixed_count += 1
    
    print(f"\nFixed {fixed_count} out of {total_count} files")

if __name__ == "__main__":
    main()
