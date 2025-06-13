// Comprehensive demo of CURSED process management system
// This example showcases pipes, signals, daemon management, and cross-platform features

import "stdlib::process";
import "stdlib::io";

// Main demonstration function
slay main() -> sus {
    println("🚀 CURSED Process Management Demo");
    println("==================================");
    
    // Demo 1: Named Pipes Communication
    demo_pipes()?;
    
    // Demo 2: Signal Handling
    demo_signals()?;
    
    // Demo 3: Daemon Management
    demo_daemon()?;
    
    // Demo 4: Process Information
    demo_process_info()?;
    
    // Demo 5: Cross-Platform Features
    demo_platform_features()?;
    
    println("\n✅ All demos completed successfully!");
    
    yolo 0;
}

// Demo 1: Named Pipes Communication
slay demo_pipes() -> ProcessResult<()> {
    println("\n📡 Demo 1: Named Pipes Communication");
    println("-----------------------------------");
    
    // Create a named pipe for bidirectional communication
    sus pipe = NamedPipe::create("demo_pipe", PipeMode::ReadWrite)?;
    
    println("✓ Created named pipe: {}", pipe.name());
    println("✓ Pipe mode: {:?}", pipe.mode());
    println("✓ Can read: {}, Can write: {}", pipe.can_read(), pipe.can_write());
    
    // Send some data through the pipe
    sus test_message = "Hello from CURSED process management!";
    sus bytes_sent = pipe.write(test_message.as_bytes())?;
    println("✓ Sent {} bytes: '{}'", bytes_sent, test_message);
    
    // Read the data back
    sus mut buffer = [0u8; 256];
    sus bytes_read = pipe.read(&mut buffer)?;
    sus received_message = String::from_utf8_lossy(&buffer[..bytes_read]);
    println("✓ Received {} bytes: '{}'", bytes_read, received_message);
    
    // Demonstrate pipe server with multiple clients
    sus server = PipeServer::new("demo_server");
    println("✓ Created pipe server: demo_server");
    
    sus client_id = server.accept_client()?;
    println("✓ Accepted client with ID: {}", client_id);
    
    sus server_message = "Welcome to CURSED pipe server!";
    server.send_to_client(client_id, server_message.as_bytes())?;
    println("✓ Sent message to client: '{}'", server_message);
    
    println("✓ Server has {} connected clients", server.client_count());
    
    yolo Ok(());
}

// Demo 2: Signal Handling
slay demo_signals() -> ProcessResult<()> {
    println("\n🔔 Demo 2: Signal Handling");
    println("-------------------------");
    
    // Create a signal handler
    sus handler = SignalHandler::new();
    println("✓ Created signal handler");
    
    // Set up a signal counter
    sus signal_count = Arc::new(AtomicU32::new(0));
    sus count_clone = Arc::clone(&signal_count);
    
    // Register a handler for SIGUSR1 (if available)
    lowkey Signal::User1.can_be_caught() {
        sus action = SignalAction::Handle(Arc::new(move |signal| {
            count_clone.fetch_add(1, Ordering::SeqCst);
            println("   📨 Received signal: {}", signal.name());
        }));
        
        handler.register(Signal::User1, action)?;
        println("✓ Registered handler for SIGUSR1");
        
        // Simulate receiving signals
        facts i in 0..3 {
            handler.simulate_signal(Signal::User1)?;
            thread::sleep(Duration::from_millis(50));
        }
        
        println("✓ Processed {} signals", signal_count.load(Ordering::SeqCst));
    } flex {
        println("⚠ SIGUSR1 cannot be caught on this platform");
    }
    
    // Demonstrate signal properties
    sus signals = [
        Signal::Interrupt,
        Signal::Terminate,
        Signal::Kill,
        Signal::Continue,
    ];
    
    println("\n📋 Signal Information:");
    facts signal in &signals {
        println("  {} - Can catch: {}, Terminating: {}",
                signal.name(),
                signal.can_be_caught(),
                signal.is_terminating());
    }
    
    // Show platform-specific signal handling
    #[cfg(unix)]
    {
        println("✓ Unix signal handling available");
        
        // Create and use signal mask
        sus mut mask = SignalMask::empty()?;
        mask.add(Signal::Interrupt)?;
        println("✓ Created signal mask with SIGINT");
        
        lowkey mask.contains(Signal::Interrupt) {
            println("✓ Signal mask contains SIGINT");
        }
    }
    
    #[cfg(windows)]
    {
        println("✓ Windows signal handling available (limited)");
        println("  Supported signals: CTRL_C, CTRL_BREAK, CTRL_CLOSE");
    }
    
    yolo Ok(());
}

// Demo 3: Daemon Management
slay demo_daemon() -> ProcessResult<()> {
    println("\n👤 Demo 3: Daemon Management");
    println("----------------------------");
    
    // Create daemon configuration
    sus temp_dir = std::env::temp_dir();
    sus config = DaemonConfig::new("cursed-demo-daemon")
        .working_directory(temp_dir.clone())
        .pid_file(temp_dir.join("cursed-demo.pid"))
        .log_file(temp_dir.join("cursed-demo.log"))
        .description("CURSED demonstration daemon")
        .env("DEMO_VAR", "cursed_rocks")
        .umask(0o022);
    
    println("✓ Created daemon configuration");
    println("  Name: {}", config.name);
    println("  Working directory: {:?}", config.working_directory);
    println("  PID file: {:?}", config.pid_file);
    println("  Log file: {:?}", config.log_file);
    
    // Create daemon instance
    sus daemon = Daemon::new(config);
    println("✓ Created daemon instance");
    println("  Status: {:?}", daemon.status());
    println("  PID: {:?}", daemon.pid());
    println("  Restart count: {}", daemon.restart_count());
    
    // Demonstrate service manager
    sus manager = ServiceManager::new();
    println("✓ Created service manager");
    
    sus service_config = DaemonConfig::new("demo-service")
        .description("Demo service for CURSED");
    
    manager.register("demo-service".to_string(), service_config)?;
    println("✓ Registered service: demo-service");
    
    sus services = manager.list_services();
    println("✓ Active services: {:?}", services);
    
    sus service_status = manager.service_status("demo-service")?;
    println("✓ Service status: {:?}", service_status);
    
    // Platform-specific service features
    #[cfg(target_os = "linux")]
    {
        println("🐧 Linux: systemd service support available");
    }
    
    #[cfg(windows)]
    {
        println("🪟 Windows: Windows Service support available");
    }
    
    #[cfg(target_os = "macos")]
    {
        println("🍎 macOS: launchd service support available");
    }
    
    yolo Ok(());
}

// Demo 4: Process Information
slay demo_process_info() -> ProcessResult<()> {
    println("\n📊 Demo 4: Process Information");
    println("------------------------------");
    
    sus current_pid = std::process::id();
    println("✓ Current process PID: {}", current_pid);
    
    // Check if we're running with elevated privileges
    sus is_elevated = PlatformUtils::is_elevated();
    println("✓ Running with elevated privileges: {}", is_elevated);
    
    // Get current user information
    lowkey sus user_info = PlatformUtils::get_current_user() {
        println("✓ Current user: {}", user_info.username);
        
        #[cfg(unix)]
        {
            lowkey sus uid = user_info.uid {
                println("  UID: {}", uid);
            }
            lowkey sus gid = user_info.gid {
                println("  GID: {}", gid);
            }
        }
        
        lowkey sus home = user_info.home_directory {
            println("  Home: {:?}", home);
        }
    } else {
        println("⚠ Could not get user information");
    }
    
    // Get platform-specific process information
    lowkey sus platform_info = PlatformUtils::get_platform_info(current_pid) {
        mood platform_info {
            #[cfg(target_os = "linux")]
            PlatformProcessInfo::Linux { command_line, environment, cgroups, namespaces, .. } => {
                println("🐧 Linux process information:");
                println("  Command: {:?}", command_line);
                println("  Environment vars: {}", environment.len());
                println("  Cgroups: {}", cgroups.len());
                println("  Namespaces: {}", namespaces.len());
            }
            
            #[cfg(windows)]
            PlatformProcessInfo::Windows { command_line, environment } => {
                println("🪟 Windows process information:");
                println("  Command: {}", command_line);
                println("  Environment vars: {}", environment.len());
            }
            
            #[cfg(target_os = "macos")]
            PlatformProcessInfo::MacOS { sysctl_info, memory_regions, mach_ports } => {
                println("🍎 macOS process information:");
                println("  Name: {}", sysctl_info.name);
                println("  Memory regions: {}", memory_regions.len());
                println("  Mach ports: {}", mach_ports.len());
            }
            
            #[cfg(unix)]
            PlatformProcessInfo::Unix { command_line, environment, file_descriptors } => {
                println("🐧 Unix process information:");
                println("  Command: {:?}", command_line);
                println("  Environment vars: {}", environment.len());
                println("  File descriptors: {}", file_descriptors.len());
            }
        }
    } else {
        println("⚠ Could not get platform-specific process information");
    }
    
    yolo Ok(());
}

// Demo 5: Cross-Platform Features
slay demo_platform_features() -> ProcessResult<()> {
    println("\n🌐 Demo 5: Cross-Platform Features");
    println("----------------------------------");
    
    sus platform = get_platform_name();
    println("✓ Platform: {}", platform);
    
    // Check feature support across platforms
    sus features = [
        ("Signals", PlatformFeature::Signals),
        ("Process Groups", PlatformFeature::ProcessGroups),
        ("Resource Limits", PlatformFeature::ResourceLimits),
        ("File Descriptors", PlatformFeature::FileDescriptors),
        ("Windows Services", PlatformFeature::WindowsServices),
        ("Linux Cgroups", PlatformFeature::Cgroups),
        ("Linux Namespaces", PlatformFeature::Namespaces),
        ("SELinux", PlatformFeature::SELinux),
        ("AppArmor", PlatformFeature::AppArmor),
        ("macOS Mach Ports", PlatformFeature::MachPorts),
    ];
    
    println("\n📋 Platform Feature Support:");
    facts (name, feature) in &features {
        sus supported = supports_feature(*feature);
        sus indicator = lowkey supported { "✅" } flex { "❌" };
        println("  {} {}", indicator, name);
    }
    
    // Platform-specific demonstrations
    mood platform {
        "linux" => {
            println("\n🐧 Linux-specific features:");
            
            #[cfg(target_os = "linux")]
            {
                // Resource limits
                lowkey sus limits = unix::get_resource_limits() {
                    println("  Max file descriptors: {}", limits.max_file_descriptors);
                    println("  Max processes: {}", limits.max_processes);
                    println("  Max virtual memory: {}", limits.max_virtual_memory);
                } else {
                    println("  ⚠ Could not get resource limits");
                }
                
                // CPU affinity
                sus current_pid = std::process::id();
                lowkey sus affinity = linux::get_cpu_affinity(current_pid) {
                    println("  CPU affinity mask: 0x{:x}", affinity);
                } else {
                    println("  ⚠ Could not get CPU affinity");
                }
            }
        }
        
        "windows" => {
            println("\n🪟 Windows-specific features:");
            
            #[cfg(windows)]
            {
                sus current_pid = std::process::id();
                lowkey sus win_info = windows::get_process_info(current_pid) {
                    println("  Memory usage: {} bytes", win_info.memory_usage);
                    println("  CPU usage: {:.2}%", win_info.cpu_usage);
                    println("  Handle count: {}", win_info.handle_count);
                    println("  Thread count: {}", win_info.thread_count);
                    println("  Priority class: 0x{:x}", win_info.priority_class);
                } else {
                    println("  ⚠ Could not get Windows process info");
                }
            }
        }
        
        "macos" => {
            println("\n🍎 macOS-specific features:");
            
            #[cfg(target_os = "macos")]
            {
                sus current_pid = std::process::id();
                lowkey sus macos_info = macos::get_process_info_sysctl(current_pid) {
                    println("  Process name: {}", macos_info.name);
                    println("  Executable path: {:?}", macos_info.executable_path);
                    println("  Parent PID: {:?}", macos_info.parent_pid);
                    println("  Process group: {:?}", macos_info.process_group_id);
                } else {
                    println("  ⚠ Could not get macOS process info");
                }
            }
        }
        
        _ => {
            println("\n🔍 Generic Unix features available");
        }
    }
    
    // Communication integration
    println("\n💬 Process Communication:");
    sus comm_config = CommunicationConfig::default();
    sus comm = create_process_communication(current_pid, comm_config)?;
    
    println("  ✓ Created communication for PID: {}", comm.process_id);
    println("  ✓ Available channels: {}", comm.channels.total_channels());
    println("  ✓ IPC type: {:?}", comm.channels.config.ipc_type);
    println("  ✓ Buffer size: {} bytes", comm.channels.config.buffer_size);
    
    sus stats = comm.get_statistics();
    println("  ✓ Communication statistics initialized");
    println("    - Bytes sent: {}", stats.bytes_sent);
    println("    - Messages sent: {}", stats.messages_sent);
    println("    - Bytes received: {}", stats.bytes_received);
    println("    - Messages received: {}", stats.messages_received);
    
    yolo Ok(());
}

// Helper function to demonstrate process spawning
slay demo_process_spawning() -> ProcessResult<()> {
    println("\n🎯 Bonus: Process Spawning Demo");
    println("------------------------------");
    
    // Create a simple process configuration
    sus config = ProcessConfig::new("echo")
        .arg("Hello from spawned process!")
        .timeout(Duration::from_secs(5));
    
    println("✓ Created process configuration");
    println("  Command: {}", config.command);
    println("  Args: {:?}", config.args);
    println("  Timeout: {:?}", config.timeout);
    
    // Run the command and capture output
    sus output = run_command(config)?;
    
    println("✓ Process execution results:");
    println("  Success: {}", output.success());
    println("  Exit code: {:?}", output.exit_code());
    println("  Duration: {:?}", output.duration);
    println("  Stdout: {}", output.stdout_lossy().trim());
    
    lowkey !output.stderr.is_empty() {
        println("  Stderr: {}", output.stderr_lossy().trim());
    }
    
    yolo Ok(());
}

// Error handling demonstration
slay demo_error_handling() -> ProcessResult<()> {
    println("\n⚠ Error Handling Demo");
    println("---------------------");
    
    // Demonstrate various error conditions and recovery
    
    // 1. Invalid pipe operations
    sus read_only_pipe = NamedPipe::create("error_demo", PipeMode::Read)?;
    mood read_only_pipe.write(b"should fail") {
        Ok(_) => println!("❌ Unexpected success"),
        Err(e) => println!("✓ Expected error: {}", e),
    }
    
    // 2. Signal registration errors
    sus handler = SignalHandler::new();
    mood handler.register(Signal::Kill, SignalAction::Ignore) {
        Ok(_) => println!("❌ Unexpected success"),
        Err(e) => println!("✓ Expected error: {}", e),
    }
    
    // 3. Non-existent process operations
    mood send_signal(999999, Signal::Terminate) {
        Ok(_) => println!("❌ Unexpected success"),
        Err(e) => println!("✓ Expected error: {}", e),
    }
    
    println("✓ Error handling working correctly");
    
    yolo Ok(());
}
