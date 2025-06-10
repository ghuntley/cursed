#!/usr/bin/env python3

import os
import re
import glob

def fix_gc_imports(content):
    """Fix GC import statements throughout the codebase"""
    
    # Replace MemoryStats with GcStats
    content = re.sub(r'MemoryStats', 'GcStats', content)
    
    # Add missing heap_manager imports for HeapConfig
    if 'HeapConfig' in content and 'use cursed::memory::heap_manager::HeapConfig' not in content:
        # Find where other memory imports are
        import_lines = re.findall(r'^use cursed::memory::[^;]+;', content, re.MULTILINE)
        if import_lines:
            # Add HeapConfig import after the last memory import
            last_import = import_lines[-1]
            content = content.replace(last_import, last_import + '\nuse cursed::memory::heap_manager::HeapConfig;')
    
    return content

def fix_file(filepath):
    """Fix a single file"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        content = fix_gc_imports(content)
        
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Fixed imports: {filepath}")
        else:
            print(f"No import changes needed: {filepath}")
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")

def main():
    # Find all test files and source files that might use GC
    patterns = [
        'tests/*.rs',
        'src/**/*.rs',
    ]
    
    files_to_fix = []
    for pattern in patterns:
        files_to_fix.extend(glob.glob(pattern, recursive=True))
    
    # Filter to only files that likely contain GC usage
    gc_files = []
    for filepath in files_to_fix:
        try:
            with open(filepath, 'r') as f:
                content = f.read()
                if 'use cursed::memory::gc' in content or 'MemoryStats' in content:
                    gc_files.append(filepath)
        except:
            pass
    
    print(f"Found {len(gc_files)} files with GC imports")
    
    for filepath in gc_files:
        fix_file(filepath)

if __name__ == '__main__':
    main()
