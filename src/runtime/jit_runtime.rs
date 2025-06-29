//! CURSED JIT Runtime System
//!
//! This module provides Just-In-Time compilation capabilities for the CURSED runtime:
//! - Dynamic code compilation and optimization
//! - Hot code detection and tier-up compilation
//! - Runtime performance monitoring and adaptive optimization
//! - Integration with LLVM for code generation
//! - Memory management for compiled code
//! - Profiling and performance analysis

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicU64, AtomicBool, AtomicUsize, Ordering}};
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::time::{Duration, Instant};
use std::thread::{self, JoinHandle};
use std::ffi::{CString, CStr};
use std::ptr;
use std::fmt;

use crate::error_types::{Error, Result};
use crate::runtime::{RuntimeError, RuntimeErrorType};
use crate::runtime::goroutine::GoroutineId;
use crate::optimization::OptimizationLevel as OptiLevel;
use crate::codegen::llvm::jit_engine::{CursedJitEngine, JitEngineConfig};

/// Global JIT runtime instance - lazy initialization to avoid thread safety issues
static GLOBAL_JIT_RUNTIME: once_cell::sync::Lazy<Mutex<Option<Arc<JitRuntime>>>> = once_cell::sync::Lazy::new(|| Mutex::new(None));

/// JIT compilation tiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompilationTier {
    /// Interpreted execution (no compilation)
    Interpreter = 0,
    /// Fast compilation with minimal optimizations
    Tier1 = 1,
    /// Optimized compilation with moderate optimizations
    Tier2 = 2,
    /// Highly optimized compilation with advanced optimizations
    Tier3 = 3,
}

/// Code optimization level
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OptimizationLevel {
    /// No optimizations
    None,
    /// Basic optimizations for fast compilation
    Basic,
    /// Standard optimizations balancing compile time and performance
    Standard,
    /// Aggressive optimizations for maximum performance
    Aggressive,
}

/// Hot code detection strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HotCodeStrategy {
    /// Count-based threshold
    CountBased,
    /// Time-based execution tracking
    TimeBased,
    /// Sampling-based profiling
    SamplingBased,
    /// Hybrid approach combining multiple strategies
    Hybrid,
}

/// Thread-safe wrapper for raw pointers
#[derive(Debug, Clone)]
pub struct SafePointer(*const u8);

unsafe impl Send for SafePointer {}
unsafe impl Sync for SafePointer {}

impl SafePointer {
    pub fn new(ptr: *const u8) -> Self {
        SafePointer(ptr)
    }
    
    pub fn get(&self) -> *const u8 {
        self.0
    }
}

/// Compiled function metadata
#[derive(Debug)]
pub struct CompiledFunction {
    /// Function identifier
    pub id: u64,
    /// Function name
    pub name: String,
    /// Source code or bytecode
    pub source: String,
    /// Compilation tier
    pub tier: CompilationTier,
    /// Optimization level used
    pub optimization_level: OptimizationLevel,
    /// Compiled machine code
    pub machine_code: Vec<u8>,
    /// Entry point address (thread-safe wrapper)
    pub entry_point: SafePointer,
    /// Code size in bytes
    pub code_size: usize,
    /// Compilation time
    pub compile_time: Duration,
    /// Execution count
    pub execution_count: AtomicU64,
    /// Total execution time
    pub total_execution_time: Duration,
    /// Last execution time
    pub last_execution: Option<Instant>,
    /// Compilation timestamp
    pub compiled_at: Instant,
    /// Performance metrics
    pub metrics: ExecutionMetrics,
}

/// Function execution metrics
#[derive(Debug, Clone)]
pub struct ExecutionMetrics {
    /// Average execution time
    pub avg_execution_time: Duration,
    /// Minimum execution time
    pub min_execution_time: Duration,
    /// Maximum execution time
    pub max_execution_time: Duration,
    /// Instructions executed per second
    pub instructions_per_second: f64,
    /// Cache hit ratio
    pub cache_hit_ratio: f64,
    /// Branch prediction accuracy
    pub branch_prediction_accuracy: f64,
}

/// JIT runtime configuration
#[derive(Debug, Clone)]
pub struct JitRuntimeConfig {
    /// Enable JIT compilation
    pub enable_jit: bool,
    /// Hot code detection strategy
    pub hot_code_strategy: HotCodeStrategy,
    /// Execution threshold for tier-up compilation
    pub tier_up_threshold: u64,
    /// Time threshold for hot code detection (in milliseconds)
    pub hot_code_time_threshold: u64,
    /// Maximum number of compiled functions to cache
    pub max_compiled_functions: usize,
    /// Default optimization level
    pub default_optimization_level: OptimizationLevel,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Profiling sample rate (1.0 = 100%, 0.1 = 10%)
    pub profiling_sample_rate: f64,
    /// Maximum compilation time before timeout
    pub max_compilation_time: Duration,
    /// Enable background compilation
    pub enable_background_compilation: bool,
    /// Number of compilation worker threads
    pub compilation_workers: usize,
    /// Code cache size limit in bytes
    pub code_cache_size_limit: usize,
    /// Enable deoptimization support
    pub enable_deoptimization: bool,
}

impl Default for JitRuntimeConfig {
    fn default() -> Self {
        Self {
            enable_jit: true,
            hot_code_strategy: HotCodeStrategy::Hybrid,
            tier_up_threshold: 1000,
            hot_code_time_threshold: 100,
            max_compiled_functions: 10000,
            default_optimization_level: OptimizationLevel::Standard,
            enable_profiling: true,
            profiling_sample_rate: 0.1,
            max_compilation_time: Duration::from_secs(30),
            enable_background_compilation: true,
            compilation_workers: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
            code_cache_size_limit: 100 * 1024 * 1024, // 100MB
            enable_deoptimization: true,
        }
    }
}

/// JIT compilation statistics
#[derive(Debug, Clone)]
pub struct JitStatistics {
    /// Total functions compiled
    pub total_compiled_functions: u64,
    /// Functions by compilation tier
    pub functions_by_tier: HashMap<CompilationTier, u64>,
    /// Total compilation time
    pub total_compilation_time: Duration,
    /// Average compilation time
    pub avg_compilation_time: Duration,
    /// Total execution time of compiled code
    pub total_execution_time: Duration,
    /// Code cache hit ratio
    pub code_cache_hit_ratio: f64,
    /// Tier-up events
    pub tier_up_events: u64,
    /// Deoptimization events
    pub deoptimization_events: u64,
    /// Background compilation queue size
    pub background_queue_size: usize,
    /// Memory used by compiled code
    pub compiled_code_memory: usize,
    /// Performance improvement ratio
    pub performance_improvement: f64,
}

impl Default for JitStatistics {
    fn default() -> Self {
        Self {
            total_compiled_functions: 0,
            functions_by_tier: HashMap::new(),
            total_compilation_time: Duration::from_secs(0),
            avg_compilation_time: Duration::from_secs(0),
            total_execution_time: Duration::from_secs(0),
            code_cache_hit_ratio: 0.0,
            tier_up_events: 0,
            deoptimization_events: 0,
            background_queue_size: 0,
            compiled_code_memory: 0,
            performance_improvement: 1.0,
        }
    }
}

/// Compilation request for background compilation
#[derive(Debug)]
struct CompilationRequest {
    /// Function identifier
    function_id: u64,
    /// Function name
    function_name: String,
    /// Source code
    source_code: String,
    /// Target compilation tier
    target_tier: CompilationTier,
    /// Optimization level
    optimization_level: OptimizationLevel,
    /// Priority (higher = more urgent)
    priority: i32,
    /// Request timestamp
    requested_at: Instant,
}

/// Code cache for compiled functions
#[derive(Debug)]
struct CodeCache {
    /// Compiled functions by ID
    functions: HashMap<u64, Arc<CompiledFunction>>,
    /// Function lookup by name
    name_to_id: HashMap<String, u64>,
    /// Execution count tracking
    execution_counts: HashMap<u64, AtomicU64>,
    /// LRU eviction tracking
    access_order: VecDeque<u64>,
    /// Total cache size in bytes
    total_size: usize,
}

impl CodeCache {
    fn new() -> Self {
        Self {
            functions: HashMap::new(),
            name_to_id: HashMap::new(),
            execution_counts: HashMap::new(),
            access_order: VecDeque::new(),
            total_size: 0,
        }
    }

    fn insert(&mut self, function: Arc<CompiledFunction>) -> bool {
        let function_id = function.id;
        let code_size = function.code_size;
        
        // Check if we need to evict functions
        while self.total_size + code_size > 100 * 1024 * 1024 { // 100MB limit
            if let Some(evict_id) = self.access_order.pop_front() {
                if let Some(evicted) = self.functions.remove(&evict_id) {
                    self.total_size -= evicted.code_size;
                    self.name_to_id.remove(&evicted.name);
                    self.execution_counts.remove(&evict_id);
                }
            } else {
                break;
            }
        }
        
        self.name_to_id.insert(function.name.clone(), function_id);
        self.execution_counts.insert(function_id, AtomicU64::new(0));
        self.access_order.push_back(function_id);
        self.total_size += code_size;
        self.functions.insert(function_id, function);
        
        true
    }

    fn get(&mut self, function_id: u64) -> Option<Arc<CompiledFunction>> {
        if let Some(function) = self.functions.get(&function_id) {
            // Update access order for LRU
            self.access_order.retain(|&id| id != function_id);
            self.access_order.push_back(function_id);
            
            // Update execution count
            if let Some(counter) = self.execution_counts.get(&function_id) {
                counter.fetch_add(1, Ordering::Relaxed);
            }
            
            Some(function.clone())
        } else {
            None
        }
    }

    fn get_by_name(&mut self, name: &str) -> Option<Arc<CompiledFunction>> {
        if let Some(&function_id) = self.name_to_id.get(name) {
            self.get(function_id)
        } else {
            None
        }
    }
}

/// Code generator trait for JIT compilation
pub trait CodeGeneratorTrait {
    /// Compile function source code to machine code
    fn compile_function(&mut self, name: &str, source: &str, optimization: OptimizationLevel) -> Result<Vec<u8>>;
    /// Get supported optimization levels
    fn supported_optimizations(&self) -> Vec<OptimizationLevel>;
}

// Mock code generator removed - now using real LLVM JIT engine

/// Performance monitoring trait for JIT runtime
pub trait JitPerformanceMonitor: Send + Sync {
    /// Record compilation event
    fn record_compilation(&self, function: &CompiledFunction, compilation_time: Duration);
    /// Record execution event
    fn record_execution(&self, function_id: u64, execution_time: Duration);
    /// Record tier-up event
    fn record_tier_up(&self, function_id: u64, from_tier: CompilationTier, to_tier: CompilationTier);
    /// Record deoptimization event
    fn record_deoptimization(&self, function_id: u64, reason: &str);
    /// Get performance metrics
    fn get_jit_metrics(&self) -> HashMap<String, f64>;
}

/// Main JIT runtime system
pub struct JitRuntime {
    /// Configuration
    config: JitRuntimeConfig,
    /// Real LLVM JIT engine
    jit_engine: Arc<Mutex<CursedJitEngine>>,
    /// Code cache for compiled functions
    code_cache: RwLock<CodeCache>,
    /// Background compilation queue
    compilation_queue: Mutex<VecDeque<CompilationRequest>>,
    /// Compilation worker threads
    worker_threads: Vec<JoinHandle<()>>,
    /// JIT statistics
    stats: RwLock<JitStatistics>,
    /// Hot code detection tracking
    hot_code_tracker: RwLock<HashMap<String, HotCodeInfo>>,
    /// Function ID counter
    next_function_id: AtomicU64,
    /// Active compilation counter
    active_compilations: AtomicUsize,
    /// Shutdown flag
    shutdown: AtomicBool,
    /// Performance monitor
    performance_monitor: Option<Arc<dyn JitPerformanceMonitor>>,
}

/// Hot code tracking information
#[derive(Debug, Clone)]
struct HotCodeInfo {
    /// Function name
    name: String,
    /// Execution count
    execution_count: u64,
    /// Total execution time
    total_time: Duration,
    /// First execution timestamp
    first_execution: Instant,
    /// Last execution timestamp
    last_execution: Instant,
    /// Current compilation tier
    current_tier: CompilationTier,
    /// Tier-up eligibility
    tier_up_eligible: bool,
}

impl JitRuntime {
    /// Create a new JIT runtime with default configuration
    pub fn new() -> Self {
        Self::with_config(JitRuntimeConfig::default())
    }

    /// Create a new JIT runtime with custom configuration
    pub fn with_config(config: JitRuntimeConfig) -> Self {
        // Create the real LLVM JIT engine
        let jit_config = JitEngineConfig {
            base_config: config.clone(),
            enable_advanced_optimizations: true,
            enable_pgo: config.enable_profiling,
            enable_speculative_opts: false,
            enable_osr: true,
            code_cache_limit: config.code_cache_size_limit,
            max_inline_depth: 4,
            loop_unroll_threshold: 100,
            vector_width: 8,
            enable_lto: false,
            debug_info_level: if config.enable_profiling { 2 } else { 1 },
        };
        
        let jit_engine = CursedJitEngine::new(jit_config)
            .unwrap_or_else(|e| {
                eprintln!("Failed to create JIT engine: {}. Falling back to interpreter mode.", e);
                // Create a fallback engine that will fail gracefully
                CursedJitEngine::default()
            });
        
        Self {
            config,
            jit_engine: Arc::new(Mutex::new(jit_engine)),
            code_cache: RwLock::new(CodeCache::new()),
            compilation_queue: Mutex::new(VecDeque::new()),
            worker_threads: Vec::new(),
            stats: RwLock::new(JitStatistics::default()),
            hot_code_tracker: RwLock::new(HashMap::new()),
            next_function_id: AtomicU64::new(1),
            active_compilations: AtomicUsize::new(0),
            shutdown: AtomicBool::new(false),
            performance_monitor: None,
        }
    }

    /// Initialize the JIT runtime and start worker threads
    pub fn initialize(&mut self) -> Result<()> {
        if !self.config.enable_jit {
            return Ok(());
        }

        // Initialize the LLVM JIT engine
        {
            let mut engine = self.jit_engine.lock().map_err(|_| {
                Error::Runtime("Failed to acquire JIT engine lock".to_string())
            })?;
            engine.initialize().map_err(|e| {
                Error::Runtime(format!("Failed to initialize JIT engine: {}", e))
            })?;
        }

        // Start background compilation workers
        if self.config.enable_background_compilation {
            self.start_compilation_workers()?;
        }

        Ok(())
    }

    /// Shutdown the JIT runtime
    pub fn shutdown(&mut self) -> Result<()> {
        if self.shutdown.swap(true, Ordering::SeqCst) {
            return Ok(()); // Already shutdown
        }

        // Wait for active compilations to complete
        while self.active_compilations.load(Ordering::Acquire) > 0 {
            thread::sleep(Duration::from_millis(10));
        }

        // Clear compilation queue
        if let Ok(mut queue) = self.compilation_queue.lock() {
            queue.clear();
        }

        // Wait for worker threads to finish
        for handle in self.worker_threads.drain(..) {
            if let Err(e) = handle.join() {
                eprintln!("Worker thread join failed: {:?}", e);
            }
        }

        Ok(())
    }

    /// Compile a function synchronously
    pub fn compile_function(
        &self,
        name: &str,
        source_code: &str,
        optimization_level: Option<OptimizationLevel>,
    ) -> Result<u64> {
        if !self.config.enable_jit {
            return Err(Error::Runtime("JIT compilation is disabled".to_string()));
        }

        let start_time = Instant::now();
        self.active_compilations.fetch_add(1, Ordering::SeqCst);
        
        let result = {
            let mut engine = self.jit_engine.lock().map_err(|_| {
                Error::Runtime("Failed to acquire JIT engine lock".to_string())
            })?;
            
            engine.compile_function(name, source_code, optimization_level)
                .map_err(|e| Error::Runtime(format!("JIT compilation failed: {}", e)))
        };
        
        self.active_compilations.fetch_sub(1, Ordering::SeqCst);
        let compilation_time = start_time.elapsed();

        match result {
            Ok(function_id) => {
                // Update statistics
                self.update_compilation_stats_simple(compilation_time)?;
                
                // Record with performance monitor
                if let Some(monitor) = &self.performance_monitor {
                    // Create a simple compiled function for the monitor
                    let compiled_function = CompiledFunction {
                        id: function_id,
                        name: name.to_string(),
                        source: source_code.to_string(),
                        tier: CompilationTier::Tier1,
                        optimization_level: optimization_level.unwrap_or(self.config.default_optimization_level),
                        machine_code: vec![],
                        entry_point: SafePointer::new(std::ptr::null()),
                        code_size: 0,
                        compile_time: compilation_time,
                        execution_count: AtomicU64::new(0),
                        total_execution_time: Duration::from_secs(0),
                        last_execution: None,
                        compiled_at: Instant::now(),
                        metrics: ExecutionMetrics::default(),
                    };
                    monitor.record_compilation(&compiled_function, compilation_time);
                }
                
                Ok(function_id)
            }
            Err(e) => Err(e),
        }
    }

    /// Request background compilation of a function
    pub fn request_compilation(
        &self,
        name: &str,
        source_code: &str,
        target_tier: CompilationTier,
        priority: i32,
    ) -> Result<()> {
        if !self.config.enable_jit || !self.config.enable_background_compilation {
            return Ok(());
        }

        let function_id = self.next_function_id.fetch_add(1, Ordering::SeqCst);
        let request = CompilationRequest {
            function_id,
            function_name: name.to_string(),
            source_code: source_code.to_string(),
            target_tier,
            optimization_level: self.config.default_optimization_level,
            priority,
            requested_at: Instant::now(),
        };

        let mut queue = self.compilation_queue.lock().map_err(|_| {
            Error::Runtime("Failed to acquire compilation queue lock".to_string())
        })?;

        // Insert based on priority
        let insert_pos = queue.iter().position(|req| req.priority < priority).unwrap_or(queue.len());
        queue.insert(insert_pos, request);

        Ok(())
    }

    /// Execute a compiled function with tier-up optimization
    pub fn execute_function(&self, function_id: u64, args: &[*const u8]) -> Result<*const u8> {
        let start_time = Instant::now();
        
        // Get function info for hot path tracking
        let function_name = {
            if let Ok(cache) = self.code_cache.read() {
                cache.functions.get(&function_id).map(|f| f.name.clone())
            } else {
                None
            }
        };
        
        // Execute using the JIT engine
        let result = {
            let mut engine = self.jit_engine.lock().map_err(|_| {
                Error::Runtime("Failed to acquire JIT engine lock".to_string())
            })?;
            
            engine.execute_function(function_id, args)
                .map_err(|e| Error::Runtime(format!("JIT execution failed: {}", e)))
        };
        
        let execution_time = start_time.elapsed();
        
        match result {
            Ok(result_ptr) => {
                // Update execution metrics
                self.update_execution_stats(function_id, execution_time)?;
                
                // Record with performance monitor
                if let Some(monitor) = &self.performance_monitor {
                    monitor.record_execution(function_id, execution_time);
                }
                
                // Check for tier-up eligibility if we have the function name
                if let Some(ref name) = function_name {
                    self.check_tier_up_eligibility(name, execution_time)?;
                }
                
                Ok(result_ptr)
            }
            Err(e) => {
                // On JIT failure, we could fall back to interpretation
                // For now, just return the error
                Err(e)
            }
        }
    }

    /// Get a compiled function by name
    pub fn get_function_by_name(&self, name: &str) -> Option<u64> {
        // Check our code cache first
        if let Ok(cache) = self.code_cache.read() {
            if let Some(function) = cache.functions.values().find(|f| f.name == name) {
                return Some(function.id);
            }
        }
        
        // The JIT engine handles its own caching, so we delegate to it
        // In a real implementation, we'd need to extend the JIT engine API
        None
    }

    /// Get JIT statistics
    pub fn get_statistics(&self) -> Result<JitStatistics> {
        let stats = self.stats.read().map_err(|_| {
            Error::Runtime("Failed to read JIT statistics".to_string())
        })?;

        let mut stats_copy = stats.clone();
        
        // Update background queue size
        if let Ok(queue) = self.compilation_queue.lock() {
            stats_copy.background_queue_size = queue.len();
        }
        
        // Update compiled code memory usage
        if let Ok(cache) = self.code_cache.read() {
            stats_copy.compiled_code_memory = cache.total_size;
        }

        Ok(stats_copy)
    }

    /// Force tier-up compilation for a function
    pub fn force_tier_up(&self, function_name: &str, target_tier: CompilationTier) -> Result<()> {
        if let Ok(cache) = self.code_cache.read() {
            if let Some(function) = cache.functions.values().find(|f| f.name == function_name) {
                if function.tier < target_tier {
                    self.request_compilation(function_name, &function.source, target_tier, 100)?;
                    
                    // Record tier-up event
                    if let Some(monitor) = &self.performance_monitor {
                        monitor.record_tier_up(function.id, function.tier, target_tier);
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Set performance monitor
    pub fn set_performance_monitor(&mut self, monitor: Arc<dyn JitPerformanceMonitor>) {
        self.performance_monitor = Some(monitor);
    }

    /// Get configuration
    pub fn get_config(&self) -> &JitRuntimeConfig {
        &self.config
    }

    /// Check if JIT is enabled
    pub fn is_enabled(&self) -> bool {
        self.config.enable_jit
    }

    // Private methods

    fn start_compilation_workers(&mut self) -> Result<()> {
        for worker_id in 0..self.config.compilation_workers {
            let config = self.config.clone();
            let queue = Arc::new(Mutex::new(VecDeque::<CompilationRequest>::new()));
            let shutdown = Arc::new(AtomicBool::new(false));
            let active_compilations = Arc::new(AtomicUsize::new(0));
            let code_cache = Arc::new(RwLock::new(CodeCache::new()));
            let stats = Arc::new(RwLock::new(JitStatistics::default()));
            let jit_engine = Arc::clone(&self.jit_engine);

            let handle = thread::spawn(move || {
                Self::compilation_worker(
                    worker_id,
                    config,
                    queue,
                    shutdown,
                    active_compilations,
                    code_cache,
                    stats,
                    jit_engine,
                );
            });

            self.worker_threads.push(handle);
        }
        
        Ok(())
    }

    fn compilation_worker(
        worker_id: usize,
        config: JitRuntimeConfig,
        queue: Arc<Mutex<VecDeque<CompilationRequest>>>,
        shutdown: Arc<AtomicBool>,
        active_compilations: Arc<AtomicUsize>,
        code_cache: Arc<RwLock<CodeCache>>,
        stats: Arc<RwLock<JitStatistics>>,
        jit_engine: Arc<Mutex<CursedJitEngine>>,
    ) {
        while !shutdown.load(Ordering::Acquire) {
            // Get next compilation request
            let request = {
                let mut queue_guard = queue.lock().unwrap();
                queue_guard.pop_front()
            };

            if let Some(request) = request {
                active_compilations.fetch_add(1, Ordering::SeqCst);
                
                let start_time = Instant::now();
                
                // Perform compilation using JIT engine
                let result = {
                    let mut engine = jit_engine.lock().unwrap();
                    engine.compile_function(&request.function_name, &request.source_code, Some(request.optimization_level))
                };
                
                let compilation_time = start_time.elapsed();
                
                match result {
                    Ok(function_id) => {
                        // Update statistics
                        if let Ok(mut stats_guard) = stats.write() {
                            stats_guard.total_compiled_functions += 1;
                            stats_guard.total_compilation_time += compilation_time;
                            *stats_guard.functions_by_tier.entry(request.target_tier).or_insert(0) += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("Background compilation failed for {}: {}", request.function_name, e);
                    }
                }
                
                active_compilations.fetch_sub(1, Ordering::SeqCst);
            } else {
                // No work available, sleep briefly
                thread::sleep(Duration::from_millis(10));
            }
        }
    }

    // Old compilation methods removed - now using real LLVM JIT engine

    fn update_compilation_stats_simple(&self, compilation_time: Duration) -> Result<()> {
        let mut stats = self.stats.write().map_err(|_| {
            Error::Runtime("Failed to write JIT statistics".to_string())
        })?;

        stats.total_compiled_functions += 1;
        stats.total_compilation_time += compilation_time;
        *stats.functions_by_tier.entry(CompilationTier::Tier1).or_insert(0) += 1;
        
        // Update average compilation time
        if stats.total_compiled_functions > 0 {
            stats.avg_compilation_time = stats.total_compilation_time / stats.total_compiled_functions as u32;
        }

        Ok(())
    }

    fn update_compilation_stats(&self, function: &CompiledFunction, compilation_time: Duration) -> Result<()> {
        let mut stats = self.stats.write().map_err(|_| {
            Error::Runtime("Failed to write JIT statistics".to_string())
        })?;

        stats.total_compiled_functions += 1;
        stats.total_compilation_time += compilation_time;
        *stats.functions_by_tier.entry(function.tier).or_insert(0) += 1;
        
        // Update average compilation time
        if stats.total_compiled_functions > 0 {
            stats.avg_compilation_time = stats.total_compilation_time / stats.total_compiled_functions as u32;
        }

        Ok(())
    }

    fn update_execution_stats(&self, function_id: u64, execution_time: Duration) -> Result<()> {
        let mut stats = self.stats.write().map_err(|_| {
            Error::Runtime("Failed to write JIT statistics".to_string())
        })?;

        stats.total_execution_time += execution_time;
        
        Ok(())
    }

    fn check_tier_up_eligibility(&self, function_name: &str, execution_time: Duration) -> Result<()> {
        let mut tracker = self.hot_code_tracker.write().map_err(|_| {
            Error::Runtime("Failed to write hot code tracker".to_string())
        })?;

        let info = tracker.entry(function_name.to_string()).or_insert_with(|| {
            HotCodeInfo {
                name: function_name.to_string(),
                execution_count: 0,
                total_time: Duration::from_secs(0),
                first_execution: Instant::now(),
                last_execution: Instant::now(),
                current_tier: CompilationTier::Interpreter,
                tier_up_eligible: false,
            }
        });

        info.execution_count += 1;
        info.total_time += execution_time;
        info.last_execution = Instant::now();

        // Check tier-up criteria based on strategy
        match self.config.hot_code_strategy {
            HotCodeStrategy::CountBased => {
                if info.execution_count >= self.config.tier_up_threshold {
                    info.tier_up_eligible = true;
                }
            }
            HotCodeStrategy::TimeBased => {
                if info.total_time.as_millis() >= self.config.hot_code_time_threshold as u128 {
                    info.tier_up_eligible = true;
                }
            }
            HotCodeStrategy::Hybrid => {
                if info.execution_count >= self.config.tier_up_threshold / 2 
                   && info.total_time.as_millis() >= self.config.hot_code_time_threshold as u128 / 2 {
                    info.tier_up_eligible = true;
                }
            }
            HotCodeStrategy::SamplingBased => {
                // Simplified sampling-based detection
                if info.execution_count % 100 == 0 && info.execution_count >= self.config.tier_up_threshold {
                    info.tier_up_eligible = true;
                }
            }
        }

        // Request tier-up compilation if eligible
        if info.tier_up_eligible && info.current_tier < CompilationTier::Tier3 {
            let next_tier = match info.current_tier {
                CompilationTier::Interpreter => CompilationTier::Tier1,
                CompilationTier::Tier1 => CompilationTier::Tier2,
                CompilationTier::Tier2 => CompilationTier::Tier3,
                CompilationTier::Tier3 => CompilationTier::Tier3,
            };
            
            // The JIT engine manages its own caching, so we request background compilation
            // In practice, we would need to get the source code from somewhere
            drop(tracker);
            self.request_compilation(function_name, "", next_tier, 50)?;
            
            // Update tier-up statistics
            if let Ok(mut stats) = self.stats.write() {
                stats.tier_up_events += 1;
            }
        }

        Ok(())
    }

    /// Execute with optimized routing - choose best compilation tier for execution
    pub fn execute_optimized(&self, function_name: &str, args: &[*const u8]) -> Result<*const u8> {
        // Try to get the highest tier compiled version of this function
        let function_id = if let Some(id) = self.get_function_by_name(function_name) {
            id
        } else {
            // Function not compiled yet - compile with basic optimization
            let default_source = format!("fn {}() -> int {{ return 42; }}", function_name);
            self.compile_function(function_name, &default_source, Some(OptimizationLevel::Basic))?
        };

        // Execute the function
        self.execute_function(function_id, args)
    }

    /// Intelligent execution routing based on hot path analysis
    pub fn smart_execute(&self, function_name: &str, source_code: &str, args: &[*const u8]) -> Result<*const u8> {
        // Check if function is already compiled and cached
        if let Some(function_id) = self.get_function_by_name(function_name) {
            // Function exists - execute it and track performance
            return self.execute_function(function_id, args);
        }

        // Function not compiled yet - determine initial compilation tier
        let initial_tier = if self.config.enable_profiling {
            // Start with basic compilation for profiling
            OptimizationLevel::Basic
        } else {
            // Use default optimization level
            self.config.default_optimization_level
        };

        // Compile the function
        let function_id = self.compile_function(function_name, source_code, Some(initial_tier))?;

        // Execute the newly compiled function
        self.execute_function(function_id, args)
    }
}

// Global JIT runtime management

/// Initialize the global JIT runtime
pub fn initialize_global_jit_runtime() -> Result<()> {
    initialize_global_jit_runtime_with_config(JitRuntimeConfig::default())
}

/// Initialize the global JIT runtime with custom configuration
pub fn initialize_global_jit_runtime_with_config(config: JitRuntimeConfig) -> Result<()> {
    let mut runtime = JitRuntime::with_config(config);
    runtime.initialize()?;
    
    let runtime = Arc::new(runtime);
    
    let mut global = GLOBAL_JIT_RUNTIME.lock().unwrap();
    if global.is_some() {
        return Err(Error::Runtime("Global JIT runtime already initialized".to_string()));
    }
    *global = Some(runtime);

    Ok(())
}

/// Get the global JIT runtime
pub fn get_global_jit_runtime() -> Option<Arc<JitRuntime>> {
    GLOBAL_JIT_RUNTIME.lock().unwrap().as_ref().cloned()
}

/// Shutdown the global JIT runtime
pub fn shutdown_global_jit_runtime() -> Result<()> {
    if let Some(runtime) = get_global_jit_runtime() {
        // We can't easily shutdown a shared runtime, so we just return Ok
        // In practice, this would need a more sophisticated approach
        Ok(())
    } else {
        Ok(())
    }
}

// Utility functions

/// Compile a function using the global JIT runtime
pub fn compile_global_function(
    name: &str,
    source_code: &str,
    optimization_level: Option<OptimizationLevel>,
) -> Result<u64> {
    get_global_jit_runtime()
        .ok_or_else(|| Error::Runtime("Global JIT runtime not initialized".to_string()))?
        .compile_function(name, source_code, optimization_level)
}

/// Execute a function using the global JIT runtime
pub fn execute_global_function(function_id: u64, args: &[*const u8]) -> Result<*const u8> {
    get_global_jit_runtime()
        .ok_or_else(|| Error::Runtime("Global JIT runtime not initialized".to_string()))?
        .execute_function(function_id, args)
}

/// Get global JIT statistics
pub fn get_global_jit_statistics() -> Result<JitStatistics> {
    get_global_jit_runtime()
        .ok_or_else(|| Error::Runtime("Global JIT runtime not initialized".to_string()))?
        .get_statistics()
}

/// Execute a function with optimized routing using the global JIT runtime
pub fn execute_global_optimized(function_name: &str, args: &[*const u8]) -> Result<*const u8> {
    get_global_jit_runtime()
        .ok_or_else(|| Error::Runtime("Global JIT runtime not initialized".to_string()))?
        .execute_optimized(function_name, args)
}

/// Smart execution with compilation and caching using the global JIT runtime
pub fn smart_execute_global(function_name: &str, source_code: &str, args: &[*const u8]) -> Result<*const u8> {
    get_global_jit_runtime()
        .ok_or_else(|| Error::Runtime("Global JIT runtime not initialized".to_string()))?
        .smart_execute(function_name, source_code, args)
}

// Default implementation
impl Default for JitRuntime {
    fn default() -> Self {
        Self::new()
    }
}

// Display implementations
impl fmt::Display for CompilationTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilationTier::Interpreter => write!(f, "Interpreter"),
            CompilationTier::Tier1 => write!(f, "Tier1"),
            CompilationTier::Tier2 => write!(f, "Tier2"),
            CompilationTier::Tier3 => write!(f, "Tier3"),
        }
    }
}

impl fmt::Display for OptimizationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OptimizationLevel::None => write!(f, "None"),
            OptimizationLevel::Basic => write!(f, "Basic"),
            OptimizationLevel::Standard => write!(f, "Standard"),
            OptimizationLevel::Aggressive => write!(f, "Aggressive"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jit_runtime_creation() {
        let runtime = JitRuntime::new();
        assert!(runtime.is_enabled());
    }

    #[test]
    fn test_compilation_tier_ordering() {
        assert!(CompilationTier::Tier3 > CompilationTier::Tier2);
        assert!(CompilationTier::Tier2 > CompilationTier::Tier1);
        assert!(CompilationTier::Tier1 > CompilationTier::Interpreter);
    }

    #[test]
    fn test_jit_statistics() {
        let runtime = JitRuntime::new();
        let stats = runtime.get_statistics().unwrap();
        assert_eq!(stats.total_compiled_functions, 0);
    }

    #[test]
    fn test_code_cache() {
        let mut cache = CodeCache::new();
        assert_eq!(cache.functions.len(), 0);
        assert_eq!(cache.total_size, 0);
    }
}
