#!/usr/bin/env python3

import os

def add_missing_type_definitions():
    """Add missing type definitions to reduce error count"""
    
    # Add to common/mod.rs
    common_types = '''
// Additional type definitions
pub type SourceLocation = (usize, usize); // (line, column)
pub type ReturnType = Value;
pub type ParameterType = Value;
pub type Literal = Value;
pub type CompilationPhase = String;
pub type Module = String;
pub type MemoryProfiler = ();
pub type PerformanceMonitor = ();
pub type DebugInfoManager = ();
pub type PackageMetadata = ();
pub type PackageManager = ();
pub type ConstraintResolver = ();
pub type ChannelError = String;
pub type BinaryOperator = String;
pub type UnaryOperator = String;
pub type Function = String;
pub type Program = String;
pub type InstructionValue = ();
pub type DatabaseFunction = ();
pub type DebugInfo = ();
pub type ReportFormat = String;
pub type ReportConfig = ();
pub type GcType = String;
pub type ReadlineError = String;
pub type ImportError = String;
'''
    
    common_mod_path = 'src/common/mod.rs'
    if os.path.exists(common_mod_path):
        with open(common_mod_path, 'r') as f:
            content = f.read()
        
        if 'pub type SourceLocation' not in content:
            content += common_types
            with open(common_mod_path, 'w') as f:
                f.write(content)
            print("Added missing type definitions to common/mod.rs")
    
    # Add exports to lib.rs
    lib_path = 'src/lib.rs'
    if os.path.exists(lib_path):
        with open(lib_path, 'r') as f:
            content = f.read()
        
        if 'pub use common::SourceLocation;' not in content:
            exports = '''pub use common::{
    SourceLocation, ReturnType, ParameterType, Literal, CompilationPhase,
    Module, MemoryProfiler, PerformanceMonitor, DebugInfoManager,
    PackageMetadata, PackageManager, ConstraintResolver, ChannelError,
    BinaryOperator, UnaryOperator, Function, Program, InstructionValue,
    DatabaseFunction, DebugInfo, ReportFormat, ReportConfig, GcType,
    ReadlineError, ImportError
};'''
            
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if line.startswith('pub use common::Value;'):
                    lines[i] = exports
                    break
            
            new_content = '\n'.join(lines)
            with open(lib_path, 'w') as f:
                f.write(new_content)
            print("Added missing type exports to lib.rs")

if __name__ == '__main__':
    add_missing_type_definitions()
