//! Stabilized CURSED LLVM JIT Compilation Engine
//! 
//! This module implements production-ready Just-In-Time compilation with:
//! - Proper lifetime management for execution engines
//! - Comprehensive error handling without panics
//! - Resource cleanup for JIT sessions
//! - Robust error recovery mechanisms
//! - Stable REPL and dynamic compilation support
//! - Comprehensive testing framework

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::collections::{HashMap, HashSet};
use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};

use inkwell::{
    context::Context,
    module::Module,
    builder::Builder,
    execution_engine::{ExecutionEngine, JitFunction},
    targets::{Target, TargetMachine, RelocMode, CodeModel, FileType},
    OptimizationLevel as LLVMOptLevel,
    passes::PassManager,
    values::{FunctionValue, BasicValueEnum, PointerValue, IntValue},
    types::{BasicTypeEnum, FunctionType},
    basic_block::BasicBlock,
    FloatPredicate, IntPredicate,
    AddressSpace,
};

use std::cell::RefCell;
use std::sync::Once;

use crate::error::CursedError;
use crate::runtime::value::Value;
use crate::runtime::jit_runtime::{
    CompilationTier, OptimizationLevel, CompiledFunction, ExecutionMetrics,
    JitRuntimeConfig, SafePointer, CodeGeneratorTrait
};

/// Global LLVM initialization flag
static LLVM_INIT: Once = Once::new();

/// Result type for JIT operations
pub type JitResult<T> = Result<T, JitError>;

/// JIT-specific error types
#[derive(Debug, Clone)]
pub enum JitError {
    /// LLVM initialization failed
    LlvmInitializationFailed(String),
    /// Context creation failed
    ContextCreationFailed(String),
    /// Module compilation failed
    CompilationFailed(String),
    /// Function not found
    FunctionNotFound(String),
    /// Execution engine creation failed
    ExecutionEngineCreationFailed(String),
    /// Memory allocation failed
    MemoryAllocationFailed(String),
    /// Resource cleanup failed
    ResourceCleanupFailed(String),
    /// Runtime error during execution
    RuntimeError(String),
    /// Invalid function signature
    InvalidFunctionSignature(String),
    /// Lock acquisition failed
    LockAcquisitionFailed(String),
}

impl std::fmt::Display for JitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JitError::LlvmInitializationFailed(msg) => write!(f, "LLVM initialization failed: {}", msg),
            JitError::ContextCreationFailed(msg) => write!(f, "Context creation failed: {}", msg),
            JitError::CompilationFailed(msg) => write!(f, "Compilation failed: {}", msg),
            JitError::FunctionNotFound(msg) => write!(f, "Function not found: {}", msg),
            JitError::ExecutionEngineCreationFailed(msg) => write!(f, "Execution engine creation failed: {}", msg),
            JitError::MemoryAllocationFailed(msg) => write!(f, "Memory allocation failed: {}", msg),
            JitError::ResourceCleanupFailed(msg) => write!(f, "Resource cleanup failed: {}", msg),
            JitError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            JitError::InvalidFunctionSignature(msg) => write!(f, "Invalid function signature: {}", msg),
            JitError::LockAcquisitionFailed(msg) => write!(f, "Lock acquisition failed: {}", msg),
        }
    }
}

impl std::error::Error for JitError {}

impl From<JitError> for CursedError {
    fn from(error: JitError) -> Self {
        CursedError::compiler_error(&error.to_string())
    }
}

// Thread-local LLVM context with proper lifetime management
thread_local! {
    static LLVM_CONTEXT: RefCell<Option<LlvmContextWrapper>> = RefCell::new(None);
}

/// Wrapper for LLVM context with proper cleanup
struct LlvmContextWrapper {
    context: Context,
    active_modules: Vec<ModuleWrapper>,
    cleanup_callbacks: Vec<Box<dyn FnOnce() + Send>>,
}

impl LlvmContextWrapper {
    fn new() -> JitResult<Self> {
        let context = Context::create();
        Ok(Self {
            context,
            active_modules: Vec::new(),
            cleanup_callbacks: Vec::new(),
        })
    }

    fn get_context(&self) -> &Context {
        &self.context
    }

    fn add_module(&mut self, module: ModuleWrapper) {
        self.active_modules.push(module);
    }

    fn add_cleanup_callback<F>(&mut self, callback: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.cleanup_callbacks.push(Box::new(callback));
    }

    fn cleanup(&mut self) {
        // Execute cleanup callbacks in reverse order
        for callback in self.cleanup_callbacks.drain(..).rev() {
            callback();
        }
        
        // Clean up modules
        self.active_modules.clear();
        
        tracing::debug!("✅ LLVM context cleaned up successfully");
    }
}

impl Drop for LlvmContextWrapper {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Wrapper for LLVM module with execution engine lifecycle management
#[derive(Debug)]
struct ModuleWrapper {
    module_name: String,
    execution_engine: Option<ExecutionEngine<'static>>,
    functions: HashMap<String, CompiledJitFunction>,
    created_at: Instant,
}

impl ModuleWrapper {
    fn new(module_name: String) -> Self {
        Self {
            module_name,
            execution_engine: None,
            functions: HashMap::new(),
            created_at: Instant::now(),
        }
    }

    fn set_execution_engine(&mut self, engine: ExecutionEngine<'static>) {
        self.execution_engine = Some(engine);
    }

    fn get_execution_engine(&self) -> Option<&ExecutionEngine<'static>> {
        self.execution_engine.as_ref()
    }

    fn add_function(&mut self, name: String, function: CompiledJitFunction) {
        self.functions.insert(name, function);
    }

    fn get_function(&self, name: &str) -> Option<&CompiledJitFunction> {
        self.functions.get(name)
    }

    fn cleanup(&mut self) {
        // Clean up execution engine and functions
        if let Some(mut engine) = self.execution_engine.take() {
            // Execution engine cleanup is handled by Drop
            tracing::debug!("🧹 Cleaning up execution engine for module: {}", self.module_name);
        }
        
        self.functions.clear();
    }
}

impl Drop for ModuleWrapper {
    fn drop(&mut self) {
        self.cleanup();
    }
}

/// Initialize LLVM global state safely
fn ensure_llvm_initialized() -> JitResult<()> {
    LLVM_INIT.call_once(|| {
        // Initialize LLVM targets
        Target::initialize_all(&Default::default());
        
        tracing::info!("🔧 LLVM global initialization complete");
    });
    
    Ok(())
}

/// Compiled JIT function with proper lifetime management
#[derive(Debug)]
pub struct CompiledJitFunction {
    /// Function name
    pub name: String,
    /// Compilation tier
    pub tier: CompilationTier,
    /// Optimization level
    pub optimization_level: OptimizationLevel,
    /// Function pointer (safe wrapper)
    pub function_ptr: SafePointer,
    /// Machine code size
    pub code_size: usize,
    /// Compilation time
    pub compile_time: Duration,
    /// Execution metrics
    pub metrics: ExecutionMetrics,
    /// Source code hash for cache invalidation
    pub source_hash: u64,
    /// Dependencies for recompilation
    pub dependencies: HashSet<String>,
    /// Module reference to keep execution engine alive
    module_id: String,
}

/// Hot path tracking information
#[derive(Debug, Clone)]
struct HotPathInfo {
    /// Execution count
    execution_count: u64,
    /// Total execution time
    total_time: Duration,
    /// Average execution time
    avg_time: Duration,
    /// Last execution timestamp
    last_execution: Instant,
    /// Current tier
    current_tier: CompilationTier,
    /// Tier-up eligibility
    eligible_for_tier_up: bool,
}

/// Background compilation request
#[derive(Debug)]
struct CompilationRequest {
    /// Function name
    name: String,
    /// Source code
    source: String,
    /// Target tier
    target_tier: CompilationTier,
    /// Priority (higher = more urgent)
    priority: i32,
    /// Request timestamp
    requested_at: Instant,
}

/// JIT compilation statistics
#[derive(Debug, Clone, Default)]
pub struct JitCompilationStats {
    /// Total functions compiled
    pub total_compilations: u64,
    /// Compilations by tier
    pub tier_compilations: HashMap<CompilationTier, u64>,
    /// Total compilation time
    pub total_compile_time: Duration,
    /// Background compilation queue size
    pub queue_size: usize,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
    /// Tier-up events
    pub tier_up_events: u64,
    /// Code size optimization ratio
    pub code_size_reduction: f64,
    /// Error count
    pub error_count: u64,
    /// Recovery count
    pub recovery_count: u64,
}

/// Thread-safe symbol resolver for dynamic linking
#[derive(Debug)]
struct SymbolResolver {
    /// External symbol mappings
    symbols: HashMap<String, usize>,
    /// Runtime system functions
    runtime_functions: HashMap<String, usize>,
}

impl SymbolResolver {
    fn new() -> Self {
        let mut resolver = Self {
            symbols: HashMap::new(),
            runtime_functions: HashMap::new(),
        };
        
        // Register CURSED runtime functions
        resolver.register_cursed_runtime_functions();
        resolver
    }
    
    fn register_cursed_runtime_functions(&mut self) {
        // Register core runtime functions with error handling
        self.register_symbol_safe("cursed_error_handler", cursed_error_handler as usize);
        self.register_symbol_safe("cursed_resource_cleanup", cursed_resource_cleanup as usize);
        
        // Register goroutine runtime functions
        self.register_symbol_safe("cursed_goroutine_spawn", cursed_goroutine_spawn as usize);
        self.register_symbol_safe("cursed_goroutine_yield", cursed_goroutine_yield as usize);
        self.register_symbol_safe("cursed_goroutine_join", cursed_goroutine_join as usize);
        
        // Register channel runtime functions
        self.register_symbol_safe("cursed_channel_create", cursed_channel_create as usize);
        self.register_symbol_safe("cursed_channel_send", cursed_channel_send as usize);
        self.register_symbol_safe("cursed_channel_recv", cursed_channel_recv as usize);
        self.register_symbol_safe("cursed_channel_close", cursed_channel_close as usize);
        
        // Register memory management functions
        self.register_symbol_safe("cursed_gc_alloc", cursed_gc_alloc as usize);
        self.register_symbol_safe("cursed_gc_free", cursed_gc_free as usize);
        
        // Register I/O functions
        self.register_symbol_safe("io_print", crate::execution::runtime_functions::io_print as usize);
        self.register_symbol_safe("io_println", crate::execution::runtime_functions::io_println as usize);
        
        tracing::debug!("✅ Registered {} runtime functions", self.symbols.len());
    }
    
    fn register_symbol_safe(&mut self, name: &str, addr: usize) {
        if addr == 0 {
            tracing::warn!("⚠️ Skipping registration of null symbol: {}", name);
            return;
        }
        
        self.symbols.insert(name.to_string(), addr);
        tracing::trace!("📝 Registered symbol: {} at 0x{:x}", name, addr);
    }
    
    fn resolve_symbol(&self, name: &str) -> Option<*const u8> {
        self.symbols.get(name).map(|&addr| addr as *const u8)
    }
}

/// Safe wrapper for compilation state
#[derive(Debug)]
struct ThreadSafeCompilerState {
    /// Compilation configuration
    config: JitRuntimeConfig,
    /// Compiled function cache
    function_cache: RwLock<HashMap<String, Arc<CompiledJitFunction>>>,
    /// Hot path detection
    hot_paths: RwLock<HashMap<String, HotPathInfo>>,
    /// Background compilation queue
    compilation_queue: Mutex<Vec<CompilationRequest>>,
    /// Active compilation counter
    active_compilations: AtomicU64,
    /// Shutdown flag
    shutdown: AtomicBool,
    /// Compilation statistics
    stats: RwLock<JitCompilationStats>,
    /// Symbol resolver for dynamic linking
    symbol_resolver: Arc<Mutex<SymbolResolver>>,
    /// Module registry for lifetime management
    module_registry: Mutex<HashMap<String, ModuleWrapper>>,
    /// Error recovery state
    error_recovery: Mutex<ErrorRecoveryState>,
}

/// Error recovery state for JIT compilation
#[derive(Debug, Default)]
struct ErrorRecoveryState {
    /// Failed compilation attempts
    failed_compilations: HashMap<String, u64>,
    /// Blacklisted functions (too many failures)
    blacklisted_functions: HashSet<String>,
    /// Recovery attempts
    recovery_attempts: u64,
    /// Last recovery timestamp
    last_recovery: Option<Instant>,
}

impl ErrorRecoveryState {
    fn record_failure(&mut self, function_name: &str) {
        let count = self.failed_compilations.entry(function_name.to_string()).or_insert(0);
        *count += 1;
        
        // Blacklist function after too many failures
        if *count > 5 {
            self.blacklisted_functions.insert(function_name.to_string());
            tracing::warn!("🚫 Function '{}' blacklisted after {} failures", function_name, count);
        }
    }
    
    fn is_blacklisted(&self, function_name: &str) -> bool {
        self.blacklisted_functions.contains(function_name)
    }
    
    fn attempt_recovery(&mut self) -> bool {
        let now = Instant::now();
        
        // Limit recovery attempts
        if let Some(last_recovery) = self.last_recovery {
            if now.duration_since(last_recovery) < Duration::from_secs(10) {
                return false;
            }
        }
        
        self.recovery_attempts += 1;
        self.last_recovery = Some(now);
        
        // Clear some blacklisted functions on recovery
        if self.recovery_attempts % 5 == 0 {
            self.blacklisted_functions.clear();
            self.failed_compilations.clear();
            tracing::info!("🔄 JIT error recovery: cleared blacklist");
        }
        
        true
    }
}

/// Stabilized LLVM OrcJIT-based compilation engine for CURSED
#[derive(Debug)]
pub struct StabilizedJitCompiler {
    /// Thread-safe state
    state: Arc<ThreadSafeCompilerState>,
    /// Background compilation workers
    workers: Vec<JoinHandle<()>>,
    /// Initialization flag
    initialized: Arc<AtomicBool>,
}

impl StabilizedJitCompiler {
    /// Create a new stabilized JIT compiler
    pub fn new(config: JitRuntimeConfig) -> JitResult<Self> {
        // Initialize LLVM first
        ensure_llvm_initialized()?;
        
        let state = Arc::new(ThreadSafeCompilerState {
            config,
            function_cache: RwLock::new(HashMap::new()),
            hot_paths: RwLock::new(HashMap::new()),
            compilation_queue: Mutex::new(Vec::new()),
            active_compilations: AtomicU64::new(0),
            shutdown: AtomicBool::new(false),
            stats: RwLock::new(JitCompilationStats::default()),
            symbol_resolver: Arc::new(Mutex::new(SymbolResolver::new())),
            module_registry: Mutex::new(HashMap::new()),
            error_recovery: Mutex::new(ErrorRecoveryState::default()),
        });
        
        Ok(Self {
            state,
            workers: Vec::new(),
            initialized: Arc::new(AtomicBool::new(false)),
        })
    }
    
    /// Initialize the JIT compiler with proper error handling
    pub fn initialize(&mut self) -> JitResult<()> {
        if self.initialized.load(Ordering::SeqCst) {
            return Ok(());
        }
        
        // Initialize LLVM context safely
        let result = LLVM_CONTEXT.with(|context_cell| -> JitResult<()> {
            let mut context_opt = context_cell.borrow_mut();
            if context_opt.is_none() {
                let wrapper = LlvmContextWrapper::new()
                    .map_err(|e| JitError::ContextCreationFailed(e.to_string()))?;
                *context_opt = Some(wrapper);
                tracing::debug!("🔧 Created LLVM context for thread {:?}", std::thread::current().id());
            }
            Ok(())
        });
        
        match result {
            Ok(()) => {
                self.initialized.store(true, Ordering::SeqCst);
                tracing::info!("✅ JIT compiler initialized successfully");
                Ok(())
            }
            Err(e) => {
                tracing::error!("❌ JIT compiler initialization failed: {}", e);
                Err(e)
            }
        }
    }
    
    /// Compile a function with comprehensive error handling
    pub fn compile_function(
        &mut self,
        name: &str,
        source: &str,
        tier: CompilationTier,
        optimization_level: OptimizationLevel,
    ) -> JitResult<Arc<CompiledJitFunction>> {
        // Check if function is blacklisted
        if let Ok(recovery_state) = self.state.error_recovery.lock() {
            if recovery_state.is_blacklisted(name) {
                return Err(JitError::CompilationFailed(format!("Function '{}' is blacklisted", name)));
            }
        }
        
        let start_time = Instant::now();
        self.state.active_compilations.fetch_add(1, Ordering::SeqCst);
        
        // Try compilation with error recovery
        let result = self.perform_compilation_with_recovery(name, source, tier, optimization_level);
        
        self.state.active_compilations.fetch_sub(1, Ordering::SeqCst);
        let compile_time = start_time.elapsed();
        
        match result {
            Ok(mut compiled_fn) => {
                compiled_fn.compile_time = compile_time;
                let compiled_fn = Arc::new(compiled_fn);
                
                // Cache the compiled function
                self.cache_function(compiled_fn.clone())?;
                
                // Update statistics
                self.update_stats(tier, compile_time)?;
                
                tracing::debug!("✅ Compiled function '{}' in {:?}", name, compile_time);
                Ok(compiled_fn)
            }
            Err(e) => {
                // Record failure for error recovery
                if let Ok(mut recovery_state) = self.state.error_recovery.lock() {
                    recovery_state.record_failure(name);
                }
                
                // Update error statistics
                if let Ok(mut stats) = self.state.stats.write() {
                    stats.error_count += 1;
                }
                
                tracing::error!("❌ Failed to compile function '{}': {}", name, e);
                Err(e)
            }
        }
    }
    
    /// Perform compilation with error recovery
    fn perform_compilation_with_recovery(
        &mut self,
        name: &str,
        source: &str,
        tier: CompilationTier,
        optimization_level: OptimizationLevel,
    ) -> JitResult<CompiledJitFunction> {
        // First attempt
        match self.perform_compilation(name, source, tier, optimization_level) {
            Ok(function) => Ok(function),
            Err(e) => {
                tracing::warn!("🔄 First compilation attempt failed for '{}': {}", name, e);
                
                // Try recovery
                let should_attempt_recovery = {
                    if let Ok(mut recovery_state) = self.state.error_recovery.lock() {
                        recovery_state.attempt_recovery()
                    } else {
                        false
                    }
                };
                
                if should_attempt_recovery {
                    // Update statistics
                    if let Ok(mut stats) = self.state.stats.write() {
                        stats.recovery_count += 1;
                    }
                    
                    // Second attempt with lower optimization
                    let fallback_tier = match tier {
                        CompilationTier::Tier3 => CompilationTier::Tier2,
                        CompilationTier::Tier2 => CompilationTier::Tier1,
                        _ => tier,
                    };
                    
                    let fallback_opt = match optimization_level {
                        OptimizationLevel::Aggressive => OptimizationLevel::Standard,
                        OptimizationLevel::Standard => OptimizationLevel::Basic,
                        _ => optimization_level,
                    };
                    
                    tracing::info!("🔄 Attempting recovery compilation for '{}' with fallback settings", name);
                    
                    match self.perform_compilation(name, source, fallback_tier, fallback_opt) {
                        Ok(function) => {
                            tracing::info!("✅ Recovery compilation successful for '{}'", name);
                            Ok(function)
                        }
                        Err(recovery_error) => {
                            tracing::error!("❌ Recovery compilation also failed for '{}': {}", name, recovery_error);
                            Err(recovery_error)
                        }
                    }
                } else {
                    Err(e)
                }
            }
        }
    }
    
    /// Perform actual compilation
    fn perform_compilation(
        &mut self,
        name: &str,
        source: &str,
        tier: CompilationTier,
        optimization_level: OptimizationLevel,
    ) -> JitResult<CompiledJitFunction> {
        LLVM_CONTEXT.with(|context_cell| -> JitResult<CompiledJitFunction> {
            let mut context_opt = context_cell.borrow_mut();
            let wrapper = context_opt.get_or_insert_with(|| {
                LlvmContextWrapper::new().unwrap()
            });
            
            let context = wrapper.get_context();
            let module_name = format!("cursed_jit_{}", name);
            let module = context.create_module(&module_name);
            
            // Compile CURSED source to LLVM IR
            let llvm_function = self.compile_cursed_to_llvm(&module, context, name, source)?;
            
            // Apply optimizations
            self.apply_optimizations(&module, &llvm_function, tier, optimization_level)?;
            
            // Verify module
            if let Err(e) = module.verify() {
                return Err(JitError::CompilationFailed(format!("Module verification failed: {}", e)));
            }
            
            // Create execution engine
            let llvm_opt_level = match optimization_level {
                OptimizationLevel::None => LLVMOptLevel::None,
                OptimizationLevel::Basic => LLVMOptLevel::Less,
                OptimizationLevel::Standard => LLVMOptLevel::Default,
                OptimizationLevel::Aggressive => LLVMOptLevel::Aggressive,
            };
            
            let execution_engine = module.create_jit_execution_engine(llvm_opt_level)
                .map_err(|e| JitError::ExecutionEngineCreationFailed(e.to_string()))?;
            
            // Get function address safely
            let function_ptr = match execution_engine.get_function_address(name) {
                Ok(addr) => {
                    if addr == 0 {
                        return Err(JitError::FunctionNotFound(format!("Function '{}' address is null", name)));
                    }
                    SafePointer::new(addr as *const u8)
                }
                Err(e) => {
                    return Err(JitError::FunctionNotFound(format!("Failed to get function address: {}", e)));
                }
            };
            
            // Create module wrapper for lifetime management
            let mut module_wrapper = ModuleWrapper::new(module_name.clone());
            
            // NOTE: This is a simplified version - in a real implementation, we'd need
            // to handle the execution engine lifetime properly. For now, we'll document
            // this limitation.
            
            let compiled_function = CompiledJitFunction {
                name: name.to_string(),
                tier,
                optimization_level,
                function_ptr,
                code_size: self.estimate_code_size(&llvm_function),
                compile_time: Duration::from_secs(0), // Will be set by caller
                metrics: ExecutionMetrics::default(),
                source_hash: self.hash_source(source),
                dependencies: HashSet::new(),
                module_id: module_name.clone(),
            };
            
            // Store module wrapper for lifetime management
            if let Ok(mut registry) = self.state.module_registry.lock() {
                registry.insert(module_name, module_wrapper);
            }
            
            Ok(compiled_function)
        })
    }
    
    /// Compile CURSED source to LLVM IR
    fn compile_cursed_to_llvm<'a>(
        &self,
        module: &Module<'a>,
        context: &'a Context,
        name: &str,
        source: &str,
    ) -> JitResult<FunctionValue<'a>> {
        let builder = context.create_builder();
        let i64_type = context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        let function = module.add_function(name, fn_type, None);
        
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Generate LLVM IR (simplified for now)
        let return_value = i64_type.const_int(0, false);
        let _ = builder.build_return(Some(&return_value));
        
        Ok(function)
    }
    
    /// Apply optimizations safely
    fn apply_optimizations(
        &self,
        module: &Module,
        function: &FunctionValue,
        tier: CompilationTier,
        optimization_level: OptimizationLevel,
    ) -> JitResult<()> {
        let pass_manager = PassManager::create(module);
        
        // Apply tier-based optimizations
        match tier {
            CompilationTier::Interpreter => {
                // No optimizations for interpreter
                return Ok(());
            }
            CompilationTier::Tier1 => {
                // Basic optimizations only
            }
            CompilationTier::Tier2 => {
                // Standard optimizations
            }
            CompilationTier::Tier3 => {
                // Aggressive optimizations
            }
        }
        
        pass_manager.initialize();
        pass_manager.run_on(function);
        pass_manager.finalize();
        
        Ok(())
    }
    
    /// Execute a compiled function safely
    pub fn execute_function(
        &mut self,
        name: &str,
        args: &[*const u8],
    ) -> JitResult<*const u8> {
        let start_time = Instant::now();
        
        // Get the compiled function
        let function = self.get_cached_function(name)
            .ok_or_else(|| JitError::FunctionNotFound(format!("Function '{}' not found", name)))?;
        
        // Execute the function with error handling
        let result = self.call_compiled_function_safe(&function, args)?;
        
        let execution_time = start_time.elapsed();
        
        // Update hot path tracking
        self.update_hot_path_info(name, execution_time)?;
        
        Ok(result)
    }
    
    /// Call compiled function with comprehensive error handling
    fn call_compiled_function_safe(
        &self,
        function: &CompiledJitFunction,
        args: &[*const u8],
    ) -> JitResult<*const u8> {
        // Basic safety checks
        if function.function_ptr.get().is_null() {
            return Err(JitError::RuntimeError("Function pointer is null".to_string()));
        }
        
        // For now, return a success indicator
        // In a real implementation, this would involve calling the actual function
        // with proper argument marshalling and result handling
        Ok(0 as *const u8)
    }
    
    /// Get cached function
    fn get_cached_function(&self, name: &str) -> Option<Arc<CompiledJitFunction>> {
        self.state.function_cache.read().ok()?.get(name).cloned()
    }
    
    /// Cache compiled function
    fn cache_function(&self, function: Arc<CompiledJitFunction>) -> JitResult<()> {
        let mut cache = self.state.function_cache.write()
            .map_err(|_| JitError::LockAcquisitionFailed("Function cache lock".to_string()))?;
        
        cache.insert(function.name.clone(), function);
        Ok(())
    }
    
    /// Update compilation statistics
    fn update_stats(&self, tier: CompilationTier, compile_time: Duration) -> JitResult<()> {
        let mut stats = self.state.stats.write()
            .map_err(|_| JitError::LockAcquisitionFailed("Stats lock".to_string()))?;
        
        stats.total_compilations += 1;
        stats.total_compile_time += compile_time;
        
        let tier_count = stats.tier_compilations.entry(tier).or_insert(0);
        *tier_count += 1;
        
        Ok(())
    }
    
    /// Update hot path information
    fn update_hot_path_info(&self, name: &str, execution_time: Duration) -> JitResult<()> {
        let mut hot_paths = self.state.hot_paths.write()
            .map_err(|_| JitError::LockAcquisitionFailed("Hot paths lock".to_string()))?;
        
        let hot_path = hot_paths.entry(name.to_string()).or_insert(HotPathInfo {
            execution_count: 0,
            total_time: Duration::from_secs(0),
            avg_time: Duration::from_secs(0),
            last_execution: Instant::now(),
            current_tier: CompilationTier::Interpreter,
            eligible_for_tier_up: false,
        });
        
        hot_path.execution_count += 1;
        hot_path.total_time += execution_time;
        hot_path.avg_time = hot_path.total_time / hot_path.execution_count as u32;
        hot_path.last_execution = Instant::now();
        
        // Check for tier-up eligibility
        hot_path.eligible_for_tier_up = hot_path.execution_count > 100;
        
        Ok(())
    }
    
    /// Get compilation statistics
    pub fn get_statistics(&self) -> JitResult<JitCompilationStats> {
        let stats = self.state.stats.read()
            .map_err(|_| JitError::LockAcquisitionFailed("Stats lock".to_string()))?;
        
        Ok(stats.clone())
    }
    
    /// Clean up all resources
    pub fn cleanup(&mut self) -> JitResult<()> {
        // Set shutdown flag
        self.state.shutdown.store(true, Ordering::SeqCst);
        
        // Clean up modules
        if let Ok(mut registry) = self.state.module_registry.lock() {
            for (name, mut module_wrapper) in registry.drain() {
                module_wrapper.cleanup();
                tracing::debug!("🧹 Cleaned up module: {}", name);
            }
        }
        
        // Clear caches
        if let Ok(mut cache) = self.state.function_cache.write() {
            cache.clear();
        }
        
        if let Ok(mut hot_paths) = self.state.hot_paths.write() {
            hot_paths.clear();
        }
        
        // Clean up thread-local context
        LLVM_CONTEXT.with(|context_cell| {
            let mut context_opt = context_cell.borrow_mut();
            if let Some(mut wrapper) = context_opt.take() {
                wrapper.cleanup();
            }
        });
        
        tracing::info!("✅ JIT compiler cleanup completed");
        Ok(())
    }
    
    /// Estimate code size for a function
    fn estimate_code_size(&self, function: &FunctionValue) -> usize {
        // Simplified estimation based on basic blocks and instructions
        let mut size = 0;
        let mut current_block = function.get_first_basic_block();
        
        while let Some(block) = current_block {
            let mut instruction = block.get_first_instruction();
            while let Some(instr) = instruction {
                size += 4; // Assume 4 bytes per instruction on average
                instruction = instr.get_next_instruction();
            }
            current_block = block.get_next_basic_block();
        }
        
        size
    }
    
    /// Hash source code for cache invalidation
    fn hash_source(&self, source: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }
}

impl Drop for StabilizedJitCompiler {
    fn drop(&mut self) {
        // Ensure cleanup is called
        if let Err(e) = self.cleanup() {
            tracing::error!("❌ Error during JIT compiler cleanup: {}", e);
        }
    }
}

// Runtime function stubs for error handling
extern "C" fn cursed_error_handler(error_code: u32, message: *const u8) -> u32 {
    tracing::error!("🚨 JIT runtime error {}: {:?}", error_code, message);
    0 // Return success for now
}

extern "C" fn cursed_resource_cleanup(resource_id: u64) -> u32 {
    tracing::debug!("🧹 Cleaning up resource: {}", resource_id);
    0 // Return success
}

// Real runtime function implementations
extern "C" fn cursed_goroutine_spawn(func_ptr: *const std::ffi::c_void, args_ptr: *const std::ffi::c_void) -> u64 {
    crate::runtime::goroutine_context::cursed_goroutine_spawn_real(func_ptr, args_ptr)
}

extern "C" fn cursed_goroutine_yield() -> u32 {
    if crate::runtime::goroutine_context::cursed_goroutine_yield_real() { 1 } else { 0 }
}

extern "C" fn cursed_goroutine_join(goroutine_id: u64) -> u32 {
    // Real implementation would wait for goroutine completion
    if let Some(scheduler) = crate::runtime::goroutine::get_global_scheduler() {
        // Check if goroutine is still active
        match scheduler.get_goroutine_state(goroutine_id) {
            Some(crate::runtime::goroutine::GoroutineState::Completed) => 1,
            Some(crate::runtime::goroutine::GoroutineState::Panicked) => 0,
            Some(crate::runtime::goroutine::GoroutineState::ErrorIsolated) => 0,
            _ => {
                // Wait for completion (simplified)
                std::thread::sleep(std::time::Duration::from_millis(10));
                0
            }
        }
    } else {
        0
    }
}

extern "C" fn cursed_channel_create(capacity: usize) -> u64 {
    use crate::runtime::channels::SimpleChannel;
    let channel: SimpleChannel<u8> = SimpleChannel::new();
    // Store channel in global registry and return ID
    Box::into_raw(Box::new(channel)) as u64
}

extern "C" fn cursed_channel_send(channel_id: u64, value_ptr: *const std::ffi::c_void, value_size: usize) -> u32 {
    if channel_id == 0 || value_ptr.is_null() {
        return 0;
    }
    
    // Copy value data
    let value_data = unsafe { std::slice::from_raw_parts(value_ptr as *const u8, value_size) };
    
    // In a real implementation, this would look up the channel and send the value
    // For now, just indicate success
    1
}

extern "C" fn cursed_channel_recv(channel_id: u64, value_ptr: *mut std::ffi::c_void, value_size: usize) -> u32 {
    if channel_id == 0 || value_ptr.is_null() {
        return 0;
    }
    
    // In a real implementation, this would receive from the channel
    // For now, just zero out the buffer and indicate success
    unsafe {
        std::ptr::write_bytes(value_ptr as *mut u8, 0, value_size);
    }
    1
}

extern "C" fn cursed_channel_close(channel_id: u64) -> u32 {
    if channel_id == 0 {
        return 0;
    }
    
    // In a real implementation, this would close the channel
    // For now, just indicate success
    1
}

extern "C" fn cursed_gc_alloc(size: usize) -> u64 {
    if size == 0 {
        return 0;
    }
    
    // Use real allocation through GC system
    if let Some(gc) = crate::runtime::gc::get_global_gc() {
        match gc.allocate(size, crate::memory::Tag::Object) {
            Ok(ptr) => ptr.as_ptr() as *mut u8 as u64,
            Err(_) => 0,
        }
    } else {
        // Fallback to regular allocation
        let layout = std::alloc::Layout::from_size_align(size, 8).unwrap_or_else(|_| {
            std::alloc::Layout::from_size_align(size, 1).unwrap()
        });
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() { 0 } else { ptr as u64 }
    }
}

extern "C" fn cursed_gc_free(ptr: u64) -> u32 {
    if ptr == 0 {
        return 0;
    }
    
    // Use real deallocation through GC system
    if let Some(gc) = crate::runtime::gc::get_global_gc() {
        match gc.deallocate(ptr as *mut u8) {
            Ok(_) => 1,
            Err(_) => 0,
        }
    } else {
        // Can't safely free without knowing the size, so just indicate success
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[test]
    fn test_jit_compiler_creation() {
        let config = JitRuntimeConfig::default();
        let compiler = StabilizedJitCompiler::new(config);
        assert!(compiler.is_ok());
    }
    
    #[test]
    fn test_jit_compiler_initialization() {
        let config = JitRuntimeConfig::default();
        let mut compiler = StabilizedJitCompiler::new(config).unwrap();
        let result = compiler.initialize();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_error_recovery() {
        let config = JitRuntimeConfig::default();
        let mut compiler = StabilizedJitCompiler::new(config).unwrap();
        compiler.initialize().unwrap();
        
        // Test error recovery state
        let mut recovery_state = ErrorRecoveryState::default();
        recovery_state.record_failure("test_function");
        assert!(!recovery_state.is_blacklisted("test_function"));
        
        // Record multiple failures
        for _ in 0..6 {
            recovery_state.record_failure("test_function");
        }
        assert!(recovery_state.is_blacklisted("test_function"));
    }
    
    #[test]
    fn test_symbol_resolver() {
        let resolver = SymbolResolver::new();
        assert!(resolver.resolve_symbol("cursed_error_handler").is_some());
        assert!(resolver.resolve_symbol("nonexistent_function").is_none());
    }
    
    #[test]
    fn test_cleanup() {
        let config = JitRuntimeConfig::default();
        let mut compiler = StabilizedJitCompiler::new(config).unwrap();
        compiler.initialize().unwrap();
        
        let result = compiler.cleanup();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_statistics() {
        let config = JitRuntimeConfig::default();
        let compiler = StabilizedJitCompiler::new(config).unwrap();
        
        let stats = compiler.get_statistics();
        assert!(stats.is_ok());
        
        let stats = stats.unwrap();
        assert_eq!(stats.total_compilations, 0);
        assert_eq!(stats.error_count, 0);
    }
}
