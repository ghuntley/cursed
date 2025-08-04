yeet "testz"
yeet "concurrenz"

fr fr Test basic concurrency primitives
test_start("basic mutex operations")

sus mutex := concurrenz.create_mutex()
assert_true(mutex != cringe)

sus lock_result := concurrenz.mutex_lock(mutex)
assert_true(lock_result)

sus unlock_result := concurrenz.mutex_unlock(mutex)
assert_true(unlock_result)

print_test_summary()

fr fr Test mutex trylock
test_start("mutex trylock")

sus mutex2 := concurrenz.create_mutex()

sus try_result := concurrenz.mutex_trylock(mutex2)
assert_true(try_result)

concurrenz.mutex_unlock(mutex2)

print_test_summary()

fr fr Test waitgroup operations
test_start("waitgroup operations")

sus wg := concurrenz.create_waitgroup()
assert_true(wg != cringe)

sus add_result := concurrenz.waitgroup_add(wg, 3)
assert_true(add_result)

sus done1 := concurrenz.waitgroup_done(wg)
assert_true(done1)

sus done2 := concurrenz.waitgroup_done(wg)
assert_true(done2)

sus done3 := concurrenz.waitgroup_done(wg)
assert_true(done3)

sus wait_result := concurrenz.waitgroup_wait(wg)
assert_true(wait_result)

print_test_summary()

fr fr Test channel operations
test_start("channel operations")

sus ch := concurrenz.create_sync_channel()
assert_true(ch != cringe)

sus send_result := concurrenz.channel_send(ch, 42)
assert_true(send_result)

sus received_data := concurrenz.channel_receive(ch)
assert_eq_int(received_data, 42)

print_test_summary()

fr fr Test read-write mutex
test_start("read-write mutex")

sus rwmutex := concurrenz.create_rwmutex()
assert_true(rwmutex != cringe)

sus rlock1 := concurrenz.rwmutex_rlock(rwmutex)
assert_true(rlock1)

sus rlock2 := concurrenz.rwmutex_rlock(rwmutex)
assert_true(rlock2)

sus runlock1 := concurrenz.rwmutex_runlock(rwmutex)
assert_true(runlock1)

sus runlock2 := concurrenz.rwmutex_runlock(rwmutex)
assert_true(runlock2)

sus wlock := concurrenz.rwmutex_lock(rwmutex)
assert_true(wlock)

sus wunlock := concurrenz.rwmutex_unlock(rwmutex)
assert_true(wunlock)

print_test_summary()

fr fr Test atomic operations
test_start("atomic operations")

sus atomic_val := concurrenz.create_mutex()  fr fr Using mutex as atomic storage

sus cas_result := concurrenz.atomic_cas(atomic_val, 0, 1)
assert_true(cas_result)

sus old_val := concurrenz.atomic_increment(atomic_val)
assert_eq_int(old_val, 1)

sus dec_val := concurrenz.atomic_decrement(atomic_val)
assert_eq_int(dec_val, 2)

print_test_summary()

fr fr Test semaphore operations
test_start("semaphore operations")

sus sem := concurrenz.create_semaphore(2)
assert_true(sem != cringe)

sus acquire1 := concurrenz.semaphore_acquire(sem)
assert_true(acquire1)

sus acquire2 := concurrenz.semaphore_acquire(sem)
assert_true(acquire2)

sus release1 := concurrenz.semaphore_release(sem)
assert_true(release1)

sus release2 := concurrenz.semaphore_release(sem)
assert_true(release2)

print_test_summary()

fr fr Test barrier operations
test_start("barrier operations")

sus barrier := concurrenz.create_barrier(3)
assert_true(barrier != cringe)

fr fr Simulate barrier waiting (simplified)
sus wait_result1 := concurrenz.barrier_wait(barrier)
assert_true(wait_result1)

print_test_summary()

fr fr Test condition variables
test_start("condition variables")

sus condition := concurrenz.create_condition()
assert_true(condition != cringe)

sus mutex_for_cond := concurrenz.create_mutex()
concurrenz.mutex_lock(mutex_for_cond)

sus signal_result := concurrenz.condition_signal(condition)
assert_true(signal_result)

sus broadcast_result := concurrenz.condition_broadcast(condition)
assert_true(broadcast_result)

concurrenz.mutex_unlock(mutex_for_cond)

print_test_summary()

fr fr Test once primitive
test_start("once primitive")

sus once := concurrenz.create_once()
assert_true(once != cringe)

sus once_result := concurrenz.once_do(once, "test_function")
assert_true(once_result)

fr fr Second call should return false (already executed)
sus once_again := concurrenz.once_do(once, "test_function")
assert_false(once_again)

print_test_summary()

fr fr Test channel creation function
test_start("channel creation")

sus chan_result := concurrenz.make("normie", 10)
assert_eq_int(chan_result, 10)

print_test_summary()

fr fr Test complex synchronization scenario
test_start("complex synchronization")

sus shared_mutex := concurrenz.create_mutex()
sus shared_wg := concurrenz.create_waitgroup()
sus shared_ch := concurrenz.create_sync_channel()

fr fr Simulate producer-consumer pattern
concurrenz.waitgroup_add(shared_wg, 2)

fr fr Producer simulation
concurrenz.mutex_lock(shared_mutex)
concurrenz.channel_send(shared_ch, 100)
concurrenz.mutex_unlock(shared_mutex)
concurrenz.waitgroup_done(shared_wg)

fr fr Consumer simulation
concurrenz.mutex_lock(shared_mutex)
sus consumed_data := concurrenz.channel_receive(shared_ch)
assert_eq_int(consumed_data, 100)
concurrenz.mutex_unlock(shared_mutex)
concurrenz.waitgroup_done(shared_wg)

concurrenz.waitgroup_wait(shared_wg)

print_test_summary()

fr fr Test atomic struct operations
test_start("atomic struct operations")

sus atomic_struct := concurrenz.atomic_new(42)
assert_true(atomic_struct != cringe)

sus loaded_value := concurrenz.atomic_load(atomic_struct)
assert_eq_int(loaded_value, 42)

concurrenz.atomic_store(atomic_struct, 84)
sus stored_value := concurrenz.atomic_load(atomic_struct)
assert_eq_int(stored_value, 84)

print_test_summary()

fr fr Test mutex struct operations
test_start("mutex struct operations")

sus mutex_struct := concurrenz.mutex_new()
assert_true(mutex_struct != cringe)

print_test_summary()

fr fr Test waitgroup struct operations
test_start("waitgroup struct operations")

sus wg_struct := concurrenz.waitgroup_new()
assert_true(wg_struct != cringe)

print_test_summary()

vibez.spill("concurrenz module comprehensive tests completed")
vibez.spill("All basic synchronization primitives verified")
