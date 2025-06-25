use crate::error::CursedError;
/// Request/response debugging and development utilities
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use notify;
use std::sync::{Arc, Mutex};

/// Request debugger for development
pub struct RequestDebugger {
/// Live reload functionality for development
pub struct LiveReload {
/// Debug mode configuration
#[derive(Debug, Clone)]
pub enum DebugMode {
/// Debug configuration
#[derive(Debug, Clone)]
pub struct DebugConfig {
impl LiveReload {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
        }
    }
impl RequestDebugger {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    pub fn log_request(&self, method: &str, path: &str, headers: &HashMap<String, String>, body: &[u8]) -> String {
        if !self.enabled {
            return String::new();
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
            debug_info.push_str(&format!("  {}\n", body_str));
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
impl ResponseDebugger {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    pub fn log_response(&self, status: u16, headers: &HashMap<String, String>, body: &[u8], duration: Duration) -> String {
        if !self.enabled {
            return String::new();
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
            debug_info.push_str(&format!("  {}\n", body_str));
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
impl HotReloadWatcher {
    pub fn new() -> Self {
        Self {
            file_patterns: vec![
        }
    }

    pub fn watch_path(&mut self, path: std::path::PathBuf) -> crate::error::Result<()> {
        if !path.exists() {
            return Err(crate::error::CursedError::Io(std::io::Error::new(
                format!("Watch path does not exist: {}", path.display())
            )));
        self.watched_paths.push(path);
        
        // Restart watcher if enabled
        if self.enabled {
            self.stop_watching()?;
            self.start_watching()?;
        Ok(())
    pub fn with_patterns(mut self, patterns: Vec<String>) -> Self {
        self.file_patterns = patterns;
        self
    pub fn with_debounce(mut self, duration: Duration) -> Self {
        self.debounce_duration = duration;
        self
    pub fn enable(&mut self) -> crate::error::Result<()> {
        if !self.enabled {
            self.enabled = true;
            self.start_watching()?;
        }
        Ok(())
    pub fn disable(&mut self) -> crate::error::Result<()> {
        if self.enabled {
            self.enabled = false;
            self.stop_watching()?;
        }
        Ok(())
    fn start_watching(&mut self) -> crate::error::Result<()> {
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
        ).map_err(|e| crate::error::CursedError::Io(std::io::Error::new(
            format!("Failed to create file watcher: {}", e)
        )))?;

        // Watch all configured paths
        for path in &self.watched_paths {
            let mode = if path.is_dir() {
                RecursiveMode::Recursive
            } else {
                RecursiveMode::NonRecursive
            
            watcher.watch(path, mode).map_err(|e| {
                crate::error::CursedError::Io(std::io::Error::new(
                    format!("Failed to watch path {}: {}", path.display(), e)
                ))
            })?;
        self.watcher = Some(watcher);
        self.event_receiver = Some(rx);
        
        Ok(())
    fn stop_watching(&mut self) -> crate::error::Result<()> {
        if let Some(mut watcher) = self.watcher.take() {
            for path in &self.watched_paths {
                let _ = watcher.unwatch(path);
            }
        }
        self.event_receiver = None;
        
        // Clear pending changes
        if let Ok(mut files) = self.changed_files.lock() {
            files.clear();
        Ok(())
    fn should_handle_event(event: &notify::Event, patterns: &[String]) -> Option<std::path::PathBuf> {
        use notify::EventKind;

        // Only handle modify and create events
        match event.kind {
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

        changed_files
    pub fn get_watched_paths(&self) -> &[std::path::PathBuf] {
        &self.watched_paths
    pub fn get_file_patterns(&self) -> &[String] {
        &self.file_patterns
    pub fn is_enabled(&self) -> bool {
        self.enabled
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

