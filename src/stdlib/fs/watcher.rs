/// File System Watcher Module for CURSED
/// 
/// Provides comprehensive file system monitoring capabilities including
/// file and directory watching, event handling, recursive monitoring,
/// and cross-platform file system event notifications.

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::fs;
use super::{FsError, FsResult};

// =============================================================================
// FILE SYSTEM EVENT TYPES
// =============================================================================

/// File system event types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WatchEvent {
    /// File or directory was created
    Created {
        path: PathBuf,
        is_dir: bool,
    },
    /// File or directory was modified
    Modified {
        path: PathBuf,
        is_dir: bool,
    },
    /// File or directory was deleted
    Deleted {
        path: PathBuf,
        was_dir: bool,
    },
    /// File or directory was renamed/moved
    Renamed {
        from: PathBuf,
        to: PathBuf,
        is_dir: bool,
    },
    /// Access permissions changed
    PermissionsChanged {
        path: PathBuf,
        is_dir: bool,
    },
    /// Attributes changed (size, timestamps, etc.)
    AttributesChanged {
        path: PathBuf,
        is_dir: bool,
    },
}

impl WatchEvent {
    /// Gets the primary path associated with this event
    pub fn path(&self) -> &Path {
        match self {
            WatchEvent::Created { path, .. } => path,
            WatchEvent::Modified { path, .. } => path,
            WatchEvent::Deleted { path, .. } => path,
            WatchEvent::Renamed { to, .. } => to,
            WatchEvent::PermissionsChanged { path, .. } => path,
            WatchEvent::AttributesChanged { path, .. } => path,
        }
    }
    
    /// Checks if event involves a directory
    pub fn is_dir(&self) -> bool {
        match self {
            WatchEvent::Created { is_dir, .. } => *is_dir,
            WatchEvent::Modified { is_dir, .. } => *is_dir,
            WatchEvent::Deleted { was_dir, .. } => *was_dir,
            WatchEvent::Renamed { is_dir, .. } => *is_dir,
            WatchEvent::PermissionsChanged { is_dir, .. } => *is_dir,
            WatchEvent::AttributesChanged { is_dir, .. } => *is_dir,
        }
    }
    
    /// Gets event type as string
    pub fn event_type(&self) -> &'static str {
        match self {
            WatchEvent::Created { .. } => "Created",
            WatchEvent::Modified { .. } => "Modified",
            WatchEvent::Deleted { .. } => "Deleted",
            WatchEvent::Renamed { .. } => "Renamed",
            WatchEvent::PermissionsChanged { .. } => "PermissionsChanged",
            WatchEvent::AttributesChanged { .. } => "AttributesChanged",
        }
    }
}

// =============================================================================
// WATCHER CONFIGURATION
// =============================================================================

/// Configuration for file system watcher
#[derive(Debug, Clone)]
pub struct WatcherConfig {
    /// Enable recursive monitoring of subdirectories
    pub recursive: bool,
    /// Include hidden files and directories
    pub include_hidden: bool,
    /// Follow symbolic links
    pub follow_symlinks: bool,
    /// Polling interval for changes (fallback when native events unavailable)
    pub poll_interval: Duration,
    /// Buffer size for event queue
    pub event_buffer_size: usize,
    /// File extensions to watch (empty = all files)
    pub extensions: HashSet<String>,
    /// Paths to ignore (relative to watch root)
    pub ignore_patterns: Vec<String>,
    /// Minimum file size to watch (0 = all files)
    pub min_file_size: u64,
    /// Maximum file size to watch (0 = no limit)
    pub max_file_size: u64,
    /// Debounce interval to avoid duplicate events
    pub debounce_interval: Duration,
}

impl Default for WatcherConfig {
    fn default() -> Self {
        Self {
            recursive: true,
            include_hidden: false,
            follow_symlinks: false,
            poll_interval: Duration::from_millis(1000),
            event_buffer_size: 1000,
            extensions: HashSet::new(),
            ignore_patterns: vec![
                ".git/**".to_string(),
                ".svn/**".to_string(),
                "node_modules/**".to_string(),
                "target/**".to_string(),
                "*.tmp".to_string(),
                "*.swp".to_string(),
            ],
            min_file_size: 0,
            max_file_size: 0,
            debounce_interval: Duration::from_millis(100),
        }
    }
}

impl WatcherConfig {
    /// Creates config for watching specific file extensions
    pub fn for_extensions(extensions: &[&str]) -> Self {
        let mut config = Self::default();
        config.extensions = extensions.iter().map(|s| s.to_string()).collect();
        config
    }
    
    /// Creates config for development environments (ignores common temp files)
    pub fn for_development() -> Self {
        let mut config = Self::default();
        config.ignore_patterns.extend([
            "*.log".to_string(),
            "*.pid".to_string(),
            ".DS_Store".to_string(),
            "Thumbs.db".to_string(),
            "~*".to_string(),
        ]);
        config
    }
    
    /// Creates config with custom polling interval
    pub fn with_poll_interval(interval: Duration) -> Self {
        let mut config = Self::default();
        config.poll_interval = interval;
        config
    }
}

// =============================================================================
// FILE SYSTEM WATCHER
// =============================================================================

/// File system watcher for monitoring changes
pub struct FileWatcher {
    config: WatcherConfig,
    watched_paths: Arc<Mutex<HashSet<PathBuf>>>,
    file_states: Arc<Mutex<HashMap<PathBuf, FileState>>>,
    event_sender: mpsc::Sender<WatchEvent>,
    event_receiver: Arc<Mutex<mpsc::Receiver<WatchEvent>>>,
    is_running: Arc<Mutex<bool>>,
    worker_handle: Option<thread::JoinHandle<()>>,
}

/// Internal file state for change detection
#[derive(Debug, Clone)]
struct FileState {
    size: u64,
    modified: SystemTime,
    permissions: u32,
    is_dir: bool,
}

impl FileWatcher {
    /// Creates a new file watcher with default configuration
    pub fn new() -> Self {
        Self::with_config(WatcherConfig::default())
    }
    
    /// Creates a new file watcher with custom configuration
    pub fn with_config(config: WatcherConfig) -> Self {
        let (sender, receiver) = mpsc::channel();
        
        Self {
            config,
            watched_paths: Arc::new(Mutex::new(HashSet::new())),
            file_states: Arc::new(Mutex::new(HashMap::new())),
            event_sender: sender,
            event_receiver: Arc::new(Mutex::new(receiver)),
            is_running: Arc::new(Mutex::new(false)),
            worker_handle: None,
        }
    }
    
    /// Adds a path to watch
    pub fn add_path<P: AsRef<Path>>(&mut self, path: P) -> FsResult<()> {
        let path = path.as_ref().to_path_buf();
        
        if !path.exists() {
            return Err(FsError::NotFound {
                path: path.to_string_lossy().to_string(),
                operation: "add_watch".to_string(),
            });
        }
        
        let mut watched_paths = self.watched_paths.lock().unwrap();
        watched_paths.insert(path.clone());
        
        // Initialize file states for this path
        self.initialize_file_states(&path)?;
        
        Ok(())
    }
    
    /// Removes a path from watching
    pub fn remove_path<P: AsRef<Path>>(&mut self, path: P) -> FsResult<()> {
        let path = path.as_ref().to_path_buf();
        
        let mut watched_paths = self.watched_paths.lock().unwrap();
        watched_paths.remove(&path);
        
        // Clean up file states for this path
        let mut file_states = self.file_states.lock().unwrap();
        file_states.retain(|p, _| !p.starts_with(&path));
        
        Ok(())
    }
    
    /// Starts watching for changes
    pub fn start(&mut self) -> FsResult<()> {
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            return Err(FsError::InvalidOperation {
                operation: "start_watcher".to_string(),
                reason: "Watcher is already running".to_string(),
            });
        }
        
        *is_running = true;
        drop(is_running);
        
        let watched_paths = Arc::clone(&self.watched_paths);
        let file_states = Arc::clone(&self.file_states);
        let event_sender = self.event_sender.clone();
        let is_running_ref = Arc::clone(&self.is_running);
        let config = self.config.clone();
        
        let handle = thread::spawn(move || {
            Self::watch_loop(watched_paths, file_states, event_sender, is_running_ref, config);
        });
        
        self.worker_handle = Some(handle);
        Ok(())
    }
    
    /// Stops watching for changes
    pub fn stop(&mut self) -> FsResult<()> {
        let mut is_running = self.is_running.lock().unwrap();
        if !*is_running {
            return Ok(());
        }
        
        *is_running = false;
        drop(is_running);
        
        if let Some(handle) = self.worker_handle.take() {
            handle.join().map_err(|_| FsError::SystemError {
                message: "Failed to join watcher thread".to_string(),
                code: 0,
            })?;
        }
        
        Ok(())
    }
    
    /// Receives next file system event (blocking)
    pub fn recv(&self) -> FsResult<WatchEvent> {
        let receiver = self.event_receiver.lock().unwrap();
        receiver.recv().map_err(|_| FsError::SystemError {
            message: "Event channel closed".to_string(),
            code: 0,
        })
    }
    
    /// Tries to receive next event (non-blocking)
    pub fn try_recv(&self) -> FsResult<Option<WatchEvent>> {
        let receiver = self.event_receiver.lock().unwrap();
        match receiver.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(mpsc::TryRecvError::Disconnected) => Err(FsError::SystemError {
                message: "Event channel closed".to_string(),
                code: 0,
            }),
        }
    }
    
    /// Receives events with timeout
    pub fn recv_timeout(&self, timeout: Duration) -> FsResult<Option<WatchEvent>> {
        let receiver = self.event_receiver.lock().unwrap();
        match receiver.recv_timeout(timeout) {
            Ok(event) => Ok(Some(event)),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(FsError::SystemError {
                message: "Event channel closed".to_string(),
                code: 0,
            }),
        }
    }
    
    /// Gets list of currently watched paths
    pub fn watched_paths(&self) -> Vec<PathBuf> {
        let watched_paths = self.watched_paths.lock().unwrap();
        watched_paths.iter().cloned().collect()
    }
    
    /// Checks if watcher is currently running
    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }
    
    /// Initializes file states for a path
    fn initialize_file_states(&self, root_path: &Path) -> FsResult<()> {
        let mut file_states = self.file_states.lock().unwrap();
        
        if root_path.is_file() {
            if let Ok(state) = Self::get_file_state(root_path) {
                file_states.insert(root_path.to_path_buf(), state);
            }
        } else if root_path.is_dir() {
            self.scan_directory(&mut file_states, root_path)?;
        }
        
        Ok(())
    }
    
    /// Recursively scans directory for files
    fn scan_directory(
        &self,
        file_states: &mut HashMap<PathBuf, FileState>,
        dir_path: &Path,
    ) -> FsResult<()> {
        let entries = fs::read_dir(dir_path).map_err(|e| FsError::SystemError {
            message: format!("Failed to read directory {}: {}", dir_path.display(), e),
            code: e.raw_os_error().unwrap_or(0),
        })?;
        
        for entry in entries {
            let entry = entry.map_err(|e| FsError::SystemError {
                message: format!("Failed to read directory entry: {}", e),
                code: e.raw_os_error().unwrap_or(0),
            })?;
            
            let path = entry.path();
            
            // Skip if should be ignored
            if self.should_ignore_path(&path) {
                continue;
            }
            
            // Get file state
            if let Ok(state) = Self::get_file_state(&path) {
                file_states.insert(path.clone(), state);
            }
            
            // Recurse into subdirectories if configured
            if self.config.recursive && path.is_dir() {
                self.scan_directory(file_states, &path)?;
            }
        }
        
        Ok(())
    }
    
    /// Main watch loop (runs in background thread)
    fn watch_loop(
        watched_paths: Arc<Mutex<HashSet<PathBuf>>>,
        file_states: Arc<Mutex<HashMap<PathBuf, FileState>>>,
        event_sender: mpsc::Sender<WatchEvent>,
        is_running: Arc<Mutex<bool>>,
        config: WatcherConfig,
    ) {
        let mut last_scan = SystemTime::now();
        
        while *is_running.lock().unwrap() {
            let now = SystemTime::now();
            
            // Only scan at specified intervals
            if now.duration_since(last_scan).unwrap_or(Duration::ZERO) >= config.poll_interval {
                Self::scan_for_changes(
                    &watched_paths,
                    &file_states,
                    &event_sender,
                    &config,
                );
                last_scan = now;
            }
            
            thread::sleep(Duration::from_millis(10));
        }
    }
    
    /// Scans for file system changes
    fn scan_for_changes(
        watched_paths: &Arc<Mutex<HashSet<PathBuf>>>,
        file_states: &Arc<Mutex<HashMap<PathBuf, FileState>>>,
        event_sender: &mpsc::Sender<WatchEvent>,
        config: &WatcherConfig,
    ) {
        let watched = watched_paths.lock().unwrap().clone();
        let mut states = file_states.lock().unwrap();
        
        // Check each watched path
        for watch_path in watched {
            Self::check_path_changes(&watch_path, &mut states, event_sender, config);
        }
        
        // Check for deleted files
        let existing_paths: HashSet<PathBuf> = states.keys().cloned().collect();
        for path in existing_paths {
            if !path.exists() {
                if let Some(old_state) = states.remove(&path) {
                    let _ = event_sender.send(WatchEvent::Deleted {
                        path,
                        was_dir: old_state.is_dir,
                    });
                }
            }
        }
    }
    
    /// Checks changes for a specific path
    fn check_path_changes(
        path: &Path,
        file_states: &mut HashMap<PathBuf, FileState>,
        event_sender: &mpsc::Sender<WatchEvent>,
        config: &WatcherConfig,
    ) {
        if path.is_file() {
            Self::check_file_changes(path, file_states, event_sender, config);
        } else if path.is_dir() {
            Self::check_directory_changes(path, file_states, event_sender, config);
        }
    }
    
    /// Checks changes for a file
    fn check_file_changes(
        file_path: &Path,
        file_states: &mut HashMap<PathBuf, FileState>,
        event_sender: &mpsc::Sender<WatchEvent>,
        _config: &WatcherConfig,
    ) {
        if let Ok(current_state) = Self::get_file_state(file_path) {
            match file_states.get(file_path) {
                Some(old_state) => {
                    // Check for modifications
                    if current_state.modified != old_state.modified || current_state.size != old_state.size {
                        let _ = event_sender.send(WatchEvent::Modified {
                            path: file_path.to_path_buf(),
                            is_dir: false,
                        });
                    }
                    
                    if current_state.permissions != old_state.permissions {
                        let _ = event_sender.send(WatchEvent::PermissionsChanged {
                            path: file_path.to_path_buf(),
                            is_dir: false,
                        });
                    }
                    
                    // Update state
                    file_states.insert(file_path.to_path_buf(), current_state);
                }
                None => {
                    // New file
                    let _ = event_sender.send(WatchEvent::Created {
                        path: file_path.to_path_buf(),
                        is_dir: false,
                    });
                    file_states.insert(file_path.to_path_buf(), current_state);
                }
            }
        }
    }
    
    /// Checks changes for a directory
    fn check_directory_changes(
        dir_path: &Path,
        file_states: &mut HashMap<PathBuf, FileState>,
        event_sender: &mpsc::Sender<WatchEvent>,
        config: &WatcherConfig,
    ) {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                
                // Skip ignored paths
                if Self::should_ignore_path_static(&path, config) {
                    continue;
                }
                
                Self::check_file_changes(&path, file_states, event_sender, config);
                
                // Recurse into subdirectories
                if config.recursive && path.is_dir() {
                    Self::check_directory_changes(&path, file_states, event_sender, config);
                }
            }
        }
    }
    
    /// Gets current state of a file
    fn get_file_state(path: &Path) -> Result<(), Error> {
        let metadata = fs::metadata(path)?;
        
        Ok(FileState {
            size: metadata.len(),
            modified: metadata.modified()?,
            permissions: Self::get_permissions(&metadata),
            is_dir: metadata.is_dir(),
        })
    }
    
    /// Extracts permissions from metadata (platform-specific)
    #[cfg(unix)]
    fn get_permissions(metadata: &fs::Metadata) -> u32 {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode()
    }
    
    #[cfg(not(unix))]
    fn get_permissions(metadata: &fs::Metadata) -> u32 {
        if metadata.permissions().readonly() { 0o444 } else { 0o666 }
    }
    
    /// Checks if path should be ignored
    fn should_ignore_path(&self, path: &Path) -> bool {
        Self::should_ignore_path_static(path, &self.config)
    }
    
    /// Static version of should_ignore_path
    fn should_ignore_path_static(path: &Path, config: &WatcherConfig) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check if hidden and hidden files not included
        if !config.include_hidden {
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().starts_with('.') {
                    return true;
                }
            }
        }
        
        // Check ignore patterns
        for pattern in &config.ignore_patterns {
            if Self::matches_pattern(&path_str, pattern) {
                return true;
            }
        }
        
        // Check file extensions
        if !config.extensions.is_empty() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if !config.extensions.contains(&ext_str) {
                    return true;
                }
            } else {
                return true; // No extension, but extensions are specified
            }
        }
        
        // Check file size
        if path.is_file() {
            if let Ok(metadata) = fs::metadata(path) {
                let size = metadata.len();
                if config.min_file_size > 0 && size < config.min_file_size {
                    return true;
                }
                if config.max_file_size > 0 && size > config.max_file_size {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Simple pattern matching (supports * wildcards)
    fn matches_pattern(text: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.is_empty() {
                return false;
            }
            
            let mut text_pos = 0;
            for (i, part) in parts.iter().enumerate() {
                if part.is_empty() {
                    continue;
                }
                
                if i == 0 {
                    // First part must match at start
                    if !text[text_pos..].starts_with(part) {
                        return false;
                    }
                    text_pos += part.len();
                } else if i == parts.len() - 1 {
                    // Last part must match at end
                    return text[text_pos..].ends_with(part);
                } else {
                    // Middle parts must be found
                    if let Some(pos) = text[text_pos..].find(part) {
                        text_pos += pos + part.len();
                    } else {
                        return false;
                    }
                }
            }
            true
        } else {
            text.contains(pattern)
        }
    }
}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

// =============================================================================
// CONVENIENCE FUNCTIONS
// =============================================================================

/// Creates a simple file watcher for a single path
pub fn watch_path<P: AsRef<Path>>(path: P) -> FsResult<FileWatcher> {
    let mut watcher = FileWatcher::new();
    watcher.add_path(path)?;
    watcher.start()?;
    Ok(watcher)
}

/// Creates a file watcher with custom configuration
pub fn watch_path_with_config<P: AsRef<Path>>(
    path: P,
    config: WatcherConfig,
) -> FsResult<FileWatcher> {
    let mut watcher = FileWatcher::with_config(config);
    watcher.add_path(path)?;
    watcher.start()?;
    Ok(watcher)
}

/// Watches multiple paths with default configuration
pub fn watch_paths<P: AsRef<Path>>(paths: &[P]) -> FsResult<FileWatcher> {
    let mut watcher = FileWatcher::new();
    for path in paths {
        watcher.add_path(path)?;
    }
    watcher.start()?;
    Ok(watcher)
}

/// Simple one-shot function to wait for next change in directory
pub fn wait_for_changes<P: AsRef<Path>>(path: P, timeout: Duration) -> FsResult<Option<WatchEvent>> {
    let mut watcher = watch_path(path)?;
    watcher.recv_timeout(timeout)
}

#[cfg(test)]
mod tests {
    use super::*;
use crate::error::Error;
    use std::fs;
    use std::io::Write;
    
    #[test]
    fn test_watcher_creation() {
        let watcher = FileWatcher::new();
        assert!(!watcher.is_running());
        assert_eq!(watcher.watched_paths().len(), 0);
    }
    
    #[test]
    fn test_config_creation() {
        let config = WatcherConfig::for_extensions(&["rs", "toml"]);
        assert!(config.extensions.contains("rs"));
        assert!(config.extensions.contains("toml"));
    }
    
    #[test]
    fn test_pattern_matching() {
        assert!(FileWatcher::matches_pattern("test.txt", "*.txt"));
        assert!(FileWatcher::matches_pattern("src/main.rs", "src/*"));
        assert!(!FileWatcher::matches_pattern("test.rs", "*.txt"));
    }
    
    #[test]
    fn test_ignore_patterns() {
        let config = WatcherConfig::default();
        assert!(FileWatcher::should_ignore_path_static(
            Path::new(".git/config"),
            &config
        ));
        assert!(FileWatcher::should_ignore_path_static(
            Path::new("target/debug/main"),
            &config
        ));
    }
    
    #[test]
    fn test_file_state() {
        // Create a temporary file
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("watcher_test.txt");
        
        fs::write(&test_file, "test content").unwrap();
        
        let state = FileWatcher::get_file_state(&test_file).unwrap();
        assert_eq!(state.size, 12);
        assert!(!state.is_dir);
        
        // Clean up
        let _ = fs::remove_file(test_file);
    }
}
