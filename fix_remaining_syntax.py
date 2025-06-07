#!/usr/bin/env python3

import re
import os
import glob

def fix_remaining_syntax_errors(file_path):
    """Fix remaining syntax errors in Rust test files"""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Fix mismatched closing delimiters - remove extra closing parentheses
    content = re.sub(r'\.to_string\(\)\)\);', '.to_string());', content)
    content = re.sub(r'\.len\(\)\)\);', '.len());', content)
    content = re.sub(r'\.verify\(\)\)\);', '.verify());', content)
    content = re.sub(r'\.unwrap_err\(\)\.to_string\(\)\)\);', '.unwrap_err().to_string());', content)
    
    # Fix function calls with extra closing parentheses 
    content = re.sub(r'assert!\([^;]+\)\);', lambda m: m.group(0)[:-1], content)
    content = re.sub(r'panic!\([^;]+\)\);', lambda m: m.group(0)[:-1], content)
    
    # Fix vec! with extra closing bracket
    content = re.sub(r'vec!\[[^\]]+\]\)\];', lambda m: m.group(0)[:-2] + ';', content)
    
    # Fix Arc::new(Object::String(...)) patterns with extra closing bracket
    content = re.sub(r'Arc::new\(Object::String\([^)]+\)\)\];', lambda m: m.group(0)[:-2] + ']);', content)
    
    # Fix .clone() patterns  
    content = re.sub(r'\.clone\(\)\);', '.clone();', content)
    content = re.sub(r'\.downgrade\(\)\);', '.downgrade();', content)
    
    # Fix specific broken function calls
    content = re.sub(r'gc\.allocate\([^)]+\)\);', lambda m: m.group(0)[:-1], content)
    content = re.sub(r'nodes\.push\([^)]+\)\);', lambda m: m.group(0)[:-1], content)
    
    # Fix missing semicolons
    content = re.sub(r'PathBuf::from\("[^"]*"\)\)', 'PathBuf::from("test.csd"))', content)
    
    # Fix Token construction patterns
    content = re.sub(r'Token::Identifier\("([^"]+)".to_string\(\),', r'Token::Identifier("\1".to_string()),', content)
    
    # Fix .set_next() patterns
    content = re.sub(r'\.set_next\([^)]+\)\);', lambda m: m.group(0)[:-1], content)
    
    # Fix assert patterns that need closing
    content = re.sub(r'assert!\([^)]+contains\([^)]+\)\);', lambda m: m.group(0)[:-1], content)
    
    # Fix contains patterns
    content = re.sub(r'\.contains\([^)]+\)\);', lambda m: m.group(0)[:-1], content)
    
    # Fix Err(Error::Compilation patterns
    content = re.sub(r'Err\(Error::Compilation\([^)]+\)\)', lambda m: m.group(0) + ')', content)
    
    # Fix vec! array patterns
    content = re.sub(r'vec!\[\s*Error::', r'vec![Error::', content)
    content = re.sub(r'(Error::[^,\]]+),\s*\];', r'\1];', content)
    
    # Fix Ok() -> Ok(())
    content = re.sub(r'\bOk\(\)\s*$', 'Ok(())', content, flags=re.MULTILINE)
    
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"Fixed remaining syntax errors in {file_path}")
        return True
    return False

def main():
    # Find all test files
    test_files = glob.glob('tests/*.rs')
    
    fixed_count = 0
    for test_file in test_files:
        if fix_remaining_syntax_errors(test_file):
            fixed_count += 1
    
    print(f"Fixed remaining syntax errors in {fixed_count} files")

if __name__ == "__main__":
    main()
