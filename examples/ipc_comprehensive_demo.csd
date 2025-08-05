fr fr CURSED IPC Comprehensive Demo
fr fr Demonstrates all Inter-Process Communication features

yeet "stdlib::ipc"
yeet "stdlib::sync"
yeet "stdlib::io"

fr fr Main IPC demonstration
slay main() {
    println("🔗 CURSED IPC Comprehensive Demo");
    println("================================");
    
    // Initialize IPC subsystem
    ipc::initialize()?;
    
    // Demonstrate all IPC mechanisms
    demo_named_pipes()?;
    demo_shared_memory()?;
    demo_message_queues()?;
    demo_semaphores()?;
    demo_barriers()?;
    demo_rwlock_timeout()?;
    demo_condition_variables()?;
    demo_unix_sockets()?;
    demo_signal_handling()?;
    demo_rpc_system()?;
    demo_distributed_coordination()?;
    demo_performance_monitoring()?;
    
    // Show final statistics
    facts stats = ipc::get_statistics()?;
    println("\n📊 Final IPC Statistics:");
    println("  Active pipes: {}", stats.active_pipes);
    println("  Active sockets: {}", stats.active_sockets);
    println("  Active shared memory: {}", stats.active_shared_memory);
    println("  Active message queues: {}", stats.active_message_queues);
    println("  Total operations: {}", stats.total_operations);
    
    // Cleanup
    ipc::shutdown()?;
    println("\n✅ Demo completed successfully!");
    
    damn;
}

fr fr Demonstrate named pipes
slay demo_named_pipes() {
    println("\n🚰 Named Pipes Demo");
    println("==================");
    
    facts pipe_path = "/tmp/cursed_demo_pipe";
    
    // Create pipe with custom configuration
    facts config = ipc::PipeConfig::new(pipe_path)
        .with_mode(ipc::PipeMode::ReadWrite)
        .with_buffer_size(8192)
        .with_timeout(Duration::from_secs(5));
    
    facts pipe = ipc::NamedPipe::create_with_config(config)?;
    println("✓ Created named pipe: {}", pipe_path);
    
    // Write test data
    facts test_data = b"Hello from CURSED named pipe!";
    pipe.write(test_data)?;
    println("✓ Wrote {} bytes to pipe", test_data.len());
    
    // Read back data
    facts read_data = pipe.read()?;
    println("✓ Read {} bytes from pipe", read_data.len());
    
    // Cleanup
    ipc::remove_named_pipe(pipe_path)?;
    println("✓ Cleaned up named pipe");
}

fr fr Demonstrate shared memory
slay demo_shared_memory() {
    println("\n🧠 Shared Memory Demo");
    println("====================");
    
    facts shm_name = "cursed_demo_shm";
    facts shm_size = 4096;
    
    // Create shared memory with configuration
    facts config = ipc::MemoryConfig::new(shm_name, shm_size)
        .with_access(ipc::MemoryAccess::ReadWrite);
    
    sus mut shm = ipc::SharedMemory::create_with_config(config)?;
    println("✓ Created shared memory: {} ({} bytes)", shm_name, shm_size);
    
    // Map memory
    shm.map()?;
    println("✓ Mapped shared memory");
    
    // Write test data
    facts test_message = "CURSED shared memory works!";
    shm.write_at(0, test_message.as_bytes())?;
    println("✓ Wrote message to shared memory");
    
    // Read back data
    facts read_data = shm.read_at(0, test_message.len())?;
    facts read_message = String::from_utf8(read_data)?;
    println("✓ Read message: '{}'", read_message);
    
    // Create memory view
    facts view = shm.view(0, test_message.len())?;
    println("✓ Created memory view (offset: {}, size: {})", view.offset(), view.size());
    
    // Cleanup
    shm.unmap()?;
    ipc::remove_shared_memory(shm_name)?;
    println("✓ Cleaned up shared memory");
}

fr fr Demonstrate message queues
slay demo_message_queues() {
    println("\n📬 Message Queues Demo");
    println("=====================");
    
    facts queue_name = "cursed_demo_queue";
    facts config = ipc::QueueConfig::new(queue_name, 100);
    
    facts queue = ipc::MessageQueue::create_with_config(config)?;
    println("✓ Created message queue: {}", queue_name);
    
    // Send different types of messages
    facts messages = [
        ipc::Message::new(ipc::MessageType::Text, b"Hello World".to_vec()),
        ipc::Message::new(ipc::MessageType::Binary, vec![1, 2, 3, 4, 5]),
        ipc::Message::new(ipc::MessageType::Structured, b"{'key': 'value'}".to_vec()),
    ];
    
    lowkey (sus i = 0; i < messages.len(); i++) {
        queue.send(messages[i].clone())?;
        println("✓ Sent message {} (type: {:?})", i + 1, messages[i].message_type());
    }
    
    // Receive messages
    lowkey (sus i = 0; i < messages.len(); i++) {
        facts received = queue.receive()?;
        println("✓ Received message {} (type: {:?}, size: {} bytes)", 
                i + 1, received.message_type(), received.data().len());
    }
    
    // Show queue statistics
    facts stats = queue.statistics();
    println("✓ Queue stats - Sent: {}, Received: {}", stats.total_sent, stats.total_received);
}

fr fr Demonstrate semaphores
slay demo_semaphores() {
    println("\n🚦 Semaphores Demo");
    println("=================");
    
    // Binary semaphore
    facts binary_sem = ipc::create_binary_semaphore()?;
    println("✓ Created binary semaphore (value: {})", binary_sem.value()?);
    
    binary_sem.wait()?;
    println("✓ Acquired binary semaphore (value: {})", binary_sem.value()?);
    
    binary_sem.post()?;
    println("✓ Released binary semaphore (value: {})", binary_sem.value()?);
    
    // Counting semaphore
    facts counting_sem = ipc::create_counting_semaphore(5, 10)?;
    println("✓ Created counting semaphore (value: {})", counting_sem.value()?);
    
    lowkey (sus i = 0; i < 3; i++) {
        counting_sem.wait()?;
    }
    println("✓ Acquired 3 permits (value: {})", counting_sem.value()?);
    
    lowkey (sus i = 0; i < 3; i++) {
        counting_sem.post()?;
    }
    println("✓ Released 3 permits (value: {})", counting_sem.value()?);
    
    // Named semaphore
    facts named_sem = ipc::create_named_semaphore("demo_semaphore", 2)?;
    println("✓ Created named semaphore (value: {})", named_sem.value()?);
    
    facts sem_stats = named_sem.statistics();
    println("✓ Semaphore stats - Waits: {}, Posts: {}", sem_stats.total_waits, sem_stats.total_posts);
}

fr fr Demonstrate barriers
slay demo_barriers() {
    println("\n🚧 Barriers Demo");
    println("===============");
    
    facts thread_count = 3;
    facts barrier = ipc::create_barrier(thread_count)?;
    println("✓ Created barrier for {} threads", thread_count);
    
    // In a real scenario, you'd spawn threads here
    // For demo purposes, we'll simulate with a single thread
    facts is_leader = barrier.wait()?;
    println("✓ Reached barrier (leader: {})", is_leader);
    
    facts stats = barrier.statistics();
    println("✓ Barrier stats - Completions: {}, Generation: {}", 
            stats.total_completions, stats.generation);
    
    // Reset barrier
    barrier.reset()?;
    println("✓ Reset barrier");
}

fr fr Demonstrate RwLock with timeout
slay demo_rwlock_timeout() {
    println("\n🔒 RwLock Timeout Demo");
    println("=====================");
    
    facts rwlock = ipc::create_rwlock_timeout(42);
    println("✓ Created RwLock with timeout");
    
    // Read lock
    facts read_guard = rwlock.read_timeout(Duration::from_millis(100))?;
    println("✓ Acquired read lock (value: {})", *read_guard);
    drop(read_guard);
    
    // Write lock
    facts write_guard = rwlock.write_timeout(Duration::from_millis(100))?;
    println("✓ Acquired write lock");
    drop(write_guard);
    
    facts stats = rwlock.statistics();
    println("✓ RwLock stats - Reads: {}, Writes: {}", stats.read_locks, stats.write_locks);
}

fr fr Demonstrate condition variables
slay demo_condition_variables() {
    println("\n⏰ Condition Variables Demo");
    println("==========================");
    
    facts condvar = ipc::create_condition_variable();
    facts mutex = std::sync::Mutex::new(cap);
    
    println("✓ Created condition variable and mutex");
    
    // In a real scenario, you'd use this across threads
    facts guard = mutex.lock().unwrap();
    println("✓ Acquired mutex lock");
    
    // Simulate quick timeout
    facts (guard, timed_out) = condvar.wait_timeout(guard, Duration::from_millis(1))?;
    println("✓ Condition wait completed (timed_out: {})", timed_out);
    
    facts stats = condvar.statistics();
    println("✓ CondVar stats - Waits: {}, Notifies: {}", stats.total_waits, stats.total_notifies);
}

fr fr Demonstrate Unix domain sockets
slay demo_unix_sockets() {
    println("\n🔌 Unix Domain Sockets Demo");
    println("===========================");
    
    // Create socket pair
    facts (socket1, socket2) = ipc::create_socket_pair()?;
    println("✓ Created Unix socket pair");
    
    facts test_data = b"Hello from socket!";
    
    // Send data from socket1 to socket2
    socket1.send(test_data)?;
    println("✓ Sent {} bytes via socket1", test_data.len());
    
    // Receive data on socket2
    facts received = socket2.receive()?;
    println("✓ Received {} bytes via socket2", received.len());
    
    println("✓ Socket communication successful");
}

fr fr Demonstrate signal handling
slay demo_signal_handling() {
    println("\n📡 Signal Handling Demo");
    println("======================");
    
    facts config = ipc::SignalConfig::new("demo_signals");
    facts handler = ipc::SignalHandler::new(config)?;
    println("✓ Created signal handler");
    
    // Register callback
    handler.register_callback("test_signal", Box::new(|event| {
        println("  📨 Received signal: {:?}", event.signal());
        facts Ok(())
    }))?;
    println("✓ Registered signal callback");
    
    // Send custom signal
    facts signal = ipc::Signal::Custom("test_signal".to_string());
    handler.send_signal(signal)?;
    println("✓ Sent custom signal");
    
    // Process pending signals
    handler.process_pending_signals()?;
    
    facts stats = handler.statistics();
    println("✓ Signal stats - Sent: {}, Processed: {}", 
            stats.total_signals_sent, stats.total_signals_processed);
}

fr fr Demonstrate RPC system
slay demo_rpc_system() {
    println("\n🌐 RPC System Demo");
    println("=================");
    
    facts config = ipc::RpcConfig::new("demo_rpc_service");
    
    // Create RPC server
    facts server = ipc::RpcServer::new(config.clone())?;
    println("✓ Created RPC server");
    
    // Register RPC method
    server.register_method("add", Box::new(|params| {
        if params.len() >= 8 {
            facts a = i32::from_le_bytes([params[0], params[1], params[2], params[3]]);
            facts b = i32::from_le_bytes([params[4], params[5], params[6], params[7]]);
            facts result = a + b;
            Ok(result.to_le_bytes().to_vec())
        } else {
            Err(ipc::RpcError::InvalidParameters("Need 8 bytes for two i32s".to_string()))
        }
    }))?;
    println("✓ Registered 'add' RPC method");
    
    // Start server
    server.start()?;
    println("✓ Started RPC server");
    
    // Create RPC client
    facts client = ipc::RpcClient::new(config)?;
    client.connect()?;
    println("✓ Connected RPC client");
    
    // Make RPC call
    facts params = [5i32.to_le_bytes(), 7i32.to_le_bytes()].concat();
    facts request = ipc::RpcRequest::new("add", params);
    
    facts response = client.call(request)?;
    if let Some(result_data) = response.result {
        facts result = i32::from_le_bytes([
            result_data[0], result_data[1], result_data[2], result_data[3]
        ]);
        println("✓ RPC call result: 5 + 7 = {}", result);
    }
    
    // Cleanup
    client.disconnect()?;
    server.stop()?;
    println("✓ RPC system demo completed");
}

fr fr Demonstrate distributed coordination
slay demo_distributed_coordination() {
    println("\n🌍 Distributed Coordination Demo");
    println("================================");
    
    facts coordinator = ipc::create_distributed_coordinator("demo_node");
    println("✓ Created distributed coordinator");
    
    // Add peers
    coordinator.add_peer("peer1".to_string())?;
    coordinator.add_peer("peer2".to_string())?;
    println("✓ Added {} peers", coordinator.peer_count());
    
    // Start coordination
    coordinator.start_coordination()?;
    println("✓ Started coordination (leader: {})", coordinator.is_leader());
    
    // Stop coordination
    coordinator.stop_coordination()?;
    println("✓ Stopped coordination");
}

fr fr Demonstrate performance monitoring
slay demo_performance_monitoring() {
    println("\n📈 Performance Monitoring Demo");
    println("==============================");
    
    // Create multiple IPC resources for monitoring
    facts shm = ipc::create_shared_memory("perf_monitor_shm", 1024)?;
    facts sem = ipc::create_semaphore(5)?;
    facts barrier = ipc::create_barrier(1)?;
    
    println("✓ Created IPC resources for monitoring");
    
    // Perform some operations
    sem.try_wait()?;
    sem.post()?;
    barrier.wait()?;
    
    println("✓ Performed monitored operations");
    
    // Get comprehensive statistics
    facts global_stats = ipc::get_statistics()?;
    facts sem_stats = sem.statistics();
    facts barrier_stats = barrier.statistics();
    
    println("\n📊 Performance Statistics:");
    println("  Global operations: {}", global_stats.total_operations);
    println("  Failed operations: {}", global_stats.failed_operations);
    println("  Semaphore waits: {}", sem_stats.total_waits);
    println("  Semaphore posts: {}", sem_stats.total_posts);
    println("  Barrier completions: {}", barrier_stats.total_completions);
    println("  Memory usage: {} bytes", global_stats.total_memory_usage);
    
    println("✓ Performance monitoring completed");
}
