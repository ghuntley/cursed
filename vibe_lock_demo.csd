// Demo of the vibe_lock synchronization primitives

// Simple demonstration of synchronization concepts
slay demo_mutex() normie {
    vibez.spill("=== MUTEX DEMO ===")
    
    // Simulate mutex creation and operations
    sus shared_resource normie = 0
    vibez.spill("Shared resource initial value: 0")
    
    // Simulate critical section
    vibez.spill("Acquiring mutex...")
    sus mutex_acquired lit = based
    
    vibe mutex_acquired {
        vibez.spill("Mutex acquired! Entering critical section...")
        shared_resource = shared_resource + 1
        vibez.spill("Updated shared resource to: 1")
        
        vibez.spill("Exiting critical section...")
        sus mutex_released lit = based
        vibe mutex_released {
            vibez.spill("Mutex released!")
        }
    }
    
    vibez.spill("Mutex demo completed!")
    damn 0
}

slay demo_rwlock() normie {
    vibez.spill("=== READ-WRITE LOCK DEMO ===")
    
    // Simulate read-write lock
    sus config_value normie = 42
    vibez.spill("Configuration value: 42")
    
    // Simulate multiple readers
    vibez.spill("Reader 1 acquiring read lock...")
    sus read_lock1 lit = based
    vibe read_lock1 {
        vibez.spill("Reader 1: Configuration value is 42")
        vibez.spill("Reader 1 releasing read lock...")
    }
    
    vibez.spill("Reader 2 acquiring read lock...")
    sus read_lock2 lit = based
    vibe read_lock2 {
        vibez.spill("Reader 2: Configuration value is 42")
        vibez.spill("Reader 2 releasing read lock...")
    }
    
    // Simulate writer
    vibez.spill("Writer acquiring write lock...")
    sus write_lock lit = based
    vibe write_lock {
        vibez.spill("Writer: Updating configuration...")
        config_value = 100
        vibez.spill("Writer: Configuration updated to 100")
        vibez.spill("Writer releasing write lock...")
    }
    
    vibez.spill("Read-write lock demo completed!")
    damn 0
}

slay demo_semaphore() normie {
    vibez.spill("=== SEMAPHORE DEMO ===")
    
    // Simulate semaphore with 3 permits
    sus available_permits normie = 3
    vibez.spill("Semaphore created with 3 permits")
    
    // Simulate acquiring permits
    vibez.spill("Worker 1 acquiring permit...")
    sus permit1 lit = based
    vibe permit1 {
        available_permits = available_permits - 1
        vibez.spill("Worker 1: Permit acquired! Available permits: 2")
        vibez.spill("Worker 1: Using resource...")
        vibez.spill("Worker 1: Releasing permit...")
        available_permits = available_permits + 1
    }
    
    vibez.spill("Worker 2 acquiring permit...")
    sus permit2 lit = based
    vibe permit2 {
        available_permits = available_permits - 1
        vibez.spill("Worker 2: Permit acquired! Available permits: 2")
        vibez.spill("Worker 2: Using resource...")
        vibez.spill("Worker 2: Releasing permit...")
        available_permits = available_permits + 1
    }
    
    vibez.spill("Semaphore demo completed!")
    damn 0
}

slay demo_once() normie {
    vibez.spill("=== ONCE INITIALIZATION DEMO ===")
    
    // Simulate once initialization
    sus initialized lit = cap
    vibez.spill("System not initialized")
    
    // First call
    vibez.spill("First call to initialize...")
    vibe initialized == cap {
        vibez.spill("Performing expensive initialization...")
        initialized = based
        vibez.spill("Initialization completed!")
    }
    
    // Second call
    vibez.spill("Second call to initialize...")
    vibe initialized == based {
        vibez.spill("Already initialized, skipping...")
    }
    
    // Third call
    vibez.spill("Third call to initialize...")
    vibe initialized == based {
        vibez.spill("Already initialized, skipping...")
    }
    
    vibez.spill("Once initialization demo completed!")
    damn 0
}

slay demo_atomic() normie {
    vibez.spill("=== ATOMIC OPERATIONS DEMO ===")
    
    // Simulate atomic counter
    sus atomic_counter normie = 0
    vibez.spill("Atomic counter initialized to: 0")
    
    // Simulate atomic increment
    vibez.spill("Thread 1: Incrementing counter...")
    sus old_value1 normie = atomic_counter
    atomic_counter = atomic_counter + 1
    vibez.spill("Thread 1: Counter incremented from 0 to 1")
    
    vibez.spill("Thread 2: Incrementing counter...")
    sus old_value2 normie = atomic_counter
    atomic_counter = atomic_counter + 1
    vibez.spill("Thread 2: Counter incremented from 1 to 2")
    
    vibez.spill("Thread 3: Incrementing counter...")
    sus old_value3 normie = atomic_counter
    atomic_counter = atomic_counter + 1
    vibez.spill("Thread 3: Counter incremented from 2 to 3")
    
    vibez.spill("Final counter value: 3")
    vibez.spill("Atomic operations demo completed!")
    damn 0
}

slay main() normie {
    vibez.spill("🔒 VIBE_LOCK SYNCHRONIZATION PRIMITIVES DEMO 🔒")
    vibez.spill("")
    
    demo_mutex()
    vibez.spill("")
    
    demo_rwlock()
    vibez.spill("")
    
    demo_semaphore()
    vibez.spill("")
    
    demo_once()
    vibez.spill("")
    
    demo_atomic()
    vibez.spill("")
    
    vibez.spill("🎉 All synchronization demos completed! 🎉")
    damn 0
}
