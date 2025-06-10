#!/usr/bin/env python3
"""
Fix remaining test syntax issues in CURSED codebase.
"""

import os
import re
import glob

def fix_string_literal_issues(content):
    """Fix unterminated and malformed string literals."""
    
    # Fix unterminated strings at end of lines
    content = re.sub(r'"([^"]*)"([^"]*)\s*$', r'"\1\2"', content, flags=re.MULTILINE)
    
    # Fix strings that got broken across multiple lines
    content = re.sub(r'"([^"]*)\s*\|\s*\n\s*([^"]*)"', r'"\1\2"', content)
    
    # Fix common string patterns
    content = re.sub(r'\.to_string\(\)"([^"]*)"', r'.to_string()', content)
    content = re.sub(r'"([^"]*)"\.to_string\(\)', r'"\1".to_string()', content)
    
    # Fix println! and format! strings
    content = re.sub(r'println!\s*\(\s*([^")]+)\s*\)\s*"', r'println!("{}", \1)', content)
    content = re.sub(r'format!\s*\(\s*([^")]+)\s*\)\s*"', r'format!("{}", \1)', content)
    
    # Fix assert message strings
    content = re.sub(r'assert!\s*\(\s*([^,]+),\s*([^")]+)\s*\)\s*"', r'assert!(\1, "\2")', content)
    content = re.sub(r'assert_eq!\s*\(\s*([^,]+),\s*([^,]+),\s*([^")]+)\s*\)\s*"', r'assert_eq!(\1, \2, "\3")', content)
    
    return content

def fix_raw_string_issues(content):
    """Fix raw string literal issues."""
    
    # Fix invalid raw string patterns
    content = re.sub(r'r#\s*;', r'r#""#', content)
    content = re.sub(r'r#([^"]*)"#\s*#;', r'r#"\1"#', content)
    content = re.sub(r'r#\s*\n([^#]*)\n\s*#', r'r#"\1"#', content, flags=re.DOTALL)
    
    return content

def fix_delimiter_issues(content):
    """Fix obvious delimiter matching problems."""
    
    # Fix vec! with extra brackets
    content = re.sub(r'vec!\[([^\]]*)\](\d+)\]', r'vec![\1]', content)
    content = re.sub(r'vec!\[([^\]]*)\]([)\]}])', r'vec![\1]\2', content)
    
    # Fix common function call patterns
    content = re.sub(r'(\w+)\s*\(\s*([^)]*)\s*"\)\s*"', r'\1(\2)', content)
    content = re.sub(r'(\w+)\s*\(\s*([^)]*)\s*\)\s*"([^"]*)"', r'\1(\2)', content)
    
    # Fix array indexing issues
    content = re.sub(r'\[([^\]]*)\]\s*(\d+)\]', r'[\1]', content)
    
    # Fix function definitions
    content = re.sub(r'fn\s+(\w+)\s*\(\s*\)\s*\{([^{}]*)\s*$', r'fn \1() {\n        \2\n    }', content, flags=re.MULTILINE)
    
    return content

def fix_expression_issues(content):
    """Fix expression syntax issues."""
    
    # Fix match expressions
    content = re.sub(r'match\s+([^{]+)\s+\{([^}]+)\s*else\s*\{([^}]*)\}', r'match \1 {\n        \2,\n        _ => {\3}\n    }', content)
    
    # Fix if-else expressions
    content = re.sub(r'if\s+([^{]+)\s+\{([^}]+)\s*\}\s*else\s*\{([^}]*)\}', r'if \1 {\n        \2\n    } else {\n        \3\n    }', content)
    
    # Fix loop expressions
    content = re.sub(r'for\s+([^{]+)\s+\{([^}]+)', r'for \1 {\n        \2', content)
    content = re.sub(r'while\s+([^{]+)\s+\{([^}]+)', r'while \1 {\n        \2', content)
    
    # Fix function calls with missing delimiters
    content = re.sub(r'(\w+)\s*\(\s*([^)]*),\s*([^)]*)\s*$', r'\1(\2, \3)', content, flags=re.MULTILINE)
    
    return content

def fix_token_and_import_issues(content):
    """Fix token construction and import issues."""
    
    # Fix Token construction patterns
    content = re.sub(r'Token::(\w+)\s*\(\s*([^)]+)"\s*\)', r'Token::\1(\2)', content)
    content = re.sub(r'Token::(\w+)\s*\(\s*"([^"]*)"([^)]*)\)', r'Token::\1("\2")', content)
    
    # Fix import paths
    content = re.sub(r'#\[path\s*=\s*"([^"]*)\]', r'#[path = "\1"]', content)
    content = re.sub(r'use\s+([^;]+);([^;]+);', r'use \1;\nuse \2;', content)
    
    return content

def fix_test_specific_patterns(content):
    """Fix test-specific syntax patterns."""
    
    # Fix test module structure
    content = re.sub(r'#\[cfg\(test\)\]\s*mod\s+(\w+)\s*::\s*#\[test\]', 
                     r'#[cfg(test)]\nmod \1 {\n    use super::*;\n\n    #[test]', content)
    
    # Fix test function headers
    content = re.sub(r'#\[test\]\s*fn\s+(\w+)\s*\(\s*\)\s*\{([^}]+)\s*#\[test\]', 
                     r'#[test]\n    fn \1() {\n        \2\n    }\n\n    #[test]', content)
    
    # Fix common test patterns
    content = re.sub(r'common::tracing::init_tracing!\(\);', r'// common::tracing::init_tracing!();', content)
    content = re.sub(r'tracing::info!\s*\(\s*([^)]+)\)\s*;', r'tracing::info!("{}", \1);', content)
    
    return content

def fix_specific_error_patterns(content):
    """Fix specific error patterns found in compilation."""
    
    # Fix "execute" prefix issue
    content = re.sub(r'#\[ignore\s*=\s*"([^"]*)\s+execute"\]', r'#[ignore = "\1"]', content)
    
    # Fix unknown token starts
    content = re.sub(r'println!\s*\(\s*([^)]+)\\n\s*([^)]+)\)', r'println!("{}\n{}", \1, \2)', content)
    
    # Fix grave accent issues in strings
    content = re.sub(r'```', r'"""', content)
    
    # Fix mod declarations
    content = re.sub(r'mod\s+(\w+)\s*\{([^}]*)\s*$', r'mod \1 {\n\2\n}', content, flags=re.MULTILINE | re.DOTALL)
    
    return content

def fix_file_comprehensive(filepath):
    """Apply comprehensive fixes to a file."""
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fix functions in order
        content = fix_string_literal_issues(content)
        content = fix_raw_string_issues(content)
        content = fix_delimiter_issues(content)
        content = fix_expression_issues(content)
        content = fix_token_and_import_issues(content)
        content = fix_test_specific_patterns(content)
        content = fix_specific_error_patterns(content)
        
        # Basic structure validation and fixes
        if content.count('{') != content.count('}'):
            # Try to balance braces
            open_braces = content.count('{') - content.count('}')
            if open_braces > 0:
                content += '\n' + '}' * open_braces
        
        # Write back if changes were made
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  ✓ Fixed {filepath}")
            return True
        else:
            print(f"  - No changes for {filepath}")
            return False
            
    except Exception as e:
        print(f"  ✗ Error fixing {filepath}: {e}")
        return False

def main():
    """Fix remaining test files."""
    
    # Files that had compilation errors in the original run
    problem_files = [
        "tests/interface_registry_extension_visualization_test.rs",
        "tests/improved_field_accessors_test.rs", 
        "tests/map_operations_test_fixed.rs",
        "tests/improved_field_accessors_integration_test.rs",
        "tests/llvm_switch_statement_test.rs",
        "tests/buffered_channel_llvm_integration_test.rs",
        "tests/quick_test_simple.rs",
        "tests/gc_thread_safety_test.rs",
        "tests/improved_generic_params_test.rs",
        "tests/simple_generic_function_test.rs",
        "tests/interface_type_assertion_basic_error_propagation_test.rs",
    ]
    
    print("Fixing remaining test files with syntax errors...")
    
    fixed_count = 0
    for filepath in problem_files:
        if os.path.exists(filepath):
            print(f"Fixing {filepath}...")
            if fix_file_comprehensive(filepath):
                fixed_count += 1
    
    print(f"\n✓ Fixed {fixed_count} additional test files")

if __name__ == "__main__":
    main()
