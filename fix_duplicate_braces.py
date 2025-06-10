#!/usr/bin/env python3
"""
Remove duplicate closing braces in statements.rs
"""

def fix_duplicate_braces():
    with open('src/ast/statements.rs', 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Replace patterns of "}\n\n}\n" with "}\n"
    lines = content.split('\n')
    fixed_lines = []
    
    for i, line in enumerate(lines):
        # Skip lines that are just a closing brace if:
        # 1. Previous non-empty line was also a closing brace
        # 2. Next non-empty line is an impl statement
        if line.strip() == '}':
            # Look backward for previous closing brace
            prev_brace_found = False
            for j in range(i-1, -1, -1):
                if lines[j].strip() == '':
                    continue
                elif lines[j].strip() == '}':
                    prev_brace_found = True
                    break
                else:
                    break
            
            # Look forward for impl statement
            next_impl_found = False
            for j in range(i+1, len(lines)):
                if lines[j].strip() == '':
                    continue
                elif lines[j].strip().startswith('impl '):
                    next_impl_found = True
                    break
                else:
                    break
            
            # Skip this brace if both conditions are met
            if prev_brace_found and next_impl_found:
                continue
        
        fixed_lines.append(line)
    
    content = '\n'.join(fixed_lines)
    
    if content != original_content:
        with open('src/ast/statements.rs', 'w') as f:
            f.write(content)
        print("Removed duplicate braces from statements.rs")
        return True
    
    print("No duplicate braces found in statements.rs")
    return False

if __name__ == '__main__':
    fix_duplicate_braces()
