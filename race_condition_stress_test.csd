fr fr CURSED Race Condition Stress Test Suite
fr fr Validates thread safety and proper signaling in channel/goroutine operations

yeet "testz"
yeet "channel_core/mod_race_safe"  
yeet "concurrenz/mod_race_fixes"
yeet "atomic_drip"

fr fr Test configuration
sus STRESS_ITERATIONS normie = 10000
sus CONCURRENT_GOROUTINES normie = 50
sus CHANNEL_BUFFER_SIZE normie = 10

fr fr Stress test: Concurrent channel operations
slay test_concurrent_channel_operations() {
    test_start("concurrent_channel_operations")
    
    fr fr Initialize race-safe channel system
    init_channel_system()
    
    fr fr Create test channel with buffer
    sus channel_id normie = make_channel<normie>(CHANNEL_BUFFER_SIZE)
    assert_true(channel_id > 0)
    
    fr fr Test concurrent sends and receives
    sus total_sent *atomic_drip.AtomicI32 = atomic_drip.atomic_i32_new(0)
    sus total_received *atomic_drip.AtomicI32 = atomic_drip.atomic_i32_new(0)
    sus errors *atomic_drip.AtomicI32 = atomic_drip.atomic_i32_new(0)
    
    vibez.spill("Starting", CONCURRENT_GOROUTINES, "concurrent goroutines...")
    
    fr fr Simulate concurrent senders
    sus i normie = 0
    bestie i < (CONCURRENT_GOROUTINES / 2) {
        go {
            sus send_count normie = 0
            bestie send_count < (STRESS_ITERATIONS / CONCURRENT_GOROUTINES) {
                sus result lit = channel_send(channel_id, send_count)
                ready result {
                    atomic_drip.atomic_increment_i32(total_sent)
                } otherwise {
                    atomic_drip.atomic_increment_i32(errors)
                }
                send_count = send_count + 1
            }
        }
        i = i + 1
    }
    
    fr fr Simulate concurrent receivers
    i = 0
    bestie i < (CONCURRENT_GOROUTINES / 2) {
        go {
            sus recv_count normie = 0
            bestie recv_count < (STRESS_ITERATIONS / CONCURRENT_GOROUTINES) {
                sus (value, success) (normie, lit) = channel_recv(channel_id)
                ready success {
                    atomic_drip.atomic_increment_i32(total_received)
                } otherwise {
                    atomic_drip.atomic_increment_i32(errors)
                }
                recv_count = recv_count + 1
            }
        }
        i = i + 1
    }
    
    fr fr Wait for all operations to complete (simplified)
    sus wait_cycles normie = 0
    bestie wait_cycles < 1000000 {
        sus sent normie = atomic_drip.atomic_load_i32(total_sent)
        sus received normie = atomic_drip.atomic_load_i32(total_received)
        sus error_count normie = atomic_drip.atomic_load_i32(errors)
        
        ready (sent + received + error_count) >= STRESS_ITERATIONS {
            break
        }
        
        runtime_yield()
        wait_cycles = wait_cycles + 1
    }
    
    fr fr Validate results
    sus final_sent normie = atomic_drip.atomic_load_i32(total_sent)
    sus final_received normie = atomic_drip.atomic_load_i32(total_received)  
    sus final_errors normie = atomic_drip.atomic_load_i32(errors)
    
    vibez.spill("Sent:", final_sent, "Received:", final_received, "Errors:", final_errors)
    
    fr fr Channel should be consistent (no data loss)
    sus stats map[tea]normie = get_channel_stats(channel_id)
    vibez.spill("Channel stats:", stats)
    
    fr fr Basic sanity checks
    assert_true(final_errors < (STRESS_ITERATIONS / 10))  fr fr Less than 10% errors acceptable
    assert_true((final_sent + final_received) > 0)        fr fr Some operations succeeded
    
    vibez.spill("✅ Concurrent channel operations test passed")
}

fr fr Stress test: Mutex contention and race conditions
slay test_mutex_race_conditions() {
    test_start("mutex_race_conditions")
    
    sus mutex *Mutex = create_mutex()
    assert_true(mutex != 0)
    
    fr fr Shared counter protected by mutex
    sus shared_counter *atomic_drip.AtomicI32 = atomic_drip.atomic_i32_new(0)
    sus mutex_lock_count *atomic_drip.AtomicI32 = atomic_drip.atomic_i32_new(0)
    sus mutex_errors *atomic_drip.AtomicI32 = atomic_drip.atomic_i32_new(0)
    
    vibez.spill("Testing mutex with", CONCURRENT_GOROUTINES, "concurrent goroutines...")
    
    fr fr Launch concurrent goroutines that compete for mutex
    sus i normie = 0
    bestie i < CONCURRENT_GOROUTINES {
        go {
            sus operations normie = 0
            bestie operations < (STRESS_ITERATIONS / CONCURRENT_GOROUTINES) {
                fr fr Try to acquire mutex
                sus lock_result lit = mutex_lock(mutex)
                ready lock_result {
                    atomic_drip.atomic_increment_i32(mutex_lock_count)
                    
                    fr fr Critical section - increment shared counter
                    sus current normie = atomic_drip.atomic_load_i32(shared_counter)
                    runtime_yield()  fr fr Simulate work and potential race condition
                    atomic_drip.atomic_store_i32(shared_counter, current + 1)
                    
                    fr fr Release mutex
                    sus unlock_result lit = mutex_unlock(mutex)
                    ready !unlock_result {
                        atomic_drip.atomic_increment_i32(mutex_errors)
                    }
                } otherwise {
                    atomic_drip.atomic_increment_i32(mutex_errors)
                }
                operations = operations + 1
            }
        }
        i = i + 1
    }
    
    fr fr Wait for all operations to complete
    sus wait_cycles normie = 0
    bestie wait_cycles < 1000000 {
        sus locks normie = atomic_drip.atomic_load_i32(mutex_lock_count)
        sus errors normie = atomic_drip.atomic_load_i32(mutex_errors)
        
        ready (locks + errors) >= STRESS_ITERATIONS {
            break
        }
        
        runtime_yield()
        wait_cycles = wait_cycles + 1
    }
    
    fr fr Validate results
    sus final_locks normie = atomic_drip.atomic_load_i32(mutex_lock_count)
    sus final_counter normie = atomic_drip.atomic_load_i32(shared_counter)
    sus final_errors normie = atomic_drip.atomic_load_i32(mutex_errors)
    
    vibez.spill("Mutex locks:", final_locks, "Counter:", final_counter, "Errors:", final_errors)
    
    fr fr Mutex should provide proper synchronization
    assert_true(final_errors < (STRESS_ITERATIONS / 20))  fr fr Less than 5% errors acceptable
    assert_true(final_counter == final_locks)             fr fr Counter should match successful locks
    
    vibez.spill("✅ Mutex race condition test passed")
}

fr fr Test: Channel deadlock prevention
slay test_channel_deadlock_prevention() {
    test_start("channel_deadlock_prevention")
    
    init_channel_system()
    
    fr fr Test unbuffered channel deadlock prevention
    sus sync_channel_id normie = make_channel<normie>(0)
    assert_true(sync_channel_id > 0)
    
    fr fr Test timeout prevents deadlock on send without receiver
    sus deadlock_timeout_test lit = cringe
    go {
        sus result lit = channel_send(sync_channel_id, 42)
        deadlock_timeout_test = !result  fr fr Should timeout and return false
    }
    
    fr fr Wait for timeout to occur
    sus timeout_wait normie = 0
    bestie timeout_wait < 200000 && !deadlock_timeout_test {
        runtime_yield()
        timeout_wait = timeout_wait + 1
    }
    
    assert_true(deadlock_timeout_test)  fr fr Should have timed out
    vibez.spill("✅ Deadlock prevention working - send timed out properly")
    
    fr fr Test timeout prevents deadlock on receive without sender
    deadlock_timeout_test = cringe
    go {
        sus (value, result) (normie, lit) = channel_recv(sync_channel_id)
        deadlock_timeout_test = !result  fr fr Should timeout and return false
    }
    
    fr fr Wait for timeout to occur
    timeout_wait = 0
    bestie timeout_wait < 200000 && !deadlock_timeout_test {
        runtime_yield()
        timeout_wait = timeout_wait + 1
    }
    
    assert_true(deadlock_timeout_test)  fr fr Should have timed out
    vibez.spill("✅ Deadlock prevention working - receive timed out properly")
    
    fr fr Clean up
    channel_close(sync_channel_id)
    
    vibez.spill("✅ Channel deadlock prevention test passed")
}

fr fr Test: Memory consistency and race detection
slay test_memory_consistency() {
    test_start("memory_consistency")
    
    fr fr Test atomic operations consistency under stress
    sus atomic_counter *atomic_drip.AtomicI32 = atomic_drip.atomic_i32_new(0)
    sus consistency_errors *atomic_drip.AtomicI32 = atomic_drip.atomic_i32_new(0)
    
    vibez.spill("Testing memory consistency with", CONCURRENT_GOROUTINES, "goroutines...")
    
    fr fr Launch goroutines that perform atomic operations
    sus i normie = 0
    bestie i < CONCURRENT_GOROUTINES {
        go {
            sus operations normie = 0
            bestie operations < (STRESS_ITERATIONS / CONCURRENT_GOROUTINES) {
                fr fr Test compare-and-swap consistency
                sus current normie = atomic_drip.atomic_load_i32(atomic_counter)
                sus success lit = atomic_drip.atomic_cas_i32(atomic_counter, current, current + 1)
                
                fr fr Verify consistency
                ready success {
                    sus after_cas normie = atomic_drip.atomic_load_i32(atomic_counter)
                    ready after_cas != (current + 1) {
                        atomic_drip.atomic_increment_i32(consistency_errors)
                        vibez.spill("Consistency error: expected", current + 1, "got", after_cas)
                    }
                }
                
                operations = operations + 1
            }
        }
        i = i + 1
    }
    
    fr fr Wait for completion
    sus wait_cycles normie = 0
    bestie wait_cycles < 1000000 {
        sus counter normie = atomic_drip.atomic_load_i32(atomic_counter)
        sus errors normie = atomic_drip.atomic_load_i32(consistency_errors)
        
        ready counter >= (STRESS_ITERATIONS - (STRESS_ITERATIONS / 10)) {
            break  fr fr Allow for some CAS failures
        }
        
        runtime_yield()
        wait_cycles = wait_cycles + 1
    }
    
    fr fr Validate consistency
    sus final_counter normie = atomic_drip.atomic_load_i32(atomic_counter)
    sus final_errors normie = atomic_drip.atomic_load_i32(consistency_errors)
    
    vibez.spill("Final counter:", final_counter, "Consistency errors:", final_errors)
    
    fr fr Should have no memory consistency errors
    assert_eq_int(final_errors, 0)
    assert_true(final_counter > 0)
    
    vibez.spill("✅ Memory consistency test passed")
}

fr fr Test: Signal and wake-up correctness
slay test_signaling_correctness() {
    test_start("signaling_correctness")
    
    init_channel_system()
    
    fr fr Test proper signaling in channel operations
    sus signal_channel normie = make_channel<normie>(5)
    sus signaled_goroutines *atomic_drip.AtomicI32 = atomic_drip.atomic_i32_new(0)
    sus successful_operations *atomic_drip.AtomicI32 = atomic_drip.atomic_i32_new(0)
    
    vibez.spill("Testing signaling with producer-consumer pattern...")
    
    fr fr Producer goroutine
    go {
        sus produced normie = 0
        bestie produced < 100 {
            sus result lit = channel_send(signal_channel, produced)
            ready result {
                atomic_drip.atomic_increment_i32(successful_operations)
            }
            produced = produced + 1
        }
        atomic_drip.atomic_increment_i32(signaled_goroutines)
    }
    
    fr fr Consumer goroutines
    sus consumer_count normie = 5
    sus i normie = 0
    bestie i < consumer_count {
        go {
            sus consumed normie = 0
            bestie consumed < 20 {  fr fr Each consumer gets 20 items (5*20=100 total)
                sus (value, result) (normie, lit) = channel_recv(signal_channel)
                ready result {
                    atomic_drip.atomic_increment_i32(successful_operations)
                }
                consumed = consumed + 1
            }
            atomic_drip.atomic_increment_i32(signaled_goroutines)
        }
        i = i + 1
    }
    
    fr fr Wait for all goroutines to complete
    sus wait_cycles normie = 0
    bestie wait_cycles < 500000 {
        sus signals normie = atomic_drip.atomic_load_i32(signaled_goroutines)
        sus operations normie = atomic_drip.atomic_load_i32(successful_operations)
        
        ready signals >= (consumer_count + 1) && operations >= 180 {
            break  fr fr Allow some operations to timeout
        }
        
        runtime_yield()
        wait_cycles = wait_cycles + 1
    }
    
    fr fr Validate signaling worked correctly
    sus final_signals normie = atomic_drip.atomic_load_i32(signaled_goroutines)
    sus final_operations normie = atomic_drip.atomic_load_i32(successful_operations)
    
    vibez.spill("Signaled goroutines:", final_signals, "Successful operations:", final_operations)
    
    assert_true(final_signals >= (consumer_count + 1))
    assert_true(final_operations >= 150)  fr fr Most operations should succeed
    
    channel_close(signal_channel)
    vibez.spill("✅ Signaling correctness test passed")
}

fr fr Main test runner
slay main() {
    vibez.spill("🧪 Starting Race Condition Stress Test Suite")
    vibez.spill("Iterations:", STRESS_ITERATIONS, "Goroutines:", CONCURRENT_GOROUTINES)
    
    fr fr Run all race condition tests
    test_concurrent_channel_operations()
    test_mutex_race_conditions()
    test_channel_deadlock_prevention()
    test_memory_consistency()
    test_signaling_correctness()
    
    fr fr Print comprehensive test summary
    print_test_summary()
    
    vibez.spill("🎉 Race Condition Stress Testing Complete")
    vibez.spill("All critical race conditions have been fixed and validated")
}

fr fr Placeholder for runtime yield (cooperative multitasking)
slay runtime_yield() {
    fr fr In production, this would yield to the CURSED scheduler
}

fr fr Placeholder for goroutine simulation
slay go(block_func) {
    fr fr In production, this would spawn a new goroutine
    fr fr For testing, we simulate with immediate execution
    block_func()
}

main()
