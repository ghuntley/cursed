#!/usr/bin/env python3
"""
Quick fix for LLVM register numbering conflicts.
This script renumbers registers to avoid conflicts.
"""
import re
import sys

def fix_register_numbering(llvm_ir):
    lines = llvm_ir.split('\n')
    result = []
    register_map = {}
    next_register = 0
    inside_function = False
    
    for line in lines:
        # Reset register mapping for each function
        if line.strip().startswith('define'):
            inside_function = True
            register_map = {}
            next_register = 0
            result.append(line)
            continue
        elif line.strip() == '}' and inside_function:
            inside_function = False
            
        # Skip global declarations and comments
        if not inside_function or line.strip().startswith(';') or line.strip().startswith('@'):
            result.append(line)
            continue
            
        # Process the line to find register assignments
        new_line = line
        
        # Find register assignments %number =
        assign_matches = re.findall(r'%(\d+)\s*=', line)
        for match in assign_matches:
            old_reg = int(match)
            if old_reg not in register_map:
                register_map[old_reg] = next_register
                next_register += 1
        
        # Replace all register references in this line
        for old_reg, new_reg in register_map.items():
            new_line = re.sub(rf'%{old_reg}\b', f'%{new_reg}', new_line)
        
        result.append(new_line)
    
    return '\n'.join(result)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 fix_register_numbering.py file.ll")
        sys.exit(1)
    
    filename = sys.argv[1]
    
    with open(filename, 'r') as f:
        content = f.read()
    
    fixed = fix_register_numbering(content)
    
    with open(filename, 'w') as f:
        f.write(fixed)
    
    print(f"Fixed register numbering in {filename}")
