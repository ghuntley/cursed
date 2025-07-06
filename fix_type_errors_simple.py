#!/usr/bin/env python3
import re

# Read the file
with open('src/type_system/checker.rs', 'r') as f:
    content = f.read()

# Replace the old struct initialization with the new constructor
# Look for TypeCheckError { message: ..., location: ..., error_type: ... }
old_pattern = r'TypeCheckError\s*{\s*message:\s*([^,\n]+),\s*location:\s*([^,\n]+),\s*error_type:\s*([^,\n\}]+)\s*}'

def replacement(match):
    message = match.group(1).strip()
    error_type = match.group(3).strip()
    return f'TypeCheckError::new({error_type}, {message}).with_suggestions(vec!["Check variable names and types".to_string()])'

# Apply the replacement
new_content = re.sub(old_pattern, replacement, content, flags=re.DOTALL)

# Write the result
with open('src/type_system/checker.rs', 'w') as f:
    f.write(new_content)

print("Replaced TypeCheckError constructors")
