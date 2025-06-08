//! Runtime support for the CURSED language
//!
//! This module provides runtime support for CURSED language features that
//! require integration with the native platform, such as concurrency,
//! garbage collection, and FFI.

pub mod channel;
pub mod channel_gc;
pub mod container;
pub mod jit_runtime;
pub mod type_assertion_runtime;
pub mod unicode_char;

// Re-export the public API
pub use channel::*;
pub use channel_gc::*;
pub use container::*;
pub use jit_runtime::*;
pub use type_assertion_runtime::*;
pub use unicode_char::*;