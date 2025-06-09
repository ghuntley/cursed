#!/usr/bin/env python3
"""
Fix Clone derive issues for AST structs containing trait objects.

This script removes Clone derives from structs that contain Box<dyn Expression> or Box<dyn Statement>
and implements custom Clone using the clone_box method.
"""

import os
import re
from pathlib import Path

def fix_file(file_path):
    """Fix a single file by replacing Clone derives with custom implementations."""
    print(f"Processing {file_path}...")
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Pattern to find structs with Clone derive that contain trait objects
    struct_pattern = r'#\[derive\(Debug, Clone\)\]\npub struct (\w+) \{'
    
    # Find all such structs
    matches = list(re.finditer(struct_pattern, content))
    
    for match in reversed(matches):  # Process in reverse to maintain positions
        struct_name = match.group(1)
        start_pos = match.start()
        
        # Find the end of the struct
        brace_count = 0
        pos = match.end()
        struct_end = pos
        
        while pos < len(content):
            if content[pos] == '{':
                brace_count += 1
            elif content[pos] == '}':
                brace_count -= 1
                if brace_count == 0:
                    struct_end = pos + 1
                    break
            pos += 1
        
        struct_content = content[start_pos:struct_end]
        
        # Check if this struct contains trait objects
        has_trait_objects = (
            'Box<dyn Expression>' in struct_content or 
            'Box<dyn Statement>' in struct_content or
            'Vec<Box<dyn Expression>>' in struct_content or
            'Vec<Box<dyn Statement>>' in struct_content or
            'Option<Box<dyn Expression>>' in struct_content or
            'Option<Box<dyn Statement>>' in struct_content
        )
        
        if has_trait_objects:
            print(f"  Found struct {struct_name} with trait objects")
            
            # Replace the derive line
            new_derive = '#[derive(Debug)]'
            content = content[:start_pos] + content[start_pos:].replace(
                '#[derive(Debug, Clone)]', new_derive, 1
            )
            
            # Add custom Clone implementation after the struct
            clone_impl = generate_clone_impl(struct_name, struct_content)
            content = content[:struct_end] + '\n\n' + clone_impl + content[struct_end:]
    
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"  Updated {file_path}")
    else:
        print(f"  No changes needed in {file_path}")

def generate_clone_impl(struct_name, struct_content):
    """Generate a custom Clone implementation for a struct."""
    
    # Extract field names and types
    field_pattern = r'pub (\w+): ([^,}]+)'
    fields = re.findall(field_pattern, struct_content)
    
    clone_fields = []
    for field_name, field_type in fields:
        field_type = field_type.strip()
        
        if 'Box<dyn Expression>' in field_type or 'Box<dyn Statement>' in field_type:
            if 'Option<' in field_type:
                clone_fields.append(f"            {field_name}: self.{field_name}.as_ref().map(|x| x.clone_box()),")
            elif 'Vec<' in field_type:
                clone_fields.append(f"            {field_name}: self.{field_name}.iter().map(|x| x.clone_box()).collect(),")
            else:
                clone_fields.append(f"            {field_name}: self.{field_name}.clone_box(),")
        elif 'Vec<(Box<dyn Expression>, Box<dyn Expression>)>' in field_type:
            clone_fields.append(f"            {field_name}: self.{field_name}.iter().map(|(k, v)| (k.clone_box(), v.clone_box())).collect(),")
        else:
            clone_fields.append(f"            {field_name}: self.{field_name}.clone(),")
    
    clone_body = '\n'.join(clone_fields)
    
    return f"""impl Clone for {struct_name} {{
    fn clone(&self) -> Self {{
        Self {{
{clone_body}
        }}
    }}
}}"""

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
