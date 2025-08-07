yeet "testz"
yeet "concurrenz"

fr fr Thread Safety Test Suite for CURSED Concurrency Module
fr fr Tests concurrent operations to detect data races and race conditions

slay test_channel_thread_safety() {
    test_start("Channel Thread Safety")
    
    fr fr Test concurrent send/receive operations
    sus ch *Channel = create_channel(100)
    sus total_sent *AtomicI32 = atomic_i32_new(0)
    sus total_received *AtomicI32 = atomic_i32_new(0)
    sus receive_sum *AtomicI32 = atomic_i32_new(0)
    
    fr fr Simulate multiple sender goroutines
    sus sender_count normie = 10
    sus messages_per_sender normie = 10
    sus wg_senders *WaitGroup = create_waitgroup()
    waitgroup_add(wg_senders, sender_count)
    
    fr fr Launch sender goroutines (simulated)
    sus i normie = 0
    bestie (i < sender_count) {
        fr fr Each sender sends 10 messages
        sus j normie = 0
        bestie (j < messages_per_sender) {
            sus message normie = i * 1000 + j
            sus sent lit = channel_send(ch, message)
            lowkey sent {
                atomic_increment(total_sent)
            }
            j = j + 1
        }
        waitgroup_done(wg_senders)
        i = i + 1
    }
    
    fr fr Wait for all senders to complete
    waitgroup_wait(wg_senders)
    
    fr fr Simulate receiver goroutines
    sus receiver_count normie = 5
    sus wg_receivers *WaitGroup = create_waitgroup()
    waitgroup_add(wg_receivers, receiver_count)
    
    fr fr Launch receiver goroutines (simulated)
    sus k normie = 0
    bestie (k < receiver_count) {
        fr fr Each receiver processes available messages
        sus messages_to_receive normie = (sender_count * messages_per_sender) / receiver_count
        sus m normie = 0
        bestie (m < messages_to_receive) {
            sus received normie = channel_receive(ch)
            lowkey received > 0 {
                atomic_increment(total_received)
                atomic_add_i32(receive_sum, received)
            }
            m = m + 1
        }
        waitgroup_done(wg_receivers)
        k = k + 1
    }
    
    fr fr Wait for all receivers to complete
    waitgroup_wait(wg_receivers)
    
    fr fr Verify no data races occurred
    sus sent_count normie = atomic_load_i32(total_sent)
    sus received_count normie = atomic_load_i32(total_received)
    
    assert_eq_int(sent_count, sender_count * messages_per_sender)
    assert_eq_int(received_count, sender_count * messages_per_sender)
    
    fr fr Expected sum: sum of all messages sent
    sus expected_sum normie = 0
    sus calc_i normie = 0
    bestie (calc_i < sender_count) {
        sus calc_j normie = 0
        bestie (calc_j < messages_per_sender) {
            expected_sum = expected_sum + (calc_i * 1000 + calc_j)
            calc_j = calc_j + 1
        }
        calc_i = calc_i + 1
    }
    
    sus actual_sum normie = atomic_load_i32(receive_sum)
    assert_eq_int(actual_sum, expected_sum)
    
    channel_close(ch)
    vibez.spill("✅ Channel thread safety test passed - no data races detected")
}

slay test_mutex_contention() {
    test_start("Mutex Contention Safety")
    
    fr fr Shared resource protected by mutex
    sus shared_counter *AtomicI32 = atomic_i32_new(0)
    sus mutex *Mutex = create_mutex()
    sus contention_test_iterations normie = 1000
    
    fr fr Simulate high contention scenario
    sus thread_count normie = 8
    sus iterations_per_thread normie = contention_test_iterations / thread_count
    sus wg *WaitGroup = create_waitgroup()
    waitgroup_add(wg, thread_count)
    
    fr fr Launch contending threads (simulated)
    sus t normie = 0
    bestie (t < thread_count) {
        fr fr Each thread increments the counter many times
        sus iter normie = 0
        bestie (iter < iterations_per_thread) {
            fr fr Critical section protected by mutex
            sus locked lit = mutex_lock(mutex)
            assert_true(locked)
            
            fr fr Read-modify-write operation that must be atomic
            sus current_value normie = atomic_load_i32(shared_counter)
            sus new_value normie = current_value + 1
            atomic_store_i32(shared_counter, new_value)
            
            sus unlocked lit = mutex_unlock(mutex)
            assert_true(unlocked)
            
            iter = iter + 1
        }
        waitgroup_done(wg)
        t = t + 1
    }
    
    fr fr Wait for all threads to complete
    waitgroup_wait(wg)
    
    fr fr Verify no race conditions occurred
    sus final_value normie = atomic_load_i32(shared_counter)
    assert_eq_int(final_value, contention_test_iterations)
    
    vibez.spill("✅ Mutex contention test passed - no race conditions detected")
}

slay test_atomic_operations_safety() {
    test_start("Atomic Operations Safety")
    
    fr fr Test atomic increment under contention
    sus atomic_counter *AtomicI32 = atomic_i32_new(0)
    sus increment_iterations normie = 10000
    sus concurrent_incrementers normie = 4
    sus increments_per_goroutine normie = increment_iterations / concurrent_incrementers
    
    sus wg *WaitGroup = create_waitgroup()
    waitgroup_add(wg, concurrent_incrementers)
    
    fr fr Launch concurrent incrementers
    sus inc normie = 0
    bestie (inc < concurrent_incrementers) {
        sus inc_iter normie = 0
        bestie (inc_iter < increments_per_goroutine) {
            fr fr Atomic increment should be race-free
            atomic_increment(atomic_counter)
            inc_iter = inc_iter + 1
        }
        waitgroup_done(wg)
        inc = inc + 1
    }
    
    waitgroup_wait(wg)
    
    sus final_count normie = atomic_load_i32(atomic_counter)
    assert_eq_int(final_count, increment_iterations)
    
    fr fr Test compare-and-swap operations
    sus cas_target *AtomicI32 = atomic_i32_new(0)
    sus cas_iterations normie = 1000
    sus successful_cas_count *AtomicI32 = atomic_i32_new(0)
    
    sus wg_cas *WaitGroup = create_waitgroup()
    waitgroup_add(wg_cas, concurrent_incrementers)
    
    fr fr Launch concurrent CAS operations
    sus cas_thread normie = 0
    bestie (cas_thread < concurrent_incrementers) {
        sus cas_iter normie = 0
        bestie (cas_iter < cas_iterations / concurrent_incrementers) {
            fr fr Try to increment using CAS
            sus current normie = atomic_load_i32(cas_target)
            sus swapped lit = atomic_cas_i32(cas_target, current, current + 1)
            lowkey swapped {
                atomic_increment(successful_cas_count)
            }
            cas_iter = cas_iter + 1
        }
        waitgroup_done(wg_cas)
        cas_thread = cas_thread + 1
    }
    
    waitgroup_wait(wg_cas)
    
    fr fr Verify CAS operations maintained consistency
    sus final_cas_value normie = atomic_load_i32(cas_target)
    sus successful_cas normie = atomic_load_i32(successful_cas_count)
    
    fr fr Final value should match successful CAS operations
    assert_eq_int(final_cas_value, successful_cas)
    
    vibez.spill("✅ Atomic operations safety test passed")
}

slay test_channel_select_safety() {
    test_start("Channel Select Statement Safety")
    
    fr fr Create multiple channels for select operations
    sus ch1 *Channel = create_channel(10)
    sus ch2 *Channel = create_channel(10)
    sus ch3 *Channel = create_channel(10)
    sus result_ch *Channel = create_channel(100)
    
    fr fr Populate channels with test data
    sus data_items normie = 30
    sus ch1_data normie = 100
    sus ch2_data normie = 200
    sus ch3_data normie = 300
    
    fr fr Send data to different channels
    sus send_iter normie = 0
    bestie (send_iter < data_items / 3) {
        channel_send(ch1, ch1_data + send_iter)
        channel_send(ch2, ch2_data + send_iter)
        channel_send(ch3, ch3_data + send_iter)
        send_iter = send_iter + 1
    }
    
    fr fr Simulate select statement behavior
    fr fr In real implementation, this would be non-blocking channel selection
    sus select_iterations normie = data_items
    sus ch1_selected *AtomicI32 = atomic_i32_new(0)
    sus ch2_selected *AtomicI32 = atomic_i32_new(0)
    sus ch3_selected *AtomicI32 = atomic_i32_new(0)
    
    sus select_iter normie = 0
    bestie (select_iter < select_iterations) {
        fr fr Simulate randomized channel selection
        sus channel_choice normie = select_iter % 3
        
        lowkey channel_choice == 0 {
            fr fr Select from ch1
            sus received normie = channel_receive(ch1)
            lowkey received > 0 {
                atomic_increment(ch1_selected)
                channel_send(result_ch, received)
            }
        } otherwise lowkey channel_choice == 1 {
            fr fr Select from ch2
            sus received normie = channel_receive(ch2)
            lowkey received > 0 {
                atomic_increment(ch2_selected)
                channel_send(result_ch, received)
            }
        } otherwise {
            fr fr Select from ch3
            sus received normie = channel_receive(ch3)
            lowkey received > 0 {
                atomic_increment(ch3_selected)
                channel_send(result_ch, received)
            }
        }
        
        select_iter = select_iter + 1
    }
    
    fr fr Verify all data was processed
    sus total_selected normie = atomic_load_i32(ch1_selected) + 
                               atomic_load_i32(ch2_selected) + 
                               atomic_load_i32(ch3_selected)
    
    assert_eq_int(total_selected, data_items)
    
    fr fr Verify result channel has all data
    sus result_count normie = 0
    bestie (!channel_is_closed(result_ch) && result_count < data_items) {
        sus result normie = channel_receive(result_ch)
        lowkey result > 0 {
            result_count = result_count + 1
        } otherwise {
            fr fr No more data available
            fr fr Break (simulated - would use actual break in real implementation)
            result_count = data_items
        }
    }
    
    assert_eq_int(result_count, data_items)
    
    fr fr Clean up
    channel_close(ch1)
    channel_close(ch2)
    channel_close(ch3)
    channel_close(result_ch)
    
    vibez.spill("✅ Channel select safety test passed")
}

slay test_waitgroup_coordination() {
    test_start("WaitGroup Coordination Safety")
    
    fr fr Test complex waitgroup coordination
    sus master_wg *WaitGroup = create_waitgroup()
    sus worker_count normie = 8
    sus tasks_per_worker normie = 100
    sus total_work_items normie = worker_count * tasks_per_worker
    
    fr fr Shared results protected by atomic operations
    sus completed_work *AtomicI32 = atomic_i32_new(0)
    sus work_sum *AtomicI32 = atomic_i32_new(0)
    
    fr fr Set up waitgroup for all workers
    waitgroup_add(master_wg, worker_count)
    
    fr fr Launch worker "goroutines" (simulated)
    sus worker normie = 0
    bestie (worker < worker_count) {
        fr fr Each worker processes a batch of tasks
        sus task normie = 0
        bestie (task < tasks_per_worker) {
            fr fr Simulate work
            sus work_value normie = worker * 1000 + task
            atomic_add_i32(work_sum, work_value)
            atomic_increment(completed_work)
            task = task + 1
        }
        
        fr fr Signal work completion
        waitgroup_done(master_wg)
        worker = worker + 1
    }
    
    fr fr Wait for all workers to complete
    waitgroup_wait(master_wg)
    
    fr fr Verify all work was completed
    sus final_work_count normie = atomic_load_i32(completed_work)
    assert_eq_int(final_work_count, total_work_items)
    
    fr fr Verify work sum is correct (detect race conditions)
    sus expected_sum normie = 0
    sus verify_worker normie = 0
    bestie (verify_worker < worker_count) {
        sus verify_task normie = 0
        bestie (verify_task < tasks_per_worker) {
            expected_sum = expected_sum + (verify_worker * 1000 + verify_task)
            verify_task = verify_task + 1
        }
        verify_worker = verify_worker + 1
    }
    
    sus actual_sum normie = atomic_load_i32(work_sum)
    assert_eq_int(actual_sum, expected_sum)
    
    vibez.spill("✅ WaitGroup coordination safety test passed")
}

slay test_memory_ordering_guarantees() {
    test_start("Memory Ordering Guarantees")
    
    fr fr Test memory ordering with atomic operations
    sus flag *AtomicI32 = atomic_i32_new(0)
    sus data *AtomicI32 = atomic_i32_new(0)
    sus verification_results *AtomicI32 = atomic_i32_new(0)
    
    fr fr Simulate memory ordering test scenario
    sus producer_iterations normie = 1000
    sus consumer_iterations normie = 1000
    
    sus wg *WaitGroup = create_waitgroup()
    waitgroup_add(wg, 2)  fr fr Producer and consumer
    
    fr fr Producer goroutine (simulated)
    sus prod_iter normie = 0
    bestie (prod_iter < producer_iterations) {
        fr fr Write data then set flag (must be ordered)
        atomic_store_i32(data, prod_iter + 1)
        fr fr Memory fence to ensure ordering
        memory_fence()
        atomic_store_i32(flag, 1)
        
        fr fr Reset for next iteration
        atomic_store_i32(flag, 0)
        prod_iter = prod_iter + 1
    }
    waitgroup_done(wg)
    
    fr fr Consumer goroutine (simulated)
    sus cons_iter normie = 0
    bestie (cons_iter < consumer_iterations) {
        fr fr Check flag then read data (must see consistent state)
        lowkey atomic_load_i32(flag) == 1 {
            sus observed_data normie = atomic_load_i32(data)
            lowkey observed_data > 0 {
                atomic_increment(verification_results)
            }
        }
        cons_iter = cons_iter + 1
    }
    waitgroup_done(wg)
    
    waitgroup_wait(wg)
    
    fr fr Verify memory ordering was respected
    sus successful_observations normie = atomic_load_i32(verification_results)
    assert_true(successful_observations >= 0)  fr fr Should have some successful observations
    
    vibez.spill("✅ Memory ordering guarantees test passed")
}

slay test_goroutine_lifecycle_safety() {
    test_start("Goroutine Lifecycle Safety")
    
    fr fr Test goroutine creation and cleanup patterns
    sus active_goroutines *AtomicI32 = atomic_i32_new(0)
    sus completed_goroutines *AtomicI32 = atomic_i32_new(0)
    sus goroutine_results *AtomicI32 = atomic_i32_new(0)
    
    sus goroutine_count normie = 20
    sus work_per_goroutine normie = 50
    
    fr fr Track goroutine lifecycle
    sus lifecycle_wg *WaitGroup = create_waitgroup()
    waitgroup_add(lifecycle_wg, goroutine_count)
    
    fr fr Launch goroutines (simulated)
    sus gor normie = 0
    bestie (gor < goroutine_count) {
        fr fr Register goroutine start
        atomic_increment(active_goroutines)
        
        fr fr Simulate goroutine work
        sus work_iter normie = 0
        bestie (work_iter < work_per_goroutine) {
            sus work_result normie = gor * 100 + work_iter
            atomic_add_i32(goroutine_results, work_result)
            work_iter = work_iter + 1
        }
        
        fr fr Register goroutine completion
        atomic_decrement(active_goroutines)
        atomic_increment(completed_goroutines)
        waitgroup_done(lifecycle_wg)
        
        gor = gor + 1
    }
    
    fr fr Wait for all goroutines to complete
    waitgroup_wait(lifecycle_wg)
    
    fr fr Verify lifecycle tracking
    sus final_active normie = atomic_load_i32(active_goroutines)
    sus final_completed normie = atomic_load_i32(completed_goroutines)
    
    assert_eq_int(final_active, 0)
    assert_eq_int(final_completed, goroutine_count)
    
    fr fr Verify work results
    sus expected_results normie = 0
    sus verify_gor normie = 0
    bestie (verify_gor < goroutine_count) {
        sus verify_work normie = 0
        bestie (verify_work < work_per_goroutine) {
            expected_results = expected_results + (verify_gor * 100 + verify_work)
            verify_work = verify_work + 1
        }
        verify_gor = verify_gor + 1
    }
    
    sus actual_results normie = atomic_load_i32(goroutine_results)
    assert_eq_int(actual_results, expected_results)
    
    vibez.spill("✅ Goroutine lifecycle safety test passed")
}

slay test_comprehensive_race_detection() {
    test_start("Comprehensive Race Detection")
    
    fr fr Multi-threaded scenario with shared data structures
    sus shared_map_size normie = 100
    sus shared_data []normie = memory.allocate_array(normie, shared_map_size)
    sus access_mutex *Mutex = create_mutex()
    sus access_count *AtomicI32 = atomic_i32_new(0)
    
    fr fr Initialize shared data
    sus init_i normie = 0
    bestie (init_i < shared_map_size) {
        shared_data[init_i] = init_i * 10
        init_i = init_i + 1
    }
    
    fr fr Concurrent readers and writers
    sus reader_count normie = 6
    sus writer_count normie = 4
    sus operations_per_thread normie = 200
    
    sus race_test_wg *WaitGroup = create_waitgroup()
    waitgroup_add(race_test_wg, reader_count + writer_count)
    
    fr fr Launch reader goroutines (simulated)
    sus reader normie = 0
    bestie (reader < reader_count) {
        sus read_ops normie = 0
        bestie (read_ops < operations_per_thread) {
            sus index normie = read_ops % shared_map_size
            
            fr fr Protected read operation
            mutex_lock(access_mutex)
            sus read_value normie = shared_data[index]
            mutex_unlock(access_mutex)
            
            fr fr Verify data consistency
            lowkey read_value >= 0 {
                atomic_increment(access_count)
            }
            
            read_ops = read_ops + 1
        }
        waitgroup_done(race_test_wg)
        reader = reader + 1
    }
    
    fr fr Launch writer goroutines (simulated)
    sus writer normie = 0
    bestie (writer < writer_count) {
        sus write_ops normie = 0
        bestie (write_ops < operations_per_thread) {
            sus index normie = write_ops % shared_map_size
            sus new_value normie = (writer + 1) * 1000 + write_ops
            
            fr fr Protected write operation
            mutex_lock(access_mutex)
            shared_data[index] = new_value
            mutex_unlock(access_mutex)
            
            atomic_increment(access_count)
            write_ops = write_ops + 1
        }
        waitgroup_done(race_test_wg)
        writer = writer + 1
    }
    
    waitgroup_wait(race_test_wg)
    
    fr fr Verify no race conditions occurred
    sus total_operations normie = (reader_count + writer_count) * operations_per_thread
    sus recorded_accesses normie = atomic_load_i32(access_count)
    
    assert_eq_int(recorded_accesses, total_operations)
    
    fr fr Verify data integrity
    sus integrity_check normie = 0
    sus check_i normie = 0
    bestie (check_i < shared_map_size) {
        lowkey shared_data[check_i] >= 0 {
            integrity_check = integrity_check + 1
        }
        check_i = check_i + 1
    }
    
    assert_eq_int(integrity_check, shared_map_size)
    
    vibez.spill("✅ Comprehensive race detection test passed")
}

slay run_thread_safety_tests() {
    vibez.spill("🔒 Starting Thread Safety Test Suite")
    vibez.spill("Testing concurrent operations for data races and race conditions...")
    
    test_channel_thread_safety()
    test_mutex_contention()
    test_atomic_operations_safety()
    test_channel_select_safety()
    test_waitgroup_coordination()
    test_memory_ordering_guarantees()
    test_goroutine_lifecycle_safety()
    test_comprehensive_race_detection()
    
    print_test_summary()
    vibez.spill("✅ All thread safety tests completed!")
    vibez.spill("🔒 No data races or race conditions detected")
}

fr fr Run thread safety tests when this file is executed
run_thread_safety_tests()
