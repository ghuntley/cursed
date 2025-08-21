#!/usr/bin/env python3
"""
Script to fix ArrayList API compatibility issues for Zig 0.15.1
This script applies the following fixes:
1. ArrayList.init(allocator) -> var list: ArrayList(T) = .empty;
2. ArrayList.append(item) -> ArrayList.append(allocator, item)
3. ArrayList.deinit() -> ArrayList.deinit(allocator)
4. ArrayList.toOwnedSlice() -> ArrayList.toOwnedSlice(allocator)
"""

import os
import re
import glob
import subprocess

def find_zig_files():
    """Find all .zig files in src-zig/ directory"""
    return glob.glob("src-zig/*.zig")

def detect_allocator_name(content, line_num):
    """Try to detect the allocator name from context around the line"""
    lines = content.split('\n')
    current_line = lines[line_num] if line_num < len(lines) else ""
    
    # Look for common patterns
    if 'self.allocator' in current_line:
        return 'self.allocator'
    elif 'ctx.allocator' in current_line:
        return 'ctx.allocator'
    elif 'allocator' in current_line:
        return 'allocator'
    
    # Look in surrounding lines for allocator usage
    start = max(0, line_num - 10)
    end = min(len(lines), line_num + 10)
    
    for i in range(start, end):
        line = lines[i]
        if 'self.allocator' in line:
            return 'self.allocator'
        elif 'ctx.allocator' in line:
            return 'ctx.allocator'
        elif ': std.mem.Allocator' in line or '= gpa.allocator()' in line:
            # Look for variable name
            match = re.search(r'(\w+)\s*[:=].*allocator', line)
            if match:
                return match.group(1)
    
    return 'allocator'  # fallback

def fix_arraylist_init(content):
    """Fix ArrayList.init(allocator) patterns"""
    # Pattern: var name = ArrayList(Type).init(allocator);
    pattern = r'var\s+(\w+)\s*=\s*std\.ArrayList\(([^)]+)\)\.init\(([^)]+)\);'
    def replace_init(match):
        var_name = match.group(1)
        type_name = match.group(2)
        return f'var {var_name}: std.ArrayList({type_name}) = .empty;'
    
    content = re.sub(pattern, replace_init, content)
    
    # Pattern: ArrayList(Type).init(allocator) (in struct initialization)
    pattern = r'ArrayList\(([^)]+)\)\.init\(([^)]+)\)'
    content = re.sub(pattern, r'.empty', content)
    
    return content

def fix_arraylist_operations(content):
    """Fix ArrayList operations that need allocator parameter"""
    lines = content.split('\n')
    fixed_lines = []
    
    for i, line in enumerate(lines):
        original_line = line
        
        # Fix .append(item) -> .append(allocator, item)
        if '.append(' in line and 'allocator,' not in line:
            # Find the ArrayList variable name
            append_match = re.search(r'(\w+)\.append\(', line)
            if append_match:
                allocator = detect_allocator_name(content, i)
                # Replace append(item) with append(allocator, item)
                line = re.sub(r'(\w+)\.append\(', f'\\1.append({allocator}, ', line)
        
        # Fix .deinit() -> .deinit(allocator)
        if '.deinit()' in line:
            allocator = detect_allocator_name(content, i)
            line = re.sub(r'(\w+)\.deinit\(\)', f'\\1.deinit({allocator})', line)
        
        # Fix .toOwnedSlice() -> .toOwnedSlice(allocator)
        if '.toOwnedSlice()' in line:
            allocator = detect_allocator_name(content, i)
            line = re.sub(r'(\w+)\.toOwnedSlice\(\)', f'\\1.toOwnedSlice({allocator})', line)
        
        # Fix .clearAndFree() -> .clearAndFree(allocator)
        if '.clearAndFree()' in line:
            allocator = detect_allocator_name(content, i)
            line = re.sub(r'(\w+)\.clearAndFree\(\)', f'\\1.clearAndFree({allocator})', line)
        
        # Fix .ensureTotalCapacity(n) -> .ensureTotalCapacity(allocator, n)
        if '.ensureTotalCapacity(' in line and 'allocator,' not in line:
            allocator = detect_allocator_name(content, i)
            line = re.sub(r'(\w+)\.ensureTotalCapacity\(', f'\\1.ensureTotalCapacity({allocator}, ', line)
        
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def fix_file(filepath):
    """Fix ArrayList API issues in a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply fixes
        content = fix_arraylist_init(content)
        content = fix_arraylist_operations(content)
        
        # Only write if changes were made
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
            return True
        else:
            print(f"No changes: {filepath}")
            return False
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    """Main function to fix all files"""
    print("Fixing ArrayList API compatibility issues...")
    
    zig_files = find_zig_files()
    fixed_count = 0
    
    for filepath in zig_files:
        if fix_file(filepath):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} files out of {len(zig_files)} total files")

if __name__ == "__main__":
    main()
