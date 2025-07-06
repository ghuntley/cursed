#!/usr/bin/env python3
"""
Fix LLVM IR register numbering gaps
"""

import re
import sys

def fix_llvm_register_numbering(llvm_ir):
    """Fix register numbering gaps in LLVM IR"""
    
    # Find all register references
    register_pattern = r'%(\d+)'
    registers_used = set()
    
    for match in re.finditer(register_pattern, llvm_ir):
        registers_used.add(int(match.group(1)))
    
    if not registers_used:
        return llvm_ir
    
    # Sort registers and create mapping
    sorted_registers = sorted(registers_used)
    register_mapping = {}
    
    for i, old_reg in enumerate(sorted_registers):
        register_mapping[old_reg] = i
    
    # Replace registers in the IR
    def replace_register(match):
        old_num = int(match.group(1))
        new_num = register_mapping.get(old_num, old_num)
        return f'%{new_num}'
    
    fixed_ir = re.sub(register_pattern, replace_register, llvm_ir)
    return fixed_ir

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: python3 fix_register_numbering.py <llvm_file>")
        sys.exit(1)
    
    filename = sys.argv[1]
    
    try:
        with open(filename, 'r') as f:
            content = f.read()
        
        fixed_content = fix_llvm_register_numbering(content)
        
        with open(filename, 'w') as f:
            f.write(fixed_content)
        
        print(f"Fixed register numbering in {filename}")
        
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)
