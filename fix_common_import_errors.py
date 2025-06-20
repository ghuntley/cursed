#!/usr/bin/env python3
"""
Script to fix common import errors in the CURSED codebase
"""
import os
import re

def fix_value_import(file_path):
    """Fix crate::value::Value imports to crate::stdlib::value::Value"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Replace the import
        content = content.replace('use crate::value::Value;', 'use crate::stdlib::value::Value;')
        
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"Fixed: {file_path}")
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")

def fix_sysinfo_imports(file_path):
    """Fix deprecated sysinfo trait imports"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Remove deprecated trait imports
        old_imports = [
            'SystemExt, CpuExt, DiskExt, NetworkExt, ProcessExt',
            'SystemExt',
            'CpuExt', 
            'DiskExt',
            'NetworkExt',
            'ProcessExt'
        ]
        
        for old_import in old_imports:
            content = content.replace(f', {old_import}', '')
            content = content.replace(f'{old_import}, ', '')
            content = content.replace(f'{old_import}', '')
        
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"Fixed sysinfo imports: {file_path}")
    except Exception as e:
        print(f"Error fixing sysinfo in {file_path}: {e}")

def find_and_fix_files():
    """Find and fix files with common import errors"""
    
    # Files with crate::value::Value imports
    value_import_files = [
        'src/stdlib/vibez/format.rs',
        'src/stdlib/vibez/debug.rs', 
        'src/stdlib/vibez/print.rs',
        'src/stdlib/vibez/sprintf.rs'
    ]
    
    for file_path in value_import_files:
        if os.path.exists(file_path):
            fix_value_import(file_path)
    
    # Files with deprecated sysinfo imports
    sysinfo_files = [
        'src/stdlib/system/monitoring.rs'
    ]
    
    for file_path in sysinfo_files:
        if os.path.exists(file_path):
            fix_sysinfo_imports(file_path)

if __name__ == '__main__':
    find_and_fix_files()
    print("Finished fixing common import errors")
