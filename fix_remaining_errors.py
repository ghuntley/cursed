#!/usr/bin/env python3
import re

# Read the file
with open('src/type_system/checker.rs', 'r') as f:
    content = f.read()

# Pattern to match incomplete TypeCheckError instances
# Look for TypeCheckError { ... error_type: ... } without the required fields
pattern = r'TypeCheckError\s*{\s*(.*?)error_type:\s*([^,\n]+)[^}]*?}\)'

def replace_error(match):
    prefix_content = match.group(1).strip()
    error_type = match.group(2).strip()
    
    # Extract message and location if they exist
    message_match = re.search(r'message:\s*([^,\n]+)', prefix_content)
    location_match = re.search(r'location:\s*([^,\n]+)', prefix_content)
    
    message = message_match.group(1) if message_match else '"Type check error".to_string()'
    location = location_match.group(1) if location_match else 'None'
    
    return f'''TypeCheckError {{
            message: {message},
            location: {location},
            error_type: {error_type},
            suggestions: vec!["Check variable names and types".to_string()],
            severity: ErrorSeverity::Error,
            recoverable: false,
        }})'''

# Replace all instances
new_content = re.sub(pattern, replace_error, content, flags=re.DOTALL)

# Write back to file
with open('src/type_system/checker.rs', 'w') as f:
    f.write(new_content)

print("Fixed remaining TypeCheckError instances")
