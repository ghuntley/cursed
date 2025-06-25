// Minimal runtime module for CURSED minimal build

pub mod stack;
pub mod value;

// Core runtime modules (enable as needed)
pub mod runtime;
// pub mod debug_info;  // DISABLED - causes too many errors
pub mod panic;
pub mod goroutine;
pub mod error_handling;
pub mod error_propagation;
pub mod recovery;
// pub mod debug_manager;  // DISABLED - causing more errors
// pub mod debug_runtime;
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
pub use value::{ValueManager, CursedValue, Value};
pub use runtime::{Runtime, RuntimeConfig, RuntimeStats, RuntimeError, RuntimeErrorType};

// Additional exports needed by other modules
pub use goroutine::{GoroutineScheduler, get_global_scheduler, initialize_global_scheduler, shutdown_global_scheduler};
pub use panic::PanicRuntime;
pub use error_handling::ErrorRuntime;
pub use jit_runtime::JitRuntime;
pub use debug_manager::{DebugManager, DebugManagerConfig, DebugManagerStats, FunctionDebugInfo, SourceFile};
pub use debug_runtime::PerformanceMonitor;
pub use debug_info::{DebugInfo, EnhancedStackFrame, EnhancedStackTrace, VariableInfo, StackTraceCapture, StackTraceConfig, SymbolInfo, SymbolResolver};
