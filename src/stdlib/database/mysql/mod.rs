/// fr fr MySQL database driver for CURSED SQLSlay
/// 
/// This module provides comprehensive MySQL database connectivity with:
/// - Real connection pooling using bb8 and mysql_async
/// - Full prepared statement support with parameter binding
/// - Transaction management with isolation levels
/// - Connection lifecycle management
/// - Proper error handling and type conversion
/// - Asynchronous and synchronous operation support

pub mod driver;
pub mod connection;
pub mod statement;
pub mod transaction;
pub mod types;
pub mod pool;
pub mod error;

// Re-export main types
pub use driver::{MySqlDriver, MySqlConfig};
pub use connection::MySqlConnection;
pub use statement::MySqlStatement;
pub use transaction::MySqlTransaction;
pub use types::{MySqlValue, convert_to_sql_value, convert_from_sql_value};
pub use pool::{MySqlPool, MySqlPoolConfig};
pub use error::{MySqlError, MySqlResult};

// Re-export implementations
pub mod simple_driver;
pub mod real_driver;
pub mod comprehensive_driver;
pub mod production_driver;

// Helper functions
pub use driver::{create_mysql_driver, parse_mysql_dsn};
pub use comprehensive_driver::{
    ComprehensiveMySqlDriver, MySqlConfig as ComprehensiveMySqlConfig,
    create_mysql_driver as create_comprehensive_mysql_driver,
    create_mysql_driver_with_config as create_comprehensive_mysql_driver_with_config,
    parse_mysql_dsn as parse_comprehensive_mysql_dsn,
};

// Production driver exports
pub use production_driver::{
    ProductionMySqlDriver, ProductionMySqlConfig, SslMode,
    create_production_mysql_driver, create_production_mysql_driver_with_config,
    SqlSanitizer, DriverHealthReport
};

/// fr fr Initialize MySQL support and register driver globally
pub fn init_mysql() -> Result<(), Error> {
    use crate::stdlib::database::driver::register_driver;
    
    let driver = Box::new(MySqlDriver::new());
    register_driver("mysql".to_string(), driver)
}

/// fr fr Create a new MySQL driver with default configuration
pub fn new_mysql_driver() -> MySqlDriver {
    MySqlDriver::new()
}

/// fr fr Create a MySQL driver with custom configuration
pub fn new_mysql_driver_with_config(config: MySqlConfig) -> MySqlDriver {
    MySqlDriver::with_config(config)
}
