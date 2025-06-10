#!/usr/bin/env python3
"""
Systematic fix for test file syntax issues in the CURSED codebase.
Addresses the most common patterns causing compilation failures.
"""

import os
import re
import glob

def fix_basic_syntax_issues(content):
    """Fix basic syntax issues like mismatched delimiters and malformed strings."""
    
    # Fix mod declarations
    content = re.sub(r'mod\s+(\w+)\s*::\s*#\[test\]', r'mod \1 {\n    use super::*;\n    \n    #[test]', content)
    
    # Fix common bracket/brace mismatches
    content = re.sub(r'vec!\[([^\]]*)\](\d+)\]', r'vec![\1]', content)  # vec![...42] -> vec![...]
    content = re.sub(r'assert_eq!\(([^,]+),\s*vec!\[([^\]]*)\](\d+)\]\)', r'assert_eq!(\1, vec![\2])', content)
    
    # Fix unterminated strings and missing delimiters
    content = re.sub(r'\.to_string\(\)"([^"]*)"', r'.to_string()', content)  # Remove malformed string literals
    content = re.sub(r'\.to_string\(\)([^)]+)\)', r'.to_string()', content)  # Clean up to_string calls
    
    # Fix missing opening braces in loops and conditions
    content = re.sub(r'for\s+([^{]+)\s+\{([^}]+)\s*$', r'for \1 {\n        \2\n    }', content, flags=re.MULTILINE)
    content = re.sub(r'if\s+([^{]+)\s+\{([^}]+)\s*else\s*\{([^}]*)\}*', r'if \1 {\n        \2\n    } else {\n        \3\n    }', content)
    
    # Fix common function call issues
    content = re.sub(r'(\w+)\s*\(\s*([^)]*)\s*"\)\s*"', r'\1(\2)', content)  # Remove extra quotes
    content = re.sub(r'assert!\(([^,]+),\s*([^)]+)"\)', r'assert!(\1, "\2")', content)  # Fix assert messages
    
    # Fix string literal issues
    content = re.sub(r'"([^"]*)"([^"]*)"', r'"\1\2"', content)  # Merge broken string literals
    content = re.sub(r'println!\(([^)]+)\)', r'println!("{}", \1)', content)  # Fix println formatting
    
    # Fix incomplete function definitions
    content = re.sub(r'fn\s+(\w+)\s*\(\s*\)\s*\{([^}]+)\s*$', r'fn \1() {\n        \2\n    }', content, flags=re.MULTILINE)
    
    return content

def fix_test_specific_issues(content):
    """Fix test-specific syntax patterns."""
    
    # Fix test function structure
    content = re.sub(r'#\[test\]\s*fn\s+(\w+)\s*\(\s*\)\s*\{([^}]+)\s*#\[test\]', 
                     r'#[test]\n    fn \1() {\n        \2\n    }\n\n    #[test]', content)
    
    # Fix assert macro calls
    content = re.sub(r'assert!\(([^,]+),\s*([^")]+)\)"', r'assert!(\1, "\2")', content)
    content = re.sub(r'assert_eq!\(([^,]+),\s*([^,]+),\s*([^")]+)\)"', r'assert_eq!(\1, \2, "\3")', content)
    
    # Fix variable declarations
    content = re.sub(r'let\s+(\w+)\s*=\s*([^;]+);*\s*([^}]+)}', r'let \1 = \2;\n        \3\n    }', content)
    
    # Fix common loop patterns
    content = re.sub(r'for\s+([^{]+)\s+\{([^}]+)\s*assert', r'for \1 {\n            \2\n            assert', content)
    
    # Fix match statements
    content = re.sub(r'match\s+([^{]+)\s+\{([^}]+)\}', r'match \1 {\n            \2\n        }', content)
    
    return content

def fix_string_and_token_issues(content):
    """Fix string literal and token-related issues."""
    
    # Fix raw string literals
    content = re.sub(r'r#\s*;', r'r#""#', content)  # Fix invalid raw string
    content = re.sub(r'r#([^"]*)"#\s*#;', r'r#"\1"#', content)  # Fix malformed raw strings
    
    # Fix escaped characters in strings
    content = re.sub(r'\\n\s*{', r'\\n", {', content)  # Fix newline escapes
    content = re.sub(r'println!\(([^)]+)\\n\s*([^)]+)\)', r'println!("\1\\n\2")', content)
    
    # Fix format string issues
    content = re.sub(r'format!\(([^)]+)\)', r'format!("{}", \1)', content)
    
    # Fix common token patterns
    content = re.sub(r'Token::(\w+)\(([^)]+)"\)', r'Token::\1(\2)', content)
    
    return content

def fix_delimiter_matching(content):
    """Attempt to fix obvious delimiter matching issues."""
    
    lines = content.split('\n')
    fixed_lines = []
    open_braces = 0
    open_parens = 0
    open_brackets = 0
    
    for line in lines:
        # Count delimiters
        open_braces += line.count('{') - line.count('}')
        open_parens += line.count('(') - line.count(')')
        open_brackets += line.count('[') - line.count(']')
        
        # Fix obvious single-line issues
        if line.strip().endswith('{') and not line.strip().startswith('//'):
            # Looks like a function/loop/condition start - keep as is
            fixed_lines.append(line)
        elif '}}' in line and open_braces < 0:
            # Too many closing braces
            line = line.replace('}}', '}')
            fixed_lines.append(line)
            open_braces += 1
        elif line.strip().endswith('}') and line.count('}') > line.count('{'):
            # Add missing opening brace
            if fixed_lines and not fixed_lines[-1].strip().endswith('{'):
                fixed_lines.append(line)
            else:
                fixed_lines.append(line)
        else:
            fixed_lines.append(line)
    
    # Add missing closing braces at the end if needed
    while open_braces > 0:
        fixed_lines.append('}')
        open_braces -= 1
    
    return '\n'.join(fixed_lines)

def fix_test_file(filepath):
    """Fix a single test file."""
    print(f"Fixing {filepath}...")
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_basic_syntax_issues(content)
        content = fix_test_specific_issues(content)
        content = fix_string_and_token_issues(content)
        content = fix_delimiter_matching(content)
        
        # Only write if we made changes
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  ✓ Fixed {filepath}")
            return True
        else:
            print(f"  - No changes needed for {filepath}")
            return False
            
    except Exception as e:
        print(f"  ✗ Error fixing {filepath}: {e}")
        return False

def main():
    """Fix all test files with syntax issues."""
    
    test_files = glob.glob("tests/*.rs")
    
    # Focus on the files that had the most obvious syntax errors
    priority_files = [
        "tests/quick_test_standalone_unit.rs",
        "tests/channel_select_test.rs", 
        "tests/goroutine_implementation_test.rs",
        "tests/interface_type_assertion_enhanced_test.rs",
        "tests/crypto_stress_test.rs",
        "tests/enhanced_gc_stress_test.rs",
        "tests/database_integration_tests.rs",
        "tests/web_vibez_benchmarks.rs",
        "tests/documentation_ast_test.rs",
    ]
    
    print("Fixing high-priority test files with syntax errors...")
    
    fixed_count = 0
    for filepath in priority_files:
        if os.path.exists(filepath):
            if fix_test_file(filepath):
                fixed_count += 1
    
    print(f"\n✓ Fixed {fixed_count} test files")
    print("Re-run tests to see remaining issues.")

if __name__ == "__main__":
    main()
