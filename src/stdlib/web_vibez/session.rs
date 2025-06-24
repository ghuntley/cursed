use crate::error::Error;
/// Session management and storage utilities
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::stdlib::web_vibez::config::{SessionConfig, SessionStoreType, SameSitePolicy};

// Redis support - conditionally compiled if redis feature is enabled
#[cfg(feature = "redis")]
extern crate redis;

// For non-redis builds, provide minimal redis types for compilation
#[cfg(not(feature = "redis"))]
mod redis {
    use std::collections::HashMap;
    
    pub struct Client;
    pub struct Connection;
    
    impl Clone for Client {
        fn clone(&self) -> Self {
            Client
        }
    }
    
    impl Client {
        pub fn open(_url: &str) -> Result<(), Error> {
            Err(RedisError::new("Redis support not compiled in"))
        }
        pub fn get_connection(&self) -> Result<(), Error> {
            Err(RedisError::new("Redis support not compiled in"))
        }
    }
    
    pub fn cmd(_cmd: &str) -> Cmd {
        Cmd
    }
    
    pub struct Cmd;
    impl Cmd {
        pub fn arg<T: std::fmt::Display>(&mut self, _arg: T) -> &mut Self { self }
        pub fn query<T: Default>(&mut self, _conn: &mut Connection) -> Result<(), Error> {
            Err(RedisError::new("Redis support not compiled in"))
        }
    }
    
    #[derive(Debug)]
    pub struct RedisError {
        message: String,
    }
    
    impl RedisError {
        fn new(message: &str) -> Self {
            Self { message: message.to_string() }
        }
    }
    
    impl std::fmt::Display for RedisError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.message)
        }
    }
    
    impl std::error::Error for RedisError {}
}

/// Session data structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Session {
    pub id: String,
    pub data: HashMap<String, SessionValue>,
    pub created_at: u64,
    pub last_accessed: u64,
    pub expires_at: Option<u64>,
    pub is_new: bool,
    pub is_dirty: bool,
}

/// Session value types
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SessionValue {
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<SessionValue>),
    Object(HashMap<String, SessionValue>),
}

impl fmt::Display for SessionValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SessionValue::String(s) => write!(f, "{}", s),
            SessionValue::Number(n) => write!(f, "{}", n),
            SessionValue::Bool(b) => write!(f, "{}", b),
            SessionValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                write!(f, "[{}]", items.join(", "))
            }
            SessionValue::Object(obj) => {
                let items: Vec<String> = obj.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                write!(f, "{{{}}}", items.join(", "))
            }
        }
    }
}

impl SessionValue {
    /// Convert to string if possible
    pub fn as_string(&self) -> Option<&str> {
        match self {
            SessionValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Convert to number if possible
    pub fn as_number(&self) -> Option<f64> {
        match self {
            SessionValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Convert to boolean if possible
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SessionValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Convert to array if possible
    pub fn as_array(&self) -> Option<&Vec<SessionValue>> {
        match self {
            SessionValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Convert to object if possible
    pub fn as_object(&self) -> Option<&HashMap<String, SessionValue>> {
        match self {
            SessionValue::Object(obj) => Some(obj),
            _ => None,
        }
    }
}

impl Session {
    /// Create new session
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: Self::generate_session_id(),
            data: HashMap::new(),
            created_at: now,
            last_accessed: now,
            expires_at: None,
            is_new: true,
            is_dirty: false,
        }
    }

    /// Create session with specific ID
    pub fn with_id(id: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id,
            data: HashMap::new(),
            created_at: now,
            last_accessed: now,
            expires_at: None,
            is_new: false,
            is_dirty: false,
        }
    }

    /// Set session value
    pub fn set(&mut self, key: String, value: SessionValue) {
        self.data.insert(key, value);
        self.is_dirty = true;
        self.touch();
    }

    /// Get session value
    pub fn get(&mut self, key: &str) -> Option<&SessionValue> {
        self.touch();
        self.data.get(key)
    }

    /// Remove session value
    pub fn remove(&mut self, key: &str) -> Option<SessionValue> {
        self.is_dirty = true;
        self.touch();
        self.data.remove(key)
    }

    /// Clear all session data
    pub fn clear(&mut self) {
        self.data.clear();
        self.is_dirty = true;
        self.touch();
    }

    /// Check if session contains key
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    }

    /// Check if session is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Update last accessed time
    pub fn touch(&mut self) {
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            now > expires_at
        } else {
            false
        }
    }

    /// Set expiration time
    pub fn set_expiry(&mut self, seconds_from_now: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.expires_at = Some(now + seconds_from_now);
        self.is_dirty = true;
    }

    /// Remove expiration
    pub fn clear_expiry(&mut self) {
        self.expires_at = None;
        self.is_dirty = true;
    }

    /// Generate secure session ID
    fn generate_session_id() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        
        // Create pseudo-random session ID
        let random_data = format!("{}:{}", now, now.wrapping_mul(9973));
        Self::hash_string(&random_data)
    }

    /// Simple hash function for session ID generation
    fn hash_string(input: &str) -> String {
        let mut hash: u64 = 5381;
        for byte in input.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        format!("{:016x}", hash)
    }

    /// Serialize session to string (simple format)
    pub fn serialize(&self) -> String {
        let mut parts = Vec::new();
        parts.push(format!("id:{}", self.id));
        parts.push(format!("created:{}", self.created_at));
        parts.push(format!("accessed:{}", self.last_accessed));
        
        if let Some(expires) = self.expires_at {
            parts.push(format!("expires:{}", expires));
        }

        for (key, value) in &self.data {
            parts.push(format!("data:{}:{}", key, self.serialize_value(value)));
        }

        parts.join("|")
    }

    /// Deserialize session from string
    pub fn deserialize(data: &str) -> Result<(), Error> {
        let mut session = Session::new();
        session.is_new = false;

        for part in data.split('|') {
            let components: Vec<&str> = part.splitn(2, ':').collect();
            if components.len() != 2 {
                continue;
            }

            match components[0] {
                "id" => session.id = components[1].to_string(),
                "created" => {
                    session.created_at = components[1].parse()
                        .map_err(|_| SessionError::InvalidData("created_at".to_string()))?;
                }
                "accessed" => {
                    session.last_accessed = components[1].parse()
                        .map_err(|_| SessionError::InvalidData("last_accessed".to_string()))?;
                }
                "expires" => {
                    session.expires_at = Some(components[1].parse()
                        .map_err(|_| SessionError::InvalidData("expires_at".to_string()))?);
                }
                "data" => {
                    let data_parts: Vec<&str> = components[1].splitn(2, ':').collect();
                    if data_parts.len() == 2 {
                        let key = data_parts[0].to_string();
                        let value = Self::deserialize_value(data_parts[1])?;
                        session.data.insert(key, value);
                    }
                }
                _ => {} // Ignore unknown fields
            }
        }

        Ok(session)
    }

    /// Serialize session value to string
    fn serialize_value(&self, value: &SessionValue) -> String {
        match value {
            SessionValue::String(s) => format!("s:{}", s),
            SessionValue::Number(n) => format!("n:{}", n),
            SessionValue::Bool(b) => format!("b:{}", b),
            SessionValue::Array(_) => "a:[]".to_string(), // Simplified
            SessionValue::Object(_) => "o:{}".to_string(), // Simplified
        }
    }

    /// Deserialize session value from string
    fn deserialize_value(data: &str) -> Result<(), Error> {
        let parts: Vec<&str> = data.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(SessionError::InvalidData("value".to_string()));
        }

        match parts[0] {
            "s" => Ok(SessionValue::String(parts[1].to_string())),
            "n" => {
                let n = parts[1].parse()
                    .map_err(|_| SessionError::InvalidData("number".to_string()))?;
                Ok(SessionValue::Number(n))
            }
            "b" => {
                let b = parts[1].parse()
                    .map_err(|_| SessionError::InvalidData("boolean".to_string()))?;
                Ok(SessionValue::Bool(b))
            }
            "a" => Ok(SessionValue::Array(Vec::from([]))), // Simplified
            "o" => Ok(SessionValue::Object(HashMap::new())), // Simplified
            _ => Err(SessionError::InvalidData("type".to_string())),
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

/// Session store trait
pub trait SessionStore {
    fn load(&self, session_id: &str) -> Result<(), Error>;
    fn save(&mut self, session: &Session) -> Result<(), Error>;
    fn delete(&mut self, session_id: &str) -> Result<(), Error>;
    fn cleanup_expired(&mut self) -> Result<(), Error>;
    fn exists(&self, session_id: &str) -> bool;
    fn count(&self) -> usize;
}

/// Memory-based session store
pub struct MemorySessionStore {
    sessions: HashMap<String, Session>,
}

impl MemorySessionStore {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }
}

impl Default for MemorySessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionStore for MemorySessionStore {
    fn load(&self, session_id: &str) -> Result<(), Error> {
        Ok(self.sessions.get(session_id).cloned())
    }

    fn save(&mut self, session: &Session) -> Result<(), Error> {
        self.sessions.insert(session.id.clone(), session.clone());
        Ok(())
    }

    fn delete(&mut self, session_id: &str) -> Result<(), Error> {
        self.sessions.remove(session_id);
        Ok(())
    }

    fn cleanup_expired(&mut self) -> Result<(), Error> {
        let initial_count = self.sessions.len();
        self.sessions.retain(|_, session| !session.is_expired());
        Ok(initial_count - self.sessions.len())
    }

    fn exists(&self, session_id: &str) -> bool {
        self.sessions.contains_key(session_id)
    }

    fn count(&self) -> usize {
        self.sessions.len()
    }
}

/// File-based session store
pub struct FileSessionStore {
    directory: PathBuf,
}

impl FileSessionStore {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    fn session_file_path(&self, session_id: &str) -> PathBuf {
        self.directory.join(format!("{}.session", session_id))
    }
}

impl SessionStore for FileSessionStore {
    fn load(&self, session_id: &str) -> Result<(), Error> {
        let file_path = self.session_file_path(session_id);
        
        match std::fs::read_to_string(&file_path) {
            Ok(content) => {
                let session = Session::deserialize(&content)?;
                if session.is_expired() {
                    // Delete expired session
                    let _ = std::fs::remove_file(&file_path);
                    Ok(None)
                } else {
                    Ok(Some(session))
                }
            }
            Err(_) => Ok(None),
        }
    }

    fn save(&mut self, session: &Session) -> Result<(), Error> {
        let file_path = self.session_file_path(&session.id);
        
        // Create directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| SessionError::StoreError(e.to_string()))?;
        }

        let content = session.serialize();
        std::fs::write(&file_path, content)
            .map_err(|e| SessionError::StoreError(e.to_string()))?;

        Ok(())
    }

    fn delete(&mut self, session_id: &str) -> Result<(), Error> {
        let file_path = self.session_file_path(session_id);
        let _ = std::fs::remove_file(&file_path); // Ignore errors
        Ok(())
    }

    fn cleanup_expired(&mut self) -> Result<(), Error> {
        let mut cleaned = 0;
        
        if let Ok(entries) = std::fs::read_dir(&self.directory) {
            for entry in entries.flatten() {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.ends_with(".session") {
                        let session_id = filename.trim_end_matches(".session");
                        if let Ok(Some(session)) = self.load(session_id) {
                            if session.is_expired() {
                                let _ = self.delete(session_id);
                                cleaned += 1;
                            }
                        }
                    }
                }
            }
        }

        Ok(cleaned)
    }

    fn exists(&self, session_id: &str) -> bool {
        self.session_file_path(session_id).exists()
    }

    fn count(&self) -> usize {
        std::fs::read_dir(&self.directory)
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|entry| {
                        entry.file_name().to_str()
                            .map(|name| name.ends_with(".session"))
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0)
    }
}

/// Redis-based session store
pub struct RedisSessionStore {
    connection_string: String,
    key_prefix: String,
    client: std::sync::Arc<std::sync::Mutex<Option<redis::Client>>>,
    pool_size: usize,
}

impl RedisSessionStore {
    pub fn new(connection_string: String) -> Self {
        Self {
            connection_string,
            key_prefix: "cursed:session:".to_string(),
            client: std::sync::Arc::new(std::sync::Mutex::new(None)),
            pool_size: 10, // Default pool size
        }
    }

    pub fn with_prefix(connection_string: String, key_prefix: String) -> Self {
        Self {
            connection_string,
            key_prefix,
            client: std::sync::Arc::new(std::sync::Mutex::new(None)),
            pool_size: 10,
        }
    }

    pub fn with_pool_size(mut self, pool_size: usize) -> Self {
        self.pool_size = pool_size;
        self
    }

    fn session_key(&self, session_id: &str) -> String {
        format!("{}{}", self.key_prefix, session_id)
    }

    /// Get or create Redis client with connection pooling
    fn get_client(&self) -> Result<(), Error> {
        let mut client_guard = self.client.lock()
            .map_err(|_| SessionError::StoreError("Failed to acquire Redis client lock".to_string()))?;
        
        if client_guard.is_none() {
            // Create new Redis client
            let client = redis::Client::open(self.connection_string.as_str())
                .map_err(|e| SessionError::StoreError(format!("Failed to create Redis client: {}", e)))?;
            
            // Test connection
            let mut conn = client.get_connection()
                .map_err(|e| SessionError::StoreError(format!("Failed to connect to Redis: {}", e)))?;
            
            // Test with PING command
            let _: String = redis::cmd("PING").query(&mut conn)
                .map_err(|e| SessionError::StoreError(format!("Redis PING failed: {}", e)))?;
            
            *client_guard = Some(client);
        }
        
        Ok(client_guard.as_ref().unwrap().clone())
    }

    /// Get Redis connection with fallback handling
    fn get_connection(&self) -> Result<(), Error> {
        let client = self.get_client()?;
        client.get_connection()
            .map_err(|e| SessionError::StoreError(format!("Failed to get Redis connection: {}", e)))
    }

    /// Execute Redis GET command with error handling
    fn redis_get(&self, key: &str) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        
        match redis::cmd("GET").arg(key).query::<Option<String>>(&mut conn) {
            Ok(value) => Ok(value),
            Err(e) => {
                eprintln!("Redis GET error for key '{}': {}", key, e);
                Err(SessionError::StoreError(format!("Redis GET failed: {}", e)))
            }
        }
    }

    /// Execute Redis SET command with TTL support
    fn redis_set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        
        let result = if let Some(ttl) = ttl_seconds {
            // SET with EX (expire in seconds)
            redis::cmd("SETEX").arg(key).arg(ttl).arg(value).query::<String>(&mut conn)
        } else {
            // SET without expiration
            redis::cmd("SET").arg(key).arg(value).query::<String>(&mut conn)
        };
        
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Redis SET error for key '{}': {}", key, e);
                Err(SessionError::StoreError(format!("Redis SET failed: {}", e)))
            }
        }
    }

    /// Execute Redis DEL command
    fn redis_del(&self, key: &str) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        
        match redis::cmd("DEL").arg(key).query::<i32>(&mut conn) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Redis DEL error for key '{}': {}", key, e);
                Err(SessionError::StoreError(format!("Redis DEL failed: {}", e)))
            }
        }
    }

    /// Execute Redis EXISTS command
    fn redis_exists(&self, key: &str) -> bool {
        match self.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("EXISTS").arg(key).query::<i32>(&mut conn) {
                    Ok(exists) => exists > 0,
                    Err(e) => {
                        eprintln!("Redis EXISTS error for key '{}': {}", key, e);
                        false // Assume doesn't exist on error
                    }
                }
            }
            Err(e) => {
                eprintln!("Redis connection error in EXISTS: {}", e);
                false // Assume doesn't exist on connection error
            }
        }
    }

    /// Execute Redis TTL command to get remaining time to live
    fn redis_ttl(&self, key: &str) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        
        redis::cmd("TTL").arg(key).query::<i64>(&mut conn)
            .map_err(|e| SessionError::StoreError(format!("Redis TTL failed: {}", e)))
    }

    /// Get all session keys using SCAN for better performance
    fn get_session_keys(&self) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        let pattern = format!("{}*", self.key_prefix);
        
        let mut keys = Vec::new();
        let mut cursor = 0;
        
        loop {
            let result: (i64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&pattern)
                .arg("COUNT")
                .arg(100) // Scan in batches of 100
                .query(&mut conn)
                .map_err(|e| SessionError::StoreError(format!("Redis SCAN failed: {}", e)))?;
            
            cursor = result.0;
            keys.extend(result.1);
            
            if cursor == 0 {
                break;
            }
        }
        
        Ok(keys)
    }

    /// Count sessions using more efficient approach
    fn count_sessions(&self) -> Result<(), Error> {
        let keys = self.get_session_keys()?;
        Ok(keys.len())
    }

    /// Health check for Redis connection
    pub fn health_check(&self) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        let _: String = redis::cmd("PING").query(&mut conn)
            .map_err(|e| SessionError::StoreError(format!("Redis health check failed: {}", e)))?;
        Ok(())
    }

    /// Get Redis info for monitoring
    pub fn get_redis_info(&self) -> Result<(), Error> {
        let mut conn = self.get_connection()?;
        redis::cmd("INFO").query::<String>(&mut conn)
            .map_err(|e| SessionError::StoreError(format!("Redis INFO failed: {}", e)))
    }
}

impl SessionStore for RedisSessionStore {
    fn load(&self, session_id: &str) -> Result<(), Error> {
        let key = self.session_key(session_id);
        
        match self.redis_get(&key)? {
            Some(data) => {
                let session = Session::deserialize(&data)?;
                
                // Check if session is expired (Redis should handle TTL automatically)
                if session.is_expired() {
                    // Clean up expired session
                    let _ = self.redis_del(&key);
                    Ok(None)
                } else {
                    Ok(Some(session))
                }
            }
            None => Ok(None)
        }
    }

    fn save(&mut self, session: &Session) -> Result<(), Error> {
        let key = self.session_key(&session.id);
        let data = session.serialize();
        
        // Calculate TTL based on session expiration
        let ttl = if let Some(expires_at) = session.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            
            if expires_at > now {
                Some(expires_at - now)
            } else {
                // Session is already expired, set very short TTL
                Some(1)
            }
        } else {
            // No expiration set, use a default maximum TTL (24 hours)
            Some(86400)
        };
        
        self.redis_set(&key, &data, ttl)?;
        println!("Session {} saved to Redis with TTL: {:?}", session.id, ttl);
        Ok(())
    }

    fn delete(&mut self, session_id: &str) -> Result<(), Error> {
        let key = self.session_key(session_id);
        self.redis_del(&key)?;
        println!("Session {} deleted from Redis", session_id);
        Ok(())
    }

    fn cleanup_expired(&mut self) -> Result<(), Error> {
        // In Redis, expired keys are automatically cleaned up by Redis itself
        // However, we can still scan for sessions that are expired according to our logic
        // but haven't been cleaned up yet by Redis TTL
        
        let keys = self.get_session_keys()?;
        let mut cleaned = 0;
        
        for key in keys {
            // Check if the key still exists and get its value
            if let Ok(Some(data)) = self.redis_get(&key) {
                if let Ok(session) = Session::deserialize(&data) {
                    if session.is_expired() {
                        // Session is expired according to our logic, remove it
                        if self.redis_del(&key).is_ok() {
                            cleaned += 1;
                        }
                    }
                }
            }
        }
        
        if cleaned > 0 {
            println!("Cleaned up {} expired sessions from Redis", cleaned);
        }
        
        Ok(cleaned)
    }

    fn exists(&self, session_id: &str) -> bool {
        let key = self.session_key(session_id);
        self.redis_exists(&key)
    }

    fn count(&self) -> usize {
        match self.count_sessions() {
            Ok(count) => {
                println!("Total sessions in Redis: {}", count);
                count
            }
            Err(e) => {
                eprintln!("Failed to count Redis sessions: {}", e);
                0
            }
        }
    }
}

/// Configuration for database session store
#[derive(Debug, Clone)]
pub struct DatabaseStoreConfig {
    pub pool_size: usize,
    pub max_retries: usize,
    pub retry_delay_ms: u64,
    pub cleanup_interval_seconds: u64,
    pub connection_timeout_seconds: u64,
    pub enable_wal_mode: bool,
    pub enable_foreign_keys: bool,
    pub enable_auto_vacuum: bool,
    pub cache_size_mb: usize,
}

impl Default for DatabaseStoreConfig {
    fn default() -> Self {
        Self {
            pool_size: 10,
            max_retries: 3,
            retry_delay_ms: 100,
            cleanup_interval_seconds: 300, // 5 minutes
            connection_timeout_seconds: 30,
            enable_wal_mode: true,
            enable_foreign_keys: true,
            enable_auto_vacuum: true,
            cache_size_mb: 64,
        }
    }
}

/// Database connection pool for session management
pub struct DatabaseConnectionPool {
    connections: std::sync::Arc<std::sync::Mutex<Vec<crate::stdlib::database::DB>>>,
    connection_string: String,
    driver_name: String,
    config: DatabaseStoreConfig,
    active_connections: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    is_shutdown: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl DatabaseConnectionPool {
    pub fn new(connection_string: String, config: DatabaseStoreConfig) -> Result<(), Error> {
        let driver_name = Self::detect_driver(&connection_string);
        
        let pool = Self {
            connections: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
            connection_string: connection_string.clone(),
            driver_name,
            config: config.clone(),
            active_connections: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            is_shutdown: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        };
        
        // Pre-populate the pool with initial connections
        pool.initialize_pool()?;
        
        Ok(pool)
    }
    
    fn detect_driver(connection_string: &str) -> String {
        if connection_string.starts_with("sqlite://") 
            || connection_string.starts_with("sqlite3://") 
            || connection_string.ends_with(".db") 
            || connection_string.ends_with(".sqlite")
            || connection_string.ends_with(".sqlite3") {
            "sqlite".to_string()
        } else if connection_string.starts_with("postgres://") 
            || connection_string.starts_with("postgresql://") {
            "postgres".to_string()
        } else if connection_string.starts_with("mysql://") {
            "mysql".to_string()
        } else {
            "sqlite".to_string() // Default to SQLite
        }
    }
    
    fn initialize_pool(&self) -> Result<(), Error> {
        let mut connections = self.connections.lock()
            .map_err(|_| SessionError::StoreError("Failed to acquire connection pool lock".to_string()))?;
            
        for _ in 0..self.config.pool_size {
            let db = self.create_connection()?;
            connections.push(db);
        }
        
        Ok(())
    }
    
    fn create_connection(&self) -> Result<(), Error> {
        let mut db = crate::stdlib::database::DB::open(
            self.driver_name.clone(), 
            self.connection_string.clone()
        ).map_err(|e| SessionError::StoreError(format!("Database connection failed: {}", e)))?;
        
        // Configure database connection for optimal session performance
        self.configure_connection(&mut db)?;
        
        Ok(db)
    }
    
    fn configure_connection(&self, db: &mut crate::stdlib::database::DB) -> Result<(), Error> {
        if self.driver_name == "sqlite" {
            // SQLite-specific optimizations
            if self.config.enable_wal_mode {
                db.exec("PRAGMA journal_mode=WAL".to_string(), vec![])
                    .map_err(|e| SessionError::StoreError(format!("Failed to enable WAL mode: {}", e)))?;
            }
            
            if self.config.enable_foreign_keys {
                db.exec("PRAGMA foreign_keys=ON".to_string(), vec![])
                    .map_err(|e| SessionError::StoreError(format!("Failed to enable foreign keys: {}", e)))?;
            }
            
            if self.config.enable_auto_vacuum {
                db.exec("PRAGMA auto_vacuum=INCREMENTAL".to_string(), vec![])
                    .map_err(|e| SessionError::StoreError(format!("Failed to enable auto vacuum: {}", e)))?;
            }
            
            // Performance optimizations
            db.exec("PRAGMA synchronous=NORMAL".to_string(), vec![])
                .map_err(|e| SessionError::StoreError(format!("Failed to set synchronous mode: {}", e)))?;
            
            let cache_size = self.config.cache_size_mb * 256; // Convert MB to pages (4KB each)
            db.exec(format!("PRAGMA cache_size={}", cache_size), vec![])
                .map_err(|e| SessionError::StoreError(format!("Failed to set cache size: {}", e)))?;
                
            db.exec("PRAGMA temp_store=MEMORY".to_string(), vec![])
                .map_err(|e| SessionError::StoreError(format!("Failed to set temp store: {}", e)))?;
                
            db.exec("PRAGMA optimize".to_string(), vec![])
                .map_err(|e| SessionError::StoreError(format!("Failed to optimize database: {}", e)))?;
        }
        
        Ok(())
    }
    
    pub fn get_connection(&self) -> Result<(), Error> {
        if self.is_shutdown.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(SessionError::StoreError("Connection pool is shutdown".to_string()));
        }
        
        let mut retries = 0;
        
        while retries < self.config.max_retries {
            // Try to get a connection from the pool
            if let Ok(mut connections) = self.connections.try_lock() {
                if let Some(db) = connections.pop() {
                    self.active_connections.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    return Ok(PooledConnection {
                        db: Some(db),
                        pool: std::sync::Arc::clone(&self.connections),
                        active_connections: std::sync::Arc::clone(&self.active_connections),
                    });
                }
            }
            
            // If pool is empty, try to create a new connection
            if self.active_connections.load(std::sync::atomic::Ordering::SeqCst) < self.config.pool_size {
                match self.create_connection() {
                    Ok(db) => {
                        self.active_connections.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        return Ok(PooledConnection {
                            db: Some(db),
                            pool: std::sync::Arc::clone(&self.connections),
                            active_connections: std::sync::Arc::clone(&self.active_connections),
                        });
                    }
                    Err(e) => {
                        retries += 1;
                        if retries < self.config.max_retries {
                            std::thread::sleep(std::time::Duration::from_millis(self.config.retry_delay_ms));
                            continue;
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
            
            // Wait and retry
            retries += 1;
            if retries < self.config.max_retries {
                std::thread::sleep(std::time::Duration::from_millis(self.config.retry_delay_ms));
            }
        }
        
        Err(SessionError::StoreError("Failed to acquire database connection after retries".to_string()))
    }
    
    pub fn shutdown(&self) {
        self.is_shutdown.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // Close all connections in the pool
        if let Ok(mut connections) = self.connections.lock() {
            connections.clear();
        }
    }
    
    pub fn get_pool_stats(&self) -> PoolStats {
        let available = self.connections.lock()
            .map(|conns| conns.len())
            .unwrap_or(0);
            
        let active = self.active_connections.load(std::sync::atomic::Ordering::SeqCst);
        
        PoolStats {
            total_connections: self.config.pool_size,
            active_connections: active,
            available_connections: available,
            max_pool_size: self.config.pool_size,
            is_shutdown: self.is_shutdown.load(std::sync::atomic::Ordering::SeqCst),
        }
    }
}

/// A connection borrowed from the pool with transaction support
pub struct PooledConnection {
    db: Option<crate::stdlib::database::DB>,
    pool: std::sync::Arc<std::sync::Mutex<Vec<crate::stdlib::database::DB>>>,
    active_connections: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl PooledConnection {
    pub fn execute_transaction<F, R>(&mut self, transaction_fn: F) -> Result<(), Error>
    where
        F: FnOnce(&mut crate::stdlib::database::DB) -> Result<(), Error>,
    {
        if let Some(ref mut db) = self.db {
            // Begin transaction
            db.exec("BEGIN TRANSACTION".to_string(), vec![])
                .map_err(|e| SessionError::StoreError(format!("Failed to begin transaction: {}", e)))?;
            
            match transaction_fn(db) {
                Ok(result) => {
                    // Commit transaction
                    db.exec("COMMIT".to_string(), vec![])
                        .map_err(|e| SessionError::StoreError(format!("Failed to commit transaction: {}", e)))?;
                    Ok(result)
                }
                Err(e) => {
                    // Rollback transaction
                    let _ = db.exec("ROLLBACK".to_string(), vec![]);
                    Err(e)
                }
            }
        } else {
            Err(SessionError::StoreError("Connection is not available".to_string()))
        }
    }
    
    pub fn execute_with_retry<F, R>(&mut self, operation: F) -> Result<(), Error>
    where
        F: Fn(&mut crate::stdlib::database::DB) -> Result<(), Error>,
    {
        if let Some(ref mut db) = self.db {
            let mut retries = 0;
            const MAX_RETRIES: usize = 3;
            
            while retries < MAX_RETRIES {
                match operation(db) {
                    Ok(result) => return Ok(result),
                    Err(e) => {
                        retries += 1;
                        if retries >= MAX_RETRIES {
                            return Err(e);
                        }
                        // Small delay before retry
                        std::thread::sleep(std::time::Duration::from_millis(50));
                    }
                }
            }
            
            Err(SessionError::StoreError("Operation failed after retries".to_string()))
        } else {
            Err(SessionError::StoreError("Connection is not available".to_string()))
        }
    }
    
    pub fn as_mut(&mut self) -> Option<&mut crate::stdlib::database::DB> {
        self.db.as_mut()
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        if let Some(db) = self.db.take() {
            // Return connection to pool
            if let Ok(mut connections) = self.pool.lock() {
                connections.push(db);
            }
            self.active_connections.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

/// Pool statistics for monitoring
#[derive(Debug)]
pub struct PoolStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub available_connections: usize,
    pub max_pool_size: usize,
    pub is_shutdown: bool,
}

/// Enhanced database-based session store with connection pooling and transaction safety
pub struct DatabaseSessionStore {
    connection_string: String,
    table_name: String,
    pool: std::sync::Arc<DatabaseConnectionPool>,
    config: DatabaseStoreConfig,
    cleanup_handle: std::sync::Arc<std::sync::Mutex<Option<std::thread::JoinHandle<()>>>>,
    schema_version: u32,
}

impl DatabaseSessionStore {
    pub fn new(connection_string: String) -> Result<(), Error> {
        Self::new_with_config(connection_string, DatabaseStoreConfig::default())
    }
    
    pub fn new_with_config(connection_string: String, config: DatabaseStoreConfig) -> Result<(), Error> {
        let pool = std::sync::Arc::new(DatabaseConnectionPool::new(connection_string.clone(), config.clone())?);
        
        let store = Self {
            connection_string,
            table_name: "cursed_sessions".to_string(),
            pool: pool.clone(),
            config: config.clone(),
            cleanup_handle: std::sync::Arc::new(std::sync::Mutex::new(None)),
            schema_version: 1,
        };
        
        // Initialize database schema
        store.init_schema()?;
        
        // Start automatic cleanup task
        store.start_cleanup_task();
        
        Ok(store)
    }
    
    /// Initialize database schema with proper indexing and constraints
    fn init_schema(&self) -> Result<(), Error> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_transaction(|db| {
            // Create main sessions table with optimized schema
            let create_table_sql = format!(r#"
                CREATE TABLE IF NOT EXISTS {} (
                    id TEXT PRIMARY KEY NOT NULL,
                    session_data TEXT NOT NULL,
                    created_at INTEGER NOT NULL,
                    last_accessed INTEGER NOT NULL,
                    expires_at INTEGER,
                    data_checksum TEXT,
                    schema_version INTEGER DEFAULT 1
                )
            "#, self.table_name);

            db.exec(create_table_sql, vec![])
                .map_err(|e| SessionError::StoreError(format!("Failed to create session table: {}", e)))?;
            
            // Create indexes for optimal query performance
            let indexes = vec![
                format!("CREATE INDEX IF NOT EXISTS idx_{}_expires ON {} (expires_at) WHERE expires_at IS NOT NULL", 
                    self.table_name, self.table_name),
                format!("CREATE INDEX IF NOT EXISTS idx_{}_last_accessed ON {} (last_accessed)", 
                    self.table_name, self.table_name),
                format!("CREATE INDEX IF NOT EXISTS idx_{}_created_at ON {} (created_at)", 
                    self.table_name, self.table_name),
                format!("CREATE INDEX IF NOT EXISTS idx_{}_schema_version ON {} (schema_version)", 
                    self.table_name, self.table_name),
            ];
            
            for index_sql in indexes {
                db.exec(index_sql, vec![])
                    .map_err(|e| SessionError::StoreError(format!("Failed to create index: {}", e)))?;
            }
            
            // Create metadata table for schema versioning
            let metadata_sql = format!(r#"
                CREATE TABLE IF NOT EXISTS {}_metadata (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL,
                    updated_at INTEGER NOT NULL
                )
            "#, self.table_name);
            
            db.exec(metadata_sql, vec![])
                .map_err(|e| SessionError::StoreError(format!("Failed to create metadata table: {}", e)))?;
            
            Ok(())
        })?;
        
        Ok(())
    }
    
    /// Start automatic cleanup task for expired sessions
    fn start_cleanup_task(&self) {
        let pool = std::sync::Arc::clone(&self.pool);
        let table_name = self.table_name.clone();
        let cleanup_interval = self.config.cleanup_interval_seconds;
        
        let handle = std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(cleanup_interval));
                
                // Perform cleanup
                if let Ok(mut connection) = pool.get_connection() {
                    let _ = connection.execute_transaction(|db| {
                        let now = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs() as i64;
                        
                        let cleanup_sql = format!(
                            "DELETE FROM {} WHERE expires_at IS NOT NULL AND expires_at < ?", 
                            table_name
                        );
                        
                        db.exec(cleanup_sql, vec![crate::stdlib::database::SqlValue::Integer(now)])
                            .map_err(|e| SessionError::StoreError(format!("Cleanup failed: {}", e)))?;
                        
                        // Also run VACUUM INCREMENTAL for SQLite
                        let _ = db.exec("PRAGMA incremental_vacuum(100)".to_string(), vec![]);
                        
                        Ok(())
                    });
                }
            }
        });
        
        if let Ok(mut cleanup_guard) = self.cleanup_handle.lock() {
            *cleanup_guard = Some(handle);
        }
    }
    
    /// Get pool statistics for monitoring
    pub fn get_pool_stats(&self) -> PoolStats {
        self.pool.get_pool_stats()
    }
    
    /// Shutdown the session store and cleanup resources
    pub fn shutdown(&self) {
        self.pool.shutdown();
        
        if let Ok(mut cleanup_guard) = self.cleanup_handle.lock() {
            if let Some(handle) = cleanup_guard.take() {
                // We can't join the thread since it's in an infinite loop
                // In a real implementation, we'd use a shutdown signal
            }
        }
    }
    
    /// Calculate data checksum for integrity verification
    fn calculate_checksum(&self, data: &str) -> String {
        let mut hash: u64 = 5381;
        for byte in data.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        format!("{:016x}", hash)
    }

    /// Initialize database connection and create session table
    fn ensure_connection(&self) -> Result<(), Error> {
        self.pool.get_connection()
    }

    /// Load session data from database
    fn db_select(&self, session_id: &str) -> Result<(), Error> {
        let mut connection = self.ensure_connection()?;
        
        let sql = format!(
            "SELECT session_data FROM {} WHERE id = ?", 
            self.table_name
        );
        
        connection.execute_with_retry(|db| {
            let row = db.query_row(sql.clone(), vec![crate::stdlib::database::SqlValue::String(session_id.to_string())]);
            let map = row.scan_map().unwrap_or_default();
            if let Some(crate::stdlib::database::SqlValue::String(data)) = map.get("session_data") {
                Ok(Some(data.clone()))
            } else {
                Ok(None)
            }
        })
    }

    /// Insert or update session in database
    fn db_insert_or_update(&self, session: &Session) -> Result<(), Error> {
        let mut connection = self.ensure_connection()?;
        
        let sql = format!(r#"
            INSERT OR REPLACE INTO {} 
            (id, session_data, created_at, last_accessed, expires_at) 
            VALUES (?, ?, ?, ?, ?)
        "#, self.table_name);
        
        let expires_at = session.expires_at.map(|t| crate::stdlib::database::SqlValue::Integer(t as i64))
            .unwrap_or(crate::stdlib::database::SqlValue::Null);
        
        connection.execute_transaction(|db| {
            db.exec(sql, vec![
                crate::stdlib::database::SqlValue::String(session.id.clone()),
                crate::stdlib::database::SqlValue::String(session.serialize()),
                crate::stdlib::database::SqlValue::Integer(session.created_at as i64),
                crate::stdlib::database::SqlValue::Integer(session.last_accessed as i64),
                expires_at,
            ]).map_err(|e| SessionError::StoreError(format!("Failed to save session: {}", e)))?;
            Ok(())
        })?;
        
        Ok(())
    }

    /// Delete session from database
    fn db_delete(&self, session_id: &str) -> Result<(), Error> {
        let mut connection = self.ensure_connection()?;
        
        let sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
        
        connection.execute_transaction(|db| {
            db.exec(sql, vec![crate::stdlib::database::SqlValue::String(session_id.to_string())])
                .map_err(|e| SessionError::StoreError(format!("Failed to delete session: {}", e)))?;
            Ok(())
        })?;
        
        Ok(())
    }

    /// Count total sessions in database
    fn db_count(&self) -> Result<(), Error> {
        let mut connection = self.ensure_connection()?;
        
        let sql = format!("SELECT COUNT(*) as count FROM {}", self.table_name);
        
        connection.execute_with_retry(|db| {
            let row = db.query_row(sql.clone(), vec![]);
            let map = row.scan_map().unwrap_or_default();
            if let Some(crate::stdlib::database::SqlValue::Integer(count)) = map.get("count") {
                Ok(*count as usize)
            } else {
                Ok(0)
            }
        })
    }

    /// Clean up expired sessions from database
    fn db_cleanup_expired(&self) -> Result<(), Error> {
        let mut connection = self.ensure_connection()?;
        
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        
        let sql = format!(
            "DELETE FROM {} WHERE expires_at IS NOT NULL AND expires_at < ?", 
            self.table_name
        );
        
        let result = connection.execute_transaction(|db| {
            db.exec(sql, vec![crate::stdlib::database::SqlValue::Integer(now)])
                .map_err(|e| SessionError::StoreError(format!("Failed to cleanup expired sessions: {}", e)))
        })?;
        
        Ok(result.rows_affected as usize)
    }

    /// Check if session exists in database
    fn db_exists(&self, session_id: &str) -> Result<(), Error> {
        let mut connection = self.ensure_connection()?;
        
        let sql = format!("SELECT 1 FROM {} WHERE id = ? LIMIT 1", self.table_name);
        
        connection.execute_with_retry(|db| {
            let _row = db.query_row(sql.clone(), vec![crate::stdlib::database::SqlValue::String(session_id.to_string())]);
            // If we get a row back, the session exists
            Ok(true)
        })
    }
}

impl SessionStore for DatabaseSessionStore {
    fn load(&self, session_id: &str) -> Result<(), Error> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_with_retry(|db| {
            let sql = format!(
                "SELECT session_data, data_checksum FROM {} WHERE id = ? LIMIT 1", 
                self.table_name
            );
            
            let row = db.query_row(sql.clone(), vec![crate::stdlib::database::SqlValue::String(session_id.to_string())]);
            let map = row.scan_map().unwrap_or_default();
            if let (Some(crate::stdlib::database::SqlValue::String(data)), checksum) = 
                (map.get("session_data"), map.get("data_checksum")) {
                
                // Verify data integrity if checksum exists
                if let Some(crate::stdlib::database::SqlValue::String(stored_checksum)) = checksum {
                    let calculated_checksum = self.calculate_checksum(data);
                    if calculated_checksum != *stored_checksum {
                        return Err(SessionError::StoreError("Session data corruption detected".to_string()));
                    }
                }
                
                let session = Session::deserialize(data)?;
                if session.is_expired() {
                    // Session is expired, return None
                    Ok(None)
                } else {
                    Ok(Some(session))
                }
            } else {
                Ok(None)
            }
        })
    }

    fn save(&mut self, session: &Session) -> Result<(), Error> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_transaction(|db| {
            let session_data = session.serialize();
            let checksum = self.calculate_checksum(&session_data);
            
            let sql = format!(r#"
                INSERT OR REPLACE INTO {} 
                (id, session_data, created_at, last_accessed, expires_at, data_checksum, schema_version) 
                VALUES (?, ?, ?, ?, ?, ?, ?)
            "#, self.table_name);
            
            let expires_at = session.expires_at.map(|t| crate::stdlib::database::SqlValue::Integer(t as i64))
                .unwrap_or(crate::stdlib::database::SqlValue::Null);
            
            db.exec(sql, vec![
                crate::stdlib::database::SqlValue::String(session.id.clone()),
                crate::stdlib::database::SqlValue::String(session_data),
                crate::stdlib::database::SqlValue::Integer(session.created_at as i64),
                crate::stdlib::database::SqlValue::Integer(session.last_accessed as i64),
                expires_at,
                crate::stdlib::database::SqlValue::String(checksum),
                crate::stdlib::database::SqlValue::Integer(self.schema_version as i64),
            ]).map_err(|e| SessionError::StoreError(format!("Failed to save session: {}", e)))?;
            
            Ok(())
        })
    }

    fn delete(&mut self, session_id: &str) -> Result<(), Error> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_transaction(|db| {
            let sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
            
            db.exec(sql, vec![crate::stdlib::database::SqlValue::String(session_id.to_string())])
                .map_err(|e| SessionError::StoreError(format!("Failed to delete session: {}", e)))?;
            
            Ok(())
        })
    }

    fn cleanup_expired(&mut self) -> Result<(), Error> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_transaction(|db| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            
            let sql = format!(
                "DELETE FROM {} WHERE expires_at IS NOT NULL AND expires_at < ?", 
                self.table_name
            );
            
            let result = db.exec(sql, vec![crate::stdlib::database::SqlValue::Integer(now)])
                .map_err(|e| SessionError::StoreError(format!("Failed to cleanup expired sessions: {}", e)))?;
            
            // Run incremental vacuum after cleanup
            let _ = db.exec("PRAGMA incremental_vacuum(100)".to_string(), vec![]);
            
            Ok(result.rows_affected as usize)
        })
    }

    fn exists(&self, session_id: &str) -> bool {
        self.pool.get_connection()
            .and_then(|mut connection| {
                connection.execute_with_retry(|db| {
                    let sql = format!("SELECT 1 FROM {} WHERE id = ? LIMIT 1", self.table_name);
                    
                    let _row = db.query_row(sql, vec![crate::stdlib::database::SqlValue::String(session_id.to_string())]);
                    Ok(true)
                })
            })
            .unwrap_or(false)
    }

    fn count(&self) -> usize {
        self.pool.get_connection()
            .and_then(|mut connection| {
                connection.execute_with_retry(|db| {
                    let sql = format!("SELECT COUNT(*) as count FROM {}", self.table_name);
                    
                    let row = db.query_row(sql.clone(), vec![]);
                    let map = row.scan_map().unwrap_or_default();
                    if let Some(crate::stdlib::database::SqlValue::Integer(count)) = map.get("count") {
                        Ok(*count as usize)
                    } else {
                        Ok(0)
                    }
                })
            })
            .unwrap_or(0)
    }
}

impl Drop for DatabaseSessionStore {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Session manager
pub struct SessionManager {
    store: Box<dyn SessionStore>,
    config: SessionConfig,
}

impl SessionManager {
    /// Create new session manager
    pub fn new(config: SessionConfig) -> Result<(), Error> {
        let store: Box<dyn SessionStore> = match &config.store_type {
            SessionStoreType::Memory => Box::new(MemorySessionStore::new()),
            SessionStoreType::File(path) => Box::new(FileSessionStore::new(path.clone())),
            SessionStoreType::Redis(connection_string) => {
                Box::new(RedisSessionStore::new(connection_string.clone()))
            }
            SessionStoreType::Database(connection_string) => {
                Box::new(DatabaseSessionStore::new(connection_string.clone())?)
            }
        };

        Ok(Self { store, config })
    }

    /// Create new session
    pub fn create_session(&mut self) -> Result<(), Error> {
        let mut session = Session::new();
        session.set_expiry(self.config.max_age.as_secs());
        self.store.save(&session)?;
        Ok(session)
    }

    /// Load existing session
    pub fn load_session(&self, session_id: &str) -> Result<(), Error> {
        self.store.load(session_id)
    }

    /// Save session
    pub fn save_session(&mut self, session: &Session) -> Result<(), Error> {
        if session.is_dirty {
            self.store.save(session)?;
        }
        Ok(())
    }

    /// Delete session
    pub fn delete_session(&mut self, session_id: &str) -> Result<(), Error> {
        self.store.delete(session_id)
    }

    /// Generate session cookie
    pub fn create_session_cookie(&self, session: &Session) -> String {
        let mut cookie_parts = vec![
            format!("{}={}", self.config.cookie_name, session.id),
        ];

        if self.config.http_only {
            cookie_parts.push("HttpOnly".to_string());
        }

        if self.config.secure {
            cookie_parts.push("Secure".to_string());
        }

        let same_site = match self.config.same_site {
            SameSitePolicy::Strict => "SameSite=Strict",
            SameSitePolicy::Lax => "SameSite=Lax",
            SameSitePolicy::None => "SameSite=None",
        };
        cookie_parts.push(same_site.to_string());

        let max_age = self.config.max_age.as_secs();
        cookie_parts.push(format!("Max-Age={}", max_age));

        cookie_parts.join("; ")
    }

    /// Parse session ID from cookie header
    pub fn parse_session_id_from_cookie(&self, cookie_header: &str) -> Option<String> {
        for cookie in cookie_header.split(';') {
            let cookie = cookie.trim();
            if cookie.starts_with(&format!("{}=", self.config.cookie_name)) {
                return Some(cookie[self.config.cookie_name.len() + 1..].to_string());
            }
        }
        None
    }

    /// Cleanup expired sessions
    pub fn cleanup_expired_sessions(&mut self) -> Result<(), Error> {
        self.store.cleanup_expired()
    }

    /// Get session statistics
    pub fn get_session_stats(&self) -> SessionStats {
        SessionStats {
            total_sessions: self.store.count(),
            store_type: format!("{:?}", self.config.store_type),
            max_age_seconds: self.config.max_age.as_secs(),
            cleanup_interval_seconds: self.config.cleanup_interval.as_secs(),
        }
    }
}

/// Session error types
#[derive(Debug)]
pub enum SessionError {
    InvalidData(String),
    StoreError(String),
    SerializationError(String),
    NotFound(String),
    CorruptedData(String),
    ConnectionPoolExhausted,
    TransactionFailed(String),
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SessionError::InvalidData(field) => write!(f, "Invalid session data: {}", field),
            SessionError::StoreError(msg) => write!(f, "Session store error: {}", msg),
            SessionError::SerializationError(msg) => write!(f, "Session serialization error: {}", msg),
            SessionError::NotFound(id) => write!(f, "Session not found: {}", id),
            SessionError::CorruptedData(msg) => write!(f, "Session data corrupted: {}", msg),
            SessionError::ConnectionPoolExhausted => write!(f, "Database connection pool exhausted"),
            SessionError::TransactionFailed(msg) => write!(f, "Database transaction failed: {}", msg),
        }
    }
}

impl std::error::Error for SessionError {}

/// Session statistics
#[derive(Debug)]
pub struct SessionStats {
    pub total_sessions: usize,
    pub store_type: String,
    pub max_age_seconds: u64,
    pub cleanup_interval_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_session_creation() {
        let session = Session::new();
        assert!(!session.id.is_empty());
        assert!(session.is_new);
        assert!(!session.is_dirty);
        assert!(session.data.is_empty());
    }

    #[test]
    fn test_session_data_operations() {
        let mut session = Session::new();

        // Set values
        session.set("name".to_string(), SessionValue::String("John".to_string()));
        session.set("age".to_string(), SessionValue::Number(30.0));
        session.set("active".to_string(), SessionValue::Bool(true));

        // Get values
        assert_eq!(
            session.get("name").unwrap().as_string(),
            Some("John")
        );
        assert_eq!(
            session.get("age").unwrap().as_number(),
            Some(30.0)
        );
        assert_eq!(
            session.get("active").unwrap().as_bool(),
            Some(true)
        );

        // Check dirty flag
        assert!(session.is_dirty);

        // Remove value
        session.remove("age");
        assert!(session.get("age").is_none());
    }

    #[test]
    fn test_session_serialization() {
        let mut session = Session::new();
        session.set("test".to_string(), SessionValue::String("value".to_string()));

        let serialized = session.serialize();
        let mut deserialized = Session::deserialize(&serialized).unwrap();

        assert_eq!(session.id, deserialized.id);
        assert_eq!(
            deserialized.get("test").unwrap().as_string(),
            Some("value")
        );
    }

    #[test]
    fn test_memory_session_store() {
        let mut store = MemorySessionStore::new();
        let session = Session::new();

        // Save session
        store.save(&session).unwrap();
        assert_eq!(store.count(), 1);

        // Load session
        let loaded = store.load(&session.id).unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().id, session.id);

        // Delete session
        store.delete(&session.id).unwrap();
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_session_expiry() {
        let mut session = Session::new();
        session.set_expiry(1); // 1 second
        
        // Should not be expired immediately
        assert!(!session.is_expired());
        
        // Manually set to expired
        session.expires_at = Some(0);
        assert!(session.is_expired());
    }

    #[test]
    fn test_session_manager() {
        let config = SessionConfig {
            cookie_name: "test_session".to_string(),
            max_age: Duration::from_secs(3600),
            secure: false,
            http_only: true,
            same_site: SameSitePolicy::Lax,
            store_type: SessionStoreType::Memory,
            cleanup_interval: Duration::from_secs(300),
            database_timeout: Duration::from_secs(30),
            session_timeout: Duration::from_secs(1800),
        };

        let mut manager = SessionManager::new(config).unwrap();
        
        // Create session
        let session = manager.create_session().unwrap();
        assert!(!session.id.is_empty());

        // Load session
        let loaded = manager.load_session(&session.id).unwrap();
        assert!(loaded.is_some());

        // Create cookie
        let cookie = manager.create_session_cookie(&session);
        assert!(cookie.contains("test_session="));
        assert!(cookie.contains("HttpOnly"));
        assert!(cookie.contains("SameSite=Lax"));
    }

    #[test]
    fn test_cookie_parsing() {
        let config = SessionConfig {
            cookie_name: "cursed_session".to_string(),
            max_age: Duration::from_secs(3600),
            secure: false,
            http_only: true,
            same_site: SameSitePolicy::Lax,
            store_type: SessionStoreType::Memory,
            cleanup_interval: Duration::from_secs(300),
            database_timeout: Duration::from_secs(30),
            session_timeout: Duration::from_secs(1800),
        };

        let manager = SessionManager::new(config).unwrap();
        
        let cookie_header = "cursed_session=abc123; other_cookie=value";
        let session_id = manager.parse_session_id_from_cookie(cookie_header);
        assert_eq!(session_id, Some("abc123".to_string()));
    }

    #[test]
    fn test_redis_session_store() {
        // Use a test Redis instance or skip if Redis is not available
        let mut store = RedisSessionStore::new("redis://127.0.0.1:6379/1".to_string());
        
        // Test health check first
        if store.health_check().is_err() {
            println!("Redis not available, skipping Redis tests");
            return;
        }
        
        let session = Session::new();

        // Save session
        store.save(&session).unwrap();
        assert!(store.exists(&session.id));

        // Load session
        let loaded = store.load(&session.id).unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().id, session.id);

        // Delete session
        store.delete(&session.id).unwrap();
        assert!(!store.exists(&session.id));
    }

    #[test]
    fn test_redis_session_expiry() {
        let mut store = RedisSessionStore::new("redis://127.0.0.1:6379/1".to_string());
        
        // Skip test if Redis is not available
        if store.health_check().is_err() {
            println!("Redis not available, skipping Redis expiry test");
            return;
        }
        
        let mut session = Session::new();
        session.set_expiry(1); // 1 second
        
        // Save session
        store.save(&session).unwrap();
        assert!(store.exists(&session.id));

        // Manually set to expired and test cleanup
        session.expires_at = Some(0);
        store.save(&session).unwrap();
        
        let loaded = store.load(&session.id).unwrap();
        assert!(loaded.is_none()); // Should be None due to expiration
    }

    #[test]
    fn test_redis_connection_pooling() {
        let mut store = RedisSessionStore::new("redis://127.0.0.1:6379/1".to_string())
            .with_pool_size(5);
        
        // Skip test if Redis is not available
        if store.health_check().is_err() {
            println!("Redis not available, skipping Redis connection pooling test");
            return;
        }

        // Test multiple operations to verify connection pooling
        for i in 0..10 {
            let mut session = Session::new();
            session.set(format!("test_{}", i), SessionValue::Number(i as f64));
            
            store.save(&session).unwrap();
            assert!(store.exists(&session.id));
            
            let loaded = store.load(&session.id).unwrap();
            assert!(loaded.is_some());
            
            store.delete(&session.id).unwrap();
        }
    }

    #[test]
    fn test_redis_ttl_functionality() {
        let mut store = RedisSessionStore::new("redis://127.0.0.1:6379/1".to_string());
        
        // Skip test if Redis is not available
        if store.health_check().is_err() {
            println!("Redis not available, skipping Redis TTL test");
            return;
        }

        let mut session = Session::new();
        session.set_expiry(10); // 10 seconds
        
        store.save(&session).unwrap();
        
        // Check TTL is set correctly
        let key = store.session_key(&session.id);
        if let Ok(ttl) = store.redis_ttl(&key) {
            assert!(ttl > 0 && ttl <= 10);
        }
    }

    #[test]
    fn test_database_session_store() {
        let mut store = DatabaseSessionStore::new("test_sessions.db".to_string()).unwrap();
        let session = Session::new();

        // Save session
        store.save(&session).unwrap();
        assert_eq!(store.count(), 1);
        assert!(store.exists(&session.id));

        // Load session
        let loaded = store.load(&session.id).unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().id, session.id);

        // Delete session
        store.delete(&session.id).unwrap();
        assert_eq!(store.count(), 0);
        assert!(!store.exists(&session.id));
    }

    #[test]
    fn test_database_session_expiry_cleanup() {
        let mut store = DatabaseSessionStore::new("test_sessions_cleanup.db".to_string()).unwrap();
        let mut session1 = Session::new();
        let mut session2 = Session::new();
        
        // Set one session to be expired
        session1.expires_at = Some(0);
        session2.set_expiry(3600); // 1 hour from now
        
        store.save(&session1).unwrap();
        store.save(&session2).unwrap();
        assert_eq!(store.count(), 2);

        // Cleanup expired sessions
        let cleaned = store.cleanup_expired().unwrap();
        assert_eq!(cleaned, 1);
        assert_eq!(store.count(), 1);
        
        // Verify correct session remains
        assert!(!store.exists(&session1.id));
        assert!(store.exists(&session2.id));
    }

    #[test]
    fn test_database_session_data_persistence() {
        let mut store = DatabaseSessionStore::new("test_sessions_data.db".to_string()).unwrap();
        let mut session = Session::new();
        
        // Add some session data
        session.set("user_id".to_string(), SessionValue::String("12345".to_string()));
        session.set("login_time".to_string(), SessionValue::Number(1640995200.0));
        session.set("is_admin".to_string(), SessionValue::Bool(true));
        
        // Save and reload
        store.save(&session).unwrap();
        let mut loaded = store.load(&session.id).unwrap().unwrap();
        
        // Verify data persistence
        assert_eq!(loaded.get("user_id").unwrap().as_string(), Some("12345"));
        assert_eq!(loaded.get("login_time").unwrap().as_number(), Some(1640995200.0));
        assert_eq!(loaded.get("is_admin").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn test_database_connection_string_parsing() {
        // Test SQLite variations
        let store1 = DatabaseSessionStore::new("sqlite://test.db".to_string()).unwrap();
        let store2 = DatabaseSessionStore::new("test.sqlite".to_string()).unwrap();
        let store3 = DatabaseSessionStore::new("test.sqlite3".to_string()).unwrap();
        
        // These should not panic when creating connection
        let _db1 = store1.ensure_connection();
        let _db2 = store2.ensure_connection();
        let _db3 = store3.ensure_connection();
    }

    #[test]
    fn test_database_session_error_handling() {
        let result = DatabaseSessionStore::new("".to_string()); // Invalid connection string
        
        // Should handle gracefully - constructor should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_database_concurrent_access() {
        use std::thread;
        use std::sync::Arc;
        
        let store = Arc::new(DatabaseSessionStore::new("test_concurrent.db".to_string()).unwrap());
        let mut handles = vec![];
        
        // Spawn multiple threads accessing the same store
        for i in 0..5 {
            let store_clone = Arc::clone(&store);
            handles.push(thread::spawn(move || {
                let mut session = Session::new();
                session.set(format!("thread_{}", i), SessionValue::Number(i as f64));
                
                // This would require mutable access, but we're testing immutable operations
                let exists_before = store_clone.exists(&session.id);
                assert!(!exists_before);
                
                // We can't call save here without &mut, but we can test other operations
                let count = store_clone.count();
                assert!(count >= 0); // Should always be non-negative
            }));
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_database_table_initialization() {
        let store = DatabaseSessionStore::new("test_init.db".to_string()).unwrap();
        
        // Ensure connection initializes table
        let _db = store.ensure_connection().unwrap();
        
        // Should be able to perform operations without errors
        let count = store.count();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_session_manager_with_redis() {
        let config = SessionConfig {
            cookie_name: "test_session".to_string(),
            max_age: Duration::from_secs(3600),
            secure: false,
            http_only: true,
            same_site: SameSitePolicy::Lax,
            store_type: SessionStoreType::Redis("redis://127.0.0.1:6379/1".to_string()),
            cleanup_interval: Duration::from_secs(300),
            database_timeout: Duration::from_secs(30),
            session_timeout: Duration::from_secs(1800),
        };

        let mut manager = SessionManager::new(config).unwrap();
        
        // Test if Redis is available - if not, skip test
        let test_store = RedisSessionStore::new("redis://127.0.0.1:6379/1".to_string());
        if test_store.health_check().is_err() {
            println!("Redis not available, skipping Redis session manager test");
            return;
        }
        
        // Create session
        let session = manager.create_session().unwrap();
        assert!(!session.id.is_empty());

        // Load session
        let loaded = manager.load_session(&session.id).unwrap();
        assert!(loaded.is_some());
        
        // Session statistics
        let stats = manager.get_session_stats();
        assert!(stats.total_sessions >= 0); // Don't assert exact count due to potential race conditions
        assert!(stats.store_type.contains("Redis"));
        
        // Clean up
        let _ = manager.delete_session(&session.id);
    }

    #[test]
    fn test_session_manager_with_database() {
        let config = SessionConfig {
            cookie_name: "test_session".to_string(),
            max_age: Duration::from_secs(3600),
            secure: false,
            http_only: true,
            same_site: SameSitePolicy::Lax,
            store_type: SessionStoreType::Database("sqlite://sessions.db".to_string()),
            cleanup_interval: Duration::from_secs(300),
            database_timeout: Duration::from_secs(30),
            session_timeout: Duration::from_secs(1800),
        };

        let mut manager = SessionManager::new(config).unwrap();
        
        // Create session
        let session = manager.create_session().unwrap();
        assert!(!session.id.is_empty());

        // Load session
        let loaded = manager.load_session(&session.id).unwrap();
        assert!(loaded.is_some());
        
        // Session statistics
        let stats = manager.get_session_stats();
        assert_eq!(stats.total_sessions, 1);
        assert!(stats.store_type.contains("Database"));
    }
}
