#!/usr/bin/env python3
"""
Fix final compilation errors to achieve clean build
"""
import os
import re
import subprocess
from pathlib import Path

def run_cargo_check():
    """Run cargo check and return error output"""
    result = subprocess.run(['./fix_linking.sh', 'cargo', 'check'], 
                          capture_output=True, text=True, cwd='.')
    return result.stderr

def fix_missing_imports():
    """Fix missing import issues"""
    # Fix ProcessStats import
    process_integration = 'src/stdlib/process/integration.rs'
    if os.path.exists(process_integration):
        with open(process_integration, 'r') as f:
            content = f.read()
        
        # Fix ProcessStats import
        content = re.sub(
            r'monitoring::\{ProcessStats, HealthStatus, PerformanceMetrics\}',
            r'monitoring::{HealthStatus, PerformanceMetrics}, ProcessStats',
            content
        )
        
        with open(process_integration, 'w') as f:
            f.write(content)
        print(f"Fixed ProcessStats import in {process_integration}")

    # Fix network_error import
    websocket_server = 'src/stdlib/net/websocket/server.rs'
    if os.path.exists(websocket_server):
        with open(websocket_server, 'r') as f:
            content = f.read()
        
        content = re.sub(
            r'use crate::stdlib::net::error::\{NetError, NetResult, websocket_error, network_error\};',
            r'use crate::stdlib::net::error::{NetError, NetResult, websocket_error};',
            content
        )
        
        with open(websocket_server, 'w') as f:
            f.write(content)
        print(f"Fixed network_error import in {websocket_server}")

    # Fix signal_boost error imports
    signal_boost_genz = 'src/stdlib/signal_boost/genZ.rs'
    if os.path.exists(signal_boost_genz):
        with open(signal_boost_genz, 'r') as f:
            content = f.read()
        
        content = re.sub(
            r'use crate::stdlib::signal_boost::error::\{SignalBoostError, SignalBoostResult, invalid_operation, not_found\};',
            r'use crate::stdlib::signal_boost::error::{SignalBoostError, SignalBoostResult};',
            content
        )
        
        # Add error helper functions
        if 'fn invalid_operation(' not in content:
            content += '''

fn invalid_operation(msg: &str) -> SignalBoostError {
    SignalBoostError::InvalidOperation(msg.to_string())
}

fn not_found(msg: &str) -> SignalBoostError {
    SignalBoostError::NotFound(msg.to_string())
}
'''
        
        with open(signal_boost_genz, 'w') as f:
            f.write(content)
        print(f"Fixed signal_boost imports in {signal_boost_genz}")

    # Fix exec_vibez timeout import
    timeout_rs = 'src/stdlib/exec_vibez/timeout.rs'
    if os.path.exists(timeout_rs):
        with open(timeout_rs, 'r') as f:
            content = f.read()
        
        content = re.sub(
            r'use super::error::\{ExecResult, ExecError, execution_failed, timeout_exceeded\};',
            r'use super::error::{ExecResult, ExecError, execution_failed};',
            content
        )
        
        # Add timeout_exceeded function
        if 'fn timeout_exceeded(' not in content:
            content += '''

fn timeout_exceeded(msg: &str) -> ExecError {
    ExecError::Timeout(msg.to_string())
}
'''
        
        with open(timeout_rs, 'w') as f:
            f.write(content)
        print(f"Fixed timeout_exceeded import in {timeout_rs}")

def fix_missing_types():
    """Fix missing type definitions"""
    
    # Fix NewProcessGroup
    groups_rs = 'src/stdlib/exec_vibez/groups.rs'
    if os.path.exists(groups_rs):
        with open(groups_rs, 'r') as f:
            content = f.read()
        
        if 'pub type NewProcessGroup' not in content:
            content += '''

pub type NewProcessGroup = ProcessGroup;
'''
        
        with open(groups_rs, 'w') as f:
            f.write(content)
        print(f"Added NewProcessGroup type alias in {groups_rs}")

    # Fix environment types
    environment_rs = 'src/stdlib/exec_vibez/environment.rs'
    if os.path.exists(environment_rs):
        with open(environment_rs, 'r') as f:
            content = f.read()
        
        if 'pub type NewEnvironment' not in content:
            content += '''

pub type NewEnvironment = Environment;
pub type CommandWithEnv = std::process::Command;
'''
        
        with open(environment_rs, 'w') as f:
            f.write(content)
        print(f"Added environment type aliases in {environment_rs}")

    # Fix streaming types
    streaming_rs = 'src/stdlib/exec_vibez/streaming.rs'
    if os.path.exists(streaming_rs):
        with open(streaming_rs, 'r') as f:
            content = f.read()
        
        if 'pub type NewOutputStreamer' not in content:
            content += '''

pub type NewOutputStreamer = OutputStreamer;
pub type NewInputGenerator = InputGenerator;
'''
        
        with open(streaming_rs, 'w') as f:
            f.write(content)
        print(f"Added streaming type aliases in {streaming_rs}")

    # Fix timeout types
    timeout_rs = 'src/stdlib/exec_vibez/timeout.rs'
    if os.path.exists(timeout_rs):
        with open(timeout_rs, 'r') as f:
            content = f.read()
        
        if 'pub trait RunWithTimeout' not in content:
            content += '''

pub trait RunWithTimeout {
    fn run_with_timeout(&self, timeout: std::time::Duration) -> ExecResult<()>;
}
'''
        
        with open(timeout_rs, 'w') as f:
            f.write(content)
        print(f"Added RunWithTimeout trait in {timeout_rs}")

def fix_context_types():
    """Fix context-related type issues"""
    
    # Fix ProcessContext
    context_rs = 'src/stdlib/exec_vibez/context.rs'
    if os.path.exists(context_rs):
        with open(context_rs, 'r') as f:
            content = f.read()
        
        if 'pub struct ProcessContext' not in content:
            content += '''

#[derive(Debug, Clone)]
pub struct ProcessContext {
    pub environment: std::collections::HashMap<String, String>,
    pub working_dir: Option<std::path::PathBuf>,
    pub timeout: Option<std::time::Duration>,
}

impl ProcessContext {
    pub fn new() -> Self {
        Self {
            environment: std::collections::HashMap::new(),
            working_dir: None,
            timeout: None,
        }
    }
}

impl Default for ProcessContext {
    fn default() -> Self {
        Self::new()
    }
}
'''
        
        with open(context_rs, 'w') as f:
            f.write(content)
        print(f"Added ProcessContext struct in {context_rs}")

def fix_enhanced_types():
    """Fix enhanced module types"""
    
    # Fix LookPath
    enhanced_rs = 'src/stdlib/exec_vibez/enhanced.rs'
    if os.path.exists(enhanced_rs):
        with open(enhanced_rs, 'r') as f:
            content = f.read()
        
        if 'pub trait LookPath' not in content:
            content += '''

pub trait LookPath {
    fn lookup_path(&self) -> Option<std::path::PathBuf>;
}
'''
        
        with open(enhanced_rs, 'w') as f:
            f.write(content)
        print(f"Added LookPath trait in {enhanced_rs}")

def fix_compression_types():
    """Fix compression-related types"""
    
    # Fix compression core types
    core_rs = 'src/stdlib/compression/core.rs'
    if os.path.exists(core_rs):
        with open(core_rs, 'r') as f:
            content = f.read()
        
        missing_types = []
        if 'pub enum CompressionQuality' not in content:
            missing_types.append('''
#[derive(Debug, Clone)]
pub enum CompressionQuality {
    Fast,
    Balanced,
    Best,
}
''')
        
        if 'pub enum CompressionStrategy' not in content:
            missing_types.append('''
#[derive(Debug, Clone)]
pub enum CompressionStrategy {
    Default,
    Filtered,
    HuffmanOnly,
    Rle,
    Fixed,
}
''')
        
        if 'pub enum FlushMode' not in content:
            missing_types.append('''
#[derive(Debug, Clone)]
pub enum FlushMode {
    None,
    Partial,
    Sync,
    Full,
    Finish,
}
''')
        
        if missing_types:
            content += '\n'.join(missing_types)
            with open(core_rs, 'w') as f:
                f.write(content)
            print(f"Added compression types in {core_rs}")

def fix_visibility_issues():
    """Fix visibility issues"""
    
    # Fix SharedProcessState visibility
    state_rs = 'src/stdlib/process/state.rs'
    if os.path.exists(state_rs):
        with open(state_rs, 'r') as f:
            content = f.read()
        
        content = re.sub(
            r'pub\(crate\) struct SharedProcessState',
            r'pub struct SharedProcessState',
            content
        )
        
        with open(state_rs, 'w') as f:
            f.write(content)
        print(f"Fixed SharedProcessState visibility in {state_rs}")

def main():
    """Main execution function"""
    print("Starting final compilation error fixes...")
    
    fix_missing_imports()
    fix_missing_types()
    fix_context_types()
    fix_enhanced_types()
    fix_compression_types()
    fix_visibility_issues()
    
    print("\nRunning compilation check...")
    errors = run_cargo_check()
    error_count = errors.count('error[E')
    print(f"Compilation check complete. Errors found: {error_count}")
    
    if error_count > 0:
        print("\nFirst 20 errors:")
        error_lines = [line for line in errors.split('\n') if 'error[E' in line][:20]
        for line in error_lines:
            print(f"  {line}")

if __name__ == "__main__":
    main()
