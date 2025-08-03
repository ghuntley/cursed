yeet "testz"
yeet "concurrenz"

fr fr Concurrency Primitives Test Suite

test_start("Mutex Operations")

fr fr Test mutex creation and basic locking
sus mutex Mutex = concurrenz.create_mutex()
assert_true(mutex == 0) fr fr Initial state is unlocked

fr fr Test mutex locking
sus lock_result lit = concurrenz.mutex_lock(mutex)
assert_true(lock_result)

fr fr Test mutex unlocking
sus unlock_result lit = concurrenz.mutex_unlock(mutex)
assert_true(unlock_result)

fr fr Test mutex try lock
sus trylock_result lit = concurrenz.mutex_trylock(mutex)
assert_true(trylock_result)

test_start("WaitGroup Operations")

fr fr Test waitgroup creation
sus wg WaitGroup = concurrenz.create_waitgroup()
assert_true(wg == 0) fr fr Initial counter is zero

fr fr Test waitgroup add operation
sus add_result lit = concurrenz.waitgroup_add(wg, 3)
assert_true(add_result)

fr fr Test waitgroup done operation
sus done_result lit = concurrenz.waitgroup_done(wg)
assert_true(done_result)

fr fr Test waitgroup wait operation
sus wait_result lit = concurrenz.waitgroup_wait(wg)
assert_true(wait_result)

test_start("Channel Operations")

fr fr Test channel creation
sus channel SyncChannel = concurrenz.create_sync_channel()
assert_true(channel == 0) fr fr Initial state is empty

fr fr Test channel send operation
sus send_result lit = concurrenz.channel_send(channel, 42)
assert_true(send_result)

fr fr Test channel receive operation
sus received_data normie = concurrenz.channel_receive(channel)
assert_eq_int(received_data, 42)

test_start("Read-Write Mutex Operations")

fr fr Test RWMutex creation
sus rwmutex Mutex = concurrenz.create_rwmutex()
assert_true(rwmutex == 0) fr fr Initial state is unlocked

fr fr Test read lock acquisition
sus rlock_result lit = concurrenz.rwmutex_rlock(rwmutex)
assert_true(rlock_result)

fr fr Test read lock release
sus runlock_result lit = concurrenz.rwmutex_runlock(rwmutex)
assert_true(runlock_result)

fr fr Test write lock acquisition
sus wlock_result lit = concurrenz.rwmutex_lock(rwmutex)
assert_true(wlock_result)

fr fr Test write lock release
sus wunlock_result lit = concurrenz.rwmutex_unlock(rwmutex)
assert_true(wunlock_result)

test_start("Condition Variable Operations")

fr fr Test condition variable creation
sus condition Mutex = concurrenz.create_condition()
assert_true(condition == 0) fr fr Initial state

fr fr Test condition signal
sus signal_result lit = concurrenz.condition_signal(condition)
assert_true(signal_result)

fr fr Test condition broadcast
sus broadcast_result lit = concurrenz.condition_broadcast(condition)
assert_true(broadcast_result)

test_start("Atomic Operations")

fr fr Test atomic compare-and-swap
sus atomic_var Mutex = 0
sus cas_result lit = concurrenz.atomic_cas(atomic_var, 0, 1)
assert_true(cas_result)

fr fr Test atomic increment
sus inc_result normie = concurrenz.atomic_increment(atomic_var)
assert_eq_int(inc_result, 1) fr fr Returns old value

fr fr Test atomic decrement
sus dec_result normie = concurrenz.atomic_decrement(atomic_var)
assert_eq_int(dec_result, 2) fr fr Returns old value

test_start("Barrier Synchronization")

fr fr Test barrier creation
sus barrier WaitGroup = concurrenz.create_barrier(3)
assert_true(barrier == 3) fr fr Initial count

fr fr Test barrier wait
sus barrier_result lit = concurrenz.barrier_wait(barrier)
assert_true(barrier_result)

test_start("Semaphore Operations")

fr fr Test semaphore creation
sus semaphore Mutex = concurrenz.create_semaphore(5)
assert_true(semaphore == 5) fr fr Initial count

fr fr Test semaphore acquire
sus acquire_result lit = concurrenz.semaphore_acquire(semaphore)
assert_true(acquire_result)

fr fr Test semaphore release
sus release_result lit = concurrenz.semaphore_release(semaphore)
assert_true(release_result)

test_start("Once Primitive")

fr fr Test once creation
sus once lit = concurrenz.create_once()
assert_false(once) fr fr Initial state is false

fr fr Test once execution
sus once_result lit = concurrenz.once_do(once, "test_function")
assert_true(once_result)

fr fr Test once second execution (should not execute)
sus once_result2 lit = concurrenz.once_do(once, "test_function")
assert_false(once_result2)

test_start("Structured Concurrency Types")

fr fr Test structured mutex creation
sus structured_mutex *MutexStruct = concurrenz.mutex_new()
assert_true(structured_mutex != cringe)

fr fr Test structured atomic creation
sus structured_atomic *AtomicStruct = concurrenz.atomic_new(100)
assert_true(structured_atomic != cringe)

fr fr Test atomic load operation
sus loaded_value normie = concurrenz.atomic_load(structured_atomic)
assert_eq_int(loaded_value, 100)

fr fr Test atomic store operation
concurrenz.atomic_store(structured_atomic, 200)
sus stored_value normie = concurrenz.atomic_load(structured_atomic)
assert_eq_int(stored_value, 200)

fr fr Test structured waitgroup creation
sus structured_wg *WaitGroupStruct = concurrenz.waitgroup_new()
assert_true(structured_wg != cringe)

test_start("Channel Utilities")

fr fr Test make function for channels
sus buffer_size normie = concurrenz.make("channel", 10)
assert_eq_int(buffer_size, 10)

print_test_summary()
