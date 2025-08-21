#!/usr/bin/env python3

import os
import re

def fix_file(filepath):
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix cases where 'allocator' is used but should be 'self.allocator'
        # This is context-sensitive, so we need to be careful
        lines = content.split('\n')
        new_lines = []
        
        for line in lines:
            # If line contains struct method and uses allocator without self, fix it
            if ('deinit(allocator)' in line or 'append(allocator' in line) and 'self.' in line:
                line = line.replace('deinit(allocator)', 'deinit(self.allocator)')
                line = line.replace('append(allocator,', 'append(self.allocator,')
            new_lines.append(line)
        
        content = '\n'.join(new_lines)
        
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
        
    except Exception as e:
        print(f"Error processing {filepath}: {e}")

# Process specific problem files
problem_files = [
    'src-zig/build_deadlock_prevention.zig',
    'src-zig/build_system_fixes.zig',
    'src-zig/linker_script_manager.zig'
]

for filepath in problem_files:
    if os.path.exists(filepath):
        fix_file(filepath)

print("Fixed allocator references!")
