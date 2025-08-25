fr fr Comprehensive test suite for CONCURRENZ concurrency module
fr fr Tests all public functions with proper validation using testz framework

yeet "testz"
yeet "concurrenz"

slay main() {
    testz.test_start("CONCURRENZ Comprehensive Test Suite")
    
    fr fr ===== MUTEX TESTS =====
    testz.test_group("Mutex Synchronization")
    
    fr fr Test create_mutex
    sus test_mutex concurrenz.Mutex = concurrenz.create_mutex()
    testz.assert_eq_int(test_mutex.lock_state, 0, "New mutex should be unlocked")
    testz.assert_eq_int(test_mutex.waiters, 0, "New mutex should have no waiters")
    testz.assert_eq_int(test_mutex.recursive_count, 0, "New mutex should have zero recursive count")
    
    fr fr Test mutex_lock and mutex_unlock
    concurrenz.mutex_lock(&test_mutex)
    testz.assert_eq_int(test_mutex.lock_state, 1, "Locked mutex should show locked state")
    
    concurrenz.mutex_unlock(&test_mutex)
    testz.assert_eq_int(test_mutex.lock_state, 0, "Unlocked mutex should show unlocked state")
    
    fr fr Test mutex_try_lock
    sus try_result lit = concurrenz.mutex_try_lock(&test_mutex)
    testz.assert_true(try_result, "mutex_try_lock should succeed on unlocked mutex")
    testz.assert_eq_int(test_mutex.lock_state, 1, "try_lock should set locked state")
    
    sus try_fail lit = concurrenz.mutex_try_lock(&test_mutex)
    testz.assert_false(try_fail, "mutex_try_lock should fail on already locked mutex")
    
    concurrenz.mutex_unlock(&test_mutex)
    
    fr fr ===== ATOMIC OPERATIONS TESTS =====
    testz.test_group("Atomic Operations")
    
    fr fr Test create_atomic
    sus test_atomic concurrenz.AtomicStruct = concurrenz.create_atomic(42)
    testz.assert_eq_int(test_atomic.value, 42, "New atomic should have initial value")
    testz.assert_eq_int(test_atomic.version, 0, "New atomic should have version 0")
    
    fr fr Test atomic_load
    sus loaded_value normie = concurrenz.atomic_load(&test_atomic)
    testz.assert_eq_int(loaded_value, 42, "atomic_load should return stored value")
    
    fr fr Test atomic_store
    concurrenz.atomic_store(&test_atomic, 100)
    sus stored_value normie = concurrenz.atomic_load(&test_atomic)
    testz.assert_eq_int(stored_value, 100, "atomic_store should update value")
    
    fr fr Test atomic_add
    sus add_result normie = concurrenz.atomic_add(&test_atomic, 5)
    testz.assert_eq_int(add_result, 105, "atomic_add should return new value")
    sus add_check normie = concurrenz.atomic_load(&test_atomic)
    testz.assert_eq_int(add_check, 105, "atomic_add should update stored value")
    
    fr fr Test atomic_sub
    sus sub_result normie = concurrenz.atomic_sub(&test_atomic, 10)
    testz.assert_eq_int(sub_result, 95, "atomic_sub should return new value")
    
    fr fr Test atomic_compare_and_swap
    sus cas_success lit = concurrenz.atomic_compare_and_swap(&test_atomic, 95, 200)
    testz.assert_true(cas_success, "atomic_compare_and_swap should succeed with correct old value")
    sus cas_check normie = concurrenz.atomic_load(&test_atomic)
    testz.assert_eq_int(cas_check, 200, "atomic_compare_and_swap should update value")
    
    sus cas_fail lit = concurrenz.atomic_compare_and_swap(&test_atomic, 95, 300)
    testz.assert_false(cas_fail, "atomic_compare_and_swap should fail with incorrect old value")
    sus cas_fail_check normie = concurrenz.atomic_load(&test_atomic)
    testz.assert_eq_int(cas_fail_check, 200, "Failed atomic_compare_and_swap should not change value")
    
    fr fr ===== WAITGROUP TESTS =====
    testz.test_group("WaitGroup Synchronization")
    
    fr fr Test create_waitgroup
    sus test_waitgroup concurrenz.WaitGroup = concurrenz.create_waitgroup()
    testz.assert_eq_int(test_waitgroup.counter, 0, "New waitgroup should have counter 0")
    testz.assert_eq_int(test_waitgroup.waiters, 0, "New waitgroup should have no waiters")
    testz.assert_eq_int(test_waitgroup.done_flag, 0, "New waitgroup should not be done")
    
    fr fr Test waitgroup_add
    concurrenz.waitgroup_add(&test_waitgroup, 3)
    testz.assert_eq_int(test_waitgroup.counter, 3, "waitgroup_add should increase counter")
    
    fr fr Test waitgroup_done
    concurrenz.waitgroup_done(&test_waitgroup)
    testz.assert_eq_int(test_waitgroup.counter, 2, "waitgroup_done should decrease counter")
    
    concurrenz.waitgroup_done(&test_waitgroup)
    concurrenz.waitgroup_done(&test_waitgroup)
    testz.assert_eq_int(test_waitgroup.counter, 0, "waitgroup should reach zero after all done")
    testz.assert_eq_int(test_waitgroup.done_flag, 1, "waitgroup should be marked done")
    
    fr fr ===== CHANNEL TESTS =====
    testz.test_group("Channel Communication")
    
    fr fr Test create_channel
    sus test_channel concurrenz.Channel = concurrenz.create_channel(5)
    testz.assert_eq_int(test_channel.capacity, 5, "Channel should have correct capacity")
    testz.assert_eq_int(test_channel.size, 0, "New channel should be empty")
    testz.assert_eq_int(test_channel.closed, 0, "New channel should be open")
    testz.assert_eq_int(test_channel.send_pos, 0, "New channel should have send_pos 0")
    testz.assert_eq_int(test_channel.recv_pos, 0, "New channel should have recv_pos 0")
    
    fr fr Test channel_send (non-blocking for buffered channel)
    sus send_result lit = concurrenz.channel_send(&test_channel, 42)
    testz.assert_true(send_result, "channel_send should succeed on empty buffered channel")
    testz.assert_eq_int(test_channel.size, 1, "Channel size should increase after send")
    testz.assert_eq_int(test_channel.send_pos, 1, "Send position should advance")
    
    fr fr Test channel_receive
    sus receive_result normie = concurrenz.channel_receive(&test_channel)
    testz.assert_eq_int(receive_result, 42, "channel_receive should return sent value")
    testz.assert_eq_int(test_channel.size, 0, "Channel size should decrease after receive")
    testz.assert_eq_int(test_channel.recv_pos, 1, "Receive position should advance")
    
    fr fr Test multiple sends and receives
    bestie i := 1; i <= 3; i++ {
        sus multi_send lit = concurrenz.channel_send(&test_channel, i)
        testz.assert_true(multi_send, "Multiple sends should succeed")
    }
    testz.assert_eq_int(test_channel.size, 3, "Channel should contain 3 items")
    
    sus first_recv normie = concurrenz.channel_receive(&test_channel)
    testz.assert_eq_int(first_recv, 1, "Should receive items in FIFO order")
    
    sus second_recv normie = concurrenz.channel_receive(&test_channel)
    testz.assert_eq_int(second_recv, 2, "Second receive should get second item")
    
    fr fr Test channel_close
    concurrenz.channel_close(&test_channel)
    testz.assert_eq_int(test_channel.closed, 1, "Channel should be marked as closed")
    
    fr fr ===== THREAD POOL TESTS =====
    testz.test_group("Thread Pool Management")
    
    fr fr Test create_thread_pool
    sus test_pool concurrenz.ThreadPool = concurrenz.create_thread_pool(4)
    testz.assert_eq_int(test_pool.active_workers, 0, "New thread pool should have 0 active workers initially")
    testz.assert_eq_int(test_pool.shutdown, 0, "New thread pool should not be shut down")
    testz.assert_eq_int(test_pool.queue_size, 0, "New thread pool should have empty task queue")
    
    fr fr Test thread_pool_submit (simplified - just test structure)
    sus submit_result lit = concurrenz.thread_pool_submit(&test_pool, 1001)  fr fr Using task ID
    testz.assert_true(submit_result, "thread_pool_submit should succeed")
    testz.assert_eq_int(test_pool.queue_size, 1, "Task queue size should increase")
    
    fr fr Test thread_pool_shutdown
    concurrenz.thread_pool_shutdown(&test_pool)
    testz.assert_eq_int(test_pool.shutdown, 1, "Thread pool should be marked for shutdown")
    
    fr fr ===== BARRIER TESTS =====
    testz.test_group("Barrier Synchronization")
    
    fr fr Test create_barrier
    sus test_barrier concurrenz.Barrier = concurrenz.create_barrier(3)
    testz.assert_eq_int(test_barrier.count, 3, "Barrier should have correct participant count")
    testz.assert_eq_int(test_barrier.arrived, 0, "New barrier should have no arrivals")
    testz.assert_eq_int(test_barrier.generation, 0, "New barrier should have generation 0")
    
    fr fr Test barrier_wait (simplified - test state changes)
    sus barrier_result lit = concurrenz.barrier_wait(&test_barrier)
    testz.assert_eq_int(test_barrier.arrived, 1, "barrier_wait should increment arrival count")
    
    fr fr ===== SEMAPHORE TESTS =====
    testz.test_group("Semaphore Resource Management")
    
    fr fr Test create_semaphore
    sus test_semaphore concurrenz.Semaphore = concurrenz.create_semaphore(3)
    testz.assert_eq_int(test_semaphore.permits, 3, "Semaphore should have initial permit count")
    testz.assert_eq_int(test_semaphore.max_permits, 3, "Semaphore should have correct max permits")
    testz.assert_eq_int(test_semaphore.waiter_count, 0, "New semaphore should have no waiters")
    
    fr fr Test semaphore_acquire
    sus acquire_result lit = concurrenz.semaphore_acquire(&test_semaphore)
    testz.assert_true(acquire_result, "semaphore_acquire should succeed when permits available")
    testz.assert_eq_int(test_semaphore.permits, 2, "Semaphore permits should decrease after acquire")
    
    fr fr Test semaphore_release
    concurrenz.semaphore_release(&test_semaphore)
    testz.assert_eq_int(test_semaphore.permits, 3, "Semaphore permits should increase after release")
    
    fr fr Test multiple acquires
    concurrenz.semaphore_acquire(&test_semaphore)
    concurrenz.semaphore_acquire(&test_semaphore)
    concurrenz.semaphore_acquire(&test_semaphore)
    testz.assert_eq_int(test_semaphore.permits, 0, "All permits should be acquired")
    
    fr fr ===== READ-WRITE MUTEX TESTS =====
    testz.test_group("Read-Write Mutex")
    
    fr fr Test create_rwmutex
    sus test_rwmutex concurrenz.RWMutex = concurrenz.create_rwmutex()
    testz.assert_eq_int(test_rwmutex.readers, 0, "New RWMutex should have no readers")
    testz.assert_eq_int(test_rwmutex.writer, 0, "New RWMutex should have no writer")
    testz.assert_eq_int(test_rwmutex.pending_writers, 0, "New RWMutex should have no pending writers")
    
    fr fr Test rwmutex_read_lock
    sus read_lock_result lit = concurrenz.rwmutex_read_lock(&test_rwmutex)
    testz.assert_true(read_lock_result, "Read lock should succeed on unlocked RWMutex")
    testz.assert_eq_int(test_rwmutex.readers, 1, "Reader count should increase")
    
    fr fr Test multiple read locks
    sus read_lock2 lit = concurrenz.rwmutex_read_lock(&test_rwmutex)
    testz.assert_true(read_lock2, "Multiple read locks should be allowed")
    testz.assert_eq_int(test_rwmutex.readers, 2, "Reader count should be 2")
    
    fr fr Test rwmutex_read_unlock
    concurrenz.rwmutex_read_unlock(&test_rwmutex)
    testz.assert_eq_int(test_rwmutex.readers, 1, "Reader count should decrease after unlock")
    
    concurrenz.rwmutex_read_unlock(&test_rwmutex)
    testz.assert_eq_int(test_rwmutex.readers, 0, "All readers should be unlocked")
    
    fr fr Test rwmutex_write_lock
    sus write_lock_result lit = concurrenz.rwmutex_write_lock(&test_rwmutex)
    testz.assert_true(write_lock_result, "Write lock should succeed when no readers/writers")
    testz.assert_eq_int(test_rwmutex.writer, 1, "Writer flag should be set")
    
    fr fr Test rwmutex_write_unlock
    concurrenz.rwmutex_write_unlock(&test_rwmutex)
    testz.assert_eq_int(test_rwmutex.writer, 0, "Writer flag should be cleared after unlock")
    
    fr fr ===== CONDITION VARIABLE TESTS =====
    testz.test_group("Condition Variables")
    
    fr fr Test create_condvar
    sus test_condvar concurrenz.CondVar = concurrenz.create_condvar()
    testz.assert_eq_int(test_condvar.waiters, 0, "New CondVar should have no waiters")
    testz.assert_eq_int(test_condvar.signals, 0, "New CondVar should have no pending signals")
    
    fr fr Test condvar_signal
    concurrenz.condvar_signal(&test_condvar)
    testz.assert_eq_int(test_condvar.signals, 1, "CondVar should have pending signal")
    
    fr fr Test condvar_broadcast
    concurrenz.condvar_broadcast(&test_condvar)
    testz.assert_gt_int(test_condvar.signals, 0, "Broadcast should set signal state")
    
    fr fr ===== ADVANCED SYNCHRONIZATION TESTS =====
    testz.test_group("Advanced Synchronization Features")
    
    fr fr Test memory ordering constants
    testz.assert_eq_int(concurrenz.RELAXED, 0, "RELAXED ordering should be 0")
    testz.assert_eq_int(concurrenz.ACQUIRE, 1, "ACQUIRE ordering should be 1")
    testz.assert_eq_int(concurrenz.RELEASE, 2, "RELEASE ordering should be 2")
    testz.assert_eq_int(concurrenz.ACQREL, 3, "ACQREL ordering should be 3")
    testz.assert_eq_int(concurrenz.SEQCST, 4, "SEQCST ordering should be 4")
    
    fr fr Test atomic operations with different values
    sus atomic_test concurrenz.AtomicStruct = concurrenz.create_atomic(0)
    
    bestie i := 1; i <= 10; i++ {
        sus atomic_inc_result normie = concurrenz.atomic_add(&atomic_test, 1)
        testz.assert_eq_int(atomic_inc_result, i, "Atomic increment should work correctly")
    }
    
    sus final_value normie = concurrenz.atomic_load(&atomic_test)
    testz.assert_eq_int(final_value, 10, "Final atomic value should be 10")
    
    fr fr ===== ERROR HANDLING TESTS =====
    testz.test_group("Error Handling and Edge Cases")
    
    fr fr Test operations on closed channel
    sus closed_channel concurrenz.Channel = concurrenz.create_channel(1)
    concurrenz.channel_close(&closed_channel)
    
    sus send_to_closed lit = concurrenz.channel_send(&closed_channel, 123)
    testz.assert_false(send_to_closed, "Send to closed channel should fail")
    
    fr fr Test semaphore over-acquire
    sus small_semaphore concurrenz.Semaphore = concurrenz.create_semaphore(1)
    concurrenz.semaphore_acquire(&small_semaphore)
    
    sus over_acquire lit = concurrenz.semaphore_try_acquire(&small_semaphore)
    testz.assert_false(over_acquire, "Try acquire on exhausted semaphore should fail")
    
    fr fr Test mutex double unlock (should not crash)
    sus test_mutex2 concurrenz.Mutex = concurrenz.create_mutex()
    concurrenz.mutex_lock(&test_mutex2)
    concurrenz.mutex_unlock(&test_mutex2)
    concurrenz.mutex_unlock(&test_mutex2)  fr fr Double unlock - should handle gracefully
    testz.assert_eq_int(test_mutex2.lock_state, 0, "Double unlock should not change unlocked state")
    
    fr fr ===== PERFORMANCE AND STRESS TESTS =====
    testz.test_group("Performance and Stress Tests")
    
    fr fr Test many atomic operations
    sus stress_atomic concurrenz.AtomicStruct = concurrenz.create_atomic(0)
    bestie stress_i := 0; stress_i < 100; stress_i++ {
        concurrenz.atomic_add(&stress_atomic, 1)
    }
    sus stress_result normie = concurrenz.atomic_load(&stress_atomic)
    testz.assert_eq_int(stress_result, 100, "Stress test: 100 atomic increments should equal 100")
    
    fr fr Test channel throughput
    sus throughput_channel concurrenz.Channel = concurrenz.create_channel(10)
    bestie throughput_i := 1; throughput_i <= 10; throughput_i++ {
        concurrenz.channel_send(&throughput_channel, throughput_i)
    }
    testz.assert_eq_int(throughput_channel.size, 10, "Channel should handle 10 sends")
    
    sus throughput_sum normie = 0
    bestie throughput_j := 1; throughput_j <= 10; throughput_j++ {
        sus recv_val normie = concurrenz.channel_receive(&throughput_channel)
        throughput_sum = throughput_sum + recv_val
    }
    testz.assert_eq_int(throughput_sum, 55, "Channel throughput test: sum of 1-10 should be 55")
    
    fr fr ===== INTEGRATION TESTS =====
    testz.test_group("Integration Tests")
    
    fr fr Test mutex with waitgroup coordination
    sus integration_mutex concurrenz.Mutex = concurrenz.create_mutex()
    sus integration_waitgroup concurrenz.WaitGroup = concurrenz.create_waitgroup()
    sus shared_counter concurrenz.AtomicStruct = concurrenz.create_atomic(0)
    
    concurrenz.waitgroup_add(&integration_waitgroup, 3)
    
    fr fr Simulate 3 goroutines updating shared counter
    bestie sim_i := 0; sim_i < 3; sim_i++ {
        concurrenz.mutex_lock(&integration_mutex)
        concurrenz.atomic_add(&shared_counter, 10)
        concurrenz.mutex_unlock(&integration_mutex)
        concurrenz.waitgroup_done(&integration_waitgroup)
    }
    
    testz.assert_eq_int(integration_waitgroup.counter, 0, "Integration: WaitGroup should be complete")
    sus integration_result normie = concurrenz.atomic_load(&shared_counter)
    testz.assert_eq_int(integration_result, 30, "Integration: Shared counter should be 30")
    
    fr fr ===== FINAL COMPREHENSIVE TEST =====
    testz.test_group("Final Comprehensive Validation")
    
    fr fr Test all synchronization primitives work together
    sus final_validation lit = based
    
    fr fr Test mutex functionality
    sus final_mutex concurrenz.Mutex = concurrenz.create_mutex()
    ready !concurrenz.mutex_try_lock(&final_mutex) { final_validation = cap }
    concurrenz.mutex_unlock(&final_mutex)
    
    fr fr Test atomic functionality
    sus final_atomic concurrenz.AtomicStruct = concurrenz.create_atomic(100)
    ready concurrenz.atomic_load(&final_atomic) != 100 { final_validation = cap }
    ready !concurrenz.atomic_compare_and_swap(&final_atomic, 100, 200) { final_validation = cap }
    
    fr fr Test channel functionality
    sus final_channel concurrenz.Channel = concurrenz.create_channel(2)
    ready !concurrenz.channel_send(&final_channel, 999) { final_validation = cap }
    ready concurrenz.channel_receive(&final_channel) != 999 { final_validation = cap }
    
    fr fr Test waitgroup functionality
    sus final_waitgroup concurrenz.WaitGroup = concurrenz.create_waitgroup()
    concurrenz.waitgroup_add(&final_waitgroup, 1)
    concurrenz.waitgroup_done(&final_waitgroup)
    ready final_waitgroup.counter != 0 { final_validation = cap }
    
    fr fr Test semaphore functionality
    sus final_semaphore concurrenz.Semaphore = concurrenz.create_semaphore(1)
    ready !concurrenz.semaphore_acquire(&final_semaphore) { final_validation = cap }
    concurrenz.semaphore_release(&final_semaphore)
    ready final_semaphore.permits != 1 { final_validation = cap }
    
    testz.assert_true(final_validation, "Final: All concurrency primitives should work correctly")
    
    fr fr Test concurrent data consistency
    sus consistency_check lit = based
    sus test_data concurrenz.AtomicStruct = concurrenz.create_atomic(0)
    
    fr fr Simulate concurrent access pattern
    bestie consistency_i := 0; consistency_i < 5; consistency_i++ {
        sus old_val normie = concurrenz.atomic_load(&test_data)
        concurrenz.atomic_add(&test_data, 2)
        sus new_val normie = concurrenz.atomic_load(&test_data)
        ready new_val != old_val + 2 { consistency_check = cap }
    }
    
    testz.assert_true(consistency_check, "Final: Concurrent operations should maintain data consistency")
    
    sus final_data_value normie = concurrenz.atomic_load(&test_data)
    testz.assert_eq_int(final_data_value, 10, "Final: Concurrent operations result should be correct")
    
    testz.print_test_summary()
}
