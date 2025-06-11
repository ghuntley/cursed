/// Process management and system interaction for CURSED
pub mod error;
pub mod core;
pub mod info;
pub mod control;
pub mod communication;
pub mod monitoring;
pub mod platform;

// Re-export public API
pub use error::*;
pub use core::*;
pub use info::*;
pub use control::*;
pub use communication::*;
pub use monitoring::*;
pub use platform::*;
