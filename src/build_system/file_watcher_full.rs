use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher,
};
use glob;

use crate::error::Error as CursedError;

pub type CursedResult<T> = Result<T, CursedError>;

/// Configuration for file watching behavior
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// File patterns to watch (e.g., ["*.csd", "*.toml", "*.md"])
    pub watch_patterns: Vec<String>,
    
    /// File patterns to ignore (e.g., ["*.tmp", "target/*", ".git/*"])
    pub ignore_patterns: Vec<String>,
    
    /// Debounce duration to prevent rapid-fire events
    pub debounce_duration: Duration,
    
    /// Maximum number of events to batch together
    pub max_batch_size: usize,
    
    /// Whether to watch directories recursively
    pub recursive: bool,
    
    /// Whether to follow symbolic links
    pub follow_symlinks: bool,
    
    /// Buffer size for event channel
    pub event_buffer_size: usize,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            watch_patterns: vec![
                "*.csd".to_string(),
                "*.toml".to_string(),
                "*.md".to_string(),
                "Makefile".to_string(),
            ],
            ignore_patterns: vec![
                "*.tmp".to_string(),
                "*.bak".to_string(),
                "target/*".to_string(),
                ".git/*".to_string(),
                ".devenv/*".to_string(),
                "coverage/*".to_string(),
            ],
            debounce_duration: Duration::from_millis(500),
            max_batch_size: 50,
            recursive: true,
            follow_symlinks: false,
            event_buffer_size: 1000,
        }
    }
}

/// Types of file watch events
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileWatchEvent {
    /// File was created
    Created {
        path: PathBuf,
        timestamp: SystemTime,
    },
    /// File was modified
    Modified {
        path: PathBuf,
        timestamp: SystemTime,
    },
    /// File was deleted
    Deleted {
        path: PathBuf,
        timestamp: SystemTime,
    },
    /// File was renamed
    Renamed {
        from: PathBuf,
        to: PathBuf,
        timestamp: SystemTime,
    },
    /// Directory was created
    DirectoryCreated {
        path: PathBuf,
        timestamp: SystemTime,
    },
    /// Directory was deleted
    DirectoryDeleted {
        path: PathBuf,
        timestamp: SystemTime,
    },
    /// Batch of events processed together
    Batch {
        events: Vec<FileWatchEvent>,
        timestamp: SystemTime,
    },
}

impl FileWatchEvent {
    /// Get the primary path affected by this event
    pub fn path(&self) -> &Path {
        match self {
            FileWatchEvent::Created { path, .. } => path,
            FileWatchEvent::Modified { path, .. } => path,
            FileWatchEvent::Deleted { path, .. } => path,
            FileWatchEvent::Renamed { to, .. } => to,
            FileWatchEvent::DirectoryCreated { path, .. } => path,
            FileWatchEvent::DirectoryDeleted { path, .. } => path,
            FileWatchEvent::Batch { events, .. } => {
                events.first().map(|e| e.path()).unwrap_or(Path::new(""))
            }
        }
    }
    
    /// Get the timestamp of this event
    pub fn timestamp(&self) -> SystemTime {
        match self {
            FileWatchEvent::Created { timestamp, .. } => *timestamp,
            FileWatchEvent::Modified { timestamp, .. } => *timestamp,
            FileWatchEvent::Deleted { timestamp, .. } => *timestamp,
            FileWatchEvent::Renamed { timestamp, .. } => *timestamp,
            FileWatchEvent::DirectoryCreated { timestamp, .. } => *timestamp,
            FileWatchEvent::DirectoryDeleted { timestamp, .. } => *timestamp,
            FileWatchEvent::Batch { timestamp, .. } => *timestamp,
        }
    }
    
    /// Check if this event indicates a file change that should trigger a rebuild
    pub fn should_trigger_rebuild(&self) -> bool {
        match self {
            FileWatchEvent::Created { .. } => true,
            FileWatchEvent::Modified { .. } => true,
            FileWatchEvent::Deleted { .. } => true,
            FileWatchEvent::Renamed { .. } => true,
            FileWatchEvent::DirectoryCreated { .. } => false,
            FileWatchEvent::DirectoryDeleted { .. } => true,
            FileWatchEvent::Batch { events, .. } => {
                events.iter().any(|e| e.should_trigger_rebuild())
            }
        }
    }
}

/// Information about a watched path
#[derive(Debug, Clone)]
pub struct WatchedPath {
    /// The path being watched
    pub path: PathBuf,
    
    /// When this path was first watched
    pub watch_started: SystemTime,
    
    /// Last time this path was modified
    pub last_modified: Option<SystemTime>,
    
    /// Number of events seen for this path
    pub event_count: u64,
    
    /// Whether this is a directory
    pub is_directory: bool,
    
    /// File size (if applicable)
    pub file_size: Option<u64>,
}

impl WatchedPath {
    /// Create a new WatchedPath
    pub fn new(path: PathBuf) -> CursedResult<Self> {
        let metadata = std::fs::metadata(&path)
            .map_err(|e| CursedError::system_error(&format!("Failed to get metadata for {}: {}", path.display(), e)))?;
        
        Ok(Self {
            path,
            watch_started: SystemTime::now(),
            last_modified: metadata.modified().ok(),
            event_count: 0,
            is_directory: metadata.is_dir(),
            file_size: if metadata.is_file() { Some(metadata.len()) } else { None },
        })
    }
    
    /// Update metadata for this watched path
    pub fn update_metadata(&mut self) -> CursedResult<()> {
        if let Ok(metadata) = std::fs::metadata(&self.path) {
            self.last_modified = metadata.modified().ok();
            self.file_size = if metadata.is_file() { Some(metadata.len()) } else { None };
        }
        self.event_count += 1;
        Ok(())
    }
}

/// Manages debouncing of file system events to prevent rapid-fire triggers
#[derive(Debug)]
pub struct DebounceManager {
    /// Map of path to last event time
    last_events: Arc<Mutex<HashMap<PathBuf, Instant>>>,
    
    /// Debounce duration
    duration: Duration,
    
    /// Pending events waiting for debounce
    pending_events: Arc<Mutex<HashMap<PathBuf, FileWatchEvent>>>,
}

impl DebounceManager {
    /// Create a new debounce manager
    pub fn new(duration: Duration) -> Self {
        Self {
            last_events: Arc::new(Mutex::new(HashMap::new())),
            duration,
            pending_events: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Check if an event should be processed (not debounced)
    pub fn should_process_event(&self, event: &FileWatchEvent) -> bool {
        let path = event.path().to_path_buf();
        let now = Instant::now();
        
        let mut last_events = self.last_events.lock().unwrap();
        
        if let Some(last_time) = last_events.get(&path) {
            if now.duration_since(*last_time) < self.duration {
                // Event is too recent, should be debounced
                let mut pending = self.pending_events.lock().unwrap();
                pending.insert(path, event.clone());
                return false;
            }
        }
        
        last_events.insert(path, now);
        true
    }
    
    /// Get all pending events that are ready to be processed
    pub fn get_ready_events(&self) -> Vec<FileWatchEvent> {
        let now = Instant::now();
        let mut ready_events = Vec::new();
        
        let mut last_events = self.last_events.lock().unwrap();
        let mut pending = self.pending_events.lock().unwrap();
        
        let mut to_remove = Vec::new();
        
        for (path, event) in pending.iter() {
            if let Some(last_time) = last_events.get(path) {
                if now.duration_since(*last_time) >= self.duration {
                    ready_events.push(event.clone());
                    to_remove.push(path.clone());
                }
            }
        }
        
        for path in to_remove {
            pending.remove(&path);
            last_events.insert(path, now);
        }
        
        ready_events
    }
    
    /// Clear all pending events (useful for shutdown)
    pub fn clear_pending(&self) {
        let mut pending = self.pending_events.lock().unwrap();
        pending.clear();
    }
}

/// Filters file system events based on patterns and rules
#[derive(Debug)]
pub struct EventFilter {
    /// Patterns to include
    include_patterns: Vec<glob::Pattern>,
    
    /// Patterns to exclude
    exclude_patterns: Vec<glob::Pattern>,
    
    /// Whether to follow symlinks
    follow_symlinks: bool,
}

impl EventFilter {
    /// Create a new event filter
    pub fn new(config: &WatchConfig) -> CursedResult<Self> {
        let mut include_patterns = Vec::new();
        for pattern in &config.watch_patterns {
            let glob_pattern = glob::Pattern::new(pattern)
                .map_err(|e| CursedError::system_error(&format!("Invalid include pattern '{}': {}", pattern, e)))?;
            include_patterns.push(glob_pattern);
        }
        
        let mut exclude_patterns = Vec::new();
        for pattern in &config.ignore_patterns {
            let glob_pattern = glob::Pattern::new(pattern)
                .map_err(|e| CursedError::system_error(&format!("Invalid exclude pattern '{}': {}", pattern, e)))?;
            exclude_patterns.push(glob_pattern);
        }
        
        Ok(Self {
            include_patterns,
            exclude_patterns,
            follow_symlinks: config.follow_symlinks,
        })
    }
    
    /// Check if a path should be watched based on the filter rules
    pub fn should_watch(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check if path matches any exclude pattern
        for pattern in &self.exclude_patterns {
            if pattern.matches(&path_str) {
                return false;
            }
        }
        
        // If no include patterns, watch everything not excluded
        if self.include_patterns.is_empty() {
            return true;
        }
        
        // Check if path matches any include pattern
        for pattern in &self.include_patterns {
            if pattern.matches(&path_str) {
                return true;
            }
        }
        
        false
    }
    
    /// Check if a symlink should be followed
    pub fn should_follow_symlink(&self, _path: &Path) -> bool {
        self.follow_symlinks
    }
}

/// Main file watcher coordinator
pub struct FileWatcher {
    /// Configuration
    config: WatchConfig,
    
    /// Underlying notify watcher
    watcher: Option<RecommendedWatcher>,
    
    /// Event receiver channel
    event_receiver: Option<mpsc::Receiver<NotifyResult<Event>>>,
    
    /// Paths currently being watched
    watched_paths: Arc<RwLock<HashMap<PathBuf, WatchedPath>>>,
    
    /// Event filter
    filter: EventFilter,
    
    /// Debounce manager
    debouncer: DebounceManager,
    
    /// Event callback function
    event_callback: Arc<Mutex<Option<Box<dyn Fn(FileWatchEvent) + Send + 'static>>>>,
    
    /// Whether the watcher is currently running
    is_running: Arc<Mutex<bool>>,
    
    /// Background thread handle
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl FileWatcher {
    /// Create a new file watcher with the given configuration
    pub fn new(config: WatchConfig) -> CursedResult<Self> {
        let filter = EventFilter::new(&config)?;
        let debouncer = DebounceManager::new(config.debounce_duration);
        
        Ok(Self {
            config,
            watcher: None,
            event_receiver: None,
            watched_paths: Arc::new(RwLock::new(HashMap::new())),
            filter,
            debouncer,
            event_callback: Arc::new(Mutex::new(None)),
            is_running: Arc::new(Mutex::new(false)),
            thread_handle: None,
        })
    }
    
    /// Set the callback function to be called when events are processed
    pub fn set_event_callback<F>(&mut self, callback: F) -> CursedResult<()>
    where
        F: Fn(FileWatchEvent) + Send + 'static,
    {
        let mut cb = self.event_callback.lock().unwrap();
        *cb = Some(Box::new(callback));
        Ok(())
    }
    
    /// Start watching the specified paths
    pub fn start_watching<P: AsRef<Path>>(&mut self, paths: &[P]) -> CursedResult<()> {
        let (tx, rx) = mpsc::channel::<NotifyResult<Event>>();
        
        let mut watcher = RecommendedWatcher::new(
            tx,
            Config::default()
                .with_poll_interval(Duration::from_millis(100))
        ).map_err(|e| CursedError::system_error(&format!("Failed to create file watcher: {}", e)))?;
        
        // Add paths to watcher
        for path in paths {
            let path_buf = path.as_ref().to_path_buf();
            
            if !path_buf.exists() {
                return Err(CursedError::system_error(&format!("Path does not exist: {}", path_buf.display())));
            }
            
            let recursive_mode = if self.config.recursive {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            };
            
            watcher.watch(&path_buf, recursive_mode)
                .map_err(|e| CursedError::system_error(&format!("Failed to watch path {}: {}", path_buf.display(), e)))?;
            
            // Add to watched paths
            let watched_path = WatchedPath::new(path_buf.clone())?;
            let mut watched_paths = self.watched_paths.write().unwrap();
            watched_paths.insert(path_buf, watched_path);
        }
        
        self.watcher = Some(watcher);
        self.event_receiver = Some(rx);
        
        // Start the event processing thread
        self.start_event_thread()?;
        
        let mut running = self.is_running.lock().unwrap();
        *running = true;
        
        Ok(())
    }
    
    /// Stop watching all paths
    pub fn stop_watching(&mut self) -> CursedResult<()> {
        let mut running = self.is_running.lock().unwrap();
        *running = false;
        drop(running);
        
        // Wait for background thread to finish
        if let Some(handle) = self.thread_handle.take() {
            handle.join().map_err(|_| CursedError::system_error("Failed to join file watcher thread"))?;
        }
        
        self.watcher = None;
        self.event_receiver = None;
        self.debouncer.clear_pending();
        
        let mut watched_paths = self.watched_paths.write().unwrap();
        watched_paths.clear();
        
        Ok(())
    }
    
    /// Check if the watcher is currently running
    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }
    
    /// Get information about all watched paths
    pub fn get_watched_paths(&self) -> HashMap<PathBuf, WatchedPath> {
        let watched_paths = self.watched_paths.read().unwrap();
        watched_paths.clone()
    }
    
    /// Get statistics about file watching
    pub fn get_statistics(&self) -> WatchStatistics {
        let watched_paths = self.watched_paths.read().unwrap();
        let total_paths = watched_paths.len();
        let total_events: u64 = watched_paths.values().map(|p| p.event_count).sum();
        let directories = watched_paths.values().filter(|p| p.is_directory).count();
        let files = total_paths - directories;
        
        WatchStatistics {
            total_watched_paths: total_paths,
            total_directories: directories,
            total_files: files,
            total_events_processed: total_events,
            is_running: self.is_running(),
        }
    }
    
    /// Start the background event processing thread
    fn start_event_thread(&mut self) -> CursedResult<()> {
        let receiver = self.event_receiver.take()
            .ok_or_else(|| CursedError::system_error("No event receiver available"))?;
        
        let watched_paths = Arc::clone(&self.watched_paths);
        let filter = EventFilter::new(&self.config)?;
        let event_callback = Arc::clone(&self.event_callback);
        let is_running = Arc::clone(&self.is_running);
        let max_batch_size = self.config.max_batch_size;
        
        let handle = thread::spawn(move || {
            let mut event_batch = Vec::new();
            let mut last_batch_time = Instant::now();
            let batch_timeout = Duration::from_millis(100);
            
            while *is_running.lock().unwrap() {
                // Process events from the channel
                match receiver.recv_timeout(batch_timeout) {
                    Ok(Ok(event)) => {
                        if let Some(watch_event) = Self::convert_notify_event(event, &filter) {
                            event_batch.push(watch_event);
                            
                            // Update watched path metadata
                            if let Ok(mut paths) = watched_paths.write() {
                                if let Some(watched_path) = paths.get_mut(watch_event.path()) {
                                    let _ = watched_path.update_metadata();
                                }
                            }
                        }
                    }
                    Ok(Err(e)) => {
                        eprintln!("File watcher error: {}", e);
                        continue;
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        // Timeout occurred, process batch if we have events
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        break;
                    }
                }
                
                // Process batch if it's full or enough time has passed
                let should_process_batch = !event_batch.is_empty() && (
                    event_batch.len() >= max_batch_size ||
                    last_batch_time.elapsed() >= batch_timeout
                );
                
                if should_process_batch {
                    let batch_event = if event_batch.len() == 1 {
                        event_batch.pop().unwrap()
                    } else {
                        FileWatchEvent::Batch {
                            events: event_batch.clone(),
                            timestamp: SystemTime::now(),
                        }
                    };
                    
                    // Call the event callback
                    if let Ok(callback_guard) = event_callback.lock() {
                        if let Some(ref callback) = *callback_guard {
                            callback(batch_event);
                        }
                    }
                    
                    event_batch.clear();
                    last_batch_time = Instant::now();
                }
            }
        });
        
        self.thread_handle = Some(handle);
        self.event_receiver = Some(receiver);
        
        Ok(())
    }
    
    /// Convert a notify event to our internal event type
    fn convert_notify_event(event: Event, filter: &EventFilter) -> Option<FileWatchEvent> {
        let timestamp = SystemTime::now();
        
        for path in &event.paths {
            if !filter.should_watch(path) {
                continue;
            }
            
            match event.kind {
                EventKind::Create(_) => {
                    if path.is_dir() {
                        return Some(FileWatchEvent::DirectoryCreated {
                            path: path.to_path_buf(),
                            timestamp,
                        });
                    } else {
                        return Some(FileWatchEvent::Created {
                            path: path.to_path_buf(),
                            timestamp,
                        });
                    }
                }
                EventKind::Modify(_) => {
                    return Some(FileWatchEvent::Modified {
                        path: path.to_path_buf(),
                        timestamp,
                    });
                }
                EventKind::Remove(_) => {
                    if path.is_dir() {
                        return Some(FileWatchEvent::DirectoryDeleted {
                            path: path.to_path_buf(),
                            timestamp,
                        });
                    } else {
                        return Some(FileWatchEvent::Deleted {
                            path: path.to_path_buf(),
                            timestamp,
                        });
                    }
                }
                _ => {
                    // Handle other event types as needed
                    continue;
                }
            }
        }
        
        None
    }
}

impl Drop for FileWatcher {
    fn drop(&mut self) {
        let _ = self.stop_watching();
    }
}

/// Statistics about file watching activity
#[derive(Debug, Clone)]
pub struct WatchStatistics {
    /// Total number of paths being watched
    pub total_watched_paths: usize,
    
    /// Number of directories being watched
    pub total_directories: usize,
    
    /// Number of files being watched
    pub total_files: usize,
    
    /// Total number of events processed
    pub total_events_processed: u64,
    
    /// Whether the watcher is currently running
    pub is_running: bool,
}

/// Builder for creating FileWatcher instances with custom configuration
pub struct FileWatcherBuilder {
    config: WatchConfig,
}

impl FileWatcherBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
            config: WatchConfig::default(),
        }
    }
    
    /// Set the file patterns to watch
    pub fn watch_patterns(mut self, patterns: Vec<String>) -> Self {
        self.config.watch_patterns = patterns;
        self
    }
    
    /// Set the file patterns to ignore
    pub fn ignore_patterns(mut self, patterns: Vec<String>) -> Self {
        self.config.ignore_patterns = patterns;
        self
    }
    
    /// Set the debounce duration
    pub fn debounce_duration(mut self, duration: Duration) -> Self {
        self.config.debounce_duration = duration;
        self
    }
    
    /// Set the maximum batch size
    pub fn max_batch_size(mut self, size: usize) -> Self {
        self.config.max_batch_size = size;
        self
    }
    
    /// Set whether to watch recursively
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.config.recursive = recursive;
        self
    }
    
    /// Set whether to follow symlinks
    pub fn follow_symlinks(mut self, follow: bool) -> Self {
        self.config.follow_symlinks = follow;
        self
    }
    
    /// Build the FileWatcher
    pub fn build(self) -> CursedResult<FileWatcher> {
        FileWatcher::new(self.config)
    }
}

impl Default for FileWatcherBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_watch_config_default() {
        let config = WatchConfig::default();
        assert!(config.watch_patterns.contains(&"*.csd".to_string()));
        assert!(config.ignore_patterns.contains(&"target/*".to_string()));
        assert_eq!(config.debounce_duration, Duration::from_millis(500));
    }
    
    #[test]
    fn test_file_watch_event_properties() {
        let path = PathBuf::from("/test/file.csd");
        let timestamp = SystemTime::now();
        
        let event = FileWatchEvent::Created {
            path: path.clone(),
            timestamp,
        };
        
        assert_eq!(event.path(), &path);
        assert_eq!(event.timestamp(), timestamp);
        assert!(event.should_trigger_rebuild());
    }
    
    #[test]
    fn test_debounce_manager() {
        let debouncer = DebounceManager::new(Duration::from_millis(100));
        let event = FileWatchEvent::Created {
            path: PathBuf::from("/test/file.csd"),
            timestamp: SystemTime::now(),
        };
        
        // First event should be processed
        assert!(debouncer.should_process_event(&event));
        
        // Immediate second event should be debounced
        assert!(!debouncer.should_process_event(&event));
    }
    
    #[test]
    fn test_event_filter() {
        let config = WatchConfig::default();
        let filter = EventFilter::new(&config).unwrap();
        
        assert!(filter.should_watch(Path::new("test.csd")));
        assert!(filter.should_watch(Path::new("Cargo.toml")));
        assert!(!filter.should_watch(Path::new("test.tmp")));
        assert!(!filter.should_watch(Path::new("target/debug/test")));
    }
    
    #[test]
    fn test_file_watcher_builder() {
        let watcher = FileWatcherBuilder::new()
            .watch_patterns(vec!["*.rs".to_string()])
            .debounce_duration(Duration::from_millis(1000))
            .recursive(false)
            .build()
            .unwrap();
        
        assert!(!watcher.is_running());
    }
}
