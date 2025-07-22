yeet "testz"
yeet "concurrenz"

fr fr Comprehensive Concurrenz Module Tests
fr fr Testing synchronization primitives and thread safety

fr fr Test mutex creation and basic operations
test_start("Mutex Creation and Operations")
sus mutex = create_mutex()
assert_eq_int(mutex, 0)
sus lock_result = mutex_lock(mutex)
assert_true(lock_result)
sus unlock_result = mutex_unlock(mutex)
assert_true(unlock_result)

fr fr Test mutex try-lock functionality
test_start("Mutex Try-Lock Operations")
sus mutex2 = create_mutex()
sus trylock_result = mutex_trylock(mutex2)
assert_true(trylock_result)
sus trylock_again = mutex_trylock(mutex2)
assert_false(trylock_again)
mutex_unlock(mutex2)

fr fr Test wait group creation and operations
test_start("WaitGroup Creation and Operations")
sus wg = create_waitgroup()
assert_eq_int(wg, 0)
sus add_result = waitgroup_add(wg, 3)
assert_true(add_result)
assert_eq_int(wg, 3)

fr fr Test wait group done operations
test_start("WaitGroup Done Operations")
sus done_result = waitgroup_done(wg)
assert_true(done_result)
assert_eq_int(wg, 2)
waitgroup_done(wg)
waitgroup_done(wg)
assert_eq_int(wg, 0)

fr fr Test wait group wait functionality
test_start("WaitGroup Wait Operations")
sus wg2 = create_waitgroup()
waitgroup_add(wg2, 1)
waitgroup_done(wg2)
sus wait_result = waitgroup_wait(wg2)
assert_true(wait_result)

fr fr Test channel creation and communication
test_start("Channel Creation and Communication")
sus channel = create_sync_channel()
assert_eq_int(channel, 0)
sus send_result = channel_send(channel, 42)
assert_true(send_result)
sus received_data = channel_receive(channel)
assert_eq_int(received_data, 42)

fr fr Test read-write mutex operations
test_start("Read-Write Mutex Operations")
sus rwmutex = create_rwmutex()
assert_eq_int(rwmutex, 0)
sus rlock_result = rwmutex_rlock(rwmutex)
assert_true(rlock_result)
assert_eq_int(rwmutex, 1)

fr fr Test multiple read locks
test_start("Multiple Read Locks")
sus rlock2_result = rwmutex_rlock(rwmutex)
assert_true(rlock2_result)
assert_eq_int(rwmutex, 2)
rwmutex_runlock(rwmutex)
rwmutex_runlock(rwmutex)
assert_eq_int(rwmutex, 0)

fr fr Test write lock exclusivity
test_start("Write Lock Exclusivity")
sus wlock_result = rwmutex_lock(rwmutex)
assert_true(wlock_result)
assert_eq_int(rwmutex, -1)
sus wlock_fail = rwmutex_lock(rwmutex)
assert_false(wlock_fail)
rwmutex_unlock(rwmutex)

fr fr Test condition variable creation
test_start("Condition Variable Creation")
sus condition = create_condition()
assert_eq_int(condition, 0)
sus signal_result = condition_signal(condition)
assert_true(signal_result)
assert_eq_int(condition, 1)

fr fr Test condition broadcast
test_start("Condition Broadcast")
sus condition2 = create_condition()
sus broadcast_result = condition_broadcast(condition2)
assert_true(broadcast_result)
assert_eq_int(condition2, 2)

fr fr Test atomic operations
test_start("Atomic Compare and Swap")
sus atomic_var = 10
sus cas_result = atomic_cas(atomic_var, 10, 20)
assert_true(cas_result)
assert_eq_int(atomic_var, 20)
sus cas_fail = atomic_cas(atomic_var, 10, 30)
assert_false(cas_fail)

fr fr Test atomic increment/decrement
test_start("Atomic Increment and Decrement")
sus atomic_counter = 5
sus old_value = atomic_increment(atomic_counter)
assert_eq_int(old_value, 5)
assert_eq_int(atomic_counter, 6)
sus old_dec = atomic_decrement(atomic_counter)
assert_eq_int(old_dec, 6)
assert_eq_int(atomic_counter, 5)

fr fr Test barrier synchronization
test_start("Barrier Synchronization")
sus barrier = create_barrier(2)
assert_eq_int(barrier, 2)
fr fr Simulate first participant
barrier = barrier - 1
assert_eq_int(barrier, 1)
fr fr Simulate second participant
sus barrier_result = barrier_wait(barrier)
assert_true(barrier_result)

fr fr Test semaphore operations
test_start("Semaphore Operations")
sus semaphore = create_semaphore(3)
assert_eq_int(semaphore, 3)
sus acquire_result = semaphore_acquire(semaphore)
assert_true(acquire_result)
assert_eq_int(semaphore, 2)
sus release_result = semaphore_release(semaphore)
assert_true(release_result)
assert_eq_int(semaphore, 3)

fr fr Test semaphore exhaustion
test_start("Semaphore Exhaustion")
sus sem2 = create_semaphore(1)
semaphore_acquire(sem2)
assert_eq_int(sem2, 0)
sus acquire_fail = semaphore_acquire(sem2)
assert_false(acquire_fail)
semaphore_release(sem2)

fr fr Test once primitive
test_start("Once Primitive")
sus once = create_once()
assert_false(once)
sus once_result = once_do(once, "init_function")
assert_true(once_result)
assert_true(once)
sus once_again = once_do(once, "init_function")
assert_false(once_again)

fr fr Test complex synchronization pattern
test_start("Complex Synchronization Pattern")
sus pattern_mutex = create_mutex()
sus pattern_wg = create_waitgroup()
waitgroup_add(pattern_wg, 2)

fr fr Simulate goroutine 1
mutex_lock(pattern_mutex)
waitgroup_done(pattern_wg)
mutex_unlock(pattern_mutex)

fr fr Simulate goroutine 2  
mutex_lock(pattern_mutex)
waitgroup_done(pattern_wg)
mutex_unlock(pattern_mutex)

fr fr Wait for completion
sus pattern_result = waitgroup_wait(pattern_wg)
assert_true(pattern_result)

fr fr Test producer-consumer pattern
test_start("Producer-Consumer Pattern")
sus buffer = create_sync_channel()
sus producer_mutex = create_mutex()
sus consumer_mutex = create_mutex()

fr fr Producer
mutex_lock(producer_mutex)
channel_send(buffer, 100)
mutex_unlock(producer_mutex)

fr fr Consumer
mutex_lock(consumer_mutex)
sus consumed_data = channel_receive(buffer)
assert_eq_int(consumed_data, 100)
mutex_unlock(consumer_mutex)

fr fr Test reader-writer pattern
test_start("Reader-Writer Pattern")
sus rw_data = 42
sus rw_mutex = create_rwmutex()

fr fr Multiple readers
rwmutex_rlock(rw_mutex)
sus read_data1 = rw_data
assert_eq_int(read_data1, 42)
rwmutex_rlock(rw_mutex)
sus read_data2 = rw_data
assert_eq_int(read_data2, 42)
rwmutex_runlock(rw_mutex)
rwmutex_runlock(rw_mutex)

fr fr Single writer
rwmutex_lock(rw_mutex)
rw_data = 84
rwmutex_unlock(rw_mutex)

fr fr Verify write
rwmutex_rlock(rw_mutex)
sus final_data = rw_data
assert_eq_int(final_data, 84)
rwmutex_runlock(rw_mutex)

print_test_summary()
