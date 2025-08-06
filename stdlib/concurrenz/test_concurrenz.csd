yeet "testz"
yeet "concurrenz"

fr fr Test suite for CURSED Concurrency Module (concurrenz)

slay test_mutex_operations() {
    test_start("Mutex Operations")
    
    fr fr Test mutex creation
    sus mutex *Mutex = create_mutex()
    assert_true(mutex != 0)
    assert_eq_int(mutex.lock_state, 0)
    assert_eq_int(mutex.waiters, 0)
    
    fr fr Test mutex locking
    sus locked lit = mutex_lock(mutex)
    assert_true(locked)
    assert_eq_int(mutex.lock_state, 1)
    
    fr fr Test mutex unlocking
    sus unlocked lit = mutex_unlock(mutex)
    assert_true(unlocked)
    assert_eq_int(mutex.lock_state, 0)
    
    fr fr Test try lock
    sus try_locked lit = mutex_trylock(mutex)
    assert_true(try_locked)
    
    fr fr Test invalid mutex operations
    sus invalid_lock lit = mutex_lock(0)
    assert_false(invalid_lock)
    
    sus invalid_unlock lit = mutex_unlock(0)
    assert_false(invalid_unlock)
    
    vibez.spill("✅ Mutex operation tests passed")
}

slay test_waitgroup_operations() {
    test_start("WaitGroup Operations")
    
    fr fr Test waitgroup creation
    sus wg *WaitGroup = create_waitgroup()
    assert_true(wg != 0)
    assert_eq_int(wg.counter, 0)
    assert_eq_int(wg.waiters, 0)
    
    fr fr Test adding count
    sus added lit = waitgroup_add(wg, 3)
    assert_true(added)
    assert_eq_int(wg.counter, 3)
    
    fr fr Test done operations
    sus done1 lit = waitgroup_done(wg)
    assert_true(done1)
    assert_eq_int(wg.counter, 2)
    
    sus done2 lit = waitgroup_done(wg)
    assert_true(done2)
    assert_eq_int(wg.counter, 1)
    
    sus done3 lit = waitgroup_done(wg)
    assert_true(done3)
    assert_eq_int(wg.counter, 0)
    
    fr fr Test wait operation
    sus waited lit = waitgroup_wait(wg)
    assert_true(waited)
    
    fr fr Test invalid operations
    sus invalid_add lit = waitgroup_add(0, 1)
    assert_false(invalid_add)
    
    sus invalid_done lit = waitgroup_done(0)
    assert_false(invalid_done)
    
    vibez.spill("✅ WaitGroup operation tests passed")
}

slay test_channel_operations() {
    test_start("Channel Operations")
    
    fr fr Test buffered channel creation
    sus ch *Channel = create_channel(3)
    assert_true(ch != 0)
    assert_eq_int(ch.capacity, 3)
    assert_eq_int(ch.size, 0)
    assert_eq_int(ch.closed, 0)
    
    fr fr Test channel send
    sus sent1 lit = channel_send(ch, 42)
    assert_true(sent1)
    assert_eq_int(ch.size, 1)
    
    sus sent2 lit = channel_send(ch, 84)
    assert_true(sent2)
    assert_eq_int(ch.size, 2)
    
    fr fr Test channel receive
    sus received1 normie = channel_receive(ch)
    assert_eq_int(received1, 42)
    assert_eq_int(ch.size, 1)
    
    sus received2 normie = channel_receive(ch)
    assert_eq_int(received2, 84)
    assert_eq_int(ch.size, 0)
    
    fr fr Test channel closing
    sus closed lit = channel_close(ch)
    assert_true(closed)
    assert_eq_int(ch.closed, 1)
    
    fr fr Test operations on closed channel
    sus sent_to_closed lit = channel_send(ch, 100)
    assert_false(sent_to_closed)
    
    assert_true(channel_is_closed(ch))
    
    fr fr Test synchronous channel
    sus sync_ch *Channel = create_sync_channel()
    assert_true(sync_ch != 0)
    assert_eq_int(sync_ch.capacity, 0)
    
    vibez.spill("✅ Channel operation tests passed")
}

slay test_atomic_operations() {
    test_start("Atomic Operations")
    
    fr fr Test atomic integer creation
    sus atomic_i32 *AtomicI32 = atomic_i32_new(10)
    assert_true(atomic_i32 != 0)
    assert_eq_int(atomic_i32.value, 10)
    
    fr fr Test atomic load and store
    sus loaded normie = atomic_load_i32(atomic_i32)
    assert_eq_int(loaded, 10)
    
    atomic_store_i32(atomic_i32, 20)
    sus loaded_after_store normie = atomic_load_i32(atomic_i32)
    assert_eq_int(loaded_after_store, 20)
    
    fr fr Test atomic increment
    sus incremented normie = atomic_increment(atomic_i32)
    assert_eq_int(incremented, 21)
    
    fr fr Test atomic decrement
    sus decremented normie = atomic_decrement(atomic_i32)
    assert_eq_int(decremented, 20)
    
    fr fr Test atomic add
    sus added normie = atomic_add_i32(atomic_i32, 5)
    assert_eq_int(added, 25)
    
    fr fr Test atomic subtract
    sus subtracted normie = atomic_sub_i32(atomic_i32, 3)
    assert_eq_int(subtracted, 22)
    
    fr fr Test compare and swap
    sus swapped lit = atomic_cas_i32(atomic_i32, 22, 100)
    assert_true(swapped)
    assert_eq_int(atomic_i32.value, 100)
    
    sus not_swapped lit = atomic_cas_i32(atomic_i32, 50, 200)
    assert_false(not_swapped)
    assert_eq_int(atomic_i32.value, 100)
    
    fr fr Test atomic boolean
    sus atomic_bool *AtomicBool = atomic_bool_new(based)
    assert_true(atomic_bool != 0)
    assert_eq_int(atomic_bool.value, 1)
    
    sus atomic_bool_false *AtomicBool = atomic_bool_new(cap)
    assert_eq_int(atomic_bool_false.value, 0)
    
    vibez.spill("✅ Atomic operation tests passed")
}

slay test_rwmutex_operations() {
    test_start("RWMutex Operations")
    
    fr fr Test RWMutex creation
    sus rwmutex *RWMutex = create_rwmutex()
    assert_true(rwmutex != 0)
    assert_eq_int(rwmutex.readers, 0)
    assert_eq_int(rwmutex.writer, 0)
    
    fr fr Test read locks (multiple allowed)
    sus rlock1 lit = rwmutex_rlock(rwmutex)
    assert_true(rlock1)
    assert_eq_int(rwmutex.readers, 1)
    
    sus rlock2 lit = rwmutex_rlock(rwmutex)
    assert_true(rlock2)
    assert_eq_int(rwmutex.readers, 2)
    
    fr fr Test read unlocks
    sus runlock1 lit = rwmutex_runlock(rwmutex)
    assert_true(runlock1)
    assert_eq_int(rwmutex.readers, 1)
    
    sus runlock2 lit = rwmutex_runlock(rwmutex)
    assert_true(runlock2)
    assert_eq_int(rwmutex.readers, 0)
    
    fr fr Test write lock (exclusive)
    sus wlock lit = rwmutex_lock(rwmutex)
    assert_true(wlock)
    assert_eq_int(rwmutex.writer, 1)
    
    fr fr Test write unlock
    sus wunlock lit = rwmutex_unlock(rwmutex)
    assert_true(wunlock)
    assert_eq_int(rwmutex.writer, 0)
    
    vibez.spill("✅ RWMutex operation tests passed")
}

slay test_barrier_operations() {
    test_start("Barrier Operations")
    
    fr fr Test barrier creation
    sus barrier *Barrier = create_barrier(3)
    assert_true(barrier != 0)
    assert_eq_int(barrier.count, 3)
    assert_eq_int(barrier.arrived, 0)
    
    fr fr Test barrier wait (simplified - single threaded test)
    fr fr In real implementation this would block until all participants arrive
    sus waited1 lit = barrier_wait(barrier)
    assert_true(waited1)
    assert_eq_int(barrier.arrived, 1)
    
    vibez.spill("✅ Barrier operation tests passed")
}

slay test_semaphore_operations() {
    test_start("Semaphore Operations")
    
    fr fr Test semaphore creation
    sus sem *Semaphore = create_semaphore(3)
    assert_true(sem != 0)
    assert_eq_int(sem.permits, 3)
    assert_eq_int(sem.max_permits, 3)
    
    fr fr Test semaphore acquire
    sus acquired1 lit = semaphore_acquire(sem)
    assert_true(acquired1)
    assert_eq_int(sem.permits, 2)
    
    sus acquired2 lit = semaphore_acquire(sem)
    assert_true(acquired2)
    assert_eq_int(sem.permits, 1)
    
    fr fr Test try acquire
    sus try_acquired lit = semaphore_try_acquire(sem)
    assert_true(try_acquired)
    assert_eq_int(sem.permits, 0)
    
    fr fr Test semaphore release
    sus released1 lit = semaphore_release(sem)
    assert_true(released1)
    assert_eq_int(sem.permits, 1)
    
    sus released2 lit = semaphore_release(sem)
    assert_true(released2)
    assert_eq_int(sem.permits, 2)
    
    fr fr Test releasing too many permits
    semaphore_release(sem)  fr fr Now at max
    sus over_release lit = semaphore_release(sem)
    assert_false(over_release)  fr fr Should fail
    
    vibez.spill("✅ Semaphore operation tests passed")
}

slay test_thread_pool_operations() {
    test_start("Thread Pool Operations")
    
    fr fr Test thread pool creation
    sus pool *ThreadPool = create_thread_pool(4, 10)
    assert_true(pool != 0)
    assert_eq_int(pool.queue_size, 0)
    assert_eq_int(pool.active_workers, 0)
    assert_eq_int(pool.shutdown, 0)
    
    fr fr Test task submission
    sus submitted1 lit = thread_pool_submit(pool, 101)
    assert_true(submitted1)
    assert_eq_int(pool.queue_size, 1)
    
    sus submitted2 lit = thread_pool_submit(pool, 102)
    assert_true(submitted2)
    assert_eq_int(pool.queue_size, 2)
    
    fr fr Test thread pool shutdown
    sus shutdown lit = thread_pool_shutdown(pool)
    assert_true(shutdown)
    assert_eq_int(pool.shutdown, 1)
    
    fr fr Test submitting to shutdown pool
    sus submitted_after_shutdown lit = thread_pool_submit(pool, 103)
    assert_false(submitted_after_shutdown)
    
    fr fr Test wait all
    sus waited lit = thread_pool_wait_all(pool)
    assert_true(waited)
    
    vibez.spill("✅ Thread pool operation tests passed")
}

slay test_once_operations() {
    test_start("Once Operations")
    
    fr fr Test once creation
    sus once *Once = create_once()
    assert_true(once != 0)
    assert_eq_int(once.done, 0)
    assert_eq_int(once.in_progress, 0)
    
    fr fr Test once execution
    sus executed lit = once_do(once, 1001)
    assert_true(executed)
    assert_eq_int(once.done, 1)
    
    fr fr Test second execution (should be skipped)
    sus executed_again lit = once_do(once, 1002)
    assert_false(executed_again)  fr fr Should return false since already done
    
    vibez.spill("✅ Once operation tests passed")
}

slay test_condition_variable_operations() {
    test_start("Condition Variable Operations")
    
    fr fr Test condition variable creation
    sus cond *CondVar = create_condition()
    assert_true(cond != 0)
    assert_eq_int(cond.waiter_count, 0)
    assert_eq_int(cond.signal_count, 0)
    
    fr fr Test condition signal
    sus signaled lit = condition_signal(cond)
    assert_true(signaled)
    
    fr fr Test condition broadcast
    sus broadcasted lit = condition_broadcast(cond)
    assert_true(broadcasted)
    
    fr fr Test condition wait (simplified - requires mutex)
    sus mutex *Mutex = create_mutex()
    mutex_lock(mutex)
    
    fr fr In real implementation this would block
    sus waited lit = condition_wait(cond, mutex)
    assert_true(waited)
    
    vibez.spill("✅ Condition variable operation tests passed")
}

slay test_utility_functions() {
    test_start("Utility Functions")
    
    fr fr Test channel creation with make function
    sus ch *Channel = make("channel", 5)
    assert_true(ch != 0)
    assert_eq_int(ch.capacity, 5)
    
    fr fr Test memory fence
    memory_fence()  fr fr Should not crash
    
    fr fr Test goroutine count
    sus goroutines normie = num_goroutines()
    assert_eq_int(goroutines, 1)  fr fr Simplified implementation
    
    fr fr Test runtime yield
    runtime_yield()  fr fr Should not crash
    
    fr fr Test sleep
    sleep_ms(10)  fr fr Should not crash
    
    vibez.spill("✅ Utility function tests passed")
}

slay test_compatibility_functions() {
    test_start("Compatibility Functions")
    
    fr fr Test legacy mutex creation
    sus legacy_mutex *Mutex = mutex_new()
    assert_true(legacy_mutex != 0)
    
    fr fr Test legacy waitgroup creation
    sus legacy_wg *WaitGroup = waitgroup_new()
    assert_true(legacy_wg != 0)
    
    fr fr Test atomic wrapper creation
    sus atomic_new_val *AtomicStruct = atomic_new(42)
    assert_true(atomic_new_val != 0)
    
    fr fr Test atomic load/store
    atomic_store(atomic_new_val, 100)
    sus loaded_val normie = atomic_load(atomic_new_val)
    assert_eq_int(loaded_val, 100)
    
    vibez.spill("✅ Compatibility function tests passed")
}

slay test_error_handling() {
    test_start("Error Handling")
    
    fr fr Test operations with null pointers
    sus null_mutex_lock lit = mutex_lock(0)
    assert_false(null_mutex_lock)
    
    sus null_waitgroup_add lit = waitgroup_add(0, 1)
    assert_false(null_waitgroup_add)
    
    sus null_channel_send lit = channel_send(0, 42)
    assert_false(null_channel_send)
    
    sus null_channel_recv normie = channel_receive(0)
    assert_eq_int(null_channel_recv, 0)
    
    sus null_atomic_increment normie = atomic_increment(0)
    assert_eq_int(null_atomic_increment, 0)
    
    sus null_barrier_wait lit = barrier_wait(0)
    assert_false(null_barrier_wait)
    
    sus null_semaphore_acquire lit = semaphore_acquire(0)
    assert_false(null_semaphore_acquire)
    
    vibez.spill("✅ Error handling tests passed")
}

slay test_comprehensive_concurrency_workflow() {
    test_start("Comprehensive Concurrency Workflow")
    
    fr fr Create synchronization primitives
    sus mutex *Mutex = create_mutex()
    sus wg *WaitGroup = create_waitgroup()
    sus ch *Channel = create_channel(2)
    sus atomic_counter *AtomicI32 = atomic_i32_new(0)
    
    fr fr Simulate concurrent workflow
    waitgroup_add(wg, 1)  fr fr One "goroutine"
    
    fr fr Send data through channel
    channel_send(ch, 100)
    channel_send(ch, 200)
    
    fr fr Process data with synchronization
    mutex_lock(mutex)
    
    sus data1 normie = channel_receive(ch)
    sus data2 normie = channel_receive(ch)
    
    atomic_add_i32(atomic_counter, data1)
    atomic_add_i32(atomic_counter, data2)
    
    mutex_unlock(mutex)
    
    fr fr Mark work as done
    waitgroup_done(wg)
    waitgroup_wait(wg)
    
    fr fr Verify results
    sus final_count normie = atomic_load_i32(atomic_counter)
    assert_eq_int(final_count, 300)
    
    fr fr Clean up
    channel_close(ch)
    assert_true(channel_is_closed(ch))
    
    vibez.spill("✅ Comprehensive concurrency workflow tests passed")
}

slay run_all_concurrenz_tests() {
    vibez.spill("🚀 Starting CURSED Concurrency (concurrenz) Tests")
    
    test_mutex_operations()
    test_waitgroup_operations()
    test_channel_operations()
    test_atomic_operations()
    test_rwmutex_operations()
    test_barrier_operations()
    test_semaphore_operations()
    test_thread_pool_operations()
    test_once_operations()
    test_condition_variable_operations()
    test_utility_functions()
    test_compatibility_functions()
    test_error_handling()
    test_comprehensive_concurrency_workflow()
    
    print_test_summary()
    vibez.spill("✅ All concurrenz tests completed!")
}

fr fr Run tests when this file is executed
run_all_concurrenz_tests()
