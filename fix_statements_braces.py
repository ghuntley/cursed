#!/usr/bin/env python3
"""
Fix missing closing braces in statements.rs
"""

import re

def fix_statements_file():
    with open('src/ast/statements.rs', 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Pattern to match the specific issue:
    # Functions ending without closing brace followed by impl
    pattern = r'(\s+}\s*)\n\s*\n?impl ([A-Za-z]+) for ([A-Za-z]+) \{'
    
    def replace_match(match):
        indent = match.group(1)
        interface = match.group(2) 
        struct_name = match.group(3)
        return f'{indent}\n}}\n\nimpl {interface} for {struct_name} {{'
    
    content = re.sub(pattern, replace_match, content)
    
    # Also fix patterns where there's a missing closing brace before impl
    pattern2 = r'(\s+})\s*\n\s*pub fn ([^{]+\{[^}]+})\s*\n\s*impl ([A-Za-z]+) for ([A-Za-z]+) \{'
    def replace_match2(match):
        indent_brace = match.group(1)
        function = match.group(2)
        interface = match.group(3)
        struct_name = match.group(4)
        return f'{indent_brace}\n    \n    pub fn {function}\n}}\n\nimpl {interface} for {struct_name} {{'
    
    content = re.sub(pattern2, replace_match2, content)
    
    # Handle unclosed impl blocks more generally
    lines = content.split('\n')
    fixed_lines = []
    in_impl = False
    brace_depth = 0
    
    for i, line in enumerate(lines):
        if 'impl ' in line and ' {' in line:
            in_impl = True
            brace_depth = 1
        elif in_impl:
            brace_depth += line.count('{') - line.count('}')
            
            # If we're at the end of the impl block
            if brace_depth == 0:
                in_impl = False
            
            # If we hit another impl without proper closing
            if 'impl ' in line and brace_depth > 0:
                # Add missing closing brace before this impl
                fixed_lines.append('    }')
                fixed_lines.append('}')
                fixed_lines.append('')
                brace_depth = 1  # Reset for new impl
        
        fixed_lines.append(line)
    
    content = '\n'.join(fixed_lines)
    
    if content != original_content:
        with open('src/ast/statements.rs', 'w') as f:
            f.write(content)
        print("Fixed statements.rs brace issues")
        return True
    
    print("No changes needed in statements.rs")
    return False

if __name__ == '__main__':
    fix_statements_file()
