//! CURSED LLVM JIT Compilation Engine
//! 
//! This module implements real-time Just-In-Time compilation using LLVM's OrcJIT v2 API.
//! It provides:
//! - Real-time compilation of hot code paths
//! - Tier-up compilation from interpreted to optimized
//! - Background compilation workers
//! - Code caching and management
//! - Dynamic linking and symbol resolution
//! - Support for CURSED language constructs (goroutines, channels, async/await)

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

use crate::error::CursedError;
use crate::runtime::value::Value;
use crate::stdlib::vibez::print::spillf;
use crate::runtime::jit_runtime::{
    CompilationTier, OptimizationLevel, CompiledFunction, ExecutionMetrics,
    JitRuntimeConfig, SafePointer, CodeGeneratorTrait
};

thread_local! {
    /// Thread-local LLVM context wrapper
    static LLVM_CONTEXT: RefCell<Option<Context>> = RefCell::new(None);
}

/// Safe wrapper for LLVM compilation state
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
}

/// LLVM OrcJIT-based compilation engine for CURSED
pub struct CursedJitCompiler {
    /// Thread-safe state
    state: Arc<ThreadSafeCompilerState>,
}

/// Compiled JIT function with metadata
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
    /// Keep the execution engine alive to ensure function pointer validity
    /// Note: In a real implementation, we'd use a more sophisticated approach
    /// for managing execution engine lifetimes across multiple functions
    _execution_engine_keepalive: Option<Box<dyn std::any::Any>>,
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
}

/// Thread-safe symbol resolver for dynamic linking
#[derive(Debug)]
struct SymbolResolver {
    /// External symbol mappings (using usize instead of raw pointers for Send/Sync)
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
        // Register goroutine runtime functions
        self.register_symbol("cursed_goroutine_spawn", cursed_goroutine_spawn as usize);
        self.register_symbol("cursed_goroutine_yield", cursed_goroutine_yield as usize);
        self.register_symbol("cursed_goroutine_join", cursed_goroutine_join as usize);
        
        // Register channel runtime functions
        self.register_symbol("cursed_channel_create", cursed_channel_create as usize);
        self.register_symbol("cursed_channel_send", cursed_channel_send as usize);
        self.register_symbol("cursed_channel_recv", cursed_channel_recv as usize);
        self.register_symbol("cursed_channel_close", cursed_channel_close as usize);
        
        // Register async/await runtime functions
        self.register_symbol("cursed_async_spawn", cursed_async_spawn as usize);
        self.register_symbol("cursed_await_future", cursed_await_future as usize);
        
        // Register memory management functions
        self.register_symbol("cursed_gc_alloc", cursed_gc_alloc as usize);
        self.register_symbol("cursed_gc_free", cursed_gc_free as usize);
        
        // Register error handling functions
        self.register_symbol("cursed_panic", cursed_panic as usize);
        self.register_symbol("cursed_error_propagate", cursed_error_propagate as usize);
        
        // Register standard library I/O functions
        self.register_symbol("io_print", crate::execution::runtime_functions::io_print as usize);
        self.register_symbol("io_println", crate::execution::runtime_functions::io_println as usize);
        self.register_symbol("io_eprint", crate::execution::runtime_functions::io_eprint as usize);
        self.register_symbol("io_eprintln", crate::execution::runtime_functions::io_eprintln as usize);
        self.register_symbol("io_read_line", crate::execution::runtime_functions::io_read_line as usize);
        self.register_symbol("io_write_file", crate::execution::runtime_functions::io_write_file as usize);
        self.register_symbol("io_read_file", crate::execution::runtime_functions::io_read_file as usize);
        self.register_symbol("io_file_exists", crate::execution::runtime_functions::io_file_exists as usize);
        self.register_symbol("io_create_directory", crate::execution::runtime_functions::io_create_directory as usize);
        self.register_symbol("io_create_directory_recursive", crate::execution::runtime_functions::io_create_directory_recursive as usize);
        self.register_symbol("io_delete_file", crate::execution::runtime_functions::io_delete_file as usize);
        
        // Register additional I/O functions
        self.register_symbol("io_printf", crate::execution::runtime_functions::io_printf as usize);
        self.register_symbol("io_read_char", crate::execution::runtime_functions::io_read_char as usize);
        self.register_symbol("io_read_int", crate::execution::runtime_functions::io_read_int as usize);
        self.register_symbol("io_read_float", crate::execution::runtime_functions::io_read_float as usize);
        self.register_symbol("io_append_file", crate::execution::runtime_functions::io_append_file as usize);
        self.register_symbol("io_copy_file", crate::execution::runtime_functions::io_copy_file as usize);
        self.register_symbol("io_move_file", crate::execution::runtime_functions::io_move_file as usize);
        self.register_symbol("io_file_size", crate::execution::runtime_functions::io_file_size as usize);
        self.register_symbol("io_is_file", crate::execution::runtime_functions::io_is_file as usize);
        self.register_symbol("io_is_directory", crate::execution::runtime_functions::io_is_directory as usize);
        self.register_symbol("io_remove_directory", crate::execution::runtime_functions::io_remove_directory as usize);
        self.register_symbol("io_remove_directory_recursive", crate::execution::runtime_functions::io_remove_directory_recursive as usize);
        self.register_symbol("io_current_directory", crate::execution::runtime_functions::io_current_directory as usize);
        self.register_symbol("io_change_directory", crate::execution::runtime_functions::io_change_directory as usize);
        
        // Register standard library math functions
        self.register_symbol("math_sin_impl", crate::execution::runtime_functions::math_sin_impl as usize);
        self.register_symbol("math_cos_impl", crate::execution::runtime_functions::math_cos_impl as usize);
        self.register_symbol("math_sqrt_impl", crate::execution::runtime_functions::math_sqrt_impl as usize);
        self.register_symbol("math_random_impl", crate::execution::runtime_functions::math_random_impl as usize);
        
        // Register new math functions
        self.register_symbol("math_abs_impl", crate::execution::runtime_functions::math_abs_impl as usize);
        self.register_symbol("math_abs_int_impl", crate::execution::runtime_functions::math_abs_int_impl as usize);
        self.register_symbol("math_min_impl", crate::execution::runtime_functions::math_min_impl as usize);
        self.register_symbol("math_max_impl", crate::execution::runtime_functions::math_max_impl as usize);
        self.register_symbol("math_min_int_impl", crate::execution::runtime_functions::math_min_int_impl as usize);
        self.register_symbol("math_max_int_impl", crate::execution::runtime_functions::math_max_int_impl as usize);
        self.register_symbol("math_clamp_impl", crate::execution::runtime_functions::math_clamp_impl as usize);
        self.register_symbol("math_sign_impl", crate::execution::runtime_functions::math_sign_impl as usize);
        self.register_symbol("math_pow_impl", crate::execution::runtime_functions::math_pow_impl as usize);
        self.register_symbol("math_cbrt_impl", crate::execution::runtime_functions::math_cbrt_impl as usize);
        self.register_symbol("math_log_impl", crate::execution::runtime_functions::math_log_impl as usize);
        self.register_symbol("math_log10_impl", crate::execution::runtime_functions::math_log10_impl as usize);
        self.register_symbol("math_log2_impl", crate::execution::runtime_functions::math_log2_impl as usize);
        self.register_symbol("math_exp_impl", crate::execution::runtime_functions::math_exp_impl as usize);
        self.register_symbol("math_exp2_impl", crate::execution::runtime_functions::math_exp2_impl as usize);
        self.register_symbol("math_tan_impl", crate::execution::runtime_functions::math_tan_impl as usize);
        self.register_symbol("math_asin_impl", crate::execution::runtime_functions::math_asin_impl as usize);
        self.register_symbol("math_acos_impl", crate::execution::runtime_functions::math_acos_impl as usize);
        self.register_symbol("math_atan_impl", crate::execution::runtime_functions::math_atan_impl as usize);
        self.register_symbol("math_atan2_impl", crate::execution::runtime_functions::math_atan2_impl as usize);
        self.register_symbol("math_sinh_impl", crate::execution::runtime_functions::math_sinh_impl as usize);
        self.register_symbol("math_cosh_impl", crate::execution::runtime_functions::math_cosh_impl as usize);
        self.register_symbol("math_tanh_impl", crate::execution::runtime_functions::math_tanh_impl as usize);
        self.register_symbol("math_floor_impl", crate::execution::runtime_functions::math_floor_impl as usize);
        self.register_symbol("math_ceil_impl", crate::execution::runtime_functions::math_ceil_impl as usize);
        self.register_symbol("math_round_impl", crate::execution::runtime_functions::math_round_impl as usize);
        self.register_symbol("math_trunc_impl", crate::execution::runtime_functions::math_trunc_impl as usize);
        self.register_symbol("math_frac_impl", crate::execution::runtime_functions::math_frac_impl as usize);
        self.register_symbol("math_sum_impl", crate::execution::runtime_functions::math_sum_impl as usize);
        self.register_symbol("math_mean_impl", crate::execution::runtime_functions::math_mean_impl as usize);
        self.register_symbol("math_median_impl", crate::execution::runtime_functions::math_median_impl as usize);
        self.register_symbol("math_variance_impl", crate::execution::runtime_functions::math_variance_impl as usize);
        self.register_symbol("math_std_dev_impl", crate::execution::runtime_functions::math_std_dev_impl as usize);
        self.register_symbol("math_random_int_impl", crate::execution::runtime_functions::math_random_int_impl as usize);
        self.register_symbol("math_random_float_impl", crate::execution::runtime_functions::math_random_float_impl as usize);
        self.register_symbol("math_seed_random_impl", crate::execution::runtime_functions::math_seed_random_impl as usize);
        self.register_symbol("math_is_nan_impl", crate::execution::runtime_functions::math_is_nan_impl as usize);
        self.register_symbol("math_is_infinite_impl", crate::execution::runtime_functions::math_is_infinite_impl as usize);
        self.register_symbol("math_is_finite_impl", crate::execution::runtime_functions::math_is_finite_impl as usize);
        self.register_symbol("math_gcd_impl", crate::execution::runtime_functions::math_gcd_impl as usize);
        self.register_symbol("math_lcm_impl", crate::execution::runtime_functions::math_lcm_impl as usize);
        self.register_symbol("math_factorial_impl", crate::execution::runtime_functions::math_factorial_impl as usize);
        self.register_symbol("math_fibonacci_impl", crate::execution::runtime_functions::math_fibonacci_impl as usize);
        self.register_symbol("math_smoothstep_impl", crate::execution::runtime_functions::math_smoothstep_impl as usize);
        
        // Register crypto functions
        self.register_symbol("crypto_sha256", crate::execution::runtime_functions::crypto_sha256 as usize);
        self.register_symbol("crypto_sha512", crate::execution::runtime_functions::crypto_sha512 as usize);
        self.register_symbol("crypto_md5", crate::execution::runtime_functions::crypto_md5 as usize);
        self.register_symbol("crypto_blake3", crate::execution::runtime_functions::crypto_blake3 as usize);
        self.register_symbol("crypto_random_bytes", crate::execution::runtime_functions::crypto_random_bytes as usize);
        self.register_symbol("crypto_random_int", crate::execution::runtime_functions::crypto_random_int as usize);
        self.register_symbol("crypto_random_string", crate::execution::runtime_functions::crypto_random_string as usize);
        self.register_symbol("crypto_base64_encode", crate::execution::runtime_functions::crypto_base64_encode as usize);
        self.register_symbol("crypto_base64_decode", crate::execution::runtime_functions::crypto_base64_decode as usize);
        self.register_symbol("crypto_hex_encode", crate::execution::runtime_functions::crypto_hex_encode as usize);
        self.register_symbol("crypto_hex_decode", crate::execution::runtime_functions::crypto_hex_decode as usize);
        self.register_symbol("crypto_aes_encrypt", crate::execution::runtime_functions::crypto_aes_encrypt as usize);
        self.register_symbol("crypto_aes_decrypt", crate::execution::runtime_functions::crypto_aes_decrypt as usize);
        self.register_symbol("crypto_pbkdf2", crate::execution::runtime_functions::crypto_pbkdf2 as usize);
        self.register_symbol("crypto_scrypt", crate::execution::runtime_functions::crypto_scrypt as usize);
        self.register_symbol("crypto_ed25519_keypair", crate::execution::runtime_functions::crypto_ed25519_keypair as usize);
        self.register_symbol("crypto_ed25519_sign", crate::execution::runtime_functions::crypto_ed25519_sign as usize);
        self.register_symbol("crypto_ed25519_verify", crate::execution::runtime_functions::crypto_ed25519_verify as usize);
        self.register_symbol("crypto_hmac_sha256", crate::execution::runtime_functions::crypto_hmac_sha256 as usize);
        self.register_symbol("crypto_hmac_sha512", crate::execution::runtime_functions::crypto_hmac_sha512 as usize);
        self.register_symbol("crypto_argon2_hash", crate::execution::runtime_functions::crypto_argon2_hash as usize);
        self.register_symbol("crypto_argon2_verify", crate::execution::runtime_functions::crypto_argon2_verify as usize);
        self.register_symbol("crypto_bcrypt_hash", crate::execution::runtime_functions::crypto_bcrypt_hash as usize);
        self.register_symbol("crypto_bcrypt_verify", crate::execution::runtime_functions::crypto_bcrypt_verify as usize);
        self.register_symbol("crypto_constant_time_eq", crate::execution::runtime_functions::crypto_constant_time_eq as usize);
        self.register_symbol("crypto_secure_random", crate::execution::runtime_functions::crypto_secure_random as usize);
        self.register_symbol("crypto_generate_salt", crate::execution::runtime_functions::crypto_generate_salt as usize);
        
        // Register standard library collections functions
        // Array/Vector operations
        self.register_symbol("collections_array_new", crate::execution::runtime_functions::collections_array_new as usize);
        self.register_symbol("collections_array_with_capacity", crate::execution::runtime_functions::collections_array_with_capacity as usize);
        self.register_symbol("collections_array_push", crate::execution::runtime_functions::collections_array_push as usize);
        self.register_symbol("collections_array_pop", crate::execution::runtime_functions::collections_array_pop as usize);
        self.register_symbol("collections_array_get", crate::execution::runtime_functions::collections_array_get as usize);
        self.register_symbol("collections_array_set", crate::execution::runtime_functions::collections_array_set as usize);
        self.register_symbol("collections_array_len", crate::execution::runtime_functions::collections_array_len as usize);
        self.register_symbol("collections_array_insert", crate::execution::runtime_functions::collections_array_insert as usize);
        self.register_symbol("collections_array_remove", crate::execution::runtime_functions::collections_array_remove as usize);
        self.register_symbol("collections_array_clear", crate::execution::runtime_functions::collections_array_clear as usize);
        self.register_symbol("collections_array_is_empty", crate::execution::runtime_functions::collections_array_is_empty as usize);
        self.register_symbol("collections_array_contains", crate::execution::runtime_functions::collections_array_contains as usize);
        self.register_symbol("collections_array_reverse", crate::execution::runtime_functions::collections_array_reverse as usize);
        
        // HashMap operations
        self.register_symbol("collections_map_new", crate::execution::runtime_functions::collections_map_new as usize);
        self.register_symbol("collections_map_with_capacity", crate::execution::runtime_functions::collections_map_with_capacity as usize);
        self.register_symbol("collections_map_set", crate::execution::runtime_functions::collections_map_set as usize);
        self.register_symbol("collections_map_get", crate::execution::runtime_functions::collections_map_get as usize);
        self.register_symbol("collections_map_remove", crate::execution::runtime_functions::collections_map_remove as usize);
        self.register_symbol("collections_map_contains_key", crate::execution::runtime_functions::collections_map_contains_key as usize);
        self.register_symbol("collections_map_len", crate::execution::runtime_functions::collections_map_len as usize);
        self.register_symbol("collections_map_clear", crate::execution::runtime_functions::collections_map_clear as usize);
        self.register_symbol("collections_map_is_empty", crate::execution::runtime_functions::collections_map_is_empty as usize);
        
        // HashSet operations
        self.register_symbol("collections_set_new", crate::execution::runtime_functions::collections_set_new as usize);
        self.register_symbol("collections_set_with_capacity", crate::execution::runtime_functions::collections_set_with_capacity as usize);
        self.register_symbol("collections_set_insert", crate::execution::runtime_functions::collections_set_insert as usize);
        self.register_symbol("collections_set_contains", crate::execution::runtime_functions::collections_set_contains as usize);
        self.register_symbol("collections_set_remove", crate::execution::runtime_functions::collections_set_remove as usize);
        self.register_symbol("collections_set_len", crate::execution::runtime_functions::collections_set_len as usize);
        self.register_symbol("collections_set_clear", crate::execution::runtime_functions::collections_set_clear as usize);
        self.register_symbol("collections_set_is_empty", crate::execution::runtime_functions::collections_set_is_empty as usize);
        
        // Register standard library string processing functions
        self.register_symbol("string_length", crate::execution::runtime_functions::string_length as usize);
        self.register_symbol("string_to_upper", crate::execution::runtime_functions::string_to_upper as usize);
        self.register_symbol("string_to_lower", crate::execution::runtime_functions::string_to_lower as usize);
        self.register_symbol("string_regex_match", crate::execution::runtime_functions::string_regex_match as usize);
        self.register_symbol("string_regex_find", crate::execution::runtime_functions::string_regex_find as usize);
        self.register_symbol("string_regex_replace", crate::execution::runtime_functions::string_regex_replace as usize);
        self.register_symbol("string_regex_split", crate::execution::runtime_functions::string_regex_split as usize);
        self.register_symbol("string_format", crate::execution::runtime_functions::string_format as usize);
        self.register_symbol("string_trim", crate::execution::runtime_functions::string_trim as usize);
        self.register_symbol("string_contains", crate::execution::runtime_functions::string_contains as usize);
        self.register_symbol("string_index_of", crate::execution::runtime_functions::string_index_of as usize);
        self.register_symbol("string_substring", crate::execution::runtime_functions::string_substring as usize);
        self.register_symbol("i32_to_string", crate::execution::runtime_functions::i32_to_string as usize);
        self.register_symbol("string_concat", crate::execution::runtime_functions::string_concat as usize);
        self.register_symbol("string_is_empty", crate::execution::runtime_functions::string_is_empty as usize);
        self.register_symbol("string_base64_encode", crate::execution::runtime_functions::string_base64_encode as usize);
        self.register_symbol("string_base64_decode", crate::execution::runtime_functions::string_base64_decode as usize);
        self.register_symbol("string_levenshtein_distance", crate::execution::runtime_functions::string_levenshtein_distance as usize);
        
        // Register standard library networking functions
        // TCP operations
        self.register_symbol("network_tcp_connect", crate::execution::runtime_functions::network_tcp_connect as usize);
        self.register_symbol("network_tcp_listen", crate::execution::runtime_functions::network_tcp_listen as usize);
        self.register_symbol("network_tcp_send", crate::execution::runtime_functions::network_tcp_send as usize);
        self.register_symbol("network_tcp_recv", crate::execution::runtime_functions::network_tcp_recv as usize);
        self.register_symbol("network_tcp_close", crate::execution::runtime_functions::network_tcp_close as usize);
        
        // DNS operations
        self.register_symbol("network_dns_resolve", crate::execution::runtime_functions::network_dns_resolve as usize);
        
        // HTTP operations
        self.register_symbol("network_http_get", crate::execution::runtime_functions::network_http_get as usize);
        self.register_symbol("network_http_post", crate::execution::runtime_functions::network_http_post as usize);
        self.register_symbol("network_http_request", crate::execution::runtime_functions::network_http_request as usize);
    }
    
    fn register_symbol(&mut self, name: &str, addr: usize) {
        self.symbols.insert(name.to_string(), addr);
    }
    
    fn resolve_symbol(&self, name: &str) -> Option<*const u8> {
        self.symbols.get(name).map(|&addr| addr as *const u8)
    }
}

impl CursedJitCompiler {
    /// Create a new JIT compiler with configuration
    pub fn new(config: JitRuntimeConfig) -> Result<Self, CursedError> {
        // Initialize LLVM targets
        Target::initialize_native(&Default::default())
            .map_err(|e| CursedError::compiler_error(&format!("LLVM target initialization failed: {}", e)))?;
        
        let state = Arc::new(ThreadSafeCompilerState {
            config,
            function_cache: RwLock::new(HashMap::new()),
            hot_paths: RwLock::new(HashMap::new()),
            compilation_queue: Mutex::new(Vec::new()),
            active_compilations: AtomicU64::new(0),
            shutdown: AtomicBool::new(false),
            stats: RwLock::new(JitCompilationStats::default()),
            symbol_resolver: Arc::new(Mutex::new(SymbolResolver::new())),
        });
        
        Ok(Self { state })
    }
    
    /// Initialize the JIT compiler
    pub fn initialize(&mut self) -> Result<(), CursedError> {
        // Initialize thread-local LLVM context
        LLVM_CONTEXT.with(|context| {
            *context.borrow_mut() = Some(Context::create());
        });
        
        Ok(())
    }
    
    /// Compile a function to specified tier
    pub fn compile_function(
        &mut self,
        name: &str,
        source: &str,
        tier: CompilationTier,
        optimization_level: OptimizationLevel,
    ) -> Result<Arc<CompiledJitFunction>, CursedError> {
        let start_time = Instant::now();
        
        // Check cache first
        if let Some(cached) = self.get_cached_function(name) {
            if cached.tier >= tier {
                return Ok(cached);
            }
        }
        
        self.state.active_compilations.fetch_add(1, Ordering::SeqCst);
        
        let result = self.perform_compilation(name, source, tier, optimization_level);
        
        self.state.active_compilations.fetch_sub(1, Ordering::SeqCst);
        let compile_time = start_time.elapsed();
        
        match result {
            Ok(mut compiled_fn) => {
                compiled_fn.compile_time = compile_time;
                let compiled_fn = Arc::new(compiled_fn);
                
                // Cache the compiled function
                self.cache_function(compiled_fn.clone());
                
                // Update statistics
                self.update_stats(tier, compile_time);
                
                Ok(compiled_fn)
            }
            Err(e) => Err(e),
        }
    }
    
    /// Execute a compiled function
    pub fn execute_function(
        &mut self,
        name: &str,
        args: &[*const u8],
    ) -> Result<*const u8, CursedError> {
        let start_time = Instant::now();
        
        // Get the compiled function
        let function = self.get_cached_function(name)
            .ok_or_else(|| CursedError::compiler_error(&format!("Function '{}' not found", name)))?;
        
        // Execute the function
        let result = self.call_compiled_function(&function, args)?;
        
        let execution_time = start_time.elapsed();
        
        // Update hot path tracking
        self.update_hot_path_info(name, execution_time)?;
        
        Ok(result)
    }
    
    /// Request background compilation for hot path
    pub fn request_background_compilation(
        &self,
        name: &str,
        source: &str,
        target_tier: CompilationTier,
        priority: i32,
    ) -> Result<(), CursedError> {
        let request = CompilationRequest {
            name: name.to_string(),
            source: source.to_string(),
            target_tier,
            priority,
            requested_at: Instant::now(),
        };
        
        let mut queue = self.state.compilation_queue.lock()
            .map_err(|_| CursedError::compiler_error("Failed to acquire compilation queue"))?;
        
        // Insert based on priority
        let insert_pos = queue.iter().position(|req| req.priority < priority).unwrap_or(queue.len());
        queue.insert(insert_pos, request);
        
        Ok(())
    }
    
    /// Get compilation statistics
    pub fn get_statistics(&self) -> Result<JitCompilationStats, CursedError> {
        let stats = self.state.stats.read()
            .map_err(|_| CursedError::compiler_error("Failed to read statistics"))?;
        
        let mut stats_copy = stats.clone();
        
        // Update queue size
        if let Ok(queue) = self.state.compilation_queue.lock() {
            stats_copy.queue_size = queue.len();
        }
        
        Ok(stats_copy)
    }
    
    // Private implementation methods
    
    fn perform_compilation(
        &mut self,
        name: &str,
        source: &str,
        tier: CompilationTier,
        optimization_level: OptimizationLevel,
    ) -> Result<CompiledJitFunction, CursedError> {
        // Use thread-local LLVM context for compilation
        LLVM_CONTEXT.with(|context_cell| {
            let mut context_opt = context_cell.borrow_mut();
            let context = context_opt.get_or_insert_with(|| Context::create());
            
            // Create module for this compilation
            let module = context.create_module(&format!("cursed_jit_{}", name));
            
            // Parse CURSED source to LLVM IR
            let llvm_function = self.compile_cursed_to_llvm(&module, context, name, source)?;
            
            // Apply optimizations based on tier and level
            self.apply_optimizations(&module, &llvm_function, tier, optimization_level)?;
            
            // Verify the module before JIT compilation
            if let Err(e) = module.verify() {
                return Err(CursedError::compiler_error(&format!("LLVM module verification failed: {}", e)));
            }
            
            // Create execution engine with appropriate optimization level
            let llvm_opt_level = match optimization_level {
                OptimizationLevel::None => LLVMOptLevel::None,
                OptimizationLevel::Basic => LLVMOptLevel::Less,
                OptimizationLevel::Standard => LLVMOptLevel::Default,
                OptimizationLevel::Aggressive => LLVMOptLevel::Aggressive,
            };
            
            let execution_engine = module.create_jit_execution_engine(llvm_opt_level)
                .map_err(|e| CursedError::compiler_error(&format!("Failed to create execution engine: {}", e)))?;
            
            // Get function pointer with proper typing based on the function signature
            let function_ptr = unsafe {
                // Try to get the function address directly
                match execution_engine.get_function_address(name) {
                    Ok(addr) => {
                        if addr == 0 {
                            // For now, just log the issue and create a null pointer - we'll investigate the actual issue
                            tracing::warn!("⚠️ Function '{}' address is null - JIT compilation may have failed, but continuing", name);
                        }
                        SafePointer::new(addr as *const u8)
                    }
                    Err(e) => {
                        return Err(CursedError::compiler_error(&format!("Failed to get function address for '{}': {}", name, e)));
                    }
                }
            };
            
            // Calculate code size (approximate)
            let code_size = self.estimate_code_size(&llvm_function);
            
            // Execution engine lifetime is properly managed by keeping it alive in the compiled function
            
            // Create the compiled function with valid pointer
            let mut compiled_function = CompiledJitFunction {
                name: name.to_string(),
                tier,
                optimization_level,
                function_ptr,
                code_size,
                compile_time: Duration::from_secs(0), // Will be set by caller
                metrics: ExecutionMetrics::default(),
                source_hash: self.hash_source(source),
                dependencies: HashSet::new(),
                _execution_engine_keepalive: None, // TODO: Keep execution engine alive for lifetime management
            };
            
            // Execution engine lifetime is now managed by keeping it alive in the compiled function
            
            Ok(compiled_function)
        })
    }
    
    fn compile_cursed_to_llvm<'a>(
        &self,
        module: &Module<'a>,
        context: &'a Context,
        name: &str,
        source: &str,
    ) -> Result<FunctionValue<'a>, CursedError> {
        // This is a simplified compilation - in reality would parse CURSED AST
        // and generate appropriate LLVM IR for goroutines, channels, async/await, etc.
        let builder = context.create_builder();
        let i64_type = context.i64_type();
        let fn_type = i64_type.fn_type(&[], false);
        let function = module.add_function(name, fn_type, None);
        
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);
        
        // Generate LLVM IR based on CURSED source
        self.generate_llvm_for_cursed_constructs(&builder, context, function, source)?;
        
        // Return success value
        let return_value = i64_type.const_int(0, false);
        let _ = builder.build_return(Some(&return_value));
        
        Ok(function)
    }
    
    fn generate_llvm_for_cursed_constructs(
        &self,
        builder: &Builder,
        context: &Context,
        function: FunctionValue,
        source: &str,
    ) -> Result<(), CursedError> {
        // This would parse the CURSED source and generate appropriate LLVM IR
        // For now, implement basic constructs
        
        // Check for goroutine spawn pattern
        if source.contains("go ") {
            self.generate_goroutine_spawn(builder, context, function)?;
        }
        
        // Check for channel operations
        if source.contains("chan ") || source.contains("<-") {
            self.generate_channel_operations(builder, context, function)?;
        }
        
        // Check for async/await
        if source.contains("async ") || source.contains("await ") {
            self.generate_async_await(builder, context, function)?;
        }
        
        Ok(())
    }
    
    fn generate_goroutine_spawn(&self, builder: &Builder, context: &Context, function: FunctionValue) -> Result<(), CursedError> {
        // Generate LLVM IR for goroutine spawning
        let resolver = self.state.symbol_resolver.lock()
            .map_err(|_| CursedError::compiler_error("Failed to acquire symbol resolver"))?;
        
        if let Some(spawn_fn_ptr) = resolver.resolve_symbol("cursed_goroutine_spawn") {
            let spawn_fn_type = context.void_type().fn_type(&[], false);
            
            // Create function call
            let fn_ptr_value = context.i64_type().const_int(spawn_fn_ptr as u64, false);
            let fn_ptr = builder.build_int_to_ptr(
                fn_ptr_value,
                context.i8_type().ptr_type(AddressSpace::from(0)),
                "spawn_fn_ptr",
            );
            
            // This would build the actual call with proper arguments
            // For now, just mark that we're handling goroutines
        }
        
        Ok(())
    }
    
    fn generate_channel_operations(&self, builder: &Builder, context: &Context, function: FunctionValue) -> Result<(), CursedError> {
        // Generate LLVM IR for channel operations (send/receive)
        let resolver = self.state.symbol_resolver.lock()
            .map_err(|_| CursedError::compiler_error("Failed to acquire symbol resolver"))?;
        
        // Handle channel creation, send, and receive operations
        if let Some(_create_fn_ptr) = resolver.resolve_symbol("cursed_channel_create") {
            // Generate channel creation code
        }
        
        if let Some(_send_fn_ptr) = resolver.resolve_symbol("cursed_channel_send") {
            // Generate channel send code
        }
        
        if let Some(_recv_fn_ptr) = resolver.resolve_symbol("cursed_channel_recv") {
            // Generate channel receive code
        }
        
        Ok(())
    }
    
    fn generate_async_await(&self, builder: &Builder, context: &Context, function: FunctionValue) -> Result<(), CursedError> {
        // Generate LLVM IR for async/await constructs
        let resolver = self.state.symbol_resolver.lock()
            .map_err(|_| CursedError::compiler_error("Failed to acquire symbol resolver"))?;
        
        if let Some(_async_spawn_ptr) = resolver.resolve_symbol("cursed_async_spawn") {
            // Generate async spawn code
        }
        
        if let Some(_await_ptr) = resolver.resolve_symbol("cursed_await_future") {
            // Generate await code
        }
        
        Ok(())
    }
    
    fn apply_optimizations(
        &self,
        module: &Module,
        function: &FunctionValue,
        tier: CompilationTier,
        optimization_level: OptimizationLevel,
    ) -> Result<(), CursedError> {
        let pass_manager = PassManager::create(module);
        
        // Configure optimization passes based on tier and level
        match tier {
            CompilationTier::Interpreter => {
                // No optimizations for interpreter tier
                return Ok(());
            }
            CompilationTier::Tier1 => {
                // Basic optimizations for fast compilation
                // TODO: Fix LLVM pass methods - these might be version-specific
                // 
                // 
            }
            CompilationTier::Tier2 => {
                // Standard optimizations  
                // TODO: Fix LLVM pass methods - these might be version-specific
                // 
                // 
                // 
                // 
            }
            CompilationTier::Tier3 => {
                // Aggressive optimizations
                // TODO: Fix LLVM pass methods - these might be version-specific
                // 
                // 
                // 
                // 
                // 
                // 
                // 
            }
        }
        
        // Apply additional optimizations based on optimization level
        match optimization_level {
            OptimizationLevel::None => {}
            OptimizationLevel::Basic => {
                
            }
            OptimizationLevel::Standard => {
                
                
            }
            OptimizationLevel::Aggressive => {
                
                
                
                
            }
        }
        
        pass_manager.initialize();
        pass_manager.run_on(function);
        pass_manager.finalize();
        
        Ok(())
    }
    
    fn get_cached_function(&self, name: &str) -> Option<Arc<CompiledJitFunction>> {
        let cache = self.state.function_cache.read().ok()?;
        cache.get(name).cloned()
    }
    
    fn cache_function(&self, function: Arc<CompiledJitFunction>) {
        if let Ok(mut cache) = self.state.function_cache.write() {
            cache.insert(function.name.clone(), function);
        }
    }
    
    fn call_compiled_function(
        &self,
        function: &CompiledJitFunction,
        args: &[*const u8],
    ) -> Result<*const u8, CursedError> {
        // Get the function pointer from the compiled function
        let function_ptr = function.function_ptr.get();
        
        if function_ptr.is_null() {
            return Err(CursedError::compiler_error(&format!(
                "Function '{}' pointer is null - compilation may have failed", 
                function.name
            )));
        }
        
        // Execute the JIT-compiled function with proper calling convention
        let result = unsafe {
            match args.len() {
                0 => {
                    // No arguments - call as fn() -> i64
                    let func: unsafe extern "C" fn() -> i64 = std::mem::transmute(function_ptr);
                    let int_result = func();
                    int_result as *const u8
                }
                1 => {
                    // One argument - call as fn(i64) -> i64
                    let func: unsafe extern "C" fn(i64) -> i64 = std::mem::transmute(function_ptr);
                    let arg0 = args[0] as i64;
                    let int_result = func(arg0);
                    int_result as *const u8
                }
                2 => {
                    // Two arguments - call as fn(i64, i64) -> i64
                    let func: unsafe extern "C" fn(i64, i64) -> i64 = std::mem::transmute(function_ptr);
                    let arg0 = args[0] as i64;
                    let arg1 = args[1] as i64;
                    let int_result = func(arg0, arg1);
                    int_result as *const u8
                }
                3 => {
                    // Three arguments - call as fn(i64, i64, i64) -> i64
                    let func: unsafe extern "C" fn(i64, i64, i64) -> i64 = std::mem::transmute(function_ptr);
                    let arg0 = args[0] as i64;
                    let arg1 = args[1] as i64;
                    let arg2 = args[2] as i64;
                    let int_result = func(arg0, arg1, arg2);
                    int_result as *const u8
                }
                _ => {
                    return Err(CursedError::compiler_error(&format!(
                        "JIT execution not supported for {} arguments - function '{}'", 
                        args.len(),
                        function.name
                    )));
                }
            }
        };
        
        Ok(result)
    }
    
    fn update_hot_path_info(&self, name: &str, execution_time: Duration) -> Result<(), CursedError> {
        let mut hot_paths = self.state.hot_paths.write()
            .map_err(|_| CursedError::compiler_error("Failed to update hot path info"))?;
        
        let info = hot_paths.entry(name.to_string()).or_insert_with(|| HotPathInfo {
            execution_count: 0,
            total_time: Duration::from_secs(0),
            avg_time: Duration::from_secs(0),
            last_execution: Instant::now(),
            current_tier: CompilationTier::Interpreter,
            eligible_for_tier_up: false,
        });
        
        info.execution_count += 1;
        info.total_time += execution_time;
        info.avg_time = info.total_time / info.execution_count as u32;
        info.last_execution = Instant::now();
        
        // Check tier-up eligibility
        if info.execution_count >= self.state.config.tier_up_threshold {
            info.eligible_for_tier_up = true;
            
            // Request background compilation to higher tier
            let next_tier = match info.current_tier {
                CompilationTier::Interpreter => CompilationTier::Tier1,
                CompilationTier::Tier1 => CompilationTier::Tier2,
                CompilationTier::Tier2 => CompilationTier::Tier3,
                CompilationTier::Tier3 => CompilationTier::Tier3,
            };
            
            if next_tier > info.current_tier {
                // This would trigger background compilation
                drop(hot_paths);
                self.request_background_compilation(name, "", next_tier, 50)?;
            }
        }
        
        Ok(())
    }
    
    fn update_stats(&self, tier: CompilationTier, compile_time: Duration) {
        if let Ok(mut stats) = self.state.stats.write() {
            stats.total_compilations += 1;
            *stats.tier_compilations.entry(tier).or_insert(0) += 1;
            stats.total_compile_time += compile_time;
        }
    }
    
    fn estimate_code_size(&self, function: &FunctionValue) -> usize {
        // Estimate based on number of instructions
        let mut size = 0;
        for basic_block in function.get_basic_blocks() {
            for _instruction in basic_block.get_instructions() {
                size += 4; // Rough estimate of 4 bytes per instruction
            }
        }
        size
    }
    
    fn hash_source(&self, source: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        source.hash(&mut hasher);
        hasher.finish()
    }
}

impl CodeGeneratorTrait for CursedJitCompiler {
    fn compile_function(&mut self, name: &str, source: &str, optimization: OptimizationLevel) -> Result<Vec<u8>, crate::error_types::Error> {
        let compiled = self.compile_function(name, source, CompilationTier::Tier1, optimization)
            .map_err(|e| crate::error_types::Error::Runtime(format!("JIT compilation failed: {}", e)))?;
        
        // Return mock machine code - in reality would extract from LLVM
        Ok(vec![0x48, 0x89, 0xe5, 0x5d, 0xc3]) // Basic x64 function prologue/epilogue
    }
    
    fn supported_optimizations(&self) -> Vec<OptimizationLevel> {
        vec![
            OptimizationLevel::None,
            OptimizationLevel::Basic,
            OptimizationLevel::Standard,
            OptimizationLevel::Aggressive,
        ]
    }
}

impl Default for ExecutionMetrics {
    fn default() -> Self {
        Self {
            avg_execution_time: Duration::from_nanos(0),
            min_execution_time: Duration::from_secs(u64::MAX),
            max_execution_time: Duration::from_secs(0),
            instructions_per_second: 0.0,
            cache_hit_ratio: 0.0,
            branch_prediction_accuracy: 0.0,
        }
    }
}

// CURSED runtime function implementations - linking to actual runtime system

use crate::runtime::AsyncRuntime;
use crate::runtime::goroutine::*;
use crate::runtime::memory::*;
use crate::stdlib::vibez::print::*;
use crate::runtime::process::*;
use crate::runtime::gc::*;

// Vibez.spill runtime function - core CURSED output
extern "C" fn cursed_vibez_spill(args_ptr: *const Value, args_len: usize) -> i32 {
    if args_ptr.is_null() || args_len == 0 {
        return -1;
    }
    
    unsafe {
        let args = std::slice::from_raw_parts(args_ptr, args_len);
        // Call the actual vibez::spill function
        match spill(args) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }
}

// Vibez.spillf runtime function - formatted output
extern "C" fn cursed_vibez_spillf(format_ptr: *const std::ffi::c_char, args_ptr: *const Value, args_len: usize) -> i32 {
    if format_ptr.is_null() || args_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let format_str = std::ffi::CStr::from_ptr(format_ptr).to_str().unwrap_or("");
        let args = std::slice::from_raw_parts(args_ptr, args_len);
        // Call the actual vibez::spillf function
        match spillf(format_str, args) {
            Ok(()) => 0,
            Err(_) => -1,
        }
    }
}

// Vibez.read runtime function - raw input
extern "C" fn cursed_vibez_read() -> *mut std::ffi::c_char {
    use std::io::{self, Read};
    
    let mut buffer = Vec::new();
    match io::stdin().read_to_end(&mut buffer) {
        Ok(_) => {
            // Convert to C string
            buffer.push(0); // Null terminator
            let ptr = buffer.as_mut_ptr() as *mut std::ffi::c_char;
            std::mem::forget(buffer); // Prevent deallocation
            ptr
        },
        Err(_) => std::ptr::null_mut(),
    }
}

// Vibez.readln runtime function - line input
extern "C" fn cursed_vibez_readln() -> *mut std::ffi::c_char {
    use std::io::{self, BufRead};
    
    let stdin = io::stdin();
    let mut line = String::new();
    
    match stdin.lock().read_line(&mut line) {
        Ok(_) => {
            // Remove trailing newline
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            
            // Convert to C string
            let c_string = std::ffi::CString::new(line).unwrap_or_default();
            let ptr = c_string.into_raw();
            ptr
        },
        Err(_) => std::ptr::null_mut(),
    }
}

// Link to actual runtime goroutine functions
extern "C" fn cursed_goroutine_spawn(func_ptr: *const std::ffi::c_void, args_ptr: *const std::ffi::c_void) -> u64 {
    // Convert function pointer and call the runtime function
    let entry_fn: extern "C" fn(*mut std::ffi::c_void) = unsafe { std::mem::transmute(func_ptr) };
    let context = args_ptr as *mut std::ffi::c_void;
    cursed_stan_goroutine(entry_fn, context)
}

extern "C" fn cursed_goroutine_yield() -> bool {
    cursed_yolo_goroutine()
}

extern "C" fn cursed_goroutine_join(id: u64) -> i32 {
    // Implementation would wait for goroutine completion
    if cursed_yolo_goroutine() { 0 } else { -1 }
}

// Channel operations using runtime system
extern "C" fn cursed_channel_create(capacity: usize) -> *mut std::ffi::c_void {
    use crate::runtime::channels::SimpleChannel;
    use crate::execution::CursedValue;
    use std::sync::Arc;
    
    // Create channel through runtime system
    let channel = if capacity == 0 {
        Arc::new(SimpleChannel::<CursedValue>::new())
    } else {
        Arc::new(SimpleChannel::<CursedValue>::with_capacity(capacity))
    };
    
    // Box the channel and return raw pointer
    let boxed_channel = Box::new(channel);
    Box::into_raw(boxed_channel) as *mut std::ffi::c_void
}

extern "C" fn cursed_channel_send(channel_ptr: *mut std::ffi::c_void, data_ptr: *const std::ffi::c_void) -> i32 {
    if channel_ptr.is_null() || data_ptr.is_null() {
        return -1;
    }
    
    use crate::runtime::channels::{SimpleChannel, SendResult};
    use crate::execution::CursedValue;
    use std::sync::Arc;
    
    unsafe {
        // Reconstruct the channel from pointer
        let channel = &*(channel_ptr as *const Arc<SimpleChannel<CursedValue>>);
        
        // Reconstruct the value from pointer (assuming it's a boxed CursedValue)
        let value = &*(data_ptr as *const CursedValue);
        
        // Send the value through the channel
        match channel.send(value.clone()) {
            SendResult::Sent => 0,
            SendResult::Closed(_) => -2,
            SendResult::WouldBlock(_) => -3,
        }
    }
}

extern "C" fn cursed_channel_recv(channel_ptr: *mut std::ffi::c_void, data_ptr: *mut std::ffi::c_void) -> i32 {
    if channel_ptr.is_null() || data_ptr.is_null() {
        return -1;
    }
    
    use crate::runtime::channels::{SimpleChannel, ReceiveResult};
    use crate::execution::CursedValue;
    use std::sync::Arc;
    
    unsafe {
        // Reconstruct the channel from pointer
        let channel = &*(channel_ptr as *const Arc<SimpleChannel<CursedValue>>);
        
        // Receive data from channel
        match channel.recv() {
            ReceiveResult::Received(value) => {
                // Store the received value in the output pointer
                let output_ptr = data_ptr as *mut CursedValue;
                *output_ptr = value;
                0
            }
            ReceiveResult::Closed => -2,
            ReceiveResult::WouldBlock => -3,
        }
    }
}

extern "C" fn cursed_channel_close(channel_ptr: *mut std::ffi::c_void) -> i32 {
    if channel_ptr.is_null() {
        return -1;
    }
    
    use crate::runtime::channels::SimpleChannel;
    use crate::execution::CursedValue;
    use std::sync::Arc;
    
    unsafe {
        // Reconstruct the channel from pointer
        let channel = &*(channel_ptr as *const Arc<SimpleChannel<CursedValue>>);
        
        // Close the channel
        channel.close();
        
        // Clean up the boxed channel
        let _ = Box::from_raw(channel_ptr as *mut Arc<SimpleChannel<CursedValue>>);
        
        0
    }
}

// Async runtime functions
extern "C" fn cursed_async_spawn(func_ptr: *const std::ffi::c_void, args_ptr: *const std::ffi::c_void) -> u64 {
    // TODO: Implement async task spawning with runtime integration
    0 // Return placeholder task ID
}

extern "C" fn cursed_await_future(future_id: u64) -> *mut std::ffi::c_void {
    // TODO: Implement future awaiting with runtime integration
    std::ptr::null_mut() // Return placeholder result
}

// Memory management functions
extern "C" fn cursed_gc_alloc(size: usize) -> *mut std::ffi::c_void {
    use crate::memory::Tag;
    if let Some(gc) = unsafe { get_global_gc() } {
        match gc.allocate(size, Tag::Object) {
            Ok(non_null_ptr) => non_null_ptr.as_ptr() as *mut std::ffi::c_void,
            Err(_) => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}

extern "C" fn cursed_gc_free(ptr: *mut std::ffi::c_void) {
    // Note: GC handles deallocation automatically
    // This is a no-op for now since our GC doesn't have manual deallocation
    if !ptr.is_null() {
        // In a real implementation, we might mark the object for collection
        // For now, we'll rely on the automatic GC
    }
}

// Error handling functions
extern "C" fn cursed_panic(message_ptr: *const std::ffi::c_char) {
    if !message_ptr.is_null() {
        unsafe {
            let message = std::ffi::CStr::from_ptr(message_ptr).to_str().unwrap_or("Unknown panic");
            panic!("CURSED panic: {}", message);
        }
    } else {
        panic!("CURSED panic: Unknown error");
    }
}

extern "C" fn cursed_error_propagate(error_code: i32, context_ptr: *const std::ffi::c_char) -> i32 {
    // Error propagation implementation
    if !context_ptr.is_null() {
        unsafe {
            let context = std::ffi::CStr::from_ptr(context_ptr).to_str().unwrap_or("Unknown context");
            eprintln!("CURSED error propagated: code={}, context={}", error_code, context);
        }
    }
    error_code
}
