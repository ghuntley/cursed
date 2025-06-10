#!/usr/bin/env python3

import os
import re
import glob

def fix_remaining_syntax_errors(content):
    """Fix remaining syntax errors after initial pass"""
    
    # Fix unterminated string literals 
    content = re.sub(r'"([^"]*)"([^"]*)"([^";]*)"?\s*$', r'"\1\2\3"', content, flags=re.MULTILINE)
    
    # Fix mismatched delimiters in function calls and arrays
    content = re.sub(r'\[([^\]]+)]a\]', r'[\1]', content)
    content = re.sub(r'\(([^)]+)\)([^;,}]*)$', r'(\1)\2;', content, flags=re.MULTILINE)
    
    # Fix assert! macros with unterminated strings
    content = re.sub(r'assert!\(([^)]+)"([^"]*)"\)', r'assert!(\1"\2")', content)
    content = re.sub(r'assert_eq!\(([^,]+),([^,]+)"([^"]*)"([^)]*)\)', r'assert_eq!(\1,\2, "\3"\4)', content)
    
    # Fix missing closing parentheses in assert statements
    content = re.sub(r'assert!\(([^)]+)(?<!;)(\n)', r'assert!(\1);\2', content)
    content = re.sub(r'assert_eq!\(([^)]+)(?<!;)(\n)', r'assert_eq!(\1);\2', content)
    
    # Fix string concatenation with missing quotes
    content = re.sub(r'\+ "([^"]*)"([^"]*)"', r'+ "\1\2"', content)
    
    # Fix format! macros
    content = re.sub(r'format!\("([^"]*)", ([^,)]+),([^)]*)\)', r'format!("\1", \2\3)', content)
    
    # Fix string literals with escaped quotes
    content = re.sub(r'\\"([^"]*)\\"', r'"\1"', content)
    
    # Fix missing closing braces and parentheses
    content = re.sub(r'fn ([^{]+){([^}]*)$', r'fn \1 {\2}', content, flags=re.MULTILINE)
    
    # Fix mismatched delimiters in vectors and arrays  
    content = re.sub(r'vec!\[([^\]]+)(?!\])', r'vec![\1]', content)
    
    # Fix common token construction issues
    content = re.sub(r'Token::new\(([^,]+),\s*"([^"]*)"([^)]*)\)', r'Token::new(\1, "\2\3")', content)
    
    # Fix extern function declarations 
    content = re.sub(r'extern\s+"C\s+fn', r'extern "C" fn', content)
    
    # Fix struct field access with missing closing parentheses
    content = re.sub(r'\.([a-zA-Z_]+)\(([^)]+)(?!\))', r'.\1(\2)', content)
    
    # Fix if-let patterns
    content = re.sub(r'if let ([^=]+) = ([^{]+)([^}]*)}', r'if let \1 = \2 {\3}', content)
    
    # Fix match arm patterns
    content = re.sub(r'([A-Za-z_:]+)\(([^)]+)\) => ([^,}]+),?', r'\1(\2) => \3,', content)
    
    # Fix string literal issues at end of lines
    content = re.sub(r'"([^"]*)"([^"]*)"(\s*[;}])', r'"\1\2"\3', content)
    
    # Fix missing semicolons in variable declarations
    content = re.sub(r'let ([^=]+) = ([^;]+)(?<!;)(\n)', r'let \1 = \2;\3', content)
    
    # Fix function call parentheses
    content = re.sub(r'([a-zA-Z_][a-zA-Z0-9_]*)\(([^)]+)(?!\))', r'\1(\2)', content)
    
    return content

def fix_specific_file_patterns(content, filepath):
    """Fix patterns specific to certain file types"""
    
    if 'stdlib_package_test.rs' in filepath:
        # Fix string object creation
        content = re.sub(r'string_object\(\s*"([^"]*)"([^)]*)\)', r'string_object("\1\2")', content)
        content = re.sub(r'Object::String\(([^)]+)\)', r'Object::String(\1)', content)
        
    if 'goroutine_runtime_basic_test.rs' in filepath:
        # Fix scheduler calls 
        content = re.sub(r'scheduler\.([a-zA-Z_]+)\(([^)]+)(?!\))', r'scheduler.\1(\2)', content)
        
    if 'struct_field_type_inference' in filepath:
        # Fix LLVM generator calls
        content = re.sub(r'generator\.([a-zA-Z_]+)\(([^)]+)(?!\))', r'generator.\1(\2)', content)
        
    # Fix common test patterns
    content = re.sub(r'#\[test\]\s*fn ([^{]+)\{', r'#[test]\nfn \1 {', content)
    
    return content

def fix_file(filepath):
    """Fix syntax errors in a single file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            original_content = f.read()
        
        fixed_content = fix_remaining_syntax_errors(original_content)
        fixed_content = fix_specific_file_patterns(fixed_content, filepath)
        
        if fixed_content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(fixed_content)
            print(f"Fixed: {filepath}")
            return True
        else:
            return False
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    """Fix remaining syntax errors in priority test files"""
    
    priority_files = [
        "tests/formatter_integration_test.rs",
        "tests/stdlib_package_test.rs", 
        "tests/goroutine_runtime_basic_test.rs",
        "tests/quick_test_fullfeature_test.rs",
        "tests/struct_field_type_inference_simplified_test.rs"
    ]
    
    # Also get files with most common error patterns
    all_test_files = glob.glob("tests/**/*.rs", recursive=True)
    
    fixed_count = 0
    total_count = len(priority_files) + len(all_test_files[:20])  # Process top priority first
    
    print(f"Processing {total_count} priority test files...")
    
    # Fix priority files first
    for filepath in priority_files:
        if os.path.exists(filepath):
            if fix_file(filepath):
                fixed_count += 1
    
    # Fix a sample of other files with common patterns
    for filepath in all_test_files[:20]:
        if fix_file(filepath):
            fixed_count += 1
    
    print(f"\nFixed {fixed_count} files")

if __name__ == "__main__":
    main()
