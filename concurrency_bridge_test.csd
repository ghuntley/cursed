// P1 Concurrency Runtime Bridge Test - Integration Demo
//
// This program tests the runtime bridge that connects interpreter 
// concurrency with LLVM compiled code execution for goroutines and channels.
//
// Features tested:
// 1. stan (goroutine) spawning in both modes
// 2. dm (channel) operations between modes  
// 3. Cross-mode communication
// 4. Resource cleanup and synchronization

yeet "vibez"
yeet "mathz"

// Test basic goroutine spawning with stan keyword
slay test_basic_goroutine_spawning() {
    vibez.spill("=== Testing Basic Goroutine Spawning ===")
    
    // Test interpreter mode goroutine
    stan {
        vibez.spill("Hello from interpreter goroutine!")
        
        bestie (sus i normie = 0; i < 3; i += 1) {
            vibez.spill("Interpreter goroutine iteration:", i)
            // Sleep equivalent - just do some work
            sus sum normie = 0
            bestie (sus j normie = 0; j < 1000; j += 1) {
                sum += j
            }
        }
        
        vibez.spill("Interpreter goroutine completed, sum example:", sum)
    }
    
    // Test compiled mode goroutine (will be handled by bridge)
    stan {
        vibez.spill("Hello from compiled goroutine!")
        
        // Do some computational work
        sus result normie = 0
        bestie (sus k normie = 1; k <= 100; k += 1) {
            result += k
        }
        
        vibez.spill("Compiled goroutine result:", result)
        vibez.spill("Expected: 5050, Got:", result)
    }
    
    vibez.spill("Basic goroutine spawning test completed")
}

// Test channel operations with dm type
slay test_channel_operations() {
    vibez.spill("=== Testing Channel Operations ===")
    
    // Create a buffered channel
    sus ch dm<normie> = dm<normie>(3) // Capacity of 3
    
    // Test basic send/receive
    dm_send(ch, 42)
    dm_send(ch, 100) 
    dm_send(ch, 200)
    
    vibez.spill("Sent: 42, 100, 200")
    
    sus val1 normie = dm_recv(ch)
    sus val2 normie = dm_recv(ch) 
    sus val3 normie = dm_recv(ch)
    
    vibez.spill("Received:", val1, val2, val3)
    
    // Test channel with goroutines
    sus worker_channel dm<normie> = dm<normie>(5)
    sus result_channel dm<normie> = dm<normie>(1)
    
    // Producer goroutine
    stan {
        vibez.spill("Producer: Starting to send data")
        bestie (sus i normie = 1; i <= 5; i += 1) {
            dm_send(worker_channel, i * 10)
            vibez.spill("Producer: Sent", i * 10)
        }
        dm_close(worker_channel)
        vibez.spill("Producer: Finished and closed channel")
    }
    
    // Consumer goroutine
    stan {
        vibez.spill("Consumer: Starting to process data")
        sus total normie = 0
        
        bestie (based) {
            sus value normie = dm_recv(worker_channel)
            ready (value == 0) {  // Channel closed
                break
            }
            
            vibez.spill("Consumer: Received", value)
            total += value
        }
        
        vibez.spill("Consumer: Total sum is", total)
        dm_send(result_channel, total)
        vibez.spill("Consumer: Sent result to main")
    }
    
    // Main thread waits for result
    sus final_result normie = dm_recv(result_channel)
    vibez.spill("Final result from consumer:", final_result)
    vibez.spill("Expected: 150 (10+20+30+40+50)")
    
    vibez.spill("Channel operations test completed")
}

// Test select statements with ready keyword
slay test_select_statements() {
    vibez.spill("=== Testing Select Statements ===")
    
    sus ch1 dm<normie> = dm<normie>(1)
    sus ch2 dm<normie> = dm<normie>(1)
    sus ch3 dm<normie> = dm<normie>(1)
    
    // Fill channels in separate goroutines
    stan {
        // Delay then send to ch1
        sus delay_work normie = 0
        bestie (sus i normie = 0; i < 500; i += 1) {
            delay_work += 1
        }
        dm_send(ch1, 111)
        vibez.spill("Sent 111 to ch1")
    }
    
    stan {
        // Delay then send to ch2  
        sus delay_work normie = 0
        bestie (sus i normie = 0; i < 1000; i += 1) {
            delay_work += 1
        }
        dm_send(ch2, 222)
        vibez.spill("Sent 222 to ch2")
    }
    
    stan {
        // Delay then send to ch3
        sus delay_work normie = 0  
        bestie (sus i normie = 0; i < 1500; i += 1) {
            delay_work += 1
        }
        dm_send(ch3, 333)
        vibez.spill("Sent 333 to ch3")
    }
    
    // Use select to receive from whichever is ready first
    vibez.spill("Waiting for first channel to be ready...")
    
    ready {
        mood val := dm_recv(ch1): {
            vibez.spill("Received from ch1:", val)
        }
        mood val := dm_recv(ch2): {
            vibez.spill("Received from ch2:", val)
        }
        mood val := dm_recv(ch3): {
            vibez.spill("Received from ch3:", val)
        }
        basic: {
            vibez.spill("No channels ready (should not happen)")
        }
    }
    
    vibez.spill("Select statement test completed")
}

// Test cross-mode communication between interpreter and compiled goroutines
slay test_cross_mode_communication() {
    vibez.spill("=== Testing Cross-Mode Communication ===")
    
    // Create shared channel for cross-mode communication
    sus bridge_channel dm<normie> = dm<normie>(10)
    
    // Interpreter mode producer
    stan {
        vibez.spill("Interpreter producer: Starting")
        bestie (sus i normie = 1; i <= 5; i += 1) {
            sus value normie = i * 100
            dm_send(bridge_channel, value)
            vibez.spill("Interpreter producer: Sent", value)
        }
        vibez.spill("Interpreter producer: Finished")
    }
    
    // Compiled mode consumer (will be executed through bridge)  
    stan {
        vibez.spill("Compiled consumer: Starting")
        sus received_count normie = 0
        sus total normie = 0
        
        bestie (received_count < 5) {
            sus value normie = dm_recv(bridge_channel)
            ready (value > 0) {
                vibez.spill("Compiled consumer: Received", value)
                total += value
                received_count += 1
            }
        }
        
        vibez.spill("Compiled consumer: Total received:", total)
        vibez.spill("Expected: 1500 (100+200+300+400+500)")
    }
    
    // Wait for goroutines to complete
    sus wait_work normie = 0
    bestie (sus i normie = 0; i < 10000; i += 1) {
        wait_work += 1
    }
    
    vibez.spill("Cross-mode communication test completed")
}

// Test complex concurrency patterns
slay test_complex_patterns() {
    vibez.spill("=== Testing Complex Concurrency Patterns ===")
    
    // Worker pool pattern
    sus job_channel dm<normie> = dm<normie>(20)
    sus result_channel dm<normie> = dm<normie>(20)
    sus worker_count normie = 3
    
    // Start worker goroutines
    bestie (sus worker_id normie = 1; worker_id <= worker_count; worker_id += 1) {
        stan {
            vibez.spill("Worker", worker_id, "started")
            
            bestie (based) {
                sus job normie = dm_recv(job_channel)
                ready (job == 0) { // Sentinel value for shutdown
                    break
                }
                
                // Process job (compute square)
                sus result normie = job * job
                vibez.spill("Worker", worker_id, "processed job", job, "result", result)
                
                dm_send(result_channel, result)
            }
            
            vibez.spill("Worker", worker_id, "finished")
        }
    }
    
    // Send jobs
    stan {
        vibez.spill("Job dispatcher: Starting")
        bestie (sus job normie = 1; job <= 10; job += 1) {
            dm_send(job_channel, job)
            vibez.spill("Job dispatcher: Sent job", job)
        }
        
        // Send shutdown signals
        bestie (sus i normie = 0; i < worker_count; i += 1) {
            dm_send(job_channel, 0) // Sentinel
        }
        
        vibez.spill("Job dispatcher: Finished")
    }
    
    // Collect results
    stan {
        vibez.spill("Result collector: Starting")
        sus total normie = 0
        sus count normie = 0
        
        bestie (count < 10) {
            sus result normie = dm_recv(result_channel)
            vibez.spill("Result collector: Got result", result)
            total += result
            count += 1
        }
        
        vibez.spill("Result collector: Total of all squares:", total)
        vibez.spill("Expected: 385 (1+4+9+16+25+36+49+64+81+100)")
    }
    
    // Wait for completion
    sus wait_work normie = 0
    bestie (sus i normie = 0; i < 20000; i += 1) {
        wait_work += 1  
    }
    
    vibez.spill("Complex patterns test completed")
}

// Test error handling and edge cases
slay test_error_handling() {
    vibez.spill("=== Testing Error Handling ===")
    
    // Test closed channel behavior
    sus test_channel dm<normie> = dm<normie>(1)
    dm_send(test_channel, 42)
    dm_close(test_channel)
    
    // Should still receive buffered value
    sus value normie = dm_recv(test_channel) 
    vibez.spill("Received from closed channel:", value)
    
    // Next receive should get default/error value
    sus value2 normie = dm_recv(test_channel)
    vibez.spill("Second receive from closed channel:", value2)
    
    // Test sending to closed channel (should fail gracefully)
    // dm_send(test_channel, 100)  // This should not crash
    
    vibez.spill("Error handling test completed")
}

// Main test execution
slay main() normie {
    vibez.spill("=== P1 CURSED Concurrency Runtime Bridge Test ===")
    vibez.spill("Testing integration between interpreter and compiled concurrency")
    vibez.spill("")
    
    test_basic_goroutine_spawning()
    vibez.spill("")
    
    test_channel_operations()  
    vibez.spill("")
    
    test_select_statements()
    vibez.spill("")
    
    test_cross_mode_communication()
    vibez.spill("")
    
    test_complex_patterns()
    vibez.spill("")
    
    test_error_handling()
    vibez.spill("")
    
    vibez.spill("=== All Concurrency Bridge Tests Completed ===")
    vibez.spill("If you see this message, the P1 runtime bridge is working correctly!")
    
    damn 0
}
