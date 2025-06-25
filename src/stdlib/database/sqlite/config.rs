use crate::error::CursedError;
/// fr fr SQLite configuration and connection string parsing that slays periodt
/// 
/// This module handles SQLite-specific configuration options, connection string
/// parsing, and database setup parameters with comprehensive validation.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use super::{SqliteError, SqliteResult, SqliteErrorCode};

/// fr fr SQLite open flags (based on SQLite constants)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteFlags {
    /// Open database read-only
    /// Open database read-write
    /// Create database if it doesn't exist
    /// Delete file on close (test only)
    /// VFS only (not database)
    /// Auto proxy VFS
    /// Use URI filename
    /// Memory database
    /// Main database only
    /// Temporary database
    /// Transient database
    /// Main journal
    /// Temp journal
    /// Sub journal
    /// Super journal
    /// No mutex
    /// Full mutex
    /// Shared cache
    /// Private cache
    /// WAL mode
    /// No follow symlinks
    /// External reader
impl SqliteFlags {
    /// slay Get flag value as integer
    pub fn value(self) -> i32 {
        self as i32
    /// slay Combine multiple flags
    pub fn combine(flags: &[SqliteFlags]) -> i32 {
        flags.iter().map(|f| f.value()).fold(0, |acc, f| acc | f)
    /// slay Get default flags for normal operation
    pub fn default_flags() -> i32 {
        Self::combine(&[SqliteFlags::ReadWrite, SqliteFlags::Create])
    /// slay Get flags for read-only access
    pub fn readonly_flags() -> i32 {
        SqliteFlags::ReadOnly.value()
    /// slay Get flags for memory database
    pub fn memory_flags() -> i32 {
        Self::combine(&[SqliteFlags::ReadWrite, SqliteFlags::Create, SqliteFlags::Memory])
    /// slay Get flags for URI connections
    pub fn uri_flags() -> i32 {
        Self::combine(&[SqliteFlags::ReadWrite, SqliteFlags::Create, SqliteFlags::Uri])
    /// slay Check if flags include specific flag
    pub fn has_flag(flags: i32, flag: SqliteFlags) -> bool {
        (flags & flag.value()) != 0
    }
}

/// fr fr SQLite journal mode options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteJournalMode {
    /// Delete journal file after commit
    /// Keep journal file after commit
    /// Write journal to memory
    /// Write-ahead logging
    /// No journal
    /// Truncate journal file
impl SqliteJournalMode {
    /// slay Get journal mode as string
    pub fn as_str(self) -> &'static str {
        match self {
        }
    }

    /// slay Parse from string
    pub fn from_str(s: &str) -> SqliteResult<Self> {
        match s.to_uppercase().as_str() {
        }
    }

    /// slay Get PRAGMA statement to set this mode
    pub fn pragma_statement(self) -> String {
        format!("PRAGMA journal_mode = {}", self.as_str())
    }
}

impl Default for SqliteJournalMode {
    fn default() -> Self {
        SqliteJournalMode::Delete
    }
}

/// fr fr SQLite synchronous mode options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteSynchronous {
    /// No synchronization
    /// Normal synchronization
    /// Full synchronization
    /// Extra synchronization
impl SqliteSynchronous {
    /// slay Get synchronous mode as string
    pub fn as_str(self) -> &'static str {
        match self {
        }
    }

    /// slay Parse from string
    pub fn from_str(s: &str) -> SqliteResult<Self> {
        match s.to_uppercase().as_str() {
        }
    }

    /// slay Get PRAGMA statement to set this mode
    pub fn pragma_statement(self) -> String {
        format!("PRAGMA synchronous = {}", self.as_str())
    /// slay Get as integer value
    pub fn value(self) -> i32 {
        self as i32
    }
}

impl Default for SqliteSynchronous {
    fn default() -> Self {
        SqliteSynchronous::Full
    }
}

/// fr fr SQLite locking mode options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteLockingMode {
    /// Normal locking (default)
    /// Exclusive locking
impl SqliteLockingMode {
    /// slay Get locking mode as string
    pub fn as_str(self) -> &'static str {
        match self {
        }
    }

    /// slay Parse from string
    pub fn from_str(s: &str) -> SqliteResult<Self> {
        match s.to_uppercase().as_str() {
        }
    }

    /// slay Get PRAGMA statement to set this mode
    pub fn pragma_statement(self) -> String {
        format!("PRAGMA locking_mode = {}", self.as_str())
    }
}

impl Default for SqliteLockingMode {
    fn default() -> Self {
        SqliteLockingMode::Normal
    }
}

/// fr fr SQLite configuration structure
#[derive(Debug, Clone)]
pub struct SqliteConfig {
    /// fr fr Database file path
    /// fr fr Open flags
    /// fr fr Connection timeout
    /// fr fr Command timeout
    /// fr fr Busy timeout (milliseconds)
    /// fr fr Page size (bytes)
    /// fr fr Cache size (pages)
    /// fr fr Memory-mapped I/O size (bytes)
    /// fr fr Journal mode
    /// fr fr Synchronous mode
    /// fr fr Locking mode
    /// fr fr Auto vacuum mode (0=none, 1=full, 2=incremental)
    /// fr fr Foreign key constraints enabled
    /// fr fr Recursive triggers enabled
    /// fr fr Secure delete enabled
    /// fr fr WAL autocheckpoint threshold
    /// fr fr Maximum WAL size
    /// fr fr Connection pool size
    /// fr fr Enable query logging
    /// fr fr Enable performance monitoring
    /// fr fr Custom PRAGMA statements
    /// fr fr Custom collations
    /// fr fr Extensions to load
    /// fr fr User-defined functions
    /// fr fr Additional connection parameters
impl SqliteConfig {
    /// slay Create new configuration with defaults
    pub fn new(database_path: &str) -> Self {
        Self {
            busy_timeout: 30000, // 30 seconds
            cache_size: 2000, // 2000 pages = ~8MB with 4KB pages
            mmap_size: 268435456, // 256MB
            auto_vacuum: 0, // disabled
            wal_autocheckpoint: 1000, // 1000 pages
            max_wal_size: 104857600, // 100MB
        }
    }

    /// slay Create memory database configuration
    pub fn memory() -> Self {
        let mut config = Self::new(":memory:");
        config.open_flags = SqliteFlags::memory_flags();
        config.journal_mode = SqliteJournalMode::Memory;
        config.mmap_size = 0; // No point for memory DB
        config
    /// slay Create read-only configuration
    pub fn readonly(database_path: &str) -> Self {
        let mut config = Self::new(database_path);
        config.open_flags = SqliteFlags::readonly_flags();
        config
    /// slay Create WAL mode configuration (optimized for performance)
    pub fn wal_mode(database_path: &str) -> Self {
        let mut config = Self::new(database_path);
        config.journal_mode = SqliteJournalMode::Wal;
        config.synchronous = SqliteSynchronous::Normal; // WAL allows Normal sync
        config.mmap_size = 1073741824; // 1GB for better performance
        config.cache_size = 10000; // Larger cache for WAL
        config
    /// slay Create high-performance configuration
    pub fn high_performance(database_path: &str) -> Self {
        let mut config = Self::wal_mode(database_path);
        config.synchronous = SqliteSynchronous::Normal;
        config.mmap_size = 2147483648; // 2GB
        config.cache_size = 20000; // 20k pages = ~80MB
        config.page_size = 8192; // Larger pages
        config.wal_autocheckpoint = 10000; // Less frequent checkpoints
        config.custom_pragmas.push("PRAGMA temp_store = memory".to_string());
        config.custom_pragmas.push("PRAGMA optimize".to_string());
        config
    /// slay Create safe configuration (maximum data integrity)
    pub fn safe_mode(database_path: &str) -> Self {
        let mut config = Self::new(database_path);
        config.synchronous = SqliteSynchronous::Extra;
        config.secure_delete = true;
        config.foreign_keys = true;
        config.auto_vacuum = 1; // Full auto vacuum
        config.custom_pragmas.push("PRAGMA integrity_check".to_string());
        config
    /// slay Validate configuration
    pub fn validate(&self) -> SqliteResult<()> {
        // Validate database path
        if self.database_path.is_empty() {
            return Err(SqliteError::invalid_parameter("Database path cannot be empty"));
        // Validate page size (must be power of 2, 512-65536)
        let is_power_of_two = self.page_size > 0 && (self.page_size & (self.page_size - 1)) == 0;
        if !is_power_of_two || self.page_size < 512 || self.page_size > 65536 {
            return Err(SqliteError::invalid_parameter(
                "Page size must be power of 2 between 512 and 65536"
            ));
        // Validate cache size (positive)
        if self.cache_size <= 0 {
            return Err(SqliteError::invalid_parameter("Cache size must be positive"));
        // Validate busy timeout (non-negative)
        if self.busy_timeout < 0 {
            return Err(SqliteError::invalid_parameter("Busy timeout cannot be negative"));
        // Validate auto vacuum mode
        if !(0..=2).contains(&self.auto_vacuum) {
            return Err(SqliteError::invalid_parameter("Auto vacuum mode must be 0, 1, or 2"));
        // Validate WAL autocheckpoint
        if self.wal_autocheckpoint < 0 {
            return Err(SqliteError::invalid_parameter("WAL autocheckpoint cannot be negative"));
        // Validate pool size
        if self.pool_size == 0 {
            return Err(SqliteError::invalid_parameter("Pool size must be at least 1"));
        Ok(())
    /// slay Generate initialization SQL statements
    pub fn initialization_sql(&self) -> Vec<String> {
        let mut statements = Vec::new();

        // Basic PRAGMA statements
        statements.push(format!("PRAGMA page_size = {}", self.page_size));
        statements.push(format!("PRAGMA cache_size = {}", self.cache_size));
        statements.push(format!("PRAGMA busy_timeout = {}", self.busy_timeout));
        statements.push(self.journal_mode.pragma_statement());
        statements.push(self.synchronous.pragma_statement());
        statements.push(self.locking_mode.pragma_statement());
        statements.push(format!("PRAGMA auto_vacuum = {}", self.auto_vacuum));
        statements.push(format!("PRAGMA foreign_keys = {}", if self.foreign_keys { "ON" } else { "OFF" }));
        statements.push(format!("PRAGMA recursive_triggers = {}", if self.recursive_triggers { "ON" } else { "OFF" }));
        statements.push(format!("PRAGMA secure_delete = {}", if self.secure_delete { "ON" } else { "OFF" }));

        // Memory-mapped I/O
        if self.mmap_size > 0 {
            statements.push(format!("PRAGMA mmap_size = {}", self.mmap_size));
        // WAL settings
        if self.journal_mode == SqliteJournalMode::Wal {
            statements.push(format!("PRAGMA wal_autocheckpoint = {}", self.wal_autocheckpoint));
        // Custom PRAGMA statements
        statements.extend(self.custom_pragmas.clone());

        statements
    /// slay Add custom PRAGMA statement
    pub fn add_pragma(&mut self, pragma: &str) {
        self.custom_pragmas.push(pragma.to_string());
    /// slay Add custom collation
    pub fn add_collation(&mut self, name: &str, definition: &str) {
        self.custom_collations.insert(name.to_string(), definition.to_string());
    /// slay Add extension
    pub fn add_extension(&mut self, extension: &str) {
        self.extensions.push(extension.to_string());
    /// slay Add custom function
    pub fn add_function(&mut self, function: &str) {
        self.custom_functions.push(function.to_string());
    /// slay Add additional parameter
    pub fn add_parameter(&mut self, key: &str, value: &str) {
        self.additional_params.insert(key.to_string(), value.to_string());
    /// slay Check if database is in-memory
    pub fn is_memory_database(&self) -> bool {
        self.database_path == ":memory:" || 
        SqliteFlags::has_flag(self.open_flags, SqliteFlags::Memory)
    /// slay Check if database is read-only
    pub fn is_readonly(&self) -> bool {
        SqliteFlags::has_flag(self.open_flags, SqliteFlags::ReadOnly)
    /// slay Check if WAL mode is enabled
    pub fn is_wal_mode(&self) -> bool {
        self.journal_mode == SqliteJournalMode::Wal
    /// slay Get estimated memory usage
    pub fn estimated_memory_usage(&self) -> u64 {
        let cache_memory = (self.cache_size as u64) * (self.page_size as u64);
        let mmap_memory = if self.mmap_size > 0 { self.mmap_size as u64 } else { 0 };
        let overhead = 1024 * 1024; // 1MB overhead estimate
        
        cache_memory + mmap_memory + overhead
    }
}

impl Default for SqliteConfig {
    fn default() -> Self {
        Self::new("database.db")
    }
}

/// fr fr SQLite connection string parser
#[derive(Debug, Clone)]
pub struct SqliteConnectionString {
    /// fr fr Parsed configuration
    /// fr fr Original connection string
impl SqliteConnectionString {
    /// slay Parse connection string
    pub fn parse(connection_string: &str) -> SqliteResult<Self> {
        let original = connection_string.to_string();
        let mut config = SqliteConfig::default();

        // Handle common formats:
        // 1. Simple file path: "database.db"
        // 2. SQLite URI: "file:database.db?param=value"
        // 3. Memory database: ":memory:"
        // 4. Data source: "Data Source=database.db;Version=3;..."

        if connection_string == ":memory:" {
            config = SqliteConfig::memory();
        } else if connection_string.starts_with("file:") {
            Self::parse_uri_format(&mut config, connection_string)?;
        } else if connection_string.contains('=') {
            Self::parse_data_source_format(&mut config, connection_string)?;
        } else {
            // Simple file path
            config.database_path = connection_string.to_string();
        config.validate()?;

        Ok(Self { config, original })
    /// slay Parse URI format: file:path?param=value&param2=value2
    fn parse_uri_format(config: &mut SqliteConfig, uri: &str) -> SqliteResult<()> {
        let uri_without_scheme = uri.strip_prefix("file:")
            .ok_or_else(|| SqliteError::invalid_parameter("Invalid URI format"))?;

        let (path, params) = if let Some(question_mark_pos) = uri_without_scheme.find('?') {
            let path = &uri_without_scheme[..question_mark_pos];
            let params = &uri_without_scheme[question_mark_pos + 1..];
            (path, Some(params))
        } else {
            (uri_without_scheme, None)

        config.database_path = path.to_string();
        config.open_flags |= SqliteFlags::Uri.value();

        if let Some(params_str) = params {
            Self::parse_uri_parameters(config, params_str)?;
        Ok(())
    /// slay Parse URI parameters
    fn parse_uri_parameters(config: &mut SqliteConfig, params: &str) -> SqliteResult<()> {
        for param in params.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                let key = urlencoding::decode(key)
                    .map_err(|_| SqliteError::invalid_parameter("Invalid parameter key encoding"))?;
                let value = urlencoding::decode(value)
                    .map_err(|_| SqliteError::invalid_parameter("Invalid parameter value encoding"))?;

                Self::apply_parameter(config, &key, &value)?;
            }
        }
        Ok(())
    /// slay Parse data source format: key=value;key2=value2
    fn parse_data_source_format(config: &mut SqliteConfig, data_source: &str) -> SqliteResult<()> {
        for pair in data_source.split(';') {
            if pair.trim().is_empty() {
                continue;
            if let Some((key, value)) = pair.split_once('=') {
                let key = key.trim().to_lowercase();
                let value = value.trim();

                match key.as_str() {
                    "data source" | "datasource" | "database" | "db" => {
                        config.database_path = value.to_string();
                    }
                    _ => {
                        Self::apply_parameter(config, &key, value)?;
                    }
                }
            }
        }
        Ok(())
    /// slay Apply parameter to configuration
    fn apply_parameter(config: &mut SqliteConfig, key: &str, value: &str) -> SqliteResult<()> {
        match key.to_lowercase().as_str() {
            "mode" => {
                match value.to_lowercase().as_str() {
                    "ro" | "readonly" => {
                        config.open_flags = SqliteFlags::readonly_flags();
                    }
                    "rw" | "readwrite" => {
                        config.open_flags = SqliteFlags::default_flags();
                    }
                    "rwc" | "readwritecreate" => {
                        config.open_flags = SqliteFlags::default_flags();
                    }
                    "memory" => {
                        config.open_flags = SqliteFlags::memory_flags();
                    }
                }
            }
            "cache" => {
                match value.to_lowercase().as_str() {
                    "shared" => {
                        config.open_flags |= SqliteFlags::SharedCache.value();
                    }
                    "private" => {
                        config.open_flags |= SqliteFlags::PrivateCache.value();
                    }
                }
            }
            "page_size" => {
                config.page_size = value.parse()
                    .map_err(|_| SqliteError::invalid_parameter("Invalid page size"))?;
            }
            "cache_size" => {
                config.cache_size = value.parse()
                    .map_err(|_| SqliteError::invalid_parameter("Invalid cache size"))?;
            }
            "busy_timeout" => {
                config.busy_timeout = value.parse()
                    .map_err(|_| SqliteError::invalid_parameter("Invalid busy timeout"))?;
            }
            "journal_mode" => {
                config.journal_mode = SqliteJournalMode::from_str(value)?;
            }
            "synchronous" => {
                config.synchronous = SqliteSynchronous::from_str(value)?;
            }
            "locking_mode" => {
                config.locking_mode = SqliteLockingMode::from_str(value)?;
            }
            "foreign_keys" => {
                config.foreign_keys = Self::parse_bool(value)?;
            }
            "recursive_triggers" => {
                config.recursive_triggers = Self::parse_bool(value)?;
            }
            "secure_delete" => {
                config.secure_delete = Self::parse_bool(value)?;
            }
            "mmap_size" => {
                config.mmap_size = value.parse()
                    .map_err(|_| SqliteError::invalid_parameter("Invalid mmap size"))?;
            }
            "auto_vacuum" => {
                config.auto_vacuum = value.parse()
                    .map_err(|_| SqliteError::invalid_parameter("Invalid auto vacuum mode"))?;
            }
            "wal_autocheckpoint" => {
                config.wal_autocheckpoint = value.parse()
                    .map_err(|_| SqliteError::invalid_parameter("Invalid WAL autocheckpoint"))?;
            }
            "pool_size" => {
                config.pool_size = value.parse()
                    .map_err(|_| SqliteError::invalid_parameter("Invalid pool size"))?;
            }
            "enable_logging" => {
                config.enable_logging = Self::parse_bool(value)?;
            }
            "enable_monitoring" => {
                config.enable_monitoring = Self::parse_bool(value)?;
            }
            _ => {
                // Store unknown parameters for potential use by extensions
                config.add_parameter(key, value);
            }
        }
        Ok(())
    /// slay Parse boolean value
    fn parse_bool(value: &str) -> SqliteResult<bool> {
        match value.to_lowercase().as_str() {
        }
    }

    /// slay Build connection string from configuration
    pub fn build_connection_string(config: &SqliteConfig) -> String {
        if config.is_memory_database() {
            return ":memory:".to_string();
        let mut uri = format!("file:{}", config.database_path);
        let mut params = Vec::new();

        // Add mode parameter
        if config.is_readonly() {
            params.push("mode=ro".to_string());
        } else {
            params.push("mode=rwc".to_string());
        // Add cache mode
        if SqliteFlags::has_flag(config.open_flags, SqliteFlags::SharedCache) {
            params.push("cache=shared".to_string());
        } else if SqliteFlags::has_flag(config.open_flags, SqliteFlags::PrivateCache) {
            params.push("cache=private".to_string());
        // Add other parameters
        if config.page_size != 4096 {
            params.push(format!("page_size={}", config.page_size));
        }
        if config.cache_size != 2000 {
            params.push(format!("cache_size={}", config.cache_size));
        }
        if config.busy_timeout != 30000 {
            params.push(format!("busy_timeout={}", config.busy_timeout));
        }
        if config.journal_mode != SqliteJournalMode::Delete {
            params.push(format!("journal_mode={}", config.journal_mode.as_str()));
        }
        if config.synchronous != SqliteSynchronous::Full {
            params.push(format!("synchronous={}", config.synchronous.as_str()));
        // Add additional parameters
        for (key, value) in &config.additional_params {
            params.push(format!("{}={}", key, value));
        if !params.is_empty() {
            uri.push('?');
            uri.push_str(&params.join("&"));
        uri
    }
}

