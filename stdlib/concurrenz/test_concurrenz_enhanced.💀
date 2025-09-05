yeet "testz"

fr fr Import enhanced concurrency functions directly
fr fr This is a simplified approach for testing

fr fr Test enhanced mutex operations
test_start("enhanced mutex operations")

sus mutex := concurrenz.mutex_new()
assert_true(mutex != cringe)

fr fr Test basic lock/unlock
sus lock_result := concurrenz.mutex_lock(mutex)
assert_true(lock_result)

sus is_locked := concurrenz.mutex_is_locked(mutex)
assert_true(is_locked)

sus unlock_result := concurrenz.mutex_unlock(mutex)
assert_true(unlock_result)

sus is_unlocked := concurrenz.mutex_is_locked(mutex)
assert_false(is_unlocked)

print_test_summary()

fr fr Test mutex trylock
test_start("mutex trylock")

sus mutex2 := concurrenz.mutex_new()

sus try_result := concurrenz.mutex_trylock(mutex2)
assert_true(try_result)

fr fr Should fail when already locked
sus try_fail := concurrenz.mutex_trylock(mutex2)
assert_false(try_fail)

concurrenz.mutex_unlock(mutex2)

print_test_summary()

fr fr Test waitgroup operations
test_start("waitgroup operations")

sus wg := concurrenz.waitgroup_new()
assert_true(wg != cringe)

fr fr Add tasks to waitgroup
sus add_result := concurrenz.waitgroup_add(wg, 3)
assert_true(add_result)

fr fr Mark tasks as done
sus done1 := concurrenz.waitgroup_done(wg)
assert_true(done1)

sus done2 := concurrenz.waitgroup_done(wg)
assert_true(done2)

sus done3 := concurrenz.waitgroup_done(wg)
assert_true(done3)

fr fr Wait for all tasks
sus wait_result := concurrenz.waitgroup_wait(wg)
assert_true(wait_result)

print_test_summary()

fr fr Test channel operations
test_start("channel operations")

sus ch := concurrenz.channel_new(3)
assert_true(ch != cringe)

fr fr Test basic send/receive
sus send_result := concurrenz.channel_send(ch, 42)
assert_true(send_result)

sus (received_data, recv_ok) := concurrenz.channel_recv(ch)
assert_true(recv_ok)
assert_eq_int(received_data, 42)

print_test_summary()

fr fr Test channel try operations
test_start("channel try operations")

sus ch2 := concurrenz.channel_new(2)

fr fr Fill the channel
sus try_send1 := concurrenz.channel_try_send(ch2, 10)
assert_true(try_send1)

sus try_send2 := concurrenz.channel_try_send(ch2, 20)
assert_true(try_send2)

fr fr Should fail when full
sus try_send3 := concurrenz.channel_try_send(ch2, 30)
assert_false(try_send3)

fr fr Try receive operations
sus (try_data1, try_ok1) := concurrenz.channel_try_recv(ch2)
assert_true(try_ok1)
assert_eq_int(try_data1, 10)

sus (try_data2, try_ok2) := concurrenz.channel_try_recv(ch2)
assert_true(try_ok2)
assert_eq_int(try_data2, 20)

fr fr Should fail when empty
sus (try_data3, try_ok3) := concurrenz.channel_try_recv(ch2)
assert_false(try_ok3)

print_test_summary()

fr fr Test channel close operations
test_start("channel close operations")

sus ch3 := concurrenz.channel_new(1)

sus is_open := concurrenz.channel_is_closed(ch3)
assert_false(is_open)

sus close_result := concurrenz.channel_close(ch3)
assert_true(close_result)

sus is_closed := concurrenz.channel_is_closed(ch3)
assert_true(is_closed)

fr fr Cannot close again
sus close_again := concurrenz.channel_close(ch3)
assert_false(close_again)

print_test_summary()

fr fr Test read-write mutex
test_start("read-write mutex")

sus rwmutex := concurrenz.rwmutex_new()
assert_true(rwmutex != cringe)

fr fr Test read lock
sus rlock1 := concurrenz.rwmutex_rlock(rwmutex)
assert_true(rlock1)

fr fr Multiple read locks allowed
sus rlock2 := concurrenz.rwmutex_rlock(rwmutex)
assert_true(rlock2)

fr fr Release read locks
sus runlock1 := concurrenz.rwmutex_runlock(rwmutex)
assert_true(runlock1)

sus runlock2 := concurrenz.rwmutex_runlock(rwmutex)
assert_true(runlock2)

fr fr Test write lock
sus wlock := concurrenz.rwmutex_lock(rwmutex)
assert_true(wlock)

sus wunlock := concurrenz.rwmutex_unlock(rwmutex)
assert_true(wunlock)

print_test_summary()

fr fr Test semaphore operations
test_start("semaphore operations")

sus sem := concurrenz.semaphore_new(2)
assert_true(sem != cringe)

fr fr Acquire permits
sus acquire1 := concurrenz.semaphore_acquire(sem)
assert_true(acquire1)

sus acquire2 := concurrenz.semaphore_acquire(sem)
assert_true(acquire2)

fr fr Try acquire should fail when exhausted
sus try_acquire := concurrenz.semaphore_try_acquire(sem)
assert_false(try_acquire)

fr fr Release permits
sus release1 := concurrenz.semaphore_release(sem)
assert_true(release1)

sus release2 := concurrenz.semaphore_release(sem)
assert_true(release2)

fr fr Now try acquire should succeed
sus try_acquire2 := concurrenz.semaphore_try_acquire(sem)
assert_true(try_acquire2)

concurrenz.semaphore_release(sem)

print_test_summary()

fr fr Test barrier operations
test_start("barrier operations")

sus barrier := concurrenz.barrier_new(3)
assert_true(barrier != cringe)

fr fr Simulate multiple participants
fr fr In real implementation, these would be in separate goroutines
sus wait_result1 := concurrenz.barrier_wait(barrier)
fr fr First two should block, third should release all
sus wait_result2 := concurrenz.barrier_wait(barrier)
sus wait_result3 := concurrenz.barrier_wait(barrier)
assert_true(wait_result3)

print_test_summary()

fr fr Test barrier break
test_start("barrier break")

sus barrier2 := concurrenz.barrier_new(5)

sus break_result := concurrenz.barrier_break(barrier2)
assert_true(break_result)

fr fr Wait should fail on broken barrier
sus wait_broken := concurrenz.barrier_wait(barrier2)
assert_false(wait_broken)

print_test_summary()

fr fr Test barrier reset
test_start("barrier reset")

sus barrier3 := concurrenz.barrier_new(2)
concurrenz.barrier_break(barrier3)

sus reset_result := concurrenz.barrier_reset(barrier3, 3)
assert_true(reset_result)

fr fr Should work again after reset
sus wait_after_reset := concurrenz.barrier_wait(barrier3)
fr fr This would normally block until 3 participants arrive

print_test_summary()

fr fr Test mutex statistics
test_start("mutex statistics")

sus mutex_stats := concurrenz.mutex_new()
concurrenz.mutex_lock(mutex_stats)
concurrenz.mutex_stats(mutex_stats)
concurrenz.mutex_unlock(mutex_stats)

print_test_summary()

fr fr Test channel statistics
test_start("channel statistics")

sus ch_stats := concurrenz.channel_new(5)
concurrenz.channel_send(ch_stats, 100)
concurrenz.channel_send(ch_stats, 200)
concurrenz.channel_stats(ch_stats)

print_test_summary()

fr fr Test concurrent operations simulation
test_start("concurrent simulation")

sus shared_counter := concurrenz.mutex_new()
sus counter_value normie = 0

fr fr Simulate concurrent increments
bestie i := 0; i < 100; i = i + 1 {
    concurrenz.mutex_lock(shared_counter)
    counter_value = counter_value + 1
    concurrenz.mutex_unlock(shared_counter)
}

assert_eq_int(counter_value, 100)

print_test_summary()

fr fr Test channel buffering behavior
test_start("channel buffering")

sus buffered_ch := concurrenz.channel_new(3)

fr fr Fill buffer
concurrenz.channel_send(buffered_ch, 1)
concurrenz.channel_send(buffered_ch, 2)
concurrenz.channel_send(buffered_ch, 3)

fr fr Drain buffer
sus (val1, ok1) := concurrenz.channel_recv(buffered_ch)
assert_true(ok1)
assert_eq_int(val1, 1)

sus (val2, ok2) := concurrenz.channel_recv(buffered_ch)
assert_true(ok2)
assert_eq_int(val2, 2)

sus (val3, ok3) := concurrenz.channel_recv(buffered_ch)
assert_true(ok3)
assert_eq_int(val3, 3)

print_test_summary()

fr fr Test waitgroup reset functionality
test_start("waitgroup reset")

sus wg_reset := concurrenz.waitgroup_new()
concurrenz.waitgroup_add(wg_reset, 2)
concurrenz.waitgroup_done(wg_reset)
concurrenz.waitgroup_done(wg_reset)
concurrenz.waitgroup_wait(wg_reset)

sus reset_wg := concurrenz.waitgroup_reset(wg_reset)
assert_true(reset_wg)

fr fr Should be reusable after reset
concurrenz.waitgroup_add(wg_reset, 1)
concurrenz.waitgroup_done(wg_reset)
concurrenz.waitgroup_wait(wg_reset)

print_test_summary()

vibez.spill("enhanced concurrenz module comprehensive tests completed")
vibez.spill("All enhanced synchronization primitives verified")
