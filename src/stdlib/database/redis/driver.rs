//! Redis driver implementation

use crate::error::CursedError;
use super::connection::RedisConnection;
use crate::stdlib::packages::IOError;

/// Result type for Redis operations
pub type RedisResult<T> = Result<T, CursedError>;

/// Redis driver
pub struct RedisDriver {
    config: RedisConfig,
    is_initialized: bool,
}

/// Redis configuration
#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub database: u8,
    pub timeout: u64,
}

impl RedisDriver {
    /// Create a new Redis driver
    pub fn new(config: RedisConfig) -> Self {
        Self {
            config,
            is_initialized: false,
        }
    }
    
    /// Initialize the driver
    pub fn initialize(&mut self) -> RedisResult<()> {
        println!("🔴 Initializing Redis driver for {}:{}", self.config.host, self.config.port);
        self.is_initialized = true;
        Ok(())
    }
    
    /// Connect to Redis
    pub fn connect(&self) -> RedisResult<RedisConnection> {
        if !self.is_initialized {
            return Err(CursedError::runtime_error(&"Driver not initialized".to_string()));
        }
        
        let connection_string = format!("redis://{}:{}/{}", 
            self.config.host, self.config.port, self.config.database);
        
        let mut conn = RedisConnection::new(connection_string);
        conn.connect()?;
        Ok(conn)
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 6379,
            password: None,
            database: 0,
            timeout: 30,
        }
    }
}
