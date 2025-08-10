/// fr fr Database connection pooling - keeping connections fresh periodt
///
/// This package provides connection pooling, load balancing, and connection
/// management for optimal database performance. Pool party bestie!

// Core pooling modules
pub mod pool;
pub mod manager;
pub mod balancer;

// Re-export important types
pub use pool::{ConnectionPool, PoolConfig};
pub use manager::{PoolManager, PoolStats};

/// slay Initialize the db_pool package
pub fn init_db_pool() -> Result<(), String> {
    println!("🏊 db_pool package initialized - connection pooling ready bestie!");
    Ok(())
}
