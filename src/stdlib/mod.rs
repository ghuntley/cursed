/// Standard library for CURSED
pub mod core;
pub mod dot_registry;
pub mod packages;
pub mod web_vibez;
pub mod http_core;
pub mod database;
pub mod crypto;
pub mod crypto_pqc;
pub mod template;
pub mod errors_simple;
pub use errors_simple as errors;
pub mod value;
pub mod exec_slay;
pub mod fs;
pub mod io;
pub mod string;
pub mod math;
pub mod time;
pub mod packrat;
pub mod collections;
pub mod env;
pub mod testing;
pub mod process;
pub mod sync;
pub mod atomic_drip;
pub mod ipc;
pub mod net;
pub mod vibe_net;
pub mod vibecheck;
pub mod system;
pub mod sys_core;
pub mod profiler;
pub mod r#async;
pub mod test_vibes;
pub mod signal_boost;
pub mod exec_vibez;
pub mod json_tea;
pub mod oglogging;
pub mod plug_vibes;
pub mod csv;
pub mod regex_vibez;
pub mod chaos_mode;
pub mod embed_that;
pub mod vibez;
pub mod glyph_gang;
pub mod glowup_http;
pub mod no_cap;
pub mod bytefit;
pub mod lookin_glass;
pub mod squish_core;

// ================================
// GEN Z STDLIB MODULES (CURSED NAMING CONVENTIONS)
// ================================

/// StringZ - Tea manipulation functions with Gen Z flair
pub mod stringz;

/// MathZ - Mathematical functions with CURSED types and Gen Z naming  
pub mod mathz;

/// ConcurrenZ - Synchronization primitives with Gen Z flair
pub mod concurrenz;

/// DropZ - Basic I/O primitives with Gen Z naming
pub mod dropz;

/// VibeLife - OS functionality with Gen Z flair
pub mod vibe_life;

// Database package re-exports for easy access
pub use database::llvm_integration::{
    DatabaseLLVMIntegration, DatabaseLLVMIntegrationImpl, 
    register_database_functions
};

// Crypto package re-exports for easy access
// TODO: Re-enable when crypto packages are fully implemented
// pub use crypto::{
//     CryptoPlatform, JwtHandler, HmacAuth, TotpGenerator, TlsHandshake,
//     SecureRandom, UuidV4Generator, SaltGenerator, NonceGenerator,
//     Base64Encoder, HexEncoder, Base32Encoder, Asn1Parser,
//     CryptoLLVMIntegration, CryptoLLVMIntegrationImpl, register_crypto_functions
// };

// Package re-exports
pub use packages::{
    db_core, db_pool, sql_vibes, db_migrate, db_orm, db_nosql, db_query
};

// Template system re-exports
pub use template::{
    TemplateEngine, TemplateContext, TemplateConfig, TemplateLoader,
    TemplateRenderer, FilterRegistry, TemplateCache, WebTemplateRenderer,
    TemplateFormat, TemplateFormatRenderer, HtmlTemplateContext, HtmlEscaper
};

// File system re-exports
pub use fs::{
    FsError, FsResult, FileMetadata, DirEntry,
    read_file, write_file, append_file, delete_file, copy_file, move_file,
    create_dir, create_dir_all, remove_dir, remove_dir_all, list_dir,
    exists, is_file, is_dir, file_size, metadata,
    join_path, parent_dir, file_name, extension, absolute_path,
    read_text_file_safe, write_text_file_safe
};

// Console I/O re-exports
pub use io::{
    IoError, IoResult, system_error, io_error, invalid_input,
    Stdin, Stdout, Stderr, stdin, stdout, stderr, flush_all,
    print, println, eprint, eprintln, printf, printfln, eprintf, eprintfln,
    read_line, read_char, read_until, read_all, flush,
    prompt, confirm, select, multi_select, read_password, paginate, ProgressBar,
    BufferedReader, BufferedWriter, SharedBufferedReader, SharedBufferedWriter,
    buffered_stdin, buffered_stdout, buffered_stderr,
    shared_buffered_stdin, shared_buffered_stdout, shared_buffered_stderr
};

// Process execution re-exports - ExecSlay module for command execution with style
pub use exec_slay::{
    // Error handling and core types
    SlayResult, SlayOptions, SignalOptions, ProcessStats, SharedProcessState,
    
    // Core command execution
    SlayCommand, SlayProcess, SlayProcessState,
    
    // Pipeline execution
    SlayPipeline,
    
    // Background task management
    SlayTask, SlayTaskManager, TaskStatus, run_background,
    
    // Command builder
    SlayCommandBuilder, slay_command,
    
    // Shell command execution
    run_shell, shell_output, run_shell_with_env, run_shell_in_dir,
    run_shell_with_env_and_dir, shell_output_with_env, shell_output_in_dir,
    shell_combined_output, ShellCommandBuilder,
    
    // Process monitoring
    ProcessMonitor, MonitorConfig, MonitoringStats, ResourceLimiter,
    LimitType, LimitViolation,
    
    // Shell utilities
    shell::utils::command_exists, shell::utils::pwd, shell::utils::ls,
    shell::utils::cd, shell::utils::mkdir, shell::utils::rm,
    shell::utils::cp, shell::utils::mv, shell::utils::get_env as shell_get_env,
    shell::utils::set_env as shell_set_env,
};

// String manipulation re-exports
pub use string::{
    StringError, StringResult,
    // Core operations
    length, is_empty, concat, concat_owned, repeat, reverse, char_at, chars, bytes, is_ascii,
    from_utf8, from_utf8_lossy,
    // Search and replace
    contains, starts_with, ends_with, find, find_last, find_all, replace, replace_first,
    replace_last, replace_n, count_occurrences, contains_ignore_case, find_ignore_case,
    // Transformations
    substring, substring_range, trim, trim_start, trim_end, trim_chars, trim_start_chars,
    trim_end_chars, to_lowercase, to_uppercase, to_title_case, to_camel_case, to_pascal_case,
    to_snake_case, to_kebab_case, capitalize, insert_at, remove_range,
    // Splitting and joining
    split, split_n, rsplit, rsplit_n, split_lines, split_whitespace, split_any, split_by,
    join, join_owned, join_with_separators, partition, rpartition, chunk, split_into_n_parts,
    // Validation
    is_numeric, is_integer, is_alphabetic, is_alphanumeric, is_whitespace, is_uppercase,
    is_lowercase, is_title_case, is_hex, is_email, is_url, is_phone_number,
    has_balanced_parentheses, has_balanced_brackets, is_palindrome,
    // Formatting
    pad_left, pad_right, center, truncate, wrap_text, format_columns, auto_detect_column_widths,
    format_table, add_line_numbers, indent_lines, dedent, escape_html, escape_json, escape_csv
};

// Mathematics re-exports - Comprehensive mathematical library
pub use math::{
    // Error handling system
    MathError, MathResult, domain_error, range_error, division_by_zero_error, negative_input_error,
    is_valid_float, validate_float,
    
    // BASIC OPERATIONS - Fundamental arithmetic and utilities
    abs, min, max, clamp, sign, 
    floor, ceil, round, math_truncate, fract, remainder, modulo,
    gcd, lcm, is_even, is_odd, lerp, inverse_lerp, smooth_step, smoother_step,
    abs_i32, abs_i64, min_i32, max_i32, clamp_i32, min_i64, max_i64, clamp_i64,
    pow2, pow10, reciprocal, is_zero, is_equal, round_to_decimals, map_range,
    average, basic_geometric_mean, basic_harmonic_mean,
    
    // TRIGONOMETRIC FUNCTIONS - Complete trigonometric operations
    sin, cos, tan, asin, acos, atan, atan2,
    sinh, cosh, tanh, asinh, acosh, atanh,
    degrees_to_radians, radians_to_degrees, deg_to_rad, rad_to_deg,
    sin_deg, cos_deg, tan_deg, sec, csc, cot,
    normalize_angle, normalize_angle_signed,
    
    // LOGARITHMIC & EXPONENTIAL FUNCTIONS - Advanced mathematical operations
    ln, log10, log2, log, expm1, ln1p,
    exp, exp2, exp10, exp2m1, exp10m1, exp_base,
    pow, powi, pow_e, pow_2, pow_10, tetration,
    sqrt, cbrt, nth_root, hypot, hypot3,
    square, cube, mul_add, inv_sqrt, ln_gamma,
    log2_abs, log10_abs, ln_abs, log_mean, sigmoid, logistic,
    softmax_single, log_sum_exp,
    is_valid_log_input, is_valid_exp_input, safe_ln, safe_exp,
    clamped_ln, clamped_exp,
    
    // MATHEMATICAL CONSTANTS - Fundamental mathematical values
    PI, TAU, E, PHI, INV_PHI, EULER_GAMMA,
    FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8,
    FRAC_1_PI, FRAC_2_PI, FRAC_2_SQRT_PI,
    SQRT_2, FRAC_1_SQRT_2, SQRT_3, SQRT_5, SQRT_PI,
    LN_2, LN_10, LOG2_E, LOG2_10, LOG10_E, LOG10_2,
    DEG_TO_RAD, RAD_TO_DEG,
    EPSILON, MIN_POSITIVE, MAX, MIN, INFINITY, NEG_INFINITY, NAN,
    
    // RANDOM NUMBER GENERATION - Comprehensive random utilities
    random, random_range, random_int, random_u64, random_bool,
    choice, choices, weighted_choice, shuffle, shuffled, sample,
    random_bytes, random_string, random_alphanumeric, random_hex,
    set_seed,
    random_normal, random_exponential, random_uniform, random_poisson,
    random_beta, random_gamma,
    
    // STATISTICAL FUNCTIONS - Data analysis and statistics
    mean, median, mode, variance, sample_variance, standard_deviation, sample_standard_deviation,
    skewness, kurtosis, harmonic_mean, geometric_mean, root_mean_square, coefficient_of_variation,
    percentile, q1, q3, five_number_summary, range, interquartile_range,
    normal_pdf, standard_normal_cdf, normal_cdf, uniform_pdf, uniform_cdf,
    covariance, sample_covariance, correlation,
    outliers_iqr, outliers_z_score, has_invalid_values, clean_data, validate_dataset,
    
    // SPECIAL FUNCTIONS - Advanced mathematical functions
    special_factorial, factorial_f64, gamma, beta, 
    binomial, binomial_f64, special_permutations,
    special_fibonacci, lucas, catalan,
    erf, erfc, erf_inv,
    bessel_j0, bessel_j1, bessel_y0, bessel_y1,
    
    // MATHEMATICAL UTILITIES - Advanced computational mathematics
    extended_gcd, is_prime, sieve_of_eratosthenes, prime_factorization, next_prime, euler_totient,
    factorial, double_factorial, factorial_stirling, permutations, combinations, 
    binomial_coefficient, multicombinations, catalan_number,
    gamma_function, beta_function, error_function, complementary_error_function,
    simpson_integration, numerical_derivative, newton_raphson, bisection_method,
    fibonacci, lucas_number, tribonacci, factorial_sequence_sum, harmonic_number,
    mod_pow, mod_inverse, convert_base, gcd_multiple, lcm_multiple,
    FibonacciMemo, is_perfect_number, digital_root,
};

// Time and date handling re-exports
pub use time::{
    TimeError, TimeResult, time_error, parse_error, invalid_date_error, timezone_error,
    // Core date/time structures
    DateTime, Date, Time, Instant, Weekday, Month,
    now, utc_now, today, tomorrow, yesterday,
    from_timestamp, from_timestamp_millis, from_timestamp_nanos,
    days_in_month, is_leap_year, day_of_year, week_of_year,
    // Duration and time arithmetic
    Duration, 
    seconds, minutes, hours, days, weeks,
    milliseconds, microseconds, nanoseconds,
    duration_between, time_until, time_since,
    // Formatting and parsing
    DateTimeFormat, format_datetime, parse_datetime,
    format_date, parse_date, format_time, parse_time,
    format_duration, parse_duration, format_iso8601, parse_iso8601,
    format_rfc3339, parse_rfc3339, format_custom, parse_custom,
    // Timezone support
    Timezone, UtcOffset, 
    utc, local_timezone, timezone_by_name, timezone_by_offset,
    convert_timezone, get_timezone_offset, list_timezones,
    // Relative time calculations
    relative_time, time_ago, time_from_now,
    humanize_duration, format_relative, parse_relative,
    next_occurrence, previous_occurrence,
    // Sleep and timing utilities
    sleep, sleep_millis, sleep_micros, sleep_nanos,
    sleep_until, timeout, delay, Timer, Stopwatch,
    // Benchmarking and performance measurement
    Benchmark, benchmark, time_it, measure_time,
    BenchmarkResult, compare_benchmarks, benchmark_multiple,
    PerformanceCounter, system_time_nanos
};

// Collections re-exports - Comprehensive data structure library
pub use collections::{
    // Error handling system
    CollectionsError, CollectionsResult,
    
    // Set types - Unique element collections
    HashSet, TreeSet, BitSet, BitSetIterator,
    
    // Queue types - FIFO and priority-based collections
    Queue, Deque, PriorityQueue, CircularQueue,
    
    // Stack types - LIFO collections with various specializations
    Stack, FixedStack, ThreadSafeStack, StackWithMin,
    
    // Simple Iterator System
    SimpleIterator, SimpleIntoIterator, VecIterator, RangeIterator,
    
    // Iterator Adapters
    MapIterator, FilterIterator, TakeIterator, SkipIterator,
    
    // Iterator Utilities
    SimpleIteratorUtils,
    
    // Utility Functions
    simple_range, simple_range_step,
    
    // Convenience functions
    hash_set_from_vec, tree_set_from_vec, bit_set_from_vec,
    hash_set_union_multiple, hash_set_intersection_multiple,
};

// Environment variables re-exports - Comprehensive environment handling
pub use env::{
    // Error handling system
    EnvError, EnvResult, env_error, not_found_error, invalid_value_error, permission_error,
    
    // Core environment operations
    get_env, set_env, remove_env, get_all_env, env_exists,
    get_env_with_default, clear_all_env, get_env_keys, get_env_values,
    get_current_dir, get_home_dir, get_temp_dir, get_username, get_hostname,
    
    // Platform utilities
    get_path_separator, is_case_sensitive_env, get_env_case_insensitive,
    
    // Type parsing and conversion
    parse_env, parse_env_with_default, get_path_env, get_numeric_env,
    get_bool_env, get_float_env, get_int_env, parse_env_list,
    parse_env_colon_list, parse_env_semicolon_list, parse_env_path_list,
    parse_env_config, parse_env_duration, parse_env_memory_size,
    
    // Environment variable expansion
    expand_env_vars, expand_env_vars_with_defaults, has_env_vars,
    validate_env_syntax, extract_env_vars, substitute_env_vars,
    escape_env_value, unescape_env_value,
};

// Testing framework re-exports - Comprehensive unit testing
pub use testing::{
    // Core testing infrastructure
    TestFramework, TestFrameworkConfig, TestExecutionMode, TestFilterMode,
    
    // Test discovery and execution
    TestDiscovery, TestFilter, DiscoveryConfig, TestInfo, TestMetadata,
    TestExecutor, TestExecutorConfig, TestResult, TestStatus, TestFailure,
    ExecutionContext, TestTimeout, ParallelExecutor,
    
    // Test runner and reporting
    TestRunner, TestRunnerConfig, RunnerResult, TestSuite, TestSuiteResult,
    TestReporter, ReportFormat, ReportConfig, TestReport, SummaryReport,
    ConsoleReporter, JsonReporter, XmlReporter, HtmlReporter,
    
    // Statistics and performance measurement
    TestStatistics, TestTiming, PerformanceStats, TestMetrics,
    ExecutionStats, MemoryStats, TestBenchmark,
    
    // Assertion framework - All assertion functions
    assert_true, assert_false, assert_eq, assert_ne, assert_null, assert_not_null,
    assert_greater, assert_greater_equal, assert_less, assert_less_equal,
    assert_close_to, assert_between, assert_positive, assert_negative, assert_zero,
    assert_contains, assert_not_contains, assert_starts_with, assert_ends_with,
    assert_matches_regex, assert_empty_string, assert_length,
    assert_empty, assert_not_empty, assert_contains_element, assert_not_contains_element,
    assert_has_length, assert_all_true, assert_any_true, assert_none_true,
    assert_error, assert_no_error, assert_error_type, assert_error_message,
    assert_panic, assert_no_panic,
    assert_eventually, assert_within_timeout, assert_file_exists, assert_file_content,
    AssertionResult, AssertionError, AssertionContext,
    
    // Test attributes and metadata
    TestAttribute, TestAttributes, TestIgnore, TestExpectedPanic,
    parse_test_attributes, validate_test_attributes,
    
    // Test macros and code generation
    test_function, ignore_test, should_panic_test, timeout_test,
    setup_function, teardown_function, test_suite_macro,
    
    // Error handling for testing framework
    TestError, TestFrameworkResult,
    discovery_error, execution_error, assertion_error, timeout_error,
    config_error, report_error, framework_error
};

// Process management re-exports - Comprehensive system process control
pub use process::{
    // Error handling system
    ProcessError, ProcessResult, process_not_found, permission_denied, invalid_state,
    execution_failed, timeout_error as process_timeout_error, invalid_arguments, environment_error,
    communication_error, system_error as process_system_error,
    
    // Core process management
    ProcessConfig, ProcessIo, ProcessOutput, Process, spawn_process, run_command,
    run_command_timeout, command_exists, which,
    
    // Process information and system utilities
    ProcessInfo, ProcessStatus, MemoryInfo, CpuInfo, ProcessListEntry,
    get_current_pid, get_parent_pid, is_process_running, get_process_info,
    get_process_memory, get_process_cpu, get_process_list, find_processes_by_name,
    get_process_tree, get_load_average, get_cpu_count, get_system_uptime,
    
    // Process control and signal handling
    Signal, Priority, ProcessControl, send_signal_to_pid, kill_process, terminate_process,
    kill_process_graceful, set_process_priority, get_process_priority, wait_for_process,
    stop_process, continue_process, kill_processes_by_name, terminate_processes_by_name,
    kill_process_tree, terminate_process_tree, setup_signal_handler, ignore_signal,
    reset_signal_handler,
    
    // Process communication and IPC
    ProcessChannels, ProcessCommunication,
    IpcType, CommunicationConfig, create_process_communication, create_pipe,
    execute_with_communication, send_and_receive, create_daemon, monitor_process_output,
    
    // Process monitoring and health checks
    HealthStatus, ResourceThresholds, HealthCheckConfig, PerformanceMetrics,
    PerformanceHistory, ProcessMonitor as IpcProcessMonitor, MonitoredProcess, ProcessWatchdog,
    collect_performance_metrics, create_process_monitor, monitor_process_once,
    get_system_resource_summary,
    
    // Platform-specific utilities
    PlatformUtils, PlatformProcessInfo, PlatformFeature, UserInfo, FileDescriptorInfo,
    ResourceLimits, ResourceType, get_platform_name, supports_feature,
};

// Threading and synchronization re-exports - Comprehensive concurrency support
pub use sync::{
    // Error handling system
    SyncError, SyncResult, thread_error, lock_error, timeout_error as sync_timeout_error, deadlock_error,
    
    // Core threading primitives
    Thread, ThreadId, ThreadBuilder, JoinHandle,
    spawn, spawn_named, current_thread_id, current_thread_name, 
    sleep as thread_sleep, yield_now, park, unpark,
    
    // Synchronization primitives
    Mutex, RwLock, Semaphore, Barrier, CondVar,
    MutexGuard, RwLockReadGuard, RwLockWriteGuard,
    
    // Atomic operations
    AtomicBool, AtomicI32, AtomicI64, AtomicUsize, AtomicPtr,
    Ordering, memory_fence, compiler_fence,
    
    // Once and lazy initialization
    Once, OnceCell, Lazy,
    
    // Concurrent collections
    ConcurrentHashMap, ConcurrentVec, ConcurrentQueue, ConcurrentStack,
    ChannelSender, ChannelReceiver, channel, bounded_channel, unbounded_channel,
    select_channel, try_select_channel, ChannelError,
    LockFreeStack, LockFreeQueue, AtomicCounter,
    
    // Parallel processing
    ThreadPool, ThreadPoolBuilder, ThreadPoolConfig,
    WorkStealingPool, TaskQueue, Task, TaskResult,
    ParallelIterator, par_map, par_filter, par_reduce, par_for_each,
    RayonCompat, parallel_sort, parallel_search,
    SchedulerPolicy, LoadBalancer,
    
    // Thread-local storage
    ThreadLocal, ThreadLocalKey, ThreadLocalCell, ThreadLocalValue,
    with_thread_local, thread_local_get, thread_local_set, thread_local_remove,
    TlsKey, create_thread_local_key,
    cleanup_current_thread, cleanup_thread_local_storage, get_thread_local_statistics,
    
    // Module management
    init_sync_module, cleanup_sync_module, get_sync_statistics,
    SyncStatistics, LockContentionStats, ChannelStatistics, ThreadLocalStatistics,
    
    // Global thread pool management
    init_global_thread_pool, shutdown_global_thread_pool, get_thread_pool_utilization,
};

// Atomic operations re-exports - Low-level atomic memory operations for goroutine synchronization
pub use atomic_drip::{
    // Error handling system
    AtomicError, AtomicResult, atomic_error, concurrent_modification_error, alignment_error,
    
    // Core atomic types
    Int32, Int64, Uint32, Uint64, Bool, Float32, Float64, String, Pointer,
    
    // Generic atomic value container
    Value, AtomicString, AtomicVec, AtomicHashMap,
    
    // Memory ordering types and constants
    MemoryOrder, MEMORY_ORDER_RELAXED, MEMORY_ORDER_ACQUIRE, MEMORY_ORDER_RELEASE,
    MEMORY_ORDER_ACQUIRE_RELEASE, MEMORY_ORDER_SEQUENTIALLY_CONSISTENT,
    
    // Memory fence operations
    fence::memory_fence as atomic_memory_fence, fence::compiler_fence as atomic_compiler_fence, fence::full_barrier, 
    fence::acquire_barrier, fence::release_barrier,
    
    // Module initialization
    init as init_atomic_drip,
};

// Inter-Process Communication re-exports - Comprehensive IPC support
pub use ipc::{
    // Error handling system
    IpcError, IpcResult,
    named_pipe_error, message_queue_error, shared_memory_error, semaphore_error, unix_socket_error,
    permission_denied as ipc_permission_denied, already_exists, not_found, timeout_error as ipc_timeout_error,
    system_error as ipc_system_error, platform_error as ipc_platform_error,
    
    // Configuration
    IpcConfig, initialize_ipc, cleanup_ipc,
    
    // Named Pipes
    NamedPipe, NamedPipeServer, NamedPipeClient, NamedPipeConfig,
    ProcessStdin, ProcessStdout, ProcessStderr,
    
    // Message Queues
    MessageQueue, Message, MessageQueueConfig, MessageQueueStats,
    
    // Shared Memory
    SharedMemory, SharedMemorySegment, SharedMemoryConfig, SharedMemoryStats,
    
    // Semaphores
    Semaphore as IpcSemaphore, NamedSemaphore, SemaphoreConfig, SemaphoreValue,
    
    // Unix Domain Sockets
    UnixSocket, UnixSocketServer, UnixSocketClient, UnixSocketConfig, UnixSocketType,
    UnixDatagramServer, cleanup_sockets,
};

// Networking re-exports - Comprehensive networking and protocol support
pub use net::{
    // Error handling system
    NetError, NetResult, connection_error, timeout_error as net_timeout_error, dns_error, protocol_error,
    
    // Core networking types
    IpAddr, IpAddrV4, IpAddrV6, SocketAddr, SocketAddrV4, SocketAddrV6,
    TcpSocket, UdpSocket, TcpListener, SocketConfig, SocketOptions,
    SocketType, SocketState, ProtocolType,
    
    // DNS operations
    DnsResolver, DnsRecord, DnsRecordType, DnsQuery, DnsResponse,
    resolve_hostname, resolve_ip, lookup_mx, lookup_txt, lookup_cname,
    
    // Network interface utilities
    NetworkInterface, InterfaceType, InterfaceStats, InterfaceConfig,
    list_interfaces, get_interface_by_name, get_default_interface,
    
    // HTTP client functionality
    HttpClient, HttpRequest, HttpResponse, HttpHeaders, HttpMethod, StatusCode,
    RequestBuilder, ConnectionPool, Cookie, HttpAuth, HttpConfig,
    
    // WebSocket functionality
    WebSocketClient, WebSocketServer, WebSocketMessage, WebSocketFrame,
    MessageType, CloseCode, WebSocketConfig, CompressionConfig,
    
    // Protocol implementations
    SmtpClient, FtpClient, SshClient, TlsConfig,
    EmailMessage, FtpTransferMode, SshCommand, SshKey,
    
    // Utility functions
    is_port_available, scan_ports, ping_host, trace_route,
    get_public_ip, get_local_ips, validate_email, validate_url,
    parse_url, format_bandwidth, network_diagnostics,
    
    // Initialization functions
    initialize as initialize_net, shutdown as shutdown_net, get_network_statistics,
    NetworkStatistics,
};

// Runtime introspection (vibecheck) re-exports - Comprehensive runtime control
pub use vibecheck::{
    // Memory statistics and management
    MemStats, read_mem_stats, update_allocation_stats, memory_profile, write_profile, free_os_memory,
    MemoryProfile, GcOverhead,
    
    // Garbage collection control
    run_gc, set_gc_percent, get_gc_percent, is_gc_enabled, get_gc_stats, configure_gc,
    GcStats, GcConfig, jit_stats, set_jit_opt_level, get_metrics, set_finalizer, keep_alive,
    cpu_profile, JitStats, RuntimeMetrics, CpuProfile,
    
    // Goroutine management
    num_goroutine, go_id, stack, num_cpu, gomaxprocs, get_all_goroutine_info, block_profile,
    goroutine_info, coordinate_gc, get_stack_bounds, set_current_goroutine_id, clear_current_goroutine_id,
    GoroutineInfo, init_scheduler,
    
    // Version and runtime information
    version, compiler, goarch, goos, caller, func_for_pc, build_info, runtime_features,
    memory_layout, StackFrame, FuncInfo, BuildInfo, RuntimeFeatures, MemoryLayout,
    
    // Runtime configuration and hooks
    start_time, update_alloc_stats, update_gc_stats, set_gc_notifier, set_memory_limit,
    set_cpu_profile_rate, get_memory_limit, get_cpu_profile_rate
};

// Core builtin functions re-exports - Fundamental types and functions for CURSED
pub use core::{
    // CURSED type aliases
    Litean, Normie, Thicc, Snack, Meal, Tea,
    
    // Type conversion functions
    lit, normie, thicc, snack, meal, tea,
    
    // Collection operations
    append, cap, len, make, new,
    
    // Panic and recovery
    shook, unbothered, try_unbothered,
    
    // Utility functions
    zero_value, is_zero_value, type_of, clone_value, equal_values,
    
    // Module management
    init_core, get_core_stats,
    
    // Error types
    CoreError, CoreResult
};

pub use dot_registry::DOT_REGISTRY;

// Performance monitoring and profiling re-exports - Comprehensive profiling tools
pub use profiler::{
    // Error handling system
    ProfilerError, ProfilerResult,
    
    // CPU profiling
    CpuProfiler, CpuProfile as ProfilerCpuProfile, CpuSample, FunctionProfile, CallGraph, 
    ProfileData, SamplingConfig, ProfilerConfig,
    start_cpu_profiling, stop_cpu_profiling, get_cpu_profile,
    
    // Memory profiling
    MemoryProfiler, MemoryProfile as ProfilerMemoryProfile, AllocationProfile, AllocationSite,
    MemoryStats as ProfilerMemoryStats, HeapProfile, GcProfile, MemoryTracker,
    start_memory_profiling, stop_memory_profiling, get_memory_profile,
    track_allocation, track_deallocation, get_memory_stats,
    
    // Benchmark framework
    Benchmark as ProfilingBenchmark, BenchmarkResult as ProfilingBenchmarkResult, BenchmarkSuite, BenchmarkConfig,
    BenchmarkRunner, BenchmarkReport, ComparisonResult,
    benchmark_function, benchmark_with_setup, run_benchmark_suite,
    generate_benchmark_report,
    
    // Performance metrics
    PerformanceMetrics as ProfilerPerformanceMetrics, MetricsCollector, MetricType, MetricValue,
    CounterMetric, GaugeMetric, HistogramMetric, TimerMetric,
    collect_metrics, start_metrics_collection, stop_metrics_collection,
    get_current_metrics, export_metrics,
    
    // Runtime integration
    ProfilerRuntime, RuntimeProfiler, IntegrationConfig,
    initialize_profiler, shutdown_profiler, get_profiler_runtime,
    integrate_with_gc, integrate_with_goroutines, integrate_with_jit,
    
    // Utility functions
    get_statistics, ProfilerStatistics, quick_performance_check, QuickStats,
    get_profiling_overhead
};

// TestVibes testing framework re-exports - Comprehensive testing with Gen Z flavor
pub use test_vibes::{
    // Core testing types
    VibeTest, VibeBench, VibeTestingManager, TestMain,
    VibeTestState, VibeBenchState, TestResult as VibeTestResult, BenchmarkResult as TestingBenchmarkResult,
    
    // Comprehensive assertion framework
    Assert, AssertEqual, AssertNotEqual, AssertNil, AssertNotNil,
    AssertTrue, AssertFalse, AssertError, AssertNoError, AssertErrorIs, AssertErrorContains,
    AssertLen, AssertEmpty, AssertNotEmpty, AssertContains, AssertNotContains,
    AssertGreater, AssertGreaterOrEqual, AssertLess, AssertLessOrEqual,
    AssertZero, AssertNotZero, AssertContainsSubtea, AssertHasPrefix, AssertHasSuffix,
    AssertMatchesRegex, AssertType, AssertImplements,
    AssertShooks, AssertShooksWithValue, AssertNoShook,
    
    // Test fixtures and table-driven tests
    FixtureVibe, NewFixtureVibe, TestCase, RunTestCases,
    
    // Mocking framework
    MockVibe, Expectation, Stub,
    
    // Test utilities
    TempFile, TempDir, Parallel, WithDeadline, WithSetup,
    RandomString, RandomInt, RandomFloat, RandomBytes,
    
    // Benchmarking utilities
    Benchmark, BenchmarkMemory, BenchmarkParallel,
    
    // Error handling
    TestVibesResult, TestVibesError,
    test_failed, test_skipped, assertion_failed, expectation_not_met, timeout_exceeded
};

// SignalBoost signal handling re-exports - Enhanced OS signal management
pub use signal_boost::{
    // Core signal types and constants
    BoostSignal, NotifyHandle,
    SIGINT, SIGTERM, SIGHUP, SIGQUIT, SIGILL, SIGTRAP, SIGABRT, SIGBUS, SIGFPE,
    SIGKILL, SIGSEGV, SIGPIPE, SIGALRM, SIGCHLD, SIGCONT, SIGSTOP, SIGTSTP,
    SIGTTIN, SIGTTOU, SIGUSR1, SIGUSR2, SIGWINCH,
    
    // Core signal handling functions
    notify, notify_context, stop, reset, ignored,
    
    // Signal handler for custom handlers
    SignalHandler, SignalHandlerConfig,
    
    // Graceful shutdown coordination
    GracefulShutdown, ShutdownOptions, ShutdownStatus, ShutdownTask, ShutdownTaskGroup,
    
    // Signal multiplexing
    SignalMultiplexer, MultiplexerHandle,
    
    // Signal actions
    SignalAction, ignore_action, exit_action, exit_with_code_action,
    log_action, shook_action, chain_actions,
    
    // Process signal management
    signal_process, signal_group, broadcast, get_targets,
    
    // Signal filtering and throttling
    filter_signals, throttle_signals, debounce_signals,
    
    // GenZ themed features
    VibeChecker, vibe_check, yeet_on_signal, no_cap_reload_config,
    
    // Error handling
    SignalBoostError, SignalBoostResult,
    
    // Module management
    initialize as initialize_signal_boost, get_statistics as get_signal_boost_statistics,
    ModuleStatistics as SignalBoostModuleStatistics
};

// VibeNet networking re-exports - Complete networking package with CURSED flair
pub use vibe_net::{
    // Core types and result handling
    VibeContext,
    
    // IP addressing
    IPVibe, IPNetVibe, IPMaskVibe,
    
    // Network addresses
    AddrVibe, TCPAddrVibe, UDPAddrVibe, UnixAddrVibe,
    
    // Connections
    ConnVibe, TCPConnVibe, UDPConnVibe, UnixConnVibe, PacketConnVibe,
    
    // Listeners
    ListenerVibe, TCPListenerVibe, UnixListenerVibe,
    
    // DNS resolution
    DNSResolverVibe, MXVibe, NSVibe, SRVVibe,
    
    // Dialer configuration
    DialerVibe,
    
    // Enhanced features
    ConnPoolVibe, ConnPoolStats,
    CircuitBreakerVibe, CircuitBreakerState,
    RateLimiterVibe, Reservation,
    
    // Protocol adapters
    WebSocketConnVibe, MQTTConnVibe, HTTP2ConnVibe, HTTP2StreamVibe,
    
    // Network interfaces
    InterfaceVibe, InterfaceFlags, HardwareAddrVibe,
    
    // High-level functions
    dial, dial_timeout, listen, listen_packet,
    resolve_tcp_addr, resolve_udp_addr, resolve_unix_addr,
    dial_tcp, dial_udp, dial_unix,
    
    // DNS functions
    lookup_host, lookup_ip, lookup_port,
    lookup_srv, lookup_ns, lookup_addr,
    
    // IPv6 support
    is_ipv6_enabled, prefer_ipv6, set_prefer_ipv6, ipv6_interface_addrs,
    
    // Interface functions
    interfaces, interface_by_index, interface_by_name,
    
    // Module utilities
    init as init_vibe_net, version as vibe_net_version, features as vibe_net_features,
    
    // Error handling
    error::{
        NetError as VibeNetError, address_resolution_error, connection_failed_error, timeout_error as vibe_net_timeout_error,
        invalid_protocol_error, dns_resolution_error, interface_error, socket_error, tls_error,
        rate_limit_error, circuit_breaker_error, pool_exhausted_error,
        io_error as vibe_net_io_error, permission_denied_error, resource_unavailable_error, invalid_config_error
    }
};

// exec_vibez command execution re-exports - Enhanced external command execution
pub use exec_vibez::{
    // Core command execution types
    Cmd, Process as ExecProcess, ProcessState, Command, CommandContext,
    
    // Error handling
    ExecError, ExecResult,
    
    // Process context and timeout support
    VibeContext as ExecVibeContext, ProcessContext, ContextError,
    
    // Process groups and coordination
    ProcessGroup, ProcessGroupOptions, NewProcessGroup,
    
    // Environment management
    Environment, NewEnvironment, CommandWithEnv,
    
    // Output streaming and input generation
    OutputStreamer, NewOutputStreamer, InputGenerator, NewInputGenerator,
    
    // Timeout and execution control
    RunWithTimeout, TimeoutConfig,
    
    // Enhanced features
    LookPath, ProcessMonitor as ExecProcessMonitor, ResourceLimits as ExecResourceLimits, SecurityOptions,
    ProcessPool, ProcessQueue, BatchRunner,
    PlatformFeatures, CrossPlatformUtils,
    
    // Module management
    initialize as initialize_exec_vibez, get_statistics as get_exec_vibez_statistics,
    ModuleStatistics as ExecVibezModuleStatistics
};

// JSON Tea - JSON encoding and decoding re-exports
pub use json_tea::{
    // Core JSON operations
    marshal, unmarshal, marshal_indent, valid, marshal_to_string, unmarshal_from_string,
    
    // Streaming support
    new_encoder, new_decoder, Encoder, Decoder, StreamingEncoder, StreamingDecoder,
    
    // JSON value types
    JsonValue, JsonResult, JsonObject, JsonArray,
    
    // JSON tag support
    tags::JsonTag,
    
    // Error handling
    JsonErrorKind,
    
    // Utility functions
    escape_json_string, unescape_json_string,
};

// OG Logging - Simple and powerful logging re-exports
pub use oglogging::{
    // Core logging types
    Logger, new_logger,
    
    // Standard logger functions
    spill, spillf, fatal, fatalf, shook, shookf,
    set_flags, set_output, set_prefix, flags, prefix,
    
    // Format flags
    Ldate, Ltime, Lmicroseconds, Llongfile, Lshortfile, LUTC, Lmsgprefix, LstdFlags,
    has_flag, set_flag, clear_flag, toggle_flag, describe_flags, validate_flags,
    
    // Preset flag combinations
    presets,
    
    // Formatting utilities
    format_log_entry, get_timestamp, format_message,
};

// CSV Mood - Comprehensive CSV processing re-exports
pub use csv::{
    // Error handling system
    CsvError, CsvResult, ParseError,
    
    // Core CSV operations
    Reader, ReaderConfig, Writer, WriterConfig,
    new_reader, new_writer,
    
    // Column-based access
    ColumnReader, TypedValue, new_column_reader,
    
    // Streaming for large files
    Streamer, StreamProcessor, new_streamer,
    
    // Schema validation
    Schema, SchemaColumn, ValidationResult, ValidationError, ColumnType, new_schema,
    
    // Data transformation
    Transformer, ColumnTransform, TransformResult, new_transformer,
    
    // Utility functions
    read_all_from_string, write_all_to_string,
    validate_csv_data, transform_csv_data,
};

// RegexVibez - Regular expression processing re-exports
pub use regex_vibez::{
    // Error handling system
    RegexVibesError, RegexVibesResult,
    
    // Core regex types
    VibePattern, VibeGroups, PatternBuilder,
    
    // Compilation functions
    compile, must_compile, compile_posix, must_compile_posix,
    
    // Helper functions
    r#match, match_string, quote_meta, new_pattern_builder,
    
    // Common patterns library
    EMAIL_PATTERN, URL_PATTERN, DATE_PATTERN, TIME_PATTERN,
    USERNAME_PATTERN, PASSWORD_PATTERN, PHONE_PATTERN, ZIP_CODE_PATTERN,
    HASHTAG_PATTERN, EMOJI_PATTERN, IPV4_PATTERN, IPV6_PATTERN,
    CREDIT_CARD_PATTERN, HEX_COLOR_PATTERN, UUID_PATTERN, HTML_TAG_PATTERN,
    JSON_STRING_PATTERN, BASE64_PATTERN, MAC_ADDRESS_PATTERN, SSN_PATTERN,
    CURRENCY_PATTERN, VERSION_PATTERN, CommonPatterns,
    
    // Utility functions
    is_valid_pattern, validate_pattern, count_capture_groups, extract_literals,
    find_common_prefix, find_common_suffix, strings_to_alternation, optimize_string_list,
    test_patterns, benchmark_pattern, BenchmarkResult as RegexBenchmarkResult, escape_replacement,
    parse_replacement_references, glob_to_regex, glob_match, find_regex_patterns,
    create_line_filter, split_keep_delimiter,
    
    // Groups functionality
    GroupStatistics, GroupValidationResult,
};

// ChaosMode runtime system re-exports - Comprehensive runtime control with Gen Z flair
pub use chaos_mode::{
    // Error handling system
    ChaosError, ChaosResult,
    
    // Core runtime functions
    num_cpu as chaos_num_cpu, num_goroutine as chaos_num_goroutine, yield_processor, gosched, gc, gomaxprocs as chaos_gomaxprocs,
    set_gc_percent as chaos_set_gc_percent, set_max_heap,
    
    // Memory management and statistics
    MemoryStats as ChaosMemoryStats, mem_stats, read_mem_stats as chaos_read_mem_stats, set_gc_enabled, free_os_memory as chaos_free_os_memory,
    set_mem_profile_rate,
    
    // Memory debugging features
    allocation_size_histogram, top_allocated_types, TypeAllocationInfo,
    is_valid_pointer, get_object_size, get_pointer_info, PointerInfo,
    
    // Goroutine and stack management
    stack_trace, all_goroutine_ids, all_goroutine_stacks, callers,
    pc_to_file_and_line, pc_to_func_name, goroutine_stack,
    
    // Enhanced goroutine management
    GoroutineData, goroutine_info as chaos_goroutine_info, set_goroutine_label, goroutines_by_label,
    goroutines_by_state, kill_goroutine,
    
    // Profiling and tracing
    start_trace, stop_trace, read_trace, set_traceback_limit,
    start_cpu_profile, stop_cpu_profile,
    
    // Runtime information
    version as chaos_version, goarch as chaos_goarch, goos as chaos_goos, compiler as chaos_compiler, runtime_stats, goroot,
    
    // Enhanced garbage collection
    GCMode, set_gc_mode, get_gc_mode, start_gc, wait_for_gc,
    register_gc_notification,
    
    // Performance tuning
    SchedulerMode, set_max_threads, num_threads, set_cpu_frequency,
    set_thread_priority, set_scheduler_mode, get_scheduler_mode,
    
    // Module management
    initialize as initialize_chaos_mode, cleanup as cleanup_chaos_mode,
    chaos_stats,
};

// EmbedThat file embedding re-exports - Comprehensive embedded file management
pub use embed_that::{
    // Error handling system
    EmbedError, EmbedResult,
    file_not_found, invalid_format, compression_error, decompression_error,
    template_parsing_error, image_loading_error, cache_error, mime_type_error,
    config_parsing_error, resource_limit_exceeded, invalid_pattern, general_error,
    
    // Core types for embedded files
    ThatFile, ThatFiles, ThatString, ThatBytes,
    FileSystemVibe, DirEntry as VibeDirEntry, FileInfo, EmbeddedFileSystem,
    
    // Resource loading functions
    load_that_file, load_that_dir, load_that_pattern, file_exists, get_embed_statistics,
    ResourceLoader, EmbedStatistics, EmbedManifest, ManifestItem,
    initialize_resource_loader,
    
    // Template integration
    parse_templates, parse_templates_with_funcs, validate_all_templates,
    get_default_template_helpers, TemplateIntegration, ValidationReport, TemplateHelpers,
    
    // Specific file type loaders
    load_image, load_image_fs, load_json, load_yaml, load_toml, load_config,
    load_text_file, load_binary_file, load_css, load_javascript, load_html,
    load_font, load_audio, load_video,
    ImageData, ImageType, CssData, JavaScriptData, HtmlData,
    FontData, FontType, AudioData, AudioType, VideoData, VideoType,
    
    // Compression support
    decompress_file, load_compressed_fs, compress_data, analyze_compression, get_compression_stats,
    CompressionType, CompressionStats, CompressionResult, CompressionAnalysis,
    CompressedEmbeddedFile,
    
    // Caching support
    new_resource_cache, new_resource_cache_with_expiry, new_resource_cache_with_config,
    get_global_cache, ResourceCache, CacheStatistics, CacheConfig,
    
    // Module management
    initialize as initialize_embed_that, get_module_info as get_embed_module_info,
    ModuleInfo as EmbedModuleInfo, ValidationSummary, MemoryUsageSummary,
    
    // Utility functions
    utils::{
        load_config_auto, load_directory_as_map, get_files_by_type,
        validate_all_embedded_files, get_memory_usage_summary
    },
    
    // Constants
    constants::{
        TEMPLATE_PATTERNS, STATIC_ASSET_PATTERNS, CONFIG_PATTERNS, DOCUMENTATION_PATTERNS,
        DEFAULT_CACHE_EXPIRY_SECONDS, DEFAULT_CACHE_MAX_SIZE, DEFAULT_CACHE_CLEANUP_INTERVAL_SECONDS,
        MIN_COMPRESSION_SIZE, COMPRESSION_RATIO_THRESHOLD
    }
};

// Vibez formatting and printing re-exports - Core formatting functionality with Gen Z flair
pub use vibez::{
    // Core printing functions (avoiding conflicts with io module)
    print_to, println_to,
    print_styled, println_styled, print_colored, println_colored,
    PrintStyle, PrintColor,
    
    // Spill functions - Essential Gen Z I/O operations
    spillstr, scan, scanln,
    
    // Advanced formatting
    format, format_args, format_with_context, interpolate,
    FormatError, FormatResult, FormatContext, FormatOptions,
    FormatPlaceholder, FormatSpec, PlaceholderType, FormatAlignment, FormatSign,
    
    // Printf-style formatting
    sprintf, snprintf, sprintf_to_writer,
    SprintfError, SprintfResult, FormatSpecifier,
    validate_format_string, count_format_specifiers,
    
    // Debug utilities
    debug_print, debug_println, debug_format, pretty_print,
    debug_dump, debug_inspect, debug_trace,
    DebugOptions, DebugStyle, DebugLevel,
    set_debug_level, get_debug_level, is_debug_enabled,
    
    // Module management
    initialize as initialize_vibez, version as vibez_version, capabilities as vibez_capabilities
};

// GlyphGang Unicode processing re-exports - Comprehensive Unicode support with CURSED flair
pub use glyph_gang::{
    // Error handling system
    GlyphGangError, GlyphGangResult,
    unicode_error, normalization_error, encoding_error, range_error as glyph_range_error,
    
    // Character classification functions
    is_letter, is_digit, is_number, is_space, is_punct, is_symbol, is_mark,
    is_control, is_graphic, is_print, is_upper, is_lower, is_title,
    
    // Advanced character classifications
    is_emoji, is_emoji_modifier, is_emoji_component, is_currency, is_math,
    is_format, is_private_use, is_surrogate, is_ascii as glyph_is_ascii,
    
    // Character conversion functions (avoiding conflicts with string module)
    to_upper as char_to_upper, to_lower as char_to_lower, to_title as char_to_title, 
    to_ascii as char_to_ascii, simple_fold as char_simple_fold,
    
    // Range and character set functions
    is_in_range, is_in_ranges, is_one_of,
    
    // Character properties and information
    get_character_name, find_character_by_name, get_block_name,
    get_category, get_properties, get_code_point, get_canonical_equivalent,
    
    // Unicode range tables and properties
    RangeTable, Range16, Range32,
    LETTER, UPPERCASE_LETTER, LOWERCASE_LETTER, TITLECASE_LETTER,
    MODIFIER_LETTER, OTHER_LETTER, NUMBER, DECIMAL_NUMBER, LETTER_NUMBER,
    OTHER_NUMBER, PUNCT, CONNECTOR_PUNCTUATION, DASH_PUNCTUATION,
    OPEN_PUNCTUATION, CLOSE_PUNCTUATION, INITIAL_PUNCTUATION,
    FINAL_PUNCTUATION, OTHER_PUNCTUATION, SYMBOL, MATH_SYMBOL,
    CURRENCY_SYMBOL, MODIFIER_SYMBOL, OTHER_SYMBOL, MARK,
    NON_SPACING_MARK, SPACING_MARK, ENCLOSING_MARK, SPACE,
    CONTROL, FORMAT, SURROGATE, PRIVATE, UNASSIGNED,
    LATIN, GREEK, CYRILLIC, HEBREW, ARABIC, DEVANAGARI, THAI,
    HAN, HIRAGANA, KATAKANA, HANGUL,
    EMOJI, EMOJI_PRESENTATION, EMOJI_MODIFIER, EMOJI_MODIFIER_BASE,
    EMOJI_COMPONENT, EXTENDED_PICTOGRAPHIC,
    
    // Enhanced Unicode-aware string operations (avoiding conflicts with string module)
    to_upper_string as unicode_to_upper_string, to_lower_string as unicode_to_lower_string, 
    to_title_string as unicode_to_title_string, normalize_string as unicode_normalize_string,
    NormalizationForm, NFC, NFD, NFKC, NFKD,
    rune_count, first_rune, last_rune, rune_at, rune_indices,
    string_width as unicode_string_width, truncate_string as unicode_truncate_string, 
    wrap_text as unicode_wrap_text, reverse_string as unicode_reverse_string,
    word_boundaries, sentence_boundaries, line_break_opportunities,
    fold_string as unicode_fold_string, equal_fold as unicode_equal_fold, 
    get_char_width, get_string_width as unicode_get_string_width,
    truncate_with_ellipsis as unicode_truncate_with_ellipsis,
    
    // Emoji support and detection
    is_emoji_sequence, contains_emoji, extract_emojis, replace_emojis,
    get_emoji_name, find_emoji_by_name, emoji_categories, emojis_in_category,
    
    // Bidirectional text support
    Direction, LTR, RTL, MIXED,
    get_direction, get_string_direction, is_rtl, is_ltr, is_mixed,
    
    // Script detection and analysis
    Script, SCRIPT_UNKNOWN, SCRIPT_LATIN, SCRIPT_GREEK, SCRIPT_CYRILLIC,
    detect_script, get_script_name, get_languages_by_script,
    
    // Module management
    initialize as initialize_glyph_gang, version as glyph_gang_version, 
    unicode_version, capabilities as glyph_gang_capabilities
};

// NoCap string conversion re-exports - CURSED string conversion utilities with Gen Z flair
pub use no_cap::{
    // Error handling system
    NoCapError, NoCapResult, ErrSyntax, ErrRange,
    
    // Type aliases
    Tea, Lit, Normie,
    
    // Core parsing functions (String to Value)
    FactsCheck, YoinkInt, YoinkUint, YoinkFloat,
    
    // Core formatting functions (Value to String)
    YeetBool, YeetInt, YeetUint, YeetFloat, SussyFloat,
    
    // Convenience functions
    Atoi, Itoa,
    
    // Enhanced utility functions
    utils::{
        AutoYoinkInt, AutoYoinkUint, AutoYoinkFloat,
        IsValidInt, IsValidFloat, IsValidBool,
        ConvertBase, FormatWithSeparators, ParseWithSeparators,
        FormatBoolCustom, LooksLikeNumber, GetNumberType, NumberType
    },
    
    // Module management
    init_no_cap, get_no_cap_stats, NoCap
};

// ByteFit byte manipulation re-exports - Comprehensive byte slice operations
pub use bytefit::{
    // Error handling system
    ByteFitError, ByteFitResult,
    
    // Basic operations (avoiding conflicts with string module)
    compare as byte_compare, equal as byte_equal, repeat as byte_repeat, runes as byte_runes,
    
    // Search functions (avoiding conflicts with string module)  
    contains_any as byte_contains_any, contains_rune as byte_contains_rune,
    has_prefix as byte_has_prefix, has_suffix as byte_has_suffix,
    index as byte_index, index_any as byte_index_any, index_byte, index_rune as byte_index_rune,
    last_index as byte_last_index, last_index_any as byte_last_index_any, last_index_byte,
    
    // Transformation functions (avoiding conflicts with string module)
    map as byte_map,
    
    // Splitting functions (avoiding conflicts with string module) 
    split_after as byte_split_after, split_after_n as byte_split_after_n, 
    fields as byte_fields, fields_func as byte_fields_func,
    
    // Trimming functions (avoiding conflicts with string module)
    trim_left as byte_trim_left, trim_right as byte_trim_right, 
    trim_space as byte_trim_space, trim_prefix as byte_trim_prefix, 
    trim_suffix as byte_trim_suffix, trim_func as byte_trim_func,
    
    // Enhanced Buffer type
    FitBuffer, new_fit_buffer,
    
    // Binary data manipulation
    from_hex as byte_from_hex, to_hex as byte_to_hex, 
    from_base64 as byte_from_base64, to_base64 as byte_to_base64,
    and as byte_and, or as byte_or, xor as byte_xor, not as byte_not, 
    shift_left as byte_shift_left, shift_right as byte_shift_right,
    
    // Pattern matching
    wildcard_match as byte_wildcard_match, regex_match as byte_regex_match, 
    regex_find_all as byte_regex_find_all, regex_replace as byte_regex_replace
};

// ================================
// GEN Z STDLIB MODULE RE-EXPORTS (CURSED CONVENTIONS)
// ================================

// StringZ - Tea manipulation functions with Gen Z flair
pub use stringz::{
    // Core tea manipulation
    string_length, is_empty_tea, concat_tea, repeat_tea, reverse_tea, char_at_index,
    
    // Search and replace  
    contains_tea, starts_with_tea, ends_with_tea, find_tea, find_last_tea,
    replace_all_tea, replace_first_tea, count_tea,
    
    // Transformations
    substring_tea, trim_tea, trim_start_tea, trim_end_tea,
    to_lowercase_tea, to_uppercase_tea, to_title_case_tea, to_camel_case_tea,
    to_pascal_case_tea, to_snake_case_tea, to_kebab_case_tea, capitalize_tea,
    
    // Splitting and joining
    split_tea, split_tea_n, split_lines_tea, split_whitespace_tea,
    join_tea, join_owned_tea,
    
    // Validation
    is_numeric_tea, is_integer_tea, is_alphabetic_tea, is_alphanumeric_tea,
    is_whitespace_tea, is_uppercase_tea, is_lowercase_tea, is_email_tea,
    is_url_tea, is_palindrome_tea,
    
    // Formatting
    pad_left_tea, pad_right_tea, center_tea, truncate_tea, wrap_text_tea,
    escape_html_tea, escape_json_tea,
    
    // Enhanced operations
    tea_to_bytes, bytes_to_tea, tea_chars, is_ascii_tea,
    insert_at_tea, remove_range_tea,
    
    // Utility functions
    partition_tea, rpartition_tea, chunk_tea, add_line_numbers_tea,
    indent_lines_tea, has_balanced_parentheses_tea, has_balanced_brackets_tea,
    
    // Module functions
    init_stringz, get_stringz_stats,
    
    // Error types
    StringzError, StringzResult
};

// MathZ - Mathematical functions with CURSED types and Gen Z naming
pub use mathz::{
    // CURSED type aliases
    Thicc, Smol, Chonky,
    
    // Normie (i32) operations
    abs_normie, min_normie, max_normie, clamp_normie, sign_normie,
    is_even_normie, is_odd_normie, gcd_normie, lcm_normie,
    
    // Thicc (i64) operations  
    abs_thicc, min_thicc, max_thicc, clamp_thicc, sign_thicc,
    is_even_thicc, is_odd_thicc,
    
    // Chonky (f64) operations
    abs_chonky, min_chonky, max_chonky, clamp_chonky, sign_chonky,
    floor_chonky, ceil_chonky, round_chonky, truncate_chonky, fract_chonky,
    is_zero_chonky, is_equal_chonky,
    
    // Power and root functions
    pow_chonky, powi_chonky, square_chonky, cube_chonky,
    sqrt_chonky, cbrt_chonky, nth_root_chonky, hypot_chonky, reciprocal_chonky,
    
    // Logarithmic and exponential
    ln_chonky, log10_chonky, log2_chonky, log_chonky,
    exp_chonky, exp2_chonky, exp10_chonky,
    
    // Trigonometric functions
    sin_chonky, cos_chonky, tan_chonky, asin_chonky, acos_chonky,
    atan_chonky, atan2_chonky, sinh_chonky, cosh_chonky, tanh_chonky,
    
    // Mathematical constants
    PI_CHONKY, TAU_CHONKY, E_CHONKY, PHI_CHONKY, SQRT_2_CHONKY,
    LN_2_CHONKY, LN_10_CHONKY,
    
    // Random number generation
    random_chonky, random_range_chonky, random_normie,
    
    // Interpolation functions
    lerp_chonky, inverse_lerp_chonky, smooth_step_chonky, smoother_step_chonky,
    map_range_chonky,
    
    // Utility functions
    is_valid_chonky, round_to_decimals_chonky, average_chonky,
    geometric_mean_chonky, harmonic_mean_chonky,
    
    // Module functions
    init_mathz, get_mathz_stats,
    
    // Error types
    MathzError, MathzResult
};

// ConcurrenZ - Synchronization primitives with Gen Z flair
pub use concurrenz::{
    // Mutex operations
    MutexVibes, new_mutex_vibes,
    
    // Read-Write Lock operations
    RwLockVibes, new_rwlock_vibes,
    
    // Atomic operations
    AtomicBoolVibes, AtomicIntVibes, new_atomic_bool_vibes, new_atomic_int_vibes,
    
    // Channel operations
    SenderVibes, ReceiverVibes, channel_vibes,
    
    // Thread operations
    ThreadHandleVibes, spawn_thread_vibes, spawn_named_thread_vibes,
    sleep_vibes, yield_vibes, current_thread_id_vibes, current_thread_name_vibes,
    
    // Barrier operations
    BarrierVibes, new_barrier_vibes,
    
    // Condition variable operations
    CondVarVibes, new_condvar_vibes,
    
    // Once operations
    OnceVibes, new_once_vibes,
    
    // Utility functions
    num_cpus_vibes, park_vibes, park_timeout_vibes, unpark_thread_vibes,
    
    // Module functions
    init_concurrenz, get_concurrenz_stats,
    
    // Error types
    ConcurrenzError, ConcurrenzResult
};

// DropZ - Basic I/O primitives with Gen Z naming
pub use dropz::{
    // Basic output operations
    drop_tea, drop_line_tea, drop_error_tea, drop_error_line_tea,
    drop_formatted_tea, flush_drops,
    
    // Basic input operations
    catch_line_tea, catch_char_vibes, catch_until_vibes, catch_all_tea,
    
    // Interactive input operations
    vibe_check_tea, vibe_check_bool, vibe_check_secret,
    vibe_check_choice, vibe_check_multiple,
    
    // Number input operations
    catch_normie_vibes, catch_thicc_vibes, catch_chonky_vibes,
    
    // Buffered I/O operations
    StreamCatcherVibes, StreamDropperVibes,
    
    // Progress operations
    ProgressVibes, new_progress_vibes,
    
    // Paginated output
    paginate_drops,
    
    // Utility functions
    clear_drops, move_cursor_vibes, hide_cursor_vibes, show_cursor_vibes,
    get_terminal_size_vibes, set_text_color_vibes, reset_text_color_vibes,
    
    // Module functions
    init_dropz, get_dropz_stats,
    
    // Error types
    DropzError, DropzResult
};

// LookinGlass - Runtime reflection capabilities for CURSED
pub use lookin_glass::{
    // Error handling system
    LookinGlassError, LookinGlassResult,
    
    // Core reflection types
    Type as ReflectionType, Value as ReflectionValue, Kind, StructField, StructTag, Method,
    
    // Core reflection functions
    type_of, value_of, new, zero, indirect,
    make_slice, make_map, make_chan, make_func,
    
    // Type construction helpers
    array_of, slice_of, map_of, ptr_to, chan_of, func_of,
    
    // Type registry functions
    register_type, lookup_type, registered_types, init_type_registry,
    
    // Enhanced reflection utilities
    deep_equal, deep_copy, struct_to_map, map_to_struct,
    get_tags, set_field, get_field, has_field, field_names, field_info,
    value_to_map, map_to_value,
    
    // VibeMapper for advanced mapping and JSON conversion
    VibeMapper, VibeMapperConfig,
    camel_to_snake, snake_to_camel, 
    to_lowercase as reflection_to_lowercase, to_uppercase as reflection_to_uppercase,
    
    // Module management
    initialize as initialize_lookin_glass, get_reflection_statistics, ReflectionStatistics,
};

// Squish Core - Compression and decompression functionality with CURSED flair
pub use squish_core::{
    // Error handling system
    SquishError, SquishResult,
    
    // Core interfaces
    Reader as SquishReader, Writer as SquishWriter, Compressor, Decompressor,
    
    // Compression constants and levels
    NO_COMPRESSION, BEST_SPEED, BEST_COMPRESSION, DEFAULT_COMPRESSION, HUFFMAN_ONLY,
    MIN_COMPRESSION_LEVEL, MAX_COMPRESSION_LEVEL, CompressionQuality, CompressionStrategy as SquishStrategy,
    FlushMode, is_valid_compression_level, quality_to_level, recommended_buffer_size,
    should_use_parallel, optimal_chunk_size,
    
    // Compression format modules
    gzip::{GzipReader, GzipWriter, new_reader as new_gzip_reader, new_writer as new_gzip_writer, 
           new_writer_level as new_gzip_writer_level, is_gzip_data},
    zlib::{ZlibReader, ZlibWriter, new_reader as new_zlib_reader, new_writer as new_zlib_writer,
           new_writer_level as new_zlib_writer_level, is_zlib_data},
    flate::{FlateReader, FlateWriter, new_reader as new_flate_reader, new_writer as new_flate_writer},
    bzip2::{Bzip2Reader, Bzip2Writer, new_reader as new_bzip2_reader, new_writer as new_bzip2_writer,
            new_writer_level as new_bzip2_writer_level, is_bzip2_data},
    lzw::{LzwReader, LzwWriter, Order as LzwOrder, new_reader as new_lzw_reader, new_writer as new_lzw_writer,
          default_literal_width, is_valid_literal_width},
    
    // Enhanced compression features
    adaptive::{AdaptiveCompressor, CompressionStrategy as AdaptiveStrategy, new_adaptive_compressor,
               new_compressor_with_strategy},
    dictionary::{Dictionary, DictionaryCompressor},
    parallel::{ParallelCompressor, ParallelOptions, compress_parallel},
    progressive::{ProgressiveCompressor, ProgressiveOptions, new_progressive_compressor,
                  new_compressor_with_options},
    
    // Statistics and performance monitoring
    statistics::{CompressionStats, PerformanceMetrics, ModuleStats, OperationTimer,
                 get_module_stats, update_global_stats, record_global_failure,
                 start_operation, end_operation},
    
    // High-level utility functions
    utils::{compress, decompress, compress_with_level, compress_adaptive, detect_format,
            max_compressed_size, validate_level_for_algorithm, get_file_extension, get_mime_type,
            supports_streaming, get_recommended_buffer_size, estimate_compression_ratio},
    
    // Module management
    initialize as initialize_squish_core, get_module_stats as get_squish_stats, cleanup as cleanup_squish_core,
};

// VibeLife - OS functionality with Gen Z flair
pub use vibe_life::{
    // CURSED type aliases (Tea, Normie, Thicc imported from other modules)
    
    // Environment variable operations
    get_env_vibe, set_env_vibe, remove_env_vibe, get_env_vibe_or,
    env_exists_vibe, get_all_env_vibes, clear_all_env_vibes,
    
    // Directory operations
    get_current_vibe, get_home_vibe, get_temp_vibe, change_directory_vibe,
    create_directory_vibe, create_directory_tree_vibe, remove_directory_vibe,
    remove_directory_tree_vibe, list_directory_vibe,
    
    // File operations
    path_exists_vibe, is_file_vibe, is_directory_vibe, get_file_size_vibe,
    copy_file_vibe, move_file_vibe, delete_file_vibe,
    read_file_vibe, write_file_vibe, append_file_vibe,
    
    // Process operations
    get_current_process_vibe, get_parent_process_vibe, is_process_alive_vibe,
    kill_process_vibe, force_kill_process_vibe, run_command_vibe,
    run_command_timeout_vibe, command_exists_vibe, which_command_vibe,
    
    // System information
    get_username_vibe, get_hostname_vibe, get_os_name_vibe,
    get_architecture_vibe, get_cpu_count_vibe, get_system_uptime_vibe,
    get_load_average_vibe,
    
    // Time operations
    get_timestamp_vibe, get_timestamp_millis_vibe, sleep_vibe,
    sleep_seconds_vibe, sleep_millis_vibe,
    
    // Path operations
    join_path_vibe, get_parent_vibe, get_filename_vibe, get_extension_vibe,
    get_absolute_path_vibe, normalize_path_vibe,
    
    // Signal operations
    exit_vibe, exit_success_vibe, exit_error_vibe, abort_vibe,
    
    // Memory operations
    get_memory_info_vibe, MemoryInfo as VibeMemoryInfo,
    
    // Utility functions
    get_path_separator_vibe, is_unix_vibe, is_windows_vibe, is_case_sensitive_vibe,
    
    // Module functions
    init_vibe_life, get_vibe_life_stats,
    
    // Error types
    VibeLifeError, VibeLifeResult
};


