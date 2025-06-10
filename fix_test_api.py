#!/usr/bin/env python3

import os
import re
import glob

def fix_test_api(content):
    """Fix test API calls to match the updated GC API"""
    
    # Fix Traceable trait implementations to only have trace method
    content = re.sub(
        r'impl Traceable for (\w+) \{[^}]*fn size\(&self\) -> usize \{[^}]*\}[^}]*fn tag\(&self\) -> Tag \{[^}]*\}[^}]*fn trace\(&self, visitor: &mut dyn Visitor\) \{[^}]*\}[^}]*\}',
        r'impl Traceable for \1 {\n        fn trace(&self, visitor: &mut dyn Visitor) {\n            // Trace any references this object contains\n        }\n    }',
        content,
        flags=re.DOTALL
    )
    
    # Fix method calls that don't exist anymore
    content = re.sub(r'\.ptr\(\)', '.object_id()', content)
    content = re.sub(r'\.inner\(\)', '.as_ref()', content)
    
    # Fix test objects to implement both Traceable and Storable requirements
    content = re.sub(
        r'(impl Traceable for \w+ \{[^}]*\})',
        r'\1\n\nunsafe impl Send for TestObject {}\nunsafe impl Sync for TestObject {}',
        content,
        flags=re.DOTALL
    )
    
    return content

def fix_file(filepath):
    """Fix a single file"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        content = fix_test_api(content)
        
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Fixed test API: {filepath}")
        else:
            print(f"No test API changes needed: {filepath}")
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")

def main():
    # Find test files that might need fixing
    patterns = [
        'tests/gc_*.rs',
        'tests/simple_gc_*.rs',
        'tests/comprehensive_gc_*.rs',
        'tests/standalone_gc_*.rs',
        'tests/memory_*.rs',
        'tests/weak_*.rs',
    ]
    
    files_to_fix = []
    for pattern in patterns:
        files_to_fix.extend(glob.glob(pattern, recursive=True))
    
    print(f"Found {len(files_to_fix)} test files to fix")
    
    for filepath in files_to_fix:
        fix_file(filepath)

if __name__ == '__main__':
    main()
