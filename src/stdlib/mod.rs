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

// Export modules as they are implemented
// Temporarily disabled due to compilation issues
// pub mod concurrenz; // Synchronization primitives (sync equivalent)
pub mod concurrenz_disabled;
pub mod cryptz; // Cryptography functions (crypto equivalent)
pub mod dot_registry; // Dot expression registry for package.function calls
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

// Re-export for convenient access
// Temporarily disabled due to compilation issues
// pub use concurrenz::*;
pub use concurrenz_disabled::*;
pub use cryptz::*;
pub use dot_registry::{DOT_REGISTRY, is_supported, execute_dot, get_packages, get_functions};
pub use dropz::*;
pub use htmlrizzler::{escape_html, escape_js, escape_url};
pub use json_tea::*;
pub use mathz::*;
pub use oglogging_simplified as oglogging;
pub use reflectz::*;
pub use regex_vibez::*;
pub use rizztemplate::*;
pub use stringz::*;
pub use timez::*;
pub use vibe_life::*;
pub use vibez::*;
pub use web_vibez::*;
pub use syslog_era::*;
// Quick test module exports
pub use quick_test::{Config, TestResult, Rand, check, int_range, boolean, string, int_array,
                    float_range, hash_map, one_of_type, for_all, string_with_length,
                    NO_SHRINK, DEFAULT_SHRINK, FULL_SHRINK, SMART_SHRINK};