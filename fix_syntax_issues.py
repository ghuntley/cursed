#!/usr/bin/env python3

import re
import os
import glob

def fix_syntax_errors(file_path):
    """Fix common syntax errors in Rust test files"""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Fix extra closing parentheses
    content = re.sub(r'\.unwrap\(\)\);', '.unwrap();', content)
    content = re.sub(r'\.sum\(\)\);', '.sum();', content)
    content = re.sub(r'\.get_data_layout\(\)\);', '.get_data_layout();', content)
    content = re.sub(r'\.from_triple\([^)]+\)\);', lambda m: m.group(0)[:-1], content)
    content = re.sub(r'\.default\(\);', '.default());', content)
    
    # Fix missing closing parentheses in function calls
    content = re.sub(r'types\.push\(Type::Struct\(\s*', 'types.push(Type::Struct(vec![Type::Int32], false));', content)
    
    # Fix parser errors pattern
    content = re.sub(r'panic!\("Parser errors: \{:\?\}", parser\.errors\(\);', 'panic!("Parser errors: {:?}", parser.errors());', content)
    
    # Fix basic llvm control flow pattern - missing closing parenthesis
    content = re.sub(r'LlvmCodeGenerator::new\(&context, "[^"]*", PathBuf::from\("[^"]*"\);', 
                    lambda m: m.group(0)[:-1] + ')', content)
    
    # Fix string to_string pattern
    content = re.sub(r'String::from_utf8_lossy\([^)]+\)\.to_string\(\)\);', 
                    lambda m: m.group(0)[:-1], content)
    
    # Fix parser.new pattern (missing mut in lexer)
    content = re.sub(r'let mut parser = Parser::new\(&mut lexer\)\.unwrap\(\)\);', 
                    'let mut parser = Parser::new(&mut lexer).unwrap();', content)
    content = re.sub(r'let program = parser\.parse_program\(\)\.unwrap\(\)\);', 
                    'let program = parser.parse_program().unwrap();', content)
    
    # Fix finalized.lock().unwrap()) pattern
    content = re.sub(r'\.lock\(\)\.unwrap\(\)\);', '.lock().unwrap();', content)
    
    # Fix add_function pattern
    content = re.sub(r'\.add_function\([^)]+\), Some\(inkwell::module::Linkage::External\);', 
                    lambda m: m.group(0)[:-1] + ')', content)
    
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"Fixed syntax errors in {file_path}")
        return True
    return False

def main():
    # Find all test files
    test_files = glob.glob('tests/*.rs')
    
    fixed_count = 0
    for test_file in test_files:
        if fix_syntax_errors(test_file):
            fixed_count += 1
    
    print(f"Fixed syntax errors in {fixed_count} files")

if __name__ == "__main__":
    main()
