#!/usr/bin/env python3
"""
Fix all syntax errors in AST files caused by malformed Clone implementations.
"""

import os
import re

def fix_file(file_path):
    """Fix all syntax errors in a single file."""
    print(f"Processing {file_path}...")
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Fix "}l Node" -> "}\n\nimpl Node"
    content = re.sub(r'\}l (Node|Statement|Expression)', r'}\n\nimpl \1', content)
    
    # Fix "}   pub fn" -> "}\n    pub fn" 
    content = re.sub(r'\}\s+pub fn', r'}\n    \n    pub fn', content)
    
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"  Updated {file_path}")
    else:
        print(f"  No changes needed in {file_path}")

def main():
    """Process all AST files."""
    ast_files = [
        'src/ast/expressions.rs',
        'src/ast/statements.rs', 
        'src/ast/declarations.rs',
        'src/ast/literals.rs',
        'src/ast/operators.rs',
        'src/ast/conditionals.rs',
        'src/ast/types.rs',
        'src/ast/identifiers.rs',
        'src/ast/block.rs',
        'src/ast/calls.rs',
        'src/ast/struct_expr.rs',
        'src/ast/if_expression.rs',
        'src/ast/dot_expression.rs',
        'src/ast/slice_literal.rs',
        'src/ast/concurrency.rs',
        'src/ast/mod.rs'
    ]
    
    for file_path in ast_files:
        if os.path.exists(file_path):
            fix_file(file_path)
        else:
            print(f"File not found: {file_path}")

if __name__ == '__main__':
    main()
