yeet "testz"
yeet "concurrenz"

# Comprehensive test suite for concurrenz (sync) module

# Test Mutex operations
test_start("Mutex creation and basic operations")
sus m := concurrenz.mutex_new()
assert_eq_int(m.locked, cap)
assert_eq_int(m.holder, -1)
assert_eq_int(m.queue_size, 0)

# Test mutex locking
sus lock_result := concurrenz.mutex_lock(&m)
assert_true(lock_result)
assert_eq_int(m.locked, based)

# Test mutex unlocking
sus unlock_result := concurrenz.mutex_unlock(&m)
assert_true(unlock_result)
assert_eq_int(m.locked, cap)

# Test mutex try_lock
sus try_lock_result := concurrenz.mutex_try_lock(&m)
assert_true(try_lock_result)
assert_eq_int(m.locked, based)

# Test WaitGroup operations
test_start("WaitGroup creation and operations")
sus wg := concurrenz.waitgroup_new()
assert_eq_int(wg.count, 0)
assert_eq_int(wg.waiting, cap)

# Test WaitGroup add
sus add_result := concurrenz.waitgroup_add(&wg, 3)
assert_true(add_result)
assert_eq_int(wg.count, 3)

# Test WaitGroup done
sus done_result := concurrenz.waitgroup_done(&wg)
assert_true(done_result)
assert_eq_int(wg.count, 2)

# Test WaitGroup negative count protection
sus add_negative := concurrenz.waitgroup_add(&wg, -5)
assert_true(add_negative)
assert_eq_int(wg.count, 0)

# Test Once operations
test_start("Once creation and do operation")
sus once := concurrenz.once_new()
assert_eq_int(once.done, cap)
assert_eq_int(once.running, cap)

# Test Once do operation
slay test_function() lit {
    vibez.spill("Once function executed")
    damn based
}

sus once_result := concurrenz.once_do(&once, test_function)
assert_true(once_result)
assert_eq_int(once.done, based)

# Test Once idempotency
sus once_second := concurrenz.once_do(&once, test_function)
assert_true(once_second)

# Test AtomicInt operations
test_start("AtomicInt creation and operations")
sus ai := concurrenz.atomic_int_new(42)
assert_eq_int(ai.value, 42)

# Test atomic load
sus load_result := concurrenz.atomic_int_load(&ai)
assert_eq_int(load_result, 42)

# Test atomic store
sus store_result := concurrenz.atomic_int_store(&ai, 100)
assert_true(store_result)
sus load_after_store := concurrenz.atomic_int_load(&ai)
assert_eq_int(load_after_store, 100)

# Test atomic add
sus add_result_atomic := concurrenz.atomic_int_add(&ai, 50)
assert_eq_int(add_result_atomic, 150)

# Test atomic compare and swap success
sus cas_success := concurrenz.atomic_int_compare_and_swap(&ai, 150, 200)
assert_true(cas_success)
sus value_after_cas := concurrenz.atomic_int_load(&ai)
assert_eq_int(value_after_cas, 200)

# Test atomic compare and swap failure
sus cas_failure := concurrenz.atomic_int_compare_and_swap(&ai, 999, 300)
assert_false(cas_failure)
sus value_after_failed_cas := concurrenz.atomic_int_load(&ai)
assert_eq_int(value_after_failed_cas, 200)

# Test AtomicBool operations
test_start("AtomicBool creation and operations")
sus ab := concurrenz.atomic_bool_new(cap)
assert_eq_int(ab.value, cap)

# Test atomic bool load
sus bool_load := concurrenz.atomic_bool_load(&ab)
assert_false(bool_load)

# Test atomic bool store
sus bool_store := concurrenz.atomic_bool_store(&ab, based)
assert_true(bool_store)
sus bool_load_after := concurrenz.atomic_bool_load(&ab)
assert_true(bool_load_after)

# Test atomic bool compare and swap
sus bool_cas_success := concurrenz.atomic_bool_compare_and_swap(&ab, based, cap)
assert_true(bool_cas_success)
sus bool_value_after_cas := concurrenz.atomic_bool_load(&ab)
assert_false(bool_value_after_cas)

# Test RWMutex operations
test_start("RWMutex creation and operations")
sus rw := concurrenz.rwmutex_new()
assert_eq_int(rw.readers, 0)
assert_eq_int(rw.writer, cap)
assert_eq_int(rw.write_pending, cap)

# Test read lock
sus read_lock_result := concurrenz.rwmutex_read_lock(&rw)
assert_true(read_lock_result)
assert_eq_int(rw.readers, 1)

# Test multiple read locks
sus read_lock_2 := concurrenz.rwmutex_read_lock(&rw)
assert_true(read_lock_2)
assert_eq_int(rw.readers, 2)

# Test read unlock
sus read_unlock_result := concurrenz.rwmutex_read_unlock(&rw)
assert_true(read_unlock_result)
assert_eq_int(rw.readers, 1)

# Test write lock preparation
sus write_lock_result := concurrenz.rwmutex_write_lock(&rw)
assert_true(write_lock_result)
assert_eq_int(rw.writer, based)

# Test write unlock
sus write_unlock_result := concurrenz.rwmutex_write_unlock(&rw)
assert_true(write_unlock_result)
assert_eq_int(rw.writer, cap)

# Test Condition variable operations
test_start("Condition variable creation and operations")
sus cond_mutex := concurrenz.mutex_new()
sus cond := concurrenz.cond_new(&cond_mutex)
assert_eq_int(cond.waiters, 0)

# Test condition signal
sus signal_result := concurrenz.cond_signal(&cond)
assert_true(signal_result)

# Test condition broadcast
sus broadcast_result := concurrenz.cond_broadcast(&cond)
assert_true(broadcast_result)

# Test Barrier operations
test_start("Barrier creation and operations")
sus barrier := concurrenz.barrier_new(3)
assert_eq_int(barrier.count, 3)
assert_eq_int(barrier.waiting, 0)
assert_eq_int(barrier.generation, 0)

# Test barrier wait (simulated)
sus barrier_wait_result := concurrenz.barrier_wait(&barrier)
assert_true(barrier_wait_result)

# Test Semaphore operations
test_start("Semaphore creation and operations")
sus sem := concurrenz.semaphore_new(5)
assert_eq_int(sem.permits, 5)
assert_eq_int(sem.available, 5)

# Test semaphore acquire
sus acquire_result := concurrenz.semaphore_acquire(&sem)
assert_true(acquire_result)
assert_eq_int(sem.available, 4)

# Test semaphore try acquire
sus try_acquire_result := concurrenz.semaphore_try_acquire(&sem)
assert_true(try_acquire_result)
assert_eq_int(sem.available, 3)

# Test semaphore release
sus release_result := concurrenz.semaphore_release(&sem)
assert_true(release_result)
assert_eq_int(sem.available, 4)

# Test semaphore try acquire when available
sus try_acquire_available := concurrenz.semaphore_try_acquire(&sem)
assert_true(try_acquire_available)

# Test utility functions
test_start("Utility functions")
sus goroutine_id := concurrenz.goroutine_id()
assert_eq_int(goroutine_id, 1)

sus yield_result := concurrenz.goroutine_yield()
assert_true(yield_result)

sus wake_result := concurrenz.goroutine_wake(1)
assert_true(wake_result)

# Test memory fence operations
test_start("Memory fence operations")
sus memory_fence_result := concurrenz.memory_fence()
assert_true(memory_fence_result)

sus acquire_fence_result := concurrenz.acquire_fence()
assert_true(acquire_fence_result)

sus release_fence_result := concurrenz.release_fence()
assert_true(release_fence_result)

# Test concurrent scenarios (simplified)
test_start("Concurrent scenarios simulation")

# Test mutex contention simulation
sus mutex_contention := concurrenz.mutex_new()
sus lock1 := concurrenz.mutex_lock(&mutex_contention)
assert_true(lock1)

# Simulate second lock attempt (would normally block)
sus try_lock_contention := concurrenz.mutex_try_lock(&mutex_contention)
assert_false(try_lock_contention)

sus unlock_contention := concurrenz.mutex_unlock(&mutex_contention)
assert_true(unlock_contention)

# Test atomic operations under contention
sus atomic_contention := concurrenz.atomic_int_new(0)
bestie i := 0; i < 10; i++ {
    sus add_contention := concurrenz.atomic_int_add(&atomic_contention, 1)
    assert_eq_int(add_contention, i + 1)
}

sus final_value := concurrenz.atomic_int_load(&atomic_contention)
assert_eq_int(final_value, 10)

# Test semaphore exhaustion
sus exhausted_sem := concurrenz.semaphore_new(1)
sus acquire_only := concurrenz.semaphore_acquire(&exhausted_sem)
assert_true(acquire_only)

sus try_acquire_exhausted := concurrenz.semaphore_try_acquire(&exhausted_sem)
assert_false(try_acquire_exhausted)

sus release_exhausted := concurrenz.semaphore_release(&exhausted_sem)
assert_true(release_exhausted)

# Test WaitGroup complete cycle
sus complete_wg := concurrenz.waitgroup_new()
sus add_workers := concurrenz.waitgroup_add(&complete_wg, 3)
assert_true(add_workers)

# Simulate worker completion
sus worker1_done := concurrenz.waitgroup_done(&complete_wg)
assert_true(worker1_done)
assert_eq_int(complete_wg.count, 2)

sus worker2_done := concurrenz.waitgroup_done(&complete_wg)
assert_true(worker2_done)
assert_eq_int(complete_wg.count, 1)

sus worker3_done := concurrenz.waitgroup_done(&complete_wg)
assert_true(worker3_done)
assert_eq_int(complete_wg.count, 0)

# Test RWMutex reader-writer scenarios
sus rw_scenario := concurrenz.rwmutex_new()

# Multiple readers
sus reader1 := concurrenz.rwmutex_read_lock(&rw_scenario)
assert_true(reader1)
sus reader2 := concurrenz.rwmutex_read_lock(&rw_scenario)
assert_true(reader2)
assert_eq_int(rw_scenario.readers, 2)

# Release readers
sus reader1_unlock := concurrenz.rwmutex_read_unlock(&rw_scenario)
assert_true(reader1_unlock)
assert_eq_int(rw_scenario.readers, 1)

sus reader2_unlock := concurrenz.rwmutex_read_unlock(&rw_scenario)
assert_true(reader2_unlock)
assert_eq_int(rw_scenario.readers, 0)

# Test edge cases
test_start("Edge cases and error conditions")

# Test mutex unlock without lock
sus unlocked_mutex := concurrenz.mutex_new()
sus unlock_unlocked := concurrenz.mutex_unlock(&unlocked_mutex)
assert_false(unlock_unlocked)

# Test atomic operations with edge values
sus edge_atomic := concurrenz.atomic_int_new(2147483647)  # Max int32
sus edge_load := concurrenz.atomic_int_load(&edge_atomic)
assert_eq_int(edge_load, 2147483647)

# Test semaphore with zero permits
sus zero_sem := concurrenz.semaphore_new(0)
sus zero_try := concurrenz.semaphore_try_acquire(&zero_sem)
assert_false(zero_try)

# Test barrier with single participant
sus single_barrier := concurrenz.barrier_new(1)
sus single_wait := concurrenz.barrier_wait(&single_barrier)
assert_true(single_wait)

print_test_summary()
