yeet "testz"
yeet "concurrency"

fr fr Concurrency Module Comprehensive Test Suite
fr fr Testing all concurrency primitives and patterns

test_start("Thread Management Tests")

fr fr Test thread creation and management
assert_eq_int(concurrency_thread_create("test_function"), 1)
assert_eq_int(concurrency_thread_create(""), -1)
assert_true(concurrency_thread_start(1))
assert_false(concurrency_thread_start(-1))
assert_true(concurrency_thread_join(1))
assert_false(concurrency_thread_join(-1))
assert_true(concurrency_thread_detach(1))
assert_false(concurrency_thread_detach(-1))

fr fr Test thread state and priority
assert_eq_int(concurrency_thread_get_state(1), THREAD_STATE_READY)
assert_eq_int(concurrency_thread_get_state(-1), -1)
assert_true(concurrency_thread_set_priority(1, 5))
assert_false(concurrency_thread_set_priority(-1, 5))
assert_false(concurrency_thread_set_priority(1, -1))
assert_false(concurrency_thread_set_priority(1, 11))
assert_eq_int(concurrency_thread_get_priority(1), 5)
assert_eq_int(concurrency_thread_get_priority(-1), -1)

fr fr Test thread utilities
assert_true(concurrency_thread_yield())
assert_true(concurrency_thread_sleep(100))
assert_false(concurrency_thread_sleep(-1))
assert_eq_int(concurrency_thread_get_id(), 1)

print_test_summary()

test_start("Mutex Operations Tests")

fr fr Test mutex creation and operations
assert_eq_int(concurrency_mutex_create(), 1)
assert_true(concurrency_mutex_lock(1))
assert_false(concurrency_mutex_lock(-1))
assert_true(concurrency_mutex_unlock(1))
assert_false(concurrency_mutex_unlock(-1))
assert_true(concurrency_mutex_try_lock(1))
assert_false(concurrency_mutex_try_lock(-1))
assert_false(concurrency_mutex_is_locked(1))
assert_true(concurrency_mutex_is_locked(-1))
assert_true(concurrency_mutex_destroy(1))
assert_false(concurrency_mutex_destroy(-1))

print_test_summary()

test_start("Semaphore Operations Tests")

fr fr Test semaphore creation and operations
assert_eq_int(concurrency_semaphore_create(5), 1)
assert_eq_int(concurrency_semaphore_create(-1), -1)
assert_true(concurrency_semaphore_wait(1))
assert_false(concurrency_semaphore_wait(-1))
assert_true(concurrency_semaphore_signal(1))
assert_false(concurrency_semaphore_signal(-1))
assert_true(concurrency_semaphore_try_wait(1))
assert_false(concurrency_semaphore_try_wait(-1))
assert_eq_int(concurrency_semaphore_get_count(1), 1)
assert_eq_int(concurrency_semaphore_get_count(-1), -1)
assert_true(concurrency_semaphore_destroy(1))
assert_false(concurrency_semaphore_destroy(-1))

print_test_summary()

test_start("Condition Variable Tests")

fr fr Test condition variable operations
assert_eq_int(concurrency_condition_create(), 1)
assert_true(concurrency_condition_wait(1, 1))
assert_false(concurrency_condition_wait(-1, 1))
assert_false(concurrency_condition_wait(1, -1))
assert_true(concurrency_condition_signal(1))
assert_false(concurrency_condition_signal(-1))
assert_true(concurrency_condition_broadcast(1))
assert_false(concurrency_condition_broadcast(-1))
assert_true(concurrency_condition_timed_wait(1, 1, 1000))
assert_false(concurrency_condition_timed_wait(-1, 1, 1000))
assert_false(concurrency_condition_timed_wait(1, -1, 1000))
assert_false(concurrency_condition_timed_wait(1, 1, -1))
assert_true(concurrency_condition_destroy(1))
assert_false(concurrency_condition_destroy(-1))

print_test_summary()

test_start("Read-Write Lock Tests")

fr fr Test read-write lock operations
assert_eq_int(concurrency_rwlock_create(), 1)
assert_true(concurrency_rwlock_read_lock(1))
assert_false(concurrency_rwlock_read_lock(-1))
assert_true(concurrency_rwlock_write_lock(1))
assert_false(concurrency_rwlock_write_lock(-1))
assert_true(concurrency_rwlock_read_unlock(1))
assert_false(concurrency_rwlock_read_unlock(-1))
assert_true(concurrency_rwlock_write_unlock(1))
assert_false(concurrency_rwlock_write_unlock(-1))
assert_true(concurrency_rwlock_try_read_lock(1))
assert_false(concurrency_rwlock_try_read_lock(-1))
assert_true(concurrency_rwlock_try_write_lock(1))
assert_false(concurrency_rwlock_try_write_lock(-1))
assert_true(concurrency_rwlock_destroy(1))
assert_false(concurrency_rwlock_destroy(-1))

print_test_summary()

test_start("Barrier Operations Tests")

fr fr Test barrier operations
assert_eq_int(concurrency_barrier_create(3), 1)
assert_eq_int(concurrency_barrier_create(0), -1)
assert_eq_int(concurrency_barrier_create(-1), -1)
assert_true(concurrency_barrier_wait(1))
assert_false(concurrency_barrier_wait(-1))
assert_true(concurrency_barrier_destroy(1))
assert_false(concurrency_barrier_destroy(-1))

print_test_summary()

test_start("Atomic Operations Tests")

fr fr Test atomic operations
assert_eq_int(concurrency_atomic_create(42), 1)
assert_eq_int(concurrency_atomic_load(1), 42)
assert_eq_int(concurrency_atomic_load(-1), -1)
assert_true(concurrency_atomic_store(1, 100))
assert_false(concurrency_atomic_store(-1, 100))
assert_eq_int(concurrency_atomic_exchange(1, 200), 42)
assert_eq_int(concurrency_atomic_exchange(-1, 200), -1)
assert_true(concurrency_atomic_compare_and_swap(1, 42, 300))
assert_false(concurrency_atomic_compare_and_swap(-1, 42, 300))
assert_eq_int(concurrency_atomic_fetch_add(1, 10), 42)
assert_eq_int(concurrency_atomic_fetch_add(-1, 10), -1)
assert_eq_int(concurrency_atomic_fetch_sub(1, 5), 42)
assert_eq_int(concurrency_atomic_fetch_sub(-1, 5), -1)
assert_true(concurrency_atomic_destroy(1))
assert_false(concurrency_atomic_destroy(-1))

print_test_summary()

test_start("Worker Pool Tests")

fr fr Test worker pool operations
assert_eq_int(concurrency_pool_create(4), 1)
assert_eq_int(concurrency_pool_create(0), -1)
assert_eq_int(concurrency_pool_create(-1), -1)
assert_eq_int(concurrency_pool_submit_task(1, "task_function"), 1)
assert_eq_int(concurrency_pool_submit_task(-1, "task_function"), -1)
assert_eq_int(concurrency_pool_submit_task(1, ""), -1)
assert_true(concurrency_pool_wait_for_task(1, 1))
assert_false(concurrency_pool_wait_for_task(-1, 1))
assert_false(concurrency_pool_wait_for_task(1, -1))
assert_eq_int(concurrency_pool_get_active_tasks(1), 5)
assert_eq_int(concurrency_pool_get_active_tasks(-1), -1)
assert_eq_int(concurrency_pool_get_completed_tasks(1), 10)
assert_eq_int(concurrency_pool_get_completed_tasks(-1), -1)
assert_eq_int(concurrency_pool_get_state(1), POOL_ACTIVE)
assert_eq_int(concurrency_pool_get_state(-1), -1)
assert_true(concurrency_pool_shutdown(1))
assert_false(concurrency_pool_shutdown(-1))

print_test_summary()

test_start("Channel Operations Tests")

fr fr Test channel operations
assert_eq_int(concurrency_channel_create(10), 1)
assert_eq_int(concurrency_channel_create(-1), -1)
assert_true(concurrency_channel_send(1, "test_message"))
assert_false(concurrency_channel_send(-1, "test_message"))
assert_eq_string(concurrency_channel_receive(1), "channel_message")
assert_eq_string(concurrency_channel_receive(-1), "")
assert_true(concurrency_channel_try_send(1, "test_message"))
assert_false(concurrency_channel_try_send(-1, "test_message"))
assert_eq_string(concurrency_channel_try_receive(1), "channel_message")
assert_eq_string(concurrency_channel_try_receive(-1), "")
assert_false(concurrency_channel_is_closed(1))
assert_true(concurrency_channel_is_closed(-1))
assert_true(concurrency_channel_close(1))
assert_false(concurrency_channel_close(-1))
assert_eq_int(concurrency_channel_get_buffer_size(1), 10)
assert_eq_int(concurrency_channel_get_buffer_size(-1), -1)
assert_eq_int(concurrency_channel_get_message_count(1), 3)
assert_eq_int(concurrency_channel_get_message_count(-1), -1)

print_test_summary()

test_start("Future/Promise Tests")

fr fr Test future/promise operations
assert_eq_int(concurrency_future_create(), 1)
assert_true(concurrency_future_set_value(1, "test_value"))
assert_false(concurrency_future_set_value(-1, "test_value"))
assert_eq_string(concurrency_future_get_value(1), "future_value")
assert_eq_string(concurrency_future_get_value(-1), "")
assert_true(concurrency_future_is_ready(1))
assert_false(concurrency_future_is_ready(-1))
assert_true(concurrency_future_wait(1))
assert_false(concurrency_future_wait(-1))
assert_true(concurrency_future_wait_timeout(1, 1000))
assert_false(concurrency_future_wait_timeout(-1, 1000))
assert_false(concurrency_future_wait_timeout(1, -1))
assert_true(concurrency_future_destroy(1))
assert_false(concurrency_future_destroy(-1))

print_test_summary()

test_start("Thread Utilities Tests")

fr fr Test thread utility functions
assert_eq_int(concurrency_get_cpu_count(), 4)
assert_eq_int(concurrency_get_thread_count(), 8)
assert_true(concurrency_is_main_thread())
assert_eq_int(concurrency_get_current_thread_id(), 1)
assert_true(concurrency_set_thread_name(1, "test_thread"))
assert_false(concurrency_set_thread_name(-1, "test_thread"))
assert_false(concurrency_set_thread_name(1, ""))
assert_eq_string(concurrency_get_thread_name(1), "thread_name")
assert_eq_string(concurrency_get_thread_name(-1), "")

print_test_summary()

test_start("Performance Monitoring Tests")

fr fr Test performance monitoring
assert_eq_int(concurrency_get_context_switches(), 100)
assert_eq_int(concurrency_get_lock_contention(), 5)
assert_eq_int(concurrency_get_deadlock_count(), 0)
assert_true(concurrency_reset_performance_counters())

print_test_summary()

test_start("Concurrency Constants Tests")

fr fr Test thread state constants
assert_eq_int(THREAD_STATE_READY, 0)
assert_eq_int(THREAD_STATE_RUNNING, 1)
assert_eq_int(THREAD_STATE_BLOCKED, 2)
assert_eq_int(THREAD_STATE_TERMINATED, 3)

fr fr Test mutex constants
assert_eq_int(MUTEX_UNLOCKED, 0)
assert_eq_int(MUTEX_LOCKED, 1)

fr fr Test semaphore constants
assert_eq_int(SEMAPHORE_AVAILABLE, 0)
assert_eq_int(SEMAPHORE_BLOCKED, 1)

fr fr Test channel constants
assert_eq_int(CHANNEL_BUFFERED, 1)
assert_eq_int(CHANNEL_UNBUFFERED, 2)
assert_eq_int(CHANNEL_CLOSED, 3)

fr fr Test worker pool constants
assert_eq_int(POOL_ACTIVE, 1)
assert_eq_int(POOL_SHUTTING_DOWN, 2)
assert_eq_int(POOL_SHUTDOWN, 3)

print_test_summary()
