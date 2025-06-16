/// Standard library for CURSED
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
    ProcessChannels, ProcessCommunication, NamedPipe, SharedMemory, MessageQueue,
    IpcType, CommunicationConfig, create_process_communication, create_pipe,
    execute_with_communication, send_and_receive, create_daemon, monitor_process_output,
    
    // Process monitoring and health checks
    HealthStatus, ResourceThresholds, HealthCheckConfig, PerformanceMetrics,
    PerformanceHistory, ProcessMonitor, MonitoredProcess, ProcessWatchdog,
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
    fence::memory_fence, fence::compiler_fence, fence::full_barrier, 
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
    Semaphore, NamedSemaphore, SemaphoreConfig, SemaphoreValue,
    
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

pub use dot_registry::DOT_REGISTRY;

// Performance monitoring and profiling re-exports - Comprehensive profiling tools
pub use profiler::{
    // Error handling system
    ProfilerError, ProfilerResult,
    
    // CPU profiling
    CpuProfiler, CpuProfile, CpuSample, FunctionProfile, CallGraph, 
    ProfileData, SamplingConfig, ProfilerConfig,
    start_cpu_profiling, stop_cpu_profiling, get_cpu_profile,
    
    // Memory profiling
    MemoryProfiler, MemoryProfile, AllocationProfile, AllocationSite,
    MemoryStats, HeapProfile, GcProfile, MemoryTracker,
    start_memory_profiling, stop_memory_profiling, get_memory_profile,
    track_allocation, track_deallocation, get_memory_stats,
    
    // Benchmark framework
    Benchmark, BenchmarkResult, BenchmarkSuite, BenchmarkConfig,
    BenchmarkRunner, BenchmarkReport, ComparisonResult,
    benchmark_function, benchmark_with_setup, run_benchmark_suite,
    generate_benchmark_report,
    
    // Performance metrics
    PerformanceMetrics, MetricsCollector, MetricType, MetricValue,
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
    VibeTestState, VibeBenchState, TestResult, BenchmarkResult,
    
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
    Cmd, Process, ProcessState, Command, CommandContext,
    
    // Error handling
    ExecError, ExecResult,
    
    // Process context and timeout support
    VibeContext, ProcessContext, ContextError,
    
    // Process groups and coordination
    ProcessGroup, ProcessGroupOptions, NewProcessGroup,
    
    // Environment management
    Environment, NewEnvironment, CommandWithEnv,
    
    // Output streaming and input generation
    OutputStreamer, NewOutputStreamer, InputGenerator, NewInputGenerator,
    
    // Timeout and execution control
    RunWithTimeout, TimeoutConfig,
    
    // Enhanced features
    LookPath, ProcessMonitor, ResourceLimits, SecurityOptions,
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
    test_patterns, benchmark_pattern, BenchmarkResult, escape_replacement,
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
    num_cpu, num_goroutine, yield_processor, gosched, gc, gomaxprocs,
    set_gc_percent, set_max_heap,
    
    // Memory management and statistics
    MemoryStats, mem_stats, read_mem_stats, set_gc_enabled, free_os_memory,
    set_mem_profile_rate,
    
    // Memory debugging features
    allocation_size_histogram, top_allocated_types, TypeAllocationInfo,
    is_valid_pointer, get_object_size, get_pointer_info, PointerInfo,
    
    // Goroutine and stack management
    stack_trace, all_goroutine_ids, all_goroutine_stacks, callers,
    pc_to_file_and_line, pc_to_func_name, goroutine_stack,
    
    // Enhanced goroutine management
    GoroutineData, goroutine_info, set_goroutine_label, goroutines_by_label,
    goroutines_by_state, kill_goroutine,
    
    // Profiling and tracing
    start_trace, stop_trace, read_trace, set_traceback_limit,
    start_cpu_profile, stop_cpu_profile,
    
    // Runtime information
    version, goarch, goos, compiler, runtime_stats, goroot,
    
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
    FileSystemVibe, DirEntry, FileInfo, EmbeddedFileSystem,
    
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


