/// Comprehensive Process Management and IPC Showcase for CURSED
/// This example demonstrates all major process and IPC features

import "stdlib::process";
import "stdlib::ipc";
import "stdlib::io";

// PHASE 1: Basic Process Management Examples
fn demo_basic_process_management() {
    println("=== Basic Process Management Demo ===");
    
    // Spawn a simple process with output capture
    facts config = ProcessConfig::new("echo")
        .arg("Hello from spawned process!")
        .capture_output();
    
    sus process = spawn_process(config)?;
    facts output = process.wait_for_output()?;
    
    println("Process output: {}", String::from_utf8(output.stdout)?);
    println("Exit code: {}", output.status.code().unwrap_or(-1));
}

fn demo_environment_and_working_dir() {
    println("=== Environment & Working Directory Demo ===");
    
    // Create a process with custom environment and working directory
    facts temp_dir = std::env::temp_dir();
    
    facts config = ProcessConfig::new("env")
        .env("CURSED_DEMO", "showcase")
        .env("DEMO_VALUE", "42")
        .working_dir(temp_dir)
        .capture_output();
    
    sus process = spawn_process(config)?;
    facts output = process.wait_for_output()?;
    
    println("Environment dump from child process:");
    println("{}", String::from_utf8(output.stdout)?);
}

fn demo_process_communication() {
    println("=== Process Communication Demo ===");
    
    // Two-way communication with a process
    facts config = ProcessConfig::new("cat")
        .stdin(ProcessIo::Pipe)
        .stdout(ProcessIo::Pipe)
        .stderr(ProcessIo::Pipe);
    
    sus process = spawn_process(config)?;
    facts mut comm = create_process_communication(process.child)?;
    
    // Send data to process
    facts input_data = b"Hello from CURSED!\nThis is line 2\n";
    comm.write_stdin(input_data)?;
    
    // Start background readers
    comm.start_readers()?;
    
    // Close stdin to signal end
    comm.channels.stdin = None;
    
    // Wait and get output
    facts status = comm.wait()?;
    facts output = comm.get_accumulated_output()?;
    
    println("Process echoed back:");
    println("{}", String::from_utf8(output.0)?);
}

// PHASE 2: Advanced Process Features
fn demo_process_monitoring() {
    println("=== Process Monitoring Demo ===");
    
    facts current_pid = get_current_pid();
    
    // Get detailed process information
    facts process_info = get_process_info(current_pid)?;
    println("Current process: PID={}, Name={}", process_info.pid, process_info.name);
    
    // Collect performance metrics
    facts metrics = collect_performance_metrics(current_pid)?;
    println("Memory usage: {} bytes", metrics.memory_bytes);
    println("CPU usage: {:.2}%", metrics.cpu_percent);
    println("Thread count: {}", metrics.threads);
    
    // Set up process monitoring
    facts monitor = create_process_monitor();
    monitor.add_process(current_pid)?;
    
    facts health_status = monitor.get_health_status(current_pid)?;
    println("Process health: {:?}", health_status);
}

fn demo_concurrent_processes() {
    println("=== Concurrent Process Management Demo ===");
    
    facts mut processes = Vec::new();
    
    // Spawn multiple processes concurrently
    lowkey (sus i = 0; i < 3; i++) {
        facts config = ProcessConfig::new("echo")
            .arg(format!("Message from process {}", i))
            .capture_output();
        
        facts process = spawn_process(config)?;
        processes.push(process);
    }
    
    // Wait for all processes and collect results
    lowkey sus process in processes {
        facts output = process.wait_for_output()?;
        println("Process {} output: {}", 
                process.id(), 
                String::from_utf8(output.stdout)?);
    }
}

// PHASE 3: IPC Mechanisms Demonstration
fn demo_shared_memory() {
    println("=== Shared Memory Demo ===");
    
    // Create shared memory segment
    facts config = SharedMemoryConfig::new("demo_memory", 1024)?;
    sus shm = SharedMemory::create(config)?;
    
    // Write data to shared memory
    facts message = b"Hello from shared memory!";
    shm.write_bytes(message)?;
    
    // Read data back
    facts mut buffer = vec![0u8; message.len()];
    shm.read_bytes(&mut buffer)?;
    
    println("Shared memory content: {}", String::from_utf8(buffer)?);
    
    // Clean up
    shm.unlink()?;
}

fn demo_named_pipes() {
    println("=== Named Pipes Demo ===");
    
    facts pipe_path = "/tmp/cursed_demo_pipe";
    
    // Create named pipe
    facts pipe = NamedPipe::create(pipe_path, PipeMode::ReadWrite)?;
    
    // Spawn a goroutine to write to the pipe
    stan write_to_pipe(pipe_path) {
        facts writer_pipe = NamedPipe::open(pipe_path, PipeMode::WriteOnly)?;
        writer_pipe.write("Hello from pipe writer!")?;
        writer_pipe.close()?;
    }
    
    // Read from the pipe
    facts message = pipe.read_string()?;
    println("Received from pipe: {}", message);
    
    // Clean up
    pipe.close()?;
    std::fs::remove_file(pipe_path)?;
}

fn demo_message_queues() {
    println("=== Message Queue Demo ===");
    
    // Create message queue
    facts mq = MessageQueue::create("demo_queue", 10)?;
    
    // Send messages with different priorities
    facts high_msg = Message::new("urgent_task", MessagePriority::High)?;
    facts normal_msg = Message::new("normal_task", MessagePriority::Normal)?;
    facts low_msg = Message::new("background_task", MessagePriority::Low)?;
    
    mq.send(high_msg)?;
    mq.send(normal_msg)?;
    mq.send(low_msg)?;
    
    // Receive messages (should come out in priority order)
    while mq.message_count()? > 0 {
        facts received = mq.receive()?;
        println("Received message: {} (priority: {:?})", 
                received.content, received.priority);
    }
    
    // Clean up
    mq.unlink()?;
}

fn demo_semaphores() {
    println("=== Semaphore Demo ===");
    
    // Create a counting semaphore
    facts sem = Semaphore::create("demo_semaphore", 3)?;
    
    // Simulate resource acquisition
    lowkey (sus i = 0; i < 5; i++) {
        stan worker_task(i, "demo_semaphore") {
            println("Worker {} trying to acquire resource...", i);
            
            facts sem = Semaphore::open("demo_semaphore")?;
            sem.acquire()?;
            
            println("Worker {} acquired resource, working...", i);
            std::thread::sleep(Duration::from_millis(500));
            
            sem.release()?;
            println("Worker {} released resource", i);
        }
    }
    
    // Wait for all workers to complete
    std::thread::sleep(Duration::from_secs(3));
    
    // Clean up
    sem.unlink()?;
}

fn demo_unix_domain_sockets() {
    println("=== Unix Domain Sockets Demo ===");
    
    facts socket_path = "/tmp/cursed_demo_socket";
    
    // Create server socket
    facts server_socket = DomainSocket::create(SocketType::Stream)?;
    server_socket.bind(socket_path)?;
    server_socket.listen(5)?;
    
    // Spawn server goroutine
    stan socket_server(socket_path) {
        facts server = DomainSocket::create(SocketType::Stream)?;
        server.bind(socket_path)?;
        server.listen(5)?;
        
        println("Server listening on {}", socket_path);
        
        facts client_socket = server.accept()?;
        facts message = client_socket.read_string()?;
        println("Server received: {}", message);
        
        client_socket.write("Hello from server!")?;
        client_socket.close()?;
        server.close()?;
    }
    
    // Give server time to start
    std::thread::sleep(Duration::from_millis(100));
    
    // Connect as client
    facts client = DomainSocket::create(SocketType::Stream)?;
    client.connect(socket_path)?;
    
    client.write("Hello from client!")?;
    facts response = client.read_string()?;
    println("Client received: {}", response);
    
    client.close()?;
    
    // Clean up
    std::fs::remove_file(socket_path)?;
}

// PHASE 4: Integration with CURSED Features
fn demo_goroutine_process_integration() {
    println("=== Goroutine-Process Integration Demo ===");
    
    facts process_results = Arc::new(Mutex::new(Vec::new()));
    facts results_clone = process_results.clone();
    
    // Spawn multiple goroutines that each manage a process
    lowkey (sus i = 0; i < 3; i++) {
        stan process_worker(i, results_clone.clone()) {
            facts config = ProcessConfig::new("echo")
                .arg(format!("Output from worker {}", i))
                .capture_output();
            
            sus process = spawn_process(config)?;
            facts output = process.wait_for_output()?;
            
            facts message = String::from_utf8(output.stdout)?;
            
            // Store result in shared data structure
            facts mut results = results_clone.lock().unwrap();
            results.push(format!("Worker {}: {}", i, message.trim()));
        }
    }
    
    // Wait for all goroutines to complete
    std::thread::sleep(Duration::from_millis(1000));
    
    // Display results
    facts results = process_results.lock().unwrap();
    lowkey result in &*results {
        println!("{}", result);
    }
}

fn demo_error_handling() {
    println("=== Error Handling Demo ===");
    
    // Demonstrate various error conditions
    
    // Invalid command
    facts invalid_config = ProcessConfig::new("nonexistent_command_xyz");
    match spawn_process(invalid_config) {
        Ok(_) => println("Unexpected success with invalid command"),
        Err(e) => println!("Expected error with invalid command: {}", e),
    }
    
    // Invalid working directory
    facts bad_dir_config = ProcessConfig::new("echo")
        .working_dir("/nonexistent/directory");
    match spawn_process(bad_dir_config) {
        Ok(_) => println("Unexpected success with invalid directory"),
        Err(e) => println!("Expected error with invalid directory: {}", e),
    }
    
    // Timeout handling
    facts timeout_config = ProcessConfig::new("sleep")
        .arg("10")
        .timeout(Duration::from_millis(100));
    match spawn_process(timeout_config) {
        Ok(mut process) => {
            match process.wait_timeout(Duration::from_millis(100)) {
                Ok(Some(_)) => println("Process completed within timeout"),
                Ok(None) => {
                    println!("Process timed out as expected");
                    process.kill()?;
                }
                Err(e) => println!("Timeout error: {}", e),
            }
        }
        Err(e) => println!("Failed to spawn timeout test process: {}", e),
    }
}

fn demo_cross_platform_features() {
    println("=== Cross-Platform Features Demo ===");
    
    facts platform = get_platform_name();
    println!("Running on platform: {}", platform);
    
    // Platform-specific feature detection
    println!("Platform capabilities:");
    
    if supports_feature(PlatformFeature::Signals) {
        println!("  ✓ Signal handling supported");
    }
    
    if supports_feature(PlatformFeature::ProcessGroups) {
        println!("  ✓ Process groups supported");
    }
    
    if supports_feature(PlatformFeature::Cgroups) {
        println!("  ✓ Control groups supported");
    }
    
    if supports_feature(PlatformFeature::WindowsServices) {
        println!("  ✓ Windows services supported");
    }
    
    if supports_feature(PlatformFeature::Namespaces) {
        println!("  ✓ Namespaces supported");
    }
    
    // User information
    if facts user_info = PlatformUtils::get_current_user() {
        println!("Current user: {}", user_info.username);
        
        if user_info.uid.is_some() {
            println!("  UID: {}", user_info.uid.unwrap());
        }
        if user_info.gid.is_some() {
            println!("  GID: {}", user_info.gid.unwrap());
        }
    }
    
    facts is_elevated = PlatformUtils::is_elevated();
    println!("Running with elevated privileges: {}", is_elevated);
}

fn demo_system_monitoring() {
    println("=== System Monitoring Demo ===");
    
    // Get system resource summary
    facts system_summary = get_system_resource_summary()?;
    
    println!("System Resources:");
    lowkey (key, value) in system_summary {
        println!("  {}: {}", key, value);
    }
    
    // Get process list
    facts process_list = get_process_list()?;
    println!("\nTotal processes running: {}", process_list.len());
    
    // Show top 5 processes by PID (just as an example)
    facts mut top_processes = process_list;
    top_processes.sort_by_key(|p| p.pid);
    
    println!("Sample processes:");
    lowkey process in top_processes.iter().take(5) {
        println!("  PID {}: {}", process.pid, process.name);
    }
    
    // Performance monitoring
    facts current_pid = get_current_pid();
    facts metrics = collect_performance_metrics(current_pid)?;
    
    println!("\nCurrent process metrics:");
    println!("  Memory: {} bytes", metrics.memory_bytes);
    println!("  CPU: {:.2}%", metrics.cpu_percent);
    println!("  Threads: {}", metrics.threads);
    
    facts uptime = metrics.uptime;
    println!("  Uptime: {:.2} seconds", uptime.as_secs_f64());
}

// Main demonstration function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 CURSED Process Management and IPC Showcase");
    println!("===============================================");
    
    // Initialize IPC subsystem
    ipc::initialize()?;
    
    // Phase 1: Basic Process Management
    demo_basic_process_management()?;
    demo_environment_and_working_dir()?;
    demo_process_communication()?;
    
    println!();
    
    // Phase 2: Advanced Process Features
    demo_process_monitoring()?;
    demo_concurrent_processes()?;
    
    println!();
    
    // Phase 3: IPC Mechanisms
    demo_shared_memory()?;
    demo_named_pipes()?;
    demo_message_queues()?;
    demo_semaphores()?;
    demo_unix_domain_sockets()?;
    
    println!();
    
    // Phase 4: Integration Features
    demo_goroutine_process_integration()?;
    demo_error_handling()?;
    demo_cross_platform_features()?;
    demo_system_monitoring()?;
    
    println!();
    
    // Display IPC statistics
    facts ipc_stats = ipc::get_ipc_statistics();
    println!("=== IPC System Statistics ===");
    println!("Active shared memory regions: {}", ipc_stats.active_shared_memory_regions);
    println!("Active pipes: {}", ipc_stats.active_pipes);
    println!("Active message queues: {}", ipc_stats.active_message_queues);
    println!("Active semaphores: {}", ipc_stats.active_semaphores);
    println!("Active sockets: {}", ipc_stats.active_sockets);
    println!("Total memory usage: {} bytes", ipc_stats.total_memory_usage);
    
    // Shutdown IPC subsystem
    ipc::shutdown()?;
    
    println!("\n✅ Process and IPC showcase completed successfully!");
    
    Ok(())
}
