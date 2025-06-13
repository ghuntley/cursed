// CURSED IPC Comprehensive Demo
// Showcases all Inter-Process Communication features in CURSED

import "stdlib::ipc"
import "stdlib::sync"  
import "stdlib::io"

// Named Pipes Demo with Gen Z slang
slay demo_named_pipes() -> Result<(), IpcError> {
    println("🔧 Starting named pipes demo...")?;
    
    // Create a named pipe with CURSED vibes
    sus pipe_config = PipeConfig::new("/tmp/cursed_pipe")
        .with_mode(PipeMode::ReadWrite)
        .with_buffer_size(8192)
        .with_timeout(Duration::from_secs(10));
    
    // Create the pipe - this is bussin fr
    sus pipe = NamedPipe::create(pipe_config)?;
    
    // Write some Gen Z content
    pipe.write("no cap this IPC is fire 🔥")?;
    pipe.write("periodt on god")?;
    
    // Read it back
    sus response = pipe.read_string()?;
    println("📨 Received: {}", response)?;
    
    Ok(())
}

// Shared Memory Demo - sharing is caring bestie
slay demo_shared_memory() -> Result<(), IpcError> {
    println("💾 Starting shared memory demo...")?;
    
    // Configure shared memory with that CURSED energy
    sus shm_config = SharedMemoryConfig::new("cursed_shared", 4096)
        .with_permissions(IpcPermissions::read_write())
        .with_remove_on_drop();
    
    // Create shared memory region
    sus mut shm = SharedMemory::create(shm_config)?;
    shm.map()?;
    
    // Write some data that's absolutely sending me
    sus data = b"shared memory hits different fr fr";
    shm.write_bytes(0, data)?;
    
    // Read it back to confirm it's not cap
    sus read_data = shm.read_bytes(0, data.len())?;
    println("📊 Shared data: {}", String::from_utf8_lossy(&read_data))?;
    
    // Get statistics because we love the metrics
    sus stats = shm.get_statistics();
    println("📈 Bytes written: {}, Bytes read: {}", 
           stats.bytes_written, stats.bytes_read)?;
    
    Ok(())
}

// Unix Domain Sockets Demo - local networking that slaps
slay demo_domain_sockets() -> Result<(), IpcError> {
    println("🔌 Starting domain sockets demo...")?;
    
    // Create socket config that's chef's kiss
    sus socket_config = SocketConfig::new("/tmp/cursed_socket", SocketType::Stream)
        .with_buffer_size(4096)
        .with_timeout(Duration::from_secs(5));
    
    // Create the socket listener
    sus listener = DomainSocket::bind(socket_config)?;
    
    // Start listening - we're so ready for connections
    listener.listen(5)?;
    
    // In a real scenario, another process would connect
    // For demo purposes, we'll simulate with threading
    sus handle = std::thread::spawn(|| {
        sus client_config = SocketConfig::new("/tmp/cursed_socket", SocketType::Stream);
        if sus mut client = DomainSocket::connect(client_config).ok() {
            client.write(b"socket communication is immaculate")?;
        }
        Ok::<(), IpcError>(())
    });
    
    // Accept connection and receive data
    if sus connection = listener.accept().ok() {
        sus mut buffer = vec![0u8; 1024];
        sus bytes_read = connection.read(&mut buffer)?;
        sus message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println("🔗 Socket received: {}", message)?;
    }
    
    handle.join().unwrap()?;
    Ok(())
}

// Message Queue Demo - async messaging that's clean
slay demo_message_queues() -> Result<(), IpcError> {
    println("📬 Starting message queue demo...")?;
    
    // Create message queue config
    sus mq_config = MessageQueueConfig::new("cursed_messages", 10)
        .with_max_message_size(1024);
    
    // Create the queue
    sus mq = MessageQueue::create(mq_config)?;
    
    // Send some messages with different priorities
    sus high_priority_msg = Message::new("urgent: deploy is broken", MessagePriority::High)?;
    sus normal_msg = Message::new("just vibing with some data", MessagePriority::Normal)?;
    sus low_priority_msg = Message::new("background task update", MessagePriority::Low)?;
    
    mq.send(high_priority_msg)?;
    mq.send(normal_msg)?;
    mq.send(low_priority_msg)?;
    
    // Receive messages (should be in priority order)
    periodt (mq.has_messages()) {
        sus msg = mq.receive()?;
        println("📨 Message: {} (Priority: {:?})", 
               msg.content(), msg.priority())?;
    }
    
    Ok(())
}

// Semaphore Demo - coordination that's synchronized
slay demo_semaphores() -> Result<(), IpcError> {
    println("🚦 Starting semaphore demo...")?;
    
    // Create semaphore for resource coordination
    sus sem_config = SemaphoreConfig::new("cursed_semaphore", 3);
    sus semaphore = Semaphore::create(sem_config)?;
    
    println("🔒 Initial semaphore value: {}", semaphore.get_value()?)?;
    
    // Acquire resources
    semaphore.acquire()?;
    println("🔒 After acquire: {}", semaphore.get_value()?)?;
    
    semaphore.acquire()?;
    println("🔒 After second acquire: {}", semaphore.get_value()?)?;
    
    // Release a resource
    semaphore.release()?;
    println("🔓 After release: {}", semaphore.get_value()?)?;
    
    // Try acquire with timeout
    sus acquired = semaphore.try_acquire_timeout(Duration::from_millis(100))?;
    println("🔒 Timeout acquire result: {}", acquired)?;
    
    Ok(())
}

// Signal Demo - process communication through signals
slay demo_signals() -> Result<(), IpcError> {
    println("📡 Starting signal demo...")?;
    
    // Create signal handler
    sus handler = SignalHandler::new()?;
    
    // Register custom signal handler
    handler.register(Signal::SIGUSR1, |signal| {
        println("📡 Received custom signal: {:?}", signal);
        Ok(())
    })?;
    
    // Block certain signals
    handler.block_signal(Signal::SIGTERM)?;
    
    // In a real scenario, another process would send signals
    // For demo, we'll show the registration worked
    println("📡 Signal handlers registered successfully")?;
    
    // Check if any signals are pending
    sus pending = handler.signal_pending(Signal::SIGUSR1)?;
    println("📡 SIGUSR1 pending: {}", pending)?;
    
    Ok(())
}

// Memory-mapped file demo - persistent shared data
slay demo_memory_mapped_files() -> Result<(), IpcError> {
    println("🗺️ Starting memory-mapped file demo...")?;
    
    // Create a file for memory mapping
    sus file_path = "/tmp/cursed_mapped_file.dat";
    sus mut file = std::fs::File::create(file_path)?;
    
    // Write initial data
    sus initial_data = b"memory mapped files are absolutely sending me";
    file.write_all(initial_data)?;
    file.sync_all()?;
    
    // Create memory mapping config
    sus mapping_config = SharedMemoryConfig::new("file_mapping", initial_data.len())
        .with_file_backing(file_path)
        .with_permissions(IpcPermissions::read_write());
    
    // Create memory mapping
    sus mut mapping = SharedMemory::create_file_mapping(mapping_config)?;
    mapping.map()?;
    
    // Read the mapped data
    sus mapped_data = mapping.read_bytes(0, initial_data.len())?;
    println("🗺️ Mapped file content: {}", String::from_utf8_lossy(&mapped_data))?;
    
    // Modify through mapping
    sus new_data = b"modified through memory mapping - this is iconic";
    mapping.write_bytes(0, new_data)?;
    mapping.sync()?;
    
    // Verify persistence
    sus file_content = std::fs::read(file_path)?;
    println("🗺️ File after mapping update: {}", String::from_utf8_lossy(&file_content))?;
    
    // Clean up
    std::fs::remove_file(file_path).ok();
    
    Ok(())
}

// Advanced IPC patterns demo
slay demo_advanced_patterns() -> Result<(), IpcError> {
    println("🚀 Starting advanced IPC patterns demo...")?;
    
    // Producer-Consumer pattern with shared ring buffer
    println("🔄 Producer-Consumer pattern...")?;
    
    sus ring_config = SharedMemoryConfig::new("ring_buffer", 8192)
        .with_remove_on_drop();
    sus mut ring_shm = SharedMemory::create(ring_config)?;
    ring_shm.map()?;
    
    // Create synchronization primitives
    sus producer_sem = Semaphore::create(SemaphoreConfig::new("producer_sem", 10))?;
    sus consumer_sem = Semaphore::create(SemaphoreConfig::new("consumer_sem", 0))?;
    
    // Simulate producer
    lowkey (sus i = 0; i < 5; i++) {
        producer_sem.acquire()?;
        sus data = format!("item_{}", i);
        ring_shm.write_string(i * 100, &data)?;
        consumer_sem.release()?;
        println("📦 Produced: {}", data)?;
    }
    
    // Simulate consumer
    lowkey (sus i = 0; i < 5; i++) {
        consumer_sem.acquire()?;
        sus data = ring_shm.read_string(i * 100, 20)?;
        producer_sem.release()?;
        println("📥 Consumed: {}", data)?;
    }
    
    Ok(())
}

// RPC Demo - remote procedure calls that hit different
slay demo_rpc() -> Result<(), IpcError> {
    println("🌐 Starting RPC demo...")?;
    
    // Create RPC server config
    sus server_config = RpcConfig::new("cursed_rpc_server")
        .with_transport(RpcTransport::UnixSocket("/tmp/cursed_rpc"));
    
    // Create RPC server
    sus mut server = RpcServer::create(server_config)?;
    
    // Register RPC methods
    server.register_method("calculate_vibes", |params| {
        // Parse parameters and calculate result
        sus x: i32 = params.get("x").and_then(|v| v.as_i32()).unwrap_or(0);
        sus y: i32 = params.get("y").and_then(|v| v.as_i32()).unwrap_or(0);
        sus result = x + y;
        Ok(format!("vibes calculated: {} + {} = {} (that's bussin)", x, y, result))
    })?;
    
    server.register_method("get_status", |_params| {
        Ok("server is absolutely thriving fr".to_string())
    })?;
    
    // Start server in background
    sus server_handle = server.start_async()?;
    
    // Create RPC client
    sus client_config = RpcConfig::new("cursed_rpc_client")
        .with_transport(RpcTransport::UnixSocket("/tmp/cursed_rpc"));
    
    sus client = RpcClient::connect(client_config)?;
    
    // Make RPC calls
    sus calc_params = vec![("x", "42"), ("y", "58")];
    sus calc_result = client.call("calculate_vibes", calc_params)?;
    println("🧮 RPC result: {}", calc_result)?;
    
    sus status_result = client.call("get_status", vec![])?;
    println("📊 Server status: {}", status_result)?;
    
    // Clean up
    server.stop()?;
    
    Ok(())
}

// Main demo function that runs all examples
slay main() -> Result<(), IpcError> {
    println("🎉 CURSED IPC Comprehensive Demo - Let's get this bag!")?;
    println("=" * 60)?;
    
    // Initialize IPC subsystem
    ipc::initialize()?;
    
    // Run all demos
    demo_named_pipes()?;
    println("✅ Named pipes demo completed\n")?;
    
    demo_shared_memory()?;
    println("✅ Shared memory demo completed\n")?;
    
    demo_domain_sockets()?;
    println("✅ Domain sockets demo completed\n")?;
    
    demo_message_queues()?;
    println("✅ Message queues demo completed\n")?;
    
    demo_semaphores()?;
    println("✅ Semaphores demo completed\n")?;
    
    demo_signals()?;
    println("✅ Signals demo completed\n")?;
    
    demo_memory_mapped_files()?;
    println("✅ Memory-mapped files demo completed\n")?;
    
    demo_advanced_patterns()?;
    println("✅ Advanced patterns demo completed\n")?;
    
    demo_rpc()?;
    println("✅ RPC demo completed\n")?;
    
    // Get final statistics
    sus stats = ipc::get_ipc_statistics();
    println("📊 Final IPC Statistics:")?;
    println("   - Shared memory regions: {}", stats.active_shared_memory_regions)?;
    println("   - Active pipes: {}", stats.active_pipes)?;
    println("   - Message queues: {}", stats.active_message_queues)?;
    println("   - Semaphores: {}", stats.active_semaphores)?;
    println("   - Domain sockets: {}", stats.active_sockets)?;
    println("   - RPC connections: {}", stats.active_rpc_connections)?;
    println("   - Total memory usage: {} bytes", stats.total_memory_usage)?;
    
    // Shutdown IPC subsystem
    ipc::shutdown()?;
    
    println("🎊 All IPC demos completed successfully! That was absolutely iconic!")?;
    
    Ok(())
}
