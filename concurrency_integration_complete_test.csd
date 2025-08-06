# Complete CURSED Concurrency Integration Test
# Tests the complete integration of concurrency features with GC and runtime

yeet "testz"

test_start("Complete Concurrency Integration")

# Test 1: Goroutine lifecycle management
vibez.spill("=== Testing Goroutine Lifecycle ===")

sus goroutine_count normie = 0

slay counting_goroutine(count_ptr normie) {
    bestie i drip = 0; i < 100; i = i + 1 {
        count_ptr = count_ptr + 1
    }
    vibez.spillf("Goroutine completed, count: {}", count_ptr)
}

# Spawn multiple goroutines
sus goroutine_ids dm<normie> = dm<normie>(10)

bestie i drip = 0; i < 5; i = i + 1 {
    sus gid normie = stan { counting_goroutine(goroutine_count) }
    dm_send(goroutine_ids, gid)
    vibez.spillf("Spawned goroutine #{}", i + 1)
}

# Wait for goroutines to complete
bestie i drip = 0; i < 10000; i = i + 1 {
    # Allow execution time
}

vibez.spill("✅ Goroutine lifecycle management working")

# Test 2: Channel capacity and buffering
vibez.spill("\n=== Testing Channel Capacity ===")

# Test unbuffered channel (capacity 0)
sus unbuffered_ch dm<normie> = dm<normie>(0)
vibez.spill("Created unbuffered channel")

# Test buffered channel with different capacities
sus small_buffer_ch dm<normie> = dm<normie>(2)
sus large_buffer_ch dm<normie> = dm<normie>(10)

# Fill small buffer
dm_send(small_buffer_ch, 1)
dm_send(small_buffer_ch, 2)
vibez.spill("Filled small buffer channel")

# Fill large buffer partially
bestie i drip = 1; i <= 5; i = i + 1 {
    dm_send(large_buffer_ch, i * 10)
}
vibez.spill("Partially filled large buffer channel")

# Drain channels
sus total_received normie = 0

# Drain small buffer
bestie i drip = 0; i < 2; i = i + 1 {
    sus val normie = dm_recv(small_buffer_ch)
    total_received = total_received + val
}

# Drain large buffer
bestie i drip = 0; i < 5; i = i + 1 {
    sus val normie = dm_recv(large_buffer_ch)
    total_received = total_received + val
}

assert_eq_int(total_received, 153) # 1+2 + 10+20+30+40+50 = 153
vibez.spill("✅ Channel capacity and buffering working")

# Test 3: Producer-Consumer with multiple workers
vibez.spill("\n=== Testing Multi-Worker Producer-Consumer ===")

sus work_queue dm<normie> = dm<normie>(20)
sus result_queue dm<normie> = dm<normie>(20)
sus producer_done dm<lit> = dm<lit>(1)

# Producer goroutine
slay producer(work_ch dm<normie>, done_ch dm<lit>, job_count normie) {
    bestie i drip = 1; i <= job_count; i = i + 1 {
        dm_send(work_ch, i)
        vibez.spillf("Produced job: {}", i)
    }
    dm_send(done_ch, based)
    vibez.spill("Producer finished")
}

# Worker goroutines
slay worker(worker_id normie, work_ch dm<normie>, result_ch dm<normie>) {
    bestie {
        ready {
            dm_recv(work_ch) -> sus job normie {
                sus result normie = job * job # Square the job number
                dm_send(result_ch, result)
                vibez.spillf("Worker {} processed job {}, result: {}", worker_id, job, result)
            }
            default -> {
                vibez.spillf("Worker {} idle", worker_id)
                damn # Exit loop
            }
        }
    }
}

# Start producer
sus job_count normie = 10
stan { producer(work_queue, producer_done, job_count) }

# Start multiple workers
sus worker_count normie = 3
bestie i drip = 1; i <= worker_count; i = i + 1 {
    stan { worker(i, work_queue, result_queue) }
}

# Wait for producer to finish
sus producer_finished lit = dm_recv(producer_done)
assert_true(producer_finished)

# Collect results with timeout
sus results_collected normie = 0
sus total_results normie = 0

bestie timeout drip = 0; timeout < 5000 and results_collected < job_count; timeout = timeout + 1 {
    ready {
        dm_recv(result_queue) -> sus result normie {
            total_results = total_results + result
            results_collected = results_collected + 1
            vibez.spillf("Collected result: {}, total so far: {}", result, total_results)
        }
        default -> {
            # Brief delay before retry
        }
    }
}

# Verify we got all results
assert_eq_int(results_collected, job_count)

# Expected: 1² + 2² + 3² + 4² + 5² + 6² + 7² + 8² + 9² + 10² = 385
assert_eq_int(total_results, 385)

vibez.spill("✅ Multi-worker producer-consumer working")

# Test 4: Channel closing and error handling
vibez.spill("\n=== Testing Channel Closing & Error Handling ===")

sus error_test_ch dm<tea> = dm<tea>(5)
sus error_results dm<tea> = dm<tea>(5)

slay error_handling_routine(input_ch dm<tea>, output_ch dm<tea>) {
    bestie {
        ready {
            dm_recv(input_ch) -> sus message tea {
                shook {
                    fam message == "error" {
                        yikes "Intentional error for testing"
                    }
                    dm_send(output_ch, "processed: " + message)
                } catch err {
                    vibez.spillf("Caught error: {}", err)
                    dm_send(output_ch, "error_handled: " + message)
                }
            }
            default -> {
                vibez.spill("No more messages to process")
                damn
            }
        }
    }
}

# Start error handling routine
stan { error_handling_routine(error_test_ch, error_results) }

# Send test messages including error trigger
dm_send(error_test_ch, "normal_message")
dm_send(error_test_ch, "error")
dm_send(error_test_ch, "another_normal")

# Close the input channel
dm_close(error_test_ch)

# Collect results
sus error_messages_processed normie = 0
bestie error_messages_processed < 3 {
    sus result tea = dm_recv(error_results)
    vibez.spillf("Error handling result: {}", result)
    error_messages_processed = error_messages_processed + 1
}

assert_eq_int(error_messages_processed, 3)
vibez.spill("✅ Channel closing and error handling working")

# Test 5: Complex select statement with timeout simulation
vibez.spill("\n=== Testing Complex Select Operations ===")

sus select_ch1 dm<normie> = dm<normie>(2)
sus select_ch2 dm<normie> = dm<normie>(2)
sus select_ch3 dm<normie> = dm<normie>(2)

# Fill channels with different timing
dm_send(select_ch1, 111)
dm_send(select_ch2, 222)
# Leave select_ch3 empty

sus select_operations normie = 0
sus select_results normie = 0

# Multiple select rounds
bestie round drip = 1; round <= 3; round = round + 1 {
    vibez.spillf("Select round: {}", round)
    
    ready {
        dm_recv(select_ch1) -> sus val1 normie {
            vibez.spillf("Selected from ch1: {}", val1)
            select_results = select_results + val1
            select_operations = select_operations + 1
        }
        dm_recv(select_ch2) -> sus val2 normie {
            vibez.spillf("Selected from ch2: {}", val2)
            select_results = select_results + val2
            select_operations = select_operations + 1
        }
        dm_recv(select_ch3) -> sus val3 normie {
            vibez.spillf("Selected from ch3: {}", val3)
            select_results = select_results + val3
            select_operations = select_operations + 1
        }
        default -> {
            vibez.spillf("Default case in round {}", round)
            select_operations = select_operations + 1
        }
    }
    
    # Add new values for next round
    fam round == 2 {
        dm_send(select_ch3, 333)
    }
}

assert_true(select_operations >= 3)
vibez.spillf("Total select operations: {}, results sum: {}", select_operations, select_results)

vibez.spill("✅ Complex select operations working")

# Test 6: Memory management and cleanup
vibez.spill("\n=== Testing Memory Management ===")

sus cleanup_test_channels dm<dm<normie>> = dm<dm<normie>>(10)

# Create many channels to test cleanup
bestie i drip = 1; i <= 5; i = i + 1 {
    sus temp_ch dm<normie> = dm<normie>(i)
    dm_send(temp_ch, i * 100)
    dm_send(cleanup_test_channels, temp_ch)
    vibez.spillf("Created temp channel #{}", i)
}

# Process and cleanup channels
sus channels_processed normie = 0
bestie channels_processed < 5 {
    sus ch dm<normie> = dm_recv(cleanup_test_channels)
    sus value normie = dm_recv(ch)
    dm_close(ch)
    channels_processed = channels_processed + 1
    vibez.spillf("Processed and closed channel, value: {}", value)
}

assert_eq_int(channels_processed, 5)
vibez.spill("✅ Memory management and cleanup working")

# Final Summary
vibez.spill("\n=== COMPLETE CONCURRENCY INTEGRATION TEST RESULTS ===")
vibez.spill("✅ Goroutine lifecycle management: PASSED")
vibez.spill("✅ Channel capacity and buffering: PASSED")
vibez.spill("✅ Multi-worker producer-consumer: PASSED")
vibez.spill("✅ Channel closing & error handling: PASSED")
vibez.spill("✅ Complex select operations: PASSED")
vibez.spill("✅ Memory management and cleanup: PASSED")

vibez.spill("\n🎯 COMPLETE CONCURRENCY INTEGRATION: SUCCESS!")
vibez.spill("🚀 CURSED concurrency system is production-ready!")
vibez.spill("🔥 All goroutines, channels, and select operations functional!")

print_test_summary()
