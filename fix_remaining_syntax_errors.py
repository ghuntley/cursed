#!/usr/bin/env python3
import os
import re

def fix_additional_patterns(content):
    """Fix additional syntax error patterns found in the tests"""
    
    # Fix path attributes: #[path = "common/mod."rs"] -> #[path = "common/mod.rs"]
    content = re.sub(r'#\[path = "([^"]+)\.([^"]+)"\s*\]', r'#[path = "\1.\2"]', content)
    
    # Fix .to_string() patterns: Object::String(message.to_string())] -> Object::String(message.to_string())]
    content = re.sub(r'Object::String\(([^)]+)\)\]', r'Object::String(\1)]', content)
    
    # Fix trailing closing delimiters: Ok(()) -> Ok(())
    content = re.sub(r'Ok\(\(\)', r'Ok(())', content)
    
    # Fix missing opening parens in expressions: format!( "text" to "other") -> format!("text to other")
    content = re.sub(r'format!\(\s*"([^"]+)"\s+to\s+"([^"]+)"\)', r'format!("\1 to \2")', content)
    
    # Fix info!/debug!/error! macros: info!("text" something "other") -> info!("text something other")
    content = re.sub(r'(info|debug|error|warn)!\("([^"]+)"\s+([^"]+)\s+"([^"]+)"\s*\)', r'\1!("\2 \3 \4")', content)
    
    # Fix contains patterns: .contains( "text" ") -> .contains("text")
    content = re.sub(r'\.contains\(\s*"([^"]+)"\s+"\)', r'.contains("\1")', content)
    
    # Fix vec! with mismatched brackets: vec![Arc::new(Object -> vec![Arc::new(Object
    content = re.sub(r'vec!\[Arc::new\(Object::([^(]+)\(([^)]+)\)\]', r'vec![Arc::new(Object::\1(\2))]', content)
    
    # Fix assert patterns: assert!(something(); -> assert!(something());
    content = re.sub(r'assert!\(([^;]+);\)', r'assert!(\1);', content)
    
    # Fix module.add_function patterns: .add_function( "name", ") -> .add_function("name",
    content = re.sub(r'\.add_function\(\s*"([^"]+)",\s*"\)', r'.add_function("\1",', content)
    
    # Fix incomplete string concatenations: "part1" "part2 -> "part1part2"
    content = re.sub(r'"([^"]+)"\s+"([^"]+)', r'"\1\2', content)
    
    # Fix escaped quotes in strings: \"n\" -> \n
    content = re.sub(r'\\"n\\"', r'\\n', content)
    content = re.sub(r'\\"([a-z])\\"', r'\\\1', content)
    
    # Fix character literals with multiple chars: 'text' -> "text"
    content = re.sub(r"'([^']{2,})'", r'"\1"', content)
    
    # Fix trailing comma issues: something(), -> something()
    content = re.sub(r'\(\),\s*$', r'()', content, flags=re.MULTILINE)
    
    # Fix unterminated strings by adding closing quotes at line ends
    lines = content.split('\n')
    fixed_lines = []
    for line in lines:
        # Count quotes to detect unterminated strings
        if line.count('"') % 2 == 1 and not line.strip().endswith('\\'):
            # Add closing quote if odd number of quotes
            line = line.rstrip() + '"'
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def fix_specific_error_patterns(content):
    """Fix specific error patterns found in the compilation output"""
    
    # Fix r# raw string patterns: r#"text"# -> r#"text"#
    content = re.sub(r'r#\s*"([^"]+)"\s*([^#])', r'r#"\1"# \2', content)
    
    # Fix Box::leak patterns: Box::leak(Box::new(context); -> Box::leak(Box::new(context))
    content = re.sub(r'Box::leak\(Box::new\(([^)]+)\);', r'Box::leak(Box::new(\1))', content)
    
    # Fix duration patterns: Duration::from_millis(100); -> Duration::from_millis(100)
    content = re.sub(r'Duration::from_millis\((\d+)\);', r'Duration::from_millis(\1)', content)
    
    # Fix match patterns: matches!(result, Error); -> matches!(result, Error)
    content = re.sub(r'matches!\(([^,]+),\s*([^)]+)\);', r'matches!(\1, \2)', content)
    
    # Fix .contains patterns with extra quotes: .contains(&"text"") -> .contains(&"text")
    content = re.sub(r'\.contains\(&\s*"([^"]+)""\)', r'.contains(&"\1")', content)
    
    # Fix get function patterns: .get_function::<...>( "name", ") -> .get_function::<...>("name")
    content = re.sub(r'\.get_function::<[^>]+>\(\s*"([^"]+)",\s*"\)', r'.get_function::<>("\\1")', content)
    
    # Fix incomplete format strings: format!( "text" {}, -> format!("text {}", 
    content = re.sub(r'format!\(\s*"([^"]+)"\s+\{\},', r'format!("\1 {}",', content)
    
    # Fix literal suffix errors: "text"42 -> "text", 42
    content = re.sub(r'"([^"]+)"(\d+)', r'"\1", \2', content)
    
    # Fix thread::sleep patterns: thread::sleep(duration); -> thread::sleep(duration)
    content = re.sub(r'thread::sleep\(([^)]+)\);', r'thread::sleep(\1)', content)
    
    return content

def fix_delimiter_balance(content):
    """Fix common delimiter balance issues"""
    
    lines = content.split('\n')
    fixed_lines = []
    
    for line in lines:
        # Fix double closing parens: )) -> )
        line = re.sub(r'\)\)', r')', line)
        
        # Fix double closing brackets: ]] -> ]
        line = re.sub(r'\]\]', r']', line)
        
        # Fix double closing braces: }} -> }
        line = re.sub(r'\}\}', r'}', line)
        
        # Fix mismatched quote/bracket combinations: "] -> "
        line = re.sub(r'"\]', r'"', line)
        line = re.sub(r'"\)', r'"', line)
        
        # Fix incomplete function calls: function( -> function()
        if line.strip().endswith('(') and not line.strip().startswith('//'):
            line = line.rstrip('(') + '()'
        
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def fix_test_file(filepath):
    """Fix a single test file with additional patterns"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_additional_patterns(content)
        content = fix_specific_error_patterns(content)
        content = fix_delimiter_balance(content)
        
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
    """Fix remaining syntax errors in all test files"""
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
