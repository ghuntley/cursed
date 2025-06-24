#!/usr/bin/env python3

import os
import re
import subprocess
from pathlib import Path

def extract_all_error_files():
    """Extract all files with Error type issues from current compilation"""
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True)
        
        files_with_line_numbers = []
        lines = result.stderr.split('\n')
        
        i = 0
        while i < len(lines):
            line = lines[i]
            
            # Look for error messages about missing Error type
            if 'cannot find type `Error` in this scope' in line:
                # Look back for the file location
                for j in range(max(0, i-5), i):
                    if '-->' in lines[j] and '.rs:' in lines[j]:
                        match = re.search(r'-->\s*([^:]+\.rs):(\d+):', lines[j])
                        if match:
                            file_path = match.group(1)
                            line_num = int(match.group(2))
                            if os.path.exists(file_path) and not file_path.startswith('/home/ghuntley/.cargo/'):
                                files_with_line_numbers.append((file_path, line_num))
                            break
            i += 1
        
        return files_with_line_numbers
    except Exception as e:
        print(f"Error extracting files: {e}")
        return []

def fix_error_in_file(file_path: str, line_number: int) -> bool:
    """Fix Error type in a specific file at a specific line"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if Error import already exists
        if any(pattern in content for pattern in [
            'use crate::error::Error',
            'use super::Error',
            'pub enum Error',
            'pub type Error'
        ]):
            return False
        
        lines = content.split('\n')
        
        # Insert import at the top after other imports
        insert_index = 0
        for i, line in enumerate(lines):
            stripped = line.strip()
            if stripped.startswith('use ') or stripped.startswith('extern crate'):
                insert_index = i + 1
            elif stripped and not stripped.startswith('//') and not stripped.startswith('#'):
                break
        
        # Choose appropriate import based on file location
        if '/stdlib/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/src/core/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/src/runtime/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/src/codegen/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/src/parser/' in file_path:
            import_stmt = 'use crate::error::Error;'
        elif '/src/ast/' in file_path:
            import_stmt = 'use crate::error::Error;'
        else:
            import_stmt = 'use crate::error::Error;'
        
        # Insert the import
        lines.insert(insert_index, import_stmt)
        
        # Write back to file
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        
        return True
        
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def fix_specific_error_types():
    """Fix specific error types that need special handling"""
    
    specific_fixes = [
        # ErrorManager and ErrorSeverity in errors.rs
        ('src/stdlib/errors.rs', [
            'pub enum ErrorSeverity {',
            '    Low,',
            '    Medium,', 
            '    High,',
            '    Critical,',
            '}',
            '',
            'pub struct ErrorManager {',
            '    errors: Vec<crate::error::Error>,',
            '    severity: ErrorSeverity,',
            '}',
            '',
            'impl ErrorManager {',
            '    pub fn new() -> Self {',
            '        Self {',
            '            errors: Vec::new(),',
            '            severity: ErrorSeverity::Low,',
            '        }',
            '    }',
            '}',
            ''
        ]),
        
        # JsonError in json_tea
        ('src/stdlib/json_tea/value.rs', [
            'pub type JsonError = crate::error::Error;',
            ''
        ]),
        
        # NetError in vibe_net
        ('src/stdlib/vibe_net/conn.rs', [
            'pub type NetError = crate::error::Error;',
            ''
        ]),
    ]
    
    fixed = 0
    
    for file_path, lines_to_add in specific_fixes:
        if os.path.exists(file_path):
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                # Check if definitions already exist
                needs_fix = False
                for line_to_add in lines_to_add:
                    if line_to_add.strip() and not any(line_to_add.strip() in content_line for content_line in content.split('\n')):
                        needs_fix = True
                        break
                
                if needs_fix:
                    # Find insertion point
                    content_lines = content.split('\n')
                    insert_index = 0
                    
                    for i, line in enumerate(content_lines):
                        stripped = line.strip()
                        if stripped.startswith('use ') or stripped.startswith('extern crate'):
                            insert_index = i + 1
                        elif stripped and not stripped.startswith('//') and not stripped.startswith('#'):
                            break
                    
                    # Insert the new definitions
                    for line_to_add in lines_to_add:
                        content_lines.insert(insert_index, line_to_add)
                        insert_index += 1
                    
                    # Write back to file
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write('\n'.join(content_lines))
                    
                    print(f"✓ Added specific error types to {file_path}")
                    fixed += 1
                else:
                    print(f"- Specific types already exist in {file_path}")
                    
            except Exception as e:
                print(f"Error fixing specific types in {file_path}: {e}")
    
    return fixed

def main():
    print("🔍 Extracting all remaining Error type issues...")
    
    # First fix specific error types
    print("\n📋 Fixing specific error types...")
    specific_fixed = fix_specific_error_types()
    
    # Then fix remaining Error imports
    print("\n📄 Extracting files with Error type issues...")
    error_files = extract_all_error_files()
    
    if not error_files:
        print("🎉 No Error type issues found!")
        return
    
    # Group by file
    file_errors = {}
    for file_path, line_num in error_files:
        if file_path not in file_errors:
            file_errors[file_path] = []
        file_errors[file_path].append(line_num)
    
    print(f"Found Error issues in {len(file_errors)} files")
    
    fixed = 0
    skipped = 0
    
    for file_path, line_numbers in file_errors.items():
        if fix_error_in_file(file_path, line_numbers[0]):
            print(f"✓ Fixed: {file_path} (lines: {', '.join(map(str, line_numbers))})")
            fixed += 1
        else:
            print(f"- Skipped: {file_path}")
            skipped += 1
    
    total_fixed = fixed + specific_fixed
    
    print(f"\n📊 Summary:")
    print(f"   Files with Error issues: {len(file_errors)}")
    print(f"   Files fixed with imports: {fixed}")
    print(f"   Files fixed with specific types: {specific_fixed}")
    print(f"   Total files fixed: {total_fixed}")
    print(f"   Files skipped: {skipped}")
    
    # Final compilation check
    print(f"\n🧪 Final compilation check...")
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True, timeout=90)
        
        remaining_error_issues = result.stderr.count("cannot find type `Error` in this scope")
        total_compilation_errors = result.stderr.count("error:")
        
        print(f"   Remaining Error type issues: {remaining_error_issues}")
        print(f"   Total compilation errors: {total_compilation_errors}")
        
        if remaining_error_issues == 0:
            print("🎉 All Error type issues resolved!")
        else:
            print(f"⚠️  {remaining_error_issues} Error type issues still remaining")
            
            # Show remaining error patterns
            remaining_patterns = {}
            for line in result.stderr.split('\n'):
                if 'cannot find type' in line and 'Error' in line:
                    match = re.search(r'cannot find type `([^`]*)`', line)
                    if match:
                        error_type = match.group(1)
                        remaining_patterns[error_type] = remaining_patterns.get(error_type, 0) + 1
            
            if remaining_patterns:
                print("\n🔍 Remaining error type patterns:")
                for error_type, count in sorted(remaining_patterns.items(), key=lambda x: x[1], reverse=True)[:10]:
                    print(f"     {count}x {error_type}")
        
    except subprocess.TimeoutExpired:
        print("⏰ Compilation check timed out")
    except Exception as e:
        print(f"❌ Error in final check: {e}")

if __name__ == "__main__":
    main()
