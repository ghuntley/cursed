#!/usr/bin/env python3
"""
Fix syntax errors in the AST files caused by malformed Clone implementations.
"""

import os
import re

def fix_file(file_path):
    """Fix syntax errors in a single file."""
    print(f"Processing {file_path}...")
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Fix "imp" -> "impl" errors
    content = re.sub(r'\bimp\b(?=\s+Clone)', 'impl', content)
    
    # Fix impl blocks inside impl blocks by moving them out
    # Look for impl blocks that start after a closing brace but before another impl
    
    # First, find all nested impl Clone blocks and extract them
    extracted_impls = []
    
    # Pattern to find impl Clone blocks inside other impls
    impl_clone_pattern = r'(\n\s*impl Clone for \w+ \{[^}]*(?:\{[^}]*\}[^}]*)*\})'
    
    # Find all impl Clone blocks
    clone_matches = list(re.finditer(impl_clone_pattern, content, re.DOTALL))
    
    for match in reversed(clone_matches):
        impl_text = match.group(1).strip()
        
        # Check if this is inside another impl block
        before_impl = content[:match.start()]
        
        # Count open and closed braces to see if we're inside an impl
        open_braces = before_impl.count('{')
        close_braces = before_impl.count('}')
        
        # If we have more open braces than close braces, we're likely inside an impl
        if open_braces > close_braces:
            # Extract this impl and remove it from its current location
            extracted_impls.append(impl_text)
            content = content[:match.start()] + content[match.end():]
    
    # Add extracted impl blocks at the end of the file
    if extracted_impls:
        content = content.rstrip() + '\n\n' + '\n\n'.join(extracted_impls) + '\n'
    
    # Fix incomplete impl lines by adding proper signatures
    content = re.sub(r'(\n\s*)imp(\s*$)', r'\1impl Node for Unknown {\n    fn string(&self) -> String { String::new() }\n    fn token_literal(&self) -> String { String::new() }\n}', content)
    
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
