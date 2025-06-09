/// Session management and storage utilities
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{SessionConfig, SessionStoreType, SameSitePolicy};

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
    fn load(&self, session_id: &str) -> Result<Option<Session>, SessionError> {
        Ok(self.sessions.get(session_id).cloned())
    }

    fn save(&mut self, session: &Session) -> Result<(), SessionError> {
        self.sessions.insert(session.id.clone(), session.clone());
        Ok(())
    }

    fn delete(&mut self, session_id: &str) -> Result<(), SessionError> {
        self.sessions.remove(session_id);
        Ok(())
    }

    fn cleanup_expired(&mut self) -> Result<usize, SessionError> {
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
    fn load(&self, session_id: &str) -> Result<Option<Session>, SessionError> {
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

    fn save(&mut self, session: &Session) -> Result<(), SessionError> {
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

    fn delete(&mut self, session_id: &str) -> Result<(), SessionError> {
        let file_path = self.session_file_path(session_id);
        let _ = std::fs::remove_file(&file_path); // Ignore errors
        Ok(())
    }

    fn cleanup_expired(&mut self) -> Result<usize, SessionError> {
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

/// Session manager
pub struct SessionManager {
    store: Box<dyn SessionStore>,
    config: SessionConfig,
}

impl SessionManager {
    /// Create new session manager
    pub fn new(config: SessionConfig) -> Self {
        let store: Box<dyn SessionStore> = match &config.store_type {
            SessionStoreType::Memory => Box::new(MemorySessionStore::new()),
            SessionStoreType::File(path) => Box::new(FileSessionStore::new(path.clone())),
            SessionStoreType::Redis(_) => {
                // TODO: Implement Redis store
                Box::new(MemorySessionStore::new())
            }
            SessionStoreType::Database(_) => {
                // TODO: Implement database store
                Box::new(MemorySessionStore::new())
            }
        };

        Self { store, config }
    }

    /// Create new session
    pub fn create_session(&mut self) -> Result<Session, SessionError> {
        let mut session = Session::new();
        session.set_expiry(self.config.max_age.as_secs());
        self.store.save(&session)?;
        Ok(session)
    }

    /// Load existing session
    pub fn load_session(&self, session_id: &str) -> Result<Option<Session>, SessionError> {
        self.store.load(session_id)
    }

    /// Save session
    pub fn save_session(&mut self, session: &Session) -> Result<(), SessionError> {
        if session.is_dirty {
            self.store.save(session)?;
        }
        Ok(())
    }

    /// Delete session
    pub fn delete_session(&mut self, session_id: &str) -> Result<(), SessionError> {
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
    pub fn cleanup_expired_sessions(&mut self) -> Result<usize, SessionError> {
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
}

impl fmt::Display for SessionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SessionError::InvalidData(field) => write!(f, "Invalid session data: {}", field),
            SessionError::StoreError(msg) => write!(f, "Session store error: {}", msg),
            SessionError::SerializationError(msg) => write!(f, "Session serialization error: {}", msg),
            SessionError::NotFound(id) => write!(f, "Session not found: {}", id),
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
        let deserialized = Session::deserialize(&serialized).unwrap();

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
        };

        let mut manager = SessionManager::new(config);
        
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
        };

        let manager = SessionManager::new(config);
        
        let cookie_header = "cursed_session=abc123; other_cookie=value";
        let session_id = manager.parse_session_id_from_cookie(cookie_header);
        assert_eq!(session_id, Some("abc123".to_string()));
    }
}
