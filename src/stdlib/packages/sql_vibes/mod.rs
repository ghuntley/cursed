use crate::error::CursedError;
/// fr fr SQL database driver implementation for CURSED language - sql_vibes package
/// This module provides no-cap database functionality with Gen Z energy periodt

// Core modules
pub mod simple_driver;
pub mod types;
pub mod error;

// Re-export core types for easy access - periodt
pub use simple_driver::{SimpleConnection, connect, quick_query};
pub use types::{SqlValue, SqlType, Row, ResultSet, Parameter, ParameterBinding};
pub use error::{SqlError, SqlResult, DatabaseErrorKind, QueryErrorKind, ConnectionErrorKind};



/// fr fr Initialize the sql_vibes package and register it with stdlib
pub fn init_sql_vibes() {
    println!("🗄️ sql_vibes package initialized - ready to store some data bestie!");
    // TODO: implement
}
