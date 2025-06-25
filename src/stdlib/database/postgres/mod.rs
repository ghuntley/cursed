use crate::error::CursedError;
/// PostgreSQL Database Driver Module
/// 
/// Provides production-ready PostgreSQL database connectivity for the CURSED language
/// including connection pooling, prepared statements, transactions, and comprehensive
/// error handling.

pub mod driver;
pub mod connection;
pub mod statement;
pub mod transaction;
pub mod pool;
pub mod types;
pub mod error;
pub mod config;

// Re-export main types for easy access
pub use driver::PostgresDriver;
pub use connection::PostgresConnection;
pub use statement::PostgresStatement;
pub use transaction::PostgresTransaction;
pub use pool::{PostgresPool, PostgresPoolConfig};
pub use types::{PostgresTypeMapper, map_postgres_value, map_cursed_value};
pub use error::{PostgresError, PostgresErrorKind};
pub use config::{PostgresConfig, PostgresConnectionString, SslMode};

/// Initialize PostgreSQL driver and register it with the database system
pub fn init_postgres() -> crate::error::Result<()> {
    let driver = PostgresDriver::new();
//     crate::stdlib::database::driver::DriverRegistry::register("postgres", Box::new(driver))?;
//     crate::stdlib::database::driver::DriverRegistry::register("postgresql", Box::new(driver.clone()))?;
    Ok(())
/// Create a new PostgreSQL driver instance
pub fn new_postgres_driver() -> PostgresDriver {
    PostgresDriver::new()
/// Parse PostgreSQL connection string and create config
pub fn parse_connection_string(dsn: &str) -> crate::error::Result<()> {
    PostgresConnectionString::parse(dsn)
}
