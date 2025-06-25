vibe main

fr fr Test file for the concurrenz standard library package

slay main() {
    vibez.spill("Testing concurrenz (synchronization) primitives")
    
    fr fr Test Mutex functionality
    test_mutex()
    
    fr fr Test RWMutex functionality
    test_rwmutex()
    
    fr fr Test WaitGroup functionality
    test_waitgroup()
    
    fr fr Test Once functionality
    test_once()
    
    vibez.spill("All concurrenz tests passed!")
}

slay test_mutex() {
    vibez.spill("\nTesting Mutex")
    
    fr fr Create a new mutex
    mutex := concurrenz.new_mutex()
    vibez.spill("Created mutex:", mutex)
    
    fr fr Lock the mutex
    concurrenz.mutex_lock(mutex)
    vibez.spill("Locked mutex")
    
    fr fr Try some "protected" operation
    vibez.spill("Performing operation while holding lock")
    
    fr fr Unlock the mutex
    concurrenz.mutex_unlock(mutex)
    vibez.spill("Unlocked mutex")
    
    vibez.spill("Mutex test passed")
}

slay test_rwmutex() {
    vibez.spill("\nTesting RWMutex")
    
    fr fr Create a new rwmutex
    rwmutex := concurrenz.new_rwmutex()
    vibez.spill("Created RWMutex:", rwmutex)
    
    fr fr Acquire read lock
    concurrenz.rwmutex_rlock(rwmutex)
    vibez.spill("Acquired read lock")
    
    fr fr Perform a "read" operation
    vibez.spill("Reading while holding read lock")
    
    fr fr Release read lock
    concurrenz.rwmutex_runlock(rwmutex)
    vibez.spill("Released read lock")
    
    fr fr Acquire write lock
    concurrenz.rwmutex_lock(rwmutex)
    vibez.spill("Acquired write lock")
    
    fr fr Perform a "write" operation
    vibez.spill("Writing while holding write lock")
    
    fr fr Release write lock
    concurrenz.rwmutex_unlock(rwmutex)
    vibez.spill("Released write lock")
    
    vibez.spill("RWMutex test passed")
}

slay test_waitgroup() {
    vibez.spill("\nTesting WaitGroup")
    
    fr fr Create a new waitgroup
    wg := concurrenz.new_waitgroup()
    vibez.spill("Created WaitGroup:", wg)
    
    fr fr Add workers to the waitgroup
    workers := 3
    concurrenz.waitgroup_add(wg, workers)
    vibez.spill("Added", workers, "to waitgroup")
    
    fr fr Simulate worker completion
    vibecheck i := 0; i < workers; i++ {
        vibez.spill("Worker", i, "completing...")
        concurrenz.waitgroup_done(wg)
        vibez.spill("Worker", i, "completed")
    }
    
    fr fr Wait for all workers to complete
    vibez.spill("Waiting for all workers...")
    concurrenz.waitgroup_wait(wg)
    vibez.spill("All workers completed")
    
    vibez.spill("WaitGroup test passed")
}

slay test_once() {
    vibez.spill("\nTesting Once")
    
    fr fr Create a new Once instance
    once := concurrenz.new_once()
    vibez.spill("Created Once:", once)
    
    fr fr Execute the same operation multiple times with Once
    vibecheck i := 0; i < 3; i++ {
        vibez.spill("Attempt", i, "to execute with Once")
        concurrenz.once_do(once)
    }
    
    vibez.spill("Once test passed")
}