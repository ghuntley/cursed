#!/bin/bash

# Fix ArrayList API changes
echo "Fixing ArrayList API changes..."
find src-zig/ -name "*.zig" -exec sed -i 's/\.deinit();/.deinit(allocator);/g' {} \;
find src-zig/ -name "*.zig" -exec sed -i 's/\.deinit(self\.allocator);/.deinit(allocator);/g' {} \;

# Fix stdin/stdout API changes - need to fix reader() and writer() calls that now need buffers
echo "Fixing stdio API changes..."

# Create a temp script that handles the reader/writer buffer fixes more carefully
cat > /tmp/fix_readers.py << 'EOF'
import os
import re
import sys

def fix_file(filepath):
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Fix reader() calls that need buffer
        # Pattern: .reader() -> need to add buffer
        if '.reader()' in content and 'std.fs.File' in content:
            # Add buffer declaration before the reader call
            lines = content.split('\n')
            new_lines = []
            for i, line in enumerate(lines):
                if '.reader()' in line and 'const stdin' in line:
                    # Add buffer before this line
                    indent = len(line) - len(line.lstrip())
                    new_lines.append(' ' * indent + 'var stdin_buffer: [4096]u8 = undefined;')
                    new_lines.append(line.replace('.reader()', '.reader(stdin_buffer[0..])'))
                elif '.reader()' in line:
                    new_lines.append(line.replace('.reader()', '.reader(&[_]u8{})'))
                else:
                    new_lines.append(line)
            content = '\n'.join(new_lines)
        
        # Fix writer() calls 
        if '.writer()' in content and 'std.fs.File' in content:
            lines = content.split('\n')
            new_lines = []
            for i, line in enumerate(lines):
                if '.writer()' in line and 'const stdout' in line:
                    # Add buffer before this line
                    indent = len(line) - len(line.lstrip())
                    new_lines.append(' ' * indent + 'var stdout_buffer: [4096]u8 = undefined;')
                    new_lines.append(line.replace('.writer()', '.writer(stdout_buffer[0..])'))
                elif '.writer()' in line:
                    new_lines.append(line.replace('.writer()', '.writer(&[_]u8{})'))
                else:
                    new_lines.append(line)
            content = '\n'.join(new_lines)
        
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
    
    except Exception as e:
        print(f"Error processing {filepath}: {e}")

# Process all Zig files
for root, dirs, files in os.walk('src-zig/'):
    for file in files:
        if file.endswith('.zig'):
            fix_file(os.path.join(root, file))
EOF

python3 /tmp/fix_readers.py

echo "API compatibility fixes completed!"
