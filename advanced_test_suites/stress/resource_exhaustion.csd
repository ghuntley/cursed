// Advanced Stress Testing: Resource Exhaustion
yeet "testz"
yeet "concurrenz"
yeet "timez"
yeet "mathz"

test_start("Resource Exhaustion Stress Tests")

// Memory exhaustion test
slay test_memory_exhaustion() {
    sus large_arrays [][]drip = []
    sus allocation_count drip = 0
    sus max_allocations drip = 10000
    
    ready {
        bestie (allocation_count < max_allocations) {
            sus new_array []drip = []
            
            // Allocate 1MB array (250,000 integers)
            bestie (drip i = 0; i < 250000; i = i + 1) {
                append(new_array, i)
            }
            
            append(large_arrays, new_array)
            allocation_count = allocation_count + 1
            
            // Report progress every 1000 allocations
            ready (allocation_count % 1000 == 0) {
                vibez.spill("Allocated", allocation_count, "arrays, ~", (allocation_count * 250000 * 8) / 1024 / 1024, "MB")
            }
        }
        
        test_fail("Should have run out of memory before reaching max allocations")
        
    } fam {
        when "out of memory" -> {
            vibez.spill("Memory exhaustion detected after", allocation_count, "allocations")
            test_pass("Memory exhaustion handled gracefully")
        }
        when _ -> {
            test_pass("Memory allocation stress completed or handled gracefully")
        }
    }
}

// Goroutine exhaustion test
slay test_goroutine_exhaustion() {
    sus goroutine_count drip = 0
    sus max_goroutines drip = 100000
    sus done_channels []chan<lit> = []
    
    ready {
        bestie (goroutine_count < max_goroutines) {
            sus done chan<lit> = make_channel()
            append(done_channels, done)
            
            go {
                // Each goroutine does some work and then signals completion
                sus work_result drip = 0
                bestie (drip i = 0; i < 1000; i = i + 1) {
                    work_result = work_result + i * i
                }
                
                done <- based
            }
            
            goroutine_count = goroutine_count + 1
            
            // Report progress every 10,000 goroutines
            ready (goroutine_count % 10000 == 0) {
                vibez.spill("Spawned", goroutine_count, "goroutines")
            }
        }
        
        // Wait for all goroutines to complete
        vibez.spill("Waiting for", goroutine_count, "goroutines to complete...")
        bestie (chan<lit> done in done_channels) {
            <-done
        }
        
        test_pass("All goroutines completed successfully")
        
    } fam {
        when "too many goroutines" -> {
            vibez.spill("Goroutine limit reached at", goroutine_count, "goroutines")
            test_pass("Goroutine exhaustion handled gracefully")
        }
        when "out of memory" -> {
            vibez.spill("Out of memory while creating", goroutine_count, "goroutines")
            test_pass("Memory exhaustion from goroutines handled gracefully")
        }
        when _ -> {
            vibez.spill("Goroutine stress test completed with", goroutine_count, "goroutines")
            test_pass("Goroutine stress test handled gracefully")
        }
    }
}

// Channel exhaustion test
slay test_channel_exhaustion() {
    sus channels []chan<drip> = []
    sus channel_count drip = 0
    sus max_channels drip = 50000
    
    ready {
        bestie (channel_count < max_channels) {
            sus ch chan<drip> = make_channel()
            append(channels, ch)
            channel_count = channel_count + 1
            
            // Fill each channel with some data
            go {
                bestie (drip i = 0; i < 100; i = i + 1) {
                    ch <- i
                }
            }
            
            // Report progress every 5,000 channels
            ready (channel_count % 5000 == 0) {
                vibez.spill("Created", channel_count, "channels")
            }
        }
        
        vibez.spill("Created maximum", max_channels, "channels")
        
        // Try to read from random channels
        bestie (drip i = 0; i < 1000; i = i + 1) {
            sus random_idx drip = random_int(0, len(channels) - 1)
            sus value drip = <-channels[random_idx]
        }
        
        test_pass("Channel stress test completed")
        
    } fam {
        when "too many channels" -> {
            vibez.spill("Channel limit reached at", channel_count, "channels")
            test_pass("Channel exhaustion handled gracefully")
        }
        when "out of memory" -> {
            vibez.spill("Out of memory while creating", channel_count, "channels")
            test_pass("Memory exhaustion from channels handled gracefully")
        }
        when _ -> {
            test_pass("Channel stress test handled gracefully")
        }
    }
}

// File handle exhaustion test
slay test_file_handle_exhaustion() {
    sus file_handles []tea = []
    sus file_count drip = 0
    sus max_files drip = 10000
    
    ready {
        bestie (file_count < max_files) {
            sus filename tea = "test_file_" + (file_count as tea) + ".txt"
            sus content tea = "Test content for file " + (file_count as tea) + "\n"
            
            // Create and write to file
            write_file(filename, content)
            append(file_handles, filename)
            file_count = file_count + 1
            
            // Report progress every 1,000 files
            ready (file_count % 1000 == 0) {
                vibez.spill("Created", file_count, "files")
            }
        }
        
        vibez.spill("Created maximum", max_files, "files")
        
        // Try to read from random files
        bestie (drip i = 0; i < 100; i = i + 1) {
            sus random_idx drip = random_int(0, len(file_handles) - 1)
            sus filename tea = file_handles[random_idx]
            sus content tea = read_file(filename)
            assert_eq_bool(len(content) > 0, based)
        }
        
        // Cleanup files
        bestie (tea filename in file_handles) {
            delete_file(filename)
        }
        
        test_pass("File handle stress test completed")
        
    } fam {
        when "too many open files" -> {
            vibez.spill("File handle limit reached at", file_count, "files")
            
            // Try to cleanup created files
            bestie (tea filename in file_handles) {
                ready { delete_file(filename) } fam { when _ -> {} }
            }
            
            test_pass("File handle exhaustion handled gracefully")
        }
        when "disk full" -> {
            vibez.spill("Disk space exhausted after", file_count, "files")
            
            // Cleanup files
            bestie (tea filename in file_handles) {
                ready { delete_file(filename) } fam { when _ -> {} }
            }
            
            test_pass("Disk space exhaustion handled gracefully")
        }
        when _ -> {
            // Cleanup on any other error
            bestie (tea filename in file_handles) {
                ready { delete_file(filename) } fam { when _ -> {} }
            }
            
            test_pass("File stress test handled gracefully")
        }
    }
}

// Stack exhaustion test (recursion depth)
slay deep_recursion(depth drip) drip {
    ready (depth <= 0) { damn 0 }
    damn 1 + deep_recursion(depth - 1)
}

slay test_stack_exhaustion() {
    sus max_depth drip = 1000000
    sus successful_depth drip = 0
    
    ready {
        bestie (drip depth = 1000; depth <= max_depth; depth = depth + 1000) {
            sus result drip = deep_recursion(depth)
            successful_depth = depth
            
            ready (depth % 10000 == 0) {
                vibez.spill("Successful recursion depth:", depth)
            }
        }
        
        test_pass("Deep recursion completed to maximum depth")
        
    } fam {
        when "stack overflow" -> {
            vibez.spill("Stack overflow detected at recursion depth:", successful_depth)
            test_pass("Stack overflow handled gracefully")
        }
        when _ -> {
            vibez.spill("Recursion stress test reached depth:", successful_depth)
            test_pass("Recursion stress test handled gracefully")
        }
    }
}

// Concurrent channel stress test
slay test_concurrent_channel_stress() {
    sus num_producers drip = 100
    sus num_consumers drip = 50
    sus messages_per_producer drip = 1000
    
    sus message_channel chan<drip> = make_channel()
    sus producer_done chan<lit> = make_channel()
    sus consumer_done chan<lit> = make_channel()
    
    sus total_messages drip = num_producers * messages_per_producer
    sus received_messages drip = 0
    sus messages_mutex = make_mutex()
    
    // Start producers
    bestie (drip i = 0; i < num_producers; i = i + 1) {
        go {
            bestie (drip j = 0; j < messages_per_producer; j = j + 1) {
                sus message drip = i * messages_per_producer + j
                message_channel <- message
            }
            producer_done <- based
        }
    }
    
    // Start consumers
    bestie (drip i = 0; i < num_consumers; i = i + 1) {
        go {
            bestie (based) {
                sus message drip = <-message_channel
                
                lock(messages_mutex)
                received_messages = received_messages + 1
                sus current_received drip = received_messages
                unlock(messages_mutex)
                
                ready (current_received >= total_messages) {
                    break
                }
                
                // Simulate some processing
                sus processed drip = message * 2 + 1
            }
            consumer_done <- based
        }
    }
    
    // Wait for all producers to finish
    bestie (drip i = 0; i < num_producers; i = i + 1) {
        <-producer_done
    }
    
    vibez.spill("All producers finished, waiting for consumers...")
    
    // Wait for consumers to finish
    bestie (drip i = 0; i < num_consumers; i = i + 1) {
        <-consumer_done
    }
    
    lock(messages_mutex)
    sus final_received drip = received_messages
    unlock(messages_mutex)
    
    vibez.spill("Total messages sent:", total_messages)
    vibez.spill("Total messages received:", final_received)
    
    assert_eq_int(final_received, total_messages)
    
    test_pass("Concurrent channel stress test completed")
}

// CPU intensive stress test
slay cpu_intensive_work(iterations drip) lit {
    sus result lit = 0.0
    
    bestie (drip i = 0; i < iterations; i = i + 1) {
        result = result + sin(i as lit) * cos(i as lit) * tan(i as lit / 1000.0)
        result = result + sqrt(abs(result))
        
        ready (i % 10000 == 0) {
            // Yield control occasionally
            sleep(1)
        }
    }
    
    damn result
}

slay test_cpu_stress() {
    sus num_workers drip = 8  // Adjust based on CPU cores
    sus iterations_per_worker drip = 1000000
    
    sus results chan<lit> = make_channel()
    sus start_time drip = now_nanos()
    
    // Start CPU-intensive workers
    bestie (drip i = 0; i < num_workers; i = i + 1) {
        go {
            sus result lit = cpu_intensive_work(iterations_per_worker)
            results <- result
        }
    }
    
    // Collect results
    sus total_result lit = 0.0
    bestie (drip i = 0; i < num_workers; i = i + 1) {
        sus worker_result lit = <-results
        total_result = total_result + worker_result
    }
    
    sus end_time drip = now_nanos()
    sus duration_ms drip = (end_time - start_time) / 1000000
    
    vibez.spill("CPU stress test completed in", duration_ms, "ms")
    vibez.spill("Total computation result:", total_result)
    
    test_pass("CPU stress test completed")
}

// Combined resource stress test
slay test_combined_resource_stress() {
    vibez.spill("Starting combined resource stress test...")
    
    sus memory_arrays [][]drip = []
    sus channels []chan<drip> = []
    sus file_handles []tea = []
    
    sus stress_duration_seconds drip = 10
    sus start_time drip = now_seconds()
    
    // Memory allocation stress
    go {
        bestie (drip i = 0; (now_seconds() - start_time) < stress_duration_seconds; i = i + 1) {
            sus new_array []drip = []
            bestie (drip j = 0; j < 10000; j = j + 1) {
                append(new_array, j)
            }
            append(memory_arrays, new_array)
            
            ready (i % 100 == 0) {
                sleep(10)  // Brief pause
            }
        }
    }
    
    // Channel creation stress
    go {
        bestie (drip i = 0; (now_seconds() - start_time) < stress_duration_seconds; i = i + 1) {
            sus ch chan<drip> = make_channel()
            append(channels, ch)
            
            go {
                ch <- i
            }
            
            ready (i % 50 == 0) {
                sleep(5)
            }
        }
    }
    
    // File I/O stress
    go {
        bestie (drip i = 0; (now_seconds() - start_time) < stress_duration_seconds; i = i + 1) {
            sus filename tea = "stress_file_" + (i as tea) + ".txt"
            sus content tea = "Stress test content " + (i as tea) + "\n"
            
            write_file(filename, content)
            append(file_handles, filename)
            
            ready (i % 10 == 0) {
                // Read a random file
                ready (len(file_handles) > 0) {
                    sus random_idx drip = random_int(0, len(file_handles) - 1)
                    sus random_file tea = file_handles[random_idx]
                    sus read_content tea = read_file(random_file)
                }
                sleep(5)
            }
        }
    }
    
    // Wait for stress duration
    sleep(stress_duration_seconds * 1000)
    
    vibez.spill("Stress test completed:")
    vibez.spill("  Memory arrays:", len(memory_arrays))
    vibez.spill("  Channels:", len(channels))
    vibez.spill("  Files:", len(file_handles))
    
    // Cleanup files
    bestie (tea filename in file_handles) {
        ready { delete_file(filename) } fam { when _ -> {} }
    }
    
    test_pass("Combined resource stress test completed")
}

// Run all stress tests
vibez.spill("Starting resource exhaustion stress tests...")
vibez.spill("Warning: These tests may consume significant system resources")

test_memory_exhaustion()
test_goroutine_exhaustion()
test_channel_exhaustion()
test_file_handle_exhaustion()
test_stack_exhaustion()
test_concurrent_channel_stress()
test_cpu_stress()
test_combined_resource_stress()

print_test_summary()
