#!/usr/bin/env python3
"""
Final comprehensive fix for all remaining test file syntax errors
"""

import os
import re
import glob

def fix_all_remaining_syntax_errors(content):
    """Apply all known syntax error fixes"""
    
    # Fix unterminated strings at end of files
    content = re.sub(r'"\s*$', '', content, flags=re.MULTILINE)
    
    # Fix unterminated function calls
    content = re.sub(r'(\w+)\(\s*"([^"]*?)"\s*\{\s*:\?\s*\}\s*", [^)]*?\)\s*;?\s*"', r'\1("\2");', content)
    
    # Fix malformed assert statements
    content = re.sub(r'assert!\s*\(\s*([^)]*?)\s*;\s*"\s*\)', r'assert!(\1)', content)
    content = re.sub(r'assert_eq!\s*\(\s*([^)]*?)\s*;\s*"\s*\)', r'assert_eq!(\1)', content)
    
    # Fix malformed debug/info/println statements
    content = re.sub(r'(debug|info|println)!\s*\(\s*([^)]*?)\s*;\s*"\s*\)', r'\1!(\2)', content)
    
    # Fix string concatenation issues
    content = re.sub(r'"([^"]*?)"\s*\{\s*:\?\s*\}\s*", ([^)]*?)\)\s*;?\s*"', r'format!("\1{:?}", \2)', content)
    
    # Fix struct literals with extra closing braces
    content = re.sub(r'\{\s*([^}]*?)\s*\}\s*\}\s*\)', r'{\1})', content)
    
    # Fix vector literals with mismatched brackets
    content = re.sub(r'vec!\[\s*([^\]]*?)\]\s*([^\]]*?)\]', r'vec![\1]', content)
    
    # Fix extra semicolons in function calls
    content = re.sub(r'([a-zA-Z_]\w*)\(\s*([^)]*?)\s*;\s*$', r'\1(\2)', content, flags=re.MULTILINE)
    
    # Fix malformed raw strings
    content = re.sub(r'r#"([^"]*?);\s*$', r'r#"\1"#', content, flags=re.MULTILINE)
    
    return content

def fix_specific_test_files():
    """Fix specific test files with known issues"""
    
    # Fix simple_lexer_test.rs
    try:
        with open('tests/simple_lexer_test.rs', 'r') as f:
            content = f.read()
        # Remove trailing unterminated string
        content = content.rstrip().rstrip('"')
        if not content.endswith('}'):
            content += '\n}'
        with open('tests/simple_lexer_test.rs', 'w') as f:
            f.write(content)
        print("Fixed tests/simple_lexer_test.rs")
    except:
        pass
    
    # Fix simple_llvm_test.rs
    try:
        with open('tests/simple_llvm_test.rs', 'r') as f:
            content = f.read()
        content = content.rstrip().rstrip('"')
        if not content.endswith('}'):
            content += '\n}'
        with open('tests/simple_llvm_test.rs', 'w') as f:
            f.write(content)
        print("Fixed tests/simple_llvm_test.rs")
    except:
        pass
    
    # Fix simple_jit_test.rs
    try:
        with open('tests/simple_jit_test.rs', 'r') as f:
            content = f.read()
        content = content.rstrip().rstrip('"')
        if not content.endswith('}'):
            content += '\n}'
        with open('tests/simple_jit_test.rs', 'w') as f:
            f.write(content)
        print("Fixed tests/simple_jit_test.rs")
    except:
        pass
    
    # Fix minimal_interface_test.rs
    try:
        with open('tests/minimal_interface_test.rs', 'r') as f:
            content = f.read()
        content = content.rstrip().rstrip('"')
        if not content.endswith('}'):
            content += '\n}'
        with open('tests/minimal_interface_test.rs', 'w') as f:
            f.write(content)
        print("Fixed tests/minimal_interface_test.rs")
    except:
        pass

def main():
    """Main function to fix all test files"""
    
    # First, fix specific files
    fix_specific_test_files()
    
    # Then apply general fixes to all test files
    test_files = glob.glob('tests/**/*.rs', recursive=True)
    
    fixed_count = 0
    for test_file in test_files:
        try:
            with open(test_file, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            content = fix_all_remaining_syntax_errors(content)
            
            if content != original_content:
                with open(test_file, 'w', encoding='utf-8') as f:
                    f.write(content)
                fixed_count += 1
        except Exception as e:
            continue
    
    print(f"Applied general fixes to {fixed_count} test files")

if __name__ == '__main__':
    main()
