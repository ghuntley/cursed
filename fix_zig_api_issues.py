#!/usr/bin/env python3
"""Fix Zig API compatibility issues comprehensively."""

import os
import re
import subprocess

def fix_allocator_issues(file_path):
    """Fix allocator-related compilation issues in a file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Fix common patterns:
        
        # 1. ArrayList.deinit(allocator) -> ArrayList.deinit()
        content = re.sub(r'\.deinit\(allocator\)', '.deinit()', content)
        
        # 2. ArrayList.append(allocator, item) -> ArrayList.append(item)
        content = re.sub(r'\.append\(allocator,\s*([^)]+)\)', r'.append(\1)', content)
        
        # 3. ArrayList.toOwnedSlice(allocator) -> ArrayList.toOwnedSlice()
        content = re.sub(r'\.toOwnedSlice\(allocator\)', '.toOwnedSlice()', content)
        
        # 4. ArrayList.clearAndFree(allocator) -> ArrayList.clearAndFree()
        content = re.sub(r'\.clearAndFree\(allocator\)', '.clearAndFree()', content)
        
        # 5. Fix GeneralPurposeAllocator deinit calls
        content = re.sub(r'defer _ = gpa\.deinit\(allocator\);', 'defer _ = gpa.deinit();', content)
        
        # 6. Fix errdefer calls
        content = re.sub(r'errdefer ([a-zA-Z_][a-zA-Z0-9_]*)\.deinit\(allocator\);', r'errdefer \1.deinit();', content)
        
        # 7. Fix defer calls in error handling
        content = re.sub(r'defer ([a-zA-Z_][a-zA-Z0-9_]*)\.deinit\(allocator\);', r'defer \1.deinit();', content)
        
        # 8. Handle function parameter issues - remove unused allocator parameters
        content = re.sub(r'pub fn init\(allocator: Allocator\) ([a-zA-Z_][a-zA-Z0-9_]*) \{', 
                        r'pub fn init() \1 {', content)
        
        # 9. Fix pointless discard errors by removing the discard
        content = re.sub(r'_ = allocator;\s*// Suppress unused warning\n', '', content)
        content = re.sub(r'_ = allocator;\n', '', content)
        
        # 10. Fix unused function parameter warnings
        content = re.sub(r'fn isValidAssignmentTarget\(self: \*Parser, target: Expression\)', 
                        'fn isValidAssignmentTarget(_: *Parser, target: Expression)', content)
                        
        # 11. Fix specific function signatures that don't use allocator
        content = re.sub(r'fn handleCompletion\(self: \*Self, message: json\.Value, writer: std\.io\.AnyWriter\)', 
                        'fn handleCompletion(_: *Self, message: json.Value, writer: std.io.AnyWriter)', content)
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {file_path}")
            return True
        else:
            print(f"No changes: {file_path}")
            return False
    
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    """Fix Zig API compatibility issues in all .zig files."""
    src_dir = "src-zig"
    fixed_count = 0
    total_count = 0
    
    if not os.path.exists(src_dir):
        print(f"Source directory {src_dir} not found")
        return
    
    # Find all .zig files
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if file.endswith('.zig'):
                file_path = os.path.join(root, file)
                total_count += 1
                if fix_allocator_issues(file_path):
                    fixed_count += 1
    
    print(f"\nFixed {fixed_count} files out of {total_count} total files")

if __name__ == "__main__":
    main()
