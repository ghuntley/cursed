#!/usr/bin/env python3
"""
Fix the remaining E0659 conflicts after the initial fixes.
"""

import os
import re
import subprocess
from pathlib import Path

def run_cargo_check():
    """Run cargo check with linking fix and return output"""
    try:
        result = subprocess.run(
            ['./fix_linking.sh', 'cargo', 'check'],
            capture_output=True,
            text=True,
            timeout=300
        )
        return result.stderr
    except subprocess.TimeoutExpired:
        return "Timeout occurred"
    except Exception as e:
        return f"Error running cargo check: {e}"

def fix_unified_process_ipc():
    """Fix src/stdlib/process/unified_process_ipc.rs"""
    file_path = Path('src/stdlib/process/unified_process_ipc.rs')
    if not file_path.exists():
        print(f"File not found: {file_path}")
        return
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Find the problematic import and replace with qualified imports
    old_import_pattern = r'use crate::stdlib::process::\{[^}]+\};'
    
    new_imports = '''use crate::stdlib::process::error::{ProcessError, ProcessResult};
use crate::stdlib::process::core::{ProcessConfig, ProcessManager};
use crate::stdlib::process::enhanced_control::{EnhancedProcess as StdEnhancedProcess};
use crate::stdlib::process::info::{ProcessState as StdProcessState};
use crate::runtime::process::{ProcessGroup as RuntimeProcessGroup};
use crate::stdlib::process::exec_vibez::{VibezResult, ExecutionContext, EnhancedCmd};

// Type aliases to resolve conflicts
type EnhancedProcess = StdEnhancedProcess;
type ProcessState = StdProcessState;
type ProcessGroup = RuntimeProcessGroup;'''
    
    # Replace the import
    content = re.sub(old_import_pattern, new_imports, content)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed unified_process_ipc imports in {file_path}")

def fix_unix_platform():
    """Fix src/stdlib/process/unix_platform.rs"""
    file_path = Path('src/stdlib/process/unix_platform.rs')
    if not file_path.exists():
        print(f"File not found: {file_path}")
        return
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Find and replace the problematic import
    old_import_pattern = r'use crate::stdlib::process::\{[^}]+\};'
    
    new_imports = '''use crate::stdlib::process::error::{ProcessError, ProcessResult};
use crate::stdlib::process::enhanced_control::{EnhancedProcess as StdEnhancedProcess};
use crate::runtime::process::{ResourceLimits as RuntimeResourceLimits, SecurityContext as RuntimeSecurityContext};
use crate::stdlib::process::info::{ProcessState as StdProcessState};

// Type aliases to resolve conflicts
type EnhancedProcess = StdEnhancedProcess;
type ResourceLimits = RuntimeResourceLimits;
type SecurityContext = RuntimeSecurityContext;
type ProcessState = StdProcessState;'''
    
    # Replace the import
    content = re.sub(old_import_pattern, new_imports, content)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed unix_platform imports in {file_path}")

def fix_windows_platform():
    """Fix src/stdlib/process/windows_platform.rs if it exists"""
    file_path = Path('src/stdlib/process/windows_platform.rs')
    if not file_path.exists():
        print(f"File not found: {file_path}")
        return
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Find and replace the problematic import
    old_import_pattern = r'use crate::stdlib::process::\{[^}]+\};'
    
    new_imports = '''use crate::stdlib::process::error::{ProcessError, ProcessResult};
use crate::stdlib::process::enhanced_control::{EnhancedProcess as StdEnhancedProcess};
use crate::runtime::process::{ResourceLimits as RuntimeResourceLimits, SecurityContext as RuntimeSecurityContext};
use crate::stdlib::process::info::{ProcessState as StdProcessState};

// Type aliases to resolve conflicts  
type EnhancedProcess = StdEnhancedProcess;
type ResourceLimits = RuntimeResourceLimits;
type SecurityContext = RuntimeSecurityContext;
type ProcessState = StdProcessState;'''
    
    # Replace the import
    content = re.sub(old_import_pattern, new_imports, content)
    
    with open(file_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed windows_platform imports in {file_path}")

def fix_enhanced_exec_vibez_complete():
    """Fix src/stdlib/process/enhanced_exec_vibez_complete.rs"""
    file_path = Path('src/stdlib/process/enhanced_exec_vibez_complete.rs')
    if not file_path.exists():
        print(f"File not found: {file_path}")
        return
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Check if it has problematic imports and fix them
    if 'use crate::stdlib::process::{' in content:
        old_import_pattern = r'use crate::stdlib::process::\{[^}]+\};'
        
        new_imports = '''use crate::stdlib::process::error::{ProcessError, ProcessResult};
use crate::stdlib::process::core::{ProcessConfig};
use crate::stdlib::process::enhanced_control::{EnhancedProcess as StdEnhancedProcess};
use crate::stdlib::process::info::{ProcessState as StdProcessState};
use crate::runtime::process::{ProcessGroup as RuntimeProcessGroup};

// Type aliases to resolve conflicts
type EnhancedProcess = StdEnhancedProcess;
type ProcessState = StdProcessState;
type ProcessGroup = RuntimeProcessGroup;'''
        
        content = re.sub(old_import_pattern, new_imports, content)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"Fixed enhanced_exec_vibez_complete imports in {file_path}")

def fix_remaining_wildcard_imports():
    """Find and fix any remaining wildcard imports causing conflicts"""
    
    # Process files that might still have conflicts
    process_files = [
        'src/stdlib/process/comprehensive_integration.rs',
        'src/stdlib/process/enhanced_exec_slay_complete.rs',
        'src/stdlib/process/exec_slay_complete.rs',
    ]
    
    for file_path_str in process_files:
        file_path = Path(file_path_str)
        if not file_path.exists():
            continue
        
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Check if it has problematic wildcard imports
        if 'use crate::stdlib::process::{' in content and ('ProcessState' in content or 'EnhancedProcess' in content):
            old_import_pattern = r'use crate::stdlib::process::\{[^}]+\};'
            
            new_imports = '''use crate::stdlib::process::error::{ProcessError, ProcessResult};
use crate::stdlib::process::core::{ProcessConfig, ProcessManager};
use crate::stdlib::process::enhanced_control::{EnhancedProcess as StdEnhancedProcess};
use crate::stdlib::process::info::{ProcessState as StdProcessState};
use crate::runtime::process::{ProcessGroup as RuntimeProcessGroup};

// Type aliases to resolve conflicts
type EnhancedProcess = StdEnhancedProcess;
type ProcessState = StdProcessState;
type ProcessGroup = RuntimeProcessGroup;'''
            
            content = re.sub(old_import_pattern, new_imports, content)
            
            with open(file_path, 'w') as f:
                f.write(content)
            
            print(f"Fixed wildcard imports in {file_path}")

def update_process_mod_further():
    """Further update the process mod.rs to remove remaining conflicts"""
    mod_path = Path('src/stdlib/process/mod.rs')
    if not mod_path.exists():
        return
    
    with open(mod_path, 'r') as f:
        content = f.read()
    
    # Remove any remaining wildcard exports that might cause conflicts
    remaining_wildcards = [
        'pub use enhanced_exec_slay_complete::*;',
        'pub use enhanced_exec_vibez_complete::*;', 
        'pub use exec_slay_complete::*;',
        'pub use comprehensive_integration::*;'
    ]
    
    for wildcard in remaining_wildcards:
        if wildcard in content:
            # Replace with a comment
            content = content.replace(wildcard, f'// {wildcard} // Removed to avoid E0659 conflicts')
    
    with open(mod_path, 'w') as f:
        f.write(content)
    
    print(f"Removed remaining wildcard exports from process mod.rs")

def main():
    print("Fixing remaining E0659 conflicts...")
    
    # Get initial error count
    print("\n1. Running initial cargo check...")
    initial_output = run_cargo_check()
    initial_e0659 = len(re.findall(r'error\[E0659\]', initial_output))
    process_conflicts = len(re.findall(r'error\[E0659\].*(?:Process|Signal)', initial_output))
    
    print(f"Initial E0659 errors: {initial_e0659}")
    print(f"Process-related conflicts: {process_conflicts}")
    
    if process_conflicts == 0:
        print("No process-related E0659 errors found!")
        return
    
    print("\n2. Fixing unified_process_ipc...")
    fix_unified_process_ipc()
    
    print("\n3. Fixing unix_platform...")
    fix_unix_platform()
    
    print("\n4. Fixing windows_platform...")
    fix_windows_platform()
    
    print("\n5. Fixing enhanced_exec_vibez_complete...")
    fix_enhanced_exec_vibez_complete()
    
    print("\n6. Fixing remaining wildcard imports...")
    fix_remaining_wildcard_imports()
    
    print("\n7. Updating process mod further...")
    update_process_mod_further()
    
    print("\n8. Running final cargo check...")
    final_output = run_cargo_check()
    final_e0659 = len(re.findall(r'error\[E0659\]', final_output))
    final_process_conflicts = len(re.findall(r'error\[E0659\].*(?:Process|Signal)', final_output))
    
    print(f"Final E0659 errors: {final_e0659}")
    print(f"Process-related conflicts: {final_process_conflicts}")
    print(f"Reduction: {initial_e0659 - final_e0659} total, {process_conflicts - final_process_conflicts} process-related")
    
    if final_process_conflicts > 0:
        print("\nRemaining process-related E0659 errors:")
        error_lines = [line for line in final_output.split('\n') if 'error[E0659]' in line and ('Process' in line or 'Signal' in line)]
        for i, line in enumerate(error_lines[:3]):  # Show first 3
            print(f"  {i+1}. {line.strip()}")
    
    print("\nRemaining E0659 fixes completed!")

if __name__ == '__main__':
    main()
