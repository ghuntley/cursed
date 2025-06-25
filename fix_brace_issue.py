#!/usr/bin/env python3

import os

def fix_database_integration_braces():
    """Fix the brace mismatch in database_integration.rs"""
    
    file_path = 'src/codegen/llvm/database_integration.rs'
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    lines = content.split('\n')
    
    # Find the problematic area and fix it
    # Look for the impl block start and ensure proper closing
    in_impl_block = False
    brace_count = 0
    fixed_lines = []
    
    for i, line in enumerate(lines):
        if 'impl<\'ctx> DatabaseLlvmRegistry<\'ctx>' in line:
            in_impl_block = True
            brace_count = 0
        
        if in_impl_block:
            brace_count += line.count('{') - line.count('}')
        
        fixed_lines.append(line)
        
        # If we're at the end of the impl block
        if in_impl_block and brace_count == 0 and '}' in line:
            in_impl_block = False
    
    # Write the fixed content
    with open(file_path, 'w') as f:
        f.write('\n'.join(fixed_lines))
    
    print(f"Fixed brace issues in {file_path}")

if __name__ == '__main__':
    fix_database_integration_braces()
