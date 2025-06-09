/// fr fr Pool manager for coordinating multiple pools

/// fr fr Pool manager
#[derive(Debug)]
pub struct PoolManager {
    pools: std::collections::HashMap<String, super::ConnectionPool>,
}

/// fr fr Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub active_connections: usize,
    pub idle_connections: usize,
    pub total_connections: usize,
}

impl PoolManager {
    pub fn new() -> Self {
        Self {
            pools: std::collections::HashMap::new(),
        }
    }
}

impl Default for PoolManager {
    fn default() -> Self {
        Self::new()
    }
}
