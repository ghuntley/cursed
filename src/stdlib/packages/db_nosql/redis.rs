/// fr fr Redis driver for CURSED - blazing fast key-value operations periodt
///
/// This module provides a comprehensive Redis database driver for the CURSED
/// programming language, supporting all major Redis operations with connection
/// pooling, async operations, and seamless CURSED Value integration.

use async_trait::async_trait;
use redis::{self, aio::ConnectionManager, AsyncCommands, Client, FromRedisValue, RedisResult, ToRedisArgs};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use base64;
use crate::stdlib::packages::db_core::error::{DatabaseError, DatabaseResult, ConnectionError, ErrorKind};
use crate::stdlib::value::Value;
use super::drivers::{NoSqlDriver, NoSqlConnection};

/// fr fr Redis configuration - customize your connection bestie!
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Connection URL (e.g., redis://localhost:6379)
    pub url: String,
    /// Database number (0-15)
    pub database: u8,
    /// Username for authentication (Redis 6.0+)
    pub username: Option<String>,
    /// Password for authentication
    pub password: Option<String>,
    /// Connection timeout in milliseconds
    pub connection_timeout: u64,
    /// Response timeout in milliseconds
    pub response_timeout: u64,
    /// Maximum number of connections in pool
    pub max_connections: u32,
    /// Minimum number of connections in pool
    pub min_connections: u32,
    /// Connection idle timeout in seconds
    pub idle_timeout: u64,
    /// Enable SSL/TLS
    pub use_tls: bool,
    /// SSL certificate validation
    pub verify_ssl: bool,
    /// Connection retry attempts
    pub retry_attempts: u32,
    /// Retry delay in milliseconds
    pub retry_delay: u64,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            database: 0,
            username: None,
            password: None,
            connection_timeout: 5000,
            response_timeout: 30000,
            max_connections: 10,
            min_connections: 1,
            idle_timeout: 300,
            use_tls: false,
            verify_ssl: true,
            retry_attempts: 3,
            retry_delay: 1000,
        }
    }
}

/// fr fr Redis operation statistics - track your performance bestie!
#[derive(Debug, Clone, Default)]
pub struct RedisStats {
    /// Total operations performed
    pub total_operations: u64,
    /// Successful operations
    pub successful_operations: u64,
    /// Failed operations
    pub failed_operations: u64,
    /// Average response time in microseconds
    pub avg_response_time_us: u64,
    /// Total connections created
    pub connections_created: u64,
    /// Active connections
    pub active_connections: u32,
    /// Connection pool hits
    pub pool_hits: u64,
    /// Connection pool misses
    pub pool_misses: u64,
}

/// fr fr Redis connection pool - efficient connection management
#[derive(Debug)]
pub struct RedisConnectionPool {
    client: Client,
    manager: Arc<Mutex<Option<ConnectionManager>>>,
    config: RedisConfig,
    stats: Arc<RwLock<RedisStats>>,
    created_at: Instant,
}

impl RedisConnectionPool {
    /// slay Create a new Redis connection pool
    pub async fn new(config: RedisConfig) -> DatabaseResult<Self> {
        let client = Client::open(config.url.as_ref())
            .map_err(|e| DatabaseError::connection(
                ConnectionError::InvalidConnectionString,
                &format!("Invalid Redis URL: {}", e)
            ))?;

        Ok(Self {
            client,
            manager: Arc::new(Mutex::new(None)),
            config,
            stats: Arc::new(RwLock::new(RedisStats::default())),
            created_at: Instant::now(),
        })
    }

    /// slay Get a connection from the pool
    pub async fn get_connection(&self) -> DatabaseResult<ConnectionManager> {
        let mut manager_guard = self.manager.lock().await;
        
        if let Some(manager) = manager_guard.as_ref() {
            let mut stats = self.stats.write().await;
            stats.pool_hits += 1;
            return Ok(manager.clone());
        }

        // Create new connection manager
        let manager = self.client
            .get_connection_manager()
            .await
            .map_err(|e| DatabaseError::connection(
                ConnectionError::FailedToConnect,
                &format!("Failed to create Redis connection manager: {}", e)
            ))?;

        *manager_guard = Some(manager.clone());
        
        let mut stats = self.stats.write().await;
        stats.connections_created += 1;
        stats.active_connections += 1;
        stats.pool_misses += 1;

        Ok(manager)
    }

    /// slay Get connection statistics
    pub async fn get_stats(&self) -> RedisStats {
        self.stats.read().await.clone()
    }

    /// slay Update operation statistics
    pub async fn update_stats(&self, success: bool, duration: Duration) {
        let mut stats = self.stats.write().await;
        stats.total_operations += 1;
        
        if success {
            stats.successful_operations += 1;
        } else {
            stats.failed_operations += 1;
        }

        // Update average response time using running average
        let new_time_us = duration.as_micros() as u64;
        if stats.total_operations == 1 {
            stats.avg_response_time_us = new_time_us;
        } else {
            stats.avg_response_time_us = 
                (stats.avg_response_time_us * (stats.total_operations - 1) + new_time_us) / stats.total_operations;
        }
    }
}

/// fr fr Redis driver implementation - the main Redis driver bestie!
#[derive(Debug)]
pub struct RedisDriver {
    config: RedisConfig,
}

impl RedisDriver {
    /// slay Create a new Redis driver
    pub fn new() -> Self {
        Self {
            config: RedisConfig::default(),
        }
    }

    /// slay Create a new Redis driver with configuration
    pub fn with_config(config: RedisConfig) -> Self {
        Self { config }
    }

    /// slay Create Redis driver from connection string
    pub fn from_url(url: &str) -> DatabaseResult<Self> {
        let mut config = RedisConfig::default();
        config.url = url.to_string();
        Ok(Self { config })
    }
}

impl Default for RedisDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NoSqlDriver for RedisDriver {
    async fn connect(&self, connection_string: &str) -> Result<(), Error> {
        let mut config = self.config.clone();
        if !connection_string.is_empty() {
            config.url = connection_string.to_string();
        }

        let pool = RedisConnectionPool::new(config).await?;
        let connection = RedisConnection::new(pool).await?;
        Ok(Box::new(connection))
    }
}

/// fr fr Redis connection implementation - execute Redis commands like a pro bestie!
#[derive(Debug)]
pub struct RedisConnection {
    pool: Arc<RedisConnectionPool>,
    connection: ConnectionManager,
}

impl RedisConnection {
    /// slay Create a new Redis connection
    pub async fn new(pool: RedisConnectionPool) -> DatabaseResult<Self> {
        let connection = pool.get_connection().await?;
        Ok(Self {
            pool: Arc::new(pool),
            connection,
        })
    }

    /// slay Execute a Redis command with timing and update stats
    async fn update_stats_and_handle_error<T>(&self, result: RedisResult<T>, duration: std::time::Duration) -> DatabaseResult<T> {
        let success = result.is_ok();
        self.pool.update_stats(success, duration).await;

        result.map_err(|e| DatabaseError::query(
            crate::stdlib::packages::db_core::error::QueryError::ExecutionFailed,
            &format!("Redis operation failed: {}", e)
        ))
    }

    /// slay Execute a Redis operation with timing and stats tracking
    async fn execute_with_timing<F, Fut, T>(&mut self, operation: F) -> DatabaseResult<T>
    where
        F: FnOnce(&mut ConnectionManager) -> Fut,
        Fut: std::future::Future<Output = RedisResult<T>>,
    {
        let start = std::time::Instant::now();
        let result = operation(&mut self.connection).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    // === Core Redis Operations ===

    /// slay GET - Retrieve value by key
    pub async fn get(&mut self, key: &str) -> DatabaseResult<Option<Value>> {
        let start = std::time::Instant::now();
        let result: RedisResult<Option<String>> = self.connection.get(key).await;
        let duration = start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;

        Ok(result.map(Value::string))
    }

    /// slay SET - Set key-value pair
    pub async fn set(&mut self, key: &str, value: &Value) -> DatabaseResult<()> {
        let redis_value = value_to_redis_string(value);
        let start = std::time::Instant::now();
        let result = self.connection.set(key, redis_value).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay SET with expiration - Set key-value pair with TTL
    pub async fn set_ex(&mut self, key: &str, value: &Value, seconds: u64) -> DatabaseResult<()> {
        let redis_value = value_to_redis_string(value);
        let start = std::time::Instant::now();
        let result = self.connection.set_ex(key, redis_value, seconds).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay DEL - Delete keys
    pub async fn del(&mut self, keys: &[&str]) -> DatabaseResult<u64> {
        let keys: Vec<String> = keys.iter().map(|&s| s.to_string()).collect();
        let start = std::time::Instant::now();
        let result = self.connection.del(keys).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay EXISTS - Check if key exists
    pub async fn exists(&mut self, key: &str) -> DatabaseResult<bool> {
        let start = std::time::Instant::now();
        let result: RedisResult<i32> = self.connection.exists(key).await;
        let duration = start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        Ok(result > 0)
    }

    /// slay EXPIRE - Set key expiration
    pub async fn expire(&mut self, key: &str, seconds: u64) -> DatabaseResult<bool> {
        let start = std::time::Instant::now();
        let result: RedisResult<i32> = self.connection.expire(key, seconds as i64).await;
        let duration = start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        Ok(result > 0)
    }

    /// slay TTL - Get key time to live
    pub async fn ttl(&mut self, key: &str) -> DatabaseResult<i64> {
        let key = key.to_string();
        
       
        let start = std::time::Instant::now();
        let result = self.connection.ttl(key).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay INCR - Increment integer value
    pub async fn incr(&mut self, key: &str) -> DatabaseResult<i64> {
        let key = key.to_string();
        
       
        let start = std::time::Instant::now();
        let result = self.connection.incr(key, 1).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay INCRBY - Increment by amount
    pub async fn incr_by(&mut self, key: &str, increment: i64) -> DatabaseResult<i64> {
        let key = key.to_string();
        
       
        let start = std::time::Instant::now();
        let result = self.connection.incr(key, increment).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay DECR - Decrement integer value
    pub async fn decr(&mut self, key: &str) -> DatabaseResult<i64> {
        let key = key.to_string();
        
       
        let start = std::time::Instant::now();
        let result = self.connection.decr(key, 1).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay DECRBY - Decrement by amount
    pub async fn decr_by(&mut self, key: &str, decrement: i64) -> DatabaseResult<i64> {
        let key = key.to_string();
        
       
        let start = std::time::Instant::now();
        let result = self.connection.decr(key, decrement).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    // === List Operations ===

    /// slay LPUSH - Push to left of list
    pub async fn lpush(&mut self, key: &str, values: &[Value]) -> DatabaseResult<u64> {
        let key = key.to_string();
        let redis_values: Vec<String> = values.iter().map(value_to_redis_string).collect();
        
        let key = key.to_string();
        let start = std::time::Instant::now();
        let result = self.connection.lpush(key, redis_values).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay RPUSH - Push to right of list
    pub async fn rpush(&mut self, key: &str, values: &[Value]) -> DatabaseResult<u64> {
        let key = key.to_string();
        let redis_values: Vec<String> = values.iter().map(value_to_redis_string).collect();
        
        let key = key.to_string();
        let start = std::time::Instant::now();
        let result = self.connection.rpush(key, redis_values).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay LPOP - Pop from left of list
    pub async fn lpop(&mut self, key: &str) -> DatabaseResult<Option<Value>> {
        let start = std::time::Instant::now();
        let result: RedisResult<Option<String>> = self.connection.lpop(key, None).await;
        let duration = start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        Ok(result.map(Value::string))
    }

    /// slay RPOP - Pop from right of list
    pub async fn rpop(&mut self, key: &str) -> DatabaseResult<Option<Value>> {
        let start = std::time::Instant::now();
        let result: RedisResult<Option<String>> = self.connection.rpop(key, None).await;
        let duration = start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        Ok(result.map(Value::string))
    }

    /// slay LLEN - Get list length
    pub async fn llen(&mut self, key: &str) -> DatabaseResult<u64> {
        let key = key.to_string();
        let start = std::time::Instant::now();
        let result = self.connection.llen(key).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay LRANGE - Get list range
    pub async fn lrange(&mut self, key: &str, start: i64, stop: i64) -> DatabaseResult<Vec<Value>> {
        let timer_start = std::time::Instant::now();
        let result: RedisResult<Vec<String>> = self.connection.lrange(key, start as isize, stop as isize).await;
        let duration = timer_start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        Ok(result.into_iter().map(Value::string).collect())
    }

    // === Set Operations ===

    /// slay SADD - Add members to set
    pub async fn sadd(&mut self, key: &str, members: &[Value]) -> DatabaseResult<u64> {
        let redis_values: Vec<String> = members.iter().map(value_to_redis_string).collect();
        
        let key = key.to_string();
        let start = std::time::Instant::now();
        let result = self.connection.sadd(key, redis_values).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay SREM - Remove members from set
    pub async fn srem(&mut self, key: &str, members: &[Value]) -> DatabaseResult<u64> {
        let redis_values: Vec<String> = members.iter().map(value_to_redis_string).collect();
        
        let key = key.to_string();
        let start = std::time::Instant::now();
        let result = self.connection.srem(key, redis_values).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay SMEMBERS - Get all set members
    pub async fn smembers(&mut self, key: &str) -> DatabaseResult<Vec<Value>> {
        let key = key.to_string();
        let timer_start = std::time::Instant::now();
        let result: RedisResult<Vec<String>> = self.connection.smembers(key).await;
        let duration = timer_start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        Ok(result.into_iter().map(Value::string).collect())
    }

    /// slay SISMEMBER - Check if member exists in set
    pub async fn sismember(&mut self, key: &str, member: &Value) -> DatabaseResult<bool> {
        let redis_value = value_to_redis_string(member);
        let key = key.to_string();
        let timer_start = std::time::Instant::now();
        let result: RedisResult<bool> = self.connection.sismember(key, redis_value).await;
        let duration = timer_start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        Ok(result)
    }

    /// slay SCARD - Get set cardinality
    pub async fn scard(&mut self, key: &str) -> DatabaseResult<u64> {
        let key = key.to_string();
        let start = std::time::Instant::now();
        let result = self.connection.scard(key).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    // === Hash Operations ===

    /// slay HSET - Set hash field
    pub async fn hset(&mut self, key: &str, field: &str, value: &Value) -> DatabaseResult<bool> {
        let redis_value = value_to_redis_string(value);
        let key = key.to_string();
       
        let timer_start = std::time::Instant::now();
        let result: RedisResult<i32> = self.connection.hset(key, field, redis_value).await;
        let duration = timer_start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        Ok(result > 0)
    }

    /// slay HGET - Get hash field
    pub async fn hget(&mut self, key: &str, field: &str) -> DatabaseResult<Option<Value>> {
        let key = key.to_string();
       
        let timer_start = std::time::Instant::now();
        let result: RedisResult<Option<String>> = self.connection.hget(key, field).await;
        let duration = timer_start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        Ok(result.map(Value::string))
    }

    /// slay HDEL - Delete hash fields
    pub async fn hdel(&mut self, key: &str, fields: &[&str]) -> DatabaseResult<u64> {
        
        let key = key.to_string();
        
       
        let fields: Vec<String> = fields.iter().map(|&s| s.to_string()).collect();
        let start = std::time::Instant::now();
        let result = self.connection.hdel(key, fields).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay HEXISTS - Check if hash field exists
    pub async fn hexists(&mut self, key: &str, field: &str) -> DatabaseResult<bool> {
        
        let key = key.to_string();
        
       
        let field = field.to_string();
        let start = std::time::Instant::now();
        let result = self.connection.hexists(key, field).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay HGETALL - Get all hash fields and values
    pub async fn hgetall(&mut self, key: &str) -> DatabaseResult<HashMap<String, Value>> {
        let key = key.to_string();
        let timer_start = std::time::Instant::now();
        let result: RedisResult<HashMap<String, String>> = self.connection.hgetall(key).await;
        let duration = timer_start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        
        Ok(result.into_iter()
            .map(|(k, v)| (k, Value::string(v)))
            .collect())
    }

    /// slay HKEYS - Get all hash field names
    pub async fn hkeys(&mut self, key: &str) -> DatabaseResult<Vec<String>> {
        
        let key = key.to_string();
        let start = std::time::Instant::now();
        let result = self.connection.hkeys(key).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay HVALS - Get all hash values
    pub async fn hvals(&mut self, key: &str) -> DatabaseResult<Vec<Value>> {
        let key = key.to_string();
        let timer_start = std::time::Instant::now();
        let result: RedisResult<Vec<String>> = self.connection.hvals(key).await;
        let duration = timer_start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        Ok(result.into_iter().map(Value::string).collect())
    }

    /// slay HLEN - Get hash length
    pub async fn hlen(&mut self, key: &str) -> DatabaseResult<u64> {
        let key = key.to_string();
        let start = std::time::Instant::now();
        let result = self.connection.hlen(key).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    // === Advanced Operations ===

    /// slay KEYS - Find keys matching pattern
    pub async fn keys(&mut self, pattern: &str) -> DatabaseResult<Vec<String>> {
        let pattern = pattern.to_string();
        let start = std::time::Instant::now();
        let result = self.connection.keys(pattern).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay SCAN - Incrementally iterate keys
    pub async fn scan(&mut self, cursor: u64, pattern: Option<&str>, count: Option<u64>) -> DatabaseResult<(u64, Vec<String>)> {
        let mut cmd = redis::cmd("SCAN");
        cmd.arg(cursor);
        
        if let Some(p) = pattern {
            cmd.arg("MATCH").arg(p);
        }
        
        if let Some(c) = count {
            cmd.arg("COUNT").arg(c);
        }

        let timer_start = std::time::Instant::now();
        let result: RedisResult<(u64, Vec<String>)> = cmd.query_async(&mut self.connection).await;
        let duration = timer_start.elapsed();
        let result = self.update_stats_and_handle_error(result, duration).await?;
        
        Ok(result)
    }

    /// slay FLUSHDB - Clear current database
    pub async fn flushdb(&mut self) -> DatabaseResult<()> {
        let start = std::time::Instant::now();
        let result = redis::cmd("FLUSHDB").query_async(&mut self.connection).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay INFO - Get Redis server information
    pub async fn info(&mut self, section: Option<&str>) -> DatabaseResult<String> {
        let mut cmd = redis::cmd("INFO");
        if let Some(s) = section {
            cmd.arg(s);
        }
        
        let start = std::time::Instant::now();
        let result = cmd.query_async(&mut self.connection).await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await
    }

    /// slay PING - Test connection
    pub async fn ping(&mut self, message: Option<&str>) -> DatabaseResult<String> {
        let result: String = if let Some(msg) = message {
            self.execute_with_timing(
            |conn| redis::cmd("PING").arg(msg).query_async(conn)
        ).await?
        } else {
            let start = std::time::Instant::now();
        let result = self.connection.get("PING").await;
        let duration = start.elapsed();
        self.update_stats_and_handle_error(result, duration).await.unwrap_or_else(|_| "PONG".to_string())
        };
        
        Ok(result)
    }

    /// slay Get connection statistics
    pub async fn get_stats(&self) -> RedisStats {
        self.pool.get_stats().await
    }
}

#[async_trait]
impl NoSqlConnection for RedisConnection {
    /// slay Insert document (stored as JSON string)
    async fn insert(&mut self, collection: &str, document: serde_json::Value) -> Result<(), Error> {
        let doc_json = document.to_string();
        let key = format!("{}:{}", collection, Uuid::new_v4());
        
        self.set(&key, &Value::string(doc_json)).await?;
        Ok(key)
    }
    
    /// slay Find documents (basic key pattern matching)
    async fn find(&mut self, collection: &str, _query: serde_json::Value) -> Result<(), Error> {
        let pattern = format!("{}:*", collection);
        let keys = self.keys(&pattern).await?;
        
        let mut documents = Vec::new();
        for key in keys {
            if let Some(value) = self.get(&key).await? {
                if let Some(json_str) = value.as_string() {
                    if let Ok(doc) = serde_json::from_str(json_str) {
                        documents.push(doc);
                    }
                }
            }
        }
        
        Ok(documents)
    }
    
    /// slay Get underlying type for downcasting
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    /// slay Get mutable underlying type for downcasting
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// === Helper Functions ===

/// slay Convert CURSED Value to Redis string representation
fn value_to_redis_string(value: &Value) -> String {
    match value {
        Value::Null => "".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Integer(i) => i.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => serde_json::to_string(arr).unwrap_or_default(),
        Value::Object(obj) => serde_json::to_string(obj).unwrap_or_default(),
        Value::Bytes(bytes) => base64::engine::general_purpose::STANDARD.encode(bytes),
    }
}

/// slay Parse Redis string back to CURSED Value
pub fn redis_string_to_value(redis_value: &str) -> Value {
    // Try to parse as JSON first
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(redis_value) {
        return json_to_cursed_value(&json_value);
    }
    
    // Try to parse as number
    if let Ok(int_val) = redis_value.parse::<i64>() {
        return Value::Integer(int_val);
    }
    
    if let Ok(float_val) = redis_value.parse::<f64>() {
        return Value::Number(float_val);
    }
    
    // Try to parse as boolean
    match redis_value.to_lowercase().as_str() {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        _ => Value::String(redis_value.to_string()),
    }
}

/// slay Convert serde_json::Value to CURSED Value
fn json_to_cursed_value(json: &serde_json::Value) -> Value {
    match json {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                Value::Number(f)
            } else {
                Value::Null
            }
        }
        serde_json::Value::String(s) => Value::String(s.clone()),
        serde_json::Value::Array(arr) => {
            let values: Vec<Value> = arr.iter().map(json_to_cursed_value).collect();
            Value::Array(values)
        }
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::new();
            for (k, v) in obj {
                map.insert(k.clone(), json_to_cursed_value(v));
            }
            Value::Object(map)
        }
    }
}

// === Redis Configuration Helpers ===

impl RedisConfig {
    /// slay Create configuration from URL
    pub fn from_url(url: &str) -> DatabaseResult<Self> {
        let mut config = Self::default();
        config.url = url.to_string();
        
        // Parse URL for additional configuration
        if let Ok(parsed_url) = url::Url::parse(url) {
            if let Some(password) = parsed_url.password() {
                config.password = Some(password.to_string());
            }
            
            if !parsed_url.username().is_empty() {
                config.username = Some(parsed_url.username().to_string());
            }
            
            // Parse query parameters for additional config
            for (key, value) in parsed_url.query_pairs() {
                match key.as_ref() {
                    "db" => {
                        config.database = value.parse().unwrap_or(0);
                    }
                    "connection_timeout" => {
                        config.connection_timeout = value.parse().unwrap_or(5000);
                    }
                    "response_timeout" => {
                        config.response_timeout = value.parse().unwrap_or(30000);
                    }
                    _ => {}
                }
            }
        }
        
        Ok(config)
    }

    /// slay Validate configuration
    pub fn validate(&self) -> DatabaseResult<()> {
        if self.url.is_empty() {
            return Err(DatabaseError::config("Redis URL cannot be empty"));
        }
        
        if self.max_connections == 0 {
            return Err(DatabaseError::config("Max connections must be greater than 0"));
        }
        
        if self.min_connections > self.max_connections {
            return Err(DatabaseError::config("Min connections cannot be greater than max connections"));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_redis_config_default() {
        let config = RedisConfig::default();
        assert_eq!(config.url, "redis://localhost:6379");
        assert_eq!(config.database, 0);
        assert_eq!(config.max_connections, 10);
    }

    #[test]
    fn test_redis_config_from_url() {
        let config = RedisConfig::from_url("redis://user:pass@localhost:6380/1?connection_timeout=10000").unwrap();
        assert_eq!(config.username, Some("user".to_string()));
        assert_eq!(config.password, Some("pass".to_string()));
        assert_eq!(config.database, 1);
        assert_eq!(config.connection_timeout, 10000);
    }

    #[test]
    fn test_redis_config_validation() {
        let mut config = RedisConfig::default();
        assert!(config.validate().is_ok());
        
        config.url = "".to_string();
        assert!(config.validate().is_err());
        
        config.url = "redis://localhost".to_string();
        config.max_connections = 0;
        assert!(config.validate().is_err());
        
        config.max_connections = 10;
        config.min_connections = 20;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_value_to_redis_string() {
        assert_eq!(value_to_redis_string(&Value::Null), "");
        assert_eq!(value_to_redis_string(&Value::Bool(true)), "true");
        assert_eq!(value_to_redis_string(&Value::Integer(42)), "42");
        assert_eq!(value_to_redis_string(&Value::Number(3.14)), "3.14");
        assert_eq!(value_to_redis_string(&Value::String("hello".to_string())), "hello");
    }

    #[test]
    fn test_redis_string_to_value() {
        assert_eq!(redis_string_to_value("42"), Value::Integer(42));
        assert_eq!(redis_string_to_value("3.14"), Value::Number(3.14));
        assert_eq!(redis_string_to_value("true"), Value::Bool(true));
        assert_eq!(redis_string_to_value("false"), Value::Bool(false));
        assert_eq!(redis_string_to_value("hello"), Value::String("hello".to_string()));
    }

    #[test]
    fn test_json_to_cursed_value() {
        let json = serde_json::json!({
            "name": "test",
            "age": 25,
            "active": true,
            "tags": ["a", "b"],
            "meta": null
        });
        
        let value = json_to_cursed_value(&json);
        assert!(value.is_object());
        
        if let Value::Object(obj) = value {
            assert_eq!(obj.get("name"), Some(&Value::String("test".to_string())));
            assert_eq!(obj.get("age"), Some(&Value::Integer(25)));
            assert_eq!(obj.get("active"), Some(&Value::Bool(true)));
            assert_eq!(obj.get("meta"), Some(&Value::Null));
        }
    }

    #[tokio::test]
    async fn test_redis_driver_creation() {
        let driver = RedisDriver::new();
        assert_eq!(driver.config.url, "redis://localhost:6379");
        
        let driver_with_config = RedisDriver::with_config(RedisConfig {
            url: "redis://example.com:6379".to_string(),
            ..Default::default()
        });
        assert_eq!(driver_with_config.config.url, "redis://example.com:6379");
        
        let driver_from_url = RedisDriver::from_url("redis://test:1234").unwrap();
        assert_eq!(driver_from_url.config.url, "redis://test:1234");
    }
}
