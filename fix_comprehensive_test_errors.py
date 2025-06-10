#!/usr/bin/env python3
"""
Comprehensive fix for test compilation errors
"""

import os
import re

def fix_crypto_integration_test():
    """Fix the crypto integration test file"""
    file_path = 'tests/crypto_integration_test.rs'
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix all .expect() calls
    content = re.sub(
        r'setup_crypto_packages\(\)\.expect\(\s*"[^"]*"\s*to\s*setup\s*crypto\s*"[^"]*"\);',
        'setup_crypto_packages().expect("Failed to setup crypto packages");',
        content
    )
    
    # Fix register_cipher calls
    content = re.sub(r'register_cipher\(\s*"test"-([^"]*)",', r'register_cipher("test-\1",', content)
    content = re.sub(r'get_cipher\(\s*"test"-([^"]*)\)', r'get_cipher("test-\1")', content)
    
    # Fix byte string literals
    content = re.sub(r'b\s*"([^"]*)"[^";]*"([^"]*)"', r'b"\1 \2"', content)
    
    # Fix info! calls with string literals
    content = re.sub(r'info!\("Testing:\s*([^"]*)"([^"]*)"([^"]*)"\);', r'info!("Testing: \1\2\3");', content)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"✓ Fixed {file_path}")

def fix_generic_interface_registry_test():
    """Fix the generic interface registry test file"""
    file_path = 'tests/generic_interface_registry_test.rs'
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix imports
    content = content.replace(
        'use cursed::core::interface_registry::{InterfaceRegistry, GenericInterfaceImpl};',
        'use cursed::interfaces::{InterfaceRegistry, GenericInterfaceImpl};'
    )
    
    # Fix Type::Struct calls - remove the first invalid string argument
    content = re.sub(
        r'Type::Struct\(\s*"[^"]*""\.to_string\(\),\s*',
        'Type::Struct("".to_string(), ',
        content
    )
    
    # Fix string literal patterns in various contexts
    patterns_to_fix = [
        ('GenericStack', 'GenericStack'),
        ('Container', 'Container'),
        ('Stack', 'Stack'),
        ('List', 'List'),
        ('Pair', 'Pair'),
        ('Custom', 'Custom'),
        ('Result', 'Result'),
        ('Monad', 'Monad'),
        ('Error', 'Error'),
        ('ErrorType', 'ErrorType'),
        ('KeyValuePair', 'KeyValuePair'),
        ('Storable', 'Storable'),
        ('Comparable', 'Comparable'),
        ('Serializable', 'Serializable'),
    ]
    
    for pattern, replacement in patterns_to_fix:
        # Fix cases like: Container""
        content = re.sub(
            rf'\b{pattern}""',
            f'"{replacement}"',
            content
        )
        # Fix cases like: vec![ K"".to_string(),
        content = re.sub(
            rf'\[\s*{pattern[0]}"".to_string\(\)',
            f'["{pattern[0]}".to_string()',
            content
        )
    
    # Fix vec! macro issues - add quotes around single letters
    content = re.sub(r'vec!\[\s*([A-Z])"".to_string\(\)', r'vec!["\1".to_string()', content)
    
    # Fix constraint tuples
    content = re.sub(
        r'\(\s*([A-Z])"".to_string\(\),\s*([A-Za-z]+)"".to_string\(\)',
        r'("\1".to_string(), "\2".to_string())',
        content
    )
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"✓ Fixed {file_path}")

def fix_specialization_generation_test():
    """Fix the specialization generation test"""
    file_path = 'tests/specialization_generation_test.rs'
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix PathBuf::from calls with test function names
    content = re.sub(
        r'PathBuf::from\(\s*test_struct_specialization"\.csd"\)',
        'PathBuf::from("test_struct_specialization.csd")',
        content
    )
    
    # Fix Token::new calls - they should have only token type and position parameters
    content = re.sub(
        r'Token::new\(TokenType::Plus,\s*"Plus""\)',
        'Token::new(TokenType::Plus, 0)',
        content
    )
    
    # Fix TypeParameter::new calls - remove extra arguments
    content = re.sub(
        r'TypeParameter::new\(\s*Token::new\([^)]+\),\s*"T""\.to_string\(\),',
        'TypeParameter::new("T".to_string(),',
        content
    )
    
    # Add missing token fields to struct initializers
    content = re.sub(
        r'(ReturnStatement\s*{\s*)(return_value:)',
        r'\1token: Token::new(TokenType::Yolo, 0), \2',
        content
    )
    
    content = re.sub(
        r'(FunctionStatement\s*{\s*)(name:)',
        r'\1token: Token::new(TokenType::Slay, 0), \2',
        content
    )
    
    content = re.sub(
        r'(FieldStatement\s*{\s*)(name:)',
        r'\1token: Token::new(TokenType::Identifier, 0), \2',
        content
    )
    
    content = re.sub(
        r'(SquadStatement\s*{\s*)(name:)',
        r'\1token: Token::new(TokenType::Squad, 0), \2',
        content
    )
    
    # Fix string literals
    content = re.sub(r'"placeholder""', '"placeholder"', content)
    content = re.sub(r'"T""', '"T"', content)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"✓ Fixed {file_path}")

def fix_interface_path_visualization_test():
    """Fix the interface path visualization test"""
    file_path = 'tests/interface_type_assertion_path_visualization_adapter_test.rs'
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Fix string literals with single letters
    letters = ['X', 'Y', 'Z', 'A', 'B', 'C', 'D']
    for letter in letters:
        content = re.sub(rf'\b{letter}""', f'"{letter}"', content)
    
    # Fix specific patterns
    content = re.sub(r'"MockType""', '"MockType"', content)
    content = re.sub(r'"Enhanced""', '"Enhanced"', content)
    content = re.sub(r'"Mock""', '"Mock"', content)
    content = re.sub(r'"Base""', '"Base"', content)
    
    # Fix test function call pattern
    content = re.sub(r'test"\.csd:123"', '"test.csd:123"', content)
    
    # Fix contains() calls
    content = re.sub(
        r'assert!\(([^.]+)\.contains\(\s*"([^"]+)"\s*"\s*([^"]*?)"\s*\)\);',
        r'assert!(\1.contains("\2 \3"));',
        content
    )
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"✓ Fixed {file_path}")

def main():
    """Main function to fix all test files"""
    print("🔧 Fixing comprehensive test compilation errors...")
    
    fix_crypto_integration_test()
    fix_generic_interface_registry_test() 
    fix_specialization_generation_test()
    fix_interface_path_visualization_test()
    
    print("✅ Completed comprehensive test fixes")

if __name__ == "__main__":
    main()
