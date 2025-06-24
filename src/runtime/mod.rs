// Minimal runtime module for CURSED minimal build

pub mod stack;
pub mod value;

// Core runtime modules (enable as needed)
pub mod debug_info;
pub mod panic;
pub mod goroutine;
pub mod error_handling;
pub mod error_propagation;
pub mod recovery;
pub mod debug_manager;
pub mod debug_runtime;
pub mod stack_trace;
pub mod stack_walker;
pub mod runtime_error;
pub mod error_context;
pub mod process;
pub mod jit_runtime;

// Async and channels
pub mod r#async;
pub mod channels;

// Basic exports for minimal build
pub use stack::RuntimeStack;
pub use value::{ValueManager, CursedValue};
