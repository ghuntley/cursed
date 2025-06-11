#!/usr/bin/env python3

import re
import os
import glob

def fix_file(filepath):
    """Fix basic syntax errors in test files."""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix common mismatched delimiters and syntax errors
        content = re.sub(r'\}\}', '}', content)
        content = re.sub(r'\)\)', ')', content)
        content = re.sub(r'\]\]', ']', content)
        content = re.sub(r'Expected:\s+(\w+)\}', r'Expected: \1', content)
        content = re.sub(r'panic!\(Expected:\s*(\w+)\}[\'\"]*\)*', r'panic!("Expected: \1")', content)
        content = re.sub(r'[\'\"]*\)*$', '', content)
        
        # Fix malformed struct definitions
        content = re.sub(r'struct\s+(\w+)\s+\{[^}]*\}', 
                        lambda m: f'struct {m.group(1)} {{\n        // TODO: Add fields\n    }}', content)
        
        # Fix incomplete function calls and expressions
        content = re.sub(r'format!\(\{:\?\),', 'format!("{:?}",', content)
        content = re.sub(r'contains\([\"\'][^\"\']*\),', lambda m: m.group(0)[:-1] + '")', content)
        
        # Fix newline literals
        content = re.sub(r'\\n\)', r'"\\n")', content)
        
        # Remove trailing incomplete syntax
        content = re.sub(r'[;\}]\s*[;\}]+', ';', content)
        
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
        
    except Exception as e:
        print(f"Error fixing {filepath}: {e}")

def main():
    """Fix syntax errors in test files."""
    test_files = glob.glob("tests/*.rs")
    
    for filepath in test_files:
        fix_file(filepath)

if __name__ == "__main__":
    main()
