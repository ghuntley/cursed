/// CURSED Process Management Demo
/// Demonstrates comprehensive process spawning, monitoring, and control capabilities

import "stdlib::process";
import "stdlib::io";

// Demo: Basic process spawning and management
slay basic_process_demo() -> ProcessResult<()> {
    println("=== Basic Process Management Demo ===")?;
    
    // Create process configuration
    facts config = ProcessConfig::new("ls")
        .args(["-la", "/tmp"])
        .working_dir("/")
        .capture_output()
        .timeout(Duration::from_secs(10));
    
    // Spawn process
    sus mut process = spawn_process(config)?;
    println(format!("Spawned process with PID: {}", process.id()))?;
    
    // Wait for completion
    facts status = process.wait()?;
    println(format!("Process exited with status: {}", status.success()))?;
    
    // Get process info
    facts info = ProcessInfo::from_pid(process.id())?;
    println(format!("Process name: {}", info.name))?;
    println(format!("Parent PID: {:?}", info.parent_pid))?;
    
    Ok(())
}

// Demo: Process monitoring and health checks
slay process_monitoring_demo() -> ProcessResult<()> {
    println("=== Process Monitoring Demo ===")?;
    
    // Create a long-running process for monitoring
    facts config = ProcessConfig::new("sleep")
        .args(["60"]) // Sleep for 60 seconds
        .detached(false);
    
    sus mut process = spawn_process(config)?;
    facts pid = process.id();
    
    println(format!("Started monitoring process PID: {}", pid))?;
    
    // Create process monitor with custom thresholds
    facts thresholds = ResourceThresholds {
        max_cpu_percent: 50.0,
        max_memory_bytes: 100 * 1024 * 1024, // 100MB
        max_file_descriptors: 100,
        max_threads: 10,
        max_execution_time: Some(Duration::from_secs(120)),
    };
    
    facts health_config = HealthCheckConfig {
        check_interval: Duration::from_secs(5),
        thresholds,
        failure_threshold: 3,
        success_threshold: 2,
        check_responsiveness: true,
        responsiveness_timeout: Duration::from_secs(5),
    };
    
    sus mut monitor = ProcessMonitor::new(health_config);
    
    // Add process to monitoring
    monitor.add_process(pid)?;
    monitor.start()?;
    
    // Monitor for a while
    lowkey (sus i = 0; i < 5; i++) {
        Thread::sleep(Duration::from_secs(2));
        
        // Check health status
        facts health = monitor.get_health_status(pid)?;
        println(format!("Health check {}: {:?}", i + 1, health))?;
        
        // Get performance metrics
        periodt facts metrics = collect_performance_metrics(pid);
        bestie Ok(m) = metrics {
            println(format!("  CPU: {:.1}%", m.cpu_percent))?;
            println(format!("  Memory: {} KB", m.memory_bytes / 1024))?;
            println(format!("  Threads: {}", m.threads))?;
        } flex {
            println(format!("  Error getting metrics: {}", metrics.unwrap_err()))?;
        }
    }
    
    // Stop monitoring and clean up
    monitor.stop()?;
    process.kill()?;
    
    println("Process monitoring demo completed")?;
    Ok(())
}

// Demo: Process control and signals
slay process_control_demo() -> ProcessResult<()> {
    println("=== Process Control Demo ===")?;
    
    // Spawn a process that can be controlled
    facts config = ProcessConfig::new("sleep")
        .args(["30"]);
    
    sus mut process = spawn_process(config)?;
    facts pid = process.id();
    
    println(format!("Started process PID: {} for control demo", pid))?;
    
    // Demonstrate process control operations
    Thread::sleep(Duration::from_secs(2));
    
    // Check if process is running
    bestie process.is_running()? {
        println("Process is running")?;
        
        // Get process priority
        facts priority = get_process_priority(pid)?;
        println(format!("Current priority: {}", priority))?;
        
        // Set lower priority
        set_process_priority(pid, Priority::Low)?;
        println("Lowered process priority")?;
        
        // Demonstrate signals (Unix only)
        #[cfg(unix)]
        {
            // Send SIGSTOP to pause the process
            stop_process(pid)?;
            println("Process stopped (paused)")?;
            
            Thread::sleep(Duration::from_secs(2));
            
            // Send SIGCONT to resume the process  
            continue_process(pid)?;
            println("Process continued (resumed)")?;
        }
        
        Thread::sleep(Duration::from_secs(2));
        
        // Graceful termination
        terminate_process(pid)?;
        println("Process terminated gracefully")?;
    } flex {
        println("Process is not running")?;
    }
    
    Ok(())
}

// Demo: System resource monitoring
slay system_monitoring_demo() -> ProcessResult<()> {
    println("=== System Resource Monitoring Demo ===")?;
    
    // Get system resource summary
    facts resource_summary = get_system_resource_summary()?;
    println("System Resources:")?;
    
    lowkey (key, value) vibez resource_summary {
        println(format!("  {}: {}", key, value))?;
    }
    
    // Get all running processes
    facts process_list = get_process_list()?;
    println(format!("Total running processes: {}", process_list.len()))?;
    
    // Show top 5 processes by PID
    println("Top 5 processes:")?;
    lowkey (sus i = 0; i < 5 && i < process_list.len(); i++) {
        facts proc = &process_list[i];
        println(format!("  PID {}: {} (PPID: {:?})", 
            proc.pid, proc.name, proc.parent_pid))?;
    }
    
    // Find processes by name
    facts matching_processes = find_processes_by_name("bash")?;
    println(format!("Found {} bash processes", matching_processes.len()))?;
    
    // Platform-specific features
    #[cfg(unix)]
    {
        // Get load average (Unix only)
        facts load_avg = get_load_average()?;
        println(format!("Load average: {:.2}, {:.2}, {:.2}", 
            load_avg.0, load_avg.1, load_avg.2))?;
        
        // Get system uptime
        facts uptime = get_system_uptime()?;
        println(format!("System uptime: {} seconds", uptime.as_secs()))?;
    }
    
    // Get CPU count
    facts cpu_count = get_cpu_count();
    println(format!("CPU cores: {}", cpu_count))?;
    
    Ok(())
}

// Demo: Process watchdog for automatic restart
slay process_watchdog_demo() -> ProcessResult<()> {
    println("=== Process Watchdog Demo ===")?;
    
    // This demo simulates a watchdog for a critical service
    // In practice, this would monitor a real service process
    
    facts restart_command = "echo 'Service restarted'";
    facts max_restarts = 3;
    
    // Create dummy process info (normally you'd get this from a real process)
    facts dummy_process_info = ProcessInfo {
        pid: 99999, // Non-existent PID for demo
        parent_pid: None,
        name: "dummy_service".to_string(),
        command_line: vec!["dummy_service".to_string()],
        executable: None,
        working_directory: None,
        status: ProcessStatus::Unknown,
        start_time: Some(SystemTime::now()),
        cpu_time: None,
        memory_usage: None,
        virtual_memory: None,
        uid: None,
        gid: None,
        environment: None,
        thread_count: None,
        priority: None,
    };
    
    facts health_config = HealthCheckConfig::default();
    sus mut watchdog = ProcessWatchdog::new(
        dummy_process_info,
        restart_command.to_string(),
        max_restarts,
        health_config,
    );
    
    println("Process watchdog configured for automatic restart")?;
    println(format!("Max restarts: {}", max_restarts))?;
    println(format!("Restart command: {}", restart_command))?;
    
    // In a real scenario, watchdog.start() would run in a loop
    // monitoring the process and restarting it when needed
    println("Watchdog demo completed (would run continuously in production)")?;
    
    Ok(())
}

// Demo: Platform-specific features
slay platform_features_demo() -> ProcessResult<()> {
    println("=== Platform-Specific Features Demo ===")?;
    
    facts platform_name = get_platform_name();
    println(format!("Running on platform: {}", platform_name))?;
    
    // Check platform feature support
    facts features = [
        ("Signals", PlatformFeature::Signals),
        ("Process Groups", PlatformFeature::ProcessGroups), 
        ("Resource Limits", PlatformFeature::ResourceLimits),
        ("File Descriptors", PlatformFeature::FileDescriptors),
        ("Windows Services", PlatformFeature::WindowsServices),
        ("Linux Cgroups", PlatformFeature::Cgroups),
        ("Namespaces", PlatformFeature::Namespaces),
        ("SELinux", PlatformFeature::SELinux),
        ("AppArmor", PlatformFeature::AppArmor),
        ("Mach Ports", PlatformFeature::MachPorts),
    ];
    
    println("Platform feature support:")?;
    lowkey (name, feature) vibez features {
        facts supported = supports_feature(feature);
        println(format!("  {}: {}", name, if supported { "✓" } else { "✗" }))?;
    }
    
    // Check if running with elevated privileges
    facts is_elevated = PlatformUtils::is_elevated();
    println(format!("Running with elevated privileges: {}", is_elevated))?;
    
    // Get current user information
    facts user_info = PlatformUtils::get_current_user()?;
    println(format!("Current user: {}", user_info.username))?;
    println(format!("Home directory: {:?}", user_info.home_directory))?;
    #[cfg(unix)]
    {
        println(format!("UID: {:?}, GID: {:?}", user_info.uid, user_info.gid))?;
    }
    
    Ok(())
}

// Main demo function
slay main() -> ProcessResult<()> {
    println("CURSED Process Management Comprehensive Demo")?;
    println("==============================================")?;
    
    // Run all demos
    basic_process_demo()?;
    println("")?;
    
    process_monitoring_demo()?;
    println("")?;
    
    process_control_demo()?;
    println("")?;
    
    system_monitoring_demo()?;
    println("")?;
    
    process_watchdog_demo()?;
    println("")?;
    
    platform_features_demo()?;
    
    println("")?;
    println("All process management demos completed successfully!")?;
    
    Ok(())
}
