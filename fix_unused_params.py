#!/usr/bin/env python3

import re
import os

def fix_unused_allocator_params(filename):
    with open(filename, 'r') as f:
        content = f.read()
    
    # Replace unused allocator parameters with _
    patterns = [
        (r'pub fn deinit\(self: \*[^,]+, allocator: Allocator\)', r'pub fn deinit(self: *\g<1>, _: Allocator)'),
        (r'pub fn ([^(]+)\([^)]*allocator: Allocator[^)]*\) [^{]*{[^}]*_ = allocator;', 
         r'pub fn \1(\g<1>_: Allocator\g<2>) \g<3>{\g<4>'),
    ]
    
    original_content = content
    
    # More specific pattern matching
    lines = content.split('\n')
    modified_lines = []
    
    for line in lines:
        # Check if line has unused allocator parameter
        if 'allocator: Allocator' in line and ('_ = allocator' in line or line.strip().endswith('_ = self;')):
            # Replace allocator with _
            line = re.sub(r'allocator: Allocator', '_: Allocator', line)
        
        modified_lines.append(line)
    
    modified_content = '\n'.join(modified_lines)
    
    if modified_content != original_content:
        with open(filename, 'w') as f:
            f.write(modified_content)
        print(f"Fixed unused parameters in {filename}")

# Fix the main files
if __name__ == "__main__":
    for root, dirs, files in os.walk('src-zig/'):
        for file in files:
            if file.endswith('.zig'):
                filepath = os.path.join(root, file)
                fix_unused_allocator_params(filepath)
