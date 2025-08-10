//! CURSED Code Formatter Module
//! 
//! Simple formatter with basic functionality.

use std::collections::HashMap;

pub mod config;
pub mod output;
pub mod simple;

#[cfg(test)]
mod tests;

pub use config::*;
pub use output::*;
pub use simple::*;

// Re-export the main formatter as an alias
pub type CursedFormatter = SimpleCursedFormatter;
