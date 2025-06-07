#!/usr/bin/env python3
"""
Script to fix only the correct Token enum usages.
Some structs use token: String, others use token: Token.
"""

import os
import re
from pathlib import Path

def find_struct_definitions():
    """Find which structs use token: Token vs token: String"""
    token_structs = {}
    
    # Search for struct definitions
    for rust_file in Path("src").glob("**/*.rs"):
        try:
            with open(rust_file, 'r') as f:
                content = f.read()
                
            # Find struct definitions with token fields
            struct_pattern = r'pub struct (\w+)\s*\{[^}]*token:\s*(\w+)(?:\([^)]*\))?,?[^}]*\}'
            matches = re.findall(struct_pattern, content, re.DOTALL)
            
            for struct_name, token_type in matches:
                token_structs[struct_name] = token_type
                
        except Exception as e:
            continue
    
    return token_structs

def fix_token_usage(filepath, token_structs):
    """Fix token usage based on the correct type for each struct."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Find struct instantiations and fix token field accordingly
        for struct_name, token_type in token_structs.items():
            if token_type == "String":
                # Convert Token::* back to string for structs that expect String
                pattern = rf'({struct_name}\s*\{{[^}}]*token:\s*)Token::\w+(?:\([^)]*\))?'
                
                def replace_with_string(match):
                    prefix = match.group(1)
                    # Use a simple string token
                    return prefix + '"token".to_string()'
                
                content = re.sub(pattern, replace_with_string, content, flags=re.DOTALL)
        
        # Write back if changed
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        else:
            return False
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    """Main function."""
    # Find which structs use which token types
    token_structs = find_struct_definitions()
    print(f"Found {len(token_structs)} structs with token fields:")
    for name, typ in token_structs.items():
        print(f"  {name}: {typ}")
    
    test_dir = Path("tests")
    if not test_dir.exists():
        print("Tests directory not found!")
        return
    
    # Find all Rust test files
    test_files = list(test_dir.glob("**/*.rs"))
    print(f"Found {len(test_files)} test files")
    
    fixed_count = 0
    for test_file in test_files:
        if fix_token_usage(test_file, token_structs):
            fixed_count += 1
            print(f"Fixed: {test_file}")
    
    print(f"Fixed {fixed_count} files")

if __name__ == "__main__":
    main()
