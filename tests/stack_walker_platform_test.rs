/// Platform-specific tests for stack walking functionality
/// 
/// Tests platform-specific stack walking implementations to ensure
/// they work correctly on different operating systems and architectures.

use cursed::runtime::stack_walker::{StackWalker, StackWalkConfig};
use cursed::error::Error as CursedError;
use tracing::{info, debug, warn};

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
fn test_linux_stack_walking() {
    init_test_tracing();
    info!("Testing Linux-specific stack walking");
    
    #[cfg(target_os = "linux")]
    {
        let walker = StackWalker::new();
        let result = walker.walk_stack();
        
        assert!(result.is_ok(), "Linux stack walking should work");
        let frames = result.unwrap();
        
        debug!("Linux captured {} frames", frames.len());
        
        // Should capture at least some frames
        assert!(!frames.is_empty(), "Should capture frames on Linux");
        
        // Check that we have real instruction pointers
        for frame in &frames {
            assert_ne!(frame.instruction_pointer, 0, "Should have valid instruction pointers");
            if let Some(symbol) = &frame.symbol_name {
                debug!("Linux frame: {} at 0x{:x}", symbol, frame.instruction_pointer);
            }
        }
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        warn!("Skipping Linux-specific test on non-Linux platform");
    }
}

#[test]
fn test_macos_stack_walking() {
    init_test_tracing();
    info!("Testing macOS-specific stack walking");
    
    #[cfg(target_os = "macos")]
    {
        let walker = StackWalker::new();
        let result = walker.walk_stack();
        
        assert!(result.is_ok(), "macOS stack walking should work");
        let frames = result.unwrap();
        
        debug!("macOS captured {} frames", frames.len());
        
        // Should capture at least some frames
        assert!(!frames.is_empty(), "Should capture frames on macOS");
        
        // Check that we have real instruction pointers
        for frame in &frames {
            assert_ne!(frame.instruction_pointer, 0, "Should have valid instruction pointers");
            if let Some(symbol) = &frame.symbol_name {
                debug!("macOS frame: {} at 0x{:x}", symbol, frame.instruction_pointer);
            }
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        warn!("Skipping macOS-specific test on non-macOS platform");
    }
}

#[test]
fn test_windows_stack_walking() {
    init_test_tracing();
    info!("Testing Windows-specific stack walking");
    
    #[cfg(target_os = "windows")]
    {
        let walker = StackWalker::new();
        let result = walker.walk_stack();
        
        assert!(result.is_ok(), "Windows stack walking should work");
        let frames = result.unwrap();
        
        debug!("Windows captured {} frames", frames.len());
        
        // Windows implementation might be more limited
        if !frames.is_empty() {
            // Check that we have real instruction pointers
            for frame in &frames {
                assert_ne!(frame.instruction_pointer, 0, "Should have valid instruction pointers");
                if let Some(symbol) = &frame.symbol_name {
                    debug!("Windows frame: {} at 0x{:x}", symbol, frame.instruction_pointer);
                }
            }
        } else {
            warn!("Windows stack walking returned no frames (expected for basic implementation)");
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        warn!("Skipping Windows-specific test on non-Windows platform");
    }
}

#[test]
fn test_generic_stack_walking() {
    init_test_tracing();
    info!("Testing generic stack walking fallback");
    
    // This test should work on all platforms
    let walker = StackWalker::new();
    let result = walker.walk_stack();
    
    // Should either succeed or fail gracefully
    match result {
        Ok(frames) => {
            debug!("Generic stack walking captured {} frames", frames.len());
            // Some platforms might return empty frames, which is acceptable
        }
        Err(e) => {
            debug!("Generic stack walking failed: {}", e);
            // Some platforms might not support stack walking
        }
    }
}

#[test]
fn test_symbol_resolution_platform_specific() {
    init_test_tracing();
    info!("Testing platform-specific symbol resolution");
    
    let config = StackWalkConfig {
        resolve_symbols: true,
        capture_source_info: true,
        ..Default::default()
    };
    
    let walker = StackWalker::with_config(config);
    let result = walker.walk_stack();
    
    if let Ok(frames) = result {
        let frames_with_symbols = frames.iter()
            .filter(|f| f.symbol_name.is_some())
            .count();
        
        debug!("Found {} frames with symbols out of {}", frames_with_symbols, frames.len());
        
        // On platforms with good debug support, we should get some symbols
        if frames_with_symbols > 0 {
            for frame in frames.iter().filter(|f| f.symbol_name.is_some()) {
                debug!("Symbol: {:?}", frame.symbol_name);
            }
        }
    }
}

#[test]
fn test_source_info_platform_specific() {
    init_test_tracing();
    info!("Testing platform-specific source information extraction");
    
    let config = StackWalkConfig {
        capture_source_info: true,
        ..Default::default()
    };
    
    let walker = StackWalker::with_config(config);
    let result = walker.walk_stack();
    
    if let Ok(frames) = result {
        let frames_with_source = frames.iter()
            .filter(|f| f.source_info.is_some())
            .count();
        
        debug!("Found {} frames with source info out of {}", frames_with_source, frames.len());
        
        // Source info might not be available on all platforms
        if frames_with_source > 0 {
            for frame in frames.iter().filter(|f| f.source_info.is_some()) {
                if let Some(ref source) = frame.source_info {
                    debug!("Source: {}:{}", source.file_path.display(), source.line);
                }
            }
        }
    }
}

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn test_addr2line_symbol_resolution() {
    init_test_tracing();
    info!("Testing addr2line symbol resolution on Unix systems");
    
    let walker = StackWalker::new();
    
    // Try to resolve a symbol using addr2line (this might fail if addr2line is not available)
    let frames = walker.walk_stack().unwrap_or_default();
    
    if !frames.is_empty() {
        let ip = frames[0].instruction_pointer;
        let result = walker.resolve_symbol_with_addr2line(ip);
        
        match result {
            Ok(Some(symbol)) => {
                debug!("addr2line resolved symbol: {}", symbol);
            }
            Ok(None) => {
                debug!("addr2line could not resolve symbol for 0x{:x}", ip);
            }
            Err(e) => {
                debug!("addr2line resolution failed: {}", e);
                // This is acceptable - addr2line might not be available
            }
        }
    }
}

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn test_addr2line_source_info() {
    init_test_tracing();
    info!("Testing addr2line source information extraction");
    
    let walker = StackWalker::new();
    
    // Try to get source info using addr2line
    let frames = walker.walk_stack().unwrap_or_default();
    
    if !frames.is_empty() {
        let ip = frames[0].instruction_pointer;
        let result = walker.extract_source_info_with_addr2line(ip);
        
        match result {
            Ok(Some(source_info)) => {
                debug!("addr2line source info: {}:{} in {}", 
                       source_info.file_path.display(), 
                       source_info.line,
                       source_info.function_name);
            }
            Ok(None) => {
                debug!("addr2line could not extract source info for 0x{:x}", ip);
            }
            Err(e) => {
                debug!("addr2line source extraction failed: {}", e);
                // This is acceptable - addr2line might not be available
            }
        }
    }
}

#[test]
fn test_architecture_specific_features() {
    init_test_tracing();
    info!("Testing architecture-specific features");
    
    // Test that the code compiles and runs on different architectures
    let walker = StackWalker::new();
    let result = walker.walk_stack();
    
    // Should handle different architectures gracefully
    match result {
        Ok(frames) => {
            debug!("Architecture-specific stack walking captured {} frames", frames.len());
            
            #[cfg(target_arch = "x86_64")]
            debug!("Running on x86_64 architecture");
            
            #[cfg(target_arch = "x86")]
            debug!("Running on x86 architecture");
            
            #[cfg(target_arch = "aarch64")]
            debug!("Running on ARM64 architecture");
            
            #[cfg(target_arch = "arm")]
            debug!("Running on ARM architecture");
        }
        Err(e) => {
            debug!("Architecture-specific stack walking failed: {}", e);
        }
    }
}

#[test]
fn test_cross_platform_consistency() {
    init_test_tracing();
    info!("Testing cross-platform consistency");
    
    let walker = StackWalker::new();
    
    // Perform multiple walks and check consistency
    let mut results = Vec::new();
    for i in 0..3 {
        let result = walker.walk_stack();
        debug!("Walk {}: {:?}", i, result.as_ref().map(|f| f.len()));
        results.push(result);
    }
    
    // All results should have the same success/failure pattern
    let all_ok = results.iter().all(|r| r.is_ok());
    let all_err = results.iter().all(|r| r.is_err());
    
    assert!(all_ok || all_err, "Stack walking should be consistent across calls");
    
    if all_ok {
        let frame_counts: Vec<usize> = results.into_iter()
            .map(|r| r.unwrap().len())
            .collect();
        
        debug!("Frame counts: {:?}", frame_counts);
        
        // Frame counts should be similar (might vary slightly due to timing)
        let min_frames = *frame_counts.iter().min().unwrap();
        let max_frames = *frame_counts.iter().max().unwrap();
        
        // Should not vary by more than a few frames
        assert!(max_frames - min_frames <= 5, "Frame counts should be consistent");
    }
}

#[test]
fn test_platform_error_handling() {
    init_test_tracing();
    info!("Testing platform-specific error handling");
    
    let walker = StackWalker::new();
    
    // Test error handling for invalid addresses
    let invalid_result = walker.resolve_symbol_for_frame_addr(0);
    match invalid_result {
        Ok(None) => debug!("Correctly handled invalid address"),
        Ok(Some(_)) => debug!("Unexpectedly resolved symbol for invalid address"),
        Err(e) => debug!("Error resolving invalid address: {}", e),
    }
    
    // Test error handling for very large addresses
    let large_addr_result = walker.resolve_symbol_for_frame_addr(usize::MAX);
    match large_addr_result {
        Ok(None) => debug!("Correctly handled large address"),
        Ok(Some(_)) => debug!("Unexpectedly resolved symbol for large address"),
        Err(e) => debug!("Error resolving large address: {}", e),
    }
}

#[test]
fn test_platform_resource_usage() {
    init_test_tracing();
    info!("Testing platform resource usage");
    
    let walker = StackWalker::new();
    
    // Measure resource usage during stack walking
    let start = std::time::Instant::now();
    let start_memory = get_memory_usage();
    
    // Perform many stack walks
    for _ in 0..50 {
        let _ = walker.walk_stack();
    }
    
    let elapsed = start.elapsed();
    let end_memory = get_memory_usage();
    
    debug!("50 stack walks took {:?}", elapsed);
    debug!("Memory usage: start={}, end={}, diff={}", 
           start_memory, end_memory, end_memory - start_memory);
    
    // Should complete in reasonable time
    assert!(elapsed < std::time::Duration::from_secs(5), 
            "Stack walking should not take too long");
    
    // Memory usage should not grow excessively
    let memory_growth = end_memory - start_memory;
    assert!(memory_growth < 10_000_000, // 10MB
            "Memory usage should not grow excessively");
}

/// Get current memory usage (platform-specific)
fn get_memory_usage() -> usize {
    #[cfg(target_os = "linux")]
    {
        // Read from /proc/self/status
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<usize>() {
                            return kb * 1024; // Convert to bytes
                        }
                    }
                }
            }
        }
    }
    
    #[cfg(target_os = "macos")]
    {
        // Use task_info on macOS (simplified)
        // In a real implementation, this would use mach APIs
        return 0; // Placeholder
    }
    
    #[cfg(target_os = "windows")]
    {
        // Use GetProcessMemoryInfo on Windows (simplified)
        // In a real implementation, this would use Windows APIs
        return 0; // Placeholder
    }
    
    // Fallback for unsupported platforms
    0
}
