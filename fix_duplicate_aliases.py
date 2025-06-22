#!/usr/bin/env python3
"""
Fix duplicate type alias definitions after E0659 fixes.
"""

import os
import re
from pathlib import Path

def fix_lifecycle_duplicates():
    """Fix duplicate type aliases in lifecycle.rs"""
    file_path = Path('src/stdlib/process/lifecycle.rs')
    if not file_path.exists():
        print(f"File not found: {file_path}")
        return
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Remove the duplicate type alias definitions since we already have them in imports
    content = re.sub(r'\n// Type aliases to resolve conflicts\n.*?\ntype ProcessStatus = .*?;\n', '\n', content, flags=re.DOTALL)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed duplicate type aliases in {file_path}")

def fix_integration_duplicates():
    """Fix duplicate type aliases in integration.rs"""
    file_path = Path('src/stdlib/process/integration.rs')
    if not file_path.exists():
        print(f"File not found: {file_path}")
        return
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Remove duplicate type alias definitions
    content = re.sub(r'\n// Type aliases to resolve conflicts\n.*?\ntype ProcessOutput = .*?;\n', '\n', content, flags=re.DOTALL)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed duplicate type aliases in {file_path}")

def fix_all_duplicate_aliases():
    """Fix duplicate type aliases in all process files"""
    
    process_files = [
        'src/stdlib/process/unified_process_ipc.rs',
        'src/stdlib/process/unix_platform.rs',
        'src/stdlib/process/windows_platform.rs',
        'src/stdlib/process/enhanced_exec_vibez_complete.rs',
        'src/stdlib/process/comprehensive_integration.rs',
        'src/stdlib/process/enhanced_exec_slay_complete.rs',
        'src/stdlib/process/exec_slay_complete.rs',
    ]
    
    for file_path_str in process_files:
        file_path = Path(file_path_str)
        if not file_path.exists():
            continue
        
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Remove duplicate type alias sections
        if '// Type aliases to resolve conflicts' in content:
            content = re.sub(r'\n// Type aliases to resolve conflicts\n.*?(?=\n\n|\n[a-zA-Z]|\nuse |\n#|\n\/\/[^\/]|\Z)', '\n', content, flags=re.DOTALL)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"Fixed duplicate type aliases in {file_path}")

def main():
    print("Fixing duplicate type alias definitions...")
    
    print("\n1. Fixing lifecycle duplicates...")
    fix_lifecycle_duplicates()
    
    print("\n2. Fixing integration duplicates...")
    fix_integration_duplicates()
    
    print("\n3. Fixing all other duplicate aliases...")
    fix_all_duplicate_aliases()
    
    print("\nDuplicate alias fixes completed!")

if __name__ == '__main__':
    main()
