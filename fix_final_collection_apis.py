#!/usr/bin/env python3

import re
import os

def fix_collection_result_apis(file_path):
    """Fix Result API calls in collection tests"""
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix assert!(collection.insert(x)) - insert returns Result<bool>
    content = re.sub(r'assert!\(([a-zA-Z_]+)\.insert\(([^)]+)\)\);', r'assert!(\1.insert(\2).unwrap());', content)
    
    # Fix assert!(!collection.insert(x)) for duplicate insertions
    content = re.sub(r'assert!\(!([a-zA-Z_]+)\.insert\(([^)]+)\)\);', r'assert!(!\1.insert(\2).unwrap());', content)
    
    # Fix other potential Result<bool> operations in asserts
    content = re.sub(r'assert!\(!([a-zA-Z_]+)\.contains\(([^)]+)\)\);', r'assert!(!\1.contains(\2));', content)
    content = re.sub(r'assert!\(([a-zA-Z_]+)\.contains\(([^)]+)\)\);', r'assert!(\1.contains(\2));', content)
    
    # Fix while loops with Result<bool> operations
    content = re.sub(r'while !([a-zA-Z_]+)\.is_empty\(\)\.unwrap_or\(false\)', r'while !\1.is_empty().unwrap_or(false)', content)
    
    with open(file_path, 'w') as f:
        f.write(content)

def fix_final_collection_issues():
    """Fix remaining collection test API issues"""
    
    collection_test_files = [
        "tests/collections_sets_test.rs",
        "tests/collections_queues_test.rs", 
        "tests/collections_stacks_test.rs",
        "tests/collections_iterators_test.rs",
        "tests/collections_integration_test.rs"
    ]
    
    for test_file in collection_test_files:
        if os.path.exists(test_file):
            print(f"Fixing Result APIs in {test_file}")
            fix_collection_result_apis(test_file)
        else:
            print(f"File {test_file} not found - skipping")
    
    print("Fixed final collection test API issues")

if __name__ == "__main__":
    fix_final_collection_issues()
