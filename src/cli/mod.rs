//! Command-line interface utilities for CURSED compiler.
//!
//! This module provides CLI functionality including:
//! - IR and bitcode generation commands
//! - Extended compilation options
//! - Debugging and inspection tools
//! - Optimization control and benchmarking
//! - Bootstrap compilation mode

pub mod ir_commands;
pub mod optimization_commands;
pub mod bootstrap_commands;
// pub mod stage2_commands; // Temporarily disabled until bootstrap module is properly set up

pub use ir_commands::{
    execute_ir_compile, handle_ir_arguments, parse_format, print_ir_help,
    IrCompileArgs,
};

pub use optimization_commands::{
    OptimizationArgs, parse_optimization_args, execute_optimization_command,
    print_optimization_help,
};

pub use bootstrap_commands::{
    BootstrapArgs, BootstrapCommand, create_bootstrap_command,
    parse_bootstrap_args, execute_bootstrap_command, print_bootstrap_help,
};

// pub use stage2_commands::{
//     Stage2Args, Stage2Command, parse_stage2_args, execute_stage2_command,
//     print_stage2_help, verify_compiler_stage, get_compiler_info,
// };
