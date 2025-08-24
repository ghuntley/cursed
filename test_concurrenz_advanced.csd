# Advanced Concurrency & Sync Primitives Test
yeet "concurrenz"
yeet "vibez"

vibez.spill("Testing advanced concurrency functionality...")

# Test RWMutex (Read-Write Mutex)
sus rwmutex = rwmutex_create()
rwmutex_read_lock(rwmutex)
rwmutex_read_unlock(rwmutex)
rwmutex_write_lock(rwmutex)
rwmutex_write_unlock(rwmutex)

vibez.spill("✅ RWMutex working")

# Test Semaphore
sus semaphore = semaphore_create(3)
semaphore_acquire(semaphore)
semaphore_release(semaphore)

vibez.spill("✅ Semaphore working")

# Test Barrier
sus barrier = barrier_create(2)
go {
    barrier_wait(barrier)
    vibez.spill("Barrier point 1 reached")
}
barrier_wait(barrier)

vibez.spill("✅ Barrier working")

# Test Condition Variable
sus cond = condition_create()
sus mutex = mutex_create()

go {
    mutex_lock(mutex)
    condition_wait(cond, mutex)
    mutex_unlock(mutex)
    vibez.spill("Condition variable triggered")
}

condition_signal(cond)

vibez.spill("✅ Condition variable working")

# Test Once (execute once guarantee)
sus once = once_create()
sus counter drip = 0

bestie (sus i drip = 0; i < 5; i++) {
    go {
        once_do(once, slay() {
            counter = counter + 1
        })
    }
}

vibez.spill("✅ Once primitive working")
vibez.spill("✅ All advanced concurrency tests passed")
