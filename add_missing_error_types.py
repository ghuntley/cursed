#!/usr/bin/env python3

import os
import re
import subprocess
from pathlib import Path
from typing import List, Dict, Set

def get_files_needing_error_types():
    """Get files that need Error types defined or imported"""
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True)
        
        files_with_errors = {}
        
        lines = result.stderr.split('\n')
        for i, line in enumerate(lines):
            if 'cannot find type `Error` in this scope' in line:
                # Look for file path in previous lines
                for j in range(max(0, i-5), i):
                    if '-->' in lines[j] and '.rs:' in lines[j]:
                        match = re.search(r'-->\s*([^:]+\.rs):(\d+):', lines[j])
                        if match:
                            file_path = match.group(1)
                            line_num = int(match.group(2))
                            
                            if file_path not in files_with_errors:
                                files_with_errors[file_path] = []
                            files_with_errors[file_path].append(line_num)
                            break
        
        return files_with_errors
    except Exception as e:
        print(f"Error getting files: {e}")
        return {}

def should_add_local_error_type(file_path: str) -> bool:
    """Determine if a file should have its own Error type defined"""
    
    # Modules that should have their own Error types
    local_error_modules = [
        'crypto/',
        'collections/',
        'fs/',
        'glowup_http/',
        'template/',
        'json_tea/',
        'vibe_net/',
        'oglogging/',
        'signal_boost/',
        'embed_that/',
        'database/',
        'process/',
        'async_runtime/',
        'memory/',
        'optimization/'
    ]
    
    for module in local_error_modules:
        if module in file_path:
            return True
    
    return False

def get_module_name_from_path(file_path: str) -> str:
    """Extract module name from file path"""
    parts = file_path.replace('src/stdlib/', '').split('/')
    if len(parts) > 1:
        return parts[0]
    return parts[0].replace('.rs', '')

def add_error_type_to_module(file_path: str) -> bool:
    """Add an Error type definition to a module"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if Error type already exists
        if 'pub enum Error' in content or 'type Error' in content:
            return False
        
        module_name = get_module_name_from_path(file_path)
        
        # Define module-specific error types
        error_definitions = {
            'crypto': '''
// Crypto module error types
pub type Error = CryptoError;

#[derive(Debug, Clone)]
pub enum CryptoError {
    InvalidKey,
    InvalidInput,
    EncryptionFailed,
    DecryptionFailed,
    HashingFailed,
    SignatureFailed,
    VerificationFailed,
    KeyGenerationFailed,
    InvalidFormat,
    UnsupportedAlgorithm,
    InsufficientEntropy,
    InvalidParameters,
    OperationFailed(String),
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidKey => write!(f, "Invalid cryptographic key"),
            CryptoError::InvalidInput => write!(f, "Invalid input data"),
            CryptoError::EncryptionFailed => write!(f, "Encryption operation failed"),
            CryptoError::DecryptionFailed => write!(f, "Decryption operation failed"),
            CryptoError::HashingFailed => write!(f, "Hashing operation failed"),
            CryptoError::SignatureFailed => write!(f, "Digital signature failed"),
            CryptoError::VerificationFailed => write!(f, "Signature verification failed"),
            CryptoError::KeyGenerationFailed => write!(f, "Key generation failed"),
            CryptoError::InvalidFormat => write!(f, "Invalid data format"),
            CryptoError::UnsupportedAlgorithm => write!(f, "Unsupported algorithm"),
            CryptoError::InsufficientEntropy => write!(f, "Insufficient entropy"),
            CryptoError::InvalidParameters => write!(f, "Invalid parameters"),
            CryptoError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
        }
    }
}

impl std::error::Error for CryptoError {}
''',
            'collections': '''
// Collections module error types
pub type Error = CollectionError;

#[derive(Debug, Clone)]
pub enum CollectionError {
    IndexOutOfBounds,
    KeyNotFound,
    DuplicateKey,
    InvalidOperation,
    CapacityExceeded,
    Empty,
    InvalidIndex,
    OperationFailed(String),
}

impl std::fmt::Display for CollectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollectionError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            CollectionError::KeyNotFound => write!(f, "Key not found"),
            CollectionError::DuplicateKey => write!(f, "Duplicate key"),
            CollectionError::InvalidOperation => write!(f, "Invalid operation"),
            CollectionError::CapacityExceeded => write!(f, "Capacity exceeded"),
            CollectionError::Empty => write!(f, "Collection is empty"),
            CollectionError::InvalidIndex => write!(f, "Invalid index"),
            CollectionError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
        }
    }
}

impl std::error::Error for CollectionError {}
''',
            'fs': '''
// File system module error types
pub type Error = FileSystemError;

#[derive(Debug, Clone)]
pub enum FileSystemError {
    FileNotFound,
    PermissionDenied,
    AlreadyExists,
    NotADirectory,
    NotAFile,
    InvalidPath,
    IoError(String),
    WatcherError(String),
    OperationFailed(String),
}

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSystemError::FileNotFound => write!(f, "File not found"),
            FileSystemError::PermissionDenied => write!(f, "Permission denied"),
            FileSystemError::AlreadyExists => write!(f, "File already exists"),
            FileSystemError::NotADirectory => write!(f, "Not a directory"),
            FileSystemError::NotAFile => write!(f, "Not a file"),
            FileSystemError::InvalidPath => write!(f, "Invalid path"),
            FileSystemError::IoError(msg) => write!(f, "I/O error: {}", msg),
            FileSystemError::WatcherError(msg) => write!(f, "Watcher error: {}", msg),
            FileSystemError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
        }
    }
}

impl std::error::Error for FileSystemError {}
''',
            'default': '''
// Module error types
pub type Error = ModuleError;

#[derive(Debug, Clone)]
pub enum ModuleError {
    InvalidInput,
    InvalidOperation,
    OperationFailed(String),
    ConfigurationError(String),
    RuntimeError(String),
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleError::InvalidInput => write!(f, "Invalid input"),
            ModuleError::InvalidOperation => write!(f, "Invalid operation"),
            ModuleError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
            ModuleError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            ModuleError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for ModuleError {}
'''
        }
        
        # Get the appropriate error definition
        error_def = error_definitions.get(module_name, error_definitions['default'])
        
        # Find a good place to insert the error definition
        lines = content.split('\n')
        insert_index = 0
        
        # Look for the end of imports
        for i, line in enumerate(lines):
            if line.strip().startswith('use ') or line.strip().startswith('mod '):
                insert_index = i + 1
            elif line.strip() and not line.strip().startswith('//') and not line.strip().startswith('#'):
                break
        
        # Add some spacing
        if insert_index > 0:
            lines.insert(insert_index, '')
            insert_index += 1
        
        # Insert the error definition
        for line in error_def.strip().split('\n'):
            lines.insert(insert_index, line)
            insert_index += 1
        
        # Write back to file
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        
        return True
        
    except Exception as e:
        print(f"Error adding error type to {file_path}: {e}")
        return False

def add_error_import_to_file(file_path: str) -> bool:
    """Add Error import to a file that doesn't need its own Error type"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if Error import already exists
        if 'use crate::error::Error' in content or 'use super::Error' in content:
            return False
        
        # Determine appropriate import
        if '/stdlib/' in file_path:
            # Check if parent module might have Error type
            module_dir = os.path.dirname(file_path)
            mod_rs_path = os.path.join(module_dir, 'mod.rs')
            
            if os.path.exists(mod_rs_path):
                import_stmt = 'use super::Error;'
            else:
                import_stmt = 'use crate::error::Error;'
        else:
            import_stmt = 'use crate::error::Error;'
        
        # Find insertion point
        lines = content.split('\n')
        insert_index = 0
        
        for i, line in enumerate(lines):
            if line.strip().startswith('use '):
                insert_index = i + 1
            elif line.strip() and not line.strip().startswith('//') and not line.strip().startswith('#'):
                break
        
        # Insert the import
        lines.insert(insert_index, import_stmt)
        
        # Write back to file
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        
        return True
        
    except Exception as e:
        print(f"Error adding import to {file_path}: {e}")
        return False

def main():
    print("🔍 Finding files that need Error types...")
    
    files_with_errors = get_files_needing_error_types()
    
    if not files_with_errors:
        print("🎉 No files need Error types!")
        return
    
    print(f"Found {len(files_with_errors)} files needing Error types")
    
    # Group files by module
    modules_needing_error_types = {}
    files_needing_imports = []
    
    for file_path in files_with_errors.keys():
        if should_add_local_error_type(file_path):
            module_name = get_module_name_from_path(file_path)
            if module_name not in modules_needing_error_types:
                modules_needing_error_types[module_name] = []
            modules_needing_error_types[module_name].append(file_path)
        else:
            files_needing_imports.append(file_path)
    
    print(f"\nModules needing local Error types: {len(modules_needing_error_types)}")
    print(f"Files needing Error imports: {len(files_needing_imports)}")
    
    total_fixed = 0
    
    # Add Error types to module mod.rs files
    for module_name, files in modules_needing_error_types.items():
        print(f"\n📁 Module: {module_name}")
        
        # Find the mod.rs file for this module
        mod_rs_candidates = [
            f'src/stdlib/{module_name}/mod.rs',
            f'src/stdlib/{module_name}.rs'
        ]
        
        mod_file = None
        for candidate in mod_rs_candidates:
            if os.path.exists(candidate):
                mod_file = candidate
                break
        
        if mod_file:
            if add_error_type_to_module(mod_file):
                print(f"   ✓ Added Error type to {mod_file}")
                total_fixed += 1
            else:
                print(f"   - Error type already exists in {mod_file}")
        else:
            print(f"   ⚠️  Could not find mod.rs for {module_name}")
    
    # Add imports to individual files
    print(f"\n📄 Adding imports to individual files...")
    for file_path in files_needing_imports:
        if add_error_import_to_file(file_path):
            print(f"   ✓ Added import to {file_path}")
            total_fixed += 1
        else:
            print(f"   - Import already exists in {file_path}")
    
    print(f"\n📊 Summary:")
    print(f"   Total files processed: {len(files_with_errors)}")
    print(f"   Files fixed: {total_fixed}")
    
    # Check compilation status
    print(f"\n🧪 Checking compilation status...")
    try:
        result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], capture_output=True, text=True, timeout=60)
        remaining_errors = result.stderr.count("cannot find type `Error` in this scope")
        
        if remaining_errors == 0:
            print("🎉 All Error type issues resolved!")
        else:
            print(f"⚠️  {remaining_errors} Error type issues remaining")
        
        total_errors = result.stderr.count("error:")
        print(f"   Total compilation errors: {total_errors}")
        
    except subprocess.TimeoutExpired:
        print("⏰ Compilation check timed out")
    except Exception as e:
        print(f"❌ Error checking compilation: {e}")

if __name__ == "__main__":
    main()
