#!/usr/bin/env python3

import os
import re

def fix_debug_imports():
    """Comment out problematic debug imports"""
    
    # Find all rust files
    rust_files = []
    for root, dirs, files in os.walk('src'):
        for file in files:
            if file.endswith('.rs'):
                rust_files.append(os.path.join(root, file))
    
    fixes_made = 0
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
            
            original_lines = lines[:]
            modified = False
            
            for i, line in enumerate(lines):
                # Comment out problematic debug imports
                if ('use crate::debug' in line or 
                    'use crate::runtime::debug_info' in line or
                    'use crate::runtime::debug_manager' in line) and not line.strip().startswith('//'):
                    lines[i] = f"// {line}"
                    modified = True
            
            if modified:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.writelines(lines)
                fixes_made += 1
                print(f"Fixed debug imports in {file_path}")
        
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
    
    print(f"Fixed {fixes_made} files")

if __name__ == "__main__":
    fix_debug_imports()
