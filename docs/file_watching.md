# File Watching System

The CURSED file watching system provides real-time file system monitoring capabilities for development workflows, build automation, and test integration. Built on the robust `notify` crate, it offers cross-platform file monitoring with intelligent debouncing, pattern matching, and event batching.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [CLI Usage](#cli-usage)
- [Programming API](#programming-api)
- [Integration Guide](#integration-guide)
- [Performance Considerations](#performance-considerations)
- [Common Use Cases](#common-use-cases)
- [Troubleshooting](#troubleshooting)
- [Advanced Topics](#advanced-topics)

## Overview

### Why File Watching?

File watching is essential for modern development workflows, providing:

- **Instant Feedback**: Automatically trigger builds, tests, or other tasks when files change
- **Developer Productivity**: Eliminate manual build steps and reduce context switching
- **Continuous Integration**: Real-time validation of code changes during development
- **Hot Reload**: Dynamic updates for web applications and development servers
- **Resource Efficiency**: Monitor only relevant files with intelligent filtering

### Key Features

- **Cross-Platform**: Works on Linux, macOS, and Windows
- **Intelligent Debouncing**: Prevents rapid-fire events from overwhelming the system
- **Pattern Matching**: Watch specific file types and ignore irrelevant changes
- **Event Batching**: Groups related events for efficient processing
- **Recursive Monitoring**: Monitor entire directory trees
- **Statistics & Monitoring**: Track watching performance and event statistics
- **Thread Safety**: Safe for concurrent use in multi-threaded applications

## Quick Start

### Basic File Watching

```bash
# Watch current directory for CURSED files and auto-rebuild
cursed build --watch

# Watch specific patterns
cursed build --watch --patterns "*.csd,*.toml,Makefile"

# Watch with custom debounce time
cursed build --watch --debounce 1000  # 1 second
```

### Programmatic Usage

```rust
use cursed::build_system::{FileWatcher, WatchConfig};
use std::time::Duration;

// Create a file watcher
let config = WatchConfig {
    watch_patterns: vec!["*.csd".to_string(), "*.toml".to_string()],
    ignore_patterns: vec!["target/*".to_string(), "*.tmp".to_string()],
    debounce_duration: Duration::from_millis(500),
    ..Default::default()
};

let mut watcher = FileWatcher::new(config)?;

// Set up event callback
watcher.set_event_callback(|event| {
    println!("File changed: {:?}", event.path());
    // Trigger your build process here
})?;

// Start watching
watcher.start_watching(&["./src", "./examples"])?;
```

## Configuration

### Watch Configuration Structure

```rust
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
```

### Default Configuration

```rust
WatchConfig {
    watch_patterns: vec![
        "*.csd".to_string(),      // CURSED source files
        "*.toml".to_string(),     // Configuration files
        "*.md".to_string(),       // Documentation
        "Makefile".to_string(),   // Build files
    ],
    ignore_patterns: vec![
        "*.tmp".to_string(),      // Temporary files
        "*.bak".to_string(),      // Backup files
        "target/*".to_string(),   // Build artifacts
        ".git/*".to_string(),     // Git files
        ".devenv/*".to_string(),  // Development environment
        "coverage/*".to_string(), // Coverage reports
    ],
    debounce_duration: Duration::from_millis(500),
    max_batch_size: 50,
    recursive: true,
    follow_symlinks: false,
    event_buffer_size: 1000,
}
```

### Configuration File (TOML)

```toml
# watch_config_example.toml
[watch]
# File patterns to monitor
watch_patterns = [
    "*.csd",
    "*.toml", 
    "*.md",
    "Makefile",
    "src/**/*.rs"  # For mixed projects
]

# Patterns to ignore
ignore_patterns = [
    "*.tmp",
    "*.bak", 
    "target/*",
    ".git/*",
    ".devenv/*",
    "coverage/*",
    "*.log",
    "node_modules/*"
]

# Debounce settings (milliseconds)
debounce_duration = 500

# Event batching
max_batch_size = 50
event_buffer_size = 1000

# Directory watching options
recursive = true
follow_symlinks = false
```

## CLI Usage

### Build System Integration

```bash
# Basic watch mode
cursed build --watch

# Watch specific directories
cursed build --watch --watch-dirs "src,examples,tests"

# Custom patterns
cursed build --watch --patterns "*.csd,*.toml,*.yaml"

# Ignore additional patterns
cursed build --watch --ignore "*.backup,logs/*"

# Custom debounce time (milliseconds)
cursed build --watch --debounce 1000

# Watch with specific build profile
cursed build --watch --profile release

# Watch with verbose output
cursed build --watch --verbose
```

### Test Integration

```bash
# Watch and run tests automatically
cursed test --watch

# Watch specific test files
cursed test --watch --patterns "tests/**/*.rs"

# Watch with coverage
cursed test --watch --coverage
```

### Linting Integration

```bash
# Watch and lint automatically
cursed lint --watch

# Watch and format code
cursed fmt --watch
```

### Package Manager Integration

```bash
# Watch for dependency changes
cursed package watch

# Watch and auto-update
cursed package watch --auto-update
```

## Programming API

### Basic File Watcher Usage

```rust
use cursed::build_system::{FileWatcher, WatchConfig, FileWatchEvent};
use std::time::Duration;

fn setup_file_watcher() -> Result<FileWatcher, Box<dyn std::error::Error>> {
    // Create configuration
    let config = WatchConfig {
        watch_patterns: vec!["*.csd".to_string()],
        debounce_duration: Duration::from_millis(300),
        ..Default::default()
    };
    
    // Create watcher
    let mut watcher = FileWatcher::new(config)?;
    
    // Set event callback
    watcher.set_event_callback(move |event| {
        match event {
            FileWatchEvent::Created { path, .. } => {
                println!("Created: {}", path.display());
                trigger_build();
            }
            FileWatchEvent::Modified { path, .. } => {
                println!("Modified: {}", path.display());
                trigger_rebuild();
            }
            FileWatchEvent::Deleted { path, .. } => {
                println!("Deleted: {}", path.display());
                trigger_cleanup();
            }
            FileWatchEvent::Batch { events, .. } => {
                println!("Batch of {} events", events.len());
                trigger_batch_build();
            }
            _ => {}
        }
    })?;
    
    Ok(watcher)
}

fn trigger_build() {
    // Your build logic here
}

fn trigger_rebuild() {
    // Your rebuild logic here
}

fn trigger_cleanup() {
    // Your cleanup logic here
}

fn trigger_batch_build() {
    // Your batch build logic here
}
```

### Builder Pattern

```rust
use cursed::build_system::FileWatcherBuilder;
use std::time::Duration;

let watcher = FileWatcherBuilder::new()
    .watch_patterns(vec!["*.csd".to_string(), "*.toml".to_string()])
    .ignore_patterns(vec!["target/*".to_string()])
    .debounce_duration(Duration::from_millis(300))
    .max_batch_size(25)
    .recursive(true)
    .build()?;
```

### Event Filtering

```rust
use cursed::build_system::{EventFilter, WatchConfig};

let config = WatchConfig::default();
let filter = EventFilter::new(&config)?;

// Check if a path should be watched
if filter.should_watch(&path) {
    println!("Watching: {}", path.display());
}
```

### Statistics and Monitoring

```rust
// Get watching statistics
let stats = watcher.get_statistics();
println!("Watching {} paths", stats.total_watched_paths);
println!("Processed {} events", stats.total_events_processed);
println!("Running: {}", stats.is_running);

// Get detailed path information
let watched_paths = watcher.get_watched_paths();
for (path, info) in watched_paths {
    println!("{}: {} events", path.display(), info.event_count);
}
```

## Integration Guide

### Build System Integration

The file watching system integrates seamlessly with the CURSED build system:

```rust
use cursed::build_system::BuildOrchestrator;

let mut orchestrator = BuildOrchestrator::new(work_dir, profile)?;

// Start watching with automatic builds
orchestrator.watch_and_build().await?;
```

### Web Framework Integration

For `web_vibez` applications:

```rust
use cursed::stdlib::web_vibez::Server;

let mut server = Server::new(config)?;

// Enable hot reload for templates and static files
server.enable_hot_reload()?;
server.start().await?;
```

### Testing Framework Integration

```rust
use cursed::testing::TestRunner;

let mut runner = TestRunner::new()?;

// Watch for test file changes
runner.watch_mode(true)?;
runner.run_continuous().await?;
```

### Custom Integration

```rust
struct CustomWatcher {
    file_watcher: FileWatcher,
    build_queue: Arc<Mutex<VecDeque<PathBuf>>>,
}

impl CustomWatcher {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut watcher = FileWatcher::new(WatchConfig::default())?;
        let build_queue = Arc::new(Mutex::new(VecDeque::new()));
        let queue_clone = Arc::clone(&build_queue);
        
        watcher.set_event_callback(move |event| {
            if event.should_trigger_rebuild() {
                if let Ok(mut queue) = queue_clone.lock() {
                    queue.push_back(event.path().to_path_buf());
                }
            }
        })?;
        
        Ok(Self {
            file_watcher: watcher,
            build_queue,
        })
    }
    
    pub async fn process_build_queue(&self) {
        while let Some(path) = self.build_queue.lock().unwrap().pop_front() {
            self.process_file_change(&path).await;
        }
    }
    
    async fn process_file_change(&self, path: &Path) {
        // Custom processing logic
    }
}
```

## Performance Considerations

### Debouncing Strategy

```rust
// For rapid development (quick feedback)
debounce_duration: Duration::from_millis(100)

// For balanced performance (recommended)
debounce_duration: Duration::from_millis(500)

// For resource-constrained environments
debounce_duration: Duration::from_millis(1000)
```

### Large Project Optimization

```rust
let config = WatchConfig {
    // Watch only essential file types
    watch_patterns: vec!["*.csd".to_string()],
    
    // Aggressive ignore patterns
    ignore_patterns: vec![
        "target/*".to_string(),
        ".git/*".to_string(),
        "node_modules/*".to_string(),
        "*.log".to_string(),
        "coverage/*".to_string(),
        "docs/generated/*".to_string(),
    ],
    
    // Larger batch sizes for efficiency
    max_batch_size: 100,
    
    // Longer debounce for stability
    debounce_duration: Duration::from_millis(1000),
    
    ..Default::default()
};
```

### Memory Usage Optimization

```rust
// Smaller event buffer for memory-constrained environments
event_buffer_size: 100,

// Process events in smaller batches
max_batch_size: 10,
```

### CPU Usage Optimization

```rust
// Reduce event frequency
debounce_duration: Duration::from_millis(1000),

// Limit recursive watching for large directories
recursive: false,  // Watch only specific directories

// Use more specific patterns
watch_patterns: vec!["src/**/*.csd".to_string()],
```

## Common Use Cases

### 1. Development Workflow with Auto-Rebuild

```bash
# Terminal 1: Start file watcher for automatic builds
cursed build --watch --verbose

# Terminal 2: Edit your code
vim src/main.csd

# Builds trigger automatically when you save
```

### 2. Test-Driven Development

```bash
# Watch tests and source files
cursed test --watch --patterns "src/**/*.csd,tests/**/*.rs"
```

### 3. Configuration File Monitoring

```rust
let config = WatchConfig {
    watch_patterns: vec![
        "*.toml".to_string(),
        "*.yaml".to_string(), 
        "*.json".to_string(),
    ],
    debounce_duration: Duration::from_millis(100), // Quick response
    ..Default::default()
};

watcher.set_event_callback(|event| {
    if event.path().extension().and_then(|s| s.to_str()) == Some("toml") {
        reload_configuration();
    }
});
```

### 4. Documentation Generation

```rust
let config = WatchConfig {
    watch_patterns: vec![
        "docs/**/*.md".to_string(),
        "src/**/*.csd".to_string(), // For docstring changes
    ],
    ..Default::default()
};

watcher.set_event_callback(|event| {
    regenerate_documentation();
    refresh_documentation_server();
});
```

### 5. Asset Processing

```rust
let config = WatchConfig {
    watch_patterns: vec![
        "assets/**/*.scss".to_string(),
        "assets/**/*.js".to_string(),
        "assets/**/*.png".to_string(),
    ],
    ..Default::default()
};

watcher.set_event_callback(|event| {
    let path = event.path();
    match path.extension().and_then(|s| s.to_str()) {
        Some("scss") => compile_scss(path),
        Some("js") => minify_javascript(path),
        Some("png") => optimize_image(path),
        _ => {}
    }
});
```

### 6. Large Project Optimization

```toml
# Configuration for projects with 10k+ files
[watch]
watch_patterns = [
    "src/**/*.csd",           # Only source files
    "Cargo.toml",             # Key configuration files
    "CursedPackage.toml",
]

ignore_patterns = [
    "target/*",               # Build artifacts
    ".git/*",                 # Git files  
    "node_modules/*",         # Dependencies
    "coverage/*",             # Generated reports
    "docs/generated/*",       # Generated docs
    "*.log",                  # Log files
    "*.tmp",                  # Temporary files
    ".devenv/*",              # Development environment
    "test_results/*",         # Test outputs
]

debounce_duration = 1000      # 1 second for stability
max_batch_size = 200          # Large batches for efficiency
event_buffer_size = 5000      # Large buffer for bursts
```

## Troubleshooting

### Common Issues

#### 1. High CPU Usage

**Symptoms**: File watcher consuming excessive CPU
**Solutions**:
```rust
// Increase debounce duration
debounce_duration: Duration::from_millis(1000),

// Add more ignore patterns
ignore_patterns: vec![
    "target/*".to_string(),
    ".git/*".to_string(),
    "node_modules/*".to_string(),
    "*.log".to_string(),
],

// Reduce batch size
max_batch_size: 10,
```

#### 2. Events Not Triggering

**Symptoms**: File changes not detected
**Debugging**:
```rust
// Enable debug logging
use tracing::{debug, info};

watcher.set_event_callback(|event| {
    debug!("Event received: {:?}", event);
    info!("Path: {}, should rebuild: {}", 
          event.path().display(), 
          event.should_trigger_rebuild());
});

// Check patterns
let filter = EventFilter::new(&config)?;
if !filter.should_watch(&problematic_path) {
    println!("Path {} is being ignored", problematic_path.display());
}
```

#### 3. Memory Leaks

**Symptoms**: Increasing memory usage over time
**Solutions**:
```rust
// Smaller event buffer
event_buffer_size: 100,

// Regular cleanup of watched paths
if watcher.get_watched_paths().len() > 1000 {
    watcher.stop_watching()?;
    watcher.start_watching(&paths)?;
}
```

#### 4. Symlink Issues

**Symptoms**: Symlinked files/directories not watched
**Solutions**:
```rust
// Enable symlink following (use with caution)
follow_symlinks: true,

// Or watch the actual target paths
let real_path = fs::read_link(&symlink_path)?;
watcher.start_watching(&[real_path])?;
```

### Platform-Specific Issues

#### Linux (inotify)

```bash
# Check inotify limits
cat /proc/sys/fs/inotify/max_user_watches

# Increase if needed
echo 524288 | sudo tee /proc/sys/fs/inotify/max_user_watches
```

#### macOS (FSEvents)

```rust
// FSEvents can have delays; use shorter debounce
debounce_duration: Duration::from_millis(100),
```

#### Windows

```rust
// Windows can be slower; use longer debounce
debounce_duration: Duration::from_millis(1000),
```

### Debugging Commands

```bash
# Test file watcher with verbose output
cursed build --watch --verbose

# Check what patterns are being used
cursed build --watch --dry-run

# Test specific patterns
cursed build --watch --patterns "*.csd" --verbose
```

## Advanced Topics

### Custom Event Filters

```rust
struct CustomEventFilter {
    inner: EventFilter,
    custom_rules: Vec<Box<dyn Fn(&Path) -> bool>>,
}

impl CustomEventFilter {
    pub fn new(config: &WatchConfig) -> Self {
        Self {
            inner: EventFilter::new(config).unwrap(),
            custom_rules: vec![
                // Only watch files larger than 1KB
                Box::new(|path| {
                    if let Ok(metadata) = fs::metadata(path) {
                        metadata.len() > 1024
                    } else {
                        false
                    }
                }),
                // Skip files with recent backup
                Box::new(|path| {
                    let backup_path = path.with_extension(
                        format!("{}.bak", path.extension().unwrap_or_default().to_string_lossy())
                    );
                    !backup_path.exists()
                }),
            ],
        }
    }
    
    pub fn should_watch(&self, path: &Path) -> bool {
        // Apply base filter first
        if !self.inner.should_watch(path) {
            return false;
        }
        
        // Apply custom rules
        self.custom_rules.iter().all(|rule| rule(path))
    }
}
```

### Multi-Watcher Setup

```rust
struct MultiWatcher {
    source_watcher: FileWatcher,
    config_watcher: FileWatcher,
    test_watcher: FileWatcher,
}

impl MultiWatcher {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Source file watcher (quick response)
        let source_config = WatchConfig {
            watch_patterns: vec!["src/**/*.csd".to_string()],
            debounce_duration: Duration::from_millis(100),
            ..Default::default()
        };
        let mut source_watcher = FileWatcher::new(source_config)?;
        source_watcher.set_event_callback(|_| quick_type_check())?;
        
        // Configuration watcher (immediate response)
        let config_config = WatchConfig {
            watch_patterns: vec!["*.toml".to_string(), "*.yaml".to_string()],
            debounce_duration: Duration::from_millis(50),
            ..Default::default()
        };
        let mut config_watcher = FileWatcher::new(config_config)?;
        config_watcher.set_event_callback(|_| reload_configuration())?;
        
        // Test watcher (balanced response)
        let test_config = WatchConfig {
            watch_patterns: vec!["tests/**/*.rs".to_string()],
            debounce_duration: Duration::from_millis(500),
            ..Default::default()
        };
        let mut test_watcher = FileWatcher::new(test_config)?;
        test_watcher.set_event_callback(|_| run_tests())?;
        
        Ok(Self {
            source_watcher,
            config_watcher,
            test_watcher,
        })
    }
}
```

### Performance Monitoring

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

struct WatcherPerformanceMonitor {
    events_processed: AtomicU64,
    last_event_time: Mutex<Option<Instant>>,
    processing_times: Mutex<Vec<Duration>>,
}

impl WatcherPerformanceMonitor {
    pub fn new() -> Self {
        Self {
            events_processed: AtomicU64::new(0),
            last_event_time: Mutex::new(None),
            processing_times: Mutex::new(Vec::new()),
        }
    }
    
    pub fn record_event(&self, processing_time: Duration) {
        self.events_processed.fetch_add(1, Ordering::Relaxed);
        
        if let Ok(mut times) = self.processing_times.lock() {
            times.push(processing_time);
            if times.len() > 1000 {
                times.drain(0..500); // Keep recent 500 samples
            }
        }
        
        if let Ok(mut last_time) = self.last_event_time.lock() {
            *last_time = Some(Instant::now());
        }
    }
    
    pub fn get_stats(&self) -> PerformanceStats {
        let events = self.events_processed.load(Ordering::Relaxed);
        
        let (avg_time, max_time) = if let Ok(times) = self.processing_times.lock() {
            if times.is_empty() {
                (Duration::ZERO, Duration::ZERO)
            } else {
                let total: Duration = times.iter().sum();
                let avg = total / times.len() as u32;
                let max = *times.iter().max().unwrap_or(&Duration::ZERO);
                (avg, max)
            }
        } else {
            (Duration::ZERO, Duration::ZERO)
        };
        
        PerformanceStats {
            total_events: events,
            average_processing_time: avg_time,
            max_processing_time: max_time,
        }
    }
}
```

### Integration with External Tools

```rust
// Integration with external build tools
struct ExternalToolIntegration {
    watcher: FileWatcher,
}

impl ExternalToolIntegration {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut watcher = FileWatcher::new(WatchConfig::default())?;
        
        watcher.set_event_callback(|event| {
            match event.path().extension().and_then(|s| s.to_str()) {
                Some("csd") => {
                    // Run CURSED compiler
                    std::process::Command::new("cursed")
                        .args(&["build", "--file", &event.path().to_string_lossy()])
                        .spawn()
                        .expect("Failed to run cursed build");
                }
                Some("md") => {
                    // Run documentation generator
                    std::process::Command::new("mdbook")
                        .args(&["build"])
                        .spawn()
                        .expect("Failed to run mdbook");
                }
                Some("scss") => {
                    // Run SASS compiler
                    std::process::Command::new("sass")
                        .args(&[&event.path().to_string_lossy(), &format!("{}.css", 
                            event.path().with_extension("").to_string_lossy())])
                        .spawn()
                        .expect("Failed to run sass");
                }
                _ => {}
            }
        })?;
        
        Ok(Self { watcher })
    }
}
```

---

## Summary

The CURSED file watching system provides a robust foundation for development automation and real-time feedback. Its combination of intelligent event processing, flexible configuration, and seamless integration makes it an essential tool for productive CURSED development workflows.

Key benefits:
- **Developer Productivity**: Eliminate manual build steps
- **Real-time Feedback**: Instant validation of code changes  
- **Resource Efficiency**: Smart filtering and event batching
- **Flexibility**: Configurable for any project size or workflow
- **Reliability**: Cross-platform support with robust error handling

For more information, see the [API documentation](../api/file_watching.html) and [examples](../examples/).
