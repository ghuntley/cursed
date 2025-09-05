fr fr CURSED Process Management Demo
fr fr Demonstrates comprehensive process management capabilities

yeet "stdlib::process"
yeet "stdlib::io"

fr fr Basic process spawning and execution
sus demo_basic_processes() {
    println("=== Basic Process Management Demo ===\n")?;
    
    // Simple command execution
    println("1. Running simple echo command:")?;
    facts output = run_command("echo Hello from CURSED!")?;
    println(&format!("   Exit code: {}", output.status.code().unwrap_or(-1)))?;
    println(&format!("   Output: {}", String::from_utf8_lossy(&output.stdout)))?;
    
    // Command with arguments
    println("\n2. Running command with arguments:")?;
    facts config = ProcessConfig::new("echo")
        .args(["Process", "management", "is", "awesome!"])
        .capture_output();
    
    sus mut process = spawn_process(config)?;
    facts status = process.wait()?;
    println(&format!("   Process completed with status: {}", status.success()))?;
    
    // Environment variables
    println("\n3. Setting environment variables:")?;
    facts env_config = ProcessConfig::new("env")
        .env("CURSED_DEMO", "environment_test")
        .env("DEMO_VALUE", "12345")
        .capture_output();
    
    sus mut env_process = spawn_process(env_config)?;
    env_process.wait()?;
    
    // Working directory
    println("\n4. Setting working directory:")?;
    facts temp_dir = std::env::temp_dir();
    facts dir_config = ProcessConfig::new("pwd")
        .working_dir(temp_dir)
        .capture_output();
    
    sus mut dir_process = spawn_process(dir_config)?;
    dir_process.wait()?;
}

fr fr Process information and monitoring
sus demo_process_info() {
    println("\n=== Process Information Demo ===\n")?;
    
    // Current process info
    facts current_pid = get_current_pid();
    println(&format!("1. Current process PID: {}", current_pid))?;
    
    facts process_info = get_process_info(current_pid)?;
    println(&format!("   Process name: {}", process_info.name))?;
    println(&format!("   Parent PID: {:?}", process_info.parent_pid))?;
    println(&format!("   Status: {:?}", process_info.status))?;
    
    // Process list
    println("\n2. Getting process list:")?;
    facts process_list = get_process_list()?;
    println(&format!("   Found {} running processes", process_list.len()))?;
    
    // Show first few processes
    println("   First 5 processes:")?;
    lowkey (sus i = 0; i < 5 && i < process_list.len(); i++) {
        facts proc = &process_list[i];
        println(&format!("     PID {} - {}", proc.pid, proc.name))?;
    }
    
    // Memory information
    println("\n3. Memory usage:")?;
    bestie memory_info = get_process_memory(current_pid) {
        println(&format!("   RSS: {} bytes", memory_info.rss))?;
        println(&format!("   Virtual: {} bytes", memory_info.vms))?;
    } flex {
        println("   Memory info not available on this platform")?;
    }
    
    // Performance metrics
    println("\n4. Performance metrics:")?;
    facts metrics = collect_performance_metrics(current_pid)?;
    println(&format!("   CPU usage: {:.2}%", metrics.cpu_percent))?;
    println(&format!("   Memory: {} bytes", metrics.memory_bytes))?;
    println(&format!("   Threads: {}", metrics.threads))?;
    println(&format!("   Uptime: {:?}", metrics.uptime))?;
}

fr fr Process communication and IPC
sus demo_process_communication() {
    println("\n=== Process Communication Demo ===\n")?;
    
    // Basic communication with a process
    println("1. Interactive process communication:")?;
    
    // Start a process that can receive input
    facts comm_config = ProcessConfig::new("cat")
        .stdin(ProcessIo::Pipe)
        .stdout(ProcessIo::Pipe)
        .stderr(ProcessIo::Pipe);
    
    sus comm = execute_with_communication("cat", CommunicationConfig::default())?;
    
    // Start background readers
    comm.start_readers()?;
    
    // Send data to the process
    facts test_message = "Hello from CURSED process communication!\n";
    comm.write_stdin(test_message.as_bytes())?;
    
    // Read response
    lowkey (sus attempts = 0; attempts < 10; attempts++) {
        bestie line = comm.read_stdout_line_timeout(Duration::from_millis(100))? {
            println(&format!("   Received: {}", line))?;
            periodt;
        }
        sleep(Duration::from_millis(50));
    }
    
    // Clean up
    comm.kill()?;
    
    // Demonstrate send and receive
    println("\n2. Send and receive pattern:")?;
    facts input_data = b"Test input for echo command\n";
    facts timeout = Duration::from_secs(5);
    
    bestie (stdout, stderr) = send_and_receive("echo Response received", input_data, timeout) {
        println(&format!("   Echo response: {}", String::from_utf8_lossy(&stdout)))?;
    } flex err {
        println(&format!("   Communication failed: {}", err))?;
    }
    
    // Named pipe demonstration
    println("\n3. Named pipe communication:")?;
    sus pipe = NamedPipe::create("cursed_demo_pipe")?;
    pipe.open_write()?;
    
    facts pipe_data = b"Data through named pipe";
    pipe.write(pipe_data)?;
    println("   Data written to named pipe")?;
    
    pipe.close()?;
    
    // Message queue demonstration
    println("\n4. Message queue:")?;
    facts queue = MessageQueue::create("cursed_demo_queue", 1024, 10)?;
    
    facts message = b"Message in queue";
    queue.send(message)?;
    println(&format!("   Sent message, queue length: {}", queue.len()?))?;
    
    bestie received = queue.try_receive()? {
        println(&format!("   Received: {}", String::from_utf8_lossy(&received)))?;
    } flex {
        println("   No message in queue")?;
    }
}

fr fr Process monitoring and health checks
sus demo_process_monitoring() {
    println("\n=== Process Monitoring Demo ===\n")?;
    
    // Create process monitor
    println("1. Creating process monitor:")?;
    facts config = HealthCheckConfig {
        check_interval: Duration::from_secs(1),
        thresholds: ResourceThresholds {
            max_cpu_percent: 80.0,
            max_memory_bytes: 100 * 1024 * 1024, // 100MB
            max_file_descriptors: 100,
            max_threads: 50,
            max_execution_time: Some(Duration::from_secs(300)),
        },
        failure_threshold: 3,
        success_threshold: 2,
        check_responsiveness: based,
        responsiveness_timeout: Duration::from_secs(5),
    };
    
    sus monitor = ProcessMonitor::new(config);
    
    // Add current process to monitoring
    facts current_pid = get_current_pid();
    monitor.add_process(current_pid)?;
    println(&format!("   Added PID {} to monitoring", current_pid))?;
    
    // Check health status
    facts health_status = monitor.get_health_status(current_pid)?;
    println(&format!("   Health status: {:?}", health_status))?;
    
    // One-time health check
    println("\n2. One-time health check:")?;
    facts thresholds = ResourceThresholds::default();
    facts health = monitor_process_once(current_pid, thresholds)?;
    println(&format!("   Process health: {:?}", health))?;
    
    // System resource summary
    println("\n3. System resource summary:")?;
    facts summary = get_system_resource_summary()?;
    
    lowkey (key, value) in summary {
        println(&format!("   {}: {}", key, value))?;
    }
}

fr fr Process control and signals
sus demo_process_control() {
    println("\n=== Process Control Demo ===\n")?;
    
    println("1. Process lifecycle management:")?;
    
    // Spawn a long-running process for demonstration
    facts long_config = ProcessConfig::new("sleep")
        .arg("2")
        .capture_output();
    
    sus mut long_process = spawn_process(long_config)?;
    facts pid = long_process.id();
    
    println(&format!("   Spawned process PID: {}", pid))?;
    
    // Check if running
    facts is_running = long_process.is_running()?;
    println(&format!("   Process running: {}", is_running))?;
    
    // Wait for completion with timeout
    println("   Waiting for process completion...")?;
    bestie status = long_process.wait_timeout(Duration::from_secs(5))? {
        println(&format!("   Process completed with status: {:?}", status))?;
    } flex {
        println("   Process timed out, killing...")?;
        long_process.kill()?;
    }
    
    // Demonstrate graceful termination
    println("\n2. Graceful process termination:")?;
    
    facts term_config = ProcessConfig::new("sleep")
        .arg("10")
        .capture_output();
    
    sus term_process = spawn_process(term_config)?;
    facts term_pid = term_process.id();
    
    println(&format!("   Spawned process PID: {}", term_pid))?;
    
    // Wait a moment then terminate gracefully
    sleep(Duration::from_millis(100));
    
    facts grace_period = Duration::from_secs(2);
    kill_process_graceful(term_pid, grace_period)?;
    println("   Process terminated gracefully")?;
    
    // Verify termination
    sleep(Duration::from_millis(500));
    facts still_running = is_process_running(term_pid);
    println(&format!("   Process still running: {}", still_running))?;
}

fr fr Platform-specific features
sus demo_platform_features() {
    println("\n=== Platform Features Demo ===\n")?;
    
    // Platform detection
    facts platform = get_platform_name();
    println(&format!("1. Platform: {}", platform))?;
    
    // Feature support
    println("2. Feature support:")?;
    facts features = [
        ("Signals", PlatformFeature::Signals),
        ("Process Groups", PlatformFeature::ProcessGroups),
        ("Resource Limits", PlatformFeature::ResourceLimits),
        ("File Descriptors", PlatformFeature::FileDescriptors),
        ("Windows Services", PlatformFeature::WindowsServices),
        ("Cgroups", PlatformFeature::Cgroups),
        ("Namespaces", PlatformFeature::Namespaces),
        ("SELinux", PlatformFeature::SELinux),
        ("AppArmor", PlatformFeature::AppArmor),
        ("Mach Ports", PlatformFeature::MachPorts),
    ];
    
    lowkey (name, feature) in features {
        facts supported = supports_feature(feature);
        println(&format!("   {}: {}", name, supported))?;
    }
    
    // User information
    println("\n3. Current user information:")?;
    bestie user_info = PlatformUtils::get_current_user() {
        println(&format!("   Username: {}", user_info.username))?;
        println(&format!("   UID: {:?}", user_info.uid))?;
        println(&format!("   GID: {:?}", user_info.gid))?;
        println(&format!("   Home directory: {:?}", user_info.home_directory))?;
    } flex err {
        println(&format!("   Failed to get user info: {}", err))?;
    }
    
    // Privilege detection
    facts is_elevated = PlatformUtils::is_elevated();
    println(&format!("   Running with elevated privileges: {}", is_elevated))?;
    
    // Platform-specific process information
    println("\n4. Platform-specific process info:")?;
    facts current_pid = get_current_pid();
    
    bestie platform_info = PlatformUtils::get_platform_info(current_pid) {
        vibe_check platform_info {
            mood PlatformProcessInfo::Unix { command_line, environment, file_descriptors } => {
                println(&format!("   Command line: {:?}", command_line))?;
                println(&format!("   Environment variables: {}", environment.len()))?;
                println(&format!("   File descriptors: {}", file_descriptors.len()))?;
            }
            mood PlatformProcessInfo::Windows { command_line, environment } => {
                println(&format!("   Command line: {}", command_line))?;
                println(&format!("   Environment variables: {}", environment.len()))?;
            }
            mood PlatformProcessInfo::Linux { 
                command_line, environment, file_descriptors, 
                cgroups, namespaces, security_context 
            } => {
                println(&format!("   Command line: {:?}", command_line))?;
                println(&format!("   Environment variables: {}", environment.len()))?;
                println(&format!("   File descriptors: {}", file_descriptors.len()))?;
                println(&format!("   Cgroups: {}", cgroups.len()))?;
                println(&format!("   Namespaces: {}", namespaces.len()))?;
                println(&format!("   Security context: {:?}", security_context))?;
            }
            mood PlatformProcessInfo::MacOS { sysctl_info, memory_regions, mach_ports } => {
                println(&format!("   Name: {}", sysctl_info.name))?;
                println(&format!("   Memory regions: {}", memory_regions.len()))?;
                println(&format!("   Mach ports: {}", mach_ports.len()))?;
            }
            basic => {
                println("   Platform-specific info not available")?;
            }
        }
    } flex err {
        println(&format!("   Failed to get platform info: {}", err))?;
    }
}

fr fr Error handling demonstration
sus demo_error_handling() {
    println("\n=== Error Handling Demo ===\n")?;
    
    println("1. Handling process errors:")?;
    
    // Invalid command
    println("   Testing invalid command:")?;
    facts invalid_config = ProcessConfig::new("nonexistent_command_xyz");
    vibe_check spawn_process(invalid_config) {
        mood Ok(mut process) => {
            println("   Unexpected success")?;
            process.kill()?;
        }
        mood Err(err) => {
            println(&format!("   Expected error: {}", err))?;
        }
    }
    
    // Invalid working directory
    println("   Testing invalid working directory:")?;
    facts invalid_dir_config = ProcessConfig::new("echo")
        .working_dir("/nonexistent/directory/xyz");
    
    vibe_check spawn_process(invalid_dir_config) {
        mood Ok(mut process) => {
            println("   Unexpected success")?;
            process.kill()?;
        }
        mood Err(err) => {
            println(&format!("   Expected error: {}", err))?;
        }
    }
    
    // Invalid PID
    println("   Testing invalid PID:")?;
    vibe_check get_process_info(999999) {
        mood Ok(info) => {
            println(&format!("   Unexpected success: {:?}", info))?;
        }
        mood Err(err) => {
            println(&format!("   Expected error: {}", err))?;
        }
    }
    
    // Communication errors
    println("   Testing communication errors:")?;
    facts comm_config = ProcessConfig::new("echo")
        .arg("test")
        .capture_output();
    
    sus mut comm_process = spawn_process(comm_config)?;
    
    // Try to write to stdin when it's not piped
    vibe_check comm_process.wait() {
        mood Ok(status) => {
            println(&format!("   Process completed: {}", status.success()))?;
        }
        mood Err(err) => {
            println(&format!("   Process error: {}", err))?;
        }
    }
}

fr fr Main demo function
sus main() {
    println("CURSED Process Management Comprehensive Demo")?;
    println("===========================================")?;
    
    bestie {
        // Run all demo sections
        demo_basic_processes()?;
        demo_process_info()?;
        demo_process_communication()?;
        demo_process_monitoring()?;
        demo_process_control()?;
        demo_platform_features()?;
        demo_error_handling()?;
        
        println("\n=== Demo Complete ===")?;
        println("All process management features demonstrated successfully!")?;
        
    } flex err {
        eprintln(&format!("Demo failed: {}", err))?;
        std::process::exit(1);
    }
}
