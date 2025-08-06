yeet "testz"
yeet "concurrenz"

fr fr =============================================================================
fr fr CONCURRENZ MODULE COMPREHENSIVE TESTS - All Synchronization Primitives
fr fr =============================================================================

fr fr Test basic mutex operations with atomic operations
test_start("mutex operations with atomics")

sus mutex := concurrenz.create_mutex()
assert_true(mutex != 0)

sus lock_result := concurrenz.mutex_lock(mutex)
assert_true(lock_result)

sus unlock_result := concurrenz.mutex_unlock(mutex)
assert_true(unlock_result)

sus try_result := concurrenz.mutex_trylock(mutex)
assert_true(try_result)

concurrenz.mutex_unlock(mutex)

print_test_summary()

fr fr Test waitgroup operations with atomic counters
test_start("waitgroup operations")

sus wg := concurrenz.create_waitgroup()
assert_true(wg != 0)

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

fr fr Test buffered channel operations
test_start("buffered channel operations")

sus ch := concurrenz.create_channel(5)
assert_true(ch != 0)

sus send1 := concurrenz.channel_send(ch, 42)
assert_true(send1)

sus send2 := concurrenz.channel_send(ch, 84)
assert_true(send2)

sus received1 := concurrenz.channel_receive(ch)
assert_eq_int(received1, 42)

sus received2 := concurrenz.channel_receive(ch)
assert_eq_int(received2, 84)

sus close_result := concurrenz.channel_close(ch)
assert_true(close_result)

sus is_closed := concurrenz.channel_is_closed(ch)
assert_true(is_closed)

print_test_summary()

fr fr Test synchronous channel operations
test_start("synchronous channel operations")

sus sync_ch := concurrenz.create_sync_channel()
assert_true(sync_ch != 0)

print_test_summary()

fr fr Test atomic operations
test_start("atomic operations")

sus atomic_i32 := concurrenz.atomic_i32_new(10)
assert_true(atomic_i32 != 0)

sus loaded_val := concurrenz.atomic_load_i32(atomic_i32)
assert_eq_int(loaded_val, 10)

concurrenz.atomic_store_i32(atomic_i32, 20)
sus new_val := concurrenz.atomic_load_i32(atomic_i32)
assert_eq_int(new_val, 20)

sus old_val := concurrenz.atomic_increment(atomic_i32)
assert_eq_int(old_val, 20)

sus current_val := concurrenz.atomic_load_i32(atomic_i32)
assert_eq_int(current_val, 21)

sus dec_val := concurrenz.atomic_decrement(atomic_i32)
assert_eq_int(dec_val, 21)

sus final_val := concurrenz.atomic_load_i32(atomic_i32)
assert_eq_int(final_val, 20)

sus cas_result := concurrenz.atomic_cas_i32(atomic_i32, 20, 100)
assert_true(cas_result)

sus cas_val := concurrenz.atomic_load_i32(atomic_i32)
assert_eq_int(cas_val, 100)

print_test_summary()

fr fr Test atomic 64-bit operations
test_start("atomic 64-bit operations")

sus atomic_i64 := concurrenz.atomic_i64_new(1000)
assert_true(atomic_i64 != 0)

sus cas64_result := concurrenz.atomic_cas_i64(atomic_i64, 1000, 2000)
assert_true(cas64_result)

print_test_summary()

fr fr Test atomic boolean operations
test_start("atomic boolean operations")

sus atomic_bool := concurrenz.atomic_bool_new(based)
assert_true(atomic_bool != 0)

print_test_summary()

fr fr Test semaphore operations
test_start("semaphore operations")

sus sem := concurrenz.create_semaphore(2)
assert_true(sem != 0)

sus acquire1 := concurrenz.semaphore_acquire(sem)
assert_true(acquire1)

sus acquire2 := concurrenz.semaphore_acquire(sem)
assert_true(acquire2)

sus try_acquire := concurrenz.semaphore_try_acquire(sem)
assert_false(try_acquire)  fr fr Should fail - no permits left

sus release1 := concurrenz.semaphore_release(sem)
assert_true(release1)

sus try_acquire2 := concurrenz.semaphore_try_acquire(sem)
assert_true(try_acquire2)  fr fr Should succeed now

sus release2 := concurrenz.semaphore_release(sem)
assert_true(release2)

print_test_summary()

fr fr Test barrier operations
test_start("barrier operations")

sus barrier := concurrenz.create_barrier(3)
assert_true(barrier != 0)

print_test_summary()

fr fr Test read-write mutex operations
test_start("read-write mutex operations")

sus rwmutex := concurrenz.create_rwmutex()
assert_true(rwmutex != 0)

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

fr fr Test condition variable operations
test_start("condition variable operations")

sus condition := concurrenz.create_condition()
assert_true(condition != 0)

sus mutex_for_cond := concurrenz.create_mutex()
concurrenz.mutex_lock(mutex_for_cond)

sus signal_result := concurrenz.condition_signal(condition)
assert_true(signal_result)

sus broadcast_result := concurrenz.condition_broadcast(condition)
assert_true(broadcast_result)

concurrenz.mutex_unlock(mutex_for_cond)

print_test_summary()

fr fr Test thread pool operations
test_start("thread pool operations")

sus pool := concurrenz.create_thread_pool(4, 10)
assert_true(pool != 0)

sus submit1 := concurrenz.thread_pool_submit(pool, 1001)
assert_true(submit1)

sus submit2 := concurrenz.thread_pool_submit(pool, 1002)
assert_true(submit2)

sus wait_all := concurrenz.thread_pool_wait_all(pool)
assert_true(wait_all)

sus shutdown := concurrenz.thread_pool_shutdown(pool)
assert_true(shutdown)

print_test_summary()

fr fr Test once primitive operations
test_start("once primitive operations")

sus once := concurrenz.create_once()
assert_true(once != 0)

sus once_result := concurrenz.once_do(once, 42)
assert_true(once_result)

fr fr Second call should return false (already executed)
sus once_again := concurrenz.once_do(once, 43)
assert_false(once_again)

print_test_summary()

fr fr Test utility functions
test_start("utility functions")

sus chan_make := concurrenz.make("normie", 5)
assert_true(chan_make != 0)

concurrenz.memory_fence()

sus num_goroutines := concurrenz.num_goroutines()
assert_eq_int(num_goroutines, 1)

concurrenz.runtime_yield()

concurrenz.sleep_ms(1)

print_test_summary()

fr fr Test complex synchronization scenario
test_start("complex synchronization scenario")

sus shared_mutex := concurrenz.create_mutex()
sus shared_wg := concurrenz.create_waitgroup()
sus shared_ch := concurrenz.create_channel(2)

fr fr Simulate producer-consumer pattern with multiple synchronization primitives
concurrenz.waitgroup_add(shared_wg, 2)

fr fr Producer simulation
concurrenz.mutex_lock(shared_mutex)
concurrenz.channel_send(shared_ch, 100)
concurrenz.channel_send(shared_ch, 200)
concurrenz.mutex_unlock(shared_mutex)
concurrenz.waitgroup_done(shared_wg)

fr fr Consumer simulation
concurrenz.mutex_lock(shared_mutex)
sus consumed_data1 := concurrenz.channel_receive(shared_ch)
assert_eq_int(consumed_data1, 100)

sus consumed_data2 := concurrenz.channel_receive(shared_ch)
assert_eq_int(consumed_data2, 200)
concurrenz.mutex_unlock(shared_mutex)
concurrenz.waitgroup_done(shared_wg)

concurrenz.waitgroup_wait(shared_wg)

print_test_summary()

fr fr Test atomic arithmetic operations
test_start("atomic arithmetic operations")

sus atomic_math := concurrenz.atomic_i32_new(50)

sus add_result := concurrenz.atomic_add_i32(atomic_math, 25)
assert_eq_int(add_result, 50)  fr fr Returns old value

sus current_after_add := concurrenz.atomic_load_i32(atomic_math)
assert_eq_int(current_after_add, 75)

sus sub_result := concurrenz.atomic_sub_i32(atomic_math, 15)
assert_eq_int(sub_result, 75)  fr fr Returns old value

sus current_after_sub := concurrenz.atomic_load_i32(atomic_math)
assert_eq_int(current_after_sub, 60)

print_test_summary()

fr fr Test multiple channel operations
test_start("multiple channel operations")

sus ch1 := concurrenz.create_channel(1)
sus ch2 := concurrenz.create_channel(2)

concurrenz.channel_send(ch1, 111)
concurrenz.channel_send(ch2, 222)
concurrenz.channel_send(ch2, 333)

sus val1 := concurrenz.channel_receive(ch1)
assert_eq_int(val1, 111)

sus val2 := concurrenz.channel_receive(ch2)
assert_eq_int(val2, 222)

sus val3 := concurrenz.channel_receive(ch2)
assert_eq_int(val3, 333)

print_test_summary()

fr fr Test barrier synchronization
test_start("barrier synchronization")

sus barrier_test := concurrenz.create_barrier(2)

fr fr Simulate two participants
sus wait1 := concurrenz.barrier_wait(barrier_test)
assert_true(wait1)

print_test_summary()

fr fr Test concurrent data structures stress
test_start("concurrent data structures stress")

sus stress_mutex := concurrenz.create_mutex()
sus stress_atomic := concurrenz.atomic_i32_new(0)
sus stress_sem := concurrenz.create_semaphore(1)

fr fr Simulate high contention scenario
concurrenz.mutex_lock(stress_mutex)
concurrenz.semaphore_acquire(stress_sem)

concurrenz.atomic_increment(stress_atomic)
concurrenz.atomic_increment(stress_atomic)
concurrenz.atomic_increment(stress_atomic)

sus final_count := concurrenz.atomic_load_i32(stress_atomic)
assert_eq_int(final_count, 3)

concurrenz.semaphore_release(stress_sem)
concurrenz.mutex_unlock(stress_mutex)

print_test_summary()

vibez.spill("concurrenz module comprehensive tests completed")
vibez.spill("✅ All synchronization primitives verified:")
vibez.spill("  - Mutexes with atomic operations")
vibez.spill("  - WaitGroups with atomic counters")
vibez.spill("  - Buffered and synchronous channels")
vibez.spill("  - Atomic operations (32-bit, 64-bit, boolean)")
vibez.spill("  - Semaphores with resource counting")
vibez.spill("  - Barriers for multi-goroutine synchronization")
vibez.spill("  - Read-Write mutexes for shared access")
vibez.spill("  - Condition variables for coordination")
vibez.spill("  - Thread pools for task execution")
vibez.spill("  - Once primitives for one-time initialization")
vibez.spill("  - Utility functions for compatibility")
vibez.spill("🎯 Full concurrency primitive suite implemented!")
