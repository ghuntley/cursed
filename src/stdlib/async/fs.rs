use crate::error::Error;
/// Async file system operations for CURSED stdlib
use std::path::Path;
use crate::stdlib::r#async::{AsyncError, AsyncResult, spawn_blocking_io};
use crate::runtime::r#async::Promise;

/// Async file handle
pub struct AsyncFile {
    path: std::path::PathBuf,
}

impl AsyncFile {
    /// Open a file for reading
    pub async fn open<P: AsRef<Path>>(path: P) -> AsyncResult<Self> {
        let path = path.as_ref().to_path_buf();
        spawn_blocking_io(move || {
            if path.exists() {
                Ok(AsyncFile { path })
            } else {
                Err(AsyncError::Io("File not found".to_string()))
            }
        }).await
    }

    /// Create a new file
    pub async fn create<P: AsRef<Path>>(path: P) -> AsyncResult<Self> {
        let path = path.as_ref().to_path_buf();
        spawn_blocking_io(move || {
            Ok(AsyncFile { path })
        }).await
    }

    /// Read entire file contents
    pub async fn read_to_string(&self) -> AsyncResult<String> {
        let path = self.path.clone();
        spawn_blocking_io(move || {
            std::fs::read_to_string(path).map_err(AsyncError::from)
        }).await
    }

    /// Write string to file
    pub async fn write_all(&self, contents: &str) -> AsyncResult<()> {
        let path = self.path.clone();
        let contents = contents.to_string();
        spawn_blocking_io(move || {
            std::fs::write(path, contents).map_err(AsyncError::from)
        }).await
    }

    /// Get file metadata
    pub async fn metadata(&self) -> AsyncResult<std::fs::Metadata> {
        let path = self.path.clone();
        spawn_blocking_io(move || {
            std::fs::metadata(path).map_err(AsyncError::from)
        }).await
    }
}

// Convenience functions
pub async fn open_async<P: AsRef<Path>>(path: P) -> AsyncResult<AsyncFile> {
    AsyncFile::open(path).await
}

pub async fn create_async<P: AsRef<Path>>(path: P) -> AsyncResult<AsyncFile> {
    AsyncFile::create(path).await
}

pub async fn read_async<P: AsRef<Path>>(path: P) -> AsyncResult<String> {
    let path = path.as_ref().to_path_buf();
    spawn_blocking_io(move || {
        std::fs::read_to_string(path).map_err(AsyncError::from)
    }).await
}

pub async fn write_async<P: AsRef<Path>>(path: P, contents: &str) -> AsyncResult<()> {
    let path = path.as_ref().to_path_buf();
    let contents = contents.to_string();
    spawn_blocking_io(move || {
        std::fs::write(path, contents).map_err(AsyncError::from)
    }).await
}

pub async fn append_async<P: AsRef<Path>>(path: P, contents: &str) -> AsyncResult<()> {
    let path = path.as_ref().to_path_buf();
    let contents = contents.to_string();
    spawn_blocking_io(move || {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(contents.as_bytes())?;
        Ok(())
    }).await
}

pub async fn copy_async<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> AsyncResult<u64> {
    let from = from.as_ref().to_path_buf();
    let to = to.as_ref().to_path_buf();
    spawn_blocking_io(move || {
        std::fs::copy(from, to).map_err(AsyncError::from)
    }).await
}

pub async fn remove_async<P: AsRef<Path>>(path: P) -> AsyncResult<()> {
    let path = path.as_ref().to_path_buf();
    spawn_blocking_io(move || {
        if path.is_dir() {
            std::fs::remove_dir_all(path).map_err(AsyncError::from)
        } else {
            std::fs::remove_file(path).map_err(AsyncError::from)
        }
    }).await
}

pub async fn metadata_async<P: AsRef<Path>>(path: P) -> AsyncResult<std::fs::Metadata> {
    let path = path.as_ref().to_path_buf();
    spawn_blocking_io(move || {
        std::fs::metadata(path).map_err(AsyncError::from)
    }).await
}

pub async fn read_dir_async<P: AsRef<Path>>(path: P) -> AsyncResult<Vec<std::path::PathBuf>> {
    let path = path.as_ref().to_path_buf();
    spawn_blocking_io(move || {
        let mut entries = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            entries.push(entry.path());
        }
        Ok(entries)
    }).await
}
