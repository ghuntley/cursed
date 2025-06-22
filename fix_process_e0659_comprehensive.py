#!/usr/bin/env python3
"""
Comprehensive fix for E0659 ambiguous import conflicts in process management modules.
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

def fix_process_mod_exports():
    """Fix src/stdlib/process/mod.rs to use explicit exports instead of wildcards"""
    mod_path = Path('src/stdlib/process/mod.rs')
    if not mod_path.exists():
        print(f"File not found: {mod_path}")
        return
    
    with open(mod_path, 'r') as f:
        content = f.read()
    
    # Replace wildcard imports with explicit imports to avoid conflicts
    explicit_exports = '''// Core types - explicit exports to avoid conflicts
pub use error::{ProcessError, ProcessResult, ProcessErrorKind};
pub use core::{ProcessManager, ProcessHandle};
pub use info::{ProcessInfo as StdProcessInfo, SystemInfo, ProcessState as StdProcessState};
pub use control::{ProcessController, ControlOptions};
pub use enhanced_control::{EnhancedProcessController, EnhancedControlOptions};
pub use communication::{ProcessCommunication, CommunicationChannel};
pub use monitoring::{ProcessMonitor, MonitoringOptions, ProcessMetrics};
pub use platform::{PlatformHandler, PlatformCapabilities};
pub use pipes::{ProcessPipe, PipeOptions};
pub use signals::{SignalHandler, SignalType};
pub use daemon::{DaemonManager, DaemonOptions};
pub use environment::{EnvironmentManager, EnvVar};
pub use lifecycle::{ProcessLifecycleManager, ManagedProcess, LifecycleEvent};

// Exec modules - prefixed to avoid conflicts
pub use exec_slay::{
    SlayProcess as ProcessSlayProcess,
    SlayCommand as ProcessSlayCommand,
    SlayResult as ProcessSlayResult
};
pub use exec_vibez::{
    VibezProcess as ProcessVibezProcess,
    VibezCommand as ProcessVibezCommand,
    VibezResult as ProcessVibezResult
};

// Enhanced modules with prefixes
pub use enhanced_exec_slay::{
    EnhancedSlayProcess,
    EnhancedSlayCommand,
    EnhancedSlayOptions
};

// Monitoring exports
pub use real_monitoring::{
    get_real_cpu_times, get_real_memory_usage, get_real_process_stats,
    RealProcessStats, CpuTimes, MemoryUsage
};

// Integration modules
pub use integration::{ProcessIntegration, IntegrationOptions};
pub use ipc_integration::{IpcIntegration, IpcOptions};
pub use comprehensive_integration::{ComprehensiveProcessManager};

// Pipeline and task management
pub use pipeline::{ProcessPipeline, PipelineStage};
pub use background_tasks::{BackgroundTaskManager, TaskHandle};
pub use shell_commands::{ShellCommandManager, ShellOptions};

// IPC and communication
pub use real_ipc::{RealIpcManager, IpcChannel, IpcMessage};

// System-level operations
pub use fork::{ForkManager, ForkOptions};
pub use resource_limits::{ResourceLimitManager, ResourceLimits};
pub use namespaces::{NamespaceManager, NamespaceOptions};
pub use privileges::{PrivilegeManager, PrivilegeOptions};
pub use mmap::{MemoryMapManager, MmapOptions};'''
    
    # Find the export section and replace it
    export_start = content.find('// Re-export all public types')
    if export_start == -1:
        export_start = content.find('pub use error::*;')
    
    if export_start != -1:
        # Find the end of the export section
        unified_start = content.find('// Unified process-IPC system exports')
        if unified_start == -1:
            unified_start = len(content)
        
        # Replace the export section
        before = content[:export_start]
        after = content[unified_start:]
        content = before + explicit_exports + '\n\n' + after
    else:
        # Add explicit exports at the end of module declarations
        module_end = content.rfind('pub mod ')
        if module_end != -1:
            line_end = content.find('\n', module_end)
            content = content[:line_end+1] + '\n' + explicit_exports + '\n' + content[line_end+1:]
    
    with open(mod_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed process module exports with explicit imports in {mod_path}")

def fix_lifecycle_imports():
    """Fix the lifecycle.rs file to use qualified imports"""
    lifecycle_path = Path('src/stdlib/process/lifecycle.rs')
    if not lifecycle_path.exists():
        print(f"File not found: {lifecycle_path}")
        return
    
    with open(lifecycle_path, 'r') as f:
        content = f.read()
    
    # Replace the problematic import with qualified imports
    old_import = '''use crate::stdlib::process::{
    ProcessError, ProcessResult, ProcessConfig, Process, ProcessInfo, ProcessStatus,
    timeout_error, execution_failed, invalid_state, system_error
};'''
    
    new_import = '''use crate::stdlib::process::error::{
    ProcessError, ProcessResult, timeout_error, execution_failed, invalid_state, system_error
};
use crate::stdlib::process::core::{ProcessConfig};
use crate::stdlib::process::info::{ProcessInfo as StdProcessInfo, ProcessState as StdProcessState};
use crate::runtime::process::{ProcessInfo as RuntimeProcessInfo, ProcessStatus as RuntimeProcessStatus};

// Type aliases to resolve conflicts
type Process = crate::runtime::process::ProcessInfo;
type ProcessInfo = crate::stdlib::process::info::ProcessInfo;
type ProcessStatus = crate::runtime::process::ProcessStatus;'''
    
    content = content.replace(old_import, new_import)
    
    # Also fix any direct usage of ambiguous types
    content = content.replace('Process::', 'RuntimeProcessInfo::')
    content = content.replace(' Process ', ' RuntimeProcessInfo ')
    content = content.replace('(Process)', '(RuntimeProcessInfo)')
    
    with open(lifecycle_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed lifecycle imports in {lifecycle_path}")

def fix_integration_imports():
    """Fix the integration.rs file imports"""
    integration_path = Path('src/stdlib/process/integration.rs')
    if not integration_path.exists():
        print(f"File not found: {integration_path}")
        return
    
    with open(integration_path, 'r') as f:
        content = f.read()
    
    # Find and replace problematic imports
    if 'ProcessOutput' in content and 'use crate::stdlib::process::{' in content:
        # Replace with qualified imports
        import_pattern = r'use crate::stdlib::process::\{[^}]+\};'
        qualified_imports = '''use crate::stdlib::process::error::{ProcessError, ProcessResult};
use crate::stdlib::process::core::{ProcessManager, ProcessHandle};
use crate::stdlib::process::info::{ProcessInfo as StdProcessInfo};
use crate::runtime::process::{ProcessInfo as RuntimeProcessInfo, ProcessOutput as RuntimeProcessOutput};

// Type aliases to resolve conflicts
type ProcessOutput = RuntimeProcessOutput;'''
        
        content = re.sub(import_pattern, qualified_imports, content)
    
    with open(integration_path, 'w') as f:
        f.write(content)
    
    print(f"Fixed integration imports in {integration_path}")

def fix_other_conflicting_files():
    """Fix other files with E0659 conflicts"""
    
    # Files that commonly have conflicts
    conflicting_files = [
        'src/stdlib/process/enhanced_control.rs',
        'src/stdlib/process/communication.rs', 
        'src/stdlib/process/monitoring.rs',
        'src/stdlib/process/exec_slay.rs',
        'src/stdlib/process/exec_vibez.rs',
    ]
    
    for file_path_str in conflicting_files:
        file_path = Path(file_path_str)
        if not file_path.exists():
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Skip if no problematic imports
        if 'use crate::stdlib::process::{' not in content:
            continue
        
        # Replace wildcard imports with qualified ones
        import_pattern = r'use crate::stdlib::process::\{[^}]+\};'
        qualified_imports = '''use crate::stdlib::process::error::{ProcessError, ProcessResult};
use crate::stdlib::process::core::{ProcessManager};
use crate::stdlib::process::info::{ProcessInfo as StdProcessInfo};
use crate::runtime::process::{ProcessInfo as RuntimeProcessInfo};'''
        
        content = re.sub(import_pattern, qualified_imports, content)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"Fixed imports in {file_path}")

def fix_stdlib_mod_process_export():
    """Fix the main stdlib/mod.rs to avoid re-exporting conflicting types"""
    stdlib_mod_path = Path('src/stdlib/mod.rs')
    if not stdlib_mod_path.exists():
        print(f"File not found: {stdlib_mod_path}")
        return
    
    with open(stdlib_mod_path, 'r') as f:
        content = f.read()
    
    # Replace the process wildcard export with specific exports
    old_export = 'pub use process::*;'
    new_export = '''pub use process::{
    ProcessManager, ProcessHandle, ProcessError, ProcessResult,
    ProcessLifecycleManager, ProcessMonitor, ProcessPipeline,
    RealProcessStats, ResourceLimitManager, NamespaceManager
};'''
    
    if old_export in content:
        content = content.replace(old_export, new_export)
        
        with open(stdlib_mod_path, 'w') as f:
            f.write(content)
        
        print(f"Fixed stdlib process exports in {stdlib_mod_path}")

def add_type_disambiguation():
    """Add type disambiguation comments and aliases where needed"""
    
    # Create a disambiguation file
    disambig_path = Path('src/stdlib/process/type_disambiguation.rs')
    
    disambig_content = '''//! Type disambiguation for process management
//! 
//! This module provides type aliases and clarifications to resolve
//! naming conflicts between different process management modules.

/// Runtime process information (from runtime::process)
pub type RuntimeProcessInfo = crate::runtime::process::ProcessInfo;

/// Standard library process information (from stdlib::process::info)
pub type StdProcessInfo = crate::stdlib::process::info::ProcessInfo;

/// Runtime process status (from runtime::process)
pub type RuntimeProcessStatus = crate::runtime::process::ProcessStatus;

/// Standard library process state (from stdlib::process::info)  
pub type StdProcessState = crate::stdlib::process::info::ProcessState;

/// Runtime process output (from runtime::process)
pub type RuntimeProcessOutput = crate::runtime::process::ProcessOutput;

/// Exec slay process (from stdlib::exec_slay)
pub type SlayProcess = crate::stdlib::exec_slay::SlayProcess;

/// Signal type disambiguation
pub type RuntimeSignal = crate::runtime::process::Signal;
pub type ProcessSignal = crate::stdlib::process::signals::SignalType;

/// Resource limits disambiguation  
pub type RuntimeResourceLimits = crate::runtime::process::ResourceLimits;
pub type ProcessResourceLimits = crate::stdlib::process::resource_limits::ResourceLimits;

/// Security context disambiguation
pub type RuntimeSecurityContext = crate::runtime::process::SecurityContext;
pub type ProcessSecurityContext = crate::stdlib::process::privileges::SecurityContext;

/// Process group disambiguation
pub type RuntimeProcessGroup = crate::runtime::process::ProcessGroup;
pub type ProcessGroup = crate::stdlib::process::core::ProcessGroup;

/// Enhanced process disambiguation
pub type RuntimeEnhancedProcess = crate::runtime::process::EnhancedProcess;
pub type ProcessEnhancedProcess = crate::stdlib::process::enhanced_control::EnhancedProcess;
'''

    with open(disambig_path, 'w') as f:
        f.write(disambig_content)
    
    print(f"Created type disambiguation file: {disambig_path}")
    
    # Add it to the process module
    mod_path = Path('src/stdlib/process/mod.rs')
    with open(mod_path, 'r') as f:
        content = f.read()
    
    if 'pub mod type_disambiguation;' not in content:
        # Add after other module declarations
        last_mod = content.rfind('pub mod ')
        if last_mod != -1:
            line_end = content.find('\n', last_mod)
            content = content[:line_end+1] + 'pub mod type_disambiguation;\n' + content[line_end+1:]
        
        # Add re-export
        content += '\n// Type disambiguation exports\npub use type_disambiguation::*;\n'
        
        with open(mod_path, 'w') as f:
            f.write(content)
        
        print(f"Added type disambiguation to process module")

def main():
    print("Fixing E0659 process management import conflicts comprehensively...")
    
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
    
    print("\n2. Creating type disambiguation...")
    add_type_disambiguation()
    
    print("\n3. Fixing process module exports...")
    fix_process_mod_exports()
    
    print("\n4. Fixing lifecycle imports...")
    fix_lifecycle_imports()
    
    print("\n5. Fixing integration imports...")
    fix_integration_imports()
    
    print("\n6. Fixing other conflicting files...")
    fix_other_conflicting_files()
    
    print("\n7. Fixing stdlib module exports...")
    fix_stdlib_mod_process_export()
    
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
        for i, line in enumerate(error_lines[:5]):  # Show first 5
            print(f"  {i+1}. {line.strip()}")
    
    print("\nProcess E0659 fixes completed!")

if __name__ == '__main__':
    main()
