#!/usr/bin/env python3

import re

def fix_bitset_usage():
    """Fix BitSet API usage in tests"""
    
    test_file = "tests/collections_integration_test.rs"
    
    with open(test_file, 'r') as f:
        content = f.read()
    
    # Fix union/intersection - they return Result<BitSet>
    content = re.sub(
        r'let ([a-zA-Z_]+) = bit_set\.union\(&([^)]+)\);',
        r'let \1 = bit_set.union(&\2).unwrap();',
        content
    )
    content = re.sub(
        r'let ([a-zA-Z_]+) = bit_set\.intersection\(&([^)]+)\);',
        r'let \1 = bit_set.intersection(&\2).unwrap();',
        content
    )
    
    # Fix count() calls on Result types
    content = re.sub(r'\.union\([^)]+\)\.count\(\)', '.union(args).unwrap().count()', content)
    content = re.sub(r'\.intersection\([^)]+\)\.count\(\)', '.intersection(args).unwrap().count()', content)
    
    # Fix is_set method calls - should be get
    content = re.sub(r'\.is_set\(', '.get(', content)
    
    # Fix test methods checking bits in BitSet
    # Look for patterns like bit_set.is_set(i) and replace with bit_set.get(i).unwrap_or(false)
    content = re.sub(r'bit_set\.get\(([^)]+)\)', r'bit_set.get(\1).unwrap_or(false)', content)
    
    with open(test_file, 'w') as f:
        f.write(content)
    
    print("Fixed BitSet API usage in tests")

if __name__ == "__main__":
    fix_bitset_usage()
