#!/usr/bin/env python3

import os
import re
import glob

def fix_common_syntax_issues(file_path):
    """Fix common syntax corruption issues in a file."""
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Warning: Could not read {file_path}: {e}")
        return False
    
    original_content = content
    changes_made = False
    
    # Fix malformed cfg attributes: #[cfg(test] -> #[cfg(test)]
    content = re.sub(r'#\[cfg\(test\]', '#[cfg(test)]', content)
    if content != original_content:
        changes_made = True
        print(f"  Fixed cfg attributes in {file_path}")
    
    # Fix double semicolons: ;; -> ;
    content = re.sub(r';;+', ';', content)
    if content != original_content and not changes_made:
        changes_made = True
        print(f"  Fixed double semicolons in {file_path}")
    
    # Fix malformed test attributes: #[test] (make sure it's properly closed)
    content = re.sub(r'#\[test(?!\])', '#[test]', content)
    
    # Fix malformed ignore attributes: #[ignore = "..."] (missing closing quote/bracket)
    content = re.sub(r'#\[ignore = "([^"]*)"?\]?', r'#[ignore = "\1"]', content)
    
    # Fix unmatched braces in simple cases
    lines = content.split('\n')
    fixed_lines = []
    
    for line in lines:
        # Fix obvious bracket/brace mismatches
        if line.strip().endswith('}') and line.count('{') > line.count('}'):
            line = line.replace('{', '{', line.count('{') - line.count('}'))
        
        fixed_lines.append(line)
    
    content = '\n'.join(fixed_lines)
    
    # Write back if changes were made
    if content != original_content:
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            if not changes_made:  # Only print if we haven't already printed
                print(f"  Fixed syntax issues in {file_path}")
            return True
        except Exception as e:
            print(f"Warning: Could not write to {file_path}: {e}")
            return False
    
    return False

def main():
    """Fix syntax issues in test files."""
    
    print("🔧 Fixing systematic syntax issues in test files...")
    
    # Find all Rust test files
    test_files = []
    for pattern in ['tests/*.rs', 'tests/**/*.rs']:
        test_files.extend(glob.glob(pattern, recursive=True))
    
    if not test_files:
        print("No test files found!")
        return
    
    print(f"Found {len(test_files)} test files to process")
    
    fixed_count = 0
    
    # Process each test file
    for test_file in sorted(test_files):
        if fix_common_syntax_issues(test_file):
            fixed_count += 1
    
    print(f"\n✅ Completed! Fixed syntax issues in {fixed_count} files")

if __name__ == "__main__":
    main()
