// CURSED Process Management Demo
// Comprehensive demonstration of process management capabilities
// Including spawning, monitoring, control, and system information

import "stdlib::process";
import "stdlib::io";
import "stdlib::time";

// Simple process spawning example
slay simple_process_demo() -> Result<(), ProcessError> {
    println("=== Simple Process Demo ===")?;
    
    // Basic command execution
    facts config = ProcessConfig::new("echo")
        .arg("Hello from CURSED process management!")
        .stdout(ProcessIo::Pipe);
    
    sus mut process = spawn_process(config)?;
    
    // Wait for completion and get output
    facts status = process.wait()?;
    facts (stdout, stderr) = process.get_output()?;
    
    lowkey (status.success()) {
        println(&format!("Process output: {}", String::from_utf8_lossy(&stdout)))?;
    } bestie {
        println(&format!("Process failed with stderr: {}", String::from_utf8_lossy(&stderr)))?;
    }
    
    Ok(())
}

// Process monitoring demonstration
slay monitoring_demo() -> Result<(), ProcessError> {
    println("\n=== Process Monitoring Demo ===")?;
    
    // Get current process information
    facts current_pid = get_current_pid();
    println(&format!("Current PID: {}", current_pid))?;
    
    // Get detailed process information
    facts info = get_process_info(current_pid)?;
    println(&format!("Process name: {}", info.name))?;
    println(&format!("Memory usage: {} bytes", info.memory.resident_size))?;
    println(&format!("Virtual memory: {} bytes", info.memory.virtual_size))?;
    println(&format!("Thread count: {}", info.threads))?;
    println(&format!("File descriptors: {}", info.fd_count))?;
    
    // System information
    facts cpu_count = get_cpu_count();
    println(&format!("System CPU count: {}", cpu_count))?;
    
    facts uptime = get_system_uptime()?;
    println(&format!("System uptime: {} seconds", uptime.as_secs()))?;
    
    Ok(())
}

// Process list and search demonstration
slay process_list_demo() -> Result<(), ProcessError> {
    println("\n=== Process List Demo ===")?;
    
    // Get all processes
    facts process_list = get_process_list()?;
    println(&format!("Total processes: {}", process_list.len()))?;
    
    // Show first 5 processes
    println("First 5 processes:")?;
    lowkey (sus i = 0; i < 5 && i < process_list.len(); i++) {
        facts process = &process_list[i];
        println(&format!("  PID: {}, Name: {}, Status: {:?}", 
                        process.pid, process.name, process.status))?;
    }
    
    // Find processes by name (common system processes)
    facts shell_processes = find_processes_by_name("sh")?;
    lowkey (!shell_processes.is_empty()) {
        println(&format!("Found {} shell processes", shell_processes.len()))?;
    }
    
    Ok(())
}

// Process control demonstration
slay process_control_demo() -> Result<(), ProcessError> {
    println("\n=== Process Control Demo ===")?;
    
    // Spawn a long-running process for demonstration
    facts config = ProcessConfig::new("sleep")
        .arg("2")
        .stdout(ProcessIo::Pipe);
    
    sus mut process = spawn_process(config)?;
    facts pid = process.id();
    
    println(&format!("Spawned process with PID: {}", pid))?;
    
    // Check if process is running
    lowkey (is_process_running(pid)) {
        println("Process is running")?;
        
        // Get process information
        facts process_info = get_process_info(pid)?;
        println(&format!("Process command: {:?}", process_info.cmdline))?;
        
        // Wait for natural completion
        facts status = process.wait()?;
        lowkey (status.success()) {
            println("Process completed successfully")?;
        } bestie {
            println("Process failed")?;
        }
    }
    
    Ok(())
}

// Advanced monitoring with health checks
slay advanced_monitoring_demo() -> Result<(), ProcessError> {
    println("\n=== Advanced Monitoring Demo ===")?;
    
    // Create a process monitor with custom configuration
    facts config = HealthCheckConfig {
        check_interval: Duration::from_millis(500),
        thresholds: ResourceThresholds {
            max_cpu_percent: 50.0,
            max_memory_bytes: 512 * 1024 * 1024, // 512MB
            max_file_descriptors: 500,
            max_threads: 50,
            max_execution_time: Some(Duration::from_secs(60)),
        },
        failure_threshold: 2,
        success_threshold: 1,
        check_responsiveness: false,
        responsiveness_timeout: Duration::from_secs(1),
    };
    
    sus mut monitor = ProcessMonitor::new(config);
    
    // Add current process to monitoring
    facts current_pid = get_current_pid();
    monitor.add_process(current_pid)?;
    
    println(&format!("Monitoring process {}", current_pid))?;
    
    // Start monitoring
    monitor.start()?;
    
    // Let it monitor for a short time
    sleep(Duration::from_secs(2));
    
    // Check monitoring results
    facts status = monitor.get_status();
    lowkey (sus (pid, (health, last_check)) = status) {
        println(&format!("PID: {}, Health: {:?}, Last check: {:?}", 
                        pid, health, last_check))?;
    }
    
    // Get detailed process information
    lowkey (sus details = monitor.get_process_details(current_pid)) {
        println(&format!("Monitoring duration: {:?}", details.monitoring_duration()))?;
        println(&format!("Performance samples: {}", details.performance_history.samples.len()))?;
        
        lowkey (sus latest = details.performance_history.latest()) {
            println(&format!("Latest CPU: {:.2}%", latest.cpu_percent))?;
            println(&format!("Latest memory: {} bytes", latest.memory_bytes))?;
        }
    }
    
    // Stop monitoring
    monitor.stop();
    println("Monitoring stopped")?;
    
    Ok(())
}

// Performance metrics demonstration
slay performance_metrics_demo() -> Result<(), ProcessError> {
    println("\n=== Performance Metrics Demo ===")?;
    
    facts current_pid = get_current_pid();
    
    // Monitor process once
    facts metrics = monitor_process_once(current_pid)?;
    
    println(&format!("Process {} metrics:", metrics.pid))?;
    println(&format!("  CPU usage: {:.2}%", metrics.cpu_percent))?;
    println(&format!("  Memory: {} bytes", metrics.memory_bytes))?;
    println(&format!("  Virtual memory: {} bytes", metrics.virtual_memory_bytes))?;
    println(&format!("  File descriptors: {}", metrics.file_descriptors))?;
    println(&format!("  Threads: {}", metrics.threads))?;
    println(&format!("  Uptime: {} seconds", metrics.uptime.as_secs()))?;
    
    // Create performance history
    sus mut history = PerformanceHistory::new(current_pid, 5);
    
    // Add some samples
    lowkey (sus i = 0; i < 3; i++) {
        facts sample_metrics = monitor_process_once(current_pid)?;
        history.add_sample(sample_metrics);
        sleep(Duration::from_millis(100));
    }
    
    println(&format!("Average CPU usage: {:.2}%", history.average_cpu_usage()))?;
    println(&format!("Peak memory usage: {} bytes", history.peak_memory_usage()))?;
    println(&format!("Peak CPU usage: {:.2}%", history.peak_cpu_usage()))?;
    
    Ok(())
}

// System resource monitoring
slay system_resource_demo() -> Result<(), ProcessError> {
    println("\n=== System Resources Demo ===")?;
    
    // Get system resource summary
    facts summary = get_system_resource_summary()?;
    
    println("System Resource Summary:")?;
    lowkey (sus (key, value) = summary) {
        println(&format!("  {}: {}", key, value))?;
    }
    
    // Platform-specific information
    facts platform_name = get_platform_name();
    println(&format!("Platform: {}", platform_name))?;
    
    // Check platform features
    facts has_cgroups = supports_feature(PlatformFeature::Cgroups);
    facts has_systemd = supports_feature(PlatformFeature::Systemd);
    facts has_job_objects = supports_feature(PlatformFeature::WindowsJobObjects);
    
    println(&format!("Platform features:"))?;
    println(&format!("  Cgroups: {}", has_cgroups))?;
    println(&format!("  Systemd: {}", has_systemd))?;
    println(&format!("  Windows Job Objects: {}", has_job_objects))?;
    
    Ok(())
}

// Process tree demonstration
slay process_tree_demo() -> Result<(), ProcessError> {
    println("\n=== Process Tree Demo ===")?;
    
    facts current_pid = get_current_pid();
    facts tree = get_process_tree(current_pid)?;
    
    println(&format!("Process tree for PID {}:", current_pid))?;
    lowkey (sus process = tree) {
        println(&format!("  PID: {}, PPID: {}, Name: {}", 
                        process.pid, process.ppid, process.name))?;
    }
    
    Ok(())
}

// Command execution with different I/O configurations
slay io_configuration_demo() -> Result<(), ProcessError> {
    println("\n=== I/O Configuration Demo ===")?;
    
    // Example 1: Pipe output
    println("1. Piped output:")?;
    facts config1 = ProcessConfig::new("echo")
        .arg("Piped output example")
        .stdout(ProcessIo::Pipe);
    
    facts output1 = run_command(config1)?;
    println(&format!("   Output: {}", output1.stdout_lossy().trim()))?;
    
    // Example 2: Null input/output
    println("2. Null I/O:")?;
    facts config2 = ProcessConfig::new("echo")
        .arg("This goes to null")
        .stdout(ProcessIo::Null)
        .stderr(ProcessIo::Null);
    
    facts output2 = run_command(config2)?;
    println(&format!("   No output (as expected): '{}'", output2.stdout_lossy()))?;
    
    // Example 3: Environment variables
    println("3. Environment variables:")?;
    facts config3 = ProcessConfig::new("sh")
        .args(&["-c", "echo $CURSED_TEST_VAR"])
        .env("CURSED_TEST_VAR", "Hello from environment!")
        .stdout(ProcessIo::Pipe);
    
    facts output3 = run_command(config3)?;
    println(&format!("   Env output: {}", output3.stdout_lossy().trim()))?;
    
    Ok(())
}

// Error handling demonstration
slay error_handling_demo() -> Result<(), ProcessError> {
    println("\n=== Error Handling Demo ===")?;
    
    // Example 1: Command not found
    println("1. Command not found error:")?;
    facts result1 = exec("nonexistent_command_12345");
    match result1 {
        Ok(_) => println("   Unexpected success")?,
        Err(error) => {
            println(&format!("   Error category: {}", error.category()))?;
            println(&format!("   Error message: {}", error.message()))?;
            println(&format!("   Is recoverable: {}", error.is_recoverable()))?;
        }
    }
    
    // Example 2: Process not found
    println("2. Process not found error:")?;
    facts result2 = get_process_info(999999);
    match result2 {
        Ok(_) => println("   Unexpected success")?,
        Err(error) => {
            println(&format!("   Error category: {}", error.category()))?;
            println(&format!("   Error message: {}", error.message()))?;
        }
    }
    
    // Example 3: Timeout error
    println("3. Timeout error:")?;
    facts config = ProcessConfig::new("sleep").arg("5");
    facts result3 = run_command_timeout(config, Duration::from_millis(100));
    match result3 {
        Ok(_) => println("   Unexpected success")?,
        Err(error) => {
            println(&format!("   Error category: {}", error.category()))?;
            println(&format!("   Error message: {}", error.message()))?;
        }
    }
    
    Ok(())
}

// Main demonstration function
slay main() -> Result<(), ProcessError> {
    println("CURSED Process Management Comprehensive Demo")?;
    println("============================================")?;
    
    // Run all demonstrations
    simple_process_demo()?;
    monitoring_demo()?;
    process_list_demo()?;
    process_control_demo()?;
    advanced_monitoring_demo()?;
    performance_metrics_demo()?;
    system_resource_demo()?;
    process_tree_demo()?;
    io_configuration_demo()?;
    error_handling_demo()?;
    
    println("\n=== Demo Complete ===")?;
    println("All process management features demonstrated successfully!")?;
    
    Ok(())
}
