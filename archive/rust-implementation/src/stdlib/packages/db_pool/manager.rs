//! Functional implementation for manager

use crate::error::CursedError;
use super::pool::{ConnectionPool, PoolConfig};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::stdlib::packages::ModuleError;

/// Result type for manager operations
pub type ModuleResult<T> = Result<T, CursedError>;

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub idle_connections: usize,
    pub total_requests: u64,
    pub failed_requests: u64,
    pub avg_response_time: f64,
}

impl Default for PoolStats {
    fn default() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            idle_connections: 0,
            total_requests: 0,
            failed_requests: 0,
            avg_response_time: 0.0,
        }
    }
}

/// Pool manager for managing multiple connection pools
pub struct PoolManager {
    pools: HashMap<String, Arc<Mutex<dyn PoolManagerTrait>>>,
    stats: Arc<Mutex<HashMap<String, PoolStats>>>,
}

/// Trait for pool management operations
pub trait PoolManagerTrait: Send + Sync {
    fn get_stats(&self) -> PoolStats;
    fn health_check(&self) -> bool;
    fn cleanup(&mut self) -> ModuleResult<()>;
}

impl PoolManager {
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn register_pool(&mut self, name: String, pool: Arc<Mutex<dyn PoolManagerTrait>>) -> ModuleResult<()> {
        self.pools.insert(name.clone(), pool);
        let mut stats = self.stats.lock()
            .map_err(|_| ModuleError::Other("Failed to acquire stats lock".to_string()))?;
        stats.insert(name, PoolStats::default());
        Ok(())
    }
    
    pub fn get_pool_stats(&self, name: &str) -> ModuleResult<PoolStats> {
        let stats = self.stats.lock()
            .map_err(|_| ModuleError::Other("Failed to acquire stats lock".to_string()))?;
        stats.get(name)
            .cloned()
            .ok_or_else(|| ModuleError::Other(format!("Pool '{}' not found", "placeholder")).into())
    }
    
    pub fn health_check_all(&self) -> ModuleResult<HashMap<String, bool>> {
        let mut results = HashMap::new();
        
        for (name, pool) in &self.pools {
            let pool = pool.lock()
                .map_err(|_| ModuleError::Other("Failed to acquire pool lock".to_string()))?;
            results.insert(name.clone(), pool.health_check());
        }
        
        Ok(results)
    }
    
    pub fn cleanup_all(&mut self) -> ModuleResult<()> {
        for (name, pool) in &self.pools {
            let mut pool = pool.lock()
                .map_err(|_| ModuleError::Other("Failed to acquire pool lock".to_string()))?;
            pool.cleanup().map_err(|e| {
                ModuleError::Other(format!("Failed to cleanup pool '{}': {}", name, e))
            })?;
        }
        Ok(())
    }
    
    pub fn update_stats(&self, pool_name: &str, stats: PoolStats) -> ModuleResult<()> {
        let mut stats_map = self.stats.lock()
            .map_err(|_| ModuleError::Other("Failed to acquire stats lock".to_string()))?;
        stats_map.insert(pool_name.to_string(), stats);
        Ok(())
    }
    
    pub fn get_all_stats(&self) -> ModuleResult<HashMap<String, PoolStats>> {
        let stats = self.stats.lock()
            .map_err(|_| ModuleError::Other("Failed to acquire stats lock".to_string()))?;
        Ok(stats.clone())
    }
}

/// manager operations handler
pub struct ModuleHandler {
    enabled: bool,
}

impl ModuleHandler {
    /// Create a new module handler
    pub fn new() -> Self {
        Self {
            enabled: true,
        }
    }
    
    /// Enable or disable the module
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
    
    /// Check if module is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Process data
    pub fn process(&self, data: &str) -> ModuleResult<String> {
        if !self.enabled {
            return Err(CursedError::runtime_error(&"Module is disabled"));
        }
        Ok(format!("Processed: {}", data))
    }
    
    /// Get module info
    pub fn info(&self) -> String {
        format!("Module: manager, Enabled: {}", self.enabled)
    }
}

impl Default for ModuleHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize manager processing
pub fn init_manager() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("test")?;
    if !result.contains("test") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    println!("⚙️  Module processing (manager) initialized");
    Ok(())
}

/// Test manager functionality
pub fn test_manager() -> ModuleResult<()> {
    let handler = ModuleHandler::new();
    let result = handler.process("Hello, CURSED!")?;
    if !result.contains("Hello, CURSED!") {
        return Err(CursedError::runtime_error(&"Module test failed"));
    }
    Ok(())
}
