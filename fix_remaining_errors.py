#!/usr/bin/env python3

import os
import re

def fix_pointless_discard(content):
    """Remove _ = allocator; lines when allocator is actually used"""
    lines = content.split('\n')
    fixed_lines = []
    
    i = 0
    while i < len(lines):
        line = lines[i]
        if '_ = allocator;' in line:
            # Look ahead to see if allocator is used in the function
            j = i + 1
            allocator_used = False
            while j < len(lines) and not (lines[j].strip().startswith('pub fn') or lines[j].strip().startswith('fn ') or lines[j].strip() == '}'):
                if 'allocator' in lines[j] and '_ = allocator;' not in lines[j]:
                    allocator_used = True
                    break
                j += 1
            
            if not allocator_used:
                fixed_lines.append(line)  # Keep the discard
            else:
                pass  # Remove the discard line
        else:
            fixed_lines.append(line)
        i += 1
    
    return '\n'.join(fixed_lines)

def fix_missing_allocator_in_append(content):
    """Fix append calls missing self.allocator"""
    # Fix cases where append needs self.allocator
    content = re.sub(r'try (\w+)\.append\(allocator, ([^)]+)\)', r'try \1.append(self.allocator, \2)', content)
    
    # Fix cases in methods where we have self
    content = re.sub(r'try (self\.\w+)\.append\(allocator, ([^)]+)\)', r'try \1.append(self.allocator, \2)', content)
    
    return content

def fix_malformed_prints(content):
    """Fix malformed print statements with extra braces"""
    # Fix double braces in print statements
    content = re.sub(r'print\("([^"]*)", \.{([^}]*)\)', r'print("\1", .\2)', content)
    return content

def fix_unused_captures(content):
    """Fix unused captures in catch expressions"""
    content = re.sub(r'catch \|err\| {', r'catch {', content)
    return content

def fix_struct_syntax(content):
    """Fix struct syntax issues"""
    # Fix trailing commas in struct initialization
    content = re.sub(r'},\s*\n\s*}', r'}\n        }', content)
    return content

def process_file(filepath):
    """Process a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply fixes
        content = fix_pointless_discard(content)
        content = fix_missing_allocator_in_append(content)
        content = fix_malformed_prints(content)
        content = fix_unused_captures(content)
        content = fix_struct_syntax(content)
        
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
    """Fix remaining compilation errors"""
    src_dir = "/home/ghuntley/cursed/src-zig"
    
    # Focus on files that are causing build errors
    error_files = [
        'main.zig',
        'ast.zig', 
        'parser.zig',
        'interpreter.zig',
        'enhanced_compiler.zig',
        'error_handling.zig',
        'hygienic_macro_system.zig',
        'llvm_real.zig',
        'ast_advanced.zig'
    ]
    
    fixed_count = 0
    for filename in error_files:
        filepath = os.path.join(src_dir, filename)
        if os.path.exists(filepath):
            if process_file(filepath):
                fixed_count += 1
    
    print(f"Fixed {fixed_count} critical files")

if __name__ == "__main__":
    main()
