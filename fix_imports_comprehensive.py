#!/usr/bin/env python3

import os
import glob
import subprocess

def fix_error_types_imports():
    """Fix all error_types import issues"""
    
    # Find all Rust files with error_types imports
    rust_files = glob.glob("src/**/*.rs", recursive=True)
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Fix common import patterns
            replacements = [
                ('use crate::error_types::', 'use crate::error::'),
                ('use crate::error_types;', 'use crate::error;'),
                ('crate::error_types::', 'crate::error::'),
                ('error_types::', 'error::'),
            ]
            
            for old, new in replacements:
                content = content.replace(old, new)
            
            # Write back if changed
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                print(f"Fixed imports in {file_path}")
                
        except Exception as e:
            print(f"Error processing {file_path}: {e}")

def fix_source_location_imports():
    """Fix SourceLocation imports"""
    
    rust_files = glob.glob("src/**/*.rs", recursive=True)
    
    for file_path in rust_files:
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            original_content = content
            
            # Fix SourceLocation import issues
            if 'use crate::{CursedError, SourceLocation}' in content:
                content = content.replace(
                    'use crate::{CursedError, SourceLocation}',
                    'use crate::error::{CursedError, SourceLocation}'
                )
            
            # Write back if changed
            if content != original_content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(content)
                print(f"Fixed SourceLocation imports in {file_path}")
                
        except Exception as e:
            print(f"Error processing {file_path}: {e}")

def fix_missing_modules():
    """Add missing module declarations to fix unresolved imports"""
    
    # Check if key modules are missing and add stubs
    module_stubs = {
        'src/common_types/mod.rs': '''//! Common types for CURSED language

pub mod optimization_level;
''',
        'src/common_types/optimization_level.rs': '''//! Optimization level definitions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    None,
    Debug,
    Release,
    Max,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::None
    }
}
''',
        'src/types/mod.rs': '''//! Type system modules

pub mod result;
''',
        'src/types/result.rs': '''//! Result type definitions

#[derive(Debug, Clone)]
pub struct ResultTypeExpression {
    // Placeholder
}

#[derive(Debug, Clone)]
pub struct OptionTypeExpression {
    // Placeholder
}
''',
        'src/optimization/mod.rs': '''//! Optimization modules

pub mod config;
pub mod real_llvm_passes; 
pub mod enhanced_llvm_passes_manager;
pub mod coordinator;
pub mod llvm_passes;
pub mod optimization_levels;
pub mod lto;
''',
        'src/runtime/panic.rs': '''//! Panic handling for CURSED runtime

#[derive(Debug, Clone)]
pub enum PanicSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub enum PanicCategory {
    Runtime,
    Memory,
    Thread,
    IO,
}

pub struct CursedPanicInfo {
    pub severity: PanicSeverity,
    pub category: PanicCategory,
    pub message: String,
}

pub struct PanicRuntime;
impl PanicRuntime {
    pub fn new() -> Self { PanicRuntime }
}
''',
        'src/runtime/channels.rs': '''//! Channel implementation

#[derive(Debug)]
pub enum ChannelError {
    Closed,
    Timeout,
}

pub type SendResult<T> = Result<(), ChannelError>;
pub type ReceiveResult<T> = Result<T, ChannelError>;
''',
        'src/runtime/goroutine.rs': '''//! Goroutine scheduler

pub struct GoroutineScheduler;
impl GoroutineScheduler {
    pub fn new() -> Self { GoroutineScheduler }
}
''',
        'src/runtime/error_handling.rs': '''//! Error handling runtime

pub struct ErrorRuntime;
pub struct ErrorContext;
''',
        'src/runtime/recovery.rs': '''//! Recovery management

pub struct RecoveryConfig;

pub fn get_recovery_manager() -> RecoveryManager {
    RecoveryManager
}

pub struct RecoveryManager;
''',
        'src/runtime/stack_trace.rs': '''//! Stack trace management

pub struct StackTraceManager;
pub struct CallFrame;
''',
        'src/memory/object_id.rs': '''//! Object ID tracking

pub struct ObjectId(pub u64);
''',
        'src/memory/gc_types.rs': '''//! Garbage collector types

// Placeholder for GC types
''',
    }
    
    for file_path, content in module_stubs.items():
        os.makedirs(os.path.dirname(file_path), exist_ok=True)
        if not os.path.exists(file_path):
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Created stub module: {file_path}")

def update_mod_files():
    """Update mod.rs files to include new modules"""
    
    mod_updates = {
        'src/lib.rs': [
            'pub mod common_types;',
            'pub mod types;',
            'pub mod optimization;',
        ],
        'src/runtime/mod.rs': [
            'pub mod panic;',
            'pub mod channels;', 
            'pub mod goroutine;',
            'pub mod error_handling;',
            'pub mod recovery;',
            'pub mod stack_trace;',
        ],
        'src/memory/mod.rs': [
            'pub mod object_id;',
            'pub mod gc_types;',
        ]
    }
    
    for mod_file, new_lines in mod_updates.items():
        if os.path.exists(mod_file):
            with open(mod_file, 'r') as f:
                content = f.read()
            
            for line in new_lines:
                if line not in content:
                    content += f"\n{line}"
                    
            with open(mod_file, 'w') as f:
                f.write(content)
            print(f"Updated {mod_file}")

def main():
    print("🔧 Fixing comprehensive import issues...")
    
    fix_error_types_imports()
    fix_source_location_imports()
    fix_missing_modules()
    update_mod_files()
    
    print("\n✅ Import fixes completed")
    
    # Test the build
    print("\n🧪 Testing build...")
    try:
        result = subprocess.run(['cargo', 'check', '--quiet'], 
                              capture_output=True, text=True)
        if result.returncode == 0:
            print("✅ Build passes!")
        else:
            print("❌ Build still has errors:")
            print(result.stderr[:1000])
    except Exception as e:
        print(f"Error running cargo check: {e}")

if __name__ == "__main__":
    main()
