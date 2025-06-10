#!/usr/bin/env python3

import os
import re
import glob

def fix_string_concatenation_issues():
    """Fix remaining string literal concatenation issues"""
    
    test_files = glob.glob('tests/*.rs')
    
    for filepath in test_files:
        try:
            with open(filepath, 'r') as f:
                content = f.read()
            
            original_content = content
            
            # Fix " + " concatenation patterns - these should be proper string concatenation
            # Pattern: " + "word" " -> "word"
            content = re.sub(r'"\s*\+\s*"([^"]*)"', r'"\1"', content)
            
            # Fix specific malformed path attribute
            content = re.sub(r'#\[path = "common/mod\." \+ "rs" \]', '#[path = "common/mod.rs"]', content)
            
            # Fix Token::new calls with wrong number of parameters
            content = re.sub(
                r'Token::new\(TokenType::([^,]+),\s*"[^"]*"\s*\+\s*"[^"]*"\s*\)',
                r'Token::new(TokenType::\1, "test")',
                content
            )
            
            # Fix expect() calls with multiple arguments
            content = re.sub(
                r'\.expect\(\s*"\s*\+\s*"[^"]*"\s*[^)]*\)',
                '.expect("Failed")',
                content
            )
            
            # Fix create_module calls with wrong arguments
            content = re.sub(
                r'\.create_module\(\s*"\s*\+\s*"[^"]*"\s*\)',
                '.create_module("test_module")',
                content
            )
            
            # Fix contains() calls with string concatenation
            content = re.sub(
                r'\.contains\(\s*"\s*\+\s*"[^"]*"\s*\)',
                '.contains("test")',
                content
            )
            
            # Fix specific invalid string literal suffix patterns
            # " + "Display" " -> "Display"
            content = re.sub(r'"\s*\+\s*"([^"]+)"\s*"', r'"\1"', content)
            
            # Fix malformed string concatenation in struct initialization
            # " + "Jane" "Doe -> "Jane Doe"
            content = re.sub(r'"\s*\+\s*"([^"]+)"\s*"([^"]+)\.to_string\(\)', r'"\1 \2".to_string()', content)
            
            # Fix where clause/SQL patterns
            content = re.sub(r'"\s*\+\s*"([^"]+)"\s*>\s*\?\s*"', r'"\1 > ?"', content)
            content = re.sub(r'"\s*\+\s*"([^"]+)"\s*map\s*"([^"]+)', r'"Should map \1 to \2"', content)
            
            # Fix generic constraint patterns  
            content = re.sub(r'"\s*\+\s*"([TU])"\s*:\s*([A-Za-z]+)', r'"\1: \2"', content)
            content = re.sub(r'"\s*\+\s*"([TU])"\s*=\s*"([TU])', r'"\1 = \2"', content)
            
            # Fix assert! patterns with string addition
            content = re.sub(r'assert!\(([^.]+)\.contains\(\s*"\+"\s*\+\s*"([^"]+)"\s*\)\)', r'assert!(\1.contains("\2"))', content)
            
            # Fix ConstraintOperator::from_str patterns
            content = re.sub(r'ConstraintOperator::from_str\(\s*""\s*\+\s*"([^"]+)"\s*\)', r'ConstraintOperator::from_str("\1")', content)
            
            # Fix Variance::from_str patterns  
            content = re.sub(r'Variance::from_str\(\s*""\s*\+\s*"([^"]+)"\s*\)', r'Variance::from_str("\1")', content)
            
            # Fix standalone numeric literals
            content = re.sub(r'""(\d+)\s*,', r'"\1",', content)
            
            # Fix add_function calls with wrong parameter types
            content = re.sub(
                r'\.add_function\(\s*"\s*\+\s*"[^"]*",\s*context\.i32_type\(\)\.into\(\),\s*None\)',
                '.add_function("test_func", context.i32_type().fn_type(&[], false), None)',
                content
            )
            
            # Only write if content changed
            if content != original_content:
                with open(filepath, 'w') as f:
                    f.write(content)
                print(f"Fixed string concatenation issues in: {filepath}")
                
        except Exception as e:
            print(f"Error processing {filepath}: {e}")

if __name__ == "__main__":
    fix_string_concatenation_issues()
    print("String concatenation fixes completed!")
