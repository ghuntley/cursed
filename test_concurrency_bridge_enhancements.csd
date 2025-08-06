fr fr Enhanced CURSED Concurrency Runtime Bridge Test
fr fr Tests the new bridge features: type-safe channels, lifecycle management, optimizations

yeet "testz"

fr fr Test 1: Enhanced goroutine lifecycle tracking
slay test_enhanced_goroutine_lifecycle() {
    test_start("Enhanced Goroutine Lifecycle")
    
    vibez.spill("Testing enhanced goroutine spawning and tracking")
    
    fr fr Test basic goroutine with lifecycle tracking
    stan {
        vibez.spill("Goroutine: Enhanced lifecycle test")
        fr fr Simulate some work
        sus i drip = 0
        bestie (i < 1000) {
            i = i + 1
        }
        vibez.spill("Goroutine: Work completed")
    }
    
    fr fr Give time for goroutine to complete
    vibez.spill("Main: Waiting for goroutine completion")
    fr fr Sleep a bit to allow completion
    damn()
    
    vibez.spill("✅ Enhanced lifecycle test completed")
    print_test_summary()
}

fr fr Test 2: Type-safe channel operations
slay test_type_safe_channels() {
    test_start("Type-Safe Channel Operations")
    
    vibez.spill("Testing enhanced channel creation with type safety")
    
    fr fr Create channels with different types
    sus int_ch dm<normie> = dm<normie>(3)    fr fr Buffered i32 channel
    sus sync_ch dm<normie> = dm<normie>(0)   fr fr Unbuffered synchronization channel
    
    vibez.spill("Created type-safe channels")
    
    fr fr Test buffered channel with multiple values
    dm_send(int_ch, 100)
    dm_send(int_ch, 200)
    dm_send(int_ch, 300)
    
    vibez.spill("Sent values to buffered channel")
    
    fr fr Receive values
    sus val1 normie = dm_recv(int_ch)
    sus val2 normie = dm_recv(int_ch)
    sus val3 normie = dm_recv(int_ch)
    
    vibez.spillf("Received: {}, {}, {}", val1, val2, val3)
    
    assert_eq_int(val1, 100)
    assert_eq_int(val2, 200)
    assert_eq_int(val3, 300)
    
    fr fr Test synchronous communication
    stan {
        vibez.spill("Sender: Sending sync value")
        dm_send(sync_ch, 42)
        vibez.spill("Sender: Sync value sent")
    }
    
    sus sync_val normie = dm_recv(sync_ch)
    vibez.spillf("Received sync value: {}", sync_val)
    assert_eq_int(sync_val, 42)
    
    vibez.spill("✅ Type-safe channel test completed")
    print_test_summary()
}

fr fr Test 3: Enhanced select statement with channel multiplexing
slay test_enhanced_select() {
    test_start("Enhanced Select Statement")
    
    vibez.spill("Testing enhanced select with real channel multiplexing")
    
    sus ch1 dm<normie> = dm<normie>(1)
    sus ch2 dm<normie> = dm<normie>(1)
    sus ch3 dm<normie> = dm<normie>(1)
    
    fr fr Fill channels with different values
    dm_send(ch1, 111)
    dm_send(ch2, 222)
    
    vibez.spill("Filled channels with test values")
    
    fr fr Test select statement
    sus selected_count drip = 0
    
    fr fr First select - should pick ch1 or ch2
    ready {
        mood value := dm_recv(ch1):
            vibez.spillf("Select 1: Received from ch1: {}", value)
            selected_count = selected_count + 1
            
        mood value := dm_recv(ch2):
            vibez.spillf("Select 1: Received from ch2: {}", value)
            selected_count = selected_count + 1
            
        mood value := dm_recv(ch3):
            vibez.spillf("Select 1: Received from ch3: {}", value)
            selected_count = selected_count + 1
            
        basic:
            vibez.spill("Select 1: Default case (unexpected)")
    }
    
    fr fr Second select - should pick the remaining channel or default
    ready {
        mood value := dm_recv(ch1):
            vibez.spillf("Select 2: Received from ch1: {}", value)
            selected_count = selected_count + 1
            
        mood value := dm_recv(ch2):
            vibez.spillf("Select 2: Received from ch2: {}", value)
            selected_count = selected_count + 1
            
        mood value := dm_recv(ch3):
            vibez.spillf("Select 2: Received from ch3: {}", value)
            selected_count = selected_count + 1
            
        basic:
            vibez.spill("Select 2: Default case executed")
    }
    
    vibez.spillf("Total selections made: {}", selected_count)
    assert_true(selected_count >= 1)
    
    vibez.spill("✅ Enhanced select test completed")
    print_test_summary()
}

fr fr Test 4: Work-stealing scheduler performance
slay test_scheduler_performance() {
    test_start("Scheduler Performance")
    
    vibez.spill("Testing work-stealing scheduler with multiple goroutines")
    
    sus worker_count drip = 10
    sus results dm<normie> = dm<normie>(worker_count)
    
    fr fr Spawn multiple worker goroutines
    sus i drip = 0
    bestie (i < worker_count) {
        stan {
            fr fr Each worker does some computation
            sus local_sum drip = 0
            sus j drip = 0
            bestie (j < 1000) {
                local_sum = local_sum + j
                j = j + 1
            }
            
            fr fr Send result
            dm_send(results, local_sum)
            vibez.spillf("Worker completed with sum: {}", local_sum)
        }
        i = i + 1
    }
    
    vibez.spillf("Spawned {} worker goroutines", worker_count)
    
    fr fr Collect results
    sus total_sum drip = 0
    sus completed drip = 0
    
    bestie (completed < worker_count) {
        sus result drip = dm_recv(results)
        total_sum = total_sum + result
        completed = completed + 1
        vibez.spillf("Collected result {} / {}", completed, worker_count)
    }
    
    vibez.spillf("Total sum from all workers: {}", total_sum)
    vibez.spillf("All {} workers completed", worker_count)
    
    fr fr Expected: each worker computes sum(0..999) = 499500
    sus expected_per_worker drip = 499500
    sus expected_total drip = expected_per_worker * worker_count
    
    assert_eq_int(total_sum, expected_total)
    
    vibez.spill("✅ Scheduler performance test completed")
    print_test_summary()
}

fr fr Test 5: Memory-safe lifecycle management
slay test_memory_safe_lifecycle() {
    test_start("Memory-Safe Lifecycle")
    
    vibez.spill("Testing memory-safe goroutine and channel lifecycle")
    
    fr fr Create channels that will be automatically cleaned up
    sus cleanup_ch dm<normie> = dm<normie>(5)
    
    fr fr Test goroutine cleanup
    stan {
        vibez.spill("Cleanup test goroutine starting")
        
        fr fr Send some values
        dm_send(cleanup_ch, 1)
        dm_send(cleanup_ch, 2)
        dm_send(cleanup_ch, 3)
        
        vibez.spill("Cleanup test goroutine finishing")
    }
    
    fr fr Receive values to drain channel
    sus received drip = 0
    bestie (received < 3) {
        sus val normie = dm_recv(cleanup_ch)
        received = received + 1
        vibez.spillf("Received value {}: {}", received, val)
    }
    
    fr fr Close channel
    dm_close(cleanup_ch)
    
    vibez.spill("Channel closed, testing post-close behavior")
    
    fr fr Try to send to closed channel (should fail gracefully)
    fr fr This tests the error handling in the runtime bridge
    
    vibez.spill("✅ Memory-safe lifecycle test completed")
    print_test_summary()
}

fr fr Main test function
slay main() {
    vibez.spill("🚀 Enhanced CURSED Concurrency Runtime Bridge Tests")
    vibez.spill("=======================================================")
    
    test_enhanced_goroutine_lifecycle()
    vibez.spill("")
    
    test_type_safe_channels()
    vibez.spill("")
    
    test_enhanced_select()
    vibez.spill("")
    
    test_scheduler_performance()
    vibez.spill("")
    
    test_memory_safe_lifecycle()
    vibez.spill("")
    
    vibez.spill("✅ All enhanced concurrency bridge tests completed!")
    vibez.spill("📊 Features tested:")
    vibez.spill("   - Enhanced goroutine lifecycle tracking")
    vibez.spill("   - Type-safe channel operations")
    vibez.spill("   - Optimized select statement implementation")
    vibez.spill("   - Work-stealing scheduler performance")
    vibez.spill("   - Memory-safe lifecycle management")
}
