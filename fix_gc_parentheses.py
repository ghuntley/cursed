#!/usr/bin/env python3

import os
import re
import glob

def fix_gc_parentheses(content):
    """Fix mismatched parentheses in GC instantiation"""
    
    # Fix double closing parentheses from my previous script
    content = re.sub(r'GarbageCollector::new\(\)\);', 'GarbageCollector::new();', content)
    
    return content

def fix_file(filepath):
    """Fix a single file"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        content = fix_gc_parentheses(content)
        
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Fixed parentheses: {filepath}")
        else:
            print(f"No parentheses changes needed: {filepath}")
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")

def main():
    # Find all test files and source files that might use GC
    patterns = [
        'tests/*.rs',
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
                if 'GarbageCollector::new())' in content:
                    gc_files.append(filepath)
        except:
            pass
    
    print(f"Found {len(gc_files)} files with parentheses issues")
    
    for filepath in gc_files:
        fix_file(filepath)

if __name__ == '__main__':
    main()
