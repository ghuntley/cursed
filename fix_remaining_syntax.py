#!/usr/bin/env python3

import os
import re
import glob

def fix_remaining_issues(content):
    """Fix remaining specific syntax issues"""
    
    # Fix specific patterns that weren't caught before
    fixes = [
        # Fix extra closing braces in struct literals
        (r'\{([^}]*),\}\}', r'{ \1 }'),
        (r'\{([^}]*)\}\}', r'{ \1 }'),
        
        # Fix Box::new with extra closing parens
        (r'Box::new\(([^)]*)\)\)\)', r'Box::new(\1)'),
        (r'Box::new\(([^)]*)\)\)', r'Box::new(\1)'),
        
        # Fix variable assignments with extra commas
        (r'value: ([^,}]*),\}', r'value: \1 }'),
        
        # Fix missing semicolons in statements
        (r'(\w+)\(\)\)\)', r'\1();'),
        
        # Fix string literals with backslashes
        (r'"\\ "([^"]*)"', r'"\1"'),
        (r'" ([^"]*)\\ "([^"]*)"', r'"\1 \2"'),
        
        # Fix unclosed delimiters in use statements
        (r'use ([^;]*)\;', r'use \1;'),
        (r'use ([^{]*)\{([^}]*);', r'use \1::\2;'),
        
        # Fix function calls with missing parens
        (r'(\w+)\.([a-zA-Z_]\w*)\(\)\)\)', r'\1.\2()'),
        
        # Fix assert macro calls
        (r'assert!\(([^)]*)\)\}', r'assert!(\1);'),
        (r'assert!\(([^)]*);', r'assert!(\1);'),
        
        # Fix vector literals
        (r'vec!\[([^\]]*)\]\]', r'vec![\1]'),
        (r'vec!\[([^\]]*)\]0\]', r'vec![\1]'),
        (r'vec!\[([^\]]*)\]e\]', r'vec![\1]'),
        
        # Fix string interpolation issues
        (r'"\{([^}]*)\}"', r'"{\1}"'),
        
        # Fix raw string delimiters
        (r'r#\s*\n([^#]*)\n', r'r#"\1"#'),
        
        # Fix macro calls with extra delimiters
        (r'info!\(([^)]*)\)\)', r'info!(\1);'),
        (r'debug!\(([^)]*)\)\)', r'debug!(\1);'),
        (r'println!\(([^)]*)\)\)', r'println!(\1);'),
        
        # Fix struct field assignments
        (r'(\w+): ([^,}]*),\}', r'\1: \2 }'),
        
        # Fix return statements
        (r'Ok\(([^)]*)\)\}', r'Ok(\1)'),
        
        # Fix loop statements
        (r'for ([^{]*)\{;', r'for \1 {'),
        (r'while ([^{]*)\{', r'while \1 {'),
        
        # Fix if statements
        (r'if ([^{]*)\{', r'if \1 {'),
        
        # Fix match statements
        (r'match ([^{]*)\{', r'match \1 {'),
        
        # Fix function definitions
        (r'fn ([a-zA-Z_]\w*)\([^{]*\{', r'fn \1() {'),
        
        # Fix struct definitions
        (r'struct ([a-zA-Z_]\w*)\s*\{', r'struct \1 {'),
        
        # Fix enum definitions
        (r'enum ([a-zA-Z_]\w*)\s*\{', r'enum \1 {'),
        
        # Fix impl blocks
        (r'impl ([^{]*)\{', r'impl \1 {'),
        
        # Fix generic angle brackets
        (r'<([^>]*)>\)', r'<\1>'),
        (r'<([^>]*)>\]', r'<\1>'),
        (r'<([^>]*)>\}', r'<\1>'),
        
        # Fix common expression patterns
        (r'\.unwrap\(\)\)', r'.unwrap()'),
        (r'\.expect\("([^"]*)"\)\)', r'.expect("\1")'),
        (r'\.is_ok\(\)\)', r'.is_ok()'),
        (r'\.is_err\(\)\)', r'.is_err()'),
        (r'\.is_some\(\)\)', r'.is_some()'),
        (r'\.is_none\(\)\)', r'.is_none()'),
        (r'\.len\(\)\)', r'.len()'),
        (r'\.is_empty\(\)\)', r'.is_empty()'),
        
        # Fix attribute syntax
        (r'#\[([^\]]*)\]\)', r'#[\1]'),
        (r'#\[([^\]]*)\];', r'#[\1]'),
        
        # Fix module declarations
        (r'mod ([a-zA-Z_]\w*)\s*\{;', r'mod \1 {'),
        
        # Fix trailing semicolons
        (r'\}\s*;\s*$', r'}'),
        
        # Fix double closing braces
        (r'\}\s*\}', r'}'),
        
        # Fix double closing brackets
        (r'\]\s*\]', r']'),
        
        # Fix double closing parens
        (r'\)\s*\)', r')'),
        
        # Clean up whitespace
        (r'\s+;', r';'),
        (r'\s+\}', r'}'),
        (r'\s+\]', r']'),
        (r'\s+\)', r')'),
        (r'\{\s+', r'{'),
        (r'\[\s+', r'['),
        (r'\(\s+', r'('),
    ]
    
    for pattern, replacement in fixes:
        content = re.sub(pattern, replacement, content)
    
    return content

def fix_specific_test_files(content, filename):
    """Fix issues specific to certain test files"""
    
    # Fix specific issues based on filename
    if 'formatter_unit_test.rs' in filename:
        # Fix string escape issues
        content = re.sub(r'"\\ "([^"]*)"', r'"\1"', content)
        content = re.sub(r'\\([;}])', r'\1', content)
        
    if 'control_flow_execution_test.rs' in filename:
        # Fix import issues
        content = re.sub(r'use ([^;]*)\{([^}]*);', r'use \1::\2;', content)
        
    if 'interface_' in filename and '_test.rs' in filename:
        # Fix interface-related test issues
        content = re.sub(r'"([^"]*)";"', r'"\1"', content)
        
    return content

def fix_test_file(filepath):
    """Fix syntax errors in a single test file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply fixes
        content = fix_remaining_issues(content)
        content = fix_specific_test_files(content, filepath)
        
        # Only write if changes were made
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"Fixed: {filepath}")
            return True
        else:
            print(f"No changes needed: {filepath}")
            return False
            
    except Exception as e:
        print(f"Error fixing {filepath}: {e}")
        return False

def main():
    """Fix remaining syntax errors in all test files"""
    test_patterns = [
        'tests/**/*.rs',
        'tests/*.rs'
    ]
    
    fixed_count = 0
    total_count = 0
    
    for pattern in test_patterns:
        for filepath in glob.glob(pattern, recursive=True):
            if os.path.isfile(filepath):
                total_count += 1
                if fix_test_file(filepath):
                    fixed_count += 1
    
    print(f"\nSummary: Fixed {fixed_count} out of {total_count} test files")

if __name__ == "__main__":
    main()
