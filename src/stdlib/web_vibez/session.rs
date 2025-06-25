use crate::error::CursedError;
/// Session management and storage utilities
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

// use crate::stdlib::web_vibez::config::{SessionConfig, SessionStoreType, SameSitePolicy};

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
        pub fn open(_url: &str) -> crate::error::Result<()> {
            Err(RedisError::new("Redis support not compiled in"))
        }
        pub fn get_connection(&self) -> crate::error::Result<()> {
            Err(RedisError::new("Redis support not compiled in"))
        }
    }
    
    pub fn cmd(_cmd: &str) -> Cmd {
        Cmd
    pub struct Cmd;
    impl Cmd {
        pub fn arg<T: std::fmt::Display>(&mut self, _arg: T) -> &mut Self { self }
        pub fn query<T: Default>(&mut self, _conn: &mut Connection) -> crate::error::Result<()> {
            Err(RedisError::new("Redis support not compiled in"))
        }
    }
    
    #[derive(Debug)]
    pub struct RedisError {
    impl RedisError {
        fn new(message: &str) -> Self {
            Self { message: message.to_string() }
        }
//     impl std::fmt::Display for RedisError {
//         fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//             write!(f, "{}", self.message)
//         }
//     }
    
//     impl std::error::CursedError for RedisError {}
// }

/// Session data structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Session {
/// Session value types
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SessionValue {
impl fmt::Display for SessionValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
        }
    }

    /// Convert to number if possible
    pub fn as_number(&self) -> Option<f64> {
        match self {
        }
    }

    /// Convert to boolean if possible
    pub fn as_bool(&self) -> Option<bool> {
        match self {
        }
    }

    /// Convert to array if possible
    pub fn as_array(&self) -> Option<&Vec<SessionValue>> {
        match self {
        }
    }

    /// Convert to object if possible
    pub fn as_object(&self) -> Option<&HashMap<String, SessionValue>> {
        match self {
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
        }
    }

    /// Create session with specific ID
    pub fn with_id(id: String) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
        }
    }

    /// Set session value
    pub fn set(&mut self, key: String, value: SessionValue) {
        self.data.insert(key, value);
        self.is_dirty = true;
        self.touch();
    /// Get session value
    pub fn get(&mut self, key: &str) -> Option<&SessionValue> {
        self.touch();
        self.data.get(key)
    /// Remove session value
    pub fn remove(&mut self, key: &str) -> Option<SessionValue> {
        self.is_dirty = true;
        self.touch();
        self.data.remove(key)
    /// Clear all session data
    pub fn clear(&mut self) {
        self.data.clear();
        self.is_dirty = true;
        self.touch();
    /// Check if session contains key
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    /// Get all keys
    pub fn keys(&self) -> Vec<&String> {
        self.data.keys().collect()
    /// Check if session is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    /// Update last accessed time
    pub fn touch(&mut self) {
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
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
    /// Remove expiration
    pub fn clear_expiry(&mut self) {
        self.expires_at = None;
        self.is_dirty = true;
    /// Generate secure session ID
    fn generate_session_id() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        
        // Create pseudo-random session ID
        let random_data = format!("{}:{}", now, now.wrapping_mul(9973));
        Self::hash_string(&random_data)
    /// Simple hash function for session ID generation
    fn hash_string(input: &str) -> String {
        let mut hash: u64 = 5381;
        for byte in input.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        format!("{:016x}", hash)
    /// Serialize session to string (simple format)
    pub fn serialize(&self) -> String {
        let mut parts = Vec::new();
        parts.push(format!("id:{}", self.id));
        parts.push(format!("created:{}", self.created_at));
        parts.push(format!("accessed:{}", self.last_accessed));
        
        if let Some(expires) = self.expires_at {
            parts.push(format!("expires:{}", expires));
        for (key, value) in &self.data {
            parts.push(format!("data:{}:{}", key, self.serialize_value(value)));
        parts.join("|")
    /// Deserialize session from string
    pub fn deserialize(data: &str) -> crate::error::Result<()> {
        let mut session = Session::new();
        session.is_new = false;

        for part in data.split('|') {
            let components: Vec<&str> = part.splitn(2, ':').collect();
            if components.len() != 2 {
                continue;
            match components[0] {
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
    /// Serialize session value to string
    fn serialize_value(&self, value: &SessionValue) -> String {
        match value {
            SessionValue::Array(_) => "a:[]".to_string(), // Simplified
            SessionValue::Object(_) => "o:{}".to_string(), // Simplified
        }
    }

    /// Deserialize session value from string
    fn deserialize_value(data: &str) -> crate::error::Result<()> {
        let parts: Vec<&str> = data.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(SessionError::InvalidData("value".to_string()));
        match parts[0] {
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
        }
    }
impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

/// Session store trait
pub trait SessionStore {
    fn load(&self, session_id: &str) -> crate::error::Result<()>;
    fn save(&mut self, session: &Session) -> crate::error::Result<()>;
    fn delete(&mut self, session_id: &str) -> crate::error::Result<()>;
    fn cleanup_expired(&mut self) -> crate::error::Result<()>;
    fn exists(&self, session_id: &str) -> bool;
    fn count(&self) -> usize;
/// Memory-based session store
pub struct MemorySessionStore {
impl MemorySessionStore {
    pub fn new() -> Self {
        Self {
        }
    }
impl Default for MemorySessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionStore for MemorySessionStore {
    fn load(&self, session_id: &str) -> crate::error::Result<()> {
        Ok(self.sessions.get(session_id).cloned())
    fn save(&mut self, session: &Session) -> crate::error::Result<()> {
        self.sessions.insert(session.id.clone(), session.clone());
        Ok(())
    fn delete(&mut self, session_id: &str) -> crate::error::Result<()> {
        self.sessions.remove(session_id);
        Ok(())
    fn cleanup_expired(&mut self) -> crate::error::Result<()> {
        let initial_count = self.sessions.len();
        self.sessions.retain(|_, session| !session.is_expired());
        Ok(initial_count - self.sessions.len())
    fn exists(&self, session_id: &str) -> bool {
        self.sessions.contains_key(session_id)
    fn count(&self) -> usize {
        self.sessions.len()
    }
}

/// File-based session store
pub struct FileSessionStore {
impl FileSessionStore {
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    fn session_file_path(&self, session_id: &str) -> PathBuf {
        self.directory.join(format!("{}.session", session_id))
    }
}

impl SessionStore for FileSessionStore {
    fn load(&self, session_id: &str) -> crate::error::Result<()> {
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
        }
    }

    fn save(&mut self, session: &Session) -> crate::error::Result<()> {
        let file_path = self.session_file_path(&session.id);
        
        // Create directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| SessionError::StoreError(e.to_string()))?;
        let content = session.serialize();
        std::fs::write(&file_path, content)
            .map_err(|e| SessionError::StoreError(e.to_string()))?;

        Ok(())
    fn delete(&mut self, session_id: &str) -> crate::error::Result<()> {
        let file_path = self.session_file_path(session_id);
        let _ = std::fs::remove_file(&file_path); // Ignore errors
        Ok(())
    fn cleanup_expired(&mut self) -> crate::error::Result<()> {
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
    fn exists(&self, session_id: &str) -> bool {
        self.session_file_path(session_id).exists()
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
impl RedisSessionStore {
    pub fn new(connection_string: String) -> Self {
        Self {
            pool_size: 10, // Default pool size
        }
    }

    pub fn with_prefix(connection_string: String, key_prefix: String) -> Self {
        Self {
        }
    }

    pub fn with_pool_size(mut self, pool_size: usize) -> Self {
        self.pool_size = pool_size;
        self
    fn session_key(&self, session_id: &str) -> String {
        format!("{}{}", self.key_prefix, session_id)
    /// Get or create Redis client with connection pooling
    fn get_client(&self) -> crate::error::Result<()> {
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
        Ok(client_guard.as_ref().unwrap().clone())
    /// Get Redis connection with fallback handling
    fn get_connection(&self) -> crate::error::Result<()> {
        let client = self.get_client()?;
        client.get_connection()
            .map_err(|e| SessionError::StoreError(format!("Failed to get Redis connection: {}", e)))
    /// Execute Redis GET command with error handling
    fn redis_get(&self, key: &str) -> crate::error::Result<()> {
        let mut conn = self.get_connection()?;
        
        match redis::cmd("GET").arg(key).query::<Option<String>>(&mut conn) {
            Err(e) => {
                eprintln!("Redis GET error for key '{}': {}", key, e);
                Err(SessionError::StoreError(format!("Redis GET failed: {}", e)))
            }
        }
    /// Execute Redis SET command with TTL support
    fn redis_set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> crate::error::Result<()> {
        let mut conn = self.get_connection()?;
        
        let result = if let Some(ttl) = ttl_seconds {
            // SET with EX (expire in seconds)
            redis::cmd("SETEX").arg(key).arg(ttl).arg(value).query::<String>(&mut conn)
        } else {
            // SET without expiration
            redis::cmd("SET").arg(key).arg(value).query::<String>(&mut conn)
        
        match result {
            Err(e) => {
                eprintln!("Redis SET error for key '{}': {}", key, e);
                Err(SessionError::StoreError(format!("Redis SET failed: {}", e)))
            }
        }
    /// Execute Redis DEL command
    fn redis_del(&self, key: &str) -> crate::error::Result<()> {
        let mut conn = self.get_connection()?;
        
        match redis::cmd("DEL").arg(key).query::<i32>(&mut conn) {
            Err(e) => {
                eprintln!("Redis DEL error for key '{}': {}", key, e);
                Err(SessionError::StoreError(format!("Redis DEL failed: {}", e)))
            }
        }
    /// Execute Redis EXISTS command
    fn redis_exists(&self, key: &str) -> bool {
        match self.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("EXISTS").arg(key).query::<i32>(&mut conn) {
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
    /// Execute Redis TTL command to get remaining time to live
    fn redis_ttl(&self, key: &str) -> crate::error::Result<()> {
        let mut conn = self.get_connection()?;
        
        redis::cmd("TTL").arg(key).query::<i64>(&mut conn)
            .map_err(|e| SessionError::StoreError(format!("Redis TTL failed: {}", e)))
    /// Get all session keys using SCAN for better performance
    fn get_session_keys(&self) -> crate::error::Result<()> {
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
    /// Count sessions using more efficient approach
    fn count_sessions(&self) -> crate::error::Result<()> {
        let keys = self.get_session_keys()?;
        Ok(keys.len())
    /// Health check for Redis connection
    pub fn health_check(&self) -> crate::error::Result<()> {
        let mut conn = self.get_connection()?;
        let _: String = redis::cmd("PING").query(&mut conn)
            .map_err(|e| SessionError::StoreError(format!("Redis health check failed: {}", e)))?;
        Ok(())
    /// Get Redis info for monitoring
    pub fn get_redis_info(&self) -> crate::error::Result<()> {
        let mut conn = self.get_connection()?;
        redis::cmd("INFO").query::<String>(&mut conn)
            .map_err(|e| SessionError::StoreError(format!("Redis INFO failed: {}", e)))
    }
}

impl SessionStore for RedisSessionStore {
    fn load(&self, session_id: &str) -> crate::error::Result<()> {
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

    fn save(&mut self, session: &Session) -> crate::error::Result<()> {
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
        
        self.redis_set(&key, &data, ttl)?;
        println!("Session {} saved to Redis with TTL: {:?}", session.id, ttl);
        Ok(())
    fn delete(&mut self, session_id: &str) -> crate::error::Result<()> {
        let key = self.session_key(session_id);
        self.redis_del(&key)?;
        println!("Session {} deleted from Redis", session_id);
        Ok(())
    fn cleanup_expired(&mut self) -> crate::error::Result<()> {
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
        if cleaned > 0 {
            println!("Cleaned up {} expired sessions from Redis", cleaned);
        Ok(cleaned)
    fn exists(&self, session_id: &str) -> bool {
        let key = self.session_key(session_id);
        self.redis_exists(&key)
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
impl Default for DatabaseStoreConfig {
    fn default() -> Self {
        Self {
            cleanup_interval_seconds: 300, // 5 minutes
        }
    }
/// Database connection pool for session management
pub struct DatabaseConnectionPool {
//     connections: std::sync::Arc<std::sync::Mutex<Vec<crate::stdlib::database::DB>>>,
impl DatabaseConnectionPool {
    pub fn new(connection_string: String, config: DatabaseStoreConfig) -> crate::error::Result<()> {
        let driver_name = Self::detect_driver(&connection_string);
        
        let pool = Self {
        
        // Pre-populate the pool with initial connections
        pool.initialize_pool()?;
        
        Ok(pool)
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
    
    fn initialize_pool(&self) -> crate::error::Result<()> {
        let mut connections = self.connections.lock()
            .map_err(|_| SessionError::StoreError("Failed to acquire connection pool lock".to_string()))?;
            
        for _ in 0..self.config.pool_size {
            let db = self.create_connection()?;
            connections.push(db);
        Ok(())
    fn create_connection(&self) -> crate::error::Result<()> {
//         let mut db = crate::stdlib::database::DB::open(
            self.connection_string.clone()
        ).map_err(|e| SessionError::StoreError(format!("Database connection failed: {}", e)))?;
        
        // Configure database connection for optimal session performance
        self.configure_connection(&mut db)?;
        
        Ok(db)
//     fn configure_connection(&self, db: &mut crate::stdlib::database::DB) -> crate::error::Result<()> {
        if self.driver_name == "sqlite" {
            // SQLite-specific optimizations
            if self.config.enable_wal_mode {
                db.exec("PRAGMA journal_mode=WAL".to_string(), vec![])
                    .map_err(|e| SessionError::StoreError(format!("Failed to enable WAL mode: {}", e)))?;
            if self.config.enable_foreign_keys {
                db.exec("PRAGMA foreign_keys=ON".to_string(), vec![])
                    .map_err(|e| SessionError::StoreError(format!("Failed to enable foreign keys: {}", e)))?;
            if self.config.enable_auto_vacuum {
                db.exec("PRAGMA auto_vacuum=INCREMENTAL".to_string(), vec![])
                    .map_err(|e| SessionError::StoreError(format!("Failed to enable auto vacuum: {}", e)))?;
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
        Ok(())
    pub fn get_connection(&self) -> crate::error::Result<()> {
        if self.is_shutdown.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(SessionError::StoreError("Connection pool is shutdown".to_string()));
        let mut retries = 0;
        
        while retries < self.config.max_retries {
            // Try to get a connection from the pool
            if let Ok(mut connections) = self.connections.try_lock() {
                if let Some(db) = connections.pop() {
                    self.active_connections.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    return Ok(PooledConnection {
                    });
                }
            }
            
            // If pool is empty, try to create a new connection
            if self.active_connections.load(std::sync::atomic::Ordering::SeqCst) < self.config.pool_size {
                match self.create_connection() {
                    Ok(db) => {
                        self.active_connections.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        return Ok(PooledConnection {
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
        }
    }
/// A connection borrowed from the pool with transaction support
pub struct PooledConnection {
//     db: Option<crate::stdlib::database::DB>,
//     pool: std::sync::Arc<std::sync::Mutex<Vec<crate::stdlib::database::DB>>>,
impl PooledConnection {
    pub fn execute_transaction<F, R>(&mut self, transaction_fn: F) -> crate::error::Result<()>
    where
//         F: FnOnce(&mut crate::stdlib::database::DB) -> crate::error::Result<()>,
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
    
    pub fn execute_with_retry<F, R>(&mut self, operation: F) -> crate::error::Result<()>
    where
//         F: Fn(&mut crate::stdlib::database::DB) -> crate::error::Result<()>,
    {
        if let Some(ref mut db) = self.db {
            let mut retries = 0;
            const MAX_RETRIES: usize = 3;
            
            while retries < MAX_RETRIES {
                match operation(db) {
                    Err(e) => {
                        retries += 1;
                        if retries >= MAX_RETRIES {
                            return Err(e);
                        }
                        // Small delay before retry
                        std::thread::sleep(std::time::Duration::from_millis(50));
                    }
                }
            Err(SessionError::StoreError("Operation failed after retries".to_string()))
        } else {
            Err(SessionError::StoreError("Connection is not available".to_string()))
        }
    }
    
//     pub fn as_mut(&mut self) -> Option<&mut crate::stdlib::database::DB> {
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
/// Pool statistics for monitoring
#[derive(Debug)]
pub struct PoolStats {
/// Enhanced database-based session store with connection pooling and transaction safety
pub struct DatabaseSessionStore {
impl DatabaseSessionStore {
    pub fn new(connection_string: String) -> crate::error::Result<()> {
        Self::new_with_config(connection_string, DatabaseStoreConfig::default())
    pub fn new_with_config(connection_string: String, config: DatabaseStoreConfig) -> crate::error::Result<()> {
        let pool = std::sync::Arc::new(DatabaseConnectionPool::new(connection_string.clone(), config.clone())?);
        
        let store = Self {
        
        // Initialize database schema
        store.init_schema()?;
        
        // Start automatic cleanup task
        store.start_cleanup_task();
        
        Ok(store)
    /// Initialize database schema with proper indexing and constraints
    fn init_schema(&self) -> crate::error::Result<()> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_transaction(|db| {
            // Create main sessions table with optimized schema
            let create_table_sql = format!(r#"
                CREATE TABLE IF NOT EXISTS {} (
                    schema_version INTEGER DEFAULT 1
                )
            "#, self.table_name);

            db.exec(create_table_sql, vec![])
                .map_err(|e| SessionError::StoreError(format!("Failed to create session table: {}", e)))?;
            
            // Create indexes for optimal query performance
            let indexes = vec![
            ];
            
            for index_sql in indexes {
                db.exec(index_sql, vec![])
                    .map_err(|e| SessionError::StoreError(format!("Failed to create index: {}", e)))?;
            // Create metadata table for schema versioning
            let metadata_sql = format!(r#"
                CREATE TABLE IF NOT EXISTS {}_metadata (
                    updated_at INTEGER NOT NULL
                )
            "#, self.table_name);
            
            db.exec(metadata_sql, vec![])
                .map_err(|e| SessionError::StoreError(format!("Failed to create metadata table: {}", e)))?;
            
            Ok(())
        })?;
        
        Ok(())
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
                            table_name
                        );
                        
//                         db.exec(cleanup_sql, vec![crate::stdlib::database::SqlValue::Integer(now)])
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
    /// Shutdown the session store and cleanup resources
    pub fn shutdown(&self) {
        self.pool.shutdown();
        
        if let Ok(mut cleanup_guard) = self.cleanup_handle.lock() {
            if let Some(handle) = cleanup_guard.take() {
                // We can't join the thread since it's in an infinite loop
                // In a real implementation, we'd use a shutdown signal
            }
        }
    /// Calculate data checksum for integrity verification
    fn calculate_checksum(&self, data: &str) -> String {
        let mut hash: u64 = 5381;
        for byte in data.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        format!("{:016x}", hash)
    /// Initialize database connection and create session table
    fn ensure_connection(&self) -> crate::error::Result<()> {
        self.pool.get_connection()
    /// Load session data from database
    fn db_select(&self, session_id: &str) -> crate::error::Result<()> {
        let mut connection = self.ensure_connection()?;
        
        let sql = format!(
            self.table_name
        );
        
        connection.execute_with_retry(|db| {
//             let row = db.query_row(sql.clone(), vec![crate::stdlib::database::SqlValue::String(session_id.to_string())]);
            let map = row.scan_map().unwrap_or_default();
//             if let Some(crate::stdlib::database::SqlValue::String(data)) = map.get("session_data") {
                Ok(Some(data.clone()))
            } else {
                Ok(None)
            }
        })
    /// Insert or update session in database
    fn db_insert_or_update(&self, session: &Session) -> crate::error::Result<()> {
        let mut connection = self.ensure_connection()?;
        
        let sql = format!(r#"
            INSERT OR REPLACE INTO {} 
            (id, session_data, created_at, last_accessed, expires_at) 
            VALUES (?, ?, ?, ?, ?)
        "#, self.table_name);
        
//         let expires_at = session.expires_at.map(|t| crate::stdlib::database::SqlValue::Integer(t as i64))
//             .unwrap_or(crate::stdlib::database::SqlValue::Null);
        
        connection.execute_transaction(|db| {
            db.exec(sql, vec![
//                 crate::stdlib::database::SqlValue::String(session.id.clone()),
//                 crate::stdlib::database::SqlValue::String(session.serialize()),
//                 crate::stdlib::database::SqlValue::Integer(session.created_at as i64),
//                 crate::stdlib::database::SqlValue::Integer(session.last_accessed as i64),
            ]).map_err(|e| SessionError::StoreError(format!("Failed to save session: {}", e)))?;
            Ok(())
        })?;
        
        Ok(())
    /// Delete session from database
    fn db_delete(&self, session_id: &str) -> crate::error::Result<()> {
        let mut connection = self.ensure_connection()?;
        
        let sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
        
        connection.execute_transaction(|db| {
//             db.exec(sql, vec![crate::stdlib::database::SqlValue::String(session_id.to_string())])
                .map_err(|e| SessionError::StoreError(format!("Failed to delete session: {}", e)))?;
            Ok(())
        })?;
        
        Ok(())
    /// Count total sessions in database
    fn db_count(&self) -> crate::error::Result<()> {
        let mut connection = self.ensure_connection()?;
        
        let sql = format!("SELECT COUNT(*) as count FROM {}", self.table_name);
        
        connection.execute_with_retry(|db| {
            let row = db.query_row(sql.clone(), vec![]);
            let map = row.scan_map().unwrap_or_default();
//             if let Some(crate::stdlib::database::SqlValue::Integer(count)) = map.get("count") {
                Ok(*count as usize)
            } else {
                Ok(0)
            }
        })
    /// Clean up expired sessions from database
    fn db_cleanup_expired(&self) -> crate::error::Result<()> {
        let mut connection = self.ensure_connection()?;
        
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        
        let sql = format!(
            self.table_name
        );
        
        let result = connection.execute_transaction(|db| {
//             db.exec(sql, vec![crate::stdlib::database::SqlValue::Integer(now)])
                .map_err(|e| SessionError::StoreError(format!("Failed to cleanup expired sessions: {}", e)))
        })?;
        
        Ok(result.rows_affected as usize)
    /// Check if session exists in database
    fn db_exists(&self, session_id: &str) -> crate::error::Result<()> {
        let mut connection = self.ensure_connection()?;
        
        let sql = format!("SELECT 1 FROM {} WHERE id = ? LIMIT 1", self.table_name);
        
        connection.execute_with_retry(|db| {
//             let _row = db.query_row(sql.clone(), vec![crate::stdlib::database::SqlValue::String(session_id.to_string())]);
            // If we get a row back, the session exists
            Ok(true)
        })
    }
}

impl SessionStore for DatabaseSessionStore {
    fn load(&self, session_id: &str) -> crate::error::Result<()> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_with_retry(|db| {
            let sql = format!(
                self.table_name
            );
            
//             let row = db.query_row(sql.clone(), vec![crate::stdlib::database::SqlValue::String(session_id.to_string())]);
            let map = row.scan_map().unwrap_or_default();
//             if let (Some(crate::stdlib::database::SqlValue::String(data)), checksum) = 
                (map.get("session_data"), map.get("data_checksum")) {
                
                // Verify data integrity if checksum exists
//                 if let Some(crate::stdlib::database::SqlValue::String(stored_checksum)) = checksum {
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
    fn save(&mut self, session: &Session) -> crate::error::Result<()> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_transaction(|db| {
            let session_data = session.serialize();
            let checksum = self.calculate_checksum(&session_data);
            
            let sql = format!(r#"
                INSERT OR REPLACE INTO {} 
                (id, session_data, created_at, last_accessed, expires_at, data_checksum, schema_version) 
                VALUES (?, ?, ?, ?, ?, ?, ?)
            "#, self.table_name);
            
//             let expires_at = session.expires_at.map(|t| crate::stdlib::database::SqlValue::Integer(t as i64))
//                 .unwrap_or(crate::stdlib::database::SqlValue::Null);
            
            db.exec(sql, vec![
//                 crate::stdlib::database::SqlValue::String(session.id.clone()),
//                 crate::stdlib::database::SqlValue::String(session_data),
//                 crate::stdlib::database::SqlValue::Integer(session.created_at as i64),
//                 crate::stdlib::database::SqlValue::Integer(session.last_accessed as i64),
//                 crate::stdlib::database::SqlValue::String(checksum),
//                 crate::stdlib::database::SqlValue::Integer(self.schema_version as i64),
            ]).map_err(|e| SessionError::StoreError(format!("Failed to save session: {}", e)))?;
            
            Ok(())
        })
    fn delete(&mut self, session_id: &str) -> crate::error::Result<()> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_transaction(|db| {
            let sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
            
//             db.exec(sql, vec![crate::stdlib::database::SqlValue::String(session_id.to_string())])
                .map_err(|e| SessionError::StoreError(format!("Failed to delete session: {}", e)))?;
            
            Ok(())
        })
    fn cleanup_expired(&mut self) -> crate::error::Result<()> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_transaction(|db| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            
            let sql = format!(
                self.table_name
            );
            
//             let result = db.exec(sql, vec![crate::stdlib::database::SqlValue::Integer(now)])
                .map_err(|e| SessionError::StoreError(format!("Failed to cleanup expired sessions: {}", e)))?;
            
            // Run incremental vacuum after cleanup
            let _ = db.exec("PRAGMA incremental_vacuum(100)".to_string(), vec![]);
            
            Ok(result.rows_affected as usize)
        })
    fn exists(&self, session_id: &str) -> bool {
        self.pool.get_connection()
            .and_then(|mut connection| {
                connection.execute_with_retry(|db| {
                    let sql = format!("SELECT 1 FROM {} WHERE id = ? LIMIT 1", self.table_name);
                    
//                     let _row = db.query_row(sql, vec![crate::stdlib::database::SqlValue::String(session_id.to_string())]);
                    Ok(true)
                })
            })
            .unwrap_or(false)
    fn count(&self) -> usize {
        self.pool.get_connection()
            .and_then(|mut connection| {
                connection.execute_with_retry(|db| {
                    let sql = format!("SELECT COUNT(*) as count FROM {}", self.table_name);
                    
                    let row = db.query_row(sql.clone(), vec![]);
                    let map = row.scan_map().unwrap_or_default();
//                     if let Some(crate::stdlib::database::SqlValue::Integer(count)) = map.get("count") {
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
impl SessionManager {
    /// Create new session manager
    pub fn new(config: SessionConfig) -> crate::error::Result<()> {
        let store: Box<dyn SessionStore> = match &config.store_type {
            SessionStoreType::Redis(connection_string) => {
                Box::new(RedisSessionStore::new(connection_string.clone()))
            }
            SessionStoreType::Database(connection_string) => {
                Box::new(DatabaseSessionStore::new(connection_string.clone())?)
            }

        Ok(Self { store, config })
    /// Create new session
    pub fn create_session(&mut self) -> crate::error::Result<()> {
        let mut session = Session::new();
        session.set_expiry(self.config.max_age.as_secs());
        self.store.save(&session)?;
        Ok(session)
    /// Load existing session
    pub fn load_session(&self, session_id: &str) -> crate::error::Result<()> {
        self.store.load(session_id)
    /// Save session
    pub fn save_session(&mut self, session: &Session) -> crate::error::Result<()> {
        if session.is_dirty {
            self.store.save(session)?;
        }
        Ok(())
    /// Delete session
    pub fn delete_session(&mut self, session_id: &str) -> crate::error::Result<()> {
        self.store.delete(session_id)
    /// Generate session cookie
    pub fn create_session_cookie(&self, session: &Session) -> String {
        let mut cookie_parts = vec![
        ];

        if self.config.http_only {
            cookie_parts.push("HttpOnly".to_string());
        if self.config.secure {
            cookie_parts.push("Secure".to_string());
        let same_site = match self.config.same_site {
        cookie_parts.push(same_site.to_string());

        let max_age = self.config.max_age.as_secs();
        cookie_parts.push(format!("Max-Age={}", max_age));

        cookie_parts.join("; ")
    /// Parse session ID from cookie header
    pub fn parse_session_id_from_cookie(&self, cookie_header: &str) -> Option<String> {
        for cookie in cookie_header.split(';') {
            let cookie = cookie.trim();
            if cookie.starts_with(&format!("{}=", self.config.cookie_name)) {
                return Some(cookie[self.config.cookie_name.len() + 1..].to_string());
            }
        }
        None
    /// Cleanup expired sessions
    pub fn cleanup_expired_sessions(&mut self) -> crate::error::Result<()> {
        self.store.cleanup_expired()
    /// Get session statistics
    pub fn get_session_stats(&self) -> SessionStats {
        SessionStats {
        }
    }
/// Session error types
#[derive(Debug)]
pub enum SessionError {
// impl fmt::Display for SessionError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             SessionError::InvalidData(field) => write!(f, "Invalid session data: {}", field),
//             SessionError::StoreError(msg) => write!(f, "Session store error: {}", msg),
//             SessionError::SerializationError(msg) => write!(f, "Session serialization error: {}", msg),
//             SessionError::NotFound(id) => write!(f, "Session not found: {}", id),
//             SessionError::CorruptedData(msg) => write!(f, "Session data corrupted: {}", msg),
//             SessionError::ConnectionPoolExhausted => write!(f, "Database connection pool exhausted"),
//             SessionError::TransactionFailed(msg) => write!(f, "Database transaction failed: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for SessionError {}
// 
/// Session statistics
#[derive(Debug)]
pub struct SessionStats {
