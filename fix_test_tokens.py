#!/usr/bin/env python3
"""
Script to fix Token compilation errors in test files.
Replaces string literals with proper Token enum variants.
"""

import os
import re
import glob
from pathlib import Path

# Mapping of common string literals to Token variants
TOKEN_MAPPINGS = {
    # Keywords
    '"be_like"': 'Token::BeLike',
    '"function"': 'Token::Slay',
    '"slay"': 'Token::Slay',
    '"sus"': 'Token::Sus',
    '"return"': 'Token::Yolo',
    '"yolo"': 'Token::Yolo',
    '"if"': 'Token::Lowkey',
    '"lowkey"': 'Token::Lowkey',
    '"mood"': 'Token::Mood',
    '"vibe_check"': 'Token::VibeCheck',
    '"based"': 'Token::Based',
    '"true"': 'Token::Based',
    '"periodt"': 'Token::Periodt',
    '"ghosted"': 'Token::Ghosted',
    '"simp"': 'Token::Simp',
    
    # Operators and delimiters
    '"{"': 'Token::LBrace',
    '"}"': 'Token::RBrace',
    '"("': 'Token::LParen',
    '")"': 'Token::RParen',
    '"="': 'Token::Assign',
    '"+"': 'Token::Plus',
    '"-"': 'Token::Minus',
    '"*"': 'Token::Asterisk',
    '"@"': 'Token::At',
    '"."': 'Token::Dot',
    
    # Types
    '"normie"': 'Token::Normie',
    '"thicc"': 'Token::Thicc',
    '"tea"': 'Token::Tea',
    '"lit"': 'Token::Lit',
    
    # Generic identifiers (these need special handling)
    '"IDENT"': 'Token::Identifier("IDENT".to_string())',
}

def fix_token_in_file(filepath):
    """Fix token assignments in a single file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Add Token import if not present
        if 'use cursed::lexer::Token;' not in content and 'token:' in content:
            # Find the last use statement and add Token import after it
            use_pattern = r'(use cursed::[^;]+;)'
            matches = list(re.finditer(use_pattern, content))
            if matches:
                last_match = matches[-1]
                insert_pos = last_match.end()
                content = content[:insert_pos] + '\nuse cursed::lexer::Token;' + content[insert_pos:]
        
        # Fix simple token assignments
        for string_literal, token_variant in TOKEN_MAPPINGS.items():
            pattern = rf'token:\s*{re.escape(string_literal)}\.to_string\(\)'
            replacement = f'token: {token_variant}'
            content = re.sub(pattern, replacement, content)
        
        # Fix identifier tokens with actual string values
        # Handle patterns like: token: "some_string".to_string()
        def replace_identifier_token(match):
            value = match.group(1)
            if value in ['IDENT', 'field', 'param']:
                return f'token: Token::Identifier("{value}".to_string())'
            elif value.isdigit():
                return f'token: Token::Int({value})'
            elif value.replace('.', '').replace('-', '').isdigit():
                return f'token: Token::Float({value})'
            elif value.startswith('"') and value.endswith('"'):
                inner_value = value[1:-1]
                return f'token: Token::String("{inner_value}".to_string())'
            else:
                # For other strings, treat as identifier
                return f'token: Token::Identifier("{value}".to_string())'
        
        # Pattern for generic string tokens
        identifier_pattern = r'token:\s*"([^"]+)"\.to_string\(\)'
        content = re.sub(identifier_pattern, replace_identifier_token, content)
        
        # Write back if changed
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
            return True
        else:
            return False
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    """Main function to fix all test files."""
    test_dir = Path("tests")
    
    if not test_dir.exists():
        print("Tests directory not found!")
        return
    
    # Find all Rust test files
    test_files = list(test_dir.glob("**/*.rs"))
    
    print(f"Found {len(test_files)} test files")
    
    fixed_count = 0
    for test_file in test_files:
        if fix_token_in_file(test_file):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files")

if __name__ == "__main__":
    main()
