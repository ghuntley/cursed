#!/usr/bin/env python3

import re
import os

def fix_parser_calls(file_path):
    """Fix incorrect parser method calls"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix parser.next_token() to parser.advance_token()
        content = re.sub(r'parser\.next_token\(\);(\s*//.*)?', r'parser.advance_token().unwrap();\1', content)
        
        # Fix self.next_token() to self.advance_token()
        content = re.sub(r'self\.next_token\(\);(\s*//.*)?', r'self.advance_token().unwrap();\1', content)
        
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed parser calls in {file_path}")
            return True
        else:
            print(f"No changes needed in {file_path}")
            return False
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    parser_files = [
        'src/parser/question_mark.rs',
    ]
    
    for file_path in parser_files:
        if os.path.exists(file_path):
            fix_parser_calls(file_path)
        else:
            print(f"File not found: {file_path}")

if __name__ == "__main__":
    main()
