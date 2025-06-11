/// Standard library for CURSED
pub mod dot_registry;
pub mod packages;
pub mod web_vibez;
pub mod http_core;
pub mod database;
pub mod crypto;
pub mod template;
pub mod errors_simple;
pub use errors_simple as errors;
pub mod value;
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
pub mod ipc;

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

// Inter-Process Communication re-exports - Comprehensive IPC support
pub use ipc::{
    // Error handling system
    IpcError, IpcResult, communication_error as ipc_communication_error, security_error, resource_error, timeout_error as ipc_timeout_error,
    invalid_operation, permission_denied as ipc_permission_denied, resource_exhausted, connection_failed,
    
    // Core IPC types and traits
    ProcessId, IpcHandle, IpcPermissions, IpcMode, IpcTimeout, IpcConfig, IpcStatistics,
    SharedMemoryId, MessageQueueId, SemaphoreId, PipeId, ResourceLimits as IpcResourceLimits,
    IpcChannel, IpcReader, IpcWriter, IpcBidirectional, Synchronizable, Lockable, 
    Waitable, Signalable, Serializable, Deserializable, IpcResource,
    
    // Shared Memory operations
    SharedMemory as IpcSharedMemory, SharedMemoryConfig, SharedMemoryRegion, SharedMemoryAccess,
    create_shared_memory, open_shared_memory, remove_shared_memory,
    
    // Named Pipes operations
    NamedPipe as IpcNamedPipe, AnonymousPipe, PipeConfig, PipeMode, PipeEnd,
    create_pipe as ipc_create_pipe, create_named_pipe, open_pipe, connect_pipe,
    
    // Message Queue operations
    MessageQueue as IpcMessageQueue, Message, MessageType, MessagePriority, MessageConfig,
    create_message_queue, open_message_queue, remove_message_queue,
    send_message, receive_message, peek_message,
    
    // Semaphore operations
    Semaphore as IpcSemaphore, SemaphoreConfig, SemaphoreValue, SemaphorePermissions,
    create_semaphore, open_semaphore, remove_semaphore,
    acquire_semaphore, release_semaphore, try_acquire_semaphore,
    
    // Signal handling
    SignalHandler, Signal as IpcSignal, SignalAction, SignalMask, SignalConfig,
    send_signal, block_signal, unblock_signal, ignore_signal as ipc_ignore_signal,
    register_signal_handler, unregister_signal_handler, wait_for_signal, signal_pending,
    
    // Domain Socket operations
    DomainSocket, UnixSocket, SocketConfig, SocketType, SocketAddress,
    create_socket, bind_socket, listen_socket, accept_connection, connect_socket,
    
    // Remote Procedure Call infrastructure
    RpcClient, RpcServer, RpcConfig, RpcMethod, RpcRequest, RpcResponse,
    RpcError, RpcHandler, RpcRegistry, RpcTransport,
    create_rpc_server, create_rpc_client, register_rpc_method, call_remote_method,
    
    // Security and permissions management
    IpcSecurityContext, SecurityPolicy, AccessControl, Permission,
    Credential, AuthenticationMethod, AuthorizationResult,
    create_security_context, validate_permissions, check_access,
    encrypt_ipc_data, decrypt_ipc_data, generate_ipc_token,
    
    // Module management
    initialize as initialize_ipc, shutdown as shutdown_ipc, get_ipc_statistics,
    ResourceContentionStats, IpcPerformanceMetrics,
};

pub use dot_registry::DOT_REGISTRY;
