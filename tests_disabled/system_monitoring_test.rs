/// Comprehensive test suite for system monitoring module
/// 
/// Tests real system monitoring capabilities including CPU, memory,
/// disk, and network usage tracking with cross-platform support.

use cursed::stdlib::system::monitoring::{
    SystemMonitor, ResourceUsage, PerformanceMetrics,
    monitor_system, monitor_system_with_cache, get_resource_usage,
    get_performance_metrics, get_cpu_usage, get_memory_usage,
    get_disk_usage, get_network_statistics, get_system_info_summary,
    get_process_info, get_top_processes_by_cpu, get_top_processes_by_memory,
    init_monitoring, cleanup_monitoring,
};
use std::time::Duration;
use std::thread;
use std::sync::Arc;

#[test]
fn test_basic_system_monitor_functionality() {
    let mut monitor = SystemMonitor::new();
    
    // Test initial state
    assert!(!monitor.is_enabled());
    
    // Test start/stop
    assert!(monitor.start().is_ok());
    assert!(monitor.is_enabled());
    
    assert!(monitor.stop().is_ok());
    assert!(!monitor.is_enabled());
}

#[test]
fn test_custom_cache_duration() {
    let cache_duration = Duration::from_millis(500);
    let monitor = SystemMonitor::with_cache_duration(cache_duration);
    assert!(!monitor.is_enabled());
    
    // Test that the monitor works with custom cache duration
    let usage_result = monitor.get_resource_usage();
    assert!(usage_result.is_ok());
}

#[test]
fn test_resource_usage_collection() {
    let result = get_resource_usage();
    assert!(result.is_ok());
    
    let usage = result.unwrap();
    
    // Validate CPU usage is within reasonable range
    assert!(usage.cpu_usage >= 0.0);
    assert!(usage.cpu_usage <= 100.0);
    
    // Memory usage should be positive
    assert!(usage.memory_usage > 0);
    
    // Disk usage should be non-negative
    assert!(usage.disk_usage >= 0);
    
    // Network stats should be non-negative
    assert!(usage.network_rx >= 0);
    assert!(usage.network_tx >= 0);
}

#[test]
fn test_individual_metric_functions() {
    // Test CPU usage
    let cpu_result = get_cpu_usage();
    assert!(cpu_result.is_ok());
    let cpu = cpu_result.unwrap();
    assert!(cpu >= 0.0 && cpu <= 100.0);
    
    // Test memory usage
    let memory_result = get_memory_usage();
    assert!(memory_result.is_ok());
    let memory = memory_result.unwrap();
    assert!(memory > 0);
    
    // Test disk usage
    let disk_result = get_disk_usage();
    assert!(disk_result.is_ok());
    let disk = disk_result.unwrap();
    assert!(disk >= 0);
    
    // Test network statistics
    let network_result = get_network_statistics();
    assert!(network_result.is_ok());
    let (rx, tx) = network_result.unwrap();
    assert!(rx >= 0);
    assert!(tx >= 0);
}

#[test]
fn test_performance_metrics() {
    let result = get_performance_metrics();
    assert!(result.is_ok());
    
    let metrics = result.unwrap();
    
    // Basic sanity checks
    assert!(metrics.process_count > 0);
    assert!(!metrics.load_average.is_empty());
    assert!(metrics.thread_count >= 0);
    assert!(metrics.handles >= 0);
    assert!(!metrics.metrics.is_empty());
    
    // Check specific metrics exist
    assert!(metrics.metrics.contains_key("cpu_usage"));
    assert!(metrics.metrics.contains_key("memory_usage"));
    assert!(metrics.metrics.contains_key("total_memory"));
    assert!(metrics.metrics.contains_key("memory_usage_percent"));
    assert!(metrics.metrics.contains_key("free_memory"));
    
    // Validate metric values
    let cpu_usage = metrics.metrics.get("cpu_usage").unwrap();
    assert!(*cpu_usage >= 0.0 && *cpu_usage <= 100.0);
    
    let memory_usage_percent = metrics.metrics.get("memory_usage_percent").unwrap();
    assert!(*memory_usage_percent >= 0.0 && *memory_usage_percent <= 100.0);
}

#[test]
fn test_system_info_summary() {
    let result = get_system_info_summary();
    assert!(result.is_ok());
    
    let info = result.unwrap();
    
    // Verify required fields are present
    assert!(info.contains_key("os_name"));
    assert!(info.contains_key("kernel_version"));
    assert!(info.contains_key("os_version"));
    assert!(info.contains_key("host_name"));
    assert!(info.contains_key("cpu_count"));
    assert!(info.contains_key("total_memory"));
    assert!(info.contains_key("uptime"));
    
    // Validate some values
    let cpu_count = info.get("cpu_count").unwrap().parse::<usize>().unwrap();
    assert!(cpu_count > 0);
    
    let total_memory = info.get("total_memory").unwrap().parse::<u64>().unwrap();
    assert!(total_memory > 0);
    
    let uptime = info.get("uptime").unwrap().parse::<u64>().unwrap();
    assert!(uptime > 0);
}

#[test]
fn test_caching_mechanism() {
    let monitor = SystemMonitor::with_cache_duration(Duration::from_secs(2));
    
    // First call should populate cache
    let start_time = std::time::Instant::now();
    let usage1 = monitor.get_resource_usage().unwrap();
    let first_call_duration = start_time.elapsed();
    
    // Second call should be faster due to caching
    let start_time2 = std::time::Instant::now();
    let usage2 = monitor.get_resource_usage().unwrap();
    let second_call_duration = start_time2.elapsed();
    
    // Second call should generally be faster (though not guaranteed on all systems)
    // At minimum, both calls should succeed
    assert!(usage1.cpu_usage >= 0.0);
    assert!(usage2.cpu_usage >= 0.0);
    
    // Values should be the same or similar due to caching
    let cpu_diff = (usage1.cpu_usage - usage2.cpu_usage).abs();
    assert!(cpu_diff < 50.0); // Allow for some variance
    
    // Test cache stats
    let cache_stats = monitor.get_cache_stats().unwrap();
    assert!(cache_stats.len() > 0);
    assert!(cache_stats.values().any(|&cached| cached)); // At least one metric should be cached
}

#[test]
fn test_cache_clearing() {
    let monitor = SystemMonitor::new();
    
    // Populate cache by making calls
    let _ = monitor.get_resource_usage();
    let _ = monitor.get_performance_metrics();
    
    // Clear cache
    assert!(monitor.clear_cache().is_ok());
    
    // Verify cache is cleared
    let cache_stats = monitor.get_cache_stats().unwrap();
    assert!(cache_stats.values().all(|&cached| !cached));
}

#[test]
fn test_load_average() {
    let monitor = SystemMonitor::new();
    let load_avg_result = monitor.get_load_average();
    assert!(load_avg_result.is_ok());
    
    let load_avg = load_avg_result.unwrap();
    assert_eq!(load_avg.len(), 3); // 1, 5, 15 minute averages
    
    // Load average values should be non-negative
    for &avg in &load_avg {
        assert!(avg >= 0.0);
    }
}

#[test]
fn test_system_uptime() {
    let monitor = SystemMonitor::new();
    let uptime_result = monitor.get_system_uptime();
    assert!(uptime_result.is_ok());
    
    let uptime = uptime_result.unwrap();
    assert!(uptime.as_secs() > 0);
}

#[test]
fn test_process_information() {
    // Test with current process
    let current_pid = std::process::id();
    let result = get_process_info(current_pid);
    assert!(result.is_ok());
    
    let info = result.unwrap();
    
    // Verify required fields
    assert!(info.contains_key("name"));
    assert!(info.contains_key("cpu_usage"));
    assert!(info.contains_key("memory"));
    assert!(info.contains_key("virtual_memory"));
    assert!(info.contains_key("status"));
    assert!(info.contains_key("start_time"));
    assert!(info.contains_key("run_time"));
    
    // Validate some values
    let memory = info.get("memory").unwrap().parse::<u64>().unwrap();
    assert!(memory > 0);
    
    let virtual_memory = info.get("virtual_memory").unwrap().parse::<u64>().unwrap();
    assert!(virtual_memory >= memory); // Virtual memory should be >= physical memory
}

#[test]
fn test_top_processes() {
    // Test top processes by CPU
    let cpu_result = get_top_processes_by_cpu(5);
    assert!(cpu_result.is_ok());
    let cpu_processes = cpu_result.unwrap();
    assert!(cpu_processes.len() <= 5);
    
    // Verify processes are sorted by CPU usage (descending)
    for i in 1..cpu_processes.len() {
        assert!(cpu_processes[i-1].2 >= cpu_processes[i].2);
    }
    
    // Test top processes by memory
    let memory_result = get_top_processes_by_memory(5);
    assert!(memory_result.is_ok());
    let memory_processes = memory_result.unwrap();
    assert!(memory_processes.len() <= 5);
    
    // Verify processes are sorted by memory usage (descending)
    for i in 1..memory_processes.len() {
        assert!(memory_processes[i-1].2 >= memory_processes[i].2);
    }
    
    // All processes should have valid PIDs and non-empty names
    for (pid, name, _) in &cpu_processes {
        assert!(*pid > 0);
        assert!(!name.is_empty());
    }
}

#[test]
fn test_thread_and_handle_counts() {
    let monitor = SystemMonitor::new();
    
    let thread_count_result = monitor.get_thread_count();
    assert!(thread_count_result.is_ok());
    let thread_count = thread_count_result.unwrap();
    assert!(thread_count > 0);
    
    let handle_count_result = monitor.get_handles_count();
    assert!(handle_count_result.is_ok());
    let handle_count = handle_count_result.unwrap();
    assert!(handle_count > 0);
}

#[test]
fn test_network_statistics() {
    let monitor = SystemMonitor::new();
    let network_result = monitor.get_network_stats();
    assert!(network_result.is_ok());
    
    let stats = network_result.unwrap();
    assert!(stats.rx_bytes >= 0);
    assert!(stats.tx_bytes >= 0);
    assert!(stats.rx_packets >= 0);
    assert!(stats.tx_packets >= 0);
}

#[test]
fn test_monitor_system_functions() {
    // Test basic monitor_system function
    let result = monitor_system();
    assert!(result.is_ok());
    let monitor = result.unwrap();
    assert!(monitor.is_enabled());
    
    // Test monitor_system_with_cache function
    let cache_duration = Duration::from_millis(100);
    let cache_result = monitor_system_with_cache(cache_duration);
    assert!(cache_result.is_ok());
    let cache_monitor = cache_result.unwrap();
    assert!(cache_monitor.is_enabled());
}

#[test]
fn test_concurrent_monitoring() {
    let monitor = Arc::new(SystemMonitor::new());
    let mut handles = vec![];
    
    // Start multiple threads accessing the monitor concurrently
    for i in 0..5 {
        let monitor_clone = Arc::clone(&monitor);
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let usage_result = monitor_clone.get_resource_usage();
                assert!(usage_result.is_ok(), "Thread {} iteration {} failed", i, j);
                
                let metrics_result = monitor_clone.get_performance_metrics();
                assert!(metrics_result.is_ok(), "Thread {} metrics iteration {} failed", i, j);
                
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    // Monitor should still be functional after concurrent access
    assert!(monitor.get_resource_usage().is_ok());
    assert!(monitor.get_performance_metrics().is_ok());
}

#[test]
fn test_monitoring_init_cleanup() {
    assert!(init_monitoring().is_ok());
    assert!(cleanup_monitoring().is_ok());
}

#[test]
fn test_platform_specific_features() {
    let monitor = SystemMonitor::new();
    
    // Test load average (different behavior on Unix vs Windows)
    let load_avg = monitor.get_load_average().unwrap();
    
    #[cfg(unix)]
    {
        // On Unix systems, load average should be real system load
        assert_eq!(load_avg.len(), 3);
        for &avg in &load_avg {
            assert!(avg >= 0.0);
            // Load average can be higher than 1.0, but shouldn't be extremely high in tests
            assert!(avg < 1000.0);
        }
    }
    
    #[cfg(windows)]
    {
        // On Windows, load average is approximated using CPU usage
        assert_eq!(load_avg.len(), 3);
        for &avg in &load_avg {
            assert!(avg >= 0.0 && avg <= 1.0);
        }
    }
    
    // Test file descriptor/handle count
    let handles = monitor.get_handles_count().unwrap();
    assert!(handles > 0);
}

#[test]
fn test_error_handling() {
    // Test with invalid PID
    let invalid_pid_result = get_process_info(0);
    assert!(invalid_pid_result.is_err());
    
    // Test with extremely high PID (likely not to exist)
    let high_pid_result = get_process_info(999999999);
    assert!(high_pid_result.is_err());
}

#[test]
fn test_monitoring_performance() {
    let monitor = SystemMonitor::new();
    
    // Test that monitoring calls complete within reasonable time
    let start_time = std::time::Instant::now();
    
    for _ in 0..10 {
        let _ = monitor.get_resource_usage();
    }
    
    let elapsed = start_time.elapsed();
    
    // All 10 calls should complete within 5 seconds (very generous)
    assert!(elapsed < Duration::from_secs(5));
    
    // Test individual call performance
    let start_time = std::time::Instant::now();
    let _ = monitor.get_cpu_usage();
    let cpu_duration = start_time.elapsed();
    
    // Single CPU usage call should be very fast
    assert!(cpu_duration < Duration::from_secs(2));
}

#[test]
fn test_metric_consistency() {
    let monitor = SystemMonitor::new();
    
    // Get metrics multiple times and ensure they're within reasonable ranges
    let mut cpu_readings = Vec::new();
    let mut memory_readings = Vec::new();
    
    for _ in 0..5 {
        if let Ok(usage) = monitor.get_resource_usage() {
            cpu_readings.push(usage.cpu_usage);
            memory_readings.push(usage.memory_usage);
        }
        thread::sleep(Duration::from_millis(100));
    }
    
    // All readings should be valid
    assert_eq!(cpu_readings.len(), 5);
    assert_eq!(memory_readings.len(), 5);
    
    // CPU readings should be within valid range
    for &cpu in &cpu_readings {
        assert!(cpu >= 0.0 && cpu <= 100.0);
    }
    
    // Memory readings should be positive and not vary wildly
    for &memory in &memory_readings {
        assert!(memory > 0);
    }
    
    // Memory shouldn't vary by more than 50% between readings
    if let (Some(&min_mem), Some(&max_mem)) = (memory_readings.iter().min(), memory_readings.iter().max()) {
        let variation_ratio = (max_mem as f64) / (min_mem as f64);
        assert!(variation_ratio < 1.5, "Memory usage varied too much: {} to {}", min_mem, max_mem);
    }
}
