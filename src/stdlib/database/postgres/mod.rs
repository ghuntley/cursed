/// PostgreSQL Driver for CURSED SQLSlay Database System  
/// 
/// A comprehensive PostgreSQL driver implementation using libpq FFI bindings.
/// This driver provides full PostgreSQL feature support including arrays, JSON,
/// custom types, COPY protocol, and connection pooling optimizations.
/// 
/// Features fr fr:
/// - Native libpq FFI bindings for optimal performance
/// - Prepared statement support with parameter binding
/// - Transaction support with savepoints and isolation levels  
/// - PostgreSQL-specific features (arrays, JSON, JSONB, custom types)
/// - Connection string parsing with PostgreSQL URI support
/// - Connection pooling with PostgreSQL-specific optimizations
/// - COPY protocol for bulk operations
/// - Comprehensive error handling with PostgreSQL error codes
/// - Thread-safe operations with proper connection management

pub mod driver;
pub mod connection;
pub mod statement;
pub mod transaction;
pub mod types;
pub mod ffi;
pub mod pool;
pub mod copy;
pub mod error;
pub mod config;

// Re-export main types for easy access
pub use driver::PostgreSQLDriver;
pub use connection::PostgreSQLConnection;
pub use statement::PostgreSQLStatement;
pub use transaction::PostgreSQLTransaction;
pub use types::{PostgreSQLType, PostgreSQLValue, ArrayType, JsonType};
pub use pool::{PostgreSQLPool, PostgreSQLPoolConfig};
pub use copy::{CopyManager, CopyFormat, CopyDirection};
pub use error::{PostgreSQLError, PostgreSQLErrorCode};
pub use config::{PostgreSQLConfig, ConnectionString};

/// fr fr Initialize the PostgreSQL driver and register it globally
pub fn init_postgres_driver() -> Result<(), super::DatabaseError> {
    let driver = Box::new(PostgreSQLDriver::new());
    super::register_driver("postgres".to_string(), driver)?;
    super::register_driver("postgresql".to_string(), Box::new(PostgreSQLDriver::new()))?;
    Ok(())
}

/// fr fr Auto-initialize PostgreSQL driver on first use
pub fn ensure_postgres_driver_registered() -> Result<(), super::DatabaseError> {
    use std::sync::Once;
    use std::sync::Mutex;
    
    static ONCE: Once = Once::new();
    static mut INIT_RESULT: Option<Result<(), super::DatabaseError>> = None;
    static INIT_MUTEX: Mutex<()> = Mutex::new(());
    
    ONCE.call_once(|| {
        let _guard = INIT_MUTEX.lock();
        unsafe {
            INIT_RESULT = Some(init_postgres_driver());
        }
    });
    
    unsafe {
        INIT_RESULT.as_ref().unwrap().clone()
    }
}
