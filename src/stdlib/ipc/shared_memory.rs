/// Basic shared memory implementation for CURSED IPC
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::stdlib::ipc::error::{IpcError, IpcResult};

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};

/// Configuration for shared memory creation
#[derive(Debug, Clone)]
pub struct MemoryConfig {
    pub name: String,
    pub size: usize,
    pub access: MemoryAccess,
    pub permissions: u32,
    pub create_if_not_exists: bool,
}

impl MemoryConfig {
    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
            access: MemoryAccess::ReadWrite,
            permissions: 0o600,
            create_if_not_exists: true,
        }
    }

    pub fn with_access(mut self, access: MemoryAccess) -> Self {
        self.access = access;
        self
    }

    pub fn with_permissions(mut self, permissions: u32) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn read_only(mut self) -> Self {
        self.access = MemoryAccess::ReadOnly;
        self
    }

    pub fn write_only(mut self) -> Self {
        self.access = MemoryAccess::WriteOnly;
        self
    }
}

/// Memory access mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

/// View into shared memory
#[derive(Debug)]
pub struct MemoryView {
    offset: usize,
    size: usize,
    data: Vec<u8>,
}

impl MemoryView {
    pub fn new(offset: usize, size: usize, data: Vec<u8>) -> Self {
        Self { offset, size, data }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }
}

/// Shared memory region
pub struct SharedMemory {
    config: MemoryConfig,
    file: File,
    path: PathBuf,
    #[cfg(unix)]
    fd: RawFd,
}

impl SharedMemory {
    /// Create or open shared memory region
    pub fn create(name: &str, size: usize) -> IpcResult<Self> {
        let config = MemoryConfig::new(name, size);
        Self::create_with_config(config)
    }

    /// Create shared memory with custom configuration
    pub fn create_with_config(config: MemoryConfig) -> IpcResult<Self> {
        // On Unix-like systems, use /dev/shm or /tmp for shared memory
        #[cfg(unix)]
        let base_path = if Path::new("/dev/shm").exists() {
            PathBuf::from("/dev/shm")
        } else {
            PathBuf::from("/tmp")
        };

        #[cfg(not(unix))]
        let base_path = std::env::temp_dir();

        let path = base_path.join(format!("cursed_shm_{}", config.name));

        let mut options = OpenOptions::new();
        match config.access {
            MemoryAccess::ReadOnly => {
                options.read(true);
            }
            MemoryAccess::WriteOnly => {
                options.write(true);
            }
            MemoryAccess::ReadWrite => {
                options.read(true).write(true);
            }
        }

        if config.create_if_not_exists {
            options.create(true);
        }

        #[cfg(unix)]
        options.mode(config.permissions);

        let mut file = options.open(&path).map_err(IpcError::from)?;

        // Ensure the file is the correct size
        if config.create_if_not_exists {
            let metadata = file.metadata().map_err(IpcError::from)?;
            if metadata.len() < config.size as u64 {
                file.set_len(config.size as u64).map_err(IpcError::from)?;
            }
        }

        #[cfg(unix)]
        let fd = file.as_raw_fd();

        let shared_memory = Self {
            config: config.clone(),
            file,
            path: path.clone(),
            #[cfg(unix)]
            fd,
        };

        // Register with IPC registry
        crate::stdlib::ipc::register_shared_memory(
            config.name.clone(),
            path.to_string_lossy().to_string(),
        )?;

        Ok(shared_memory)
    }

    /// Open existing shared memory region
    pub fn open(name: &str) -> IpcResult<Self> {
        let config = MemoryConfig {
            name: name.to_string(),
            size: 0, // Will be determined from existing file
            access: MemoryAccess::ReadWrite,
            permissions: 0o600,
            create_if_not_exists: false,
        };

        #[cfg(unix)]
        let base_path = if Path::new("/dev/shm").exists() {
            PathBuf::from("/dev/shm")
        } else {
            PathBuf::from("/tmp")
        };

        #[cfg(not(unix))]
        let base_path = std::env::temp_dir();

        let path = base_path.join(format!("cursed_shm_{}", name));

        if !path.exists() {
            return Err(IpcError::NotFound(format!("Shared memory '{}' not found", name)));
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .map_err(IpcError::from)?;

        #[cfg(unix)]
        let fd = file.as_raw_fd();

        Ok(Self {
            config,
            file,
            path,
            #[cfg(unix)]
            fd,
        })
    }

    /// Write data at specified offset
    pub fn write_at(&mut self, offset: usize, data: &[u8]) -> IpcResult<usize> {
        if !self.can_write() {
            return Err(IpcError::InvalidOperation("Shared memory not open for writing".to_string()));
        }

        crate::stdlib::ipc::increment_operations();

        self.file.seek(SeekFrom::Start(offset as u64))
            .map_err(|e| {
                crate::stdlib::ipc::increment_failed_operations();
                IpcError::from(e)
            })?;

        self.file.write(data).map_err(|e| {
            crate::stdlib::ipc::increment_failed_operations();
            IpcError::from(e)
        })
    }

    /// Read data from specified offset
    pub fn read_at(&mut self, offset: usize, size: usize) -> IpcResult<Vec<u8>> {
        if !self.can_read() {
            return Err(IpcError::InvalidOperation("Shared memory not open for reading".to_string()));
        }

        crate::stdlib::ipc::increment_operations();

        self.file.seek(SeekFrom::Start(offset as u64))
            .map_err(|e| {
                crate::stdlib::ipc::increment_failed_operations();
                IpcError::from(e)
            })?;

        let mut buffer = vec![0u8; size];
        match self.file.read(&mut buffer) {
            Ok(bytes_read) => {
                buffer.truncate(bytes_read);
                Ok(buffer)
            }
            Err(e) => {
                crate::stdlib::ipc::increment_failed_operations();
                Err(IpcError::from(e))
            }
        }
    }

    /// Write string at specified offset
    pub fn write_string_at(&mut self, offset: usize, s: &str) -> IpcResult<usize> {
        self.write_at(offset, s.as_bytes())
    }

    /// Read string from specified offset
    pub fn read_string_at(&mut self, offset: usize, size: usize) -> IpcResult<String> {
        let data = self.read_at(offset, size)?;
        // Find null terminator or use all data
        let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
        String::from_utf8(data[..end].to_vec())
            .map_err(|e| IpcError::InvalidInput(format!("Invalid UTF-8: {}", e)))
    }

    /// Get a view of the shared memory
    pub fn view(&mut self, offset: usize, size: usize) -> IpcResult<MemoryView> {
        let data = self.read_at(offset, size)?;
        Ok(MemoryView::new(offset, size, data))
    }

    /// Get the full contents as a view
    pub fn full_view(&mut self) -> IpcResult<MemoryView> {
        let size = self.size()?;
        self.view(0, size)
    }

    /// Flush any pending writes
    pub fn flush(&mut self) -> IpcResult<()> {
        self.file.flush().map_err(IpcError::from)
    }

    /// Get the size of the shared memory region
    pub fn size(&self) -> IpcResult<usize> {
        let metadata = self.file.metadata().map_err(IpcError::from)?;
        Ok(metadata.len() as usize)
    }

    /// Get the name of the shared memory region
    pub fn name(&self) -> &str {
        &self.config.name
    }

    /// Get the file path
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Check if memory is readable
    pub fn can_read(&self) -> bool {
        matches!(self.config.access, MemoryAccess::ReadOnly | MemoryAccess::ReadWrite)
    }

    /// Check if memory is writable
    pub fn can_write(&self) -> bool {
        matches!(self.config.access, MemoryAccess::WriteOnly | MemoryAccess::ReadWrite)
    }

    /// Resize the shared memory region
    pub fn resize(&mut self, new_size: usize) -> IpcResult<()> {
        self.file.set_len(new_size as u64).map_err(IpcError::from)?;
        self.config.size = new_size;
        Ok(())
    }
}

impl Drop for SharedMemory {
    fn drop(&mut self) {
        let _ = self.flush();
        let _ = crate::stdlib::ipc::unregister_shared_memory(&self.config.name);
    }
}

/// Create shared memory region
pub fn create_shared_memory(name: &str, size: usize) -> IpcResult<SharedMemory> {
    SharedMemory::create(name, size)
}

/// Open existing shared memory region
pub fn open_shared_memory(name: &str) -> IpcResult<SharedMemory> {
    SharedMemory::open(name)
}

/// Remove shared memory region
pub fn remove_shared_memory(name: &str) -> IpcResult<()> {
    #[cfg(unix)]
    let base_path = if Path::new("/dev/shm").exists() {
        PathBuf::from("/dev/shm")
    } else {
        PathBuf::from("/tmp")
    };

    #[cfg(not(unix))]
    let base_path = std::env::temp_dir();

    let path = base_path.join(format!("cursed_shm_{}", name));

    if path.exists() {
        std::fs::remove_file(path).map_err(IpcError::from)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_config() {
        let config = MemoryConfig::new("test", 1024)
            .with_access(MemoryAccess::ReadOnly)
            .with_permissions(0o644);

        assert_eq!(config.name, "test");
        assert_eq!(config.size, 1024);
        assert_eq!(config.access, MemoryAccess::ReadOnly);
        assert_eq!(config.permissions, 0o644);
    }

    #[test]
    fn test_memory_view() {
        let data = vec![1, 2, 3, 4, 5];
        let view = MemoryView::new(10, 5, data.clone());

        assert_eq!(view.offset(), 10);
        assert_eq!(view.size(), 5);
        assert_eq!(view.data(), &data);
        assert_eq!(view.as_bytes(), &data);
    }

    #[test]
    fn test_shared_memory_creation() {
        let shm = SharedMemory::create("test_memory", 1024);
        assert!(shm.is_ok());

        let shm = shm.unwrap();
        assert_eq!(shm.name(), "test_memory");
        assert!(shm.can_read());
        assert!(shm.can_write());

        // Cleanup
        let _ = remove_shared_memory("test_memory");
    }

    #[test]
    fn test_shared_memory_read_write() {
        let mut shm = SharedMemory::create("test_rw", 1024).unwrap();

        // Write data
        let data = b"Hello, shared memory!";
        let written = shm.write_at(0, data).unwrap();
        assert_eq!(written, data.len());

        // Read data back
        let read_data = shm.read_at(0, data.len()).unwrap();
        assert_eq!(read_data, data);

        // Test string operations
        let text = "Hello, world!";
        shm.write_string_at(100, text).unwrap();
        let read_text = shm.read_string_at(100, text.len()).unwrap();
        assert_eq!(read_text, text);

        // Cleanup
        let _ = remove_shared_memory("test_rw");
    }

    #[test]
    fn test_shared_memory_view() {
        let mut shm = SharedMemory::create("test_view", 1024).unwrap();

        // Write some data
        let data = b"Hello, view!";
        shm.write_at(50, data).unwrap();

        // Get a view
        let view = shm.view(50, data.len()).unwrap();
        assert_eq!(view.offset(), 50);
        assert_eq!(view.size(), data.len());
        assert_eq!(view.data(), data);

        // Cleanup
        let _ = remove_shared_memory("test_view");
    }

    #[test]
    fn test_error_handling() {
        // Test opening non-existent shared memory
        let result = SharedMemory::open("non_existent");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), IpcError::NotFound(_)));
    }

    #[test]
    fn test_access_modes() {
        let config = MemoryConfig::new("test_access", 1024).read_only();
        let shm = SharedMemory::create_with_config(config).unwrap();

        assert!(shm.can_read());
        assert!(!shm.can_write());

        // Cleanup
        let _ = remove_shared_memory("test_access");
    }
}
