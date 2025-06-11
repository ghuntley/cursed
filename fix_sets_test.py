#!/usr/bin/env python3

import re

def fix_sets_test():
    """Fix collections sets test API issues"""
    
    test_file = "tests/collections_sets_test.rs"
    
    with open(test_file, 'r') as f:
        content = f.read()
    
    # Fix is_empty() calls - they return bool, not Result
    content = re.sub(r'\.is_empty\(\)\.unwrap_or\(false\)', '.is_empty()', content)
    
    # Fix negated is_empty() calls
    content = re.sub(r'!set\.is_empty\(\)\.unwrap_or\(false\)', '!set.is_empty()', content)
    content = re.sub(r'!([a-zA-Z_]+)\.is_empty\(\)', r'!\1.is_empty().unwrap_or(false)', content)
    
    # Fix Result<bool> negation issues
    content = re.sub(r'assert!\(!([a-zA-Z_]+)\.is_empty\(\)\);', r'assert!(!\1.is_empty().unwrap_or(false));', content)
    
    with open(test_file, 'w') as f:
        f.write(content)
    
    print("Fixed collections sets test API issues")

if __name__ == "__main__":
    fix_sets_test()
