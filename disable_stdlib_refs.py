#!/usr/bin/env python3

import os
import re
import subprocess

def disable_stdlib_imports():
    """Disable stdlib imports causing E0433 errors"""
    
    # Find files with stdlib imports
    result = subprocess.run(['grep', '-r', '-l', 'use crate::stdlib', 'src/'], 
                          capture_output=True, text=True)
    
    if result.returncode != 0:
        print("No stdlib imports found")
        return
    
    files_to_fix = result.stdout.strip().split('\n')
    
    for file_path in files_to_fix:
        if not file_path or not file_path.endswith('.rs'):
            continue
            
        print(f"Disabling stdlib imports in {file_path}")
        
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Comment out use crate::stdlib lines
            lines = content.split('\n')
            modified = False
            
            for i, line in enumerate(lines):
                if 'use crate::stdlib' in line and not line.strip().startswith('//'):
                    lines[i] = f"// {line}"
                    modified = True
                    print(f"  Commented: {line.strip()}")
            
            if modified:
                with open(file_path, 'w') as f:
                    f.write('\n'.join(lines))
                
        except Exception as e:
            print(f"Error processing {file_path}: {e}")

def disable_profiling_imports():
    """Disable profiling imports causing E0433 errors"""
    
    # Find files with profiling imports
    result = subprocess.run(['grep', '-r', '-l', 'use crate::profiling', 'src/'], 
                          capture_output=True, text=True)
    
    if result.returncode != 0:
        print("No profiling imports found")
        return
    
    files_to_fix = result.stdout.strip().split('\n')
    
    for file_path in files_to_fix:
        if not file_path or not file_path.endswith('.rs'):
            continue
            
        print(f"Disabling profiling imports in {file_path}")
        
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Comment out use crate::profiling lines
            lines = content.split('\n')
            modified = False
            
            for i, line in enumerate(lines):
                if 'use crate::profiling' in line and not line.strip().startswith('//'):
                    lines[i] = f"// {line}"
                    modified = True
                    print(f"  Commented: {line.strip()}")
            
            if modified:
                with open(file_path, 'w') as f:
                    f.write('\n'.join(lines))
                
        except Exception as e:
            print(f"Error processing {file_path}: {e}")

if __name__ == "__main__":
    print("Disabling problematic module imports...")
    disable_stdlib_imports()
    disable_profiling_imports()
    print("Done!")
