#!/usr/bin/env python3
"""
Fix "imp" lines that should be removed or fixed.
"""

import os
import re

def fix_file(file_path):
    """Fix imp issues in a single file."""
    print(f"Processing {file_path}...")
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Remove standalone "imp" lines
    content = re.sub(r'\nimp\n', '\n', content)
    content = re.sub(r'^imp\n', '', content, flags=re.MULTILINE)
    
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
