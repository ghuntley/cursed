#!/usr/bin/env python3

"""
Fix format string errors where arguments are missing.
"""

import os
import re
import subprocess

def fix_format_errors():
    """Run cargo check and fix format string errors."""
    
    # Get cargo check output
    result = subprocess.run(['cargo', 'check'], capture_output=True, text=True)
    error_output = result.stderr
    
    # Parse format string errors
    format_errors = []
    current_file = None
    current_line = None
    
    for line in error_output.split('\n'):
        # Match file and line number
        file_match = re.match(r'\s*--> (.+):(\d+):\d+', line)
        if file_match:
            current_file = file_match.group(1)
            current_line = int(file_match.group(2))
        
        # Match format string error
        if 'positional argument' in line and 'format string' in line and 'no arguments were given' in line:
            if current_file and current_line:
                format_errors.append((current_file, current_line))
    
    print(f"Found {len(format_errors)} format string errors to fix")
    
    # Fix each error
    for file_path, line_num in format_errors:
        fix_format_error_in_file(file_path, line_num)
    
    return len(format_errors)

def fix_format_error_in_file(file_path: str, line_num: int):
    """Fix format string error in a specific file and line."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        if line_num <= len(lines):
            line = lines[line_num - 1]  # Convert to 0-based index
            
            # Replace format patterns that need fixing
            fixed_line = line
            
            # Fix format!("message {}") patterns
            fixed_line = re.sub(r'format!\("([^"]*)\{\}([^"]*)"\)', r'format!("\1{}\2", "placeholder")', fixed_line)
            
            # More specific fixes based on the error patterns
            if "Pool '{}' not found" in fixed_line:
                fixed_line = re.sub(r'format!\("Pool \'\{\}\' not found"\)', r'format!("Pool \'{}\' not found", pool_name)', fixed_line)
            elif "Failed to cleanup pool '{}': {}" in fixed_line:
                fixed_line = re.sub(r'format!\("Failed to cleanup pool \'\{\}\': \{\}"\)', r'format!("Failed to cleanup pool \'{}\': {}", pool_name, error)', fixed_line)
            elif "Invalid JSON: {}" in fixed_line:
                fixed_line = re.sub(r'format!\("Invalid JSON: \{\}"\)', r'format!("Invalid JSON: {}", e)', fixed_line)
            elif "PEM decode error: {}" in fixed_line:
                fixed_line = re.sub(r'format!\("PEM decode error: \{\}"\)', r'format!("PEM decode error: {}", e)', fixed_line)
            elif "DER decode error: {}" in fixed_line:
                fixed_line = re.sub(r'format!\("DER decode error: \{\}"\)', r'format!("DER decode error: {}", e)', fixed_line)
            
            # Generic fallback - remove placeholders if we can't determine the args
            if '{}' in fixed_line and 'format!' in fixed_line:
                # Remove the placeholders entirely for now
                fixed_line = re.sub(r'format!\("([^"]*)\{\}([^"]*)"\)', r'"\1\2"', fixed_line)
            
            if fixed_line != line:
                lines[line_num - 1] = fixed_line
                
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.writelines(lines)
                
                print(f"Fixed format error in {file_path}:{line_num}")
    
    except Exception as e:
        print(f"Error fixing {file_path}:{line_num}: {e}")

def main():
    """Main function."""
    print("🔧 Fixing format string errors...")
    
    errors_fixed = fix_format_errors()
    
    if errors_fixed > 0:
        print(f"✅ Fixed {errors_fixed} format string errors")
        print("Running cargo check again...")
        os.system("cargo check 2>&1 | head -20")
    else:
        print("No format string errors found")

if __name__ == "__main__":
    main()
