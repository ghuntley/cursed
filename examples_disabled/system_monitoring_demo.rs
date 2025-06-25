/// System Monitoring Demo
/// 
/// Demonstrates the comprehensive system monitoring capabilities of the CURSED
/// programming language standard library including real-time metrics collection,
/// process monitoring, and performance analysis.

use cursed::stdlib::system::monitoring::{
    SystemMonitor, monitor_system, monitor_system_with_cache, monitor_continuous,
    get_resource_usage, get_performance_metrics, get_cpu_usage, get_memory_usage,
    get_disk_usage, get_network_statistics, get_system_info_summary,
    get_process_info, get_top_processes_by_cpu, get_top_processes_by_memory,
    init_monitoring, cleanup_monitoring,
};
use std::time::Duration;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CURSED System Monitoring Demo ===\n");
    
    // Initialize monitoring subsystem
    init_monitoring()?;
    println!("✓ Monitoring subsystem initialized\n");
    
    // Demo 1: Basic System Information
    demo_system_info()?;
    
    // Demo 2: Resource Usage Monitoring
    demo_resource_monitoring()?;
    
    // Demo 3: Performance Metrics
    demo_performance_metrics()?;
    
    // Demo 4: Process Information
    demo_process_monitoring()?;
    
    // Demo 5: Caching Demonstration
    demo_caching_performance()?;
    
    // Demo 6: Continuous Monitoring
    demo_continuous_monitoring()?;
    
    // Demo 7: Concurrent Access
    demo_concurrent_monitoring()?;
    
    // Cleanup
    cleanup_monitoring()?;
    println!("✓ Monitoring subsystem cleaned up");
    
    Ok(())
}

fn demo_system_info() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== System Information ===");
    
    let info = get_system_info_summary()?;
    
    println!("Operating System: {}", info.get("os_name").unwrap_or(&"Unknown".to_string()));
    println!("Kernel Version: {}", info.get("kernel_version").unwrap_or(&"Unknown".to_string()));
    println!("OS Version: {}", info.get("os_version").unwrap_or(&"Unknown".to_string()));
    println!("Hostname: {}", info.get("host_name").unwrap_or(&"Unknown".to_string()));
    println!("CPU Count: {}", info.get("cpu_count").unwrap_or(&"0".to_string()));
    
    let total_memory_str = info.get("total_memory").unwrap_or(&"0".to_string());
    let total_memory: u64 = total_memory_str.parse().unwrap_or(0);
    println!("Total Memory: {:.2} GB", total_memory as f64 / (1024.0 * 1024.0 * 1024.0));
    
    let uptime_str = info.get("uptime").unwrap_or(&"0".to_string());
    let uptime: u64 = uptime_str.parse().unwrap_or(0);
    let uptime_hours = uptime / 3600;
    let uptime_minutes = (uptime % 3600) / 60;
    println!("System Uptime: {}h {}m", uptime_hours, uptime_minutes);
    
    println!();
    Ok(())
}

fn demo_resource_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Resource Usage Monitoring ===");
    
    // Get overall resource usage
    let usage = get_resource_usage()?;
    
    println!("CPU Usage: {:.1}%", usage.cpu_usage);
    println!("Memory Usage: {:.2} GB", usage.memory_usage as f64 / (1024.0 * 1024.0 * 1024.0));
    println!("Disk Usage: {:.2} GB", usage.disk_usage as f64 / (1024.0 * 1024.0 * 1024.0));
    println!("Network RX: {:.2} MB", usage.network_rx as f64 / (1024.0 * 1024.0));
    println!("Network TX: {:.2} MB", usage.network_tx as f64 / (1024.0 * 1024.0));
    
    // Get individual metrics
    println!("\nIndividual Metrics:");
    println!("CPU: {:.1}%", get_cpu_usage()?);
    println!("Memory: {:.2} GB", get_memory_usage()? as f64 / (1024.0 * 1024.0 * 1024.0));
    println!("Disk: {:.2} GB", get_disk_usage()? as f64 / (1024.0 * 1024.0 * 1024.0));
    
    let (net_rx, net_tx) = get_network_statistics()?;
    println!("Network: RX {:.2} MB, TX {:.2} MB", 
             net_rx as f64 / (1024.0 * 1024.0),
             net_tx as f64 / (1024.0 * 1024.0));
    
    println!();
    Ok(())
}

fn demo_performance_metrics() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Performance Metrics ===");
    
    let metrics = get_performance_metrics()?;
    
    println!("Uptime: {:.2} hours", metrics.uptime.as_secs_f64() / 3600.0);
    println!("Load Average: {:?}", metrics.load_average);
    println!("Process Count: {}", metrics.process_count);
    println!("Thread Count: {}", metrics.thread_count);
    println!("Handles: {}", metrics.handles);
    
    println!("\nDetailed Metrics:");
    for (key, value) in &metrics.metrics {
        if key.starts_with("cpu_") && key != "cpu_usage" {
            println!("  {}: {:.1}%", key, value);
        } else if key.contains("memory") {
            if key.contains("percent") {
                println!("  {}: {:.1}%", key, value);
            } else {
                println!("  {}: {:.2} GB", key, value / (1024.0 * 1024.0 * 1024.0));
            }
        }
    }
    
    println!();
    Ok(())
}

fn demo_process_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Process Monitoring ===");
    
    // Get current process info
    let current_pid = std::process::id();
    println!("Current Process (PID {}):", current_pid);
    
    match get_process_info(current_pid) {
        Ok(info) => {
            println!("  Name: {}", info.get("name").unwrap_or(&"Unknown".to_string()));
            println!("  CPU Usage: {}%", info.get("cpu_usage").unwrap_or(&"0".to_string()));
            
            let memory_str = info.get("memory").unwrap_or(&"0".to_string());
            let memory: u64 = memory_str.parse().unwrap_or(0);
            println!("  Memory: {:.2} MB", memory as f64 / (1024.0 * 1024.0));
            
            println!("  Status: {}", info.get("status").unwrap_or(&"Unknown".to_string()));
        }
        Err(e) => println!("  Error getting process info: {}", e),
    }
    
    // Get top processes by CPU
    println!("\nTop 5 Processes by CPU Usage:");
    match get_top_processes_by_cpu(5) {
        Ok(processes) => {
            for (i, (pid, name, cpu)) in processes.iter().enumerate() {
                println!("  {}. {} (PID {}): {:.1}%", i + 1, name, pid, cpu);
            }
        }
        Err(e) => println!("  Error getting top processes: {}", e),
    }
    
    // Get top processes by memory
    println!("\nTop 5 Processes by Memory Usage:");
    match get_top_processes_by_memory(5) {
        Ok(processes) => {
            for (i, (pid, name, memory)) in processes.iter().enumerate() {
                println!("  {}. {} (PID {}): {:.2} MB", 
                         i + 1, name, pid, *memory as f64 / (1024.0 * 1024.0));
            }
        }
        Err(e) => println!("  Error getting top processes: {}", e),
    }
    
    println!();
    Ok(())
}

fn demo_caching_performance() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Caching Performance Demo ===");
    
    // Create monitor with short cache duration for demo
    let monitor = SystemMonitor::with_cache_duration(Duration::from_millis(500));
    
    println!("Testing cache performance...");
    
    // First call (should populate cache)
    let start = std::time::Instant::now();
    let _usage1 = monitor.get_resource_usage()?;
    let first_call_time = start.elapsed();
    
    // Second call (should use cache)
    let start = std::time::Instant::now();
    let _usage2 = monitor.get_resource_usage()?;
    let second_call_time = start.elapsed();
    
    println!("First call (populate cache): {:.3}ms", first_call_time.as_secs_f64() * 1000.0);
    println!("Second call (use cache): {:.3}ms", second_call_time.as_secs_f64() * 1000.0);
    
    // Show cache statistics
    let cache_stats = monitor.get_cache_stats()?;
    println!("\nCache Statistics:");
    for (metric, cached) in &cache_stats {
        println!("  {}: {}", metric, if *cached { "Cached" } else { "Not Cached" });
    }
    
    // Clear cache and test again
    monitor.clear_cache()?;
    println!("\nCache cleared.");
    
    let cache_stats_after = monitor.get_cache_stats()?;
    let cached_count = cache_stats_after.values().filter(|&&v| v).count();
    println!("Cached metrics after clear: {}", cached_count);
    
    println!();
    Ok(())
}

fn demo_continuous_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Continuous Monitoring Demo ===");
    println!("Monitoring for 5 seconds...");
    
    let start_time = std::time::Instant::now();
    let mut sample_count = 0;
    
    monitor_continuous(Duration::from_millis(500), |usage| {
        sample_count += 1;
        let elapsed = start_time.elapsed();
        
        println!("Sample {}: CPU {:.1}%, Memory {:.2} GB, Time {:.1}s",
                 sample_count,
                 usage.cpu_usage,
                 usage.memory_usage as f64 / (1024.0 * 1024.0 * 1024.0),
                 elapsed.as_secs_f64());
        
        // Stop after 5 seconds
        elapsed < Duration::from_secs(5)
    })?;
    
    println!("Continuous monitoring completed. Total samples: {}\n", sample_count);
    Ok(())
}

fn demo_concurrent_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Concurrent Monitoring Demo ===");
    
    let monitor = std::sync::Arc::new(SystemMonitor::new());
    let mut handles = vec![];
    
    println!("Starting 4 monitoring threads...");
    
    for thread_id in 0..4 {
        let monitor_clone = std::sync::Arc::clone(&monitor);
        let handle = thread::spawn(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            for i in 0..3 {
                let usage = monitor_clone.get_resource_usage()?;
                println!("Thread {}, Sample {}: CPU {:.1}%, Memory {:.2} GB",
                         thread_id, i + 1,
                         usage.cpu_usage,
                         usage.memory_usage as f64 / (1024.0 * 1024.0 * 1024.0));
                thread::sleep(Duration::from_millis(200));
            }
            Ok(())
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(result) => {
                if let Err(e) = result {
                    println!("Thread {} error: {}", i, e);
                }
            }
            Err(_) => println!("Thread {} panicked", i),
        }
    }
    
    println!("All monitoring threads completed.");
    
    // Verify monitor is still functional
    let final_usage = monitor.get_resource_usage()?;
    println!("Final check - CPU: {:.1}%, Memory: {:.2} GB",
             final_usage.cpu_usage,
             final_usage.memory_usage as f64 / (1024.0 * 1024.0 * 1024.0));
    
    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_demo_functions() {
        // Test that all demo functions can run without panicking
        assert!(init_monitoring().is_ok());
        assert!(demo_system_info().is_ok());
        assert!(demo_resource_monitoring().is_ok());
        assert!(demo_performance_metrics().is_ok());
        assert!(demo_process_monitoring().is_ok());
        assert!(demo_caching_performance().is_ok());
        assert!(cleanup_monitoring().is_ok());
    }
}
