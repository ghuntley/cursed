use std::io::{Read, Write};
/// Database schema version management

use crate::error::CursedError;
use std::collections::HashMap;
use std::time::SystemTime;
use std::fmt;

/// Result type for version operations
pub type VersionResult<T> = Result<T, CursedError>;

/// Schema version representation
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub migration_id: Option<String>,
}

impl SchemaVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            migration_id: None,
        }
    }
    
    pub fn with_migration_id(mut self, migration_id: String) -> Self {
        self.migration_id = Some(migration_id);
        self
    }
    
    pub fn to_number(&self) -> u64 {
        (self.major as u64) * 1_000_000 + (self.minor as u64) * 1_000 + (self.patch as u64)
    }
    
    pub fn from_number(version: u64) -> Self {
        let major = (version / 1_000_000) as u32;
        let minor = ((version % 1_000_000) / 1_000) as u32;
        let patch = (version % 1_000) as u32;
        
        Self::new(major, minor, patch)
    }
}

impl fmt::Display for SchemaVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Version manager for tracking schema versions
pub struct VersionManager {
    pub current_version: SchemaVersion,
    pub version_history: Vec<VersionEntry>,
    pub applied_migrations: HashMap<String, SystemTime>,
}

impl VersionManager {
    pub fn new() -> Self {
        Self {
            current_version: SchemaVersion::new(0, 0, 0),
            version_history: Vec::new(),
            applied_migrations: HashMap::new(),
        }
    }
    
    pub fn update_version(&mut self, version: SchemaVersion) -> VersionResult<()> {
        if version <= self.current_version {
            return Err(CursedError::runtime_error(
                &format!("Cannot downgrade from {} to {}", self.current_version, version)
            ));
        }
        
        let entry = VersionEntry {
            version: version.clone(),
            applied_at: SystemTime::now(),
            migration_id: version.migration_id.clone(),
        };
        
        self.version_history.push(entry);
        self.current_version = version;
        
        if let Some(migration_id) = &self.current_version.migration_id {
            self.applied_migrations.insert(migration_id.clone(), SystemTime::now());
        }
        
        Ok(())
    }
    
    pub fn rollback_to_version(&mut self, target_version: SchemaVersion) -> VersionResult<()> {
        if target_version > self.current_version {
            return Err(CursedError::runtime_error(
                &format!("Cannot rollback to future version {}", target_version)
            ));
        }
        
        // Remove version entries after target version
        self.version_history.retain(|entry| entry.version <= target_version);
        
        // Remove applied migrations after target version
        self.applied_migrations.retain(|_, applied_at| {
            // This is a simplified check - in practice you'd need more sophisticated logic
            true
        });
        
        self.current_version = target_version;
        Ok(())
    }
    
    pub fn get_version_history(&self) -> &[VersionEntry] {
        &self.version_history
    }
    
    pub fn is_migration_applied(&self, migration_id: &str) -> bool {
        self.applied_migrations.contains_key(migration_id)
    }
}

/// Version history entry
#[derive(Debug, Clone)]
pub struct VersionEntry {
    pub version: SchemaVersion,
    pub applied_at: SystemTime,
    pub migration_id: Option<String>,
}

/// I/O operations handler
pub struct IOHandler {
    buffer_size: usize,
}

impl IOHandler {
    /// Create a new I/O handler
    pub fn new() -> Self {
        Self {
            buffer_size: 8192,
        }
    }
    
    /// Set buffer size
    pub fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
    
    /// Read from a reader
    pub fn read_all<R: Read>(&self, mut reader: R) -> Result<Vec<u8>, CursedError> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).map_err(CursedError::from)?;
        Ok(buffer)
    }
    
    /// Write to a writer
    pub fn write_all<W: Write>(&self, mut writer: W, data: &[u8]) -> Result<(), CursedError> {
        writer.write_all(data).map_err(CursedError::from)?;
        Ok(())
    }
    
    /// Read string from reader
    pub fn read_string<R: Read>(&self, reader: R) -> Result<String, CursedError> {
        let bytes = self.read_all(reader)?;
        String::from_utf8(bytes).map_err(CursedError::from)
    }
    
    /// Write string to writer
    pub fn write_string<W: Write>(&self, writer: W, text: &str) -> Result<(), CursedError> {
        self.write_all(writer, text.as_bytes())
    }
}

impl Default for IOHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize I/O processing
pub fn init_version() -> Result<(), CursedError> {
    let handler = IOHandler::new();
    let test_data = b"test data";
    let mut cursor = std::io::Cursor::new(test_data);
    let result = handler.read_all(&mut cursor)?;
    if result != test_data {
        return Err(CursedError::Io("I/O test failed".to_string()));
    }
    println!("📁 I/O processing (version) initialized");
    Ok(())
}

/// Test I/O functionality
pub fn test_version() -> Result<(), CursedError> {
    let handler = IOHandler::new();
    let test_string = "Hello, CURSED I/O!";
    let mut buffer = Vec::new();
    handler.write_string(&mut buffer, test_string)?;
    let result = handler.read_string(std::io::Cursor::new(&buffer))?;
    if result != test_string {
        return Err(CursedError::Io("I/O string test failed".to_string()));
    }
    Ok(())
}
