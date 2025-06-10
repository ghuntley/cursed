#!/usr/bin/env python3

import re
import os

def fix_expect_token_calls(file_path):
    """Fix expect_token calls to use the correct signature"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Pattern to match expect_token calls with references
        patterns = [
            # self.expect_token(&TokenType::Something)
            (r'self\.expect_token\(&TokenType::([^)]+)\)', r'self.expect_token(TokenType::\1)?'),
            
            # expect_token(&TokenType::Something)
            (r'expect_token\(&TokenType::([^)]+)\)', r'expect_token(TokenType::\1)?'),
            
            # Fix cases where we check boolean return
            (r'if !self\.expect_token\(TokenType::([^)]+)\)\?\s*{', r'if self.expect_token(TokenType::\1).is_err() {'),
            (r'if !expect_token\(TokenType::([^)]+)\)\?\s*{', r'if expect_token(TokenType::\1).is_err() {'),
        ]
        
        original_content = content
        for pattern, replacement in patterns:
            content = re.sub(pattern, replacement, content)
        
        # Additional fixes for specific patterns
        content = re.sub(r'if !self\.expect_token\(TokenType::([^)]+)\)\.is_ok\(\)\s*{', 
                        r'if self.expect_token(TokenType::\1).is_err() {', content)
        
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed expect_token calls in {file_path}")
            return True
        else:
            print(f"No changes needed in {file_path}")
            return False
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    parser_files = [
        'src/parser/mod_parser_statements.rs',
        'src/parser/mod_parser_expressions.rs', 
        'src/parser/result_types.rs'
    ]
    
    for file_path in parser_files:
        if os.path.exists(file_path):
            fix_expect_token_calls(file_path)
        else:
            print(f"File not found: {file_path}")

if __name__ == "__main__":
    main()
