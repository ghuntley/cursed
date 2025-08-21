#!/usr/bin/env python3
"""Fix unused function parameter issues."""

import os
import re

def fix_unused_parameters(file_path):
    """Fix unused function parameters by prefixing them with underscore."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix unused allocator parameters by prefixing with underscore
        content = re.sub(r'pub fn init\(allocator: Allocator,', 'pub fn init(_: Allocator,', content)
        content = re.sub(r'pub fn deinit\(self: \*[a-zA-Z_][a-zA-Z0-9_]*, allocator: Allocator\)', 
                        r'pub fn deinit(self: *\g<0>, _: Allocator)', content)
        content = re.sub(r'deinit\(self: \*([a-zA-Z_][a-zA-Z0-9_]*), allocator: Allocator\)', 
                        r'deinit(self: *\1, _: Allocator)', content)
        content = re.sub(r'addMultiplePath\(self: \*([a-zA-Z_][a-zA-Z0-9_]*), allocator: Allocator,', 
                        r'addMultiplePath(self: *\1, _: Allocator,', content)
        content = re.sub(r'addSelectiveItem\(self: \*([a-zA-Z_][a-zA-Z0-9_]*), allocator: Allocator,', 
                        r'addSelectiveItem(self: *\1, _: Allocator,', content)
                        
        # Fix specific HashMap init pattern
        content = re.sub(r'\.init\(allocator\)', '.init(_allocator)', content)
        
        # Fix allocator field assignment
        content = re.sub(r'\.allocator = allocator,', '.allocator = _allocator,', content)
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {file_path}")
            return True
        else:
            return False
    
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """Fix unused parameters in key files that have remaining issues."""
    problem_files = [
        'src-zig/advanced_import_resolver.zig',
        'src-zig/ast.zig'
    ]
    
    for file_path in problem_files:
        if os.path.exists(file_path):
            fix_unused_parameters(file_path)

if __name__ == "__main__":
    main()
