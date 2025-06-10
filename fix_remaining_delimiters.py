#!/usr/bin/env python3
"""
Fix remaining delimiter issues in CURSED test files.
"""

import os
import re
import glob

def fix_delimiter_issues(content):
    """Fix specific delimiter issues."""
    
    # Fix raw string literal prefixes
    content = re.sub(r'prefix `fixed` is unknown', '', content)
    content = re.sub(r'prefix `rs` is unknown', '', content)
    content = re.sub(r'r#"fixed"#', '"fixed"', content)
    content = re.sub(r'r#"rs"#', '"rs"', content)
    
    # Fix mismatched closing delimiters
    lines = content.split('\n')
    fixed_lines = []
    bracket_stack = []
    
    for line_num, line in enumerate(lines):
        original_line = line
        
        # Track opening and closing brackets
        for char in line:
            if char in '({[':
                bracket_stack.append((char, line_num))
            elif char in ')}]':
                if bracket_stack:
                    open_char, _ = bracket_stack.pop()
                    expected = {'(': ')', '{': '}', '[': ']'}
                    if expected.get(open_char) != char:
                        # Mismatched delimiter
                        if expected.get(open_char):
                            line = line.replace(char, expected[open_char], 1)
        
        # Fix specific patterns that cause delimiter issues
        if 'this file contains an unclosed delimiter' in line:
            continue  # Skip this error message
            
        # Fix unclosed attribute brackets
        if line.strip().startswith('#[') and not line.strip().endswith(']'):
            if '"' in line and not line.count('"') % 2 == 0:
                # Fix unclosed quotes in attributes
                line = line.replace('#[path = ', '#[path = "').rstrip() + '"]'
        
        # Fix function signatures with mismatched brackets
        if 'fn test_' in line and '{' in line and '}' in line:
            # Make sure function signature is properly formed
            if line.count('{') > line.count('}'):
                # Add missing closing brace on next line
                fixed_lines.append(line)
                fixed_lines.append('}')
                continue
        
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def fix_specific_file_issues(filepath, content):
    """Fix issues specific to certain files."""
    
    filename = os.path.basename(filepath)
    
    if 'result_system_test.rs' in filename:
        # Fix the use statement syntax errors
        content = re.sub(r'use cursed::types::result::\{([^}]*)\}', r'use cursed::types::result::{*}', content)
        content = re.sub(r'use cursed::error::types::\{\}', 'use cursed::error::types::*;', content)
        content = re.sub(r'use cursed::stdlib::errors::\{\}', 'use cursed::stdlib::errors::*;', content)
        content = re.sub(r'use cursed::parser::result_types::\{\}', 'use cursed::parser::result_types::*;', content)
    
    if 'quick_test_enhanced_test.rs' in filename:
        # Fix bracket mismatches in this file
        content = re.sub(r'\]\s*fn', '] fn', content)
        content = re.sub(r'#\[test\]\s*fn', '#[test]\nfn', content)
    
    return content

def fix_test_file(filepath):
    """Fix a single test file with remaining issues."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply fixes
        content = fix_delimiter_issues(content)
        content = fix_specific_file_issues(filepath, content)
        
        # Only write if we made changes
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
            
        return False
        
    except Exception as e:
        print(f"Error fixing {filepath}: {e}")
        return False

def main():
    """Fix test files with remaining delimiter issues."""
    
    # Focus on files that were mentioned in the error output
    problem_files = [
        'interface_type_assertion_integrated_test.rs',
        'quick_test_enhanced_test.rs', 
        'interface_type_assertion_complex_error_chain_test.rs',
        'interface_registry_visualization_integration_test.rs',
        'enhanced_gc_stress_test.rs',
        'comprehensive_circular_references_test.rs',
        'interface_type_assertion_path_visualization_standalone_test.rs',
        'channel_range_test.rs',
        'constrained_generics_performance_test.rs',
        'error_propagation_integration_test.rs',
        'interface_type_assertion_path_visualization_integration_enhanced_test.rs',
        'interface_registry_performance_test.rs',
        'result_system_test.rs',
        'documentation_ast_test.rs',
        'channels_simple_test.rs',
        'import_llvm_integration_test.rs',
        'web_vibez_test.rs',
        'llvm_expression_test.rs',
        'crypto_integration_test.rs',
        'range_clause_error_recovery_simple_test.rs',
        'error_handling_basic_test.rs',
        'import_end_to_end_test.rs',
        'documentation_simple_test.rs',
        'channel_close_runtime_test.rs',
        'comprehensive_map_parsing_test.rs',
        'interface_registry_extension_visualization_test.rs',
        'interface_type_assertion_debug_test.rs',
        'map_operations_test_fixed.rs',
        'interface_dynamic_dispatch_full_test.rs'
    ]
    
    fixed_count = 0
    
    for filename in problem_files:
        filepath = f'tests/{filename}'
        if os.path.exists(filepath):
            if fix_test_file(filepath):
                fixed_count += 1
                print(f"Fixed: {filename}")
    
    print(f"\nFixed {fixed_count} problem files")

if __name__ == "__main__":
    main()
