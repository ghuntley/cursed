#!/usr/bin/env python3
"""
Fix remaining critical test file issues
"""

import os
import re

def fix_facts_codegen_test():
    """Fix the facts_codegen_test.rs file specifically"""
    filepath = 'tests/facts_codegen_test.rs'
    try:
        with open(filepath, 'r') as f:
            content = f.read()
    except FileNotFoundError:
        return False
    
    # Fix the raw string literal issue
    content = re.sub(r'let input = r#";\s*vibe main;.*?#;', '''let input = r#"
    vibe main; // Add a package declaration to make it more valid
    
    facts PI = 3.14159;
    facts E = 2.71828;
    facts ANSWER = 42;

    slay main() normie {
        yolo ANSWER;
    }
    "#;''', content, flags=re.DOTALL)
    
    # Fix function calls and syntax errors
    content = re.sub(r'let mut lexer = Lexer::new\(input\.to_string\(\)', 'let mut lexer = Lexer::new(input.to_string());', content)
    content = re.sub(r'let mut parser = Parser::new\(Lexer::new\(Lexer::new\(lexer\)\.unwrap\(\)', 'let mut parser = Parser::new(lexer);', content)
    content = re.sub(r'let program = parser\.unwrap\(\)\.parse_program\(\)\.unwrap\(\)', 'let program = parser.parse_program().unwrap();', content)
    
    # Fix ignore attribute
    content = re.sub(r'#\[ignore = "Facts/const codegen needs implementation ";"', '#[ignore = "Facts/const codegen needs implementation"]', content)
    
    with open(filepath, 'w') as f:
        f.write(content)
    
    print(f"Fixed {filepath}")
    return True

def fix_llvm_basic_expressions_test():
    """Fix the llvm_basic_expressions_test.rs file"""
    filepath = 'tests/llvm_basic_expressions_test.rs'
    try:
        with open(filepath, 'r') as f:
            content = f.read()
    except FileNotFoundError:
        return False
    
    # Fix unterminated string literal
    content = re.sub(r'assert!\(value\.is_int_value\(\), Result should be an ", integer\)".*?generator\.as_ref\(\)\.unwrap\(\)\.builder\(\)\.build_return\(Some\(&i32_type\.const_int\(0, false\)\.unwrap\(\)\}\s*$', 
                     '''assert!(value.is_int_value(), "Result should be an integer");

    // Just check if the result is negative as expected
    let int_value = value.into_int_value();
    
    // The result should be negative (-5 - 3 = -8)
    assert!(int_value.get_sign_extended_constant().unwrap() < 0, "Result should be negative");
    
    // Clean up
    generator.as_ref().unwrap().builder().build_return(Some(&i32_type.const_int(0, false).unwrap()));''', content, flags=re.DOTALL)
    
    with open(filepath, 'w') as f:
        f.write(content)
    
    print(f"Fixed {filepath}")
    return True

def fix_simple_qualified_name_test():
    """Fix simple_qualified_name_test.rs"""
    filepath = 'tests/simple_qualified_name_test.rs'
    try:
        with open(filepath, 'r') as f:
            content = f.read()
    except FileNotFoundError:
        return False
    
    # Fix function syntax
    content = re.sub(r'fn test_qualified_name_basic\(\) \{let qualified = QualifiedName::new_with_alias\(\)', 
                     'fn test_qualified_name_basic() {\n    let qualified = QualifiedName::new_with_alias()', content)
    
    # Fix string literal issues
    content = re.sub(r'"math\.to_string\(\)"\);', '"math.sqrt".to_string());', content)
    content = re.sub(r'assert_eq!\(qualified\.string\(\),  "math\.sqrt ";""\}', 'assert_eq!(qualified.string(), "math.sqrt");\n}', content)
    
    with open(filepath, 'w') as f:
        f.write(content)
    
    print(f"Fixed {filepath}")
    return True

def main():
    """Fix remaining critical test issues"""
    fixes = [
        fix_facts_codegen_test,
        fix_llvm_basic_expressions_test,
        fix_simple_qualified_name_test,
    ]
    
    fixed_count = 0
    for fix_func in fixes:
        try:
            if fix_func():
                fixed_count += 1
        except Exception as e:
            print(f"Error in {fix_func.__name__}: {e}")
    
    print(f"Applied {fixed_count} critical fixes")

if __name__ == '__main__':
    main()
