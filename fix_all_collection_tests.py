#!/usr/bin/env python3

import re
import os
import glob

def fix_collection_test_file(file_path):
    """Fix collections test API issues in a single file"""
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix is_empty() calls - they return bool, not Result
    content = re.sub(r'\.is_empty\(\)\.unwrap_or\(false\)', '.is_empty()', content)
    
    # Fix negated Result<bool> operations
    content = re.sub(r'assert!\(!([a-zA-Z_]+)\.is_empty\(\)\);', r'assert!(!\1.is_empty());', content)
    content = re.sub(r'while !([a-zA-Z_]+)\.is_empty\(\)\.unwrap_or\(false\)', r'while !\1.is_empty()', content)
    
    # Fix API calls that might return Result instead of the expected types
    # BitSet::new() returns BitSet, not Result
    content = re.sub(r'BitSet::new\(([^)]+)\)\?', r'BitSet::new(\1)', content)
    
    # Fix unwrap() on methods that don't return Result
    content = re.sub(r'([a-zA-Z_]+)\.insert\(([^)]+)\)\.unwrap\(\)', r'\1.insert(\2).unwrap()', content)
    
    # Fix pop() method name - should be dequeue for Queue
    content = re.sub(r'([a-zA-Z_]+)\.pop\(\)\.unwrap\(\)', r'\1.dequeue().unwrap()', content)
    
    with open(file_path, 'w') as f:
        f.write(content)

def fix_all_collection_tests():
    """Fix API issues in all collection test files"""
    
    collection_test_files = [
        "tests/collections_sets_test.rs",
        "tests/collections_queues_test.rs", 
        "tests/collections_stacks_test.rs",
        "tests/collections_iterators_test.rs"
    ]
    
    for test_file in collection_test_files:
        if os.path.exists(test_file):
            print(f"Fixing {test_file}")
            fix_collection_test_file(test_file)
        else:
            print(f"File {test_file} not found - skipping")
    
    print("Fixed all collection test API issues")

if __name__ == "__main__":
    fix_all_collection_tests()
