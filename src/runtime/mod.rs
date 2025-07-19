// Minimal runtime module for CURSED minimal build

pub mod stack;
pub mod value;

// Core runtime modules - ENABLING ADVANCED FEATURES
pub mod runtime;
pub mod debug_info;
pub mod dwarf_parser;  // RE-ENABLED - Advanced debug information
pub mod panic;
pub mod panic_recover;
pub mod goroutine;
// pub mod preemptive_scheduler;
pub mod error_handling;
pub mod enhanced_error_handling;
pub mod simple_enhanced_error_handling;
pub mod error_propagation;
pub mod recovery;
pub mod debug_manager;  // RE-ENABLED - Advanced debug management
pub mod debug_runtime;  // RE-ENABLED - Runtime debugging
pub mod stack_trace;
pub mod stack_walker;
pub mod runtime_error;
pub mod error_context;
pub mod production_error;  // Production runtime error handling
pub mod process;
pub mod runtime_value;
pub mod debug_output;
pub mod unicode_char;
pub mod jit_runtime;
pub mod borrowing;
pub mod interface_dispatch;
pub mod type_assertion;
pub mod type_switch;

// Test modules
#[cfg(test)]
pub mod debug_output_tests;
#[cfg(test)]
pub mod production_gc_test;
#[cfg(test)]
pub mod panic_recover_tests;

// Memory management system
pub mod gc;          // Comprehensive garbage collection system
pub mod gc_tuning;   // GC performance tuning and tri-color collection
pub mod memory;      // Memory manager that integrates GC with runtime
pub mod memory_profiler; // Memory profiling and leak detection
pub mod concurrent_gc;   // Concurrent garbage collection
pub mod heap_optimizer;  // Heap allocation optimization
pub mod gc_monitor;      // GC monitoring and alerting
pub mod performance_hooks;  // Runtime performance monitoring hooks
pub mod gc_write_barrier_safe; // Thread-safe write barriers
pub mod heap_optimizer_safe;   // Thread-safe heap optimizer
pub mod gc_race_detector;      // Race condition detection
pub mod memory_bridge;         // FFI bridge for C runtime memory functions

// Async and channels
pub mod r#async;
pub mod channels;
pub mod goroutine_context;
pub mod async_real;
pub mod performance_tracker;

// Basic exports for minimal build
pub use stack::RuntimeStack;
pub use value::{ValueManager, CursedValue, Value};
pub use runtime::{Runtime, RuntimeConfig, RuntimeStats, RuntimeError, RuntimeErrorType, 
                  GoroutineSchedulerTrait, SchedulerStatistics,
                  create_runtime_with_scheduler, create_runtime_with_default_scheduler, 
                  initialize_runtime_with_scheduler};

// Production runtime exports
pub mod production_runtime;
pub use production_runtime::{
    ProductionRuntime, ProductionRuntimeConfig, ProductionRuntimeStats,
    ChannelConfig, ErrorStats, PerformanceMetrics,
    initialize_production_runtime, get_production_runtime, shutdown_production_runtime,
    spawn_goroutine, create_production_channel, create_production_buffered_channel,
    get_production_stats
};

// Memory management exports
pub use gc::{GarbageCollector, GcConfig, GcStats, GcState, RootType,
             GcMemoryManager, RuntimeMemoryManager};
pub use memory::{MemoryManager, MemoryConfig, MemoryStats, MemoryError, ObjectHandle, 
                initialize_memory_manager, get_global_memory_manager, shutdown_memory_manager,
                allocate, allocate_raw, collect_garbage};
pub use memory_profiler::{MemoryProfiler, ProfilingConfig, ProfilingStats, LeakInfo, 
                         initialize_profiler, get_profiler, record_allocation, record_deallocation};
pub use concurrent_gc::{ConcurrentGarbageCollector, ConcurrentGcConfig, ConcurrentStats, 
                       initialize_concurrent_gc, get_concurrent_gc};
pub use heap_optimizer::{HeapOptimizer, HeapOptimizerConfig, HeapStats, AllocationStrategy};
pub use gc_monitor::{GcMonitor, GcMonitorConfig, GcEvent, GcEventType, EventSeverity, 
                    TuningRecommendation, RecommendationType};
pub use memory_bridge::{initialize_memory_system, shutdown_memory_system};

// Additional exports needed by other modules - ADVANCED FEATURES ENABLED
pub use goroutine::{GoroutineScheduler, GoroutineSchedulerWrapper, SchedulerConfig, get_global_scheduler, initialize_global_scheduler, shutdown_global_scheduler};
pub use panic::PanicRuntime;
pub use panic_recover::{PanicRecoverRuntime, cursed_panic, cursed_recover, is_in_panic, with_panic_recovery, reset_panic_state};
pub use error_handling::ErrorRuntime;
pub use simple_enhanced_error_handling::{
    SimpleCursedErrorType, SimpleEnhancedErrorRuntime,
    initialize_simple_error_runtime, get_simple_error_runtime,
    simple_handle_yikes, simple_handle_shook, simple_handle_fam
};
pub use jit_runtime::JitRuntime;
pub use debug_manager::{DebugManager, DebugManagerConfig, DebugManagerStats, FunctionDebugInfo, SourceFile};
pub use debug_runtime::{PerformanceMonitor, RuntimeDebugger, VariableInspection, RuntimeStackFrame, Breakpoint};
pub use debug_info::{
    StackTraceCapture, EnhancedStackTraceConfig, SymbolInfo, SymbolResolver
};
pub use interface_dispatch::{
    InterfaceDispatchRegistry, InterfaceVTable, InterfaceValue, InterfaceMethod, VTableEntry,
    initialize_interface_dispatch, get_global_dispatch_registry, register_global_interface,
    register_global_implementation, create_global_interface_value, dispatch_global_method
};
pub use type_assertion::{
    CursedTypeId, CursedTypeInfo, initialize_type_assertion_runtime, cleanup_type_assertion_runtime
};

// Complete runtime system initialization
pub use runtime::{initialize_complete_runtime, shutdown_complete_runtime};

// Re-export channels and async runtime for goroutine system
pub use channels::{Channel, ChannelSender, ChannelReceiver, ChannelError};
pub use r#async::{AsyncRuntime, AsyncScheduler};
pub use r#async::executor::TaskHandle;

// Performance tracking exports  
pub use performance_tracker::{
    PerformanceTracker, PERFORMANCE_TRACKER, PerformanceReport,
    FutureStats, ContextStats, ThreadStats, NetworkStats
};
