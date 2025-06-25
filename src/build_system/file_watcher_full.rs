use crate::error::CursedError;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, mpsc, RwLock};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use notify::{
// };

use glob;


pub type CursedResult<T> = std::result::Result<T, SystemError>;

/// Configuration for file watching behavior
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// File patterns to watch (e.g., ["*.csd", "*.toml", "*.md"])
    
    /// File patterns to ignore (e.g., ["*.tmp", "target/*", ".git/*"])
    
    /// Debounce duration to prevent rapid-fire events
    
    /// Maximum number of events to batch together
    
    /// Whether to watch directories recursively
    
    /// Whether to follow symbolic links
    
    /// Buffer size for event channel
impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            watch_patterns: vec![
            ignore_patterns: vec![
                "target/*".to_string(),
                ".git/*".to_string(),
                ".devenv/*".to_string(),
                "coverage/*".to_string(),
        }
    }
/// Types of file watch events
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileWatchEvent {
    /// File was created
    Created {
    /// File was modified
    Modified {
    /// File was deleted
    Deleted {
    /// File was renamed
    Renamed {
    /// Directory was created
    DirectoryCreated {
    /// Directory was deleted
    DirectoryDeleted {
    /// Batch of events processed together
    Batch {
impl FileWatchEvent {
    /// Get the primary path affected by this event
    pub fn path(&self) -> &Path {
        match self {
            FileWatchEvent::Batch { events, .. } => {
                events.first().map(|e| e.path()).unwrap_or(Path::new(""))
            }
        }
    /// Get the timestamp of this event
    pub fn timestamp(&self) -> SystemTime {
        match self {
        }
    }
    
    /// Check if this event indicates a file change that should trigger a rebuild
    pub fn should_trigger_rebuild(&self) -> bool {
        match self {
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
    
    /// When this path was first watched
    
    /// Last time this path was modified
    
    /// Number of events seen for this path
    
    /// Whether this is a directory
    
    /// File size (if applicable)
impl WatchedPath {
    /// Create a new WatchedPath
    pub fn new(path: PathBuf) -> CursedResult<Self> {
        let metadata = std::fs::metadata(&path)
            .map_err(|e| CursedError::system_error(&format!("Failed to get metadata for {}: {}", path.display(), e)))?;
        
        Ok(Self {
        })
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
    
    /// Debounce duration
    
    /// Pending events waiting for debounce
impl DebounceManager {
    /// Create a new debounce manager
    pub fn new(duration: Duration) -> Self {
        Self {
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
        for path in to_remove {
            pending.remove(&path);
            last_events.insert(path, now);
        ready_events
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
    
    /// Patterns to exclude
    
    /// Whether to follow symlinks
impl EventFilter {
    /// Create a new event filter
    pub fn new(config: &WatchConfig) -> CursedResult<Self> {
        let mut include_patterns = Vec::new();
        for pattern in &config.watch_patterns {
            let glob_pattern = glob::Pattern::new(pattern)
                .map_err(|e| CursedError::system_error(&format!("Invalid include pattern '{}': {}", pattern, e)))?;
            include_patterns.push(glob_pattern);
        let mut exclude_patterns = Vec::new();
        for pattern in &config.ignore_patterns {
            let glob_pattern = glob::Pattern::new(pattern)
                .map_err(|e| CursedError::system_error(&format!("Invalid exclude pattern '{}': {}", pattern, e)))?;
            exclude_patterns.push(glob_pattern);
        Ok(Self {
        })
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
        // Check if path matches any include pattern
        for pattern in &self.include_patterns {
            if pattern.matches(&path_str) {
                return true;
            }
        }
        
        false
    /// Check if a symlink should be followed
    pub fn should_follow_symlink(&self, _path: &Path) -> bool {
        self.follow_symlinks
    }
}

/// Main file watcher coordinator
pub struct FileWatcher {
    /// Configuration
    
    /// Underlying notify watcher
    
    /// Event receiver channel
    
    /// Paths currently being watched
    
    /// Event filter
    
    /// Debounce manager
    
    /// Event callback function
    
    /// Whether the watcher is currently running
    
    /// Background thread handle
impl FileWatcher {
    /// Create a new file watcher with the given configuration
    pub fn new(config: WatchConfig) -> CursedResult<Self> {
        let filter = EventFilter::new(&config)?;
        let debouncer = DebounceManager::new(config.debounce_duration);
        
        Ok(Self {
        })
    /// Set the callback function to be called when events are processed
    pub fn set_event_callback<F>(&mut self, callback: F) -> CursedResult<()>
    where
    {
        let mut cb = self.event_callback.lock().unwrap();
        *cb = Some(Box::new(callback));
        Ok(())
    /// Start watching the specified paths
    pub fn start_watching<P: AsRef<Path>>(&mut self, paths: &[P]) -> CursedResult<()> {
        let (tx, rx) = mpsc::channel::<NotifyResult<Event>>();
        
        let mut watcher = RecommendedWatcher::new(
            Config::default()
                .with_poll_interval(Duration::from_millis(100))
        ).map_err(|e| CursedError::system_error(&format!("Failed to create file watcher: {}", e)))?;
        
        // Add paths to watcher
        for path in paths {
            let path_buf = path.as_ref().to_path_buf();
            
            if !path_buf.exists() {
                return Err(CursedError::system_error(&format!("Path does not exist: {}", path_buf.display())));
            let recursive_mode = if self.config.recursive {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            
            watcher.watch(&path_buf, recursive_mode)
                .map_err(|e| CursedError::system_error(&format!("Failed to watch path {}: {}", path_buf.display(), e)))?;
            
            // Add to watched paths
            let watched_path = WatchedPath::new(path_buf.clone())?;
            let mut watched_paths = self.watched_paths.write().unwrap();
            watched_paths.insert(path_buf, watched_path);
        self.watcher = Some(watcher);
        self.event_receiver = Some(rx);
        
        // Start the event processing thread
        self.start_event_thread()?;
        
        let mut running = self.is_running.lock().unwrap();
        *running = true;
        
        Ok(())
    /// Stop watching all paths
    pub fn stop_watching(&mut self) -> CursedResult<()> {
        let mut running = self.is_running.lock().unwrap();
        *running = false;
        drop(running);
        
        // Wait for background thread to finish
        if let Some(handle) = self.thread_handle.take() {
            handle.join().map_err(|_| CursedError::system_error("Failed to join file watcher thread"))?;
        self.watcher = None;
        self.event_receiver = None;
        self.debouncer.clear_pending();
        
        let mut watched_paths = self.watched_paths.write().unwrap();
        watched_paths.clear();
        
        Ok(())
    /// Check if the watcher is currently running
    pub fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    /// Get information about all watched paths
    pub fn get_watched_paths(&self) -> HashMap<PathBuf, WatchedPath> {
        let watched_paths = self.watched_paths.read().unwrap();
        watched_paths.clone()
    /// Get statistics about file watching
    pub fn get_statistics(&self) -> WatchStatistics {
        let watched_paths = self.watched_paths.read().unwrap();
        let total_paths = watched_paths.len();
        let total_events: u64 = watched_paths.values().map(|p| p.event_count).sum();
        let directories = watched_paths.values().filter(|p| p.is_directory).count();
        let files = total_paths - directories;
        
        WatchStatistics {
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
                        }
                    
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
    /// Convert a notify event to our internal event type
    fn convert_notify_event(event: Event, filter: &EventFilter) -> Option<FileWatchEvent> {
        let timestamp = SystemTime::now();
        
        for path in &event.paths {
            if !filter.should_watch(path) {
                continue;
            match event.kind {
                EventKind::Create(_) => {
                    if path.is_dir() {
                        return Some(FileWatchEvent::DirectoryCreated {
                        });
                    } else {
                        return Some(FileWatchEvent::Created {
                        });
                    }
                }
                EventKind::Modify(_) => {
                    return Some(FileWatchEvent::Modified {
                    });
                }
                EventKind::Remove(_) => {
                    if path.is_dir() {
                        return Some(FileWatchEvent::DirectoryDeleted {
                        });
                    } else {
                        return Some(FileWatchEvent::Deleted {
                        });
                    }
                }
                _ => {
                    // Handle other event types as needed
                    continue;
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
    
    /// Number of directories being watched
    
    /// Number of files being watched
    
    /// Total number of events processed
    
    /// Whether the watcher is currently running
/// Builder for creating FileWatcher instances with custom configuration
pub struct FileWatcherBuilder {
impl FileWatcherBuilder {
    /// Create a new builder with default configuration
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Set the file patterns to watch
    pub fn watch_patterns(mut self, patterns: Vec<String>) -> Self {
        self.config.watch_patterns = patterns;
        self
    /// Set the file patterns to ignore
    pub fn ignore_patterns(mut self, patterns: Vec<String>) -> Self {
        self.config.ignore_patterns = patterns;
        self
    /// Set the debounce duration
    pub fn debounce_duration(mut self, duration: Duration) -> Self {
        self.config.debounce_duration = duration;
        self
    /// Set the maximum batch size
    pub fn max_batch_size(mut self, size: usize) -> Self {
        self.config.max_batch_size = size;
        self
    /// Set whether to watch recursively
    pub fn recursive(mut self, recursive: bool) -> Self {
        self.config.recursive = recursive;
        self
    /// Set whether to follow symlinks
    pub fn follow_symlinks(mut self, follow: bool) -> Self {
        self.config.follow_symlinks = follow;
        self
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

