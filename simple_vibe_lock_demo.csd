// Simple demo of vibe_lock synchronization concepts

slay main() normie {
    vibez.spill("🔒 VIBE_LOCK SYNCHRONIZATION DEMO 🔒")
    vibez.spill("")
    
    // Mutex demo
    vibez.spill("=== MUTEX DEMO ===")
    sus mutex_value normie = 0
    vibez.spill("Mutex protecting shared resource: 0")
    
    // Simulate critical section
    vibez.spill("Acquiring mutex...")
    mutex_value = mutex_value + 1
    vibez.spill("Updated shared resource: 1")
    vibez.spill("Releasing mutex...")
    vibez.spill("Mutex demo completed!")
    vibez.spill("")
    
    // Read-Write Lock demo
    vibez.spill("=== READ-WRITE LOCK DEMO ===")
    sus config_data normie = 42
    vibez.spill("Configuration data: 42")
    
    vibez.spill("Multiple readers accessing data...")
    vibez.spill("Reader 1: Data is 42")
    vibez.spill("Reader 2: Data is 42")
    vibez.spill("Reader 3: Data is 42")
    
    vibez.spill("Writer updating data...")
    config_data = 100
    vibez.spill("Writer: Updated data to 100")
    vibez.spill("RWLock demo completed!")
    vibez.spill("")
    
    // Semaphore demo
    vibez.spill("=== SEMAPHORE DEMO ===")
    sus permits normie = 3
    vibez.spill("Semaphore with 3 permits")
    
    vibez.spill("Worker 1 acquiring permit...")
    permits = permits - 1
    vibez.spill("Worker 1: Using resource (permits left: 2)")
    permits = permits + 1
    vibez.spill("Worker 1: Released permit")
    
    vibez.spill("Worker 2 acquiring permit...")
    permits = permits - 1
    vibez.spill("Worker 2: Using resource (permits left: 2)")
    permits = permits + 1
    vibez.spill("Worker 2: Released permit")
    vibez.spill("Semaphore demo completed!")
    vibez.spill("")
    
    // Once demo
    vibez.spill("=== ONCE INITIALIZATION DEMO ===")
    sus initialized lit = cap
    vibez.spill("System not initialized")
    
    // First call
    vibe initialized == cap {
        vibez.spill("Performing expensive initialization...")
        initialized = based
        vibez.spill("Initialization completed!")
    }
    
    // Second call
    vibe initialized == based {
        vibez.spill("Already initialized, skipping...")
    }
    vibez.spill("Once demo completed!")
    vibez.spill("")
    
    // Atomic demo
    vibez.spill("=== ATOMIC COUNTER DEMO ===")
    sus counter normie = 0
    vibez.spill("Atomic counter: 0")
    
    counter = counter + 1
    vibez.spill("Thread 1: Counter incremented to 1")
    
    counter = counter + 1
    vibez.spill("Thread 2: Counter incremented to 2")
    
    counter = counter + 1
    vibez.spill("Thread 3: Counter incremented to 3")
    
    vibez.spill("Final counter value: 3")
    vibez.spill("Atomic demo completed!")
    vibez.spill("")
    
    vibez.spill("🎉 All synchronization demos completed! 🎉")
    damn 0
}
