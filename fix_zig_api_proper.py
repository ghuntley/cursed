#!/usr/bin/env python3

import os
import re

def fix_deinit_calls(content):
    """Fix .deinit() calls - they shouldn't have allocator parameter in most cases"""
    # Fix incorrect deinit calls that I added allocator to
    content = re.sub(r'\.deinit\(allocator\)', r'.deinit()', content)
    # Fix specific cases where we need self.allocator
    content = re.sub(r'self\.(\w+)\.deinit\(\)', r'self.\1.deinit(self.allocator)', content)
    return content

def fix_gpa_deinit(content):
    """Fix GeneralPurposeAllocator deinit calls"""
    content = re.sub(r'defer _ = gpa\.deinit\(allocator\);', r'defer _ = gpa.deinit();', content)
    content = re.sub(r'_ = gpa\.deinit\(allocator\);', r'_ = gpa.deinit();', content)
    return content

def fix_append_calls(content):
    """Fix append calls that were incorrectly modified"""
    # Fix double-allocator issue
    content = re.sub(r'\.append\(allocator, self\.allocator, ([^)]+)\)', r'.append(self.allocator, \1)', content)
    content = re.sub(r'\.append\(allocator, allocator, ([^)]+)\)', r'.append(allocator, \1)', content)
    
    # Fix cases where we need to use self.allocator
    content = re.sub(r'(\w+)\.append\(allocator, ([^)]+)\)', lambda m: f'{m.group(1)}.append(self.allocator, {m.group(2)})' if 'self.' in m.group(0) else m.group(0), content)
    return content

def process_file(filepath):
    """Process a single file with fixes"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply fixes
        content = fix_deinit_calls(content)
        content = fix_gpa_deinit(content)
        content = fix_append_calls(content)
        
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
    """Fix Zig API issues"""
    src_dir = "/home/ghuntley/cursed/src-zig"
    
    zig_files = []
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if file.endswith('.zig'):
                zig_files.append(os.path.join(root, file))
    
    print(f"Processing {len(zig_files)} Zig files")
    
    fixed_count = 0
    for filepath in zig_files:
        if process_file(filepath):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files")

if __name__ == "__main__":
    main()
