/// Request/response debugging and development utilities
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use notify;
use std::sync::{Arc, Mutex};

/// Request debugger for development
pub struct RequestDebugger {
    enabled: bool,
    log_headers: bool,
    log_body: bool,
    max_body_length: usize,
}

/// Live reload functionality for development
pub struct LiveReload {
    enabled: bool,
    port: u16,
}

/// Debug mode configuration
#[derive(Debug, Clone)]
pub enum DebugMode {
    Off,
    Basic,
    Verbose,
    Full,
}

/// Debug configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub mode: DebugMode,
    pub log_requests: bool,
    pub log_responses: bool,
    pub enable_live_reload: bool,
    pub live_reload_port: u16,
}

impl LiveReload {
    pub fn new() -> Self {
        Self {
            enabled: false,
            port: 35729,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            mode: DebugMode::Off,
            log_requests: false,
            log_responses: false,
            enable_live_reload: false,
            live_reload_port: 35729,
        }
    }
}

impl RequestDebugger {
    pub fn new() -> Self {
        Self {
            enabled: true,
            log_headers: true,
            log_body: true,
            max_body_length: 1024,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn log_request(&self, method: &str, path: &str, headers: &HashMap<String, String>, body: &[u8]) -> String {
        if !self.enabled {
            return String::new();
        }

        let mut debug_info = String::new();
        debug_info.push_str(&format!("=== REQUEST DEBUG ===\n"));
        debug_info.push_str(&format!("Method: {}\n", method));
        debug_info.push_str(&format!("Path: {}\n", path));

        if self.log_headers {
            debug_info.push_str("Headers:\n");
            for (key, value) in headers {
                debug_info.push_str(&format!("  {}: {}\n", key, value));
            }
        }

        if self.log_body && !body.is_empty() {
            debug_info.push_str("Body:\n");
            let body_str = if body.len() > self.max_body_length {
                format!("{}... (truncated)", String::from_utf8_lossy(&body[..self.max_body_length]))
            } else {
                String::from_utf8_lossy(body).to_string()
            };
            debug_info.push_str(&format!("  {}\n", body_str));
        }

        debug_info.push_str("==================\n");
        debug_info
    }
}

impl Default for RequestDebugger {
    fn default() -> Self {
        Self::new()
    }
}

/// Response debugger
pub struct ResponseDebugger {
    enabled: bool,
    log_headers: bool,
    log_body: bool,
    max_body_length: usize,
}

impl ResponseDebugger {
    pub fn new() -> Self {
        Self {
            enabled: true,
            log_headers: true,
            log_body: true,
            max_body_length: 1024,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn log_response(&self, status: u16, headers: &HashMap<String, String>, body: &[u8], duration: Duration) -> String {
        if !self.enabled {
            return String::new();
        }

        let mut debug_info = String::new();
        debug_info.push_str(&format!("=== RESPONSE DEBUG ===\n"));
        debug_info.push_str(&format!("Status: {}\n", status));
        debug_info.push_str(&format!("Duration: {:?}\n", duration));

        if self.log_headers {
            debug_info.push_str("Headers:\n");
            for (key, value) in headers {
                debug_info.push_str(&format!("  {}: {}\n", key, value));
            }
        }

        if self.log_body && !body.is_empty() {
            debug_info.push_str("Body:\n");
            let body_str = if body.len() > self.max_body_length {
                format!("{}... (truncated)", String::from_utf8_lossy(&body[..self.max_body_length]))
            } else {
                String::from_utf8_lossy(body).to_string()
            };
            debug_info.push_str(&format!("  {}\n", body_str));
        }

        debug_info.push_str("===================\n");
        debug_info
    }
}

impl Default for ResponseDebugger {
    fn default() -> Self {
        Self::new()
    }
}

/// Hot reload support for development
pub struct HotReloadWatcher {
    watched_paths: Vec<std::path::PathBuf>,
    enabled: bool,
    file_patterns: Vec<String>,
    debounce_duration: Duration,
    last_change_time: Option<SystemTime>,
    watcher: Option<notify::RecommendedWatcher>,
    event_receiver: Option<std::sync::mpsc::Receiver<notify::Result<notify::Event>>>,
    changed_files: std::sync::Arc<std::sync::Mutex<Vec<std::path::PathBuf>>>,
}

impl HotReloadWatcher {
    pub fn new() -> Self {
        Self {
            watched_paths: Vec::new(),
            enabled: false,
            file_patterns: vec![
                "*.csd".to_string(),
                "*.cursed".to_string(),
                "*.toml".to_string(),
            ],
            debounce_duration: Duration::from_millis(300),
            last_change_time: None,
            watcher: None,
            event_receiver: None,
            changed_files: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())),
        }
    }

    pub fn watch_path(&mut self, path: std::path::PathBuf) -> Result<(), Error> {
        if !path.exists() {
            return Err(crate::error::Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Watch path does not exist: {}", path.display())
            )));
        }

        self.watched_paths.push(path);
        
        // Restart watcher if enabled
        if self.enabled {
            self.stop_watching()?;
            self.start_watching()?;
        }
        
        Ok(())
    }

    pub fn with_patterns(mut self, patterns: Vec<String>) -> Self {
        self.file_patterns = patterns;
        self
    }

    pub fn with_debounce(mut self, duration: Duration) -> Self {
        self.debounce_duration = duration;
        self
    }

    pub fn enable(&mut self) -> Result<(), Error> {
        if !self.enabled {
            self.enabled = true;
            self.start_watching()?;
        }
        Ok(())
    }

    pub fn disable(&mut self) -> Result<(), Error> {
        if self.enabled {
            self.enabled = false;
            self.stop_watching()?;
        }
        Ok(())
    }

    fn start_watching(&mut self) -> Result<(), Error> {
        use notify::{Watcher, RecursiveMode, Config};
        
        let (tx, rx) = std::sync::mpsc::channel();
        let changed_files = self.changed_files.clone();
        let patterns = self.file_patterns.clone();
        
        let mut watcher = notify::RecommendedWatcher::new(
            move |res| {
                if let Ok(event) = &res {
                    if let Some(changed_path) = Self::should_handle_event(event, &patterns) {
                        if let Ok(mut files) = changed_files.lock() {
                            if !files.contains(&changed_path) {
                                files.push(changed_path);
                            }
                        }
                    }
                }
                let _ = tx.send(res);
            },
            Config::default(),
        ).map_err(|e| crate::error::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to create file watcher: {}", e)
        )))?;

        // Watch all configured paths
        for path in &self.watched_paths {
            let mode = if path.is_dir() {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            };
            
            watcher.watch(path, mode).map_err(|e| {
                crate::error::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to watch path {}: {}", path.display(), e)
                ))
            })?;
        }

        self.watcher = Some(watcher);
        self.event_receiver = Some(rx);
        
        Ok(())
    }

    fn stop_watching(&mut self) -> Result<(), Error> {
        if let Some(mut watcher) = self.watcher.take() {
            for path in &self.watched_paths {
                let _ = watcher.unwatch(path);
            }
        }
        self.event_receiver = None;
        
        // Clear pending changes
        if let Ok(mut files) = self.changed_files.lock() {
            files.clear();
        }
        
        Ok(())
    }

    fn should_handle_event(event: &notify::Event, patterns: &[String]) -> Option<std::path::PathBuf> {
        use notify::EventKind;

        // Only handle modify and create events
        match event.kind {
            EventKind::Modify(_) | EventKind::Create(_) => {},
            _ => return None,
        }

        // Check if any path matches our patterns
        for path in &event.paths {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                for pattern in patterns {
                    if Self::matches_pattern(file_name, pattern) {
                        return Some(path.clone());
                    }
                }
            }
        }

        None
    }

    fn matches_pattern(file_name: &str, pattern: &str) -> bool {
        if pattern.starts_with('*') && pattern.len() > 1 {
            let suffix = &pattern[1..];
            file_name.ends_with(suffix)
        } else if pattern.ends_with('*') && pattern.len() > 1 {
            let prefix = &pattern[..pattern.len()-1];
            file_name.starts_with(prefix)
        } else {
            file_name == pattern
        }
    }

    pub fn check_for_changes(&mut self) -> Vec<std::path::PathBuf> {
        if !self.enabled {
            return Vec::new();
        }

        // Process any pending events (non-blocking)
        if let Some(rx) = &self.event_receiver {
            while let Ok(_event) = rx.try_recv() {
                // Events are already processed in the callback
            }
        }

        // Apply debouncing
        let now = SystemTime::now();
        if let Some(last_change) = self.last_change_time {
            if now.duration_since(last_change).unwrap_or(Duration::ZERO) < self.debounce_duration {
                return Vec::new();
            }
        }

        // Get and clear changed files
        let changed_files = if let Ok(mut files) = self.changed_files.lock() {
            if !files.is_empty() {
                self.last_change_time = Some(now);
                let result = files.clone();
                files.clear();
                result
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        changed_files
    }

    pub fn get_watched_paths(&self) -> &[std::path::PathBuf] {
        &self.watched_paths
    }

    pub fn get_file_patterns(&self) -> &[String] {
        &self.file_patterns
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_debounce_duration(&self) -> Duration {
        self.debounce_duration
    }
}

impl Default for HotReloadWatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HotReloadWatcher {
    fn drop(&mut self) {
        let _ = self.disable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_request_debugger() {
        let debugger = RequestDebugger::new();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let debug_output = debugger.log_request("GET", "/api/test", &headers, b"test body");
        assert!(debug_output.contains("GET"));
        assert!(debug_output.contains("/api/test"));
        assert!(debug_output.contains("Content-Type"));
        assert!(debug_output.contains("test body"));
    }

    #[test]
    fn test_response_debugger() {
        let debugger = ResponseDebugger::new();
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        
        let debug_output = debugger.log_response(
            200, 
            &headers, 
            b"<html></html>", 
            Duration::from_millis(150)
        );
        assert!(debug_output.contains("200"));
        assert!(debug_output.contains("Content-Type"));
        assert!(debug_output.contains("<html></html>"));
        assert!(debug_output.contains("150ms") || debug_output.contains("0.15"));
    }

    #[test]
    fn test_hot_reload_watcher_creation() {
        let watcher = HotReloadWatcher::new();
        assert!(!watcher.is_enabled());
        assert_eq!(watcher.get_watched_paths().len(), 0);
        assert!(watcher.get_file_patterns().contains(&"*.csd".to_string()));
        assert_eq!(watcher.get_debounce_duration(), Duration::from_millis(300));
    }

    #[test]
    fn test_hot_reload_watcher_configuration() {
        let watcher = HotReloadWatcher::new()
            .with_patterns(vec!["*.rs".to_string(), "*.md".to_string()])
            .with_debounce(Duration::from_millis(500));
        
        assert_eq!(watcher.get_file_patterns(), &["*.rs", "*.md"]);
        assert_eq!(watcher.get_debounce_duration(), Duration::from_millis(500));
    }

    #[test]
    fn test_pattern_matching() {
        assert!(HotReloadWatcher::matches_pattern("test.csd", "*.csd"));
        assert!(HotReloadWatcher::matches_pattern("file.cursed", "*.cursed"));
        assert!(HotReloadWatcher::matches_pattern("config.toml", "*.toml"));
        assert!(HotReloadWatcher::matches_pattern("prefix_test", "prefix_*"));
        assert!(HotReloadWatcher::matches_pattern("exact_match", "exact_match"));
        
        assert!(!HotReloadWatcher::matches_pattern("test.rs", "*.csd"));
        assert!(!HotReloadWatcher::matches_pattern("test.csd", "*.rs"));
        assert!(!HotReloadWatcher::matches_pattern("wrong_prefix", "prefix_*"));
    }

    #[test]
    fn test_watch_nonexistent_path() {
        let mut watcher = HotReloadWatcher::new();
        let result = watcher.watch_path(std::path::PathBuf::from("/nonexistent/path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_watch_valid_path() -> Result<(), Error> {
        let temp_dir = TempDir::new()?;
        let mut watcher = HotReloadWatcher::new();
        
        let result = watcher.watch_path(temp_dir.path().to_path_buf());
        assert!(result.is_ok());
        assert_eq!(watcher.get_watched_paths().len(), 1);
        
        Ok(())
    }

    #[test]
    fn test_enable_disable_watcher() -> Result<(), Error> {
        let temp_dir = TempDir::new()?;
        let mut watcher = HotReloadWatcher::new();
        
        watcher.watch_path(temp_dir.path().to_path_buf())?;
        
        let result = watcher.enable();
        assert!(result.is_ok());
        assert!(watcher.is_enabled());
        
        let result = watcher.disable();
        assert!(result.is_ok());
        assert!(!watcher.is_enabled());
        
        Ok(())
    }

    #[test]
    fn test_debugger_disabled() {
        let debugger = RequestDebugger::new().enabled(false);
        let headers = HashMap::new();
        
        let debug_output = debugger.log_request("GET", "/test", &headers, b"");
        assert!(debug_output.is_empty());
    }

    #[test]
    fn test_body_truncation() {
        let debugger = RequestDebugger::new();
        let headers = HashMap::new();
        let long_body = vec![b'x'; 2000]; // Longer than default max_body_length (1024)
        
        let debug_output = debugger.log_request("POST", "/test", &headers, &long_body);
        assert!(debug_output.contains("truncated"));
    }
}
