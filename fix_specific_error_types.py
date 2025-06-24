#!/usr/bin/env python3

import os
import re
import subprocess
import sys
from pathlib import Path
from typing import List, Dict, Set, Tuple

def extract_error_patterns():
    """Extract specific Error type patterns from compilation output"""
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True)
        
        patterns = {}
        current_file = None
        current_line = None
        
        lines = result.stderr.split('\n')
        for i, line in enumerate(lines):
            # Match file path pattern
            if '-->' in line and '.rs:' in line:
                match = re.search(r'-->\s*([^:]+\.rs):(\d+):(\d+)', line)
                if match:
                    current_file = match.group(1)
                    current_line = int(match.group(2))
            
            # Match Error type issues
            if 'cannot find type' in line and 'Error' in line and 'in this scope' in line:
                error_match = re.search(r'cannot find type `([^`]+)` in this scope', line)
                if error_match and current_file:
                    error_type = error_match.group(1)
                    if current_file not in patterns:
                        patterns[current_file] = []
                    patterns[current_file].append({
                        'type': error_type,
                        'line': current_line,
                        'full_error': line.strip()
                    })
        
        return patterns
    except Exception as e:
        print(f"Error extracting patterns: {e}")
        return {}

def determine_import_for_error_type(error_type: str, file_path: str) -> str:
    """Determine the correct import for a specific Error type"""
    
    # Map of error types to their imports
    error_type_imports = {
        'Error': 'use crate::error::Error;',
        'CursedError': 'use crate::error::CursedError;',
        'ParseError': 'use crate::error::ParseError;',
        'RuntimeError': 'use crate::error::RuntimeError;',
        'CompilationError': 'use crate::error::CompilationError;',
        'ValidationError': 'use crate::error::ValidationError;',
        'TypeError': 'use crate::error::TypeError;',
        'SyntaxError': 'use crate::error::SyntaxError;',
        'MemoryError': 'use crate::error::MemoryError;',
        'IoError': 'use crate::error::IoError;',
        'ConfigError': 'use crate::error::ConfigError;',
        'NetworkError': 'use crate::error::NetworkError;',
        'DatabaseError': 'use crate::error::DatabaseError;',
        'SerializationError': 'use crate::error::SerializationError;',
        'DeserializationError': 'use crate::error::DeserializationError;',
        'FileSystemError': 'use crate::error::FileSystemError;',
        'ProcessError': 'use crate::error::ProcessError;',
        'SecurityError': 'use crate::error::SecurityError;',
        'EncryptionError': 'use crate::error::EncryptionError;',
        'DecryptionError': 'use crate::error::DecryptionError;',
        'AuthenticationError': 'use crate::error::AuthenticationError;',
        'AuthorizationError': 'use crate::error::AuthorizationError;',
        'TimeoutError': 'use crate::error::TimeoutError;',
        'ChannelError': 'use crate::error::ChannelError;',
        'LexerError': 'use crate::error::LexerError;',
        'ParserError': 'use crate::error::ParserError;',
        'OptimizationError': 'use crate::error::OptimizationError;',
        'GcError': 'use crate::error::GcError;',
        'AllocationError': 'use crate::error::AllocationError;',
        'ThreadError': 'use crate::error::ThreadError;',
        'MutexError': 'use crate::error::MutexError;',
        'WebServerError': 'use crate::stdlib::glowup_http::WebServerError;',
        'HttpError': 'use crate::stdlib::glowup_http::HttpError;',
        'RequestError': 'use crate::stdlib::glowup_http::RequestError;',
        'ResponseError': 'use crate::stdlib::glowup_http::ResponseError;',
        'TemplateError': 'use crate::stdlib::template::TemplateError;',
        'CryptoError': 'use crate::stdlib::crypto::CryptoError;',
        'HashError': 'use crate::stdlib::crypto::HashError;',
        'EncodingError': 'use crate::stdlib::encoding::EncodingError;',
        'CollectionError': 'use crate::stdlib::collections::CollectionError;',
        'StringError': 'use crate::stdlib::string::StringError;',
        'MathError': 'use crate::stdlib::math::MathError;',
        'ErrorManager': 'use crate::stdlib::errors::ErrorManager;',
        'ErrorSeverity': 'use crate::stdlib::errors::ErrorSeverity;',
        'ErrorContext': 'use crate::stdlib::errors::ErrorContext;',
        'ErrorHandler': 'use crate::stdlib::errors::ErrorHandler;',
    }
    
    # Check specific file patterns for special cases
    if '/stdlib/errors.rs' in file_path:
        # In the errors module itself, use local types
        if error_type in ['ErrorManager', 'ErrorSeverity', 'ErrorContext', 'ErrorHandler']:
            return None  # These should be defined locally
    
    if '/stdlib/crypto/' in file_path and error_type in ['CryptoError', 'HashError', 'EncryptionError', 'DecryptionError']:
        return f'use super::{error_type};'
    
    if '/stdlib/glowup_http/' in file_path and error_type in ['WebServerError', 'HttpError', 'RequestError', 'ResponseError']:
        return f'use super::{error_type};'
    
    if '/stdlib/template/' in file_path and error_type == 'TemplateError':
        return f'use super::{error_type};'
    
    # Return the mapped import or default to Error
    return error_type_imports.get(error_type, 'use crate::error::Error;')

def fix_file_errors(file_path: str, error_patterns: List[Dict]) -> bool:
    """Fix Error type issues in a specific file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Collect all unique imports needed
        imports_needed = set()
        
        for pattern in error_patterns:
            error_type = pattern['type']
            import_stmt = determine_import_for_error_type(error_type, file_path)
            if import_stmt:
                imports_needed.add(import_stmt)
        
        if not imports_needed:
            return False
        
        # Check which imports are already present
        existing_imports = set()
        for line in content.split('\n'):
            if line.strip().startswith('use ') and any(error_type in line for error_type in ['Error', 'error']):
                existing_imports.add(line.strip())
        
        # Filter out imports that are already present
        new_imports = []
        for import_stmt in imports_needed:
            # Check if this import or a similar one already exists
            import_exists = False
            for existing in existing_imports:
                if import_stmt in existing or existing in import_stmt:
                    import_exists = True
                    break
            
            if not import_exists:
                new_imports.append(import_stmt)
        
        if not new_imports:
            return False
        
        # Find insertion point for imports
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
        
        # Insert the new imports
        for import_stmt in sorted(new_imports):
            lines.insert(insert_index, import_stmt)
            insert_index += 1
        
        # Write back to file
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        
        return True
        
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def main():
    print("🔍 Extracting specific Error type patterns...")
    
    # Extract error patterns from compilation
    error_patterns = extract_error_patterns()
    
    if not error_patterns:
        print("🎉 No Error type issues found!")
        return
    
    print(f"Found Error type issues in {len(error_patterns)} files:")
    
    total_fixed = 0
    total_patterns = 0
    
    for file_path, patterns in error_patterns.items():
        total_patterns += len(patterns)
        print(f"\n📁 {file_path}:")
        
        for pattern in patterns:
            print(f"   Line {pattern['line']}: {pattern['type']}")
        
        # Fix the file
        if fix_file_errors(file_path, patterns):
            print(f"   ✓ Fixed {len(patterns)} error types")
            total_fixed += 1
        else:
            print(f"   - Skipped (no changes needed)")
    
    print(f"\n📊 Summary:")
    print(f"   Files with errors: {len(error_patterns)}")
    print(f"   Total error patterns: {total_patterns}")
    print(f"   Files fixed: {total_fixed}")
    
    # Check compilation status
    print(f"\n🧪 Checking compilation status...")
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True, timeout=60)
        remaining_errors = result.stderr.count("cannot find type") + result.stderr.count("Error") 
        error_in_scope = result.stderr.count("in this scope")
        
        print(f"   Remaining 'cannot find type' errors: {remaining_errors}")
        print(f"   Remaining 'in this scope' errors: {error_in_scope}")
        
        # Count total compilation errors
        total_errors = result.stderr.count("error:")
        print(f"   Total compilation errors: {total_errors}")
        
    except subprocess.TimeoutExpired:
        print("⏰ Compilation check timed out")
    except Exception as e:
        print(f"❌ Error checking compilation: {e}")

if __name__ == "__main__":
    main()
