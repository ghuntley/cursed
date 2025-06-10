#!/usr/bin/env python3
"""
Fix specific remaining syntax errors in CURSED test files
"""

import os
import re
import glob

def fix_specific_errors(content):
    """Fix specific error patterns from the test output"""
    
    # Fix unterminated raw strings
    content = re.sub(r'r#"([^"]*?)$\s*', r'r#"\1"#', content, flags=re.MULTILINE)
    
    # Fix malformed string literals with semicolons
    content = re.sub(r'"([^"]*?);"\s*\}', r'"\1";}', content)
    
    # Fix unterminated double quote strings
    content = re.sub(r'"([^"]*?)"\s*"\s*\}', r'"\1";}', content)
    
    # Fix unknown prefixes - add space
    content = re.sub(r'([a-zA-Z_]\w*)"([^"]*?)"', r'\1 "\2"', content)
    
    # Fix mismatched delimiters in function calls
    content = re.sub(r'(\w+)\(\s*([^)]*?);\s*"\s*\)', r'\1(\2)', content)
    content = re.sub(r'(\w+)\(\s*([^)]*?);\s*$', r'\1(\2)', content, flags=re.MULTILINE)
    
    # Fix unexpected closing delimiters
    content = re.sub(r'\)\s*;\s*"\s*\}', r');}', content)
    content = re.sub(r'\)\s*;\s*$', r')', content, flags=re.MULTILINE)
    
    # Fix assert! macros with malformed strings
    content = re.sub(r'assert!\s*\(\s*([^)]*?);\s*"\s*\)', r'assert!(\1)', content)
    
    # Fix debug! macros
    content = re.sub(r'debug!\s*\(\s*([^)]*?);\s*"\s*\)', r'debug!(\1)', content)
    content = re.sub(r'debug!\s*\(\s*"([^"]*?)"\s*\)\s*"\s*\}', r'debug!("\1");}', content)
    
    # Fix info! macros
    content = re.sub(r'info!\s*\(\s*([^)]*?);\s*"\s*\)', r'info!(\1)', content)
    content = re.sub(r'info!\s*\(\s*"([^"]*?)"\s*\)\s*"\s*\}', r'info!("\1");}', content)
    
    # Fix println! macros  
    content = re.sub(r'println!\s*\(\s*([^)]*?);\s*"\s*\)', r'println!(\1)', content)
    
    # Fix vector literals with extra brackets
    content = re.sub(r'vec!\[([^\]]*?)\]\s*([^\]]*?)\]', r'vec![\1]', content)
    
    # Fix struct field assignments
    content = re.sub(r'token:\s*([^,}]*?)\s*,?\s*value:\s*([^}]*?)\s*\}\s*\}', r'token: \1, value: \2}', content)
    
    # Fix special character issues (Unicode infinity symbols)
    content = content.replace('∞', 'INFINITY')
    content = content.replace('−', '-')
    
    return content

def fix_file(filepath):
    """Fix a single test file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
    except (UnicodeDecodeError, FileNotFoundError):
        return False
    
    original_content = content
    content = fix_specific_errors(content)
    
    if content != original_content:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"Fixed {filepath}")
        return True
    
    return False

def main():
    """Fix specific test files that had errors"""
    test_files = [
        'tests/simple_crypto_test.rs',
        'tests/package_manager_cli_test.rs', 
        'tests/nested_generics_test.rs',
        'tests/interface_type_assertion_error_handling_test.rs',
        'tests/simple_sql_vibes_test.rs',
        'tests/interface_type_assertion_result_integration_test.rs',
        'tests/basic_float_conversions_test.rs',
        'tests/facts_codegen_test.rs',
        'tests/interface_type_assertion_filesystem_error_propagation_test.rs',
        'tests/gc_fixed_test.rs',
        'tests/formatter_test.rs',
        'tests/standardized_llvm_structure_test.rs',
        'tests/llvm_basic_expressions_test.rs',
        'tests/linter_comprehensive_test.rs',
        'tests/unicode_runtime_standalone_test.rs',
        'tests/simple_qualified_name_test.rs',
        'tests/function_return_type_inference_test.rs',
        'tests/defer_basic_test.rs',
        'tests/comprehensive_circular_references_test.rs',
        'tests/property_access_test.rs'
    ]
    
    fixed = 0
    for filepath in test_files:
        if os.path.exists(filepath):
            if fix_file(filepath):
                fixed += 1
        else:
            print(f"File not found: {filepath}")
    
    print(f"Fixed {fixed} specific test files")

if __name__ == '__main__':
    main()
