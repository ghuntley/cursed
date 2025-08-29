#!/usr/bin/env python3

import os
import re

def fix_unused_parameters(content):
    """Fix unused function parameters by adding _ = param; statements"""
    # Pattern for function with unused allocator parameter
    pattern = r'(pub fn \w+\([^)]*allocator: Allocator\) [^{]*\{)'
    
    def add_underscore(match):
        func_def = match.group(1)
        return func_def + '\n        _ = allocator;'
    
    content = re.sub(pattern, add_underscore, content)
    return content

def fix_calling_conventions(content):
    """Fix calling convention issues"""
    content = re.sub(r'callconv\(\.C\)', r'callconv(.c)', content)
    return content

def fix_intcast_issues(content):
    """Fix @intCast issues that need explicit result type"""
    # Pattern: @intCast(expression) -> @intCast(u32, expression)
    content = re.sub(r'@intCast\(cpu_count\)', r'@as(u32, @intCast(cpu_count))', content)
    return content

def fix_format_specifiers(content):
    """Fix format specifier issues"""
    # Fix slice formatting without specifier
    def fix_format(match):
        fmt_str = match.group(1).replace('{}', '{s}')
        args = match.group(2)
        return f'print("{fmt_str}", .{{{args}}})'
    
    content = re.sub(r'print\("([^"]*\{}[^"]*)", \.([^}]+)\}\)', fix_format, content)
    return content

def fix_enum_literal_variables(content):
    """Fix enum literal variable issues"""
    # Pattern: var name = .empty; -> var name: ?Type = null; or similar appropriate fix
    content = re.sub(r'var (\w+) = \.empty;', r'var \1 = std.ArrayList(u8){};', content)
    content = re.sub(r'var (\w+) = \.(\w+);', r'const \1 = .\2;', content)
    return content

def fix_tuple_types(content):
    """Fix Tuple type issues"""
    # Remove .Tuple references as they don't exist in newer Zig
    content = re.sub(r'\.Tuple => \|tuple\| [^,}]+,?', '', content)
    content = re.sub(r'\.Tuple => [^,}]+,?', '', content)
    return content

def fix_variable_references(content):
    """Fix missing Variable type references"""
    # Add import for Variable if missing
    if 'Variable' in content and 'const Variable = ' not in content:
        # Try to add a simple Variable definition
        if 'pub const Value = union(enum)' in content:
            content = content.replace('pub const Value = union(enum)', 'const Variable = struct { name: []const u8, value: Value };\n\npub const Value = union(enum)')
    return content

def process_file(filepath):
    """Process a single file with all fixes"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_unused_parameters(content)
        content = fix_calling_conventions(content)
        content = fix_intcast_issues(content)
        content = fix_format_specifiers(content)
        content = fix_enum_literal_variables(content)
        content = fix_tuple_types(content)
        content = fix_variable_references(content)
        
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
    """Fix remaining Zig compilation issues"""
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
