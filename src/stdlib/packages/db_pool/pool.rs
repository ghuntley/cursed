/// fr fr Connection pool implementation

use std::time::Duration;

/// fr fr Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub min_size: usize,
    pub max_size: usize,
    pub timeout: Duration,
}

/// fr fr Connection pool
#[derive(Debug)]
pub struct ConnectionPool {
    config: PoolConfig,
}

impl ConnectionPool {
    pub fn new(config: PoolConfig) -> Self {
        Self { config }
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            min_size: 1,
            max_size: 10,
            timeout: Duration::from_secs(30),
        }
    }
}
