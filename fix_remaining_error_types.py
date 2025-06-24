#!/usr/bin/env python3

import os
import re
import subprocess
import sys
import time
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor, as_completed
from typing import List, Dict, Set, Tuple

def get_compilation_errors():
    """Get all compilation errors related to Error type resolution"""
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True, cwd='.')
        if result.returncode == 0:
            print("No compilation errors found!")
            return []
        
        errors = []
        current_file = None
        
        for line in result.stderr.split('\n'):
            # Extract file paths from error messages
            if '-->' in line and '.rs:' in line:
                match = re.search(r'-->\s*([^:]+\.rs):(\d+):(\d+)', line)
                if match:
                    current_file = match.group(1)
            
            # Look for Error type resolution issues
            if current_file and any(error_pattern in line for error_pattern in [
                "cannot find type `Error` in this scope",
                "type `Error` is not in scope", 
                "cannot find name `Error`",
                "unresolved type `Error`",
                "Error` is not defined",
                "use of undeclared type `Error`",
                "cannot find type `Error`"
            ]):
                errors.append(current_file)
        
        return list(set(errors))  # Remove duplicates
    except Exception as e:
        print(f"Error getting compilation errors: {e}")
        return []

def analyze_error_usage(file_path: str) -> Dict[str, List[str]]:
    """Analyze how Error types are used in a file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        analysis = {
            'unqualified_error': [],
            'result_error': [],
            'anyhow_error': [],
            'std_error': [],
            'custom_error': [],
            'existing_imports': []
        }
        
        lines = content.split('\n')
        for i, line in enumerate(lines, 1):
            line_stripped = line.strip()
            
            # Check existing imports
            if line_stripped.startswith('use ') and 'error' in line_stripped.lower():
                analysis['existing_imports'].append(f"Line {i}: {line_stripped}")
            
            # Check various Error usage patterns
            if re.search(r'\bError\b', line) and not line_stripped.startswith('//'):
                if 'Result<' in line and 'Error>' in line:
                    analysis['result_error'].append(f"Line {i}: {line_stripped}")
                elif 'anyhow::Error' in line:
                    analysis['anyhow_error'].append(f"Line {i}: {line_stripped}")
                elif 'std::error::Error' in line:
                    analysis['std_error'].append(f"Line {i}: {line_stripped}")
                elif re.search(r'\b[A-Z]\w*Error\b', line):
                    analysis['custom_error'].append(f"Line {i}: {line_stripped}")
                elif re.search(r'\bError\b', line):
                    analysis['unqualified_error'].append(f"Line {i}: {line_stripped}")
        
        return analysis
    except Exception as e:
        print(f"Error analyzing {file_path}: {e}")
        return {}

def determine_error_import(file_path: str, analysis: Dict[str, List[str]]) -> str:
    """Determine the appropriate Error import for a file"""
    
    # Check if file is in a specific module that might have its own Error type
    if '/error/' in file_path:
        return 'use crate::error::CursedError as Error;'
    elif '/ast/' in file_path:
        return 'use crate::error::Error;'
    elif '/parser/' in file_path:
        return 'use crate::error::Error;'
    elif '/codegen/' in file_path:
        return 'use crate::error::Error;'
    elif '/runtime/' in file_path:
        return 'use crate::error::Error;'
    elif '/stdlib/' in file_path:
        return 'use crate::error::Error;'
    elif '/memory/' in file_path:
        return 'use crate::error::Error;'
    elif '/optimization/' in file_path:
        return 'use crate::error::Error;'
    elif 'tests/' in file_path:
        return 'use crate::error::Error;'
    
    # Check for specific error patterns
    if analysis.get('anyhow_error'):
        return 'use anyhow::Error;'
    elif analysis.get('std_error'):
        return 'use std::error::Error;'
    elif analysis.get('result_error') or analysis.get('unqualified_error'):
        return 'use crate::error::Error;'
    
    # Default to crate error
    return 'use crate::error::Error;'

def fix_error_imports(file_path: str) -> bool:
    """Fix Error type imports in a single file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Skip if file already has proper Error imports
        if any(import_pattern in content for import_pattern in [
            'use crate::error::Error',
            'use anyhow::Error',
            'use std::error::Error',
            'use crate::error::CursedError'
        ]):
            return False
        
        analysis = analyze_error_usage(file_path)
        
        # Skip if no Error usage found
        if not any(analysis.get(key, []) for key in ['unqualified_error', 'result_error', 'custom_error']):
            return False
        
        # Determine appropriate import
        error_import = determine_error_import(file_path, analysis)
        
        # Find insertion point for import
        lines = content.split('\n')
        insert_index = 0
        
        # Find the last use statement or beginning of file
        for i, line in enumerate(lines):
            if line.strip().startswith('use '):
                insert_index = i + 1
            elif line.strip().startswith('mod ') or line.strip().startswith('pub mod '):
                insert_index = i + 1
            elif line.strip() and not line.strip().startswith('//') and not line.strip().startswith('#'):
                break
        
        # Insert the import
        lines.insert(insert_index, error_import)
        
        # Write back to file
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        
        return True
        
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def find_all_rust_files() -> List[str]:
    """Find all Rust files in src and tests directories"""
    rust_files = []
    
    for directory in ['src', 'tests']:
        if os.path.exists(directory):
            for root, dirs, files in os.walk(directory):
                for file in files:
                    if file.endswith('.rs'):
                        rust_files.append(os.path.join(root, file))
    
    return rust_files

def fix_file_batch(files: List[str]) -> Dict[str, bool]:
    """Fix a batch of files in parallel"""
    results = {}
    
    with ThreadPoolExecutor(max_workers=8) as executor:
        future_to_file = {executor.submit(fix_error_imports, file): file for file in files}
        
        for future in as_completed(future_to_file):
            file = future_to_file[future]
            try:
                fixed = future.result()
                results[file] = fixed
                if fixed:
                    print(f"✓ Fixed: {file}")
                else:
                    print(f"- Skipped: {file}")
            except Exception as e:
                print(f"✗ Error fixing {file}: {e}")
                results[file] = False
    
    return results

def get_specific_error_patterns() -> List[str]:
    """Get files with specific Error patterns from compilation"""
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True)
        error_files = set()
        
        for line in result.stderr.split('\n'):
            if 'cannot find type `Error` in this scope' in line:
                # Extract file path from previous context
                prev_lines = result.stderr.split('\n')
                for i, prev_line in enumerate(prev_lines):
                    if line in prev_lines[i:i+5]:  # Look in nearby lines
                        for j in range(max(0, i-5), min(len(prev_lines), i+5)):
                            if '-->' in prev_lines[j] and '.rs:' in prev_lines[j]:
                                match = re.search(r'-->\s*([^:]+\.rs)', prev_lines[j])
                                if match:
                                    error_files.add(match.group(1))
                                    break
                        break
        
        return list(error_files)
    except Exception as e:
        print(f"Error getting specific patterns: {e}")
        return []

def main():
    print("🔍 Analyzing Error type resolution issues...")
    
    # Get files with compilation errors
    error_files = get_compilation_errors()
    print(f"Found {len(error_files)} files with Error type issues from compilation")
    
    # Get additional files that might need fixing
    all_rust_files = find_all_rust_files()
    print(f"Found {len(all_rust_files)} total Rust files")
    
    # Get specific error patterns
    specific_files = get_specific_error_patterns()
    print(f"Found {len(specific_files)} files with specific Error patterns")
    
    # Combine all files that might need fixing
    files_to_check = list(set(error_files + specific_files))
    
    if not files_to_check:
        print("🎉 No Error type issues found!")
        return
    
    print(f"\n🔧 Processing {len(files_to_check)} files...")
    
    # Process files in batches
    batch_size = 20
    total_fixed = 0
    total_skipped = 0
    total_errors = 0
    
    for i in range(0, len(files_to_check), batch_size):
        batch = files_to_check[i:i+batch_size]
        print(f"\nProcessing batch {i//batch_size + 1}/{(len(files_to_check) + batch_size - 1)//batch_size}...")
        
        results = fix_file_batch(batch)
        
        batch_fixed = sum(1 for fixed in results.values() if fixed)
        batch_skipped = sum(1 for fixed in results.values() if not fixed)
        
        total_fixed += batch_fixed
        total_skipped += batch_skipped
        
        print(f"Batch results: {batch_fixed} fixed, {batch_skipped} skipped")
    
    print(f"\n📊 Summary:")
    print(f"   Files processed: {len(files_to_check)}")
    print(f"   Files fixed: {total_fixed}")
    print(f"   Files skipped: {total_skipped}")
    print(f"   Files with errors: {total_errors}")
    
    # Check if fixes resolved issues
    print(f"\n🧪 Checking compilation status...")
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True, timeout=60)
        remaining_errors = result.stderr.count("cannot find type `Error` in this scope")
        
        if remaining_errors == 0:
            print("🎉 All Error type issues resolved!")
        else:
            print(f"⚠️  {remaining_errors} Error type issues remaining")
            
        # Count total compilation errors
        total_errors = result.stderr.count("error:")
        print(f"   Total compilation errors: {total_errors}")
        
    except subprocess.TimeoutExpired:
        print("⏰ Compilation check timed out")
    except Exception as e:
        print(f"❌ Error checking compilation: {e}")

if __name__ == "__main__":
    main()
