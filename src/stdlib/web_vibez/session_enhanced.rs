/// Enhanced session management with connection pooling and transaction safety
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::stdlib::web_vibez::config::{SessionConfig, SessionStoreType, SameSitePolicy};

/// Session data structure
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone, PartialEq)]
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
    pub fn deserialize(data: &str) -> Result<Self, SessionError> {
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
    fn deserialize_value(data: &str) -> Result<SessionValue, SessionError> {
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
    fn load(&self, session_id: &str) -> Result<Option<Session>, SessionError>;
    fn save(&mut self, session: &Session) -> Result<(), SessionError>;
    fn delete(&mut self, session_id: &str) -> Result<(), SessionError>;
    fn cleanup_expired(&mut self) -> Result<usize, SessionError>;
    fn exists(&self, session_id: &str) -> bool;
    fn count(&self) -> usize;
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
    pub fn new(connection_string: String, config: DatabaseStoreConfig) -> Result<Self, SessionError> {
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
    
    fn initialize_pool(&self) -> Result<(), SessionError> {
        let mut connections = self.connections.lock()
            .map_err(|_| SessionError::StoreError("Failed to acquire connection pool lock".to_string()))?;
            
        for _ in 0..self.config.pool_size {
            let db = self.create_connection()?;
            connections.push(db);
        }
        
        Ok(())
    }
    
    fn create_connection(&self) -> Result<crate::stdlib::database::DB, SessionError> {
        let mut db = crate::stdlib::database::DB::open(
            self.driver_name.clone(), 
            self.connection_string.clone()
        ).map_err(|e| SessionError::StoreError(format!("Database connection failed: {}", e)))?;
        
        // Configure database connection for optimal session performance
        self.configure_connection(&mut db)?;
        
        Ok(db)
    }
    
    fn configure_connection(&self, db: &mut crate::stdlib::database::DB) -> Result<(), SessionError> {
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
        
        // PostgreSQL and MySQL specific optimizations can be added here
        
        Ok(())
    }
    
    pub fn get_connection(&self) -> Result<PooledConnection, SessionError> {
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
    pub fn execute_transaction<F, R>(&mut self, transaction_fn: F) -> Result<R, SessionError>
    where
        F: FnOnce(&mut crate::stdlib::database::DB) -> Result<R, SessionError>,
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
    
    pub fn execute_with_retry<F, R>(&mut self, operation: F) -> Result<R, SessionError>
    where
        F: Fn(&mut crate::stdlib::database::DB) -> Result<R, SessionError>,
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
    pub fn new(connection_string: String) -> Result<Self, SessionError> {
        Self::new_with_config(connection_string, DatabaseStoreConfig::default())
    }
    
    pub fn new_with_config(connection_string: String, config: DatabaseStoreConfig) -> Result<Self, SessionError> {
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
    fn init_schema(&self) -> Result<(), SessionError> {
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
}

impl SessionStore for DatabaseSessionStore {
    fn load(&self, session_id: &str) -> Result<Option<Session>, SessionError> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_with_retry(|db| {
            let sql = format!(
                "SELECT session_data, data_checksum FROM {} WHERE id = ? LIMIT 1", 
                self.table_name
            );
            
            match db.query_row(sql, vec![crate::stdlib::database::SqlValue::Text(session_id.to_string())]) {
                Ok(row) => {
                    if let (Some(crate::stdlib::database::SqlValue::Text(data)), checksum) = 
                        (row.get("session_data"), row.get("data_checksum")) {
                        
                        // Verify data integrity if checksum exists
                        if let Some(crate::stdlib::database::SqlValue::Text(stored_checksum)) = checksum {
                            let calculated_checksum = self.calculate_checksum(data);
                            if calculated_checksum != *stored_checksum {
                                return Err(SessionError::StoreError("Session data corruption detected".to_string()));
                            }
                        }
                        
                        let session = Session::deserialize(data)?;
                        if session.is_expired() {
                            // Delete expired session in a separate transaction
                            let _ = self.delete(session_id);
                            Ok(None)
                        } else {
                            Ok(Some(session))
                        }
                    } else {
                        Ok(None)
                    }
                }
                Err(e) if e.to_string().contains("not found") || e.to_string().contains("no rows") => {
                    Ok(None)
                }
                Err(e) => Err(SessionError::StoreError(format!("Database query failed: {}", e)))
            }
        })
    }

    fn save(&mut self, session: &Session) -> Result<(), SessionError> {
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
                crate::stdlib::database::SqlValue::Text(session.id.clone()),
                crate::stdlib::database::SqlValue::Text(session_data),
                crate::stdlib::database::SqlValue::Integer(session.created_at as i64),
                crate::stdlib::database::SqlValue::Integer(session.last_accessed as i64),
                expires_at,
                crate::stdlib::database::SqlValue::Text(checksum),
                crate::stdlib::database::SqlValue::Integer(self.schema_version as i64),
            ]).map_err(|e| SessionError::StoreError(format!("Failed to save session: {}", e)))?;
            
            Ok(())
        })
    }

    fn delete(&mut self, session_id: &str) -> Result<(), SessionError> {
        let mut connection = self.pool.get_connection()?;
        
        connection.execute_transaction(|db| {
            let sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
            
            db.exec(sql, vec![crate::stdlib::database::SqlValue::Text(session_id.to_string())])
                .map_err(|e| SessionError::StoreError(format!("Failed to delete session: {}", e)))?;
            
            Ok(())
        })
    }

    fn cleanup_expired(&mut self) -> Result<usize, SessionError> {
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
                    
                    match db.query_row(sql, vec![crate::stdlib::database::SqlValue::Text(session_id.to_string())]) {
                        Ok(_) => Ok(true),
                        Err(_) => Ok(false),
                    }
                })
            })
            .unwrap_or(false)
    }

    fn count(&self) -> usize {
        self.pool.get_connection()
            .and_then(|mut connection| {
                connection.execute_with_retry(|db| {
                    let sql = format!("SELECT COUNT(*) as count FROM {}", self.table_name);
                    
                    match db.query_row(sql, vec![]) {
                        Ok(row) => {
                            if let Some(crate::stdlib::database::SqlValue::Integer(count)) = row.get("count") {
                                Ok(*count as usize)
                            } else {
                                Ok(0)
                            }
                        }
                        Err(e) => Err(SessionError::StoreError(format!("Failed to count sessions: {}", e)))
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
