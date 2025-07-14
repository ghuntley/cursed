#!/usr/bin/env python3

import re

# Read the file
with open("src/codegen/llvm/main.rs", "r") as f:
    content = f.read()

# Replace all instances of self.variable_counter with self.get_current_register_number()
# But be careful with variable_counter = assignments

# Replace direct variable_counter references (but not assignments)
content = re.sub(
    r'self\.variable_counter(?!\s*[=\+\-])',
    'self.get_current_register_number()',
    content
)

# Fix the main function register counter reset
content = re.sub(
    r'self\.variable_counter = 1;',
    '// Register counter handled by RegisterTracker::set_global_counter(1);',
    content
)

# Write the file back
with open("src/codegen/llvm/main.rs", "w") as f:
    f.write(content)

print("Fixed all variable_counter references in main.rs")
