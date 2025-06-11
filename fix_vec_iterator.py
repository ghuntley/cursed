#!/usr/bin/env python3

import re

# Read the file
with open('tests/collections_iterators_simple_test.rs', 'r') as f:
    content = f.read()

# Replace patterns that need VecIterator
# Pattern 1: vec.into_iter().method() -> VecIterator::new(vec).method()
content = re.sub(r'(\w+)\.into_iter\(\)\.', r'VecIterator::new(\1).', content)

# Pattern 2: vec.clone().into_iter().method() -> VecIterator::new(vec.clone()).method()
content = re.sub(r'(\w+)\.clone\(\)\.into_iter\(\)\.', r'VecIterator::new(\1.clone()).', content)

# Write back
with open('tests/collections_iterators_simple_test.rs', 'w') as f:
    f.write(content)

print("Fixed VecIterator usages")
