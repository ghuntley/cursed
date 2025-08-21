#!/usr/bin/env python3

import os
import re

def fix_build_file(filepath):
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Find struct/function context and fix allocator references
        lines = content.split('\n')
        new_lines = []
        
        for i, line in enumerate(lines):
            # Look for patterns where we need self.allocator instead of allocator
            if ('deinit(allocator)' in line or 'append(allocator' in line) and 'self.' in line[:50]:
                # Check if this is in a struct method context
                context_lines = lines[max(0, i-10):i+1]
                context = '\n'.join(context_lines)
                
                if 'fn ' in context and 'self: *' in context:
                    # This is likely a struct method, use self.allocator
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

# Fix specific build files that are failing
problem_files = [
    'src-zig/build_deadlock_prevention.zig',
    'src-zig/build_system_fixes.zig',
    'src-zig/linker_script_manager.zig'
]

for filepath in problem_files:
    if os.path.exists(filepath):
        fix_build_file(filepath)

print("Fixed build file allocator references!")
