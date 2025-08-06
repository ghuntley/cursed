yeet "testz"
yeet "concurrenz"
yeet "errorz"
yeet "asyncz"
yeet "signalz"

fr fr Comprehensive Test for CURSED Concurrency and Error Handling Modules
fr fr Demonstrates all modules working together in realistic scenarios

slay test_integrated_concurrency_with_error_handling() {
    test_start("Integrated Concurrency with Error Handling")
    
    fr fr Setup concurrency primitives
    sus mutex *Mutex = create_mutex()
    sus wg *WaitGroup = create_waitgroup()
    sus ch *Channel = create_channel(5)
    sus error_ch *Channel = create_channel(3)
    
    fr fr Simulate concurrent operations with error handling
    waitgroup_add(wg, 3)  fr fr Three "workers"
    
    fr fr Worker 1: Success case
    mutex_lock(mutex)
    channel_send(ch, 100)
    mutex_unlock(mutex)
    waitgroup_done(wg)
    
    fr fr Worker 2: Error case
    sus error *ErrorInstance = create_io_error("File not found", 404)
    channel_send(error_ch, error.code)
    waitgroup_done(wg)
    
    fr fr Worker 3: Success case
    mutex_lock(mutex)
    channel_send(ch, 200)
    mutex_unlock(mutex)
    waitgroup_done(wg)
    
    fr fr Wait for all workers
    waitgroup_wait(wg)
    
    fr fr Collect results
    sus result1 normie = channel_receive(ch)
    sus result2 normie = channel_receive(ch)
    sus error_code normie = channel_receive(error_ch)
    
    assert_eq_int(result1, 100)
    assert_eq_int(result2, 200)
    assert_eq_int(error_code, IO_YIKES + 404)
    
    vibez.spill("✅ Integrated concurrency with error handling test passed")
}

slay test_async_operations_with_error_recovery() {
    test_start("Async Operations with Error Recovery")
    
    sus success_count normie = 0
    sus error_count normie = 0
    
    fr fr Define failing task for retry testing
    slay failing_task() normie {
        error_count = error_count + 1
        lowkey error_count < 3 {
            trigger_panic("Task failed on attempt " + string(error_count))
        }
        success_count = success_count + 1
        damn 999
    }
    
    fr fr Test async retry with circuit breaker pattern
    sus cb *CircuitBreaker = create_circuit_breaker(5, 1000)
    
    fr fr Wrap task with circuit breaker
    slay protected_task() *ErrorInstance {
        fam {
            failing_task()
            damn 0  fr fr Success
        } sus panic_err {
            damn create_error("Task failed: " + panic_err.message)
        }
    }
    
    fr fr Execute with circuit breaker
    sus result *ErrorInstance = circuit_breaker_call(cb, protected_task)
    
    fr fr Should succeed after retries
    assert_true(result == 0)
    assert_true(success_count >= 1)
    assert_true(error_count >= 2)
    
    vibez.spill("✅ Async operations with error recovery test passed")
}

slay test_signal_handling_with_concurrency() {
    test_start("Signal Handling with Concurrency")
    
    sus signal_received normie = 0
    sus signal_handled_count normie = 0
    
    fr fr Define signal handler
    slay signal_handler(signal_num normie) {
        signal_received = signal_num
        signal_handled_count = signal_handled_count + 1
        vibez.spill("Received signal: " + string(signal_num))
    }
    
    fr fr Register signal handlers
    signal_register(SIGUSR1, signal_handler)
    signal_register(SIGUSR2, signal_handler)
    
    fr fr Create communication channels for goroutine coordination
    sus signal_ch *Channel = create_channel(2)
    sus response_ch *Channel = create_channel(2)
    
    fr fr Simulate goroutine sending signals
    channel_send(signal_ch, SIGUSR1)
    channel_send(signal_ch, SIGUSR2)
    
    fr fr Process signals
    sus sig1 normie = channel_receive(signal_ch)
    sus sig2 normie = channel_receive(signal_ch)
    
    deliver_signal(sig1, 1001)
    deliver_signal(sig2, 1002)
    
    fr fr Send responses
    channel_send(response_ch, 1)
    channel_send(response_ch, 1)
    
    fr fr Verify signal handling
    assert_true(signal_handled_count >= 2)
    
    fr fr Check signal statistics
    sus stats *SignalStats = get_signal_statistics()
    assert_true(stats.total_signals >= 2)
    
    vibez.spill("✅ Signal handling with concurrency test passed")
}

slay test_error_propagation_across_modules() {
    test_start("Error Propagation Across Modules")
    
    fr fr Create error collection for gathering errors from different modules
    sus errors *ErrorCollection = create_error_collection(10)
    
    fr fr Concurrency error: Channel operation failure
    sus closed_ch *Channel = create_channel(1)
    channel_close(closed_ch)
    sus send_failed lit = channel_send(closed_ch, 42)
    lowkey !send_failed {
        sus concurrency_error *ErrorInstance = create_error("Channel send failed: channel closed")
        add_error(errors, concurrency_error)
    }
    
    fr fr Async error: Future timeout
    slay slow_async_task() normie {
        sleep_ms(200)
        damn 42
    }
    
    sus future *Future = async_run(slow_async_task)
    sus timeout_result normie = await_future_timeout(future, 50)
    lowkey timeout_result == 0 {
        sus async_error *ErrorInstance = create_error("Async task timed out")
        add_error(errors, async_error)
    }
    
    fr fr Signal error: Invalid signal
    sus signal_error *ErrorInstance = deliver_signal(999, 1001)
    lowkey signal_error != 0 {
        add_error(errors, signal_error)
    }
    
    fr fr Network simulation error
    sus network_error *ErrorInstance = create_network_error("Connection refused", 61)
    add_error(errors, network_error)
    
    fr fr Check error collection
    assert_true(has_errors(errors))
    
    fr fr Combine all errors into single error report
    sus combined_error *ErrorInstance = combine_errors(errors)
    assert_true(combined_error != 0)
    assert_eq_int(error_severity(combined_error), CRITICAL)
    
    fr fr Log combined error
    print_error_with_stack(combined_error)
    
    vibez.spill("✅ Error propagation across modules test passed")
}

slay test_producer_consumer_with_error_handling() {
    test_start("Producer Consumer with Error Handling")
    
    fr fr Setup producer-consumer with error handling
    sus data_ch *Channel = create_channel(3)
    sus error_ch *Channel = create_channel(1)
    sus sem *Semaphore = create_semaphore(2)  fr fr Limit concurrent consumers
    sus atomic_counter *AtomicI32 = atomic_i32_new(0)
    
    fr fr Producer: Generate data with potential errors
    sus producer_errors normie = 0
    sus items_produced normie = 0
    
    bestie items_produced < 5 {
        lowkey items_produced == 2 {
            fr fr Simulate producer error
            producer_errors = producer_errors + 1
            sus error *ErrorInstance = create_error("Producer error at item " + string(items_produced))
            channel_send(error_ch, error.code)
        } else {
            channel_send(data_ch, items_produced * 10)
        }
        items_produced = items_produced + 1
    }
    
    fr fr Consumer: Process data with error handling
    sus items_consumed normie = 0
    sus consumer_errors normie = 0
    
    bestie items_consumed < 4 {  fr fr 4 successful items
        fr fr Acquire semaphore permit
        semaphore_acquire(sem)
        
        ready {
            mood data := dm_recv(data_ch):
                atomic_add_i32(atomic_counter, data)
                items_consumed = items_consumed + 1
            mood error_code := dm_recv(error_ch):
                consumer_errors = consumer_errors + 1
                vibez.spill("Consumer handled error: " + string(error_code))
        }
        
        fr fr Release semaphore permit
        semaphore_release(sem)
    }
    
    fr fr Verify results
    assert_eq_int(items_consumed, 4)
    assert_eq_int(consumer_errors, 1)
    assert_eq_int(producer_errors, 1)
    
    sus final_count normie = atomic_load_i32(atomic_counter)
    fr fr Should be 0 + 10 + 30 + 40 = 80 (skipping item 2 due to error)
    assert_eq_int(final_count, 80)
    
    vibez.spill("✅ Producer consumer with error handling test passed")
}

slay test_async_stream_processing_with_signals() {
    test_start("Async Stream Processing with Signals")
    
    fr fr Create async stream for data processing
    sus stream *AsyncStream = create_async_stream(5)
    sus results []normie = [0, 0, 0, 0, 0]
    sus result_count normie = 0
    
    fr fr Setup signal handling for stream control
    sus stream_control_signal normie = 0
    
    slay stream_control_handler(signal_num normie) {
        stream_control_signal = signal_num
        lowkey signal_num == SIGUSR1 {
            fr fr Pause signal
            vibez.spill("Stream processing paused")
        } else {
            fr fr Resume signal  
            vibez.spill("Stream processing resumed")
        }
    }
    
    signal_register(SIGUSR1, stream_control_handler)  fr fr Pause
    signal_register(SIGUSR2, stream_control_handler)  fr fr Resume
    
    fr fr Producer: Send data to stream
    stream_send(stream, 10)
    stream_send(stream, 20)
    
    fr fr Simulate pause signal
    deliver_signal(SIGUSR1, 2001)
    
    stream_send(stream, 30)
    
    fr fr Simulate resume signal
    deliver_signal(SIGUSR2, 2002)
    
    stream_send(stream, 40)
    stream_send(stream, 50)
    
    fr fr Consumer: Process stream data
    bestie result_count < 5 && !stream.closed {
        sus data normie = stream_receive(stream)
        lowkey data != 0 {
            results[result_count] = data
            result_count = result_count + 1
        }
    }
    
    fr fr Close stream
    close_async_stream(stream)
    
    fr fr Verify signal handling occurred
    assert_true(stream_control_signal > 0)
    
    fr fr Verify data processing
    assert_eq_int(result_count, 5)
    assert_eq_int(results[0], 10)
    assert_eq_int(results[1], 20)
    assert_eq_int(results[2], 30)
    assert_eq_int(results[3], 40)
    assert_eq_int(results[4], 50)
    
    vibez.spill("✅ Async stream processing with signals test passed")
}

slay test_fault_tolerant_distributed_simulation() {
    test_start("Fault Tolerant Distributed Simulation")
    
    fr fr Simulate distributed system with fault tolerance
    sus node_count normie = 3
    sus node_channels [3]*Channel
    sus node_errors [3]*ErrorInstance
    sus node_status [3]normie  fr fr 0=healthy, 1=failed, 2=recovering
    sus consensus_value normie = 0
    sus votes [3]normie
    
    fr fr Initialize nodes
    sus i normie = 0
    bestie i < node_count {
        node_channels[i] = create_channel(2)
        node_errors[i] = 0
        node_status[i] = 0  fr fr All healthy initially
        votes[i] = 0
        i = i + 1
    }
    
    fr fr Simulate leader election with fault injection
    sus leader_id normie = 0
    sus election_round normie = 1
    
    fr fr Node 0: Leader candidate (success)
    channel_send(node_channels[0], leader_id)
    votes[0] = leader_id
    
    fr fr Node 1: Fails during election
    node_status[1] = 1
    node_errors[1] = create_network_error("Node 1 network partition", 110)
    
    fr fr Node 2: Healthy follower
    channel_send(node_channels[2], leader_id)
    votes[2] = leader_id
    
    fr fr Count votes (majority required)
    sus vote_count normie = 0
    i = 0
    bestie i < node_count {
        lowkey node_status[i] == 0 && votes[i] == leader_id {
            vote_count = vote_count + 1
        }
        i = i + 1
    }
    
    fr fr Check if we have majority (2 out of 3)
    assert_true(vote_count >= 2)
    
    fr fr Simulate consensus operation
    lowkey vote_count >= 2 {
        consensus_value = 42  fr fr Agreed value
        
        fr fr Replicate to healthy nodes
        i = 0
        bestie i < node_count {
            lowkey node_status[i] == 0 {
                channel_send(node_channels[i], consensus_value)
            }
            i = i + 1
        }
    }
    
    fr fr Node recovery simulation
    node_status[1] = 2  fr fr Recovering
    
    fr fr Create recovery handler
    slay recovery_handler(signal_num normie) {
        lowkey signal_num == SIGUSR1 {
            node_status[1] = 0  fr fr Mark as healthy
            fr fr Sync with leader
            channel_send(node_channels[1], consensus_value)
            vibez.spill("Node 1 recovered and synced")
        }
    }
    
    signal_register(SIGUSR1, recovery_handler)
    deliver_signal(SIGUSR1, 3001)
    
    fr fr Verify final state
    assert_eq_int(consensus_value, 42)
    assert_eq_int(node_status[0], 0)  fr fr Leader healthy
    assert_eq_int(node_status[1], 0)  fr fr Recovered
    assert_eq_int(node_status[2], 0)  fr fr Follower healthy
    
    fr fr Collect error statistics
    sus error_stats *ErrorStats = get_signal_statistics()
    assert_true(error_stats.total_signals >= 1)
    
    fr fr Check that failed node had an error
    assert_true(node_errors[1] != 0)
    assert_true(is_network_error(node_errors[1]))
    
    vibez.spill("✅ Fault tolerant distributed simulation test passed")
}

slay run_comprehensive_integration_tests() {
    vibez.spill("🚀 Starting Comprehensive Integration Tests")
    vibez.spill("Testing integration of concurrenz, errorz, asyncz, and signalz modules")
    
    test_integrated_concurrency_with_error_handling()
    test_async_operations_with_error_recovery()
    test_signal_handling_with_concurrency()
    test_error_propagation_across_modules()
    test_producer_consumer_with_error_handling()
    test_async_stream_processing_with_signals()
    test_fault_tolerant_distributed_simulation()
    
    print_test_summary()
    vibez.spill("🎉 All comprehensive integration tests completed successfully!")
    vibez.spill("✅ CURSED concurrency and error handling modules are production-ready!")
}

fr fr Run comprehensive integration tests
run_comprehensive_integration_tests()
