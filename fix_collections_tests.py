#!/usr/bin/env python3
"""
Fix collections test files to use the correct API methods.
"""

import re
import glob

def fix_test_file(filename):
    """Fix a test file by correcting API usage."""
    
    with open(filename, 'r') as f:
        content = f.read()
    
    # Fix .unwrap() calls on methods that return bool
    # Just remove .unwrap() from insert calls since they now return Result<bool>
    # content = re.sub(r'\.insert\([^)]+\)\.unwrap\(\)', r'.insert(\1)', content)
    
    # Fix iterator collect to use cloned() 
    content = re.sub(r'\.iter\(\)\.collect\(\)', r'.iter().cloned().collect()', content)
    
    # Fix size() calls to use len()
    content = re.sub(r'\.size\(\)', r'.len()', content)
    
    # Fix Result<> vs Option<> mismatches in if let
    content = re.sub(r'if let Ok\(([^)]+)\) = ([^.]+)\.pop\(\)', r'if let Some(\1) = \2.pop()', content)
    content = re.sub(r'if let Ok\(([^)]+)\) = ([^.]+)\.dequeue\(\)', r'if let Some(\1) = \2.dequeue()', content)
    content = re.sub(r'if let Ok\(([^)]+)\) = ([^.]+)\.pop_front\(\)', r'if let Some(\1) = \2.pop_front()', content)
    content = re.sub(r'if let Ok\(([^)]+)\) = ([^.]+)\.pop_back\(\)', r'if let Some(\1) = \2.pop_back()', content)
    
    # Fix Err() pattern matching for Option types
    content = re.sub(r'if let Err\(e\) = ([^.]+)\.dequeue\(\)', r'if \1.dequeue().is_none()', content)
    
    # Fix .unwrap() calls on () return types - these now return Result<()>
    # So the .unwrap() is correct, no change needed for these
    
    # Fix priority queue API calls
    content = re.sub(r'\.enqueue\(([^,]+), ([^)]+)\)\.unwrap\(\)', r'.push(\1).unwrap()', content)
    content = re.sub(r'([^.]+)\.dequeue\(\)\.unwrap\(\)', r'\1.pop().unwrap()', content)
    
    # Fix BitSet::new calls
    content = re.sub(r'BitSet::new\(([^)]+)\)\.unwrap\(\)', r'BitSet::new(\1)?', content)
    
    # Fix Result vs bool issues  
    content = re.sub(r'!([^.]+)\.is_empty\(\)', r'!\1.is_empty().unwrap_or(false)', content)
    
    with open(filename, 'w') as f:
        f.write(content)

if __name__ == '__main__':
    test_files = glob.glob('tests/collections*test.rs')
    
    for filename in test_files:
        try:
            fix_test_file(filename)
            print(f"✅ Fixed {filename}")
        except Exception as e:
            print(f"❌ Error fixing {filename}: {e}")
