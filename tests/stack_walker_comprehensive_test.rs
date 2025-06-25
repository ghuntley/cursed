/// Comprehensive test suite for the stack walking implementation
/// 
/// Tests the production-ready stack walking system across different platforms
/// and scenarios including symbol resolution, source information extraction,
/// and error handling.

use cursed::runtime::stack_walker::{
    StackWalker, StackWalkConfig, RawStackFrame, SourceFrameInfo,
    get_global_stack_walker, walk_current_stack, initialize_global_stack_walker
};
use cursed::runtime::debug_info::{MockSymbolResolver, SymbolInfo};
use cursed::error::Error as CursedError;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, debug};

/// Initialize tracing for tests
fn init_test_tracing() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .init();
    });
}

#[test]
fn test_basic_stack_walking() {
    init_test_tracing();
    info!("Testing basic stack walking functionality");
    
    let walker = StackWalker::new();
    let result = walker.walk_stack();
    
    assert!(result.is_ok(), "Stack walking should succeed");
    let frames = result.unwrap();
    
    // We should have at least one frame from this test function
    assert!(!frames.is_empty(), "Should capture at least one stack frame");
    
    debug!("Captured {} frames", frames.len());
    for (i, frame) in frames.iter().enumerate() {
        debug!("Frame {}: {}", i, frame);
    }
}

#[test]
fn test_stack_walk_with_config() {
    init_test_tracing();
    info!("Testing stack walking with custom configuration");
    
    let config = StackWalkConfig {
        max_frames: 10,
        resolve_symbols: true,
        capture_source_info: true,
        skip_system_frames: true,
        cursed_frames_only: false,
        ..Default::default()
    };
    
    let walker = StackWalker::with_config(config);
    let result = walker.walk_stack();
    
    assert!(result.is_ok());
    let frames = result.unwrap();
    
    // Should not exceed max_frames
    assert!(frames.len() <= 10, "Should not exceed configured max frames");
}

#[test]
fn test_symbol_resolution() {
    init_test_tracing();
    info!("Testing symbol resolution functionality");
    
    // Create a mock symbol resolver
    let mut resolver = MockSymbolResolver::new();
    resolver.add_symbol(0x1000, SymbolInfo {
        name: "test_symbol_resolution".to_string(),
        file: Some(PathBuf::from("test.csd")),
        line: Some(42),
        column: Some(10),
        offset: Some(0),
    });
    
    let walker = StackWalker::new().with_symbol_resolver(resolver);
    
    // Test symbol resolution
    let result = walker.resolve_symbol_for_frame_addr(0x1000);
    assert!(result.is_ok());
    
    let symbol = result.unwrap();
    assert!(symbol.is_some());
    assert_eq!(symbol.unwrap(), "test_symbol_resolution");
}

#[test]
fn test_cursed_frame_detection() {
    init_test_tracing();
    info!("Testing CURSED frame detection");
    
    // Test with CURSED file extension
    let source_info = SourceFrameInfo {
        file_path: PathBuf::from("game.csd"),
        line: 10,
        column: Some(5),
        function_name: "slay_dragons".to_string(),
        module_name: Some("combat".to_string()),
    };
    
    let frame = RawStackFrame::new(0x12345678)
        .with_source_info(source_info);
    
    assert!(frame.is_cursed_frame, "Should detect CURSED frame from .csd extension");
    
    // Test with CURSED keyword in function name
    let source_info2 = SourceFrameInfo {
        file_path: PathBuf::from("test.rs"),
        line: 20,
        column: Some(15),
        function_name: "yolo_function".to_string(),
        module_name: Some("test".to_string()),
    };
    
    let frame2 = RawStackFrame::new(0x87654321)
        .with_source_info(source_info2);
    
    assert!(frame2.is_cursed_frame, "Should detect CURSED frame from keyword");
}

#[test]
fn test_system_frame_filtering() {
    init_test_tracing();
    info!("Testing system frame filtering");
    
    let config = StackWalkConfig {
        skip_system_frames: true,
        ..Default::default()
    };
    
    let walker = StackWalker::with_config(config);
    
    // Test system frames
    let system_frame = RawStackFrame::new(0x1000)
        .with_symbol("std::thread::spawn".to_string());
    
    let rust_frame = RawStackFrame::new(0x2000)
        .with_symbol("rust_begin_unwind".to_string());
    
    let user_frame = RawStackFrame::new(0x3000)
        .with_symbol("my_cursed_function".to_string());
    
    assert!(!walker.should_include_frame(&system_frame), "Should skip system frames");
    assert!(!walker.should_include_frame(&rust_frame), "Should skip Rust runtime frames");
    assert!(walker.should_include_frame(&user_frame), "Should include user frames");
}

#[test]
fn test_cursed_only_filtering() {
    init_test_tracing();
    info!("Testing CURSED-only frame filtering");
    
    let config = StackWalkConfig {
        cursed_frames_only: true,
        ..Default::default()
    };
    
    let walker = StackWalker::with_config(config);
    
    // Create CURSED frame
    let cursed_source = SourceFrameInfo {
        file_path: PathBuf::from("test.csd"),
        line: 10,
        column: Some(5),
        function_name: "slay_function".to_string(),
        module_name: Some("game".to_string()),
    };
    
    let cursed_frame = RawStackFrame::new(0x1000)
        .with_source_info(cursed_source);
    
    // Create non-CURSED frame
    let non_cursed_frame = RawStackFrame::new(0x2000)
        .with_symbol("regular_function".to_string());
    
    assert!(walker.should_include_frame(&cursed_frame), "Should include CURSED frames");
    assert!(!walker.should_include_frame(&non_cursed_frame), "Should skip non-CURSED frames");
}

#[test]
fn test_function_name_demangling() {
    init_test_tracing();
    info!("Testing function name demangling");
    
    let walker = StackWalker::new();
    
    // Test CURSED function extraction
    let demangled = walker.extract_cursed_function_name("cursed_slay_dragons");
    assert_eq!(demangled, Some("slay_dragons".to_string()));
    
    let demangled2 = walker.extract_cursed_function_name("my_module::yolo_function");
    assert_eq!(demangled2, Some("yolo_function".to_string()));
    
    // Test non-CURSED function
    let demangled3 = walker.extract_cursed_function_name("regular_function");
    assert_eq!(demangled3, None);
}

#[test]
fn test_contextual_stack_walk() {
    init_test_tracing();
    info!("Testing contextual stack walk");
    
    let walker = StackWalker::new();
    let result = walker.walk_stack_with_context(None, Some(42));
    
    assert!(result.is_ok());
    let contextual = result.unwrap();
    
    assert_eq!(contextual.goroutine_id, Some(42));
    assert_eq!(contextual.thread_id, std::thread::current().id());
    assert!(!contextual.frames.is_empty());
    
    // Test CURSED frames filtering
    let cursed_frames = contextual.cursed_frames();
    debug!("Found {} CURSED frames", cursed_frames.len());
    
    // Test frame search
    if let Some(_frame) = contextual.find_frame("test_contextual_stack_walk") {
        debug!("Found test frame in stack");
    }
}

#[test]
fn test_global_stack_walker() {
    init_test_tracing();
    info!("Testing global stack walker");
    
    // Initialize with custom config
    let config = StackWalkConfig {
        max_frames: 50,
        resolve_symbols: true,
        ..Default::default()
    };
    initialize_global_stack_walker(config);
    
    // Test global access
    let global_walker = get_global_stack_walker();
    assert!(global_walker.lock().is_ok());
    
    // Test convenience function
    let result = walk_current_stack();
    assert!(result.is_ok());
    
    let frames = result.unwrap();
    assert!(!frames.is_empty());
    debug!("Global walker captured {} frames", frames.len());
}

#[test]
fn test_statistics_tracking() {
    init_test_tracing();
    info!("Testing statistics tracking");
    
    let walker = StackWalker::new();
    
    // Perform multiple walks
    for i in 0..3 {
        let _ = walker.walk_stack();
        debug!("Completed walk {}", i + 1);
    }
    
    // Check statistics
    let stats = walker.get_statistics();
    assert!(stats.is_ok());
    
    let stats = stats.unwrap();
    assert!(stats.total_walks >= 3, "Should track multiple walks");
    assert!(stats.total_frames > 0, "Should track frame count");
    
    debug!("Statistics: {:?}", stats);
}

#[test]
fn test_symbol_cache() {
    init_test_tracing();
    info!("Testing symbol cache functionality");
    
    let walker = StackWalker::new();
    
    // Clear cache first
    walker.clear_cache();
    
    // Perform symbol resolution to populate cache
    let _ = walker.resolve_symbol_for_frame_addr(0x12345678);
    
    // Perform same resolution again (should hit cache)
    let result1 = walker.resolve_symbol_for_frame_addr(0x12345678);
    let result2 = walker.resolve_symbol_for_frame_addr(0x12345678);
    
    // Both should be consistent
    assert_eq!(result1.is_ok(), result2.is_ok());
    
    if let (Ok(sym1), Ok(sym2)) = (result1, result2) {
        assert_eq!(sym1, sym2, "Cached results should be consistent");
    }
}

#[test]
fn test_error_handling() {
    init_test_tracing();
    info!("Testing error handling in stack walking");
    
    let walker = StackWalker::new();
    
    // Test with various edge cases
    let result = walker.walk_stack();
    
    // Should handle errors gracefully
    match result {
        Ok(frames) => {
            debug!("Successfully captured {} frames", frames.len());
        }
        Err(e) => {
            debug!("Stack walking failed with error: {}", e);
            // Some platforms might not support full stack walking
        }
    }
}

#[test]
fn test_source_info_extraction() {
    init_test_tracing();
    info!("Testing source information extraction");
    
    let walker = StackWalker::new();
    
    // Test module name extraction
    let module1 = walker.extract_module_name("my_crate::my_module::function");
    assert_eq!(module1, Some("my_module".to_string()));
    
    let module2 = walker.extract_module_name("simple_function");
    assert_eq!(module2, None);
    
    let module3 = walker.extract_module_name("crate::function");
    assert_eq!(module3, Some("crate".to_string()));
}

#[test]
fn test_frame_display() {
    init_test_tracing();
    info!("Testing frame display formatting");
    
    let source_info = SourceFrameInfo {
        file_path: PathBuf::from("test.csd"),
        line: 42,
        column: Some(10),
        function_name: "test_function".to_string(),
        module_name: Some("test_module".to_string()),
    };
    
    let frame = RawStackFrame::new(0x12345678)
        .with_symbol("test_function".to_string())
        .with_source_info(source_info);
    
    let display_str = format!("{}", frame);
    
    assert!(display_str.contains("0x12345678"), "Should show instruction pointer");
    assert!(display_str.contains("test_function"), "Should show function name");
    assert!(display_str.contains("test.csd:42"), "Should show file and line");
    assert!(display_str.contains("[CURSED]"), "Should show CURSED marker");
}

#[test]
fn test_platform_specific_implementations() {
    init_test_tracing();
    info!("Testing platform-specific stack walking");
    
    let walker = StackWalker::new();
    let result = walker.walk_stack();
    
    // Should work on all supported platforms
    assert!(result.is_ok(), "Platform-specific implementation should work");
    
    let frames = result.unwrap();
    debug!("Platform implementation captured {} frames", frames.len());
    
    // Verify we have real addresses (not just mock data)
    if !frames.is_empty() {
        let first_frame = &frames[0];
        debug!("First frame: 0x{:x}", first_frame.instruction_pointer);
        
        // Real addresses should be non-zero and in reasonable ranges
        assert_ne!(first_frame.instruction_pointer, 0, "Should have real instruction pointer");
    }
}

#[test]
fn test_concurrent_stack_walking() {
    init_test_tracing();
    info!("Testing concurrent stack walking");
    
    use std::thread;
    use std::sync::Arc;
    
    let walker = Arc::new(StackWalker::new());
    let mut handles = vec![];
    
    // Spawn multiple threads that do stack walking
    for i in 0..4 {
        let walker_clone = walker.clone();
        let handle = thread::spawn(move || {
            debug!("Thread {} starting stack walk", i);
            let result = walker_clone.walk_stack();
            debug!("Thread {} completed stack walk", i);
            result
        });
        handles.push(handle);
    }
    
    // Wait for all threads and check results
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.join().expect("Thread should not panic");
        assert!(result.is_ok(), "Concurrent stack walking should work in thread {}", i);
    }
    
    // Check that statistics reflect all walks
    let stats = walker.get_statistics().unwrap();
    assert!(stats.total_walks >= 4, "Should track concurrent walks");
}

/// Test helper function that creates a deep call stack
fn create_deep_call_stack(depth: usize) -> Result<Vec<RawStackFrame>, CursedError> {
    if depth == 0 {
        let walker = StackWalker::new();
        walker.walk_stack()
    } else {
        create_deep_call_stack(depth - 1)
    }
}

#[test]
fn test_deep_call_stack() {
    init_test_tracing();
    info!("Testing deep call stack handling");
    
    let result = create_deep_call_stack(10);
    assert!(result.is_ok(), "Should handle deep call stacks");
    
    let frames = result.unwrap();
    debug!("Deep stack captured {} frames", frames.len());
    
    // Should capture the recursive calls
    assert!(frames.len() > 5, "Should capture multiple frames from deep stack");
}

#[test]
fn test_stack_walk_performance() {
    init_test_tracing();
    info!("Testing stack walk performance");
    
    let walker = StackWalker::new();
    let start = std::time::Instant::now();
    
    // Perform multiple stack walks
    for _ in 0..100 {
        let _ = walker.walk_stack();
    }
    
    let elapsed = start.elapsed();
    debug!("100 stack walks took {:?}", elapsed);
    
    // Should complete reasonably quickly
    assert!(elapsed < std::time::Duration::from_secs(10), "Stack walking should be performant");
    
    // Check average time from statistics
    let stats = walker.get_statistics().unwrap();
    debug!("Average walk time: {:?}", stats.average_walk_time);
}
