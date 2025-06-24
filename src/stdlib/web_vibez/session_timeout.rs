/// Session management with timeout support
/// 
/// Provides timeout-aware session stores with integration to the timeout middleware

use crate::stdlib::web_vibez::session::{Session, SessionValue, SessionError};
use crate::stdlib::web_vibez::timeout_middleware::{TimeoutMiddleware, TimeoutResult, TimeoutError};
use crate::stdlib::web_vibez::config::{SessionConfig, SessionStoreType};
use crate::stdlib::database::{DatabaseError, DatabaseConfig};
use crate::error::Error;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use std::fs;
use std::io::{Read, Write};
use parking_lot::Mutex;
use tracing::{debug, info, warn, error, instrument};
use uuid::Uuid;
use redis::{Client as RedisClient, Connection as RedisConnection, Commands, RedisError, AsyncCommands};
use tokio::sync::Mutex as AsyncMutex;

/// Session store with timeout support - using enum wrapper instead of trait object
#[derive(Debug)]
pub enum TimeoutAwareSessionStore {
    Memory(TimeoutMemorySessionStore),
    File(TimeoutFileSessionStore),
    Redis(TimeoutRedisSessionStore),
    Database(TimeoutDatabaseSessionStore),
}

impl TimeoutAwareSessionStore {
    /// Load session with timeout
    pub async fn load_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<Option<Session>> {
        match self {
            TimeoutAwareSessionStore::Memory(store) => {
                store.load_with_timeout(session_id, timeout_middleware).await
            }
            TimeoutAwareSessionStore::File(store) => {
                store.load_with_timeout(session_id, timeout_middleware).await
            }
            TimeoutAwareSessionStore::Redis(store) => {
                store.load_with_timeout(session_id, timeout_middleware).await
            }
            TimeoutAwareSessionStore::Database(store) => {
                store.load_with_timeout(session_id, timeout_middleware).await
            }
        }
    }

    /// Save session with timeout
    pub async fn save_with_timeout(
        &self,
        session: &Session,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        match self {
            TimeoutAwareSessionStore::Memory(store) => {
                store.save_with_timeout(session, timeout_middleware).await
            }
            TimeoutAwareSessionStore::File(store) => {
                store.save_with_timeout(session, timeout_middleware).await
            }
            TimeoutAwareSessionStore::Redis(store) => {
                store.save_with_timeout(session, timeout_middleware).await
            }
            TimeoutAwareSessionStore::Database(store) => {
                store.save_with_timeout(session, timeout_middleware).await
            }
        }
    }

    /// Delete session with timeout
    pub async fn delete_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        match self {
            TimeoutAwareSessionStore::Memory(store) => {
                store.delete_with_timeout(session_id, timeout_middleware).await
            }
            TimeoutAwareSessionStore::File(store) => {
                store.delete_with_timeout(session_id, timeout_middleware).await
            }
            TimeoutAwareSessionStore::Redis(store) => {
                store.delete_with_timeout(session_id, timeout_middleware).await
            }
            TimeoutAwareSessionStore::Database(store) => {
                store.delete_with_timeout(session_id, timeout_middleware).await
            }
        }
    }

    /// Cleanup expired sessions with timeout
    pub async fn cleanup_with_timeout(
        &self,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<usize> {
        match self {
            TimeoutAwareSessionStore::Memory(store) => {
                store.cleanup_with_timeout(timeout_middleware).await
            }
            TimeoutAwareSessionStore::File(store) => {
                store.cleanup_with_timeout(timeout_middleware).await
            }
            TimeoutAwareSessionStore::Redis(store) => {
                store.cleanup_with_timeout(timeout_middleware).await
            }
            TimeoutAwareSessionStore::Database(store) => {
                store.cleanup_with_timeout(timeout_middleware).await
            }
        }
    }

    /// Check if session exists with timeout
    pub async fn exists_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<bool> {
        match self {
            TimeoutAwareSessionStore::Memory(store) => {
                store.exists_with_timeout(session_id, timeout_middleware).await
            }
            TimeoutAwareSessionStore::File(store) => {
                store.exists_with_timeout(session_id, timeout_middleware).await
            }
            TimeoutAwareSessionStore::Redis(store) => {
                store.exists_with_timeout(session_id, timeout_middleware).await
            }
            TimeoutAwareSessionStore::Database(store) => {
                store.exists_with_timeout(session_id, timeout_middleware).await
            }
        }
    }
}

/// Redis session store with timeout support
#[derive(Debug)]
pub struct TimeoutRedisSessionStore {
    client: RedisClient,
    connection: Arc<AsyncMutex<redis::aio::Connection>>,
    config: SessionConfig,
    key_prefix: String,
}

impl TimeoutRedisSessionStore {
    pub async fn new(redis_url: &str, config: SessionConfig) -> Result<(), Error> {
        let client = RedisClient::open(redis_url)
            .map_err(|e| SessionError::StoreError(format!("Failed to create Redis client: {}", e)))?;
        
        let connection = client.get_async_connection().await
            .map_err(|e| SessionError::StoreError(format!("Failed to connect to Redis: {}", e)))?;
        
        Ok(Self {
            client,
            connection: Arc::new(AsyncMutex::new(connection)),
            config,
            key_prefix: "cursed:session:".to_string(),
        })
    }

    fn session_key(&self, session_id: &str) -> String {
        format!("{}{}", self.key_prefix, session_id)
    }

    fn serialize_session(&self, session: &Session) -> Result<(), Error> {
        serde_json::to_string(session)
            .map_err(|e| SessionError::SerializationError(format!("Failed to serialize session: {}", e)))
    }

    fn deserialize_session(&self, data: &str) -> Result<(), Error> {
        serde_json::from_str(data)
            .map_err(|e| SessionError::SerializationError(format!("Failed to deserialize session: {}", e)))
    }

    fn is_session_expired(&self, session: &Session) -> bool {
        if let Some(expires_at) = session.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            expires_at <= now
        } else {
            false
        }
    }

    async fn cleanup_expired_sessions(&self) -> TimeoutResult<usize> {
        let mut conn = self.connection.lock().await;
        let mut removed = 0;

        // Use SCAN to iterate through session keys
        let pattern = format!("{}*", self.key_prefix);
        let mut iter: redis::AsyncIter<String> = conn.scan_match(&pattern).await
            .map_err(|e| TimeoutError::DatabaseTimeout {
                elapsed: Duration::from_secs(0),
                timeout: self.config.database_timeout,
                operation: format!("redis_scan: {}", e),
            })?;

        while let Some(key) = iter.next_item().await {
            if let Ok(data) = conn.get::<String, String>(key.clone()).await {
                if let Ok(session) = self.deserialize_session(&data) {
                    if self.is_session_expired(&session) {
                        match conn.del::<String, i32>(key.clone()).await {
                            Ok(_) => {
                                removed += 1;
                                debug!(key = %key, "Removed expired session from Redis");
                            }
                            Err(e) => {
                                warn!(key = %key, error = %e, "Failed to remove expired session from Redis");
                            }
                        }
                    }
                }
            }
        }

        Ok(removed)
    }
}

impl TimeoutRedisSessionStore {
    async fn load_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<Option<Session>> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_load_redis".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let key = self.session_key(session_id);
                let mut conn = self.connection.lock().await;
                
                match conn.get::<String, Option<String>>(key.clone()).await {
                    Ok(Some(data)) => {
                        match self.deserialize_session(&data) {
                            Ok(session) => {
                                if self.is_session_expired(&session) {
                                    // Remove expired session
                                    let _ = conn.del::<String, i32>(key.clone()).await;
                                    debug!(session_id = %session_id, "Session expired and removed from Redis");
                                    None
                                } else {
                                    debug!(session_id = %session_id, "Session loaded from Redis store");
                                    Some(session)
                                }
                            }
                            Err(e) => {
                                error!(session_id = %session_id, error = %e, "Failed to deserialize session from Redis");
                                None
                            }
                        }
                    }
                    Ok(None) => {
                        debug!(session_id = %session_id, "Session not found in Redis store");
                        None
                    }
                    Err(e) => {
                        error!(session_id = %session_id, error = %e, "Failed to load session from Redis");
                        None
                    }
                }
            }
        ).await?
    }

    async fn save_with_timeout(
        &self,
        session: &Session,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_save_redis".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let key = self.session_key(&session.id);
                let data = self.serialize_session(session)
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("serialize_session: {}", e),
                    })?;

                let mut conn = self.connection.lock().await;
                
                // Set with expiration if available
                if let Some(expires_at) = session.expires_at {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    
                    if expires_at > now {
                        let ttl = (expires_at - now) as u64;
                        conn.set_ex::<String, String, String>(key.clone(), data.clone(), ttl).await
                            .map_err(|e| TimeoutError::DatabaseTimeout {
                                elapsed: Duration::from_secs(0),
                                timeout: self.config.database_timeout,
                                operation: format!("redis_set_ex: {}", e),
                            })?;
                    } else {
                        // Session already expired, don't save
                        debug!(session_id = %session.id, "Session already expired, not saving to Redis");
                        return Ok(());
                    }
                } else {
                    // No expiration, use default TTL
                    let ttl = self.config.max_age.as_secs();
                    conn.set_ex::<String, String, String>(key.clone(), data.clone(), ttl).await
                        .map_err(|e| TimeoutError::DatabaseTimeout {
                            elapsed: Duration::from_secs(0),
                            timeout: self.config.database_timeout,
                            operation: format!("redis_set_ex_default: {}", e),
                        })?;
                }

                debug!(session_id = %session.id, "Session saved to Redis store");
                Ok(())
            }
        ).await?
    }

    async fn delete_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_delete_redis".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let key = self.session_key(session_id);
                let mut conn = self.connection.lock().await;
                
                match conn.del::<String, i32>(key.clone()).await {
                    Ok(deleted_count) => {
                        if deleted_count > 0 {
                            debug!(session_id = %session_id, "Session deleted from Redis store");
                        } else {
                            debug!(session_id = %session_id, "Session not found for deletion in Redis store");
                        }
                    }
                    Err(e) => {
                        warn!(session_id = %session_id, error = %e, "Failed to delete session from Redis");
                    }
                }
            }
        ).await?
    }

    async fn cleanup_with_timeout(
        &self,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<usize> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_cleanup_redis".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            self.cleanup_expired_sessions()
        ).await?
    }

    async fn exists_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<bool> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_exists_redis".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let key = self.session_key(session_id);
                let mut conn = self.connection.lock().await;
                
                match conn.exists::<String, bool>(key.clone()).await {
                    Ok(exists) => {
                        if exists {
                            // Check if session is expired by loading it
                            if let Ok(Some(data)) = conn.get::<String, Option<String>>(key.clone()).await {
                                if let Ok(session) = self.deserialize_session(&data) {
                                    if self.is_session_expired(&session) {
                                        // Remove expired session
                                        let _ = conn.del::<String, i32>(key.clone()).await;
                                        debug!(session_id = %session_id, "Session expired and removed during exists check");
                                        false
                                    } else {
                                        true
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                }
            }
        ).await?
    }
}

/// Database session store with timeout support
#[derive(Debug)]
pub struct TimeoutDatabaseSessionStore {
    pool: Arc<ConnectionPool>,
    config: SessionConfig,
    table_name: String,
}

impl TimeoutDatabaseSessionStore {
    pub async fn new(database_config: DatabaseConfig, config: SessionConfig) -> Result<(), Error> {
        let pool = ConnectionPool::new(database_config)
            .await
            .map_err(|e| SessionError::StoreError(format!("Failed to create database pool: {}", e)))?;

        let store = Self {
            pool: Arc::new(pool),
            config,
            table_name: "cursed_sessions".to_string(),
        };

        // Initialize session table
        store.initialize_table().await?;
        
        Ok(store)
    }

    async fn initialize_table(&self) -> Result<(), Error> {
        let mut conn = self.pool.get_connection().await
            .map_err(|e| SessionError::StoreError(format!("Failed to get database connection: {}", e)))?;

        let create_table_sql = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                id VARCHAR(255) PRIMARY KEY,
                data TEXT NOT NULL,
                created_at BIGINT NOT NULL,
                last_accessed BIGINT NOT NULL,
                expires_at BIGINT,
                INDEX idx_expires_at (expires_at)
            )
            "#,
            self.table_name
        );

        conn.execute(&create_table_sql, &[]).await
            .map_err(|e| SessionError::StoreError(format!("Failed to create sessions table: {}", e)))?;

        debug!(table = %self.table_name, "Session table initialized");
        Ok(())
    }

    fn serialize_session(&self, session: &Session) -> Result<(), Error> {
        serde_json::to_string(session)
            .map_err(|e| SessionError::SerializationError(format!("Failed to serialize session: {}", e)))
    }

    fn deserialize_session(&self, data: &str) -> Result<(), Error> {
        serde_json::from_str(data)
            .map_err(|e| SessionError::SerializationError(format!("Failed to deserialize session: {}", e)))
    }

    fn is_session_expired(&self, session: &Session) -> bool {
        if let Some(expires_at) = session.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            expires_at <= now
        } else {
            false
        }
    }

    async fn cleanup_expired_sessions(&self) -> TimeoutResult<usize> {
        let mut conn = self.pool.get_connection().await
            .map_err(|e| TimeoutError::DatabaseTimeout {
                elapsed: Duration::from_secs(0),
                timeout: self.config.database_timeout,
                operation: format!("get_connection: {}", e),
            })?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let delete_sql = format!(
            "DELETE FROM {} WHERE expires_at IS NOT NULL AND expires_at <= ?",
            self.table_name
        );

        let result = conn.execute(&delete_sql, &[&now]).await
            .map_err(|e| TimeoutError::DatabaseTimeout {
                elapsed: Duration::from_secs(0),
                timeout: self.config.database_timeout,
                operation: format!("delete_expired: {}", e),
            })?;

        let removed = result.rows_affected() as usize;
        if removed > 0 {
            debug!(removed = removed, "Cleaned up expired sessions from database");
        }

        Ok(removed)
    }
}

impl TimeoutDatabaseSessionStore {
    async fn load_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<Option<Session>> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_load_database".to_string();
        let pool = self.pool.clone();
        let config = self.config.clone();
        let table_name = self.table_name.clone();
        let session_id = session_id.to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let mut conn = pool.get_connection().await
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("get_connection: {}", e),
                    })?;

                let select_sql = format!(
                    "SELECT data FROM {} WHERE id = ?",
                    self.table_name
                );

                let rows = conn.query(&select_sql, &[&session_id]).await
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("select_session: {}", e),
                    })?;

                if let Some(row) = rows.first() {
                    let data: String = row.get(0)
                        .map_err(|e| TimeoutError::DatabaseTimeout {
                            elapsed: Duration::from_secs(0),
                            timeout: self.config.database_timeout,
                            operation: format!("get_session_data: {}", e),
                        })?;

                    match self.deserialize_session(&data) {
                        Ok(session) => {
                            if self.is_session_expired(&session) {
                                // Remove expired session
                                let delete_sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
                                let _ = conn.execute(&delete_sql, &[&session_id]).await;
                                debug!(session_id = %session_id, "Session expired and removed from database");
                                Ok(None)
                            } else {
                                debug!(session_id = %session_id, "Session loaded from database store");
                                Ok(Some(session))
                            }
                        }
                        Err(e) => {
                            error!(session_id = %session_id, error = %e, "Failed to deserialize session from database");
                            Ok(None)
                        }
                    }
                } else {
                    debug!(session_id = %session_id, "Session not found in database store");
                    Ok(None)
                }
            }
        ).await?
    }

    async fn save_with_timeout(
        &self,
        session: &Session,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_save_database".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let data = self.serialize_session(session)
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("serialize_session: {}", e),
                    })?;

                let mut conn = self.pool.get_connection().await
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("get_connection: {}", e),
                    })?;

                let upsert_sql = format!(
                    r#"
                    INSERT INTO {} (id, data, created_at, last_accessed, expires_at)
                    VALUES (?, ?, ?, ?, ?)
                    ON DUPLICATE KEY UPDATE
                        data = VALUES(data),
                        last_accessed = VALUES(last_accessed),
                        expires_at = VALUES(expires_at)
                    "#,
                    self.table_name
                );

                conn.execute(&upsert_sql, &[
                    &session.id,
                    &data,
                    &(session.created_at as i64).to_string(),
                    &(session.last_accessed as i64).to_string(),
                    &session.expires_at.map(|t| (t as i64).to_string()).unwrap_or_default(),
                ]).await
                .map_err(|e| TimeoutError::DatabaseTimeout {
                    elapsed: Duration::from_secs(0),
                    timeout: self.config.database_timeout,
                    operation: format!("upsert_session: {}", e),
                })?;

                debug!(session_id = %session.id, "Session saved to database store");
                Ok(())
            }
        ).await?
    }

    async fn delete_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_delete_database".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let mut conn = self.pool.get_connection().await
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("get_connection: {}", e),
                    })?;

                let delete_sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
                
                let result = conn.execute(&delete_sql, &[&session_id]).await
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("delete_session: {}", e),
                    })?;

                if result.rows_affected() > 0 {
                    debug!(session_id = %session_id, "Session deleted from database store");
                } else {
                    debug!(session_id = %session_id, "Session not found for deletion in database store");
                }

                Ok(())
            }
        ).await?
    }

    async fn cleanup_with_timeout(
        &self,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<usize> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_cleanup_database".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            self.cleanup_expired_sessions()
        ).await?
    }

    async fn exists_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<bool> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_exists_database".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let mut conn = self.pool.get_connection().await
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("get_connection: {}", e),
                    })?;

                let select_sql = format!(
                    "SELECT data FROM {} WHERE id = ?",
                    self.table_name
                );

                let rows = conn.query(&select_sql, &[&session_id]).await
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("select_session: {}", e),
                    })?;

                if let Some(row) = rows.first() {
                    let data: String = row.get(0)
                        .map_err(|e| TimeoutError::DatabaseTimeout {
                            elapsed: Duration::from_secs(0),
                            timeout: self.config.database_timeout,
                            operation: format!("get_session_data: {}", e),
                        })?;

                    if let Ok(session) = self.deserialize_session(&data) {
                        if self.is_session_expired(&session) {
                            // Remove expired session
                            let delete_sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
                            let _ = conn.execute(&delete_sql, &[&session_id]).await;
                            debug!(session_id = %session_id, "Session expired and removed during exists check");
                            Ok(false)
                        } else {
                            Ok(true)
                        }
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
        ).await?
    }
}

/// Memory session store with timeout support
#[derive(Debug)]
pub struct TimeoutMemorySessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    config: SessionConfig,
}

impl TimeoutMemorySessionStore {
    pub fn new(config: SessionConfig) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    fn is_session_expired(&self, session: &Session) -> bool {
        if let Some(expires_at) = session.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            expires_at <= now
        } else {
            false
        }
    }

    fn cleanup_expired_sessions(&self) -> usize {
        let mut sessions = self.sessions.write().unwrap();
        let initial_count = sessions.len();
        
        sessions.retain(|_, session| !self.is_session_expired(session));
        
        let removed = initial_count - sessions.len();
        if removed > 0 {
            debug!(removed = removed, "Cleaned up expired sessions from memory store");
        }
        removed
    }
}

impl TimeoutMemorySessionStore {
    async fn load_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<Option<Session>> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_load_memory".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let sessions = self.sessions.read().unwrap();
                if let Some(session) = sessions.get(session_id) {
                    if self.is_session_expired(session) {
                        debug!(session_id = %session_id, "Session expired during load");
                        None
                    } else {
                        debug!(session_id = %session_id, "Session loaded from memory store");
                        Some(session.clone())
                    }
                } else {
                    debug!(session_id = %session_id, "Session not found in memory store");
                    None
                }
            }
        ).await?
    }

    async fn save_with_timeout(
        &self,
        session: &Session,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_save_memory".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let mut sessions = self.sessions.write().unwrap();
                sessions.insert(session.id.clone(), session.clone());
                debug!(session_id = %session.id, "Session saved to memory store");
            }
        ).await?
    }

    async fn delete_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_delete_memory".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let mut sessions = self.sessions.write().unwrap();
                let removed = sessions.remove(session_id).is_some();
                if removed {
                    debug!(session_id = %session_id, "Session deleted from memory store");
                } else {
                    debug!(session_id = %session_id, "Session not found for deletion in memory store");
                }
            }
        ).await?
    }

    async fn cleanup_with_timeout(
        &self,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<usize> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_cleanup_memory".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                self.cleanup_expired_sessions()
            }
        ).await?
    }

    async fn exists_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<bool> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_exists_memory".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let sessions = self.sessions.read().unwrap();
                if let Some(session) = sessions.get(session_id) {
                    !self.is_session_expired(session)
                } else {
                    false
                }
            }
        ).await?
    }
}

/// File-based session store with timeout support
#[derive(Debug)]
pub struct TimeoutFileSessionStore {
    directory: PathBuf,
    config: SessionConfig,
}

impl TimeoutFileSessionStore {
    pub fn new(directory: PathBuf, config: SessionConfig) -> Result<(), Error> {
        // Ensure directory exists
        if !directory.exists() {
            fs::create_dir_all(&directory)
                .map_err(|e| SessionError::StoreError(format!("Failed to create session directory: {}", e)))?;
        }

        Ok(Self {
            directory,
            config,
        })
    }

    fn session_file_path(&self, session_id: &str) -> PathBuf {
        self.directory.join(format!("{}.session", session_id))
    }

    fn serialize_session(&self, session: &Session) -> Result<(), Error> {
        serde_json::to_vec(session)
            .map_err(|e| SessionError::SerializationError(format!("Failed to serialize session: {}", e)))
    }

    fn deserialize_session(&self, data: &[u8]) -> Result<(), Error> {
        serde_json::from_slice(data)
            .map_err(|e| SessionError::SerializationError(format!("Failed to deserialize session: {}", e)))
    }

    fn is_session_expired(&self, session: &Session) -> bool {
        if let Some(expires_at) = session.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            expires_at <= now
        } else {
            false
        }
    }

    async fn cleanup_expired_files(&self) -> TimeoutResult<usize> {
        let mut removed = 0;
        
        let entries = tokio::fs::read_dir(&self.directory).await
            .map_err(|e| TimeoutError::DatabaseTimeout {
                elapsed: Duration::from_secs(0),
                timeout: self.config.database_timeout,
                operation: format!("read_session_directory: {}", e),
            })?;

        let mut entries = entries;
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| TimeoutError::DatabaseTimeout {
                elapsed: Duration::from_secs(0),
                timeout: self.config.database_timeout,
                operation: format!("read_directory_entry: {}", e),
            })? {
            
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("session") {
                // Try to read and check if expired
                if let Ok(data) = tokio::fs::read(&path).await {
                    if let Ok(session) = self.deserialize_session(&data) {
                        if self.is_session_expired(&session) {
                            if let Err(e) = tokio::fs::remove_file(&path).await {
                                warn!(path = ?path, error = %e, "Failed to remove expired session file");
                            } else {
                                removed += 1;
                                debug!(path = ?path, "Removed expired session file");
                            }
                        }
                    }
                }
            }
        }

        Ok(removed)
    }
}

impl TimeoutFileSessionStore {
    async fn load_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<Option<Session>> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_load_file".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let file_path = self.session_file_path(session_id);
                
                match tokio::fs::read(&file_path).await {
                    Ok(data) => {
                        match self.deserialize_session(&data) {
                            Ok(session) => {
                                if self.is_session_expired(&session) {
                                    // Remove expired session file
                                    let _ = tokio::fs::remove_file(&file_path).await;
                                    debug!(session_id = %session_id, "Session expired and file removed");
                                    None
                                } else {
                                    debug!(session_id = %session_id, "Session loaded from file store");
                                    Some(session)
                                }
                            }
                            Err(e) => {
                                error!(session_id = %session_id, error = %e, "Failed to deserialize session");
                                None
                            }
                        }
                    }
                    Err(_) => {
                        debug!(session_id = %session_id, "Session file not found");
                        None
                    }
                }
            }
        ).await?
    }

    async fn save_with_timeout(
        &self,
        session: &Session,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_save_file".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let file_path = self.session_file_path(&session.id);
                let data = self.serialize_session(session)
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("serialize_session: {}", e),
                    })?;

                tokio::fs::write(&file_path, data).await
                    .map_err(|e| TimeoutError::DatabaseTimeout {
                        elapsed: Duration::from_secs(0),
                        timeout: self.config.database_timeout,
                        operation: format!("write_session_file: {}", e),
                    })?;

                debug!(session_id = %session.id, "Session saved to file store");
                Ok(())
            }
        ).await?
    }

    async fn delete_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_delete_file".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let file_path = self.session_file_path(session_id);
                
                match tokio::fs::remove_file(&file_path).await {
                    Ok(_) => {
                        debug!(session_id = %session_id, "Session file deleted");
                    }
                    Err(_) => {
                        debug!(session_id = %session_id, "Session file not found for deletion");
                    }
                }
            }
        ).await?
    }

    async fn cleanup_with_timeout(
        &self,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<usize> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_cleanup_file".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            self.cleanup_expired_files()
        ).await?
    }

    async fn exists_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<bool> {
        let operation_id = Uuid::new_v4().to_string();
        let operation_type = "session_exists_file".to_string();

        timeout_middleware.with_database_timeout(
            operation_id,
            operation_type,
            async move {
                let file_path = self.session_file_path(session_id);
                
                match tokio::fs::read(&file_path).await {
                    Ok(data) => {
                        match self.deserialize_session(&data) {
                            Ok(session) => !self.is_session_expired(&session),
                            Err(_) => false,
                        }
                    }
                    Err(_) => false,
                }
            }
        ).await?
    }
}

/// Session manager with timeout support
#[derive(Debug)]
pub struct TimeoutSessionManager {
    store: TimeoutAwareSessionStore,
    config: SessionConfig,
}

impl TimeoutSessionManager {
    /// Create session manager with timeout support
    pub async fn new(config: SessionConfig) -> Result<(), Error> {
        let store = match &config.store_type {
            SessionStoreType::Memory => {
                TimeoutAwareSessionStore::Memory(TimeoutMemorySessionStore::new(config.clone()))
            }
            SessionStoreType::File(path) => {
                TimeoutAwareSessionStore::File(TimeoutFileSessionStore::new(path.clone(), config.clone())?)
            }
            SessionStoreType::Redis(redis_url) => {
                let redis_store = TimeoutRedisSessionStore::new(redis_url, config.clone()).await?;
                TimeoutAwareSessionStore::Redis(redis_store)
            }
            SessionStoreType::Database(db_url) => {
                // Create a basic database config from URL
                use crate::stdlib::database::DatabaseConfig;
                
                let db_config = DatabaseConfig {
                    max_open_connections: 10,
                    max_idle_connections: 5,
                    connection_max_lifetime_seconds: 3600,
                    connection_max_idle_seconds: 600,
                    connection_timeout_seconds: 30,
                    query_timeout_seconds: 30,
                    enable_pool_monitoring: false,
                    enable_query_logging: false,
                    max_retry_attempts: 3,
                    retry_delay_ms: 100,
                    enable_prepared_statements: true,
                    enable_transaction_pooling: false,
                    enable_read_replicas: false,
                    schema_validation_enabled: true,
                    migration_auto_run: false,
                };
                
                let db_store = TimeoutDatabaseSessionStore::new(db_config, config.clone()).await?;
                TimeoutAwareSessionStore::Database(db_store)
            }
        };

        Ok(Self {
            store,
            config,
        })
    }

    /// Create new session with timeout tracking
    pub async fn create_session_with_timeout(
        &self,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<Session> {
        let session_id = Uuid::new_v4().to_string();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let session = Session {
            id: session_id.clone(),
            data: HashMap::new(),
            created_at: now,
            last_accessed: now,
            expires_at: Some(now + self.config.max_age.as_secs()),
            is_new: true,
            is_dirty: true,
        };

        // Start session timeout tracking
        timeout_middleware.start_session_timeout(session_id.clone());

        // Save the session
        self.store.save_with_timeout(&session, timeout_middleware).await?;

        debug!(session_id = %session_id, "Created new session with timeout tracking");
        Ok(session)
    }

    /// Load session with timeout
    pub async fn load_session_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<Option<Session>> {
        // Check if session is timed out first
        if timeout_middleware.is_session_timed_out(session_id) {
            debug!(session_id = %session_id, "Session timed out, removing");
            let _ = self.store.delete_with_timeout(session_id, timeout_middleware).await;
            return Ok(None);
        }

        // Load from store
        if let Some(mut session) = self.store.load_with_timeout(session_id, timeout_middleware).await? {
            // Update last accessed time
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            session.last_accessed = now;
            session.is_new = false;
            session.is_dirty = true;

            // Update timeout tracking
            timeout_middleware.update_session_activity(session_id);

            debug!(session_id = %session_id, "Loaded session with timeout update");
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    /// Save session with timeout
    pub async fn save_session_with_timeout(
        &self,
        session: &Session,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        self.store.save_with_timeout(session, timeout_middleware).await?;
        timeout_middleware.update_session_activity(&session.id);
        debug!(session_id = %session.id, "Saved session with timeout update");
        Ok(())
    }

    /// Delete session with timeout
    pub async fn delete_session_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<()> {
        self.store.delete_with_timeout(session_id, timeout_middleware).await?;
        debug!(session_id = %session_id, "Deleted session with timeout tracking");
        Ok(())
    }

    /// Cleanup expired sessions with timeout
    pub async fn cleanup_expired_sessions_with_timeout(
        &self,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<usize> {
        let removed = self.store.cleanup_with_timeout(timeout_middleware).await?;
        if removed > 0 {
            info!(removed = removed, "Cleaned up expired sessions");
        }
        Ok(removed)
    }

    /// Check if session exists with timeout
    pub async fn session_exists_with_timeout(
        &self,
        session_id: &str,
        timeout_middleware: &TimeoutMiddleware,
    ) -> TimeoutResult<bool> {
        self.store.exists_with_timeout(session_id, timeout_middleware).await?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_timeout_memory_session_store() {
        let config = SessionConfig::default();
        let store = TimeoutMemorySessionStore::new(config.clone());
        let timeout_middleware = TimeoutMiddleware::new(
            crate::stdlib::web_vibez::config::ServerConfig::default(),
            config
        );

        let session_id = "test_session";
        
        // Test session doesn't exist
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Create and save session
        let session = Session {
            id: session_id.to_string(),
            data: HashMap::new(),
            created_at: 0,
            last_accessed: 0,
            expires_at: None,
            is_new: true,
            is_dirty: false,
        };

        let result = store.save_with_timeout(&session, &timeout_middleware).await;
        assert!(result.is_ok());

        // Test session exists
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Load session
        let result = store.load_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // Delete session
        let result = store.delete_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());

        // Test session no longer exists
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_timeout_file_session_store() {
        let temp_dir = tempdir().unwrap();
        let config = SessionConfig::default();
        let store = TimeoutFileSessionStore::new(temp_dir.path().to_path_buf(), config.clone()).unwrap();
        let timeout_middleware = TimeoutMiddleware::new(
            crate::stdlib::web_vibez::config::ServerConfig::default(),
            config
        );

        let session_id = "test_file_session";
        
        // Test session doesn't exist
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Create and save session
        let session = Session {
            id: session_id.to_string(),
            data: HashMap::new(),
            created_at: 0,
            last_accessed: 0,
            expires_at: None,
            is_new: true,
            is_dirty: false,
        };

        let result = store.save_with_timeout(&session, &timeout_middleware).await;
        assert!(result.is_ok());

        // Test session exists
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Load session
        let result = store.load_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // Delete session
        let result = store.delete_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());

        // Test session no longer exists
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_timeout_session_manager() {
        let config = SessionConfig::default();
        let manager = TimeoutSessionManager::new(config.clone()).await.unwrap();
        let timeout_middleware = TimeoutMiddleware::new(
            crate::stdlib::web_vibez::config::ServerConfig::default(),
            config
        );

        // Create new session
        let result = manager.create_session_with_timeout(&timeout_middleware).await;
        assert!(result.is_ok());
        let session = result.unwrap();

        // Load session
        let result = manager.load_session_with_timeout(&session.id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // Save session
        let result = manager.save_session_with_timeout(&session, &timeout_middleware).await;
        assert!(result.is_ok());

        // Check if session exists
        let result = manager.session_exists_with_timeout(&session.id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Delete session
        let result = manager.delete_session_with_timeout(&session.id, &timeout_middleware).await;
        assert!(result.is_ok());

        // Check if session no longer exists
        let result = manager.session_exists_with_timeout(&session.id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    #[ignore] // Requires Redis server
    async fn test_timeout_redis_session_store() {
        let redis_url = "redis://localhost:6379/1"; // Use test database
        let config = SessionConfig::default();
        let store = TimeoutRedisSessionStore::new(redis_url, config.clone()).await.unwrap();
        let timeout_middleware = TimeoutMiddleware::new(
            crate::stdlib::web_vibez::config::ServerConfig::default(),
            config
        );

        let session_id = "test_redis_session";
        
        // Test session doesn't exist
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Create and save session
        let session = Session {
            id: session_id.to_string(),
            data: HashMap::new(),
            created_at: 0,
            last_accessed: 0,
            expires_at: None,
            is_new: true,
            is_dirty: false,
        };

        let result = store.save_with_timeout(&session, &timeout_middleware).await;
        assert!(result.is_ok());

        // Test session exists
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Load session
        let result = store.load_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // Delete session
        let result = store.delete_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());

        // Test session no longer exists
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Test cleanup
        let result = store.cleanup_with_timeout(&timeout_middleware).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires database setup
    async fn test_timeout_database_session_store() {
        let db_config = crate::stdlib::database::DatabaseConfig {
            url: "sqlite://test_sessions.db".to_string(),
            max_connections: 5,
            timeout: std::time::Duration::from_secs(30),
            enable_logging: false,
        };
        let config = SessionConfig::default();
        let store = TimeoutDatabaseSessionStore::new(db_config, config.clone()).await.unwrap();
        let timeout_middleware = TimeoutMiddleware::new(
            crate::stdlib::web_vibez::config::ServerConfig::default(),
            config
        );

        let session_id = "test_db_session";
        
        // Test session doesn't exist
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Create and save session
        let session = Session {
            id: session_id.to_string(),
            data: HashMap::new(),
            created_at: 0,
            last_accessed: 0,
            expires_at: None,
            is_new: true,
            is_dirty: false,
        };

        let result = store.save_with_timeout(&session, &timeout_middleware).await;
        assert!(result.is_ok());

        // Test session exists
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Load session
        let result = store.load_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());

        // Delete session
        let result = store.delete_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());

        // Test session no longer exists
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());

        // Test cleanup
        let result = store.cleanup_with_timeout(&timeout_middleware).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_session_expiration_handling() {
        let config = SessionConfig::default();
        let store = TimeoutMemorySessionStore::new(config.clone());
        let timeout_middleware = TimeoutMiddleware::new(
            crate::stdlib::web_vibez::config::ServerConfig::default(),
            config
        );

        let session_id = "test_expired_session";
        
        // Create expired session
        let expired_session = Session {
            id: session_id.to_string(),
            data: HashMap::new(),
            created_at: 0,
            last_accessed: 0,
            expires_at: Some(1), // Already expired
            is_new: true,
            is_dirty: false,
        };

        // Save expired session
        let result = store.save_with_timeout(&expired_session, &timeout_middleware).await;
        assert!(result.is_ok());

        // Try to load expired session - should return None
        let result = store.load_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());

        // Session should not exist
        let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
