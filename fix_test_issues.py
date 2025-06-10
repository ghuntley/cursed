#!/usr/bin/env python3
"""
Fix common test compilation issues in CURSED project
"""
import os
import re
import glob
from pathlib import Path

def fix_token_patterns(content):
    """Fix Token::new pattern matching issues"""
    # Replace Token::new patterns with proper enum patterns
    patterns = [
        (r'Token::new\(TokenType::(\w+), _\)', r'Token::Type(\1)'),
        (r'Token::new\(TokenType::(\w+), "([^"]+)"\)', r'Token::Value(\1, "\2".to_string())'),
        (r'Token::new\(TokenType::(\w+), &(\w+)\)', r'Token::Value(\1, \2.clone())'),
    ]
    
    for old_pattern, replacement in patterns:
        content = re.sub(old_pattern, replacement, content)
    
    return content

def fix_string_literal_issues(content):
    """Fix unknown prefix and escape sequence issues"""
    # Fix unknown prefix issues by adding spaces
    content = re.sub(r'"([^"]*?)([a-zA-Z_][a-zA-Z0-9_]*)"', lambda m: f'"{m.group(1)}{m.group(2)} "', content)
    
    # Fix escape sequences in strings
    content = re.sub(r'"\\n([^"]*)"', r'"\n\1"', content)
    content = re.sub(r'"\\t([^"]*)"', r'"\t\1"', content)
    
    return content

def fix_import_issues(content):
    """Fix common unresolved import issues"""
    import_fixes = {
        'cursed::memory::AdaptationParameters': 'cursed::memory::gc::AdaptationParameters',
        'cursed::lexer::token': 'cursed::lexer::Token',
        'cursed::core::goroutine': 'cursed::runtime::goroutine',
        'cursed::stdlib::quick_test': 'cursed::stdlib::test_vibes',
        'cursed::object_thread_safe': 'cursed::runtime::object_thread_safe',
        'cursed::core::interface_registry_extensions': 'cursed::codegen::llvm::interface_registry_extensions',
        'cursed::core::interface_registry': 'cursed::codegen::llvm::interface_registry',
    }
    
    for old_import, new_import in import_fixes.items():
        content = content.replace(f'use {old_import}', f'use {new_import}')
        content = content.replace(f'{old_import}::', f'{new_import}::')
    
    return content

def fix_struct_field_issues(content):
    """Fix common struct field issues"""
    # Fix StanExpression struct usage
    content = re.sub(
        r'StanExpression\s*\{\s*token:\s*([^,]+),\s*expression:\s*([^}]+)\s*\}',
        r'StanExpression { token: \1, call: \2 }',
        content
    )
    
    # Fix QualifiedName::new calls  
    content = re.sub(
        r'QualifiedName::new\(\s*([^,]+),\s*([^,]+)\s*\)',
        r'QualifiedName::new(\1, \2, QualifiedSymbolKind::Function)',
        content
    )
    
    return content

def fix_method_issues(content):
    """Fix missing method issues"""
    # Replace missing methods with available alternatives
    method_replacements = {
        'get_expression_type': 'get_type',
        'symbol': 'name',
        'symbol_kind': 'kind',
        'freed_objects': 'total_objects_collected',
        'active_count': 'get_active_count',
    }
    
    for old_method, new_method in method_replacements.items():
        content = re.sub(f'\.{old_method}\\b', f'.{new_method}', content)
    
    return content

def fix_test_file(file_path):
    """Fix a single test file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply fixes in order
        content = fix_import_issues(content)
        content = fix_token_patterns(content)
        content = fix_string_literal_issues(content)
        content = fix_struct_field_issues(content)
        content = fix_method_issues(content)
        
        # Only write if content changed
        if content != original_content:
            # Create backup
            backup_path = file_path + '.backup'
            if not os.path.exists(backup_path):
                with open(backup_path, 'w', encoding='utf-8') as f:
                    f.write(original_content)
            
            # Write fixed content
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            return True
        
        return False
    
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def main():
    """Main fix function"""
    print("Fixing CURSED test compilation issues...")
    
    # Get all test files
    test_files = glob.glob("tests/*.rs")
    
    # Focus on commonly failing test patterns first
    priority_tests = [
        "tests/minimal_interface_test.rs",
        "tests/stan_simple_test.rs", 
        "tests/gc_simple_test.rs",
        "tests/simple_qualified_name_test.rs",
        "tests/goroutine_comprehensive_test.rs",
    ]
    
    fixed_count = 0
    
    # Fix priority tests first
    for test_file in priority_tests:
        if os.path.exists(test_file):
            print(f"Fixing {test_file}...")
            if fix_test_file(test_file):
                fixed_count += 1
                print(f"  ✅ Fixed")
            else:
                print(f"  ℹ️  No changes needed")
    
    print(f"\nFixed {fixed_count} test files")
    print("Run tests again to see improvements")

if __name__ == "__main__":
    main()
