//! Standard Library for the CURSED programming language
//!
//! This module implements the standard library for CURSED, providing essential
//! functions, types, and utilities that form the core functionality available
//! to all CURSED programs. The standard library is organized into packages
//! similar to Go's standard library, but using Gen Z slang naming conventions.
//!
//! Each package provides a specific category of functionality:
//!
//! - `vibez`: Formatted I/O (like Go's fmt)
//! - `stringz`: String manipulation (like Go's strings)
//! - `mathz`: Mathematical functions (like Go's math)
//! - `timez`: Time-related utilities (like Go's time)
//! - `vibe_life`: OS interaction (like Go's os)
//! - `dropz`: I/O primitives (like Go's io)
//! - `concurrenz`: Synchronization primitives (like Go's sync)
//! - `web_vibez`: HTTP client and server (like Go's net/http)
//! - `json_tea`: JSON parsing and generation (like Go's encoding/json)
//! - `regex_vibez`: Regular expressions (like Go's regexp)
//! - `cryptz`: Cryptographic operations (like Go's crypto)
//! - `reflectz`: Runtime reflection (like Go's reflect)
//! - `htmlrizzler`: HTML templates (like Go's html/template)
//! - `rizztemplate`: Text templates (like Go's text/template)
//! - `core`: Core built-in functions (like Go's builtin)

// Export modules as they are implemented
pub mod core; // Core built-in functions (builtin equivalent)
pub mod concurrenz; // Synchronization primitives (sync equivalent)
// Keep disabled version for backward compatibility
pub mod concurrenz_disabled;
pub mod cryptz; // Cryptography functions (crypto equivalent)
pub mod dot_registry; // Dot expression registry for package.function calls
pub mod error_drip; // Error handling utilities (errors package equivalent)
pub mod dropz; // I/O primitives (io equivalent)
pub mod vector2d; // Vector2D type implementation and methods
pub mod htmlrizzler; // HTML templates (html/template equivalent)
pub mod json_tea; // JSON encoding/decoding (encoding/json equivalent)
pub mod mathz; // Math functions (math equivalent)
pub mod oglogging_simplified; // Logging functionality (log equivalent)
pub mod reflectz; // Runtime reflection (reflect equivalent)
pub mod regex_vibez; // Regular expressions (regexp equivalent)
pub mod rizztemplate; // Text templates (text/template equivalent)
pub mod stringz; // String manipulation functions (strings equivalent)
pub mod timez; // Time-related functionality (time equivalent)
pub mod vibe_life; // OS functionality (os equivalent)
pub mod vibez; // Printf-style functions (fmt equivalent)
pub mod web_vibez; // HTTP client and server (net/http equivalent)
pub mod syslog_era; // Syslog client functionality (log/syslog equivalent)
pub mod quick_test; // Property-based testing module
pub mod quick_test_generators; // Generators for property-based testing
pub mod chadlogging; // Structured logging (log/slog equivalent)
pub mod is_uppercase; // Character classification functions
pub mod generator; // Generator trait for property-based testing

// Re-export for convenient access - explicit imports to avoid name conflicts

// concurrenz exports
pub use concurrenz::{Mutex, RwMutex, WaitGroup, Once};

// Keep disabled version for backward compatibility (explicit imports)
pub use concurrenz_disabled::{create_mutex, lock_mutex, unlock_mutex, create_rwmutex, 
                            rlock_rwmutex, wlock_rwmutex, runlock_rwmutex, wunlock_rwmutex,
                            create_waitgroup, add_waitgroup, done_waitgroup, wait_waitgroup,
                            create_once, do_once};

// cryptz exports
pub use cryptz::{hash, verify, encrypt, decrypt, generate_key};

// dot_registry exports - already explicit
pub use dot_registry::{DOT_REGISTRY, is_supported, execute_dot, get_packages, get_functions};

// dropz exports
pub use dropz::{open, close, read, write, seek, open_file, close_file, read_file, write_file};

// error_drip exports
pub use error_drip::{new_error, wrap_error, unwrap_error, is_error, error_message};

// htmlrizzler exports - already explicit
pub use htmlrizzler::{escape_html, escape_js, escape_url};

// json_tea exports
pub use json_tea::{marshal, unmarshal};

// mathz exports
pub use mathz::{abs, sqrt, sin, cos, tan, log, exp, floor, ceil, round, max, min, pow, random};

// oglogging exports - aliased to avoid conflict
pub use oglogging_simplified as oglogging;

// reflectz exports
pub use reflectz::{type_name, kind_of, to_string, deep_equal, clone, set_prop, get_prop, has_prop};

// regex_vibez exports
pub use regex_vibez::{compile, match_str, find, find_all, replace as regex_replace, replace_all as regex_replace_all};

// rizztemplate exports
pub use rizztemplate::{parse_template, execute_template, add_func, parse_file};

// stringz exports (use explicit names to avoid conflicts)
pub use stringz::{len as str_len, contains, count, has_prefix, has_suffix, join, split,
                 to_lower, to_upper, trim, trim_space, trim_prefix, trim_suffix,
                 index, last_index, replace as str_replace, replace_all as str_replace_all, repeat};
                 
// timez exports
pub use timez::{now, format, parse, unix, sleep, after, tick, duration, add, sub};

// vibe_life exports
pub use vibe_life::{getenv, setenv, args, exit, hostname, executable, temp_dir, working_dir};

// vibez exports
pub use vibez::{println, printf, sprintf, errorf, scan, fscan};

// web_vibez exports
pub use web_vibez::{get as http_get, post as http_post, listen, handle, client, server};

// syslog_era exports
pub use syslog_era::{syslog, facility, priority, log_message, open_log, close_log};
// Quick test module exports 
pub use quick_test::{Config, TestResult, Rand, StateMachine, check, int_range, boolean, string, int_array,
float_range, hash_map, one_of_type, for_all, string_with_length, combine, weighted,
string_of, string_of_n_from, complex128, struct_of, alpha_numeric, slice_of, slice_of_n,
NO_SHRINK, DEFAULT_SHRINK, FULL_SHRINK, SMART_SHRINK};

// Quick test generators exports
pub use quick_test_generators::{string_of_n, int_range_gen, combine as combine_gen, 
StateMachineImpl, RandImpl, weighted as weighted_gen};

// Chadlogging module exports
pub use chadlogging::{Logger, Handler, Record, Attr, TextHandler, JSONHandler,
                    LEVEL_DEBUG, LEVEL_INFO, LEVEL_WARN, LEVEL_ERROR,
                    debug, info, warn, error, group, new, default};

// Character classification exports - with explicit names to avoid conflicts
pub use is_uppercase::{is_uppercase, is_lowercase, is_digit, is_alpha, 
                     to_uppercase as char_to_uppercase, to_lowercase as char_to_lowercase};

// Generator trait exports
pub use generator::{Generator, RandGen, clone_generator, prepare_generator_object, 
                   register_generators, value, one_of as generator_one_of};

// Core built-in exports - use explicit alias for len to avoid conflict
pub use core::{lit, normie, thicc, snack, meal, tea, len as core_len, 
              cap, append, make, panic, recover};

// Re-export core::new with a different name to avoid conflict with chadlogging::new
pub use core::new as core_new;