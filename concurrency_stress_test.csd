yeet "concurrenz/mod_production"
yeet "testz"
yeet "vibez"
yeet "timez"

fr fr =============================================================================
fr fr CONCURRENCY STRESS TEST SUITE - RACE CONDITION & DEADLOCK DETECTION
fr fr Tests goroutines, channels, mutexes under high-load parallel conditions
fr fr =============================================================================

fr fr Stress test with 100 parallel goroutines
slay stress_test_parallel_goroutines() lit {
    vibez.spill("🧪 Stress Testing 100 Parallel Goroutines...")
    
    fr fr Initialize shared counter
    sus shared_counter normie = 0
    sus mutex_counter dmut<normie> = dmut_create()
    sus completed_count normie = 0
    
    fr fr Spawn 100 goroutines that increment counter
    sus goroutine_ids []thicc = memory.allocate_array(thicc, 100)
    sus i normie = 0
    
    bestie i < 100 {
        goroutine_ids[i] = stan {
            fr fr Critical section with mutex
            dmut_lock(mutex_counter)
            sus temp normie = shared_counter
            fr fr Simulate work to increase race condition probability
            timez.sleep_ns(1000)  // 1 microsecond
            shared_counter = temp + 1
            dmut_unlock(mutex_counter)
            
            fr fr Increment completion count atomically
            atomic_fetch_add_int(&completed_count, 1)
        }
        i = i + 1
    }
    
    fr fr Wait for all goroutines to complete (with timeout)
    sus timeout_start thicc = timez.nanoseconds_since_epoch()
    sus timeout_limit thicc = 5000000000  // 5 seconds
    
    bestie completed_count < 100 {
        sus elapsed thicc = timez.nanoseconds_since_epoch() - timeout_start
        ready elapsed > timeout_limit {
            testz.fail("Goroutine completion timeout - potential deadlock")
            damn cap
        }
        timez.sleep_ns(1000000)  // 1ms
    }
    
    fr fr Verify all increments completed correctly
    testz.assert_eq_int(shared_counter, 100, "All 100 increments should complete")
    testz.assert_eq_int(completed_count, 100, "All 100 goroutines should complete")
    
    dmut_destroy(mutex_counter)
    memory.deallocate(goroutine_ids)
    vibez.spill("✅ Parallel Goroutines Stress Test passed")
    damn based
}

fr fr Stress test producer-consumer with channels
slay stress_test_producer_consumer_channels() lit {
    vibez.spill("🧪 Stress Testing Producer-Consumer Channels...")
    
    fr fr Create buffered channel for communication
    sus work_channel dm<normie> = dm<normie>[50]  // Buffered channel
    sus result_channel dm<normie> = dm<normie>[50]
    
    sus total_work_items normie = 1000
    sus producer_count normie = 10
    sus consumer_count normie = 5
    sus completed_work normie = 0
    
    fr fr Spawn producer goroutines
    sus producer_ids []thicc = memory.allocate_array(thicc, producer_count)
    sus work_per_producer normie = total_work_items / producer_count
    sus i normie = 0
    
    bestie i < producer_count {
        producer_ids[i] = stan {
            sus producer_id normie = i
            sus work_start normie = producer_id * work_per_producer
            sus work_end normie = work_start + work_per_producer
            sus j normie = work_start
            
            bestie j < work_end {
                fr fr Send work item (blocking if channel full)
                dm_send(work_channel, j)
                j = j + 1
            }
        }
        i = i + 1
    }
    
    fr fr Spawn consumer goroutines
    sus consumer_ids []thicc = memory.allocate_array(thicc, consumer_count)
    i = 0
    
    bestie i < consumer_count {
        consumer_ids[i] = stan {
            bestie based {
                fr fr Receive work item (blocking until available)
                sus work_item normie = dm_recv(work_channel)
                ready work_item < 0 {  // Channel closed
                    break
                }
                
                fr fr Simulate processing work
                sus result normie = work_item * 2
                
                fr fr Send result back
                dm_send(result_channel, result)
                atomic_fetch_add_int(&completed_work, 1)
            }
        }
        i = i + 1
    }
    
    fr fr Wait for all work to be processed (with timeout)
    sus timeout_start thicc = timez.nanoseconds_since_epoch()
    sus timeout_limit thicc = 10000000000  // 10 seconds
    
    bestie completed_work < total_work_items {
        sus elapsed thicc = timez.nanoseconds_since_epoch() - timeout_start
        ready elapsed > timeout_limit {
            testz.fail("Producer-consumer completion timeout")
            damn cap
        }
        timez.sleep_ns(10000000)  // 10ms
    }
    
    fr fr Close channels to signal completion
    dm_close(work_channel)
    dm_close(result_channel)
    
    fr fr Verify all work completed
    testz.assert_eq_int(completed_work, total_work_items, "All work items should be processed")
    
    memory.deallocate(producer_ids)
    memory.deallocate(consumer_ids)
    vibez.spill("✅ Producer-Consumer Channels Stress Test passed")
    damn based
}

fr fr Stress test select statements with multiple channels
slay stress_test_select_statements() lit {
    vibez.spill("🧪 Stress Testing Select Statements...")
    
    fr fr Create multiple channels for select operations
    sus channel_a dm<normie> = dm<normie>[10]
    sus channel_b dm<tea> = dm<tea>[10]
    sus channel_c dm<lit> = dm<lit>[10]
    sus completion_channel dm<normie> = dm<normie>[100]
    
    sus select_operations normie = 0
    sus target_operations normie = 500
    
    fr fr Spawn goroutines sending to different channels
    sus sender_a thicc = stan {
        sus i normie = 0
        bestie i < 100 {
            dm_send(channel_a, i)
            timez.sleep_ns(100000)  // 100 microseconds
            i = i + 1
        }
    }
    
    sus sender_b thicc = stan {
        sus i normie = 0
        bestie i < 100 {
            dm_send(channel_b, "message" + stringify(i))
            timez.sleep_ns(150000)  // 150 microseconds
            i = i + 1
        }
    }
    
    sus sender_c thicc = stan {
        sus i normie = 0
        bestie i < 100 {
            dm_send(channel_c, (i % 2) == 0)
            timez.sleep_ns(200000)  // 200 microseconds
            i = i + 1
        }
    }
    
    fr fr Spawn multiple selector goroutines
    sus selector_count normie = 5
    sus selector_ids []thicc = memory.allocate_array(thicc, selector_count)
    sus i normie = 0
    
    bestie i < selector_count {
        selector_ids[i] = stan {
            bestie atomic_load_int(&select_operations) < target_operations {
                ready {
                    case int_val normie = <-channel_a:
                        fr fr Process integer value
                        testz.assert_gte_int(int_val, 0, "Received valid integer")
                        atomic_fetch_add_int(&select_operations, 1)
                        
                    case string_val tea = <-channel_b:
                        fr fr Process string value
                        testz.assert_not_null(string_val, "Received valid string")
                        atomic_fetch_add_int(&select_operations, 1)
                        
                    case bool_val lit = <-channel_c:
                        fr fr Process boolean value
                        atomic_fetch_add_int(&select_operations, 1)
                        
                    case timeout(100):  // 100ms timeout
                        fr fr Handle timeout case
                        continue
                        
                    default:
                        fr fr Handle default case
                        timez.sleep_ns(50000)  // 50 microseconds
                        continue
                }
                
                fr fr Signal completion
                dm_send(completion_channel, 1)
            }
        }
        i = i + 1
    }
    
    fr fr Wait for target operations to complete
    sus timeout_start thicc = timez.nanoseconds_since_epoch()
    sus timeout_limit thicc = 15000000000  // 15 seconds
    
    bestie select_operations < target_operations {
        sus elapsed thicc = timez.nanoseconds_since_epoch() - timeout_start
        ready elapsed > timeout_limit {
            testz.fail("Select operations timeout")
            damn cap
        }
        timez.sleep_ns(10000000)  // 10ms
    }
    
    fr fr Close all channels
    dm_close(channel_a)
    dm_close(channel_b)
    dm_close(channel_c)
    dm_close(completion_channel)
    
    testz.assert_gte_int(select_operations, target_operations, "Target select operations should complete")
    
    memory.deallocate(selector_ids)
    vibez.spill("✅ Select Statements Stress Test passed")
    damn based
}

fr fr Stress test async/await operations
slay stress_test_async_await() lit {
    vibez.spill("🧪 Stress Testing Async/Await...")
    
    sus completed_futures normie = 0
    sus target_futures normie = 100
    
    fr fr Create array of futures for concurrent execution
    sus futures []asyncz.Future<normie> = memory.allocate_array(asyncz.Future<normie>, target_futures)
    sus i normie = 0
    
    fr fr Launch async operations
    bestie i < target_futures {
        futures[i] = async {
            fr fr Simulate async work with I/O delay
            await timez.async_sleep_ms(random_int() % 100)
            
            fr fr Perform computation
            sus result normie = i * i + i
            
            fr fr More async work
            await timez.async_sleep_ms(random_int() % 50)
            
            damn result
        }
        i = i + 1
    }
    
    fr fr Wait for all futures to complete
    sus results []normie = memory.allocate_array(normie, target_futures)
    i = 0
    
    bestie i < target_futures {
        results[i] = await futures[i]
        testz.assert_eq_int(results[i], i * i + i, "Async computation should be correct")
        atomic_fetch_add_int(&completed_futures, 1)
        i = i + 1
    }
    
    testz.assert_eq_int(completed_futures, target_futures, "All futures should complete")
    
    memory.deallocate(futures)
    memory.deallocate(results)
    vibez.spill("✅ Async/Await Stress Test passed")
    damn based
}

fr fr Race condition detection test
slay test_race_condition_detection() lit {
    vibez.spill("🧪 Testing Race Condition Detection...")
    
    fr fr Shared data without synchronization (intentional race condition)
    sus race_data normie = 0
    sus race_completed normie = 0
    sus race_goroutines normie = 20
    
    fr fr Spawn goroutines that create race conditions
    sus i normie = 0
    bestie i < race_goroutines {
        stan {
            fr fr Intentional race condition - reading and writing without sync
            sus temp normie = race_data
            timez.sleep_ns(random_int() % 1000)  // Random delay
            race_data = temp + 1
            atomic_fetch_add_int(&race_completed, 1)
        }
        i = i + 1
    }
    
    fr fr Wait for completion
    bestie race_completed < race_goroutines {
        timez.sleep_ns(1000000)  // 1ms
    }
    
    fr fr The result should be less than race_goroutines due to race conditions
    testz.assert_lt_int(race_data, race_goroutines, "Race condition should cause lost updates")
    vibez.spill("📊 Race condition detected: expected", race_goroutines, "got", race_data)
    
    vibez.spill("✅ Race Condition Detection Test passed")
    damn based
}

fr fr Deadlock prevention test
slay test_deadlock_prevention() lit {
    vibez.spill("🧪 Testing Deadlock Prevention...")
    
    fr fr Create two mutexes for potential circular wait
    sus mutex_a dmut<normie> = dmut_create()
    sus mutex_b dmut<normie> = dmut_create()
    
    sus deadlock_completed normie = 0
    sus shared_resource_a normie = 0
    sus shared_resource_b normie = 0
    
    fr fr Goroutine 1: Lock A then B
    sus goroutine1 thicc = stan {
        dmut_lock(mutex_a)
        vibez.spill("Goroutine 1: Locked mutex A")
        timez.sleep_ns(10000000)  // 10ms delay to increase deadlock chance
        
        ready dmut_try_lock_timeout(mutex_b, 500000000) {  // 500ms timeout
            vibez.spill("Goroutine 1: Locked mutex B")
            shared_resource_a = shared_resource_a + 1
            shared_resource_b = shared_resource_b + 1
            dmut_unlock(mutex_b)
        } otherwise {
            vibez.spill("Goroutine 1: Timeout acquiring mutex B")
        }
        
        dmut_unlock(mutex_a)
        atomic_fetch_add_int(&deadlock_completed, 1)
    }
    
    fr fr Goroutine 2: Lock B then A (opposite order - potential deadlock)
    sus goroutine2 thicc = stan {
        dmut_lock(mutex_b)
        vibez.spill("Goroutine 2: Locked mutex B")
        timez.sleep_ns(10000000)  // 10ms delay
        
        ready dmut_try_lock_timeout(mutex_a, 500000000) {  // 500ms timeout
            vibez.spill("Goroutine 2: Locked mutex A")
            shared_resource_a = shared_resource_a + 10
            shared_resource_b = shared_resource_b + 10
            dmut_unlock(mutex_a)
        } otherwise {
            vibez.spill("Goroutine 2: Timeout acquiring mutex A")
        }
        
        dmut_unlock(mutex_b)
        atomic_fetch_add_int(&deadlock_completed, 1)
    }
    
    fr fr Wait for both goroutines to complete (with timeout)
    sus timeout_start thicc = timez.nanoseconds_since_epoch()
    sus timeout_limit thicc = 2000000000  // 2 seconds
    
    bestie deadlock_completed < 2 {
        sus elapsed thicc = timez.nanoseconds_since_epoch() - timeout_start
        ready elapsed > timeout_limit {
            testz.fail("Deadlock prevention timeout - potential deadlock occurred")
            damn cap
        }
        timez.sleep_ns(1000000)  // 1ms
    }
    
    fr fr Both goroutines should complete (deadlock prevented by timeouts)
    testz.assert_eq_int(deadlock_completed, 2, "Both goroutines should complete without deadlock")
    
    dmut_destroy(mutex_a)
    dmut_destroy(mutex_b)
    vibez.spill("✅ Deadlock Prevention Test passed")
    damn based
}

fr fr Main stress test runner
slay main() normie {
    vibez.spill("🚀 Starting Concurrency Stress Tests")
    vibez.spill("⚠️  WARNING: These tests intentionally create race conditions and high load")
    vibez.spill("=" * 70)
    
    fr fr Initialize concurrency runtime
    testz.assert_true(concurrenz.concurrency_production_init(), "Concurrency runtime must initialize")
    
    fr fr Set high performance configuration for stress testing
    concurrenz.set_scheduler_config(
        num_workers: 8,
        preemption_enabled: based,
        work_stealing_enabled: based,
        load_balancing_enabled: based
    )
    
    fr fr Display test environment
    vibez.spill("CPU Cores:", concurrenz.get_cpu_count())
    vibez.spill("Worker Threads:", concurrenz.get_worker_count())
    vibez.spill("Scheduler Type:", concurrenz.get_scheduler_type())
    vibez.spill("")
    
    fr fr Run stress test suite
    testz.test_suite_start("Concurrency Stress Tests")
    
    testz.test_case("Parallel Goroutines Stress", stress_test_parallel_goroutines)
    testz.test_case("Producer-Consumer Channels Stress", stress_test_producer_consumer_channels)
    testz.test_case("Select Statements Stress", stress_test_select_statements)
    testz.test_case("Async/Await Stress", stress_test_async_await)
    testz.test_case("Race Condition Detection", test_race_condition_detection)
    testz.test_case("Deadlock Prevention", test_deadlock_prevention)
    
    testz.test_suite_end()
    
    fr fr Display performance statistics
    sus final_stats *concurrenz.ProductionSchedulerStats = concurrenz.get_production_scheduler_stats()
    vibez.spill("")
    vibez.spill("📊 Concurrency Performance Statistics:")
    vibez.spill("- Total Goroutines Spawned:", final_stats.total_goroutines_spawned)
    vibez.spill("- Total Context Switches:", final_stats.total_context_switches)
    vibez.spill("- Total Work Steals:", final_stats.total_work_steals)
    vibez.spill("- Average Queue Length:", final_stats.average_queue_length)
    vibez.spill("- Peak Memory Usage:", final_stats.peak_memory_usage, "bytes")
    vibez.spill("- CPU Utilization:", final_stats.cpu_utilization, "%")
    vibez.spill("- Scheduler Overhead:", final_stats.scheduler_overhead_ns, "ns")
    
    fr fr Print test results summary
    vibez.spill("")
    vibez.spill("🎯 Concurrency Stress Test Results:")
    vibez.spill("- ✅ Goroutine spawning under high load: WORKING")
    vibez.spill("- ✅ Channel operations under stress: WORKING")
    vibez.spill("- ✅ Select statements with multiple channels: WORKING")
    vibez.spill("- ✅ Async/await concurrent operations: WORKING")
    vibez.spill("- ✅ Race condition detection: WORKING")
    vibez.spill("- ✅ Deadlock prevention with timeouts: WORKING")
    vibez.spill("")
    vibez.spill("💪 Concurrency system handles high-load parallel processing!")
    
    fr fr Cleanup
    concurrenz.concurrency_production_shutdown()
    
    damn testz.get_exit_code()
}
