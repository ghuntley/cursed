#!/usr/bin/env python3
"""
Fix E0412 missing type errors in process management modules.
This script identifies missing type imports and adds them.
"""
import os
import re
import subprocess
from pathlib import Path

def find_process_files():
    """Find all process-related files"""
    process_files = []
    
    # Find all process/IPC related files
    for root, dirs, files in os.walk("src"):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                if any(keyword in file_path.lower() for keyword in ['process', 'ipc', 'exec_']):
                    process_files.append(file_path)
    
    return process_files

def check_compilation_errors():
    """Get compilation errors from cargo check"""
    try:
        # Use linking fix for Nix environment
        env = os.environ.copy()
        env['LIBRARY_PATH'] = "/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib:/nix/store/l5g2v1jgfyf3j0jp9iv5b79fi8yrwzpp-zlib-1.3.1/lib:/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib:/nix/store/hd6llsw2dkiazk9d2ywv13cc6alhflly-libxml2-2.13.5/lib"
        env['RUSTFLAGS'] = "-C linker=gcc -C link-arg=-fuse-ld=bfd"
        
        result = subprocess.run(['cargo', 'check', '--message-format=short'], 
                              capture_output=True, text=True, env=env, timeout=60)
        return result.stderr
    except subprocess.TimeoutExpired:
        print("Cargo check timed out, checking recent errors...")
        return ""
    except Exception as e:
        print(f"Error running cargo check: {e}")
        return ""

def analyze_type_definitions():
    """Analyze where process types are defined"""
    type_definitions = {}
    
    # Common process types to track
    types_to_find = [
        'ProcessHandle', 'ProcessInfo', 'ProcessState', 'ProcessResult', 'ProcessError',
        'IpcMessage', 'IpcChannel', 'IpcResult', 'IpcError', 'IpcHandle',
        'ProcessManager', 'ProcessController', 'ProcessMonitor',
        'SlayProcess', 'VibezProcess', 'EnhancedProcessInfo'
    ]
    
    for type_name in types_to_find:
        # Search for type definitions
        try:
            result = subprocess.run(['grep', '-r', f'struct {type_name}\\|enum {type_name}\\|type {type_name}', 
                                   'src/', '--include=*.rs'], 
                                  capture_output=True, text=True)
            if result.stdout:
                for line in result.stdout.strip().split('\n'):
                    if ':' in line:
                        file_path = line.split(':')[0]
                        if type_name not in type_definitions:
                            type_definitions[type_name] = []
                        type_definitions[type_name].append(file_path)
        except:
            continue
    
    return type_definitions

def fix_missing_imports(file_path):
    """Fix missing imports in a specific file"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Common import fixes for process files
        imports_to_add = []
        
        # Check if file uses types that need imports
        if 'ProcessHandle' in content and 'use crate::stdlib::process::core::ProcessHandle' not in content:
            imports_to_add.append('use crate::stdlib::process::core::ProcessHandle;')
        
        if 'ProcessInfo' in content and 'use crate::stdlib::process::info::ProcessInfo' not in content:
            imports_to_add.append('use crate::stdlib::process::info::ProcessInfo;')
        
        if 'ProcessState' in content and 'use crate::stdlib::process::info::ProcessState' not in content:
            imports_to_add.append('use crate::stdlib::process::info::ProcessState;')
        
        if 'ProcessResult' in content and 'use crate::stdlib::process::error::ProcessResult' not in content:
            imports_to_add.append('use crate::stdlib::process::error::ProcessResult;')
        
        if 'ProcessError' in content and 'use crate::stdlib::process::error::ProcessError' not in content:
            imports_to_add.append('use crate::stdlib::process::error::ProcessError;')
        
        if 'IpcMessage' in content and 'use crate::stdlib::process::real_ipc::IpcMessage' not in content:
            imports_to_add.append('use crate::stdlib::process::real_ipc::IpcMessage;')
        
        if 'IpcChannel' in content and 'use crate::stdlib::process::real_ipc::IpcChannel' not in content:
            imports_to_add.append('use crate::stdlib::process::real_ipc::IpcChannel;')
        
        if 'EnhancedProcessInfo' in content and 'use crate::stdlib::process::enhanced_control::EnhancedProcessInfo' not in content:
            imports_to_add.append('use crate::stdlib::process::enhanced_control::EnhancedProcessInfo;')
        
        # Add imports if needed
        if imports_to_add:
            # Find where to insert imports
            lines = content.split('\n')
            insert_pos = 0
            
            # Find last import line
            for i, line in enumerate(lines):
                if line.strip().startswith('use ') or line.strip().startswith('extern crate'):
                    insert_pos = i + 1
            
            # Insert new imports
            for import_line in imports_to_add:
                if import_line not in content:
                    lines.insert(insert_pos, import_line)
                    insert_pos += 1
            
            content = '\n'.join(lines)
        
        # Write back if changed
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed imports in {file_path}")
            return True
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
    
    return False

def fix_circular_dependencies():
    """Fix circular dependency issues"""
    
    # Common circular dependency fixes
    fixes = [
        # Remove problematic re-exports that cause cycles
        {
            'file': 'src/stdlib/process/mod.rs',
            'remove_lines': [
                'pub use comprehensive_integration::*;',
                'pub use enhanced_exec_slay_complete::*;',
                'pub use enhanced_exec_vibez_complete::*;'
            ]
        }
    ]
    
    for fix in fixes:
        try:
            with open(fix['file'], 'r') as f:
                content = f.read()
            
            original_content = content
            
            for line_to_remove in fix['remove_lines']:
                if line_to_remove in content:
                    content = content.replace(line_to_remove, f'// {line_to_remove} // Removed to avoid circular dependency')
            
            if content != original_content:
                with open(fix['file'], 'w') as f:
                    f.write(content)
                print(f"Fixed circular dependencies in {fix['file']}")
                
        except Exception as e:
            print(f"Error fixing circular dependencies in {fix['file']}: {e}")

def main():
    print("Fixing E0412 missing type errors in process management modules...")
    
    # Find process files
    process_files = find_process_files()
    print(f"Found {len(process_files)} process-related files")
    
    # Analyze type definitions
    print("Analyzing type definitions...")
    type_definitions = analyze_type_definitions()
    for type_name, files in type_definitions.items():
        print(f"  {type_name}: found in {len(files)} files")
    
    # Fix missing imports
    print("Fixing missing imports...")
    fixed_count = 0
    for file_path in process_files:
        if fix_missing_imports(file_path):
            fixed_count += 1
    
    print(f"Fixed imports in {fixed_count} files")
    
    # Fix circular dependencies
    print("Fixing circular dependencies...")
    fix_circular_dependencies()
    
    print("Process E0412 fixes completed!")

if __name__ == "__main__":
    main()
