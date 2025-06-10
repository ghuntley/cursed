#!/usr/bin/env python3

import re

# Read the file
with open('tests/gc_test.rs', 'r') as f:
    content = f.read()

# Fix the specific pattern in gc_obj access
content = re.sub(
    r'gc_obj\s*\n\s*\.as_ref\(\)\s*\n\s*\.unwrap\(\)\s*\n\s*\.next',
    'gc_obj\n                .next',
    content,
    flags=re.MULTILINE
)

# Write the file back
with open('tests/gc_test.rs', 'w') as f:
    f.write(content)

print("Fixed gc_test.rs")
