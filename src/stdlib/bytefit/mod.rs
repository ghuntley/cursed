/// ByteFit - Comprehensive byte slice manipulation library for CURSED
/// 
/// Provides functions for manipulating byte slices, offering efficient, 
/// fit-for-purpose operations on binary data with enhanced functionality
/// and a modern approach to byte manipulation.

pub mod basic;
pub mod search;
pub mod transform;
pub mod split;
pub mod trim;
pub mod buffer;
pub mod binary;
pub mod pattern;
pub mod error;

// Re-export all public functions and types
pub use error::{ByteFitError, ByteFitResult};

// Basic operations
pub use basic::{
    compare, equal, equal_fold, repeat, runes
};

// Search functions
pub use search::{
    contains, contains_any, contains_rune, count, has_prefix, has_suffix,
    index, index_any, index_byte, index_rune, 
    last_index, last_index_any, last_index_byte
};

// Transformation functions
pub use transform::{
    join, replace, replace_all, map, to_upper, to_lower, to_title
};

// Splitting functions
pub use split::{
    split, split_n, split_after, split_after_n, fields, fields_func
};

// Trimming functions
pub use trim::{
    trim, trim_left, trim_right, trim_space, trim_prefix, trim_suffix, trim_func
};

// Enhanced Buffer type
pub use buffer::{FitBuffer, new_fit_buffer};

// Binary data manipulation
pub use binary::{
    from_hex, to_hex, from_base64, to_base64,
    and, or, xor, not, shift_left, shift_right
};

// Pattern matching
pub use pattern::{
    wildcard_match, regex_match, regex_find_all, regex_replace
};
