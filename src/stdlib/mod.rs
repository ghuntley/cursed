//! CURSED Standard Library Implementation
//! 
//! This module provides essential functions and types for the CURSED language.

// Export modules as they are implemented
pub mod vibez;     // Printf-style functions (fmt equivalent)
pub mod stringz;   // String manipulation functions (strings equivalent)
pub mod mathz;     // Math functions (math equivalent) 
pub mod timez;     // Time-related functionality (time equivalent)
pub mod vibe_life; // OS functionality (os equivalent)
pub mod dropz;     // I/O primitives (io equivalent)
pub mod concurrenz; // Synchronization primitives (sync equivalent)
pub mod web_vibez;  // HTTP client and server (net/http equivalent)
pub mod json_tea;   // JSON encoding/decoding (encoding/json equivalent)
pub mod regex_vibez; // Regular expressions (regexp equivalent)
pub mod cryptz;     // Cryptography functions (crypto equivalent)
pub mod reflectz;   // Runtime reflection (reflect equivalent)

// Re-export for convenient access
pub use vibez::*;
pub use stringz::*;
pub use mathz::*;
pub use timez::*;
pub use vibe_life::*;
pub use dropz::*;
pub use concurrenz::*;
pub use web_vibez::*;
pub use json_tea::*;
pub use regex_vibez::*;
pub use cryptz::*;
pub use reflectz::*;