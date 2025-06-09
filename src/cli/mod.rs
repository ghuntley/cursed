//! Command-line interface utilities for CURSED compiler.
//!
//! This module provides CLI functionality including:
//! - IR and bitcode generation commands
//! - Extended compilation options
//! - Debugging and inspection tools
//! - Optimization control and benchmarking

pub mod ir_commands;
pub mod optimization_commands;

pub use ir_commands::{
    execute_ir_compile, handle_ir_arguments, parse_format, print_ir_help,
    IrCompileArgs,
};

pub use optimization_commands::{
    OptimizationArgs, parse_optimization_args, execute_optimization_command,
    print_optimization_help,
};
