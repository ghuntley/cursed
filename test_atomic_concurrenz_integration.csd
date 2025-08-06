yeet "atomic_drip"
yeet "concurrenz"
yeet "testz"

fr fr Test integration between atomic_drip and concurrenz modules
fr fr This verifies that concurrency primitives work with hardware atomics

test_start("Atomic Drip - Concurrenz Integration Test")

fr fr Test 1: Basic atomic operations
test_start("Basic Atomic Operations Test")
sus atomic_counter = atomic_drip.atomic_i32_new(0)
sus initial_value = atomic_drip.atomic_load_i32(atomic_counter)
assert_eq_int(initial_value, 0)

sus old_value = atomic_drip.atomic_add_i32(atomic_counter, 5)
assert_eq_int(old_value, 0)

sus current_value = atomic_drip.atomic_load_i32(atomic_counter)
assert_eq_int(current_value, 5)

sus cas_success = atomic_drip.atomic_cas_i32(atomic_counter, 5, 10)
assert_true(cas_success)

sus final_value = atomic_drip.atomic_load_i32(atomic_counter)
assert_eq_int(final_value, 10)

print_test_summary()

fr fr Test 2: Atomic flag operations
test_start("Atomic Flag Operations Test")
sus flag = atomic_drip.atomic_flag_new()
sus flag_set = atomic_drip.atomic_flag_test_and_set(flag)
assert_false(flag_set)  fr fr Should be false initially

sus flag_set_again = atomic_drip.atomic_flag_test_and_set(flag)
assert_true(flag_set_again)  fr fr Should be true now

atomic_drip.atomic_flag_clear(flag)
sus flag_after_clear = atomic_drip.atomic_flag_is_set(flag)
assert_false(flag_after_clear)

print_test_summary()

fr fr Test 3: Memory ordering operations
test_start("Memory Ordering Operations Test")
sus atomic64 = atomic_drip.atomic_i64_new(100)

fr fr Test different memory ordering semantics
atomic_drip.atomic_store_i64_ordered(atomic64, 200, atomic_drip.MEMORY_ORDER_RELEASE)
sus loaded_relaxed = atomic_drip.atomic_load_i64_ordered(atomic64, atomic_drip.MEMORY_ORDER_ACQUIRE)
assert_eq_int(loaded_relaxed, 200)

fr fr Test memory fences
atomic_drip.memory_fence()
atomic_drip.acquire_fence()
atomic_drip.release_fence()
atomic_drip.compiler_fence()

print_test_summary()

fr fr Test 4: Concurrenz mutex with atomic operations
test_start("Concurrenz Mutex Integration Test")
sus mutex = concurrenz.create_mutex()
sus mutex_locked = concurrenz.mutex_lock(mutex)
assert_true(mutex_locked)

sus trylock_result = concurrenz.mutex_trylock(mutex)
assert_false(trylock_result)  fr fr Should fail because already locked

sus mutex_unlocked = concurrenz.mutex_unlock(mutex)
assert_true(mutex_unlocked)

sus trylock_after_unlock = concurrenz.mutex_trylock(mutex)
assert_true(trylock_after_unlock)  fr fr Should succeed now

concurrenz.mutex_unlock(mutex)
print_test_summary()

fr fr Test 5: Concurrenz waitgroup with atomic operations
test_start("Concurrenz WaitGroup Integration Test")
sus wg = concurrenz.create_waitgroup()
sus add_result = concurrenz.waitgroup_add(wg, 3)
assert_true(add_result)

sus done_result1 = concurrenz.waitgroup_done(wg)
assert_true(done_result1)

sus done_result2 = concurrenz.waitgroup_done(wg)
assert_true(done_result2)

sus done_result3 = concurrenz.waitgroup_done(wg)
assert_true(done_result3)

sus wait_result = concurrenz.waitgroup_wait(wg)
assert_true(wait_result)

print_test_summary()

fr fr Test 6: Concurrenz channel with atomic operations
test_start("Concurrenz Channel Integration Test")
sus channel = concurrenz.create_channel(5)  fr fr Buffered channel
sus send_result = concurrenz.channel_send(channel, 42)
assert_true(send_result)

sus received_data = concurrenz.channel_receive(channel)
assert_eq_int(received_data, 42)

sus close_result = concurrenz.channel_close(channel)
assert_true(close_result)

sus is_closed = concurrenz.channel_is_closed(channel)
assert_true(is_closed)

print_test_summary()

fr fr Test 7: Spinlock using atomic operations
test_start("Spinlock Integration Test")
sus spinlock = atomic_drip.spinlock_new()

atomic_drip.spinlock_lock(spinlock)
sus trylock_spinlock = atomic_drip.spinlock_try_lock(spinlock)
assert_false(trylock_spinlock)  fr fr Should fail

atomic_drip.spinlock_unlock(spinlock)
sus trylock_after_unlock = atomic_drip.spinlock_try_lock(spinlock)
assert_true(trylock_after_unlock)  fr fr Should succeed

atomic_drip.spinlock_unlock(spinlock)
print_test_summary()

fr fr Test 8: Read-write spinlock
test_start("Read-Write Spinlock Integration Test")
sus rw_spinlock = atomic_drip.rw_spinlock_new()

atomic_drip.rw_spinlock_read_lock(rw_spinlock)
atomic_drip.rw_spinlock_read_lock(rw_spinlock)  fr fr Multiple readers OK

atomic_drip.rw_spinlock_read_unlock(rw_spinlock)
atomic_drip.rw_spinlock_read_unlock(rw_spinlock)

atomic_drip.rw_spinlock_write_lock(rw_spinlock)
sus try_write_while_locked = atomic_drip.rw_spinlock_try_write_lock(rw_spinlock)
assert_false(try_write_while_locked)  fr fr Should fail

atomic_drip.rw_spinlock_write_unlock(rw_spinlock)
sus try_write_after_unlock = atomic_drip.rw_spinlock_try_write_lock(rw_spinlock)
assert_true(try_write_after_unlock)  fr fr Should succeed

atomic_drip.rw_spinlock_write_unlock(rw_spinlock)
print_test_summary()

fr fr Test 9: Compare and swap with weak semantics
test_start("Compare and Swap Weak Semantics Test")
sus atomic_weak = atomic_drip.atomic_i32_new(100)
sus expected = 100
sus weak_cas = atomic_drip.atomic_cas_weak_i32(atomic_weak, &expected, 200)
assert_true(weak_cas)

sus final_weak_value = atomic_drip.atomic_load_i32(atomic_weak)
assert_eq_int(final_weak_value, 200)

print_test_summary()

fr fr Test 10: Atomic pointer operations
test_start("Atomic Pointer Operations Test")
sus ptr_data = 0x1000  fr fr Simulate pointer address
sus atomic_ptr = atomic_drip.atomic_ptr_new(ptr_data)

sus loaded_ptr = atomic_drip.atomic_ptr_load(atomic_ptr)
assert_eq_int(loaded_ptr, ptr_data)

sus new_ptr_data = 0x2000
atomic_drip.atomic_ptr_store(atomic_ptr, new_ptr_data)

sus loaded_new_ptr = atomic_drip.atomic_ptr_load(atomic_ptr)
assert_eq_int(loaded_new_ptr, new_ptr_data)

sus cas_ptr_success = atomic_drip.atomic_ptr_cas(atomic_ptr, new_ptr_data, 0x3000)
assert_true(cas_ptr_success)

print_test_summary()

fr fr Test 11: Concurrenz atomic integration
test_start("Concurrenz Atomic Wrapper Integration Test")
sus atomic_i32 = concurrenz.atomic_i32_new(50)
sus atomic_value = concurrenz.atomic_load_i32(atomic_i32)
assert_eq_int(atomic_value, 50)

concurrenz.atomic_store_i32(atomic_i32, 75)
sus stored_value = concurrenz.atomic_load_i32(atomic_i32)
assert_eq_int(stored_value, 75)

sus increment_result = concurrenz.atomic_increment(atomic_i32)
assert_eq_int(increment_result, 75)  fr fr Returns old value

sus after_increment = concurrenz.atomic_load_i32(atomic_i32)
assert_eq_int(after_increment, 76)

print_test_summary()

fr fr Test 12: Hardware atomic vs software compatibility
test_start("Hardware Atomic Compatibility Test")
sus hw_atomic = atomic_drip.atomic_i32_new(1000)
sus sw_atomic = concurrenz.atomic_i32_new(1000)

fr fr Both should behave identically
sus hw_cas = atomic_drip.atomic_cas_i32(hw_atomic, 1000, 2000)
sus sw_cas = concurrenz.atomic_cas_i32(sw_atomic, 1000, 2000)

assert_true(hw_cas)
assert_true(sw_cas)

sus hw_final = atomic_drip.atomic_load_i32(hw_atomic)
sus sw_final = concurrenz.atomic_load_i32(sw_atomic)

assert_eq_int(hw_final, 2000)
assert_eq_int(sw_final, 2000)

print_test_summary()

fr fr Overall test summary
print_test_summary()
