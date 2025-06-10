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

// Mathematics re-exports
pub use math::{
    MathError, MathResult, domain_error, range_error, division_by_zero_error, negative_input_error,
    is_valid_float, validate_float,
    // Basic operations
    abs, min, max, clamp, sign, floor, ceil, round, math_truncate, fract, remainder, modulo,
    gcd, lcm, is_even, is_odd, lerp, inverse_lerp, smooth_step, smoother_step,
    abs_i32, abs_i64, min_i32, max_i32, clamp_i32,
    // Trigonometric functions
    sin, cos, tan, asin, acos, atan, atan2, sinh, cosh, tanh, asinh, acosh, atanh,
    degrees_to_radians, radians_to_degrees, deg_to_rad, rad_to_deg,
    sin_deg, cos_deg, tan_deg, sec, csc, cot, normalize_angle, normalize_angle_signed,
    // Logarithmic/exponential functions
    ln, log10, log2, log, exp, exp2, exp10, pow, powi, sqrt, cbrt, nth_root, hypot, hypot3,
    expm1, ln1p, mul_add, inv_sqrt, ln_gamma, square, cube,
    // Mathematical constants
    PI, TAU, E, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8,
    FRAC_1_PI, FRAC_2_PI, FRAC_2_SQRT_PI, SQRT_2, FRAC_1_SQRT_2, SQRT_3, SQRT_5, SQRT_PI,
    LN_2, LN_10, LOG2_E, LOG2_10, LOG10_E, LOG10_2, PHI, INV_PHI, EULER_GAMMA,
    DEG_TO_RAD, RAD_TO_DEG, EPSILON, MIN_POSITIVE, MAX, MIN, INFINITY, NEG_INFINITY, NAN,
    // Special functions
    factorial, factorial_f64, gamma, beta, binomial, binomial_f64, permutations,
    erf, erfc, erf_inv, bessel_j0, bessel_j1, bessel_y0, bessel_y1,
    fibonacci, lucas, catalan,
    // Random number generation
    random, random_range, random_int, random_u64, random_bool, choice, choices, weighted_choice,
    shuffle, shuffled, sample, random_bytes, random_string, random_alphanumeric, random_hex,
    set_seed, random_normal, random_exponential, random_uniform, random_poisson,
    random_beta, random_gamma
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

pub use dot_registry::DOT_REGISTRY;
