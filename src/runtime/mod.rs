/// Runtime system for CURSED
pub mod goroutine;
pub mod channels;
pub mod unicode_char;
pub mod panic;
pub mod recovery;
pub mod stack_trace;
pub mod stack_walker;
pub mod debug_output;
pub mod error_handling;
pub mod debug_info;
pub mod debug_manager;
pub mod error_propagation_runtime;
pub mod debug_runtime;
pub mod error_propagation;
pub mod error_context;

pub use goroutine::{
    GoroutineScheduler, Goroutine, GoroutineState, GoroutineStack,
    SchedulerConfig, GcCoordinator, SafePoint,
    cursed_spawn_goroutine, cursed_yield_goroutine, cursed_safe_point, cursed_gc_requested
};
pub use channels::*;
pub use unicode_char::*;
pub use panic::{
    PanicRuntime, CursedPanicInfo, PanicSeverity, PanicCategory, StackFrame,
    RecoveryAction, RecoveryHandler, PanicConfig, PanicStatistics,
    initialize_panic_runtime, get_panic_runtime, shutdown_panic_runtime,
    cursed_panic, cursed_recover, cursed_has_panic, cursed_get_panic_message,
    no_cap_panic, sus_panic, cap_panic, not_vibing_panic, cursed_panic_with_message,
    cursed_no_cap_panic, cursed_sus_panic, cursed_cap_panic, cursed_not_vibing_panic,
    cursed_panic_message
};
pub use recovery::{
    RecoveryScope, RecoveryConfig, RecoveryManager, RecoveryStatistics, RecoveryScopeGuard,
    initialize_recovery_manager, get_recovery_manager,
    catch_panic, catch_panic_with_config, panic_to_error, error_to_recovery_action,
    is_recoverable_error,
    cursed_enter_recovery_scope, cursed_exit_recovery_scope, cursed_in_recovery_scope,
    cursed_attempt_recovery
};
pub use stack_trace::{
    StackTraceManager, StackTrace, CallFrame, DebugInfo, StackTraceConfig, StackTraceStatistics,
    cursed_stack_enter_function, cursed_stack_exit_function, cursed_get_call_depth
};
pub use stack_walker::{
    StackWalker, RawStackFrame, SourceFrameInfo, ContextualStackWalk, StackWalkConfig, StackWalkStatistics,
    get_global_stack_walker, initialize_global_stack_walker, walk_current_stack
};
pub use debug_output::{
    DebugFormatter, DebugOutputConfig, GenZMessages, 
    format_panic_trace, format_error_with_context, print_gen_z_message
};
pub use error_handling::{
    ErrorRuntime, ErrorContext, ErrorChainEntry, ErrorPropagationConfig, ErrorHandlingStatistics,
    initialize_error_runtime, get_error_runtime, shutdown_error_runtime,
    cursed_propagate_error, cursed_is_in_error_handling, cursed_clear_error_context, cursed_get_error_context_info
};
pub use debug_info::{
    DebugInfo as EnhancedDebugInfo, EnhancedStackFrame, EnhancedStackTrace, VariableInfo,
    StackTraceCapture, StackTraceConfig as EnhancedStackTraceConfig, SymbolResolver, SymbolInfo
};
pub use debug_manager::{
    DebugManager, SourceFile, FunctionDebugInfo, DebugManagerConfig, DebugManagerStats
};
pub use debug_runtime::{
    RuntimeDebugger, RuntimeSymbolTable, RuntimeVariable, RuntimeStackFrame,
    VariableInspector, VariableInspection, TypeAnalysis, BreakpointManager, Breakpoint,
    PerformanceMonitor, PerformanceSummary, DebugReport
};
pub use error_propagation::{
    ErrorPropagationOperator, PropagationError, NoneError, ErrorPropagationContext,
    ErrorContextStack, PropagationStatistics, PropagationConfig, PropagationResult,
    helpers
};
pub use error_context::{
    ErrorContextManager, EnhancedErrorContext, FunctionCallContext, FunctionCallStack,
    SourceInfo, SourceLocationMapper, ErrorContextRegistry, ErrorChainTracker,
    ErrorChain, ErrorContextInfo, ErrorReport, ContextManagerConfig
};
