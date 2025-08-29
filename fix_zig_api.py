#!/usr/bin/env python3

import os
import re
import glob

def fix_arraylist_init(content):
    """Fix ArrayList.init() calls to work with new Zig API"""
    # Pattern: ArrayList(Type).init(allocator)
    pattern1 = r'ArrayList\(([^)]+)\)\.init\(([^)]+)\)'
    replacement1 = r'ArrayList(\1){}'
    
    content = re.sub(pattern1, replacement1, content)
    
    # Pattern: std.ArrayList(Type).init(allocator)
    pattern2 = r'std\.ArrayList\(([^)]+)\)\.init\(([^)]+)\)'
    replacement2 = r'std.ArrayList(\1){}'
    
    content = re.sub(pattern2, replacement2, content)
    
    return content

def fix_arraylist_methods(content):
    """Fix ArrayList method calls that need allocator parameter"""
    # Fix .append() calls
    content = re.sub(r'\.append\(([^,)]+)\)', r'.append(allocator, \1)', content)
    content = re.sub(r'\.append\(self\.allocator, ([^,)]+)\)', r'.append(self.allocator, \1)', content)
    
    # Fix .deinit() calls
    content = re.sub(r'(\w+)\.deinit\(\)', r'\1.deinit(allocator)', content)
    
    return content

def fix_hashmap_init(content):
    """Fix HashMap.init() calls"""
    # HashMap.init(allocator) pattern
    pattern = r'HashMap\([^)]+\)\.init\(([^)]+)\)'
    
    def replace_hashmap(match):
        allocator = match.group(1)
        return match.group(0).replace('.init(' + allocator + ')', '{}')
    
    content = re.sub(pattern, replace_hashmap, content)
    return content

def fix_file_writer(content):
    """Fix file writer calls"""
    content = re.sub(r'(\w+)\.writeAll\(', r'\1.writer().writeAll(', content)
    content = re.sub(r'(\w+)\.print\(', r'\1.writer().print(', content)
    return content

def fix_childprocess(content):
    """Fix ChildProcess API calls"""
    # std.process.execv -> std.ChildProcess.exec
    content = re.sub(r'std\.process\.execv', r'std.ChildProcess.exec', content)
    return content

def fix_enum_literals(content):
    """Fix enum literal issues"""
    content = re.sub(r'var (\w+) = \.empty;', r'var \1 = std.ArrayList(u8){};', content)
    return content

def fix_calling_convention(content):
    """Fix calling convention issues"""
    content = re.sub(r'callconv\(\.C\)', r'callconv(.c)', content)
    return content

def process_file(filepath):
    """Process a single file with all fixes"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_arraylist_init(content)
        content = fix_arraylist_methods(content)
        content = fix_hashmap_init(content)
        content = fix_file_writer(content)
        content = fix_childprocess(content)
        content = fix_enum_literals(content)
        content = fix_calling_convention(content)
        
        # Only write if changed
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
            return True
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False
    
    return False

def main():
    """Main function to fix all Zig files"""
    src_dir = "/home/ghuntley/cursed/src-zig"
    
    # Get all .zig files
    zig_files = []
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if file.endswith('.zig'):
                zig_files.append(os.path.join(root, file))
    
    print(f"Found {len(zig_files)} Zig files to process")
    
    fixed_count = 0
    for filepath in zig_files:
        if process_file(filepath):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files")

if __name__ == "__main__":
    main()
