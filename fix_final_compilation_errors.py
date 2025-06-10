#!/usr/bin/env python3
"""
Fix final compilation errors in CURSED codebase.
This fixes specific string literal and API issues found in the latest compilation run.
"""

import os
import re
import glob

def fix_string_literal_parsing(content):
    """Fix string literals that are being parsed as prefixed identifiers."""
    # Fix patterns like: info!("something"something) -> info!("something something")
    content = re.sub(
        r'(info!|debug!|warn!|error!|trace!)\("([^"]+)"([a-zA-Z_][a-zA-Z0-9_]*)\)',
        r'\1("\2 \3")',
        content
    )
    
    # Fix patterns like: "path/module" -> "path/module"
    content = re.sub(
        r'"([^"]+)"([a-zA-Z_][a-zA-Z0-9_]*)',
        r'"\1 \2"',
        content
    )
    
    # Fix format strings and backslashes
    content = re.sub(
        r'format!\("\\?\\"{}\\?\\"",',
        r'format!("{}".',
        content
    )
    
    return content

def fix_database_api_calls(content):
    """Fix database API method calls and parameters."""
    # Fix vec![] parameters to &[] for database calls
    content = re.sub(r'\.execute\(([^,]+),\s*vec!\[\]\)', r'.execute(\1, &[])', content)
    content = re.sub(r'\.query\(([^,]+),\s*vec!\[\]\)', r'.query(\1, &[])', content)
    
    # Fix parameter conversion from SqlValue to Parameter
    content = re.sub(
        r'vec!\[\s*(SqlValue::[^]]+)\s*\]',
        r'&[Parameter::from(\1)]',
        content
    )
    
    # Fix begin_transaction calls to include None parameter
    content = re.sub(r'\.begin_transaction\(\)\.await', r'.begin_transaction(None).await', content)
    
    # Fix Option<usize> display issues
    content = re.sub(r'result\.row_count\(\)', r'result.row_count().unwrap_or(0)', content)
    content = re.sub(r'result\.row_count\(\) as f64', r'result.row_count().unwrap_or(0) as f64', content)
    content = re.sub(r'result\.row_count\(\) >', r'result.row_count().unwrap_or(0) >', content)
    
    # Fix rows() method calls
    content = re.sub(r'result\.rows\(\)', r'result.next().unwrap()', content)
    
    # Fix method calls on database connections
    content = re.sub(r'\.sender\(\)\.sender\(\)\.close\(\)', r'.close()', content)
    
    return content

def fix_unterminated_strings(content):
    """Fix unterminated string literals."""
    # Look for lines with unterminated strings and fix them
    lines = content.split('\n')
    fixed_lines = []
    
    for line in lines:
        # Fix unterminated strings in info!() macros
        if 'info!(' in line and line.count('"') % 2 == 1:
            line = line + '"'
        
        # Fix other unterminated strings
        if line.count('"') % 2 == 1 and not line.strip().endswith('\\'):
            # Find the last quote and see if it's properly terminated
            if '"' in line and not line.strip().endswith('";'):
                line = line.rstrip() + '"'
        
        fixed_lines.append(line)
    
    return '\n'.join(fixed_lines)

def fix_format_string_errors(content):
    """Fix format string and macro call errors."""
    # Fix format! calls with invalid escapes
    content = re.sub(
        r'format!\("\\\"{}\\\"",\s*([^)]+)\)',
        r'format!("{}",\1)',
        content
    )
    
    # Fix missing closing delimiters in format strings
    content = re.sub(
        r'token:\s*format!\("([^"]+)",\s*([^)]+)\),',
        r'token: format!("\1", \2).to_string(),',
        content
    )
    
    return content

def fix_identifier_initialization(content):
    """Fix Identifier struct initialization issues."""
    # Fix missing 'token' field in Identifier structs
    content = re.sub(
        r'Identifier\s*{\s*value:\s*([^,}\n]+),?\s*}',
        r'Identifier {\n            token: "identifier".to_string(),\n            value: \1,\n        }',
        content
    )
    
    return content

def fix_arc_type_issues(content):
    """Fix Arc type annotation issues."""
    # Remove problematic Arc<T, A> type annotations
    content = re.sub(
        r'let\s+(\w+):\s*Arc<[^>]+>\s*=\s*Arc::clone\(([^)]+)\);',
        r'let \1 = Arc::clone(\2);',
        content
    )
    
    return content

def fix_method_call_chains(content):
    """Fix problematic method call chains."""
    # Fix channel method calls
    content = re.sub(r'\.sender\(\)\.send\(', r'.send(', content)
    content = re.sub(r'\.receiver\(\)\.receive\(', r'.receive(', content)
    content = re.sub(r'\.sender\(\)\.close\(', r'.close(', content)
    
    return content

def process_test_file(filepath):
    """Process a single test file and apply all fixes."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_string_literal_parsing(content)
        content = fix_database_api_calls(content)
        content = fix_unterminated_strings(content)
        content = fix_format_string_errors(content)
        content = fix_identifier_initialization(content)
        content = fix_arc_type_issues(content)
        content = fix_method_call_chains(content)
        
        # Only write if changes were made
        if content != original_content:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"✅ Fixed: {filepath}")
            return True
        else:
            print(f"⏭️  No changes: {filepath}")
            return False
            
    except Exception as e:
        print(f"❌ Error processing {filepath}: {e}")
        return False

def main():
    """Main function to process all test files."""
    print("🔧 Fixing final compilation errors...")
    
    # Find all test files
    test_files = []
    test_files.extend(glob.glob("tests/*.rs"))
    test_files.extend(glob.glob("tests/**/*.rs", recursive=True))
    
    # Focus on files that were mentioned in the error output
    priority_files = [
        "tests/database_stress_tests.rs",
        "tests/map_functionality_integration_test.rs",
        "tests/import_llvm_integration_test.rs",
        "tests/basic_float_conversions_test.rs",
        "tests/interface_dynamic_dispatch_test.rs",
        "tests/database_llvm_integration_test.rs",
        "tests/crypto_asymmetric_test.rs",
        "tests/crypto_symmetric_test.rs"
    ]
    
    # Process priority files first
    fixed_count = 0
    total_count = 0
    
    for test_file in priority_files:
        if os.path.exists(test_file):
            total_count += 1
            if process_test_file(test_file):
                fixed_count += 1
    
    # Process remaining files
    for test_file in test_files:
        if test_file not in priority_files:
            total_count += 1
            if process_test_file(test_file):
                fixed_count += 1
    
    print(f"\n📊 Summary:")
    print(f"   Total files: {total_count}")
    print(f"   Fixed files: {fixed_count}")
    print(f"   Unchanged: {total_count - fixed_count}")
    
    if fixed_count > 0:
        print(f"\n✅ Fixed {fixed_count} test files. Try running tests again.")
    else:
        print(f"\n⚠️  No files needed fixing. Manual intervention may be required.")

if __name__ == "__main__":
    main()
