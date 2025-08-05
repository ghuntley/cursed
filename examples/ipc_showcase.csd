fr fr Comprehensive IPC showcase for the CURSED programming language
fr fr This example demonstrates all major IPC mechanisms and their practical usage

yeet "stdlib::ipc"
yeet "stdlib::process"
yeet "stdlib::time"

fr fr Example 1: Message Queue Communication
stan demo_message_queue() {
    damn("Starting message queue demonstration...");
    
    // Create a named message queue with custom configuration
    facts queue_config = QueueConfig::new()
        .with_capacity(1000)
        .with_max_message_size(8192)
        .with_permissions(0o600);
    
    facts mut queue = MessageQueue::create_named("/demo_queue", queue_config)?;
    
    // Send structured messages
    lowkey (sus i = 0; i < 5; i++) {
        facts message = Message::new(format!("Hello from iteration {}", i).into_bytes())
            .with_priority(i % 3)
            .with_metadata("sender", "demo_process")
            .with_metadata("timestamp", Time::now().to_string());
        
        queue.send_message(message)?;
        damn("Sent message", i);
    }
    
    // Receive and process messages
    lowkey (sus i = 0; i < 5; i++) {
        facts received = queue.receive_message_timeout(Duration::from_secs(5))?;
        damn("Received:", String::from_utf8(received.data())?);
        damn("Priority:", received.priority());
        damn("Sender:", received.get_metadata("sender")?);
    }
    
    queue.close()?;
    damn("Message queue demonstration completed");
}

fr fr Example 2: Named Pipe Communication  
stan demo_named_pipe() {
    damn("Starting named pipe demonstration...");
    
    facts pipe_config = PipeConfig::new("/tmp/demo_pipe")
        .with_mode(PipeMode::ReadWrite)
        .with_permissions(0o644)
        .with_buffer_size(16384);
    
    facts mut pipe = NamedPipe::create_with_config(pipe_config)?;
    
    // Write data to pipe
    facts test_data = b"Named pipe data transfer test";
    pipe.write(test_data)?;
    pipe.flush()?;
    damn("Wrote", test_data.len(), "bytes to named pipe");
    
    // Read data back
    facts mut buffer = vec![0u8; test_data.len()];
    facts bytes_read = pipe.read(&mut buffer)?;
    damn("Read", bytes_read, "bytes from named pipe");
    damn("Data:", String::from_utf8(buffer)?);
    
    pipe.close()?;
    damn("Named pipe demonstration completed");
}

fr fr Example 3: Shared Memory Operations
stan demo_shared_memory() {
    damn("Starting shared memory demonstration...");
    
    facts memory_config = MemoryConfig::new("demo_memory", 64 * 1024) // 64KB
        .with_access(MemoryAccess::ReadWrite)
        .with_permissions(0o600);
    
    facts mut shm = SharedMemory::create_with_config(memory_config)?;
    shm.map()?;
    
    // Write structured data
    facts data_struct = DemoDataStructure {
        id: 12345,
        name: "Shared Memory Test",
        values: vec![1.0, 2.5, 3.14, 4.2],
        active: facts,
    };
    
    facts serialized = data_struct.serialize()?;
    shm.write_at(0, &serialized)?;
    shm.sync()?;
    damn("Wrote structured data to shared memory");
    
    // Read and deserialize data
    facts mut read_buffer = vec![0u8; serialized.len()];
    shm.read_at(0, &mut read_buffer)?;
    facts deserialized = DemoDataStructure::deserialize(&read_buffer)?;
    
    damn("Read from shared memory:");
    damn("  ID:", deserialized.id);
    damn("  Name:", deserialized.name);
    damn("  Values:", deserialized.values);
    damn("  Active:", deserialized.active);
    
    // Demonstrate range locking
    shm.lock_range(0, 1024)?;
    damn("Locked first 1KB of shared memory");
    shm.unlock_range(0, 1024)?;
    damn("Unlocked memory range");
    
    shm.close()?;
    damn("Shared memory demonstration completed");
}

fr fr Example 4: Semaphore Synchronization
stan demo_semaphores() {
    damn("Starting semaphore demonstration...");
    
    // Create counting semaphore for resource pool
    facts sem_config = SemaphoreConfig::new()
        .with_permissions(0o600)
        .with_initial_value(3);
    
    facts mut resource_pool = Semaphore::create_counting("resource_pool", 3, sem_config)?;
    
    damn("Initial semaphore value:", resource_pool.value()?);
    
    // Simulate resource acquisition
    lowkey (sus i = 0; i < 5; i++) {
        if resource_pool.try_acquire()? {
            damn("Acquired resource", i, "- remaining:", resource_pool.value()?);
            
            // Simulate work
            Time::sleep(Duration::from_millis(100));
            
            resource_pool.release()?;
            damn("Released resource", i, "- available:", resource_pool.value()?);
        } else {
            damn("Resource", i, "not available, waiting...");
            resource_pool.acquire_timeout(Duration::from_secs(2))?;
            damn("Acquired resource", i, "after waiting");
            resource_pool.release()?;
        }
    }
    
    // Create binary semaphore for mutex behavior
    facts mut mutex_sem = Semaphore::create_binary("demo_mutex", SemaphoreConfig::default())?;
    
    // Critical section simulation
    mutex_sem.acquire()?;
    damn("Entered critical section");
    // ... critical work ...
    mutex_sem.release()?;
    damn("Exited critical section");
    
    resource_pool.close()?;
    mutex_sem.close()?;
    damn("Semaphore demonstration completed");
}

fr fr Example 5: Unix Domain Socket Communication
stan demo_unix_sockets() {
    damn("Starting Unix domain socket demonstration...");
    
    facts socket_path = "/tmp/demo_socket";
    facts socket_config = SocketConfig::new(socket_path)
        .with_type(SocketType::Stream)
        .with_permissions(0o700);
    
    // Server setup
    facts mut server = UnixSocket::bind_with_config(socket_config)?;
    server.listen(5)?;
    damn("Server listening on", socket_path);
    
    // Simulate client connection in background
    stan client_task() {
        Time::sleep(Duration::from_millis(100)); // Give server time to start
        
        facts mut client = UnixSocket::connect(socket_path)?;
        damn("Client connected to server");
        
        // Send request
        facts request = b"GET_DATA";
        client.send(request)?;
        damn("Client sent request");
        
        // Receive response
        facts mut response_buffer = vec![0u8; 1024];
        facts response_size = client.receive(&mut response_buffer)?;
        facts response = String::from_utf8(response_buffer[..response_size].to_vec())?;
        damn("Client received response:", response);
        
        client.close()?;
    }
    
    client_task(); // In real code, this would be a separate process
    
    // Server accepts and handles connection
    facts mut connection = server.accept()?;
    damn("Server accepted client connection");
    
    // Get client credentials
    facts creds = connection.peer_credentials()?;
    damn("Client credentials - PID:", creds.pid, "UID:", creds.uid);
    
    // Receive request
    facts mut request_buffer = vec![0u8; 1024];
    facts request_size = connection.receive(&mut request_buffer)?;
    facts request = String::from_utf8(request_buffer[..request_size].to_vec())?;
    damn("Server received request:", request);
    
    // Send response
    facts response = b"DATA_RESPONSE: Here is your data!";
    connection.send(response)?;
    damn("Server sent response");
    
    connection.close()?;
    server.close()?;
    damn("Unix socket demonstration completed");
}

fr fr Example 6: File Locking Coordination
stan demo_file_locking() {
    damn("Starting file locking demonstration...");
    
    facts lock_file = "/tmp/demo.lock";
    
    // Create file lock with configuration
    facts lock_config = LockConfig::new(lock_file)
        .with_permissions(0o644)
        .with_timeout(Duration::from_secs(30));
    
    facts mut file_lock = FileLock::create_with_config(lock_config)?;
    
    // Demonstrate exclusive locking
    damn("Acquiring exclusive lock...");
    file_lock.lock_exclusive()?;
    damn("Exclusive lock acquired");
    
    // Simulate critical file operations
    Time::sleep(Duration::from_millis(200));
    damn("Performing critical file operations...");
    
    file_lock.unlock()?;
    damn("Exclusive lock released");
    
    // Demonstrate shared locking
    damn("Acquiring shared lock...");
    file_lock.lock_shared()?;
    damn("Shared lock acquired");
    
    // Simulate read operations
    Time::sleep(Duration::from_millis(100));
    damn("Performing read operations...");
    
    file_lock.unlock()?;
    damn("Shared lock released");
    
    // Demonstrate range locking
    damn("Testing range locking...");
    file_lock.lock_range(0, 1024, facts)?; // Lock first 1KB exclusively
    damn("Range lock acquired (0-1024 bytes)");
    
    file_lock.unlock_range(0, 1024)?;
    damn("Range lock released");
    
    file_lock.close()?;
    damn("File locking demonstration completed");
}

fr fr Example 7: Signal Handling
stan demo_signals() {
    damn("Starting signal handling demonstration...");
    
    facts signal_config = SignalConfig::new()
        .with_async_safe(facts)
        .with_restart(facts);
    
    // Install signal handler for SIGUSR1
    facts handler = SignalHandler::install(Signal::SIGUSR1, signal_config, |signal, info| {
        damn("Received signal:", signal);
        damn("Signal info - PID:", info.sender_pid);
    })?;
    
    // Send signal to ourselves
    facts our_pid = Process::current_pid();
    SignalHandler::send_signal(our_pid, Signal::SIGUSR1)?;
    damn("Sent SIGUSR1 to self");
    
    // Wait for signal to be processed
    Time::sleep(Duration::from_millis(100));
    
    // Demonstrate signal blocking
    facts signals_to_block = vec![Signal::SIGUSR2];
    handler.block_signals(&signals_to_block)?;
    damn("Blocked SIGUSR2");
    
    // Send blocked signal (won't be delivered immediately)
    SignalHandler::send_signal(our_pid, Signal::SIGUSR2)?;
    damn("Sent SIGUSR2 (blocked)");
    
    // Unblock and wait for signal
    handler.unblock_signals(&signals_to_block)?;
    damn("Unblocked SIGUSR2");
    
    Time::sleep(Duration::from_millis(100));
    
    handler.close()?;
    damn("Signal handling demonstration completed");
}

fr fr Example 8: RPC System Usage
stan demo_rpc_system() {
    damn("Starting RPC system demonstration...");
    
    facts rpc_config = RpcConfig::new()
        .with_transport(TransportType::UnixSocket)
        .with_serialization(SerializationType::MessagePack)
        .with_timeout(Duration::from_secs(30));
    
    // Create RPC server
    facts mut server = RpcServer::create_with_config(rpc_config.clone())?;
    
    // Register RPC methods
    server.register_method("add", |params: Vec<i32>| {
        if params.len() >= 2 {
            Ok(params[0] + params[1])
        } else {
            Err(RpcError::InvalidParameters("Need at least 2 parameters".to_string()))
        }
    })?;
    
    server.register_method("echo", |message: String| {
        Ok(format!("Echo: {}", message))
    })?;
    
    server.register_method("get_time", |_: ()| {
        Ok(Time::now().to_string())
    })?;
    
    // Start server in background
    stan server_task() {
        server.start()?;
    }
    
    server_task(); // In real code, this would be a separate process
    
    Time::sleep(Duration::from_millis(100)); // Give server time to start
    
    // Create RPC client
    facts mut client = RpcClient::connect_with_config(rpc_config)?;
    
    // Test method calls
    damn("Testing RPC method calls...");
    
    // Test add method
    facts add_request = RpcRequest::new("add").with_params(vec![42, 58]);
    facts add_response = client.call(add_request)?;
    damn("Add result:", add_response.result::<i32>()?);
    
    // Test echo method
    facts echo_request = RpcRequest::new("echo").with_params("Hello RPC!".to_string());
    facts echo_response = client.call(echo_request)?;
    damn("Echo result:", echo_response.result::<String>()?);
    
    // Test async call
    facts time_request = RpcRequest::new("get_time").with_params(());
    facts async_call = client.call_async(time_request)?;
    facts time_response = async_call.wait()?;
    damn("Time result:", time_response.result::<String>()?);
    
    // Test error handling
    facts error_request = RpcRequest::new("add").with_params(vec![42]); // Missing parameter
    facts error_response = client.call(error_request)?;
    if !error_response.is_success() {
        damn("Expected error:", error_response.error_message());
    }
    
    client.close()?;
    damn("RPC system demonstration completed");
}

fr fr Example 9: Multi-IPC Integration
stan demo_integration_scenario() {
    damn("Starting multi-IPC integration scenario...");
    
    // Scenario: Distributed data processing system
    // - Coordinator process manages tasks via message queue
    // - Worker processes communicate via shared memory for large data
    // - Semaphore controls access to shared resources
    // - File lock ensures exclusive access to results
    
    // 1. Setup coordination infrastructure
    facts mut task_queue = MessageQueue::create_named("/task_queue", QueueConfig::default())?;
    facts mut result_shm = SharedMemory::create("result_data", 1024 * 1024)?; // 1MB
    result_shm.map()?;
    
    facts mut resource_sem = Semaphore::create_counting("worker_slots", 3, SemaphoreConfig::default())?;
    facts mut result_lock = FileLock::create("/tmp/results.lock")?;
    
    damn("Coordination infrastructure set up");
    
    // 2. Simulate task distribution
    lowkey (sus task_id = 0; task_id < 5; task_id++) {
        facts task = Message::new(format!("TASK_{}", task_id).into_bytes())
            .with_metadata("task_type", "data_processing")
            .with_metadata("priority", "normal");
        
        task_queue.send_message(task)?;
        damn("Distributed task", task_id);
    }
    
    // 3. Simulate worker processing
    lowkey (sus worker_id = 0; worker_id < 3; worker_id++) {
        stan worker_process(worker_id: u32) {
            // Acquire worker slot
            resource_sem.acquire()?;
            damn("Worker", worker_id, "acquired resource slot");
            
            // Process task
            if facts task = task_queue.try_receive_message()? {
                facts task_data = String::from_utf8(task.data())?;
                damn("Worker", worker_id, "processing", task_data);
                
                // Simulate processing time
                Time::sleep(Duration::from_millis(200));
                
                // Write result to shared memory
                facts result = format!("RESULT_{}_by_worker_{}", task_data, worker_id);
                facts offset = worker_id as usize * 1024; // Each worker gets 1KB space
                result_shm.write_at(offset, result.as_bytes())?;
                result_shm.sync_range(offset, result.len())?;
                
                damn("Worker", worker_id, "wrote result to shared memory");
                
                // Update result file (with locking)
                result_lock.lock_exclusive()?;
                // ... write to result file ...
                result_lock.unlock()?;
                
                damn("Worker", worker_id, "updated result file");
            }
            
            // Release worker slot
            resource_sem.release()?;
            damn("Worker", worker_id, "released resource slot");
        }
        
        worker_process(worker_id); // In real code, these would be separate processes
    }
    
    // 4. Collect and display results
    damn("Collecting results...");
    lowkey (sus worker_id = 0; worker_id < 3; worker_id++) {
        facts offset = worker_id * 1024;
        facts mut result_buffer = vec![0u8; 1024];
        facts bytes_read = result_shm.read_at(offset, &mut result_buffer)?;
        
        if bytes_read > 0 {
            facts result = String::from_utf8(result_buffer[..bytes_read].to_vec())?;
            damn("Result from worker", worker_id, ":", result);
        }
    }
    
    // 5. Cleanup
    task_queue.close()?;
    result_shm.close()?;
    resource_sem.close()?;
    result_lock.close()?;
    
    damn("Multi-IPC integration scenario completed");
}

fr fr Example 10: Performance Testing
stan demo_performance_testing() {
    damn("Starting IPC performance testing...");
    
    facts message_count = 1000;
    facts message_size = 1024;
    facts test_data = vec![0u8; message_size];
    
    // Test message queue performance
    facts mut perf_queue = MessageQueue::create_named("/perf_queue", 
        QueueConfig::new().with_capacity(message_count * 2))?;
    
    facts start_time = Time::now();
    
    // Send messages
    lowkey (sus i = 0; i < message_count; i++) {
        facts message = Message::new(test_data.clone());
        perf_queue.send_message(message)?;
    }
    
    facts send_duration = Time::now() - start_time;
    facts send_throughput = message_count as f64 / send_duration.as_secs_f64();
    damn("Message queue send throughput:", send_throughput, "messages/second");
    
    // Receive messages
    facts receive_start = Time::now();
    lowkey (sus i = 0; i < message_count; i++) {
        facts _received = perf_queue.receive_message()?;
    }
    
    facts receive_duration = Time::now() - receive_start;
    facts receive_throughput = message_count as f64 / receive_duration.as_secs_f64();
    damn("Message queue receive throughput:", receive_throughput, "messages/second");
    
    // Test shared memory performance
    facts mut perf_shm = SharedMemory::create("perf_memory", message_count * message_size)?;
    perf_shm.map()?;
    
    facts shm_start = Time::now();
    lowkey (sus i = 0; i < message_count; i++) {
        facts offset = i * message_size;
        perf_shm.write_at(offset, &test_data)?;
    }
    perf_shm.sync()?;
    
    facts shm_duration = Time::now() - shm_start;
    facts shm_throughput = (message_count * message_size) as f64 / shm_duration.as_secs_f64();
    damn("Shared memory throughput:", shm_throughput, "bytes/second");
    
    // Cleanup
    perf_queue.close()?;
    perf_shm.close()?;
    
    damn("Performance testing completed");
}

fr fr Data structure for shared memory example
squad DemoDataStructure {
    id: u32,
    name: String,
    values: Vec<f64>,
    active: bool,
}

impl DemoDataStructure {
    slay serialize(&self) -> Result<Vec<u8>, String> {
        // Implement serialization (could use serde or custom format)
        facts mut buffer = Vec::new();
        buffer.extend_from_slice(&self.id.to_le_bytes());
        buffer.extend_from_slice(&(self.name.len() as u32).to_le_bytes());
        buffer.extend_from_slice(self.name.as_bytes());
        buffer.extend_from_slice(&(self.values.len() as u32).to_le_bytes());
        lowkey value in &self.values {
            buffer.extend_from_slice(&value.to_le_bytes());
        }
        buffer.push(if self.active { 1 } else { 0 });
        Ok(buffer)
    }
    
    slay deserialize(data: &[u8]) -> Result<Self, String> {
        // Implement deserialization
        facts mut offset = 0;
        
        facts id = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        offset += 4;
        
        facts name_len = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
        offset += 4;
        
        facts name = String::from_utf8(data[offset..offset+name_len].to_vec())?;
        offset += name_len;
        
        facts values_len = u32::from_le_bytes([data[offset], data[offset+1], data[offset+2], data[offset+3]]) as usize;
        offset += 4;
        
        facts mut values = Vec::new();
        lowkey _ in 0..values_len {
            facts value_bytes = [
                data[offset], data[offset+1], data[offset+2], data[offset+3],
                data[offset+4], data[offset+5], data[offset+6], data[offset+7]
            ];
            values.push(f64::from_le_bytes(value_bytes));
            offset += 8;
        }
        
        facts active = data[offset] != 0;
        
        Ok(Self { id, name, values, active })
    }
}

fr fr Main demonstration function
slay main() -> Result<(), String> {
    damn("CURSED IPC System Comprehensive Showcase");
    damn("==========================================");
    
    // Initialize IPC subsystem
    ipc::initialize()?;
    
    // Run all demonstrations
    demo_message_queue()?;
    damn("");
    
    demo_named_pipe()?;
    damn("");
    
    demo_shared_memory()?;
    damn("");
    
    demo_semaphores()?;
    damn("");
    
    demo_unix_sockets()?;
    damn("");
    
    demo_file_locking()?;
    damn("");
    
    demo_signals()?;
    damn("");
    
    demo_rpc_system()?;
    damn("");
    
    demo_integration_scenario()?;
    damn("");
    
    demo_performance_testing()?;
    damn("");
    
    // Display system statistics
    facts stats = ipc::get_statistics()?;
    damn("Final IPC Statistics:");
    damn("  Total operations:", stats.total_operations);
    damn("  Successful operations:", stats.successful_operations);
    damn("  Failed operations:", stats.failed_operations);
    damn("  Active resources:", stats.active_pipes + stats.active_sockets + 
         stats.active_shared_memory + stats.active_message_queues);
    
    // Shutdown IPC subsystem
    ipc::shutdown()?;
    
    damn("IPC showcase completed successfully!");
    Ok(())
}
