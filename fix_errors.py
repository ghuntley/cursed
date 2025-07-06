#!/usr/bin/env python3
import re

# Read the file
with open('src/type_system/checker.rs', 'r') as f:
    content = f.read()

# Pattern to match TypeCheckError instances without the required fields
pattern = r'TypeCheckError\s*{\s*([^}]+?)error_type:\s*([^,\n]+)[^}]*?}'

def replace_error(match):
    # Extract the existing fields
    prefix = match.group(1)
    error_type = match.group(2)
    
    # Build the replacement with all required fields
    return f'''TypeCheckError {{
            {prefix.strip()}
            error_type: {error_type.strip()},
            suggestions: vec!["Check variable names and types".to_string()],
            severity: ErrorSeverity::Error,
            recoverable: false,
        }}'''

# Replace all instances
new_content = re.sub(pattern, replace_error, content, flags=re.DOTALL)

# Write back to file
with open('src/type_system/checker.rs', 'w') as f:
    f.write(new_content)

print("Fixed TypeCheckError instances")
