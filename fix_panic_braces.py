#!/usr/bin/env python3
"""
Fix all the brace alignment issues in panic.rs
"""

import re

def fix_panic_file():
    with open("src/runtime/panic.rs", "r") as f:
        content = f.read()
    
    # Pattern to match the problematic structure
    pattern = r'(pub extern "C" fn \w+\(\) \{)\s*// TODO: implement\s*\}\s*(.*?)\s*\}'
    
    # Replace with correct structure
    def replacement(match):
        func_decl = match.group(1)
        body = match.group(2).strip()
        if body:
            return f"{func_decl}\n    {body}\n}}"
        else:
            return f"{func_decl}\n    // TODO: implement\n}}"
    
    # Apply the fix
    content = re.sub(pattern, replacement, content, flags=re.DOTALL)
    
    # Also fix any remaining standalone TODO blocks
    content = re.sub(
        r'(\w+\(\) \{)\s*// TODO: implement\s*\}\s*(\w+)',
        r'\1\n    // TODO: implement\n}\n\n\2',
        content
    )
    
    with open("src/runtime/panic.rs", "w") as f:
        f.write(content)
    
    print("Fixed panic.rs brace issues")

if __name__ == "__main__":
    fix_panic_file()
