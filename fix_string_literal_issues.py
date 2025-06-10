#!/usr/bin/env python3

import os
import re
import glob

def fix_string_literal_suffix_errors(content):
    """Fix string literal suffix errors in test files"""
    
    # Pattern to match invalid string literal suffixes like "hello "world
    pattern = r'"([^"]*)\s*"([a-zA-Z_][a-zA-Z0-9_]*)\b'
    
    def replace_match(match):
        string_part = match.group(1)
        suffix_part = match.group(2)
        
        # Common cases where we want to concatenate strings
        if suffix_part in ['works', 'work', 'error', 'incompatible', 'reader', 'c', 'handling', 'Point', 'compare', 'csd']:
            return f'"{string_part}{suffix_part}"'
        else:
            # Keep the original for complex cases
            return f'"{string_part}" + "{suffix_part}"'
    
    fixed_content = re.sub(pattern, replace_match, content)
    return fixed_content

def fix_specific_file_issues(filepath, content):
    """Fix specific issues in certain files"""
    
    # Fix minimal_goroutine_test.rs token issues
    if 'minimal_goroutine_test.rs' in filepath:
        # Fix Token::new usage where String is expected
        content = re.sub(r'token:\s*Token::new\([^)]+\)', 'token: "test_token".to_string()', content)
        # Fix function field expecting Box<dyn Expression>
        content = re.sub(r'function:\s*"[^"]*"\.to_string\(\)', 'function: Box::new(DummyExpression {})', content)
        
    # Fix interface type assertion error issues
    if 'interface_type_assertion_error_propagation_integration_test.rs' in filepath:
        # Fix Error::TypeAssertion and Error::Compilation usage
        content = re.sub(r'Error::TypeAssertion\(([^)]+)\)', r'Error::from_str("Type assertion error")', content)
        content = re.sub(r'Error::Compilation\(([^)]+)\)', r'Error::from_str("Compilation error")', content)
    
    # Fix simple_constraint_check_test.rs issues
    if 'simple_constraint_check_test.rs' in filepath:
        # Fix GenericConstraint::new calls (it only takes 2 parameters)
        content = re.sub(
            r'GenericConstraint::new\(\s*"([^"]+)"\s*\.to_string\(\),\s*"([^"]+)"\s*\.to_string\(\),\s*"([^"]+)"\s*\.to_string\(\)',
            r'GenericConstraint::new("\1".to_string(), vec!["\3".to_string()])',
            content
        )
        # Fix field access issues
        content = re.sub(r'\.parameter_name', '.type_param', content)
        content = re.sub(r'\.interface_name', '.constraints[0]', content)
    
    return content

def process_test_files():
    """Process all test files to fix string literal issues"""
    
    test_files = glob.glob('tests/*.rs')
    
    for filepath in test_files:
        try:
            with open(filepath, 'r') as f:
                content = f.read()
            
            original_content = content
            
            # Apply fixes
            content = fix_string_literal_suffix_errors(content)
            content = fix_specific_file_issues(filepath, content)
            
            # Only write if content changed
            if content != original_content:
                with open(filepath, 'w') as f:
                    f.write(content)
                print(f"Fixed: {filepath}")
                
        except Exception as e:
            print(f"Error processing {filepath}: {e}")

if __name__ == "__main__":
    process_test_files()
    print("String literal fixes completed!")
