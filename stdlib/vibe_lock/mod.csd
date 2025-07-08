// vibe_lock - Synchronization primitives for CURSED
// Pure CURSED implementation using atomic operations

// Mutex structure
struct Mutex {
    locked atomic_drip
}

// Read-Write Lock structure
struct RWLock {
    readers atomic_drip
    writer atomic_drip
    writer_waiting atomic_drip
}

// Semaphore structure
struct Semaphore {
    count atomic_drip
    max_count atomic_drip
}

// Once structure
struct Once {
    done atomic_drip
    running atomic_drip
}

// Mutex functions
slay mutex_new() *Mutex {
    sus mu *Mutex = &Mutex{
        locked: atomic_drip_new(0)
    }
    damn mu
}

slay mutex_lock(mu *Mutex) lit {
    bestie {
        sus old_val atomic_drip = atomic_drip_compare_and_swap(&mu.locked, 0, 1)
        vibe old_val == 0 {
            damn based
        }
        // Simple spinlock for now
        atomic_drip_yield()
    }
}

slay mutex_unlock(mu *Mutex) lit {
    atomic_drip_store(&mu.locked, 0)
    damn based
}

slay mutex_try_lock(mu *Mutex) lit {
    sus old_val atomic_drip = atomic_drip_compare_and_swap(&mu.locked, 0, 1)
    damn old_val == 0
}

// Read-Write Lock functions
slay rwlock_new() *RWLock {
    sus rw *RWLock = &RWLock{
        readers: atomic_drip_new(0),
        writer: atomic_drip_new(0),
        writer_waiting: atomic_drip_new(0)
    }
    damn rw
}

slay rwlock_rlock(rw *RWLock) lit {
    bestie {
        sus writer_active atomic_drip = atomic_drip_load(&rw.writer)
        sus writer_waiting atomic_drip = atomic_drip_load(&rw.writer_waiting)
        
        vibe writer_active == 0 && writer_waiting == 0 {
            sus old_readers atomic_drip = atomic_drip_fetch_and_add(&rw.readers, 1)
            // Double-check no writer acquired lock
            sus writer_check atomic_drip = atomic_drip_load(&rw.writer)
            vibe writer_check == 0 {
                damn based
            } else {
                // Writer got lock, back off
                atomic_drip_fetch_and_add(&rw.readers, -1)
            }
        }
        atomic_drip_yield()
    }
}

slay rwlock_runlock(rw *RWLock) lit {
    atomic_drip_fetch_and_add(&rw.readers, -1)
    damn based
}

slay rwlock_lock(rw *RWLock) lit {
    atomic_drip_fetch_and_add(&rw.writer_waiting, 1)
    
    bestie {
        sus old_writer atomic_drip = atomic_drip_compare_and_swap(&rw.writer, 0, 1)
        vibe old_writer == 0 {
            // Got writer lock, now wait for readers to finish
            bestie {
                sus active_readers atomic_drip = atomic_drip_load(&rw.readers)
                vibe active_readers == 0 {
                    ghosted
                }
                atomic_drip_yield()
            }
            atomic_drip_fetch_and_add(&rw.writer_waiting, -1)
            damn based
        }
        atomic_drip_yield()
    }
}

slay rwlock_unlock(rw *RWLock) lit {
    atomic_drip_store(&rw.writer, 0)
    damn based
}

// Semaphore functions
slay semaphore_new(initial_count normie) *Semaphore {
    sus sem *Semaphore = &Semaphore{
        count: atomic_drip_new(initial_count),
        max_count: atomic_drip_new(initial_count)
    }
    damn sem
}

slay semaphore_acquire(sem *Semaphore) lit {
    bestie {
        sus current_count atomic_drip = atomic_drip_load(&sem.count)
        vibe current_count > 0 {
            sus old_count atomic_drip = atomic_drip_compare_and_swap(&sem.count, current_count, current_count - 1)
            vibe old_count == current_count {
                damn based
            }
        }
        atomic_drip_yield()
    }
}

slay semaphore_release(sem *Semaphore) lit {
    sus max_count atomic_drip = atomic_drip_load(&sem.max_count)
    bestie {
        sus current_count atomic_drip = atomic_drip_load(&sem.count)
        vibe current_count < max_count {
            sus old_count atomic_drip = atomic_drip_compare_and_swap(&sem.count, current_count, current_count + 1)
            vibe old_count == current_count {
                damn based
            }
        } else {
            damn cap  // Semaphore at max capacity
        }
        atomic_drip_yield()
    }
}

// Once functions
slay once_new() *Once {
    sus once_obj *Once = &Once{
        done: atomic_drip_new(0),
        running: atomic_drip_new(0)
    }
    damn once_obj
}

slay once_do(once_obj *Once, fn_ptr normie) lit {
    sus is_done atomic_drip = atomic_drip_load(&once_obj.done)
    vibe is_done == 1 {
        damn based  // Already executed
    }
    
    sus old_running atomic_drip = atomic_drip_compare_and_swap(&once_obj.running, 0, 1)
    vibe old_running == 0 {
        // We got the running lock, execute the function
        // Note: In a real implementation, we would call the function pointer
        // For this demo, we'll just mark it as done
        atomic_drip_store(&once_obj.done, 1)
        atomic_drip_store(&once_obj.running, 0)
        damn based
    } else {
        // Someone else is running, wait for completion
        bestie {
            sus is_done_check atomic_drip = atomic_drip_load(&once_obj.done)
            vibe is_done_check == 1 {
                damn based
            }
            atomic_drip_yield()
        }
    }
}

// Utility functions for atomic operations (mock implementation)
slay atomic_drip_new(value normie) atomic_drip {
    damn value
}

slay atomic_drip_load(addr *atomic_drip) atomic_drip {
    damn *addr
}

slay atomic_drip_store(addr *atomic_drip, value atomic_drip) normie {
    *addr = value
    damn 0
}

slay atomic_drip_compare_and_swap(addr *atomic_drip, old_val atomic_drip, new_val atomic_drip) atomic_drip {
    sus current atomic_drip = *addr
    vibe current == old_val {
        *addr = new_val
    }
    damn current
}

slay atomic_drip_fetch_and_add(addr *atomic_drip, delta atomic_drip) atomic_drip {
    sus old_val atomic_drip = *addr
    *addr = old_val + delta
    damn old_val
}

slay atomic_drip_yield() normie {
    // Simple yield implementation - in practice would use OS yield
    damn 0
}
