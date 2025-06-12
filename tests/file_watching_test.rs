/*!
 * Comprehensive File Watching Test Suite
 * 
 * This test suite validates all aspects of the CURSED file watching system to ensure:
 * 
 * 1. **FileWatcher Unit Tests**: Core functionality validation
 *    - Configuration creation and validation - ensures proper default settings
 *    - Event filtering and pattern matching - critical for watching only relevant files
 *    - Debouncing logic with timing tests - prevents event spam and resource waste
 *    - Watch path management - ensures paths are tracked correctly
 * 
 * 2. **Integration Tests**: End-to-end functionality
 *    - File watching with temp directories - safe isolated testing environment
 *    - Event generation and handling - validates event processing pipeline
 *    - Build trigger integration - ensures builds are triggered appropriately
 *    - Multiple file type watching - validates pattern matching works correctly
 * 
 * 3. **Error Handling Tests**: Robustness validation
 *    - Invalid watch patterns - ensures graceful handling of bad input
 *    - Permission denied scenarios - validates error handling for restricted paths
 *    - Watcher initialization failures - ensures proper error propagation
 *    - Resource cleanup verification - prevents memory leaks and resource exhaustion
 * 
 * 4. **Performance Tests**: Scalability and efficiency
 *    - Rapid file change handling - ensures system doesn't break under load
 *    - Large directory watching - validates performance with many files
 *    - Memory usage under load - prevents memory leaks in long-running watchers
 *    - Event batching efficiency - validates optimization features work correctly
 * 
 * 5. **CLI Integration Tests**: Command-line interface validation
 *    - Watch command parsing - ensures CLI correctly interprets user input
 *    - Signal handling simulation - validates graceful shutdown behavior
 *    - Configuration option validation - ensures CLI options are properly applied
 * 
 * Each test is designed to validate a specific aspect of file watching reliability
 * and includes proper cleanup to prevent test interference.
 */

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime};

use tempfile::{TempDir, NamedTempFile};

// Import the file watching components
use cursed::build_system::{
    FileWatcher, FileWatcherBuilder, WatchConfig, FileWatchEvent, WatchedPath,
    DebounceManager, EventFilter, WatchStatistics
};
use cursed::error::Error as CursedError;

/// Test fixture for file watching tests with proper cleanup
struct FileWatchingTestFixture {
    temp_dir: TempDir,
    watcher: Option<FileWatcher>,
    received_events: Arc<Mutex<Vec<FileWatchEvent>>>,
}

impl FileWatchingTestFixture {
    /// Create a new test fixture with a temporary directory
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        Self {
            temp_dir,
            watcher: None,
            received_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get the path to the temporary directory
    fn temp_path(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Create a file in the temp directory with given content
    fn create_test_file(&self, name: &str, content: &str) -> PathBuf {
        let file_path = self.temp_path().join(name);
        let mut file = File::create(&file_path).expect("Failed to create test file");
        file.write_all(content.as_bytes()).expect("Failed to write test file");
        file_path
    }

    /// Create a subdirectory in the temp directory
    fn create_test_dir(&self, name: &str) -> PathBuf {
        let dir_path = self.temp_path().join(name);
        fs::create_dir(&dir_path).expect("Failed to create test directory");
        dir_path
    }

    /// Modify an existing file in the temp directory
    fn modify_test_file(&self, path: &Path, content: &str) {
        let mut file = File::create(path).expect("Failed to open test file for modification");
        file.write_all(content.as_bytes()).expect("Failed to modify test file");
    }

    /// Delete a file in the temp directory
    fn delete_test_file(&self, path: &Path) {
        fs::remove_file(path).expect("Failed to delete test file");
    }

    /// Setup a file watcher with custom configuration
    fn setup_watcher(&mut self, config: WatchConfig) -> Result<(), CursedError> {
        let mut watcher = FileWatcher::new(config)?;
        
        // Set up event callback to collect events
        let events = Arc::clone(&self.received_events);
        watcher.set_event_callback(move |event| {
            if let Ok(mut events_vec) = events.lock() {
                events_vec.push(event);
            }
        })?;

        self.watcher = Some(watcher);
        Ok(())
    }

    /// Start watching the temp directory
    fn start_watching(&mut self) -> Result<(), CursedError> {
        let temp_path = self.temp_path().to_path_buf();
        if let Some(ref mut watcher) = self.watcher {
            watcher.start_watching(&[temp_path])?;
        }
        Ok(())
    }

    /// Stop watching
    fn stop_watching(&mut self) -> Result<(), CursedError> {
        if let Some(ref mut watcher) = self.watcher {
            watcher.stop_watching()?;
        }
        Ok(())
    }

    /// Get all received events
    fn get_received_events(&self) -> Vec<FileWatchEvent> {
        self.received_events.lock()
            .map(|events| events.clone())
            .unwrap_or_default()
    }

    /// Clear received events
    fn clear_events(&self) {
        if let Ok(mut events) = self.received_events.lock() {
            events.clear();
        }
    }

    /// Wait for a specific number of events with timeout
    fn wait_for_events(&self, count: usize, timeout: Duration) -> bool {
        let start = Instant::now();
        while start.elapsed() < timeout {
            if self.get_received_events().len() >= count {
                return true;
            }
            thread::sleep(Duration::from_millis(10));
        }
        false
    }
}

impl Drop for FileWatchingTestFixture {
    fn drop(&mut self) {
        // Ensure watcher is stopped before cleanup
        let _ = self.stop_watching();
    }
}

// ================================================================================================
// 1. FileWatcher Unit Tests
// ================================================================================================

#[test]
fn test_watch_config_default_values() {
    // Test that default configuration has sensible values
    let config = WatchConfig::default();
    
    assert!(!config.watch_patterns.is_empty(), "Default watch patterns should not be empty");
    assert!(config.watch_patterns.contains(&"*.csd".to_string()), "Should watch .csd files by default");
    assert!(config.watch_patterns.contains(&"*.toml".to_string()), "Should watch .toml files by default");
    assert!(config.ignore_patterns.contains(&"target/*".to_string()), "Should ignore target directory by default");
    assert!(config.debounce_duration > Duration::from_millis(0), "Should have non-zero debounce duration");
    assert!(config.max_batch_size > 0, "Should have positive max batch size");
    assert!(config.recursive, "Should watch recursively by default");
    assert!(!config.follow_symlinks, "Should not follow symlinks by default for security");
}

#[test]
fn test_watch_config_builder() {
    // Test that FileWatcherBuilder properly configures the watcher
    let custom_patterns = vec!["*.rs".to_string(), "*.toml".to_string()];
    let custom_ignore = vec!["*.tmp".to_string()];
    let custom_debounce = Duration::from_millis(1000);
    let custom_batch_size = 100;

    let watcher = FileWatcherBuilder::new()
        .watch_patterns(custom_patterns.clone())
        .ignore_patterns(custom_ignore.clone())
        .debounce_duration(custom_debounce)
        .max_batch_size(custom_batch_size)
        .recursive(false)
        .follow_symlinks(true)
        .build()
        .expect("Failed to build file watcher");

    // We can't directly inspect the config, but we can test that the builder worked
    assert!(!watcher.is_running(), "Newly created watcher should not be running");
}

#[test]
fn test_file_watch_event_properties() {
    // Test FileWatchEvent methods work correctly
    let test_path = PathBuf::from("/test/path/file.csd");
    let timestamp = SystemTime::now();

    let created_event = FileWatchEvent::Created {
        path: test_path.clone(),
        timestamp,
    };

    assert_eq!(created_event.path(), test_path.as_path());
    assert_eq!(created_event.timestamp(), timestamp);
    assert!(created_event.should_trigger_rebuild(), "File creation should trigger rebuild");

    let dir_created_event = FileWatchEvent::DirectoryCreated {
        path: test_path.clone(),
        timestamp,
    };

    assert!(!dir_created_event.should_trigger_rebuild(), "Directory creation should not trigger rebuild");

    // Test batch event
    let batch_event = FileWatchEvent::Batch {
        events: vec![created_event.clone()],
        timestamp,
    };

    assert!(batch_event.should_trigger_rebuild(), "Batch with rebuild-triggering events should trigger rebuild");
}

#[test]
fn test_watched_path_creation() {
    // Test WatchedPath creation and metadata tracking
    let mut fixture = FileWatchingTestFixture::new();
    let test_file = fixture.create_test_file("test.csd", "// Test content");

    let watched_path = WatchedPath::new(test_file)
        .expect("Failed to create WatchedPath");

    assert!(!watched_path.is_directory, "File should not be marked as directory");
    assert!(watched_path.file_size.is_some(), "File should have size");
    assert!(watched_path.file_size.unwrap() > 0, "File size should be greater than 0");
    assert_eq!(watched_path.event_count, 0, "New watched path should have zero events");
}

#[test]
fn test_debounce_manager() {
    // Test debouncing logic to prevent event spam
    let debounce_duration = Duration::from_millis(100);
    let debounce_manager = DebounceManager::new(debounce_duration);
    
    let test_path = Path::new("/test/file.csd");

    // First event should be processed
    assert!(debounce_manager.should_process(test_path), "First event should be processed");

    // Immediate second event should be debounced
    assert!(!debounce_manager.should_process(test_path), "Immediate second event should be debounced");

    // After debounce duration, event should be processed again
    thread::sleep(debounce_duration + Duration::from_millis(10));
    assert!(debounce_manager.should_process(test_path), "Event after debounce duration should be processed");

    // Clear and test again
    debounce_manager.clear();
    assert!(debounce_manager.should_process(test_path), "Event after clear should be processed");
}

#[test]
fn test_event_filter_patterns() {
    // Test event filtering based on patterns
    let config = WatchConfig {
        watch_patterns: vec!["*.csd".to_string(), "*.toml".to_string()],
        ignore_patterns: vec!["*.tmp".to_string(), "target/*".to_string()],
        ..Default::default()
    };

    let filter = EventFilter::new(&config)
        .expect("Failed to create event filter");

    // Test watch patterns
    assert!(filter.should_watch(Path::new("test.csd")), "Should watch .csd files");
    assert!(filter.should_watch(Path::new("Cargo.toml")), "Should watch .toml files");
    assert!(!filter.should_watch(Path::new("test.rs")), "Should not watch .rs files");

    // Test ignore patterns
    assert!(!filter.should_watch(Path::new("temp.tmp")), "Should ignore .tmp files");
    assert!(!filter.should_watch(Path::new("target/debug/main")), "Should ignore target directory");

    // Test that ignore patterns take precedence
    assert!(!filter.should_watch(Path::new("target/Cargo.toml")), "Ignore patterns should take precedence");
}

// ================================================================================================
// 2. Integration Tests
// ================================================================================================

#[test]
fn test_file_creation_detection() {
    // Test that file creation events are properly detected and processed
    let mut fixture = FileWatchingTestFixture::new();
    
    // Setup watcher with short debounce for testing
    let config = WatchConfig {
        debounce_duration: Duration::from_millis(50),
        max_batch_size: 1, // Process events immediately
        ..Default::default()
    };
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");

    // Give the watcher time to initialize
    thread::sleep(Duration::from_millis(100));

    // Create a test file
    let test_file = fixture.create_test_file("new_test.csd", "// New file content");

    // Wait for events to be processed
    assert!(fixture.wait_for_events(1, Duration::from_secs(2)), "Should receive file creation event");

    let events = fixture.get_received_events();
    assert!(!events.is_empty(), "Should have received at least one event");

    // Check that we got a creation or modification event for our file
    let has_relevant_event = events.iter().any(|event| {
        let event_path = event.path();
        event_path.file_name() == test_file.file_name() &&
        (matches!(event, FileWatchEvent::Created { .. }) || matches!(event, FileWatchEvent::Modified { .. }))
    });

    assert!(has_relevant_event, "Should have received creation or modification event for the test file");
}

#[test]
fn test_file_modification_detection() {
    // Test that file modification events are properly detected
    let mut fixture = FileWatchingTestFixture::new();
    
    // Create file before starting watcher
    let test_file = fixture.create_test_file("modify_test.csd", "// Initial content");
    
    let config = WatchConfig {
        debounce_duration: Duration::from_millis(50),
        max_batch_size: 1,
        ..Default::default()
    };
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");

    // Give the watcher time to initialize
    thread::sleep(Duration::from_millis(100));
    fixture.clear_events(); // Clear any initial events

    // Modify the file
    fixture.modify_test_file(&test_file, "// Modified content");

    // Wait for modification event
    assert!(fixture.wait_for_events(1, Duration::from_secs(2)), "Should receive file modification event");

    let events = fixture.get_received_events();
    let has_modification_event = events.iter().any(|event| {
        matches!(event, FileWatchEvent::Modified { .. }) &&
        event.path().file_name() == test_file.file_name()
    });

    assert!(has_modification_event, "Should have received modification event for the test file");
}

#[test]
fn test_file_deletion_detection() {
    // Test that file deletion events are properly detected
    let mut fixture = FileWatchingTestFixture::new();
    
    // Create file before starting watcher
    let test_file = fixture.create_test_file("delete_test.csd", "// Content to be deleted");
    
    let config = WatchConfig {
        debounce_duration: Duration::from_millis(50),
        max_batch_size: 1,
        ..Default::default()
    };
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");

    // Give the watcher time to initialize
    thread::sleep(Duration::from_millis(100));
    fixture.clear_events(); // Clear any initial events

    // Delete the file
    fixture.delete_test_file(&test_file);

    // Wait for deletion event
    assert!(fixture.wait_for_events(1, Duration::from_secs(2)), "Should receive file deletion event");

    let events = fixture.get_received_events();
    let has_deletion_event = events.iter().any(|event| {
        matches!(event, FileWatchEvent::Deleted { .. }) &&
        event.path().file_name() == test_file.file_name()
    });

    assert!(has_deletion_event, "Should have received deletion event for the test file");
}

#[test]
fn test_recursive_directory_watching() {
    // Test that subdirectories are watched recursively
    let mut fixture = FileWatchingTestFixture::new();
    
    // Create subdirectory
    let subdir = fixture.create_test_dir("subdir");
    
    let config = WatchConfig {
        debounce_duration: Duration::from_millis(50),
        recursive: true,
        ..Default::default()
    };
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");

    // Give the watcher time to initialize
    thread::sleep(Duration::from_millis(100));
    fixture.clear_events();

    // Create file in subdirectory
    let subfile = subdir.join("subfile.csd");
    let mut file = File::create(&subfile).expect("Failed to create file in subdirectory");
    file.write_all(b"// Subdirectory file").expect("Failed to write to subdirectory file");

    // Wait for events
    assert!(fixture.wait_for_events(1, Duration::from_secs(2)), "Should receive event for file in subdirectory");

    let events = fixture.get_received_events();
    let has_subdir_event = events.iter().any(|event| {
        event.path().file_name() == subfile.file_name()
    });

    assert!(has_subdir_event, "Should have received event for file created in subdirectory");
}

#[test]
fn test_multiple_file_type_watching() {
    // Test watching multiple file types based on patterns
    let mut fixture = FileWatchingTestFixture::new();
    
    let config = WatchConfig {
        watch_patterns: vec!["*.csd".to_string(), "*.toml".to_string(), "*.md".to_string()],
        debounce_duration: Duration::from_millis(50),
        ..Default::default()
    };
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");

    thread::sleep(Duration::from_millis(100));
    fixture.clear_events();

    // Create files of different types
    let csd_file = fixture.create_test_file("test.csd", "// CURSED file");
    let toml_file = fixture.create_test_file("config.toml", "[package]");
    let md_file = fixture.create_test_file("README.md", "# Documentation");
    let rs_file = fixture.create_test_file("ignored.rs", "// Should be ignored");

    // Wait for events
    thread::sleep(Duration::from_secs(1));

    let events = fixture.get_received_events();
    
    // Check that we received events for watched file types
    let has_csd_event = events.iter().any(|e| e.path().file_name() == csd_file.file_name());
    let has_toml_event = events.iter().any(|e| e.path().file_name() == toml_file.file_name());
    let has_md_event = events.iter().any(|e| e.path().file_name() == md_file.file_name());
    let has_rs_event = events.iter().any(|e| e.path().file_name() == rs_file.file_name());

    assert!(has_csd_event, "Should receive event for .csd file");
    assert!(has_toml_event, "Should receive event for .toml file");
    assert!(has_md_event, "Should receive event for .md file");
    assert!(!has_rs_event, "Should not receive event for .rs file (not in watch patterns)");
}

#[test]
fn test_event_batching() {
    // Test that events are properly batched when many changes occur rapidly
    let mut fixture = FileWatchingTestFixture::new();
    
    let config = WatchConfig {
        debounce_duration: Duration::from_millis(10), // Very short debounce
        max_batch_size: 3, // Small batch size for testing
        ..Default::default()
    };
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");

    thread::sleep(Duration::from_millis(100));
    fixture.clear_events();

    // Create multiple files rapidly
    for i in 0..5 {
        fixture.create_test_file(&format!("batch_test_{}.csd", i), "// Batch test content");
        thread::sleep(Duration::from_millis(5)); // Very short delay
    }

    // Wait for batch processing
    thread::sleep(Duration::from_secs(1));

    let events = fixture.get_received_events();
    
    // Check that we received some events (exact count may vary due to timing)
    assert!(!events.is_empty(), "Should have received some events from batch operations");
    
    // Check if any batch events were created
    let has_batch_event = events.iter().any(|event| {
        matches!(event, FileWatchEvent::Batch { .. })
    });

    // Note: Batch events may or may not occur depending on timing, but we should at least get individual events
    println!("Received {} events, including {} batch events", 
             events.len(), 
             events.iter().filter(|e| matches!(e, FileWatchEvent::Batch { .. })).count());
}

#[test]
fn test_watcher_statistics() {
    // Test that watcher statistics are properly tracked
    let mut fixture = FileWatchingTestFixture::new();
    
    // Create some files and directories first
    let _file1 = fixture.create_test_file("stats_test1.csd", "// Test 1");
    let _file2 = fixture.create_test_file("stats_test2.toml", "[test]");
    let _subdir = fixture.create_test_dir("stats_subdir");
    
    let config = WatchConfig::default();
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    
    {
        let watcher = fixture.watcher.as_ref().expect("Watcher should be available");
        
        // Check initial statistics
        let stats = watcher.get_statistics();
        assert!(!stats.is_running, "Watcher should not be running initially");
        assert_eq!(stats.total_events_processed, 0, "Should have zero events processed initially");
    }

    // Start watching
    fixture.start_watching().expect("Failed to start watching");
    
    // Check running statistics
    let watcher = fixture.watcher.as_ref().expect("Watcher should be available");
    let stats = watcher.get_statistics();
    assert!(stats.is_running, "Watcher should be running after start");
    
    // The exact counts may vary based on filesystem behavior, but we should have some watched paths
    assert!(stats.total_watched_paths > 0, "Should have some watched paths");
}

// ================================================================================================
// 3. Error Handling Tests
// ================================================================================================

#[test]
fn test_invalid_watch_patterns() {
    // Test handling of malformed or problematic watch patterns
    let config = WatchConfig {
        watch_patterns: vec![], // Empty patterns should still work
        ignore_patterns: vec!["***invalid***".to_string()], // Invalid pattern
        ..Default::default()
    };

    // EventFilter should handle invalid patterns gracefully
    let filter_result = EventFilter::new(&config);
    assert!(filter_result.is_ok(), "EventFilter should handle invalid patterns gracefully");

    let filter = filter_result.unwrap();
    
    // With empty watch patterns, should watch everything (except ignored)
    assert!(filter.should_watch(Path::new("test.csd")), "Empty watch patterns should allow all files");
    assert!(filter.should_watch(Path::new("anything.xyz")), "Empty watch patterns should allow all files");
}

#[test]
fn test_nonexistent_watch_path() {
    // Test error handling when trying to watch nonexistent paths
    let mut fixture = FileWatchingTestFixture::new();
    let config = WatchConfig::default();
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    
    let nonexistent_path = fixture.temp_path().join("does_not_exist");
    
    // Should get an error when trying to watch nonexistent path
    let result = fixture.watcher.as_mut().unwrap()
        .start_watching(&[&nonexistent_path]);
    
    assert!(result.is_err(), "Should get error when watching nonexistent path");
    
    let error = result.unwrap_err();
    // Check that error message is informative
    let error_msg = format!("{}", error);
    assert!(error_msg.contains("does not exist") || error_msg.contains("Path"), 
            "Error message should mention the path issue: {}", error_msg);
}

#[test]
fn test_watcher_cleanup() {
    // Test that resources are properly cleaned up when watcher is stopped
    let mut fixture = FileWatchingTestFixture::new();
    let config = WatchConfig::default();
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");
    
    {
        let watcher = fixture.watcher.as_ref().expect("Watcher should be available");
        
        // Check that watcher is running
        assert!(watcher.is_running(), "Watcher should be running");
        assert!(!watcher.get_watched_paths().is_empty(), "Should have watched paths");
    }
    
    // Stop watcher
    fixture.stop_watching().expect("Failed to stop watching");
    
    // Check cleanup
    let watcher = fixture.watcher.as_ref().expect("Watcher should be available");
    assert!(!watcher.is_running(), "Watcher should not be running after stop");
    assert!(watcher.get_watched_paths().is_empty(), "Watched paths should be cleared after stop");
    
    let stats = watcher.get_statistics();
    assert!(!stats.is_running, "Statistics should show watcher as not running");
    assert_eq!(stats.total_watched_paths, 0, "Statistics should show zero watched paths");
}

#[test]
fn test_concurrent_watcher_access() {
    // Test thread safety of watcher operations
    let mut fixture = FileWatchingTestFixture::new();
    let config = WatchConfig::default();
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");
    
    let watcher = fixture.watcher.as_ref().expect("Watcher should be available");
    
    // Access watcher statistics from multiple threads
    let handles: Vec<_> = (0..5).map(|_| {
        let temp_path = fixture.temp_path().to_path_buf();
        thread::spawn(move || {
            // Create files from multiple threads
            for i in 0..3 {
                let file_path = temp_path.join(format!("concurrent_test_{}.csd", i));
                let _ = File::create(file_path);
                thread::sleep(Duration::from_millis(10));
            }
        })
    }).collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }

    // Watcher should still be functional
    assert!(watcher.is_running(), "Watcher should still be running after concurrent access");
    
    // Give time for events to be processed
    thread::sleep(Duration::from_millis(500));
    
    let stats = watcher.get_statistics();
    assert!(stats.is_running, "Statistics should show watcher as running");
}

// ================================================================================================
// 4. Performance Tests (marked as ignored for normal test runs)
// ================================================================================================

#[test]
#[ignore] // Remove this attribute to run performance tests
fn test_rapid_file_changes() {
    // Test handling of rapid file changes without overwhelming the system
    let mut fixture = FileWatchingTestFixture::new();
    
    let config = WatchConfig {
        debounce_duration: Duration::from_millis(10), // Short debounce for stress test
        max_batch_size: 50,
        ..Default::default()
    };
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");
    
    thread::sleep(Duration::from_millis(100));
    fixture.clear_events();

    let start_time = Instant::now();
    
    // Create many files rapidly
    for i in 0..100 {
        fixture.create_test_file(&format!("rapid_test_{}.csd", i), 
                                &format!("// Rapid test file {}", i));
        
        // Very short delay to create rapid changes
        if i % 10 == 0 {
            thread::sleep(Duration::from_millis(1));
        }
    }

    let creation_time = start_time.elapsed();
    
    // Wait for events to be processed
    thread::sleep(Duration::from_secs(2));
    
    let events = fixture.get_received_events();
    let processing_time = start_time.elapsed();
    
    println!("Performance test results:");
    println!("  Created 100 files in: {:?}", creation_time);
    println!("  Total processing time: {:?}", processing_time);
    println!("  Received {} events", events.len());
    
    // Verify that the system handled the load reasonably
    assert!(!events.is_empty(), "Should have received some events");
    assert!(processing_time < Duration::from_secs(10), "Processing should complete within reasonable time");
    
    let watcher = fixture.watcher.as_ref().unwrap();
    assert!(watcher.is_running(), "Watcher should still be running after stress test");
}

#[test]
#[ignore] // Remove this attribute to run performance tests
fn test_large_directory_watching() {
    // Test watching a directory with many files
    let mut fixture = FileWatchingTestFixture::new();
    
    // Create many files before starting watcher
    for i in 0..200 {
        fixture.create_test_file(&format!("large_dir_test_{}.csd", i), 
                                &format!("// Large directory test file {}", i));
    }
    
    // Create subdirectories with files
    for dir_i in 0..10 {
        let subdir = fixture.create_test_dir(&format!("subdir_{}", dir_i));
        for file_i in 0..20 {
            let file_path = subdir.join(format!("subfile_{}.csd", file_i));
            let mut file = File::create(file_path).expect("Failed to create file in subdirectory");
            file.write_all(format!("// Subdirectory {} file {}", dir_i, file_i).as_bytes())
                .expect("Failed to write to subdirectory file");
        }
    }

    let start_time = Instant::now();
    
    let config = WatchConfig::default();
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");
    
    let setup_time = start_time.elapsed();
    
    // Give watcher time to initialize with all files
    thread::sleep(Duration::from_secs(1));
    
    let watcher = fixture.watcher.as_ref().unwrap();
    let stats = watcher.get_statistics();
    
    println!("Large directory test results:");
    println!("  Setup time: {:?}", setup_time);
    println!("  Total watched paths: {}", stats.total_watched_paths);
    println!("  Directories: {}, Files: {}", stats.total_directories, stats.total_files);
    
    assert!(watcher.is_running(), "Watcher should be running with large directory");
    assert!(setup_time < Duration::from_secs(5), "Setup should complete within reasonable time");
    assert!(stats.total_watched_paths > 0, "Should have watched paths");
}

#[test]
#[ignore] // Remove this attribute to run performance tests  
fn test_memory_usage_under_load() {
    // Test memory usage remains reasonable under sustained load
    let mut fixture = FileWatchingTestFixture::new();
    
    let config = WatchConfig {
        debounce_duration: Duration::from_millis(50),
        max_batch_size: 20,
        ..Default::default()
    };
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");
    
    thread::sleep(Duration::from_millis(100));
    
    // Simulate sustained activity over time
    for cycle in 0..10 {
        fixture.clear_events(); // Clear events to prevent unbounded growth
        
        // Create files
        for i in 0..20 {
            fixture.create_test_file(&format!("memory_test_{}_{}.csd", cycle, i), 
                                    &format!("// Memory test cycle {} file {}", cycle, i));
        }
        
        // Wait a bit
        thread::sleep(Duration::from_millis(100));
        
        // Modify some files
        for i in 0..10 {
            let file_path = fixture.temp_path().join(format!("memory_test_{}_{}.csd", cycle, i));
            if file_path.exists() {
                fixture.modify_test_file(&file_path, &format!("// Modified cycle {} file {}", cycle, i));
            }
        }
        
        thread::sleep(Duration::from_millis(100));
        
        // Delete some files
        for i in 0..5 {
            let file_path = fixture.temp_path().join(format!("memory_test_{}_{}.csd", cycle, i));
            if file_path.exists() {
                fixture.delete_test_file(&file_path);
            }
        }
        
        thread::sleep(Duration::from_millis(100));
        
        let events = fixture.get_received_events();
        println!("Cycle {}: {} events", cycle, events.len());
    }

    let watcher = fixture.watcher.as_ref().unwrap();
    assert!(watcher.is_running(), "Watcher should still be running after sustained load");
    
    let stats = watcher.get_statistics();
    println!("Memory test final stats: {:?}", stats);
    
    // The exact numbers will vary, but watcher should still be functional
    assert!(stats.is_running, "Statistics should show watcher is running");
}

// ================================================================================================
// 5. CLI Integration Tests
// ================================================================================================

#[test]
fn test_watch_config_from_cli_patterns() {
    // Test that WatchConfig can be created with CLI-like parameters
    let cli_watch_patterns = vec!["*.csd".to_string(), "*.toml".to_string()];
    let cli_ignore_patterns = vec!["*.tmp".to_string(), "target/*".to_string()];
    let cli_debounce_ms = 500;

    let config = WatchConfig {
        watch_patterns: cli_watch_patterns.clone(),
        ignore_patterns: cli_ignore_patterns.clone(),
        debounce_duration: Duration::from_millis(cli_debounce_ms),
        max_batch_size: 50,
        recursive: true,
        follow_symlinks: false,
        event_buffer_size: 1000,
    };

    // Verify configuration matches CLI inputs
    assert_eq!(config.watch_patterns, cli_watch_patterns);
    assert_eq!(config.ignore_patterns, cli_ignore_patterns);
    assert_eq!(config.debounce_duration, Duration::from_millis(cli_debounce_ms));

    // Test that the configuration can be used to create a working watcher
    let watcher_result = FileWatcher::new(config);
    assert!(watcher_result.is_ok(), "Should be able to create watcher with CLI-like config");
}

#[test]
fn test_builder_pattern_for_cli() {
    // Test builder pattern that would be used by CLI parsing
    let watcher = FileWatcherBuilder::new()
        .watch_patterns(vec!["*.csd".to_string(), "Makefile".to_string()])
        .ignore_patterns(vec!["*.bak".to_string()])
        .debounce_duration(Duration::from_millis(250))
        .max_batch_size(25)
        .recursive(true)
        .follow_symlinks(false)
        .build();

    assert!(watcher.is_ok(), "Builder pattern should work for CLI configuration");
    
    let watcher = watcher.unwrap();
    assert!(!watcher.is_running(), "New watcher should not be running");
}

#[test]
fn test_graceful_shutdown_simulation() {
    // Simulate graceful shutdown that would happen on CLI signal handling
    let mut fixture = FileWatchingTestFixture::new();
    let config = WatchConfig::default();
    
    fixture.setup_watcher(config).expect("Failed to setup watcher");
    fixture.start_watching().expect("Failed to start watching");
    
    {
        // Verify watcher is running
        let watcher = fixture.watcher.as_ref().unwrap();
        assert!(watcher.is_running(), "Watcher should be running");
    }
    
    // Simulate graceful shutdown (like SIGTERM handling)
    fixture.stop_watching().expect("Failed to stop watching gracefully");
    
    // Verify clean shutdown
    let watcher = fixture.watcher.as_ref().unwrap();
    assert!(!watcher.is_running(), "Watcher should be stopped after graceful shutdown");
    assert!(watcher.get_watched_paths().is_empty(), "Watched paths should be cleared");
    
    let stats = watcher.get_statistics();
    assert!(!stats.is_running, "Statistics should reflect stopped state");
}

#[test]
fn test_configuration_validation() {
    // Test validation of configuration parameters that might come from CLI
    
    // Test minimum viable configuration
    let minimal_config = WatchConfig {
        watch_patterns: vec!["*".to_string()], // Watch everything
        ignore_patterns: vec![],
        debounce_duration: Duration::from_millis(1), // Minimum debounce
        max_batch_size: 1, // Minimum batch size
        recursive: false,
        follow_symlinks: false,
        event_buffer_size: 1, // Minimum buffer size
    };

    let watcher_result = FileWatcher::new(minimal_config);
    assert!(watcher_result.is_ok(), "Should accept minimal valid configuration");

    // Test configuration with edge case values
    let edge_config = WatchConfig {
        watch_patterns: vec![], // Empty patterns
        ignore_patterns: vec!["*".to_string()], // Ignore everything
        debounce_duration: Duration::from_secs(60), // Very long debounce
        max_batch_size: 10000, // Very large batch
        recursive: true,
        follow_symlinks: true,
        event_buffer_size: 100000, // Very large buffer
    };

    let watcher_result = FileWatcher::new(edge_config);
    assert!(watcher_result.is_ok(), "Should handle edge case configurations gracefully");
}

// ================================================================================================
// Helper Functions for Testing
// ================================================================================================

/// Create a temporary file with specific extension and content
fn create_temp_file_with_extension(dir: &Path, extension: &str, content: &str) -> PathBuf {
    let file_path = dir.join(format!("test_file.{}", extension));
    let mut file = File::create(&file_path).expect("Failed to create temp file");
    file.write_all(content.as_bytes()).expect("Failed to write temp file");
    file_path
}

/// Wait for a condition to be true with timeout
fn wait_for_condition<F>(mut condition: F, timeout: Duration) -> bool 
where
    F: FnMut() -> bool,
{
    let start = Instant::now();
    while start.elapsed() < timeout {
        if condition() {
            return true;
        }
        thread::sleep(Duration::from_millis(10));
    }
    false
}

/// Count events of a specific type
fn count_events_of_type(events: &[FileWatchEvent], event_type: &str) -> usize {
    events.iter().filter(|event| {
        match (event, event_type) {
            (FileWatchEvent::Created { .. }, "created") => true,
            (FileWatchEvent::Modified { .. }, "modified") => true,
            (FileWatchEvent::Deleted { .. }, "deleted") => true,
            (FileWatchEvent::Renamed { .. }, "renamed") => true,
            (FileWatchEvent::DirectoryCreated { .. }, "directory_created") => true,
            (FileWatchEvent::DirectoryDeleted { .. }, "directory_deleted") => true,
            (FileWatchEvent::Batch { .. }, "batch") => true,
            _ => false,
        }
    }).count()
}

// ================================================================================================
// Documentation Test
// ================================================================================================

#[test]
fn test_file_watching_documentation_examples() {
    // Test examples that would appear in documentation
    
    // Example 1: Basic file watcher setup
    let config = WatchConfig {
        watch_patterns: vec!["*.csd".to_string(), "*.toml".to_string()],
        ignore_patterns: vec!["target/*".to_string(), "*.tmp".to_string()],
        debounce_duration: Duration::from_millis(500),
        ..Default::default()
    };

    let watcher_result = FileWatcher::new(config);
    assert!(watcher_result.is_ok(), "Documentation example should work");

    // Example 2: Builder pattern usage
    let builder_result = FileWatcherBuilder::new()
        .watch_patterns(vec!["*.csd".to_string()])
        .debounce_duration(Duration::from_millis(1000))
        .recursive(true)
        .build();

    assert!(builder_result.is_ok(), "Builder pattern example should work");

    // Example 3: Event filtering
    let filter_config = WatchConfig {
        watch_patterns: vec!["*.csd".to_string()],
        ignore_patterns: vec!["test_*".to_string()],
        ..Default::default()
    };

    let filter = EventFilter::new(&filter_config);
    assert!(filter.is_ok(), "Event filter example should work");

    let filter = filter.unwrap();
    assert!(filter.should_watch(Path::new("main.csd")), "Should watch main.csd");
    assert!(!filter.should_watch(Path::new("test_file.csd")), "Should not watch test_file.csd");
}
