#!/usr/bin/env python3

import re
import sys

def fix_string_literals(content):
    """Fix string literals that are being interpreted as prefixed literals"""
    
    # Pattern to match strings ending with problematic suffixes
    problematic_suffixes = [
        'html', 'css', 'js', 'csd', 'json', 'md', 'bin', 'content', 'test', 
        'fixtures', 'failed', 'passed', 'generated', 'DOCTYPE', 'tag', 'head', 
        'body', 'encoding', 'references', 'package', 'information', 'items', 
        'format', 'object', 'source', 'output', 'headers', 'yet', 'empty', 
        'directory', 'file'
    ]
    
    for suffix in problematic_suffixes:
        # Match strings ending with the problematic suffix
        pattern = f'"([^"]*{suffix})"'
        
        def replacement(match):
            original = match.group(1)
            # Add a space before the closing quote
            return f'"{original} "'
        
        content = re.sub(pattern, replacement, content)
    
    # Also fix Unicode checkmark character
    content = content.replace('"✓', '"')
    content = content.replace('"⚠', '"')
    
    return content

def main():
    filepath = "tests/documentation_integration_test.rs"
    
    with open(filepath, 'r') as f:
        content = f.read()
    
    fixed_content = fix_string_literals(content)
    
    with open(filepath, 'w') as f:
        f.write(fixed_content)
    
    print(f"Fixed string literals in {filepath}")

if __name__ == "__main__":
    main()
