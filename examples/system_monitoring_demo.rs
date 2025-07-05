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
    
    println!("Operating System: {}", info.os);
    println!("Architecture: {}", info.architecture);
    println!("Hostname: {}", info.hostname);
    println!("CPU Count: {}", info.cpu_count);
    println!("Total Memory: {:.2} GB", info.total_memory as f64 / (1024.0 * 1024.0 * 1024.0));
    
    let uptime = info.boot_time.elapsed().unwrap_or_default().as_secs();
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
    
    println!("CPU Usage: {:.1}%", usage.cpu_percent);
    println!("Memory Usage: {:.2} GB", usage.memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0));
    println!("Disk Usage: {:.2} GB", usage.disk_usage_bytes as f64 / (1024.0 * 1024.0 * 1024.0));
    println!("Network RX: {:.2} MB", usage.network_bytes_received as f64 / (1024.0 * 1024.0));
    println!("Network TX: {:.2} MB", usage.network_bytes_sent as f64 / (1024.0 * 1024.0));
    
    // Get individual metrics
    println!("\nIndividual Metrics:");
    println!("CPU: {:.1}%", get_cpu_usage()?);
    println!("Memory: {:.2} GB", get_memory_usage()? as f64 / (1024.0 * 1024.0 * 1024.0));
    println!("Disk: {:.2} GB", get_disk_usage()? as f64 / (1024.0 * 1024.0 * 1024.0));
    
    let network_stats = get_network_statistics()?;
    println!("Network: RX {:.2} MB, TX {:.2} MB", 
             network_stats.bytes_received as f64 / (1024.0 * 1024.0),
             network_stats.bytes_sent as f64 / (1024.0 * 1024.0));
    
    println!();
    Ok(())
}

fn demo_performance_metrics() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Performance Metrics ===");
    
    let metrics = get_performance_metrics()?;
    
    println!("CPU Usage: {:.1}%", metrics.cpu_usage);
    println!("Memory Usage: {:.1}%", metrics.memory_usage);
    println!("Disk I/O Read: {} bytes", metrics.disk_io_read);
    println!("Disk I/O Write: {} bytes", metrics.disk_io_write);
    println!("Network In: {} bytes", metrics.network_in);
    println!("Network Out: {} bytes", metrics.network_out);
    
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
            println!("  Name: {}", info.name);
            println!("  CPU Usage: {:.1}%", info.cpu_percent);
            println!("  Memory: {:.2} MB", info.memory_bytes as f64 / (1024.0 * 1024.0));
            println!("  Status: {}", info.status);
        }
        Err(e) => println!("  Error getting process info: {}", e),
    }
    
    // Get top processes by CPU
    println!("\nTop Processes by CPU Usage:");
    match get_top_processes_by_cpu() {
        Ok(processes) => {
            for (i, process) in processes.iter().enumerate().take(5) {
                println!("  {}. {} (PID {}): {:.1}%", i + 1, process.name, process.pid, process.cpu_percent);
            }
        }
        Err(e) => println!("  Error getting top processes: {}", e),
    }
    
    // Get top processes by memory
    println!("\nTop Processes by Memory Usage:");
    match get_top_processes_by_memory() {
        Ok(processes) => {
            for (i, process) in processes.iter().enumerate().take(5) {
                println!("  {}. {} (PID {}): {:.2} MB", 
                         i + 1, process.name, process.pid, process.memory_bytes as f64 / (1024.0 * 1024.0));
            }
        }
        Err(e) => println!("  Error getting top processes: {}", e),
    }
    
    println!();
    Ok(())
}

fn demo_caching_performance() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Caching Performance Demo ===");
    
    println!("Note: SystemMonitor caching methods not yet implemented");
    
    // Simple performance test using direct function calls
    println!("Testing function call performance...");
    
    // First call
    let start = std::time::Instant::now();
    let _usage1 = get_resource_usage()?;
    let first_call_time = start.elapsed();
    
    // Second call
    let start = std::time::Instant::now();
    let _usage2 = get_resource_usage()?;
    let second_call_time = start.elapsed();
    
    println!("First call: {:.3}ms", first_call_time.as_secs_f64() * 1000.0);
    println!("Second call: {:.3}ms", second_call_time.as_secs_f64() * 1000.0);
    
    println!();
    Ok(())
}

fn demo_continuous_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Continuous Monitoring Demo ===");
    println!("Simulating continuous monitoring for 5 seconds...");
    
    let start_time = std::time::Instant::now();
    let mut sample_count = 0;
    
    // Simulate continuous monitoring with a loop
    while start_time.elapsed() < Duration::from_secs(5) {
        sample_count += 1;
        let elapsed = start_time.elapsed();
        
        let usage = get_resource_usage()?;
        println!("Sample {}: CPU {:.1}%, Memory {:.2} GB, Time {:.1}s",
                 sample_count,
                 usage.cpu_percent,
                 usage.memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
                 elapsed.as_secs_f64());
        
        thread::sleep(Duration::from_millis(500));
    }
    
    println!("Continuous monitoring completed. Total samples: {}\n", sample_count);
    Ok(())
}

fn demo_concurrent_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Concurrent Monitoring Demo ===");
    
    let mut handles = vec![];
    
    println!("Starting 4 monitoring threads...");
    
    for thread_id in 0..4 {
        let handle = thread::spawn(move || {
            for i in 0..3 {
                match get_resource_usage() {
                    Ok(usage) => {
                        println!("Thread {}, Sample {}: CPU {:.1}%, Memory {:.2} GB",
                                 thread_id, i + 1,
                                 usage.cpu_percent,
                                 usage.memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0));
                    }
                    Err(e) => println!("Thread {} error: {}", thread_id, e),
                }
                thread::sleep(Duration::from_millis(200));
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.join() {
            Ok(_) => (), // Thread completed successfully
            Err(_) => println!("Thread {} panicked", i),
        }
    }
    
    println!("All monitoring threads completed.");
    
    // Verify functionality with a final check
    let final_usage = get_resource_usage()?;
    println!("Final check - CPU: {:.1}%, Memory: {:.2} GB",
             final_usage.cpu_percent,
             final_usage.memory_bytes as f64 / (1024.0 * 1024.0 * 1024.0));
    
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
