#!/usr/bin/env python3
"""
Fix remaining string literal parsing errors.
This script handles the specific cases where string literals are being incorrectly parsed.
"""

import os
import re
import glob

def fix_remaining_string_issues(content):
    """Fix remaining string literal issues in test files."""
    
    # Fix specific patterns like: test" error" -> "test error"
    content = re.sub(
        r'(\w+)"\s*([^"]+)"',
        r'"\1 \2"',
        content
    )
    
    # Fix patterns in function calls where identifiers are parsed as prefixes
    content = re.sub(
        r'\.contains\(\s*(\w+)"\s*([^"]+)"\s*\)',
        r'.contains("\1 \2")',
        content
    )
    
    content = re.sub(
        r'Error::repl_error\(\s*(\w+)"\s*([^"]+)"\.to_string\(\)\)',
        r'Error::repl_error("\1 \2".to_string())',
        content
    )
    
    content = re.sub(
        r'get_variable\(\s*(\w+)"\s*([^"]*)"\)',
        r'get_variable("\1\2")',
        content
    )
    
    # Fix assert! patterns
    content = re.sub(
        r'assert!\([^,]+,\s*(\w+)"\s*([^"]+)"\)',
        r'assert!(result, "\1 \2")',
        content
    )
    
    return content

def process_file(filepath):
    """Process a single file and apply fixes."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        content = fix_remaining_string_issues(content)
        
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Fixed: {filepath}")
            return True
        else:
            print(f"⏭️  No changes: {filepath}")
            return False
            
    except Exception as e:
        print(f"❌ Error processing {filepath}: {e}")
        return False

def main():
    """Main function to process test files with string issues."""
    print("🔧 Fixing remaining string literal issues...")
    
    # Target specific files that had string issues
    problem_files = [
        "tests/simple_core_test.rs",
        "tests/variable_management_integration_test.rs"
    ]
    
    fixed_count = 0
    
    for test_file in problem_files:
        if os.path.exists(test_file):
            if process_file(test_file):
                fixed_count += 1
    
    print(f"\n📊 Summary:")
    print(f"   Fixed files: {fixed_count}")
    
    if fixed_count > 0:
        print(f"\n✅ Fixed {fixed_count} files with string literal issues.")
    else:
        print(f"\n⚠️  No files needed fixing.")

if __name__ == "__main__":
    main()
