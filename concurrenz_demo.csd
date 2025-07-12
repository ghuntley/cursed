yeet "concurrenz"

vibez.spill("=== CURSED Concurrenz Demo ===")

# Test basic mutex operations
vibez.spill("Testing Mutex...")
sus m := concurrenz.mutex_new()
vibez.spill("Mutex created")

sus lock_result := concurrenz.mutex_lock(&m)
vibez.spill("Mutex locked:", lock_result)

sus unlock_result := concurrenz.mutex_unlock(&m)
vibez.spill("Mutex unlocked:", unlock_result)

# Test atomic operations
vibez.spill("\nTesting AtomicInt...")
sus ai := concurrenz.atomic_int_new(42)
sus value := concurrenz.atomic_int_load(&ai)
vibez.spill("Initial atomic value:", value)

sus new_value := concurrenz.atomic_int_add(&ai, 10)
vibez.spill("After adding 10:", new_value)

# Test WaitGroup
vibez.spill("\nTesting WaitGroup...")
sus wg := concurrenz.waitgroup_new()
concurrenz.waitgroup_add(&wg, 3)
vibez.spill("WaitGroup count:", wg.count)

concurrenz.waitgroup_done(&wg)
vibez.spill("After done:", wg.count)

# Test Semaphore
vibez.spill("\nTesting Semaphore...")
sus sem := concurrenz.semaphore_new(2)
vibez.spill("Semaphore available:", sem.available)

concurrenz.semaphore_acquire(&sem)
vibez.spill("After acquire:", sem.available)

concurrenz.semaphore_release(&sem)
vibez.spill("After release:", sem.available)

vibez.spill("\n=== Concurrenz Demo Complete ===")
