#!/usr/bin/env python3

import os
import re
import glob

def fix_syntax_errors(content):
    """Fix common syntax errors in test files"""
    
    # Fix unterminated string literals and escape sequences
    content = re.sub(r'(?<!\\)"([^"]*?)\\([^\\])', r'"\1\\\\\2', content)
    
    # Fix broken string concatenations and unclosed quotes
    content = re.sub(r'"([^"]*?)"([^"]*?)"([^"]*?)"', r'"\1\2\3"', content)
    
    # Fix missing closing parentheses in format! and similar macros
    content = re.sub(r'format!\("([^"]*?)"([^)]*?)""', r'format!("\1\2")', content)
    content = re.sub(r'assert!\(([^)]*?)""', r'assert!(\1)', content)
    content = re.sub(r'println!\(([^)]*?)""', r'println!(\1)', content)
    content = re.sub(r'debug!\(([^)]*?)""', r'debug!(\1)', content)
    content = re.sub(r'error!\(([^)]*?)""', r'error!(\1)', content)
    content = re.sub(r'panic!\(([^)]*?)""', r'panic!(\1)', content)
    
    # Fix mismatched braces and brackets
    content = re.sub(r'\{([^{}]*?)\]', r'{\1}', content)
    content = re.sub(r'\[([^[\]]*?)\}', r'[\1]', content)
    content = re.sub(r'\(([^()]*?)\}', r'(\1)', content)
    content = re.sub(r'\{([^{}]*?)\)', r'{\1}', content)
    
    # Fix unterminated string literals at line endings
    content = re.sub(r'"([^"\n]*?)""$', r'"\1"', content, flags=re.MULTILINE)
    
    # Fix string literals with embedded quotes
    content = re.sub(r'"([^"]*?)([A-Za-z_][A-Za-z0-9_]*?)"([^"]*?)"', r'"\1\2\3"', content)
    
    # Fix missing semicolons at end of statements
    content = re.sub(r'(\w+\([^)]*\))\s*$', r'\1;', content, flags=re.MULTILINE)
    
    # Fix broken prefixed strings
    content = re.sub(r'([a-zA-Z_][a-zA-Z0-9_]*)"([^"]*?)"', r'"\1\2"', content)
    
    # Fix missing closing delimiters
    lines = content.split('\n')
    fixed_lines = []
    
    for line in lines:
        # Count brackets, braces, and parentheses
        open_paren = line.count('(') - line.count(')')
        open_brace = line.count('{') - line.count('}')
        open_bracket = line.count('[') - line.count(']')
        
        # Add missing closing delimiters if needed
        while open_paren > 0:
            line += ')'
            open_paren -= 1
        while open_brace > 0:
            line += '}'
            open_brace -= 1
        while open_bracket > 0:
            line += ']'
            open_bracket -= 1
            
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def main():
    """Fix syntax errors in all test files"""
    
    test_files = glob.glob("tests/*.rs")
    
    for file_path in test_files:
        print(f"Processing {file_path}...")
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Skip if file is already valid
            if '""' not in content and not re.search(r'[{\[(][^}\])]*$', content):
                continue
                
            fixed_content = fix_syntax_errors(content)
            
            # Write back the fixed content
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
                
            print(f"  ✓ Fixed {file_path}")
            
        except Exception as e:
            print(f"  ✗ Error processing {file_path}: {e}")

if __name__ == "__main__":
    main()
