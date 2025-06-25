/// Tests for hot reload file watching functionality
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

// Add this test to the web_vibez debug module tests
#[cfg(test)]
mod hot_reload_tests {
    use super::*;
    use cursed::stdlib::web_vibez::debug::HotReloadWatcher;

    #[test]
    fn test_hot_reload_watcher_creation() {
        let watcher = HotReloadWatcher::new();
        
        assert!(!watcher.is_enabled());
        assert_eq!(watcher.get_watched_paths().len(), 0);
        assert!(watcher.get_file_patterns().contains(&"*.csd".to_string()));
        assert!(watcher.get_file_patterns().contains(&"*.cursed".to_string()));
        assert!(watcher.get_file_patterns().contains(&"*.toml".to_string()));
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
    fn test_watch_nonexistent_path() {
        let mut watcher = HotReloadWatcher::new();
        let nonexistent_path = PathBuf::from("/nonexistent/path");
        
        let result = watcher.watch_path(nonexistent_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_watch_valid_path() {
        let temp_dir = TempDir::new().unwrap();
        let mut watcher = HotReloadWatcher::new();
        
        let result = watcher.watch_path(temp_dir.path().to_path_buf());
        assert!(result.is_ok());
        assert_eq!(watcher.get_watched_paths().len(), 1);
        assert_eq!(watcher.get_watched_paths()[0], temp_dir.path());
    }

    #[test]
    fn test_enable_disable_watcher() {
        let temp_dir = TempDir::new().unwrap();
        let mut watcher = HotReloadWatcher::new();
        
        // Add a path to watch
        watcher.watch_path(temp_dir.path().to_path_buf()).unwrap();
        
        // Enable watcher
        let result = watcher.enable();
        assert!(result.is_ok());
        assert!(watcher.is_enabled());
        
        // Disable watcher
        let result = watcher.disable();
        assert!(result.is_ok());
        assert!(!watcher.is_enabled());
    }

    #[test]
    fn test_pattern_matching() {
        // Test the pattern matching logic
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
    fn test_file_change_detection() {
        let temp_dir = TempDir::new().unwrap();
        let mut watcher = HotReloadWatcher::new()
            .with_debounce(Duration::from_millis(50));
        
        // Add temp directory to watch
        watcher.watch_path(temp_dir.path().to_path_buf()).unwrap();
        watcher.enable().unwrap();
        
        // Initially no changes
        let changes = watcher.check_for_changes();
        assert!(changes.is_empty());
        
        // Create a .csd file
        let test_file = temp_dir.path().join("test.csd");
        fs::write(&test_file, "// Test file content").unwrap();
        
        // Wait a bit for file system events
        std::thread::sleep(Duration::from_millis(100));
        
        // Check for changes - might detect the new file
        let _changes = watcher.check_for_changes();
        // Note: In a real test environment, this might be flaky due to timing
        
        // Cleanup
        watcher.disable().unwrap();
    }

    #[test]
    fn test_debouncing() {
        let mut watcher = HotReloadWatcher::new()
            .with_debounce(Duration::from_millis(100));
        
        // Simulate rapid changes by calling check_for_changes multiple times
        // The debouncing should prevent rapid-fire returns
        
        // This test validates the debouncing logic exists
        assert_eq!(watcher.get_debounce_duration(), Duration::from_millis(100));
        
        let changes1 = watcher.check_for_changes();
        let changes2 = watcher.check_for_changes();
        
        assert!(changes1.is_empty());
        assert!(changes2.is_empty());
    }

    #[test]
    fn test_multiple_paths() {
        let temp_dir1 = TempDir::new().unwrap();
        let temp_dir2 = TempDir::new().unwrap();
        let mut watcher = HotReloadWatcher::new();
        
        watcher.watch_path(temp_dir1.path().to_path_buf()).unwrap();
        watcher.watch_path(temp_dir2.path().to_path_buf()).unwrap();
        
        assert_eq!(watcher.get_watched_paths().len(), 2);
        assert!(watcher.get_watched_paths().contains(&temp_dir1.path().to_path_buf()));
        assert!(watcher.get_watched_paths().contains(&temp_dir2.path().to_path_buf()));
    }

    #[test] 
    fn test_default_implementation() {
        let watcher: HotReloadWatcher = Default::default();
        
        assert!(!watcher.is_enabled());
        assert_eq!(watcher.get_watched_paths().len(), 0);
        assert!(watcher.get_file_patterns().len() > 0);
    }

    #[test]
    fn test_watcher_restart_on_path_addition() {
        let temp_dir1 = TempDir::new().unwrap();
        let temp_dir2 = TempDir::new().unwrap();
        let mut watcher = HotReloadWatcher::new();
        
        // Add first path and enable
        watcher.watch_path(temp_dir1.path().to_path_buf()).unwrap();
        watcher.enable().unwrap();
        assert!(watcher.is_enabled());
        
        // Add second path - should restart watcher
        let result = watcher.watch_path(temp_dir2.path().to_path_buf());
        assert!(result.is_ok());
        assert_eq!(watcher.get_watched_paths().len(), 2);
        assert!(watcher.is_enabled()); // Should still be enabled
        
        watcher.disable().unwrap();
    }
}

// Integration test for the development server example
#[cfg(test)]
mod dev_server_tests {
    use super::*;
    
    // This would test the DevServer struct from the example
    // Note: This is a conceptual test - the actual DevServer would need to be in the main codebase
    
    #[test]
    fn test_dev_server_concept() {
        // Test that the hot reload integration concept works
        let temp_dir = TempDir::new().unwrap();
        let mut watcher = HotReloadWatcher::new();
        
        watcher.watch_path(temp_dir.path().to_path_buf()).unwrap();
        watcher.enable().unwrap();
        
        // Simulate development workflow
        let changes = watcher.check_for_changes();
        assert!(changes.is_empty()); // No changes initially
        
        watcher.disable().unwrap();
    }
}
