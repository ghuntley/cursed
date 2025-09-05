fr fr CURSED Producer-Consumer Pattern Demo
fr fr Real-world IPC pattern using shared memory and semaphores

yeet "stdlib::ipc"
yeet "stdlib::sync"
yeet "stdlib::io" 
yeet "stdlib::process"

fr fr Shared ring buffer structure
squad RingBuffer {
    data: [String; 10],     // Fixed size buffer
    head: usize,            // Producer index
    tail: usize,            // Consumer index
    count: usize,           // Current item count
}

impl RingBuffer {
    slay new() -> Self {
        Self {
            data: Default::default(),
            head: 0,
            tail: 0,
            count: 0,
        }
    }
    
    slay is_full(&self) -> bool {
        self.count == 10
    }
    
    slay is_empty(&self) -> bool {
        self.count == 0
    }
    
    slay produce(&mut self, item: String) -> Result<(), String> {
        bestie (self.is_full()) {
            return Err("Buffer is full - no cap".to_string());
        }
        
        self.data[self.head] = item;
        self.head = (self.head + 1) % 10;
        self.count += 1;
        Ok(())
    }
    
    slay consume(&mut self) -> Result<String, String> {
        bestie (self.is_empty()) {
            return Err("Buffer is empty - that's rough".to_string());
        }
        
        sus item = self.data[self.tail].clone();
        self.tail = (self.tail + 1) % 10;
        self.count -= 1;
        Ok(item)
    }
}

fr fr Producer process that generates work items
slay producer_process(process_id: i32) -> Result<(), IpcError> {
    println("🏭 Producer {} starting up - about to be productive fr", process_id)?;
    
    // Connect to shared memory
    sus config = SharedMemoryConfig::new("ring_buffer_shm", std::mem::size_of::<RingBuffer>())
        .with_permissions(IpcPermissions::read_write());
    
    sus mut shm = SharedMemory::open(config)?;
    shm.map()?;
    
    // Connect to semaphores
    sus empty_slots = Semaphore::open(SemaphoreConfig::new("empty_slots", 10))?;
    sus filled_slots = Semaphore::open(SemaphoreConfig::new("filled_slots", 0))?;
    sus mutex = Semaphore::open(SemaphoreConfig::new("buffer_mutex", 1))?;
    
    // Produce items
    lowkey (sus i = 0; i < 5; i++) {
        sus item = format!("Item {} from Producer {} - this one hits different", i, process_id);
        
        // Wait for empty slot
        empty_slots.acquire()?;
        
        // Lock buffer access
        mutex.acquire()?;
        
        // Read buffer, add item, write back
        sus mut buffer: RingBuffer = shm.read_object(0)?;
        match buffer.produce(item.clone()) {
            Ok(_) => {
                shm.write_object(0, &buffer)?;
                println("📦 Producer {} produced: {}", process_id, item)?;
            }
            Err(e) => {
                println("❌ Producer {} failed: {}", process_id, e)?;
            }
        }
        
        // Release buffer lock
        mutex.release()?;
        
        // Signal filled slot
        filled_slots.release()?;
        
        // Simulate work time
        std::thread::sleep(Duration::from_millis(100));
    }
    
    println("✅ Producer {} finished - that was lowkey fire", process_id)?;
    Ok(())
}

fr fr Consumer process that processes work items
slay consumer_process(process_id: i32) -> Result<(), IpcError> {
    println("🛒 Consumer {} starting up - ready to consume fr", process_id)?;
    
    // Connect to shared memory
    sus config = SharedMemoryConfig::new("ring_buffer_shm", std::mem::size_of::<RingBuffer>())
        .with_permissions(IpcPermissions::read_write());
    
    sus mut shm = SharedMemory::open(config)?;
    shm.map()?;
    
    // Connect to semaphores
    sus empty_slots = Semaphore::open(SemaphoreConfig::new("empty_slots", 10))?;
    sus filled_slots = Semaphore::open(SemaphoreConfig::new("filled_slots", 0))?;
    sus mutex = Semaphore::open(SemaphoreConfig::new("buffer_mutex", 1))?;
    
    // Consume items
    lowkey (sus i = 0; i < 3; i++) {
        // Wait for filled slot
        filled_slots.acquire()?;
        
        // Lock buffer access
        mutex.acquire()?;
        
        // Read buffer, consume item, write back
        sus mut buffer: RingBuffer = shm.read_object(0)?;
        match buffer.consume() {
            Ok(item) => {
                shm.write_object(0, &buffer)?;
                println("📥 Consumer {} consumed: {}", process_id, item)?;
                
                // Simulate processing time
                std::thread::sleep(Duration::from_millis(50));
                println("⚡ Consumer {} processed item - that was bussin", process_id)?;
            }
            Err(e) => {
                println("❌ Consumer {} failed: {}", process_id, e)?;
            }
        }
        
        // Release buffer lock
        mutex.release()?;
        
        // Signal empty slot
        empty_slots.release()?;
    }
    
    println("✅ Consumer {} finished - absolutely devoured those items", process_id)?;
    Ok(())
}

fr fr Coordinator process that sets up shared resources
slay coordinator_process() -> Result<(), IpcError> {
    println("🎯 Coordinator starting - setting up the vibes")?;
    
    // Initialize IPC subsystem
    ipc::initialize()?;
    
    // Create shared memory for ring buffer
    sus config = SharedMemoryConfig::new("ring_buffer_shm", std::mem::size_of::<RingBuffer>())
        .with_permissions(IpcPermissions::read_write())
        .with_remove_on_drop();
    
    sus mut shm = SharedMemory::create(config)?;
    shm.map()?;
    
    // Initialize empty ring buffer
    sus initial_buffer = RingBuffer::new();
    shm.write_object(0, &initial_buffer)?;
    
    // Create semaphores for synchronization
    sus empty_slots = Semaphore::create(SemaphoreConfig::new("empty_slots", 10))?; // 10 empty slots initially
    sus filled_slots = Semaphore::create(SemaphoreConfig::new("filled_slots", 0))?;  // 0 filled slots initially
    sus mutex = Semaphore::create(SemaphoreConfig::new("buffer_mutex", 1))?;         // Binary mutex
    
    println("🚀 Shared resources created - ready for action")?;
    
    // Spawn producer processes
    sus producer_handles = vec![];
    lowkey (sus i = 0; i < 2; i++) {
        sus handle = std::thread::spawn(move || {
            producer_process(i)
        });
        producer_handles.push(handle);
    }
    
    // Spawn consumer processes  
    sus consumer_handles = vec![];
    lowkey (sus i = 0; i < 3; i++) {
        sus handle = std::thread::spawn(move || {
            consumer_process(i)
        });
        consumer_handles.push(handle);
    }
    
    // Wait for all producers to finish
    for handle in producer_handles {
        handle.join().unwrap()?;
    }
    
    println("📊 All producers finished - production line complete")?;
    
    // Wait a bit for consumers to finish processing
    std::thread::sleep(Duration::from_secs(2));
    
    // Signal consumers to finish (in real scenario, might use special termination signal)
    // For demo, we'll just wait for them
    for handle in consumer_handles {
        // In a real implementation, you'd signal consumers to finish gracefully
        handle.join().unwrap()?;
    }
    
    // Get final buffer state
    sus final_buffer: RingBuffer = shm.read_object(0)?;
    println("📈 Final buffer state: {} items remaining", final_buffer.count)?;
    
    // Get IPC statistics
    sus stats = ipc::get_ipc_statistics();
    println("📊 IPC Statistics:")?;
    println("   - Shared memory regions: {}", stats.active_shared_memory_regions)?;
    println("   - Active semaphores: {}", stats.active_semaphores)?;
    println("   - Memory usage: {} bytes", stats.total_memory_usage)?;
    
    // Cleanup resources
    println("🧹 Cleaning up resources...")?;
    empty_slots.remove()?;
    filled_slots.remove()?;
    mutex.remove()?;
    
    // Shutdown IPC subsystem
    ipc::shutdown()?;
    
    println("🎉 Producer-Consumer demo completed - that was absolutely iconic!")?;
    
    Ok(())
}

slay main_character() -> Result<(), IpcError> {
    println("🎊 CURSED Producer-Consumer Pattern Demo")?;
    println("This demonstrates real-world IPC patterns with shared memory and semaphores")?;
    println("=" * 70)?;
    
    coordinator_process()
}
