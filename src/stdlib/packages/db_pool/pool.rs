/// fr fr Connection pool implementation

use std::time::Duration;

/// fr fr Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
/// fr fr Connection pool
#[derive(Debug)]
pub struct ConnectionPool {
impl ConnectionPool {
    pub fn new(config: PoolConfig) -> Self {
        Self { config }
    }
impl Default for PoolConfig {
    fn default() -> Self {
        Self {
        }
    }
}
