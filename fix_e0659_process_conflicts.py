#!/usr/bin/env python3
"""
Fix E0659 ambiguous import conflicts for process management modules.
"""

import os
import re
import subprocess
from pathlib import Path

def run_cargo_check():
    """Run cargo check and return output"""
    try:
        result = subprocess.run(
            ['cargo', 'check'],
            capture_output=True,
            text=True,
            timeout=300
        )
        return result.stderr
    except subprocess.TimeoutExpired:
        return "Timeout occurred"
    except Exception as e:
        return f"Error running cargo check: {e}"

def extract_e0659_errors(output):
    """Extract E0659 errors from cargo output"""
    errors = []
    lines = output.split('\n')
    
    for i, line in enumerate(lines):
        if 'error[E0659]:' in line and ('Process' in line or 'Signal' in line or 'ProcessInfo' in line or 'ProcessStatus' in line):
            # Get context around the error
            context_start = max(0, i - 3)
            context_end = min(len(lines), i + 10)
            context = '\n'.join(lines[context_start:context_end])
            errors.append({
                'line': line,
                'context': context,
                'line_num': i
            })
    
    return errors

def fix_exec_slay_mod():
    """Fix the exec_slay mod.rs to use explicit imports"""
    mod_path = Path('src/stdlib/exec_slay/mod.rs')
    if not mod_path.exists():
        print(f"File not found: {mod_path}")
        return
    
    with open(mod_path, 'r') as f:
        content = f.read()
    
    # Replace wildcard re-exports with explicit imports
    replacements = [
        # Replace wildcard re-exports with explicit ones
        ('pub use command::*;', '''pub use command::{
    SlayCommand, CommandResult, CommandOptions
};'''),
        ('pub use process::*;', '''pub use process::{
    SlayProcess, SlayProcessState
};'''),
        ('pub use pipeline::*;', '''pub use pipeline::{
    SlayPipeline, PipelineResult
};'''),
        ('pub use task::*;', '''pub use task::{
    SlayTask, TaskResult, TaskStatus
};'''),
        ('pub use builder::*;', '''pub use builder::{
    SlayCommandBuilder
};'''),
        ('pub use shell::*;', '''pub use shell::{
    SlayShell, ShellOptions
};'''),
        ('pub use monitor::*;', '''pub use monitor::{
    SlayMonitor, MonitorOptions
};'''),
        ('pub use enhanced_command::*;', '''pub use enhanced_command::{
    EnhancedSlayCommand, EnhancedCommandOptions
};'''),
        ('pub use timeout::*;', '''pub use timeout::{
    with_timeout, timeout_command
};''')
    ]
    
    for old, new in replacements:
        content = content.replace(old, new)
    
    with open(mod_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed explicit imports in {mod_path}")

def fix_runtime_process_imports():
    """Fix imports in runtime/process.rs to avoid conflicts"""
    process_path = Path('src/runtime/process.rs')
    if not process_path.exists():
        print(f"File not found: {process_path}")
        return
    
    with open(process_path, 'r') as f:
        content = f.read()
    
    # Add type aliases to avoid conflicts with exec_slay
    import_section = '''use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::process::{Command, Child, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_long, c_void};
use std::ptr;

// Alias types to avoid conflicts with exec_slay module
pub type RuntimeProcess = ProcessInfo;
pub type RuntimeProcessInfo = ProcessInfo;
pub type RuntimeProcessStatus = ProcessStatus;'''
    
    # Replace the imports section
    content = re.sub(
        r'use std::collections::HashMap;.*?use std::ptr;',
        import_section,
        content,
        flags=re.DOTALL
    )
    
    # Add explicit module qualifiers where needed
    content = content.replace(
        'pub struct ProcessInfo {',
        'pub struct ProcessInfo {'
    )
    
    with open(process_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed runtime process imports in {process_path}")

def fix_stdlib_mod_exports():
    """Fix stdlib/mod.rs to use qualified exports"""
    stdlib_mod_path = Path('src/stdlib/mod.rs')
    if not stdlib_mod_path.exists():
        print(f"File not found: {stdlib_mod_path}")
        return
    
    with open(stdlib_mod_path, 'r') as f:
        content = f.read()
    
    # Replace conflicting re-exports with qualified ones
    replacements = [
        # Fix exec_slay exports to be qualified
        ('pub use exec_slay::*;', '''pub use exec_slay::{
    SlayCommand, SlayProcess, SlayProcessState, SlayPipeline,
    SlayTask, SlayCommandBuilder, SlayShell, SlayMonitor,
    SlayResult, SlayOptions, SignalOptions, ProcessStats as ExecProcessStats
};'''),
        # Fix process exports to be qualified  
        ('pub use process::*;', '''pub use process::{
    real_monitoring, monitoring, enhanced_monitoring,
    memory_stats, resource_monitoring
};'''),
    ]
    
    for old, new in replacements:
        if old in content:
            content = content.replace(old, new)
    
    with open(stdlib_mod_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed stdlib module exports in {stdlib_mod_path}")

def fix_specific_conflicts():
    """Fix specific file conflicts found in errors"""
    
    # Common patterns of conflicts to fix
    conflict_fixes = {
        'src/stdlib/exec_slay/': {
            'Process': 'crate::runtime::process::ProcessInfo as RuntimeProcessInfo',
            'ProcessInfo': 'crate::runtime::process::ProcessInfo as RuntimeProcessInfo', 
            'ProcessStatus': 'crate::runtime::process::ProcessStatus as RuntimeProcessStatus',
            'Signal': 'crate::runtime::process::signals as RuntimeSignals',
        }
    }
    
    # Apply fixes to exec_slay files
    exec_slay_dir = Path('src/stdlib/exec_slay')
    if exec_slay_dir.exists():
        for file_path in exec_slay_dir.glob('*.rs'):
            if file_path.name == 'mod.rs':
                continue
                
            with open(file_path, 'r') as f:
                content = f.read()
            
            modified = False
            # Add qualified imports at the top if they reference runtime types
            if 'Process' in content and 'use crate::runtime::process' not in content:
                # Add qualified import
                import_line = 'use crate::runtime::process::{ProcessInfo as RuntimeProcessInfo, ProcessStatus as RuntimeProcessStatus};\n'
                # Insert after existing imports
                import_pos = content.find('use crate::error::CursedError;')
                if import_pos != -1:
                    end_pos = content.find('\n', import_pos)
                    content = content[:end_pos+1] + import_line + content[end_pos+1:]
                    modified = True
            
            if modified:
                with open(file_path, 'w') as f:
                    f.write(content)
                print(f"Fixed conflicts in {file_path}")

def create_process_types_module():
    """Create a common process types module to resolve conflicts"""
    types_path = Path('src/runtime/process_types.rs')
    
    types_content = '''//! Common process types to avoid import conflicts

/// Process state enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessState {
    Running = 0,
    Exited = 1,
    Killed = 2,
    Stopped = 3,
    Zombie = 4,
    Unknown = 5,
}

/// Process output structure
#[derive(Debug, Clone)]
pub struct ProcessOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub status: ProcessState,
    pub exit_code: i32,
}

/// Resource limits for processes
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub memory_mb: Option<i32>,
    pub cpu_percent: Option<f64>,
    pub file_descriptors: Option<i32>,
    pub time_limit: Option<std::time::Duration>,
}

/// Security context for process execution
#[derive(Debug, Clone)]
pub struct SecurityContext {
    pub user_id: Option<u32>,
    pub group_id: Option<u32>,
    pub working_directory: Option<std::path::PathBuf>,
    pub environment: std::collections::HashMap<String, String>,
}

/// Process group management
#[derive(Debug, Clone)]
pub struct ProcessGroup {
    pub id: u32,
    pub processes: Vec<u32>,
    pub leader: Option<u32>,
}

/// Enhanced process with additional capabilities
#[derive(Debug)]
pub struct EnhancedProcess {
    pub info: crate::runtime::process::ProcessInfo,
    pub limits: Option<ResourceLimits>,
    pub security: Option<SecurityContext>,
    pub group: Option<ProcessGroup>,
}

/// Signal enumeration for cross-platform compatibility
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    Term = 15,
    Kill = 9,
    Stop = 19,
    Cont = 18,
    Int = 2,
    Hup = 1,
    Usr1 = 10,
    Usr2 = 12,
}

impl Signal {
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}
'''
    
    with open(types_path, 'w') as f:
        f.write(types_content)
    
    print(f"Created common process types module: {types_path}")

def update_runtime_mod():
    """Update runtime/mod.rs to include process_types"""
    runtime_mod_path = Path('src/runtime/mod.rs')
    if not runtime_mod_path.exists():
        print(f"File not found: {runtime_mod_path}")
        return
    
    with open(runtime_mod_path, 'r') as f:
        content = f.read()
    
    # Add process_types module if not already present
    if 'pub mod process_types;' not in content:
        # Find a good place to insert it
        if 'pub mod process;' in content:
            content = content.replace('pub mod process;', 'pub mod process;\npub mod process_types;')
        else:
            # Add at the end of module declarations
            content += '\npub mod process_types;\n'
    
    # Add re-exports
    if 'pub use process_types::' not in content:
        content += '''
// Re-export common process types to avoid conflicts
pub use process_types::{
    ProcessState, ProcessOutput, ResourceLimits, SecurityContext,
    ProcessGroup, EnhancedProcess, Signal
};
'''
    
    with open(runtime_mod_path, 'w') as f:
        f.write(content)
    
    print(f"Updated runtime module exports in {runtime_mod_path}")

def main():
    print("Fixing E0659 import conflicts for process management modules...")
    
    # Get initial error count
    print("\n1. Running initial cargo check...")
    initial_output = run_cargo_check()
    initial_e0659 = len(re.findall(r'error\[E0659\]', initial_output))
    process_e0659 = len(extract_e0659_errors(initial_output))
    
    print(f"Initial E0659 errors: {initial_e0659}")
    print(f"Process-related E0659 errors: {process_e0659}")
    
    if process_e0659 == 0:
        print("No process-related E0659 errors found!")
        return
    
    print("\n2. Creating common process types module...")
    create_process_types_module()
    
    print("\n3. Updating runtime module...")
    update_runtime_mod()
    
    print("\n4. Fixing exec_slay module exports...")
    fix_exec_slay_mod()
    
    print("\n5. Fixing runtime process imports...")
    fix_runtime_process_imports()
    
    print("\n6. Fixing stdlib module exports...")
    fix_stdlib_mod_exports()
    
    print("\n7. Fixing specific conflicts...")
    fix_specific_conflicts()
    
    print("\n8. Running final cargo check...")
    final_output = run_cargo_check()
    final_e0659 = len(re.findall(r'error\[E0659\]', final_output))
    final_process_e0659 = len(extract_e0659_errors(final_output))
    
    print(f"Final E0659 errors: {final_e0659}")
    print(f"Process-related E0659 errors: {final_process_e0659}")
    print(f"Reduction: {initial_e0659 - final_e0659} total, {process_e0659 - final_process_e0659} process-related")
    
    if final_process_e0659 > 0:
        print("\nRemaining process-related E0659 errors:")
        remaining_errors = extract_e0659_errors(final_output)
        for i, error in enumerate(remaining_errors[:5]):  # Show first 5
            print(f"\nError {i+1}:")
            print(error['context'])
    
    print("\nE0659 process conflict fixes completed!")

if __name__ == '__main__':
    main()
