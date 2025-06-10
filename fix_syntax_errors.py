#!/usr/bin/env python3
import os
import re

def fix_string_concatenation_errors(content):
    """Fix string literal concatenation errors and missing commas"""
    
    # Fix patterns like: "first"".to_string() -> "first".to_string()
    content = re.sub(r'"([^"]+)""\.to_string\(\)', r'"\1".to_string()', content)
    
    # Fix patterns like: "text" "more_text" -> "text", "more_text"
    content = re.sub(r'"([^"]+)"\s+"([^"]+)"', r'"\1", "\2"', content)
    
    # Fix patterns like: Object::String( "text"".to_string()) -> Object::String("text".to_string())
    content = re.sub(r'Object::String\(\s*"([^"]+)""\.to_string\(\)\)', r'Object::String("\1".to_string())', content)
    
    # Fix patterns like: "text" "other_text".to_string() -> "text".to_string(), "other_text".to_string()
    content = re.sub(r'"([^"]+)"\s+"([^"]+)"\.to_string\(\)', r'"\1".to_string(), "\2".to_string()', content)
    
    # Fix patterns like: format_with_args("%"x" -> format_with_args("%x",
    content = re.sub(r'format_with_args\("%"([^"]+)"\s*,', r'format_with_args("%\1",', content)
    
    # Fix patterns like: assert_eq!(result, "text"42) -> assert_eq!(result, "text", 42)
    content = re.sub(r'assert_eq!\(([^,]+),\s*"([^"]+)"(\d+)', r'assert_eq!(\1, "\2", \3', content)
    
    # Fix patterns like: ("text", "other_text"") -> ("text", "other_text")
    content = re.sub(r'\("([^"]+)",\s*"([^"]+)""\)', r'("\1", "\2")', content)
    
    # Fix patterns like: .contains( "text" ") -> .contains("text")
    content = re.sub(r'\.contains\(\s*"([^"]+)"\s*"\)', r'.contains("\1")', content)
    
    # Fix missing opening parentheses: new()) -> new()
    content = re.sub(r'::new\(\)\)', r'::new()', content)
    
    # Fix patterns like: Token::new(TokenType::Something, "{"), -> Token::new(TokenType::Something, "{")
    content = re.sub(r'Token::new\(([^,]+),\s*"([^"]+)"\),', r'Token::new(\1, "\2")', content)
    
    # Fix patterns like: "text" .something() -> "text".something()
    content = re.sub(r'"([^"]+)"\s+\.([a-zA-Z_][a-zA-Z0-9_]*)', r'"\1".\2', content)
    
    return content

def fix_unicode_characters(content):
    """Fix unicode characters that cause compilation errors"""
    # Replace checkmark unicode with regular text
    content = content.replace('✓', 'OK')
    return content

def fix_literal_errors(content):
    """Fix various literal and token errors"""
    
    # Fix underscore literal suffix errors: "_" " -> "_"
    content = re.sub(r'"_"\s*"', r'"_"', content)
    
    # Fix character literal errors: 'text' -> "text"
    content = re.sub(r"'([^']{2,})'", r'"\1"', content)
    
    # Fix raw string delimiter errors: r# "text" -> r#"text"#
    content = re.sub(r'r#\s*"([^"]+)"\s*([^#])', r'r#"\1"# \2', content)
    
    # Fix escape sequence errors: \ " -> \"
    content = re.sub(r'\\"\s*"', r'\\"', content)
    
    return content

def fix_delimiter_errors(content):
    """Fix mismatched delimiter errors"""
    
    # Fix common patterns with extra closing delimiters
    lines = content.split('\n')
    fixed_lines = []
    
    for line in lines:
        # Fix patterns like: function_name(arg1, arg2)) -> function_name(arg1, arg2)
        line = re.sub(r'\)\)', r')', line)
        
        # Fix patterns like: .expect( "text" to "other_text) -> .expect("text to other_text")
        line = re.sub(r'\.expect\(\s*"([^"]+)"\s+to\s+"([^"]+)\)', r'.expect("\1 to \2")', line)
        
        # Fix patterns like: info!("text" something "other_text" ) -> info!("text something other_text")
        line = re.sub(r'info!\("([^"]+)"\s+([^"]+)\s+"([^"]+)"\s*\)', r'info!("\1 \2 \3")', line)
        
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def fix_import_errors(content):
    """Fix import and path errors"""
    
    # Fix common module path issues
    content = re.sub(r'#\[path = "([^"]+)\.([^"]+)"\s*\]', r'#[path = "\1.\2"]', content)
    
    return content

def fix_test_file(filepath):
    """Fix a single test file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_string_concatenation_errors(content)
        content = fix_unicode_characters(content)
        content = fix_literal_errors(content)
        content = fix_delimiter_errors(content)
        content = fix_import_errors(content)
        
        # Only write if we made changes
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
            return True
        
        return False
        
    except Exception as e:
        print(f"Error fixing {filepath}: {e}")
        return False

def main():
    """Fix syntax errors in all test files"""
    tests_dir = "tests"
    fixed_count = 0
    total_count = 0
    
    for root, dirs, files in os.walk(tests_dir):
        for file in files:
            if file.endswith('.rs'):
                filepath = os.path.join(root, file)
                total_count += 1
                if fix_test_file(filepath):
                    fixed_count += 1
    
    print(f"\nFixed {fixed_count} out of {total_count} test files")

if __name__ == "__main__":
    main()
