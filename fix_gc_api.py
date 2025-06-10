#!/usr/bin/env python3

import os
import re
import glob

def fix_gc_api_calls(content):
    """Fix GC API calls throughout the codebase"""
    
    # Fix collect_garbage() calls to collect()
    content = re.sub(r'\.collect_garbage\(\)', '.collect().expect("Failed to collect garbage")', content)
    
    # Fix syntax errors with missing parentheses
    content = re.sub(r'GarbageCollector::new\(\);', 'GarbageCollector::new());', content)
    content = re.sub(r'\.allocate\(([^)]+)\);', r'.allocate(\1).expect("Failed to allocate");', content)
    
    # Fix incremental collection calls 
    content = re.sub(r'\.collect_garbage_incremental\(\)', '.collect().expect("Failed to collect garbage")', content)
    
    # Fix legacy collect calls
    content = re.sub(r'\.legacy_collect\(\)', '.collect().expect("Failed to collect garbage")', content)
    
    return content

def fix_file(filepath):
    """Fix a single file"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        content = fix_gc_api_calls(content)
        
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
        else:
            print(f"No changes needed: {filepath}")
            
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
                if 'GarbageCollector' in content or 'collect_garbage' in content:
                    gc_files.append(filepath)
        except:
            pass
    
    print(f"Found {len(gc_files)} files with GC usage")
    
    for filepath in gc_files:
        fix_file(filepath)

if __name__ == '__main__':
    main()
