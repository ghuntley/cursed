yeet "testz"
yeet "concurrenz"

test_start("concurrenz Concurrency Comprehensive Tests")

fr fr ===== MUTEX TESTS =====

test_group("Mutex Synchronization")

fr fr Test mutex creation
sus mutex *Mutex = create_mutex()
assert_not_null(mutex, "Mutex created successfully")

fr fr Test mutex lock/unlock
sus lock_result lit = mutex_lock(mutex)
assert_bool(lock_result, "Mutex lock succeeded")

sus unlock_result lit = mutex_unlock(mutex)
assert_bool(unlock_result, "Mutex unlock succeeded")

fr fr Test mutex try_lock
sus try_lock_result lit = mutex_try_lock(mutex)
assert_bool(try_lock_result, "Mutex try_lock succeeded when unlocked")

unlock_result = mutex_unlock(mutex)
assert_bool(unlock_result, "Mutex unlock after try_lock")

fr fr Test null mutex handling
sus null_result lit = mutex_lock(0)
assert_bool(!null_result, "Null mutex lock handled safely")

fr fr Cleanup mutex
destroy_mutex(mutex)

fr fr ===== WAIT GROUP TESTS =====

test_group("WaitGroup Synchronization")

fr fr Test wait group creation
sus wg *WaitGroup = create_wait_group()
assert_not_null(wg, "WaitGroup created successfully")

fr fr Test wait group add/done operations
wg_add(wg, 3)
sus wait_result lit = wg_done(wg)
assert_bool(wait_result, "WaitGroup done operation succeeded")

wait_result = wg_done(wg)
assert_bool(wait_result, "WaitGroup second done operation")

wait_result = wg_done(wg)
assert_bool(wait_result, "WaitGroup third done operation")

fr fr Test wait group null handling
sus null_wg_result lit = wg_add(0, 1)
assert_bool(!null_wg_result, "Null WaitGroup add handled safely")

fr fr Cleanup wait group
destroy_wait_group(wg)

fr fr ===== CHANNEL TESTS =====

test_group("Channel Communication")

fr fr Test unbuffered channel creation
sus ch *Channel = create_channel(0)
assert_not_null(ch, "Unbuffered channel created")

fr fr Test buffered channel creation
sus buffered_ch *Channel = create_channel(5)
assert_not_null(buffered_ch, "Buffered channel created")

fr fr Test channel send/receive operations
sus send_result lit = channel_send(buffered_ch, 42)
assert_bool(send_result, "Channel send succeeded")

sus received_value normie = 0
sus recv_result lit = channel_receive(buffered_ch, &received_value)
assert_bool(recv_result, "Channel receive succeeded")
assert_eq_int(received_value, 42, "Channel received correct value")

fr fr Test channel buffering
send_result = channel_send(buffered_ch, 10)
assert_bool(send_result, "Channel send 1")
send_result = channel_send(buffered_ch, 20)
assert_bool(send_result, "Channel send 2")
send_result = channel_send(buffered_ch, 30)
assert_bool(send_result, "Channel send 3")

sus val1 normie = 0
sus val2 normie = 0
sus val3 normie = 0

recv_result = channel_receive(buffered_ch, &val1)
assert_bool(recv_result, "Channel receive 1")
recv_result = channel_receive(buffered_ch, &val2)
assert_bool(recv_result, "Channel receive 2")
recv_result = channel_receive(buffered_ch, &val3)
assert_bool(recv_result, "Channel receive 3")

assert_eq_int(val1, 10, "First value correct")
assert_eq_int(val2, 20, "Second value correct")
assert_eq_int(val3, 30, "Third value correct")

fr fr Test channel close
sus close_result lit = channel_close(buffered_ch)
assert_bool(close_result, "Channel close succeeded")

fr fr Test receive from closed channel
recv_result = channel_receive(buffered_ch, &received_value)
assert_bool(!recv_result, "Receive from closed channel returns false")

fr fr Test send to closed channel
send_result = channel_send(buffered_ch, 99)
assert_bool(!send_result, "Send to closed channel returns false")

fr fr Cleanup channels
destroy_channel(ch)
destroy_channel(buffered_ch)

fr fr ===== ATOMIC OPERATIONS TESTS =====

test_group("Atomic Operations")

fr fr Test atomic i32 operations
sus atomic_i32 *AtomicI32 = create_atomic_i32(0)
assert_not_null(atomic_i32, "AtomicI32 created")

sus load_result normie = atomic_i32_load(atomic_i32)
assert_eq_int(load_result, 0, "AtomicI32 initial value")

atomic_i32_store(atomic_i32, 42)
load_result = atomic_i32_load(atomic_i32)
assert_eq_int(load_result, 42, "AtomicI32 store/load")

sus add_result normie = atomic_i32_fetch_add(atomic_i32, 10)
assert_eq_int(add_result, 42, "AtomicI32 fetch_add returns old value")
load_result = atomic_i32_load(atomic_i32)
assert_eq_int(load_result, 52, "AtomicI32 add result correct")

sus compare_result lit = atomic_i32_compare_exchange(atomic_i32, 52, 100)
assert_bool(compare_result, "AtomicI32 compare_exchange succeeded")
load_result = atomic_i32_load(atomic_i32)
assert_eq_int(load_result, 100, "AtomicI32 compare_exchange value")

fr fr Test failed compare_exchange
compare_result = atomic_i32_compare_exchange(atomic_i32, 99, 200)
assert_bool(!compare_result, "AtomicI32 compare_exchange failed correctly")
load_result = atomic_i32_load(atomic_i32)
assert_eq_int(load_result, 100, "AtomicI32 value unchanged after failed CAS")

destroy_atomic_i32(atomic_i32)

fr fr Test atomic bool operations
sus atomic_bool *AtomicBool = create_atomic_bool(cap)
assert_not_null(atomic_bool, "AtomicBool created")

sus bool_load lit = atomic_bool_load(atomic_bool)
assert_bool(!bool_load, "AtomicBool initial value false")

atomic_bool_store(atomic_bool, no_cap)
bool_load = atomic_bool_load(atomic_bool)
assert_bool(bool_load, "AtomicBool store/load true")

sus exchange_result lit = atomic_bool_exchange(atomic_bool, cap)
assert_bool(exchange_result, "AtomicBool exchange returns old value")
bool_load = atomic_bool_load(atomic_bool)
assert_bool(!bool_load, "AtomicBool exchange result")

destroy_atomic_bool(atomic_bool)

fr fr ===== THREAD POOL TESTS =====

test_group("Thread Pool")

fr fr Test thread pool creation
sus pool *ThreadPool = create_thread_pool(4)
assert_not_null(pool, "ThreadPool created")

fr fr Test task submission (simplified test)
sus task_submitted lit = thread_pool_submit_task(pool, 1001)
assert_bool(task_submitted, "Task submitted to thread pool")

fr fr Test thread pool shutdown
sus shutdown_result lit = thread_pool_shutdown(pool)
assert_bool(shutdown_result, "ThreadPool shutdown succeeded")

destroy_thread_pool(pool)

fr fr ===== BARRIER TESTS =====

test_group("Barrier Synchronization")

fr fr Test barrier creation
sus barrier *Barrier = create_barrier(3)
assert_not_null(barrier, "Barrier created")

fr fr Test barrier wait (simplified single-threaded test)
sus barrier_result lit = barrier_wait(barrier)
assert_bool(barrier_result, "Barrier wait operation")

destroy_barrier(barrier)

fr fr ===== SEMAPHORE TESTS =====

test_group("Semaphore")

fr fr Test semaphore creation
sus sem *Semaphore = create_semaphore(2)
assert_not_null(sem, "Semaphore created")

fr fr Test semaphore acquire/release
sus acquire_result lit = semaphore_acquire(sem)
assert_bool(acquire_result, "Semaphore acquire succeeded")

acquire_result = semaphore_acquire(sem)
assert_bool(acquire_result, "Semaphore second acquire succeeded")

sus release_result lit = semaphore_release(sem)
assert_bool(release_result, "Semaphore release succeeded")

release_result = semaphore_release(sem)
assert_bool(release_result, "Semaphore second release succeeded")

fr fr Test try_acquire
sus try_acquire_result lit = semaphore_try_acquire(sem)
assert_bool(try_acquire_result, "Semaphore try_acquire succeeded when permits available")

destroy_semaphore(sem)

fr fr ===== READ-WRITE MUTEX TESTS =====

test_group("Read-Write Mutex")

fr fr Test RWMutex creation
sus rw_mutex *RWMutex = create_rw_mutex()
assert_not_null(rw_mutex, "RWMutex created")

fr fr Test read lock
sus read_lock_result lit = rw_mutex_read_lock(rw_mutex)
assert_bool(read_lock_result, "RWMutex read lock succeeded")

sus read_unlock_result lit = rw_mutex_read_unlock(rw_mutex)
assert_bool(read_unlock_result, "RWMutex read unlock succeeded")

fr fr Test write lock
sus write_lock_result lit = rw_mutex_write_lock(rw_mutex)
assert_bool(write_lock_result, "RWMutex write lock succeeded")

sus write_unlock_result lit = rw_mutex_write_unlock(rw_mutex)
assert_bool(write_unlock_result, "RWMutex write unlock succeeded")

destroy_rw_mutex(rw_mutex)

fr fr ===== CONDITION VARIABLE TESTS =====

test_group("Condition Variable")

fr fr Test condition variable creation
sus cond *CondVar = create_condition_variable()
assert_not_null(cond, "CondVar created")

fr fr Test signal/broadcast operations
sus signal_result lit = cond_var_signal(cond)
assert_bool(signal_result, "CondVar signal succeeded")

sus broadcast_result lit = cond_var_broadcast(cond)
assert_bool(broadcast_result, "CondVar broadcast succeeded")

destroy_condition_variable(cond)

fr fr ===== ONCE TESTS =====

test_group("Once Synchronization")

fr fr Test Once creation
sus once *Once = create_once()
assert_not_null(once, "Once created")

fr fr Test Once do operation
sus once_result lit = once_do(once, 12345)  fr fr Simplified function ID
assert_bool(once_result, "Once do operation succeeded")

fr fr Test Once do operation second time (should be no-op)
once_result = once_do(once, 12345)
assert_bool(!once_result, "Once do operation only runs once")

destroy_once(once)

fr fr ===== MEMORY ORDERING TESTS =====

test_group("Memory Ordering and Atomics")

fr fr Test memory ordering constants
assert_eq_int(RELAXED, 0, "RELAXED memory ordering constant")
assert_eq_int(ACQUIRE, 1, "ACQUIRE memory ordering constant")
assert_eq_int(RELEASE, 2, "RELEASE memory ordering constant")
assert_eq_int(ACQREL, 3, "ACQREL memory ordering constant")
assert_eq_int(SEQCST, 4, "SEQCST memory ordering constant")

fr fr Test fence operations
sus fence_result lit = atomic_fence(SEQCST)
assert_bool(fence_result, "Atomic fence operation")

fr fr ===== ERROR HANDLING AND EDGE CASES =====

test_group("Error Handling and Edge Cases")

fr fr Test null pointer handling
sus null_mutex_result lit = mutex_lock(0)
assert_bool(!null_mutex_result, "Null mutex handled safely")

sus null_channel_result lit = channel_send(0, 42)
assert_bool(!null_channel_result, "Null channel handled safely")

sus null_wg_result lit = wg_add(0, 1)
assert_bool(!null_wg_result, "Null WaitGroup handled safely")

fr fr Test invalid parameters
sus invalid_channel *Channel = create_channel(-1)
assert_null(invalid_channel, "Invalid channel capacity handled")

sus invalid_barrier *Barrier = create_barrier(0)
assert_null(invalid_barrier, "Invalid barrier count handled")

sus invalid_sem *Semaphore = create_semaphore(-1)
assert_null(invalid_sem, "Invalid semaphore permits handled")

fr fr Test resource cleanup
sus test_mutex *Mutex = create_mutex()
assert_not_null(test_mutex, "Test mutex created for cleanup")
destroy_mutex(test_mutex)
fr fr Note: Cannot easily test post-cleanup access without memory debugging

fr fr ===== STRESS TESTING =====

test_group("Concurrency Stress Testing")

fr fr Test channel with multiple operations
sus stress_channel *Channel = create_channel(10)
assert_not_null(stress_channel, "Stress test channel created")

fr fr Fill channel buffer
sus i normie = 0
bestie (i < 10) {
    sus fill_result lit = channel_send(stress_channel, i)
    assert_bool(fill_result, "Channel fill operation")
    i = i + 1
}

fr fr Empty channel buffer
i = 0
bestie (i < 10) {
    sus drain_value normie = 0
    sus drain_result lit = channel_receive(stress_channel, &drain_value)
    assert_bool(drain_result, "Channel drain operation")
    assert_eq_int(drain_value, i, "Channel value order preserved")
    i = i + 1
}

destroy_channel(stress_channel)

fr fr Test atomic operations stress
sus stress_atomic *AtomicI32 = create_atomic_i32(0)
assert_not_null(stress_atomic, "Stress test atomic created")

i = 0
bestie (i < 100) {
    atomic_i32_fetch_add(stress_atomic, 1)
    i = i + 1
}

sus final_value normie = atomic_i32_load(stress_atomic)
assert_eq_int(final_value, 100, "Atomic stress test final value")

destroy_atomic_i32(stress_atomic)

fr fr Test multiple mutex operations
sus stress_mutex *Mutex = create_mutex()
assert_not_null(stress_mutex, "Stress test mutex created")

i = 0
bestie (i < 50) {
    sus stress_lock lit = mutex_lock(stress_mutex)
    assert_bool(stress_lock, "Stress mutex lock")
    sus stress_unlock lit = mutex_unlock(stress_mutex)
    assert_bool(stress_unlock, "Stress mutex unlock")
    i = i + 1
}

destroy_mutex(stress_mutex)

print_test_summary()
