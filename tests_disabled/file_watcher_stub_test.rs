use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tempfile::TempDir;

use cursed::build_system::file_watcher::{
    FileWatcher, FileWatcherBuilder, FileWatchEvent, WatchConfig, EventFilter, DebounceManager
};

#[test]
fn test_watch_config_creation() {
    let config = WatchConfig::default();
    
    assert!(!config.watch_patterns.is_empty());
    assert!(!config.ignore_patterns.is_empty());
    assert_eq!(config.debounce_duration, Duration::from_millis(500));
    assert_eq!(config.max_batch_size, 50);
    assert!(config.recursive);
    assert!(!config.follow_symlinks);
}

#[test]
fn test_file_watch_event_properties() {
    let path = PathBuf::from("/test/file.csd");
    let timestamp = std::time::SystemTime::now();
    
    let event = FileWatchEvent::Created {
        path: path.clone(),
        timestamp,
    };
    
    assert_eq!(event.path(), &path);
    assert_eq!(event.timestamp(), timestamp);
    assert!(event.should_trigger_rebuild());
    
    let dir_event = FileWatchEvent::DirectoryCreated {
        path: path.clone(),
        timestamp,
    };
    
    assert!(!dir_event.should_trigger_rebuild());
}

#[test]
fn test_event_filter_creation() {
    let config = WatchConfig::default();
    let _filter = EventFilter::new(&config).unwrap();
    
    // Note: Stub implementation doesn't have pattern matching methods
    // This test just verifies the EventFilter can be created
}

#[test]
fn test_debounce_manager() {
    let _debouncer = DebounceManager::new(Duration::from_millis(100));
    
    // Note: Stub implementation doesn't have event processing methods
    // This test just verifies the DebounceManager can be created
}

#[test]
fn test_file_watcher_builder() {
    let watcher = FileWatcherBuilder::new()
        .watch_patterns(vec!["*.rs".to_string(), "*.toml".to_string()])
        .ignore_patterns(vec!["target/*".to_string()])
        .debounce_duration(Duration::from_millis(1000))
        .max_batch_size(20)
        .recursive(false)
        .follow_symlinks(true)
        .build()
        .unwrap();
    
    assert!(!watcher.is_running());
    
    let stats = watcher.get_statistics();
    assert_eq!(stats.total_watched_paths, 0);
    assert!(!stats.is_running);
}

#[test]
fn test_file_watcher_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    let mut watcher = FileWatcherBuilder::new()
        .debounce_duration(Duration::from_millis(10))
        .build()
        .unwrap();
    
    // Test initial state
    assert!(!watcher.is_running());
    assert_eq!(watcher.get_watched_paths().len(), 0);
    
    // Test event callback setup
    watcher.set_event_callback(move |_event| {
        // Stub callback - doesn't do anything
    }).unwrap();
    
    // Start watching temporary directory
    watcher.start_watching(&[&temp_path]).unwrap();
    assert!(watcher.is_running());
    
    let watched_paths = watcher.get_watched_paths();
    assert_eq!(watched_paths.len(), 1);
    assert!(watched_paths.contains_key(&temp_path));
    
    // Stop watching
    watcher.stop_watching().unwrap();
    assert!(!watcher.is_running());
}

#[test]
fn test_file_watcher_statistics() {
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path().to_path_buf();
    
    // Create some test files
    let test_file = temp_path.join("test.csd");
    fs::write(&test_file, "sus x = 42;").unwrap();
    
    let test_subdir = temp_path.join("subdir");
    fs::create_dir(&test_subdir).unwrap();
    
    let mut watcher = FileWatcherBuilder::new()
        .debounce_duration(Duration::from_millis(10))
        .build()
        .unwrap();
    
    watcher.start_watching(&[&temp_path]).unwrap();
    
    let stats = watcher.get_statistics();
    assert!(stats.is_running);
    assert!(stats.total_watched_paths > 0);
    
    watcher.stop_watching().unwrap();
    
    let final_stats = watcher.get_statistics();
    assert!(!final_stats.is_running);
}

#[test]
fn test_batch_events() {
    let events = vec![
        FileWatchEvent::Created {
            path: PathBuf::from("file1.csd"),
            timestamp: std::time::SystemTime::now(),
        },
        FileWatchEvent::Modified {
            path: PathBuf::from("file2.csd"),
            timestamp: std::time::SystemTime::now(),
        },
    ];
    
    let batch_event = FileWatchEvent::Batch {
        events: events.clone(),
        timestamp: std::time::SystemTime::now(),
    };
    
    assert!(batch_event.should_trigger_rebuild());
    
    if let FileWatchEvent::Batch { events: batch_events, .. } = batch_event {
        assert_eq!(batch_events.len(), 2);
    } else {
        panic!("Expected batch event");
    }
}

#[test]
fn test_watched_path_creation() {
    let temp_dir = TempDir::new().unwrap();
    let temp_file = temp_dir.path().join("test.csd");
    fs::write(&temp_file, "test content").unwrap();
    
    let watched_path = cursed::build_system::file_watcher::WatchedPath::new(temp_file.clone()).unwrap();
    
    assert_eq!(watched_path.path, temp_file);
    assert!(!watched_path.is_directory);
    assert_eq!(watched_path.event_count, 0);
    assert!(watched_path.file_size.is_some());
    assert!(watched_path.file_size.unwrap() > 0);
}

#[test]
fn test_file_watcher_with_nonexistent_path() {
    let mut watcher = FileWatcherBuilder::new().build().unwrap();
    
    let nonexistent_path = PathBuf::from("/definitely/does/not/exist");
    let result = watcher.start_watching(&[&nonexistent_path]);
    
    assert!(result.is_err());
}

#[test]
fn test_multiple_path_watching() {
    let temp_dir1 = TempDir::new().unwrap();
    let temp_dir2 = TempDir::new().unwrap();
    
    let mut watcher = FileWatcherBuilder::new()
        .debounce_duration(Duration::from_millis(10))
        .build()
        .unwrap();
    
    let paths = vec![temp_dir1.path(), temp_dir2.path()];
    watcher.start_watching(&paths).unwrap();
    
    let watched_paths = watcher.get_watched_paths();
    assert_eq!(watched_paths.len(), 2);
    
    watcher.stop_watching().unwrap();
}
