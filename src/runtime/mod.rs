/// Runtime system for CURSED
pub mod value;
pub mod goroutine;
pub mod channels;
pub mod unicode_char;
pub mod panic;
pub mod recovery;
pub mod runtime_value;
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
pub mod jit_runtime;
pub mod process;
pub mod r#async;

pub use value::Value;
pub use runtime_value::{RuntimeValue, TypeInfo, SpecializedRuntimeValue};
pub use goroutine::{
    GoroutineScheduler, Goroutine, GoroutineState, GoroutineStack,
    SchedulerConfig, GcCoordinator, SafePoint,
    initialize_global_scheduler, get_global_scheduler, shutdown_global_scheduler,
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
    helpers,
    cursed_question_mark_operator, cursed_enhanced_question_mark, cursed_check_result,
    cursed_check_option, cursed_error_propagation_check
};
pub use error_propagation_runtime::{
    cursed_error_propagation, cursed_error_propagation_init, cursed_error_propagation_cleanup,
    cursed_error_propagation_panic, cursed_record_error_context
};
pub use error_context::{
    ErrorContextManager, EnhancedErrorContext, FunctionCallContext, FunctionCallStack,
    SourceInfo, SourceLocationMapper, ErrorContextRegistry, ErrorChainTracker,
    ErrorChain, ErrorContextInfo, ErrorReport, ContextManagerConfig
};
pub use jit_runtime::{
    JitRuntime, JitRuntimeConfig, JitRuntimeStats, JitPerformanceMonitor, JitMemoryManager,
    OptimizationOpportunity, OptimizationReason,
    cursed_jit_runtime_init, cursed_jit_execute_function, cursed_jit_runtime_cleanup
};
pub use process::{
    ProcessRuntime, ProcessInfo, ProcessStatus, IpcChannel, IpcChannelType, IpcConfig,
    SharedMemorySegment, SignalHandler,
    initialize_process_runtime, get_process_runtime, shutdown_process_runtime,
    cursed_process_spawn, cursed_process_kill, cursed_process_terminate, cursed_process_pause,
    cursed_process_resume, cursed_process_wait, cursed_process_get_status, cursed_process_get_info,
    cursed_pipe_create, cursed_named_pipe_create, cursed_message_queue_create,
    cursed_shared_memory_create, cursed_socket_create, cursed_semaphore_create,
    cursed_ipc_send, cursed_ipc_receive,
    cursed_shm_create, cursed_shm_open, cursed_shm_map, cursed_shm_unmap,
    cursed_shm_read, cursed_shm_write, cursed_shm_sync, cursed_shm_lock, cursed_shm_unlock,
    cursed_signal_send, cursed_signal_register, cursed_signal_unregister,
    cursed_signal_block, cursed_signal_unblock, cursed_signal_wait
};
pub use r#async::{
    Future, FutureState, FutureResult, FutureError, BoxFuture, LocalFuture,
    Promise, PromiseResolver, PromiseRejecter, PromiseState,
    AsyncExecutor, ExecutorConfig, ExecutorStatistics, TaskQueue,
    AsyncRuntime, AsyncRuntimeConfig, RuntimeStatistics, RuntimeCoordinator,
    Task, TaskId, TaskState, TaskHandle, TaskContext, TaskWaker,
    AsyncScheduler, Timer, Delay, Timeout, Interval, TimerWheel, TimerHandle,
    initialize_async_runtime, get_async_runtime, shutdown_async_runtime,
    spawn, block_on, yield_now, delay, timeout,
    cursed_spawn_async_task, cursed_await_future, cursed_future_is_ready,
    cursed_future_get_result, cursed_create_delay, cursed_create_timeout
};

/// Main runtime system that aggregates all runtime components
pub struct Runtime {
    pub goroutine_scheduler: Option<GoroutineScheduler>,
    pub panic_runtime: Option<std::sync::Arc<PanicRuntime>>,
    pub error_runtime: Option<std::sync::Arc<ErrorRuntime>>,
    pub stack_trace_manager: Option<StackTraceManager>,
    pub debug_manager: Option<DebugManager>,
    pub process_runtime: Option<std::sync::Arc<ProcessRuntime>>,
    pub async_runtime: Option<std::sync::Arc<AsyncRuntime>>,
}

impl Runtime {
    /// Create a new runtime system
    pub fn new() -> Self {
        Self {
            goroutine_scheduler: None,
            panic_runtime: None,
            error_runtime: None,
            stack_trace_manager: None,
            debug_manager: None,
            process_runtime: None,
            async_runtime: None,
        }
    }
    
    /// Initialize all runtime components
    pub fn initialize(&mut self) -> Result<(), crate::error::Error> {
        // Initialize panic runtime
        initialize_panic_runtime();
        if let Some(runtime) = get_panic_runtime() {
            self.panic_runtime = Some(runtime.clone());
        }
        
        // Initialize error runtime
        initialize_error_runtime();
        if let Some(runtime) = get_error_runtime() {
            self.error_runtime = Some(runtime.clone());
        }
        
        // Initialize process runtime
        initialize_process_runtime();
        if let Some(runtime) = get_process_runtime() {
            self.process_runtime = Some(runtime.clone());
        }
        
        // Initialize async runtime
        initialize_async_runtime()?;
        if let Some(runtime) = get_async_runtime() {
            self.async_runtime = Some(runtime.clone());
        }
        
        // Initialize other components as needed
        Ok(())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export process types
pub use process::{ResourceLimits, SecurityContext};
