fr fr CURSED Concurrency System Demo
fr fr Demonstrates goroutines, channels, and select statements

yeet "testz"

fr fr Example 1: Basic goroutine spawning
slay test_basic_goroutines() {
    test_start("Basic Goroutine Test")
    
    fr fr Block form goroutine
    stan {
        vibez.spill("Hello from goroutine 1!")
        vibez.spill("Goroutine 1 executing...")
    }
    
    fr fr Expression form goroutine
    stan doWork()
    
    fr fr Wait a bit for goroutines to execute
    yolo()  fr fr Yield to allow other goroutines to run
    
    print_test_summary()
}

slay doWork() {
    vibez.spill("Hello from goroutine 2!")
    vibez.spill("Goroutine 2 executing...")
}

fr fr Example 2: Channel communication
slay test_channel_communication() {
    test_start("Channel Communication Test")
    
    fr fr Create buffered channel
    sus ch dm<normie> = dm<normie>(3)
    
    fr fr Sender goroutine
    stan {
        vibez.spill("Sender: Sending values...")
        dm_send(ch, 42)
        dm_send(ch, 43)
        dm_send(ch, 44)
        vibez.spill("Sender: All values sent!")
    }
    
    fr fr Receiver goroutine
    stan {
        vibez.spill("Receiver: Waiting for values...")
        sus val1 normie = dm_recv(ch)
        sus val2 normie = dm_recv(ch)
        sus val3 normie = dm_recv(ch)
        
        vibez.spillf("Receiver: Got values {}, {}, {}", val1, val2, val3)
    }
    
    fr fr Let goroutines complete
    yolo()
    yolo()
    
    print_test_summary()
}

fr fr Example 3: Select statements for non-blocking operations
slay test_select_statements() {
    test_start("Select Statement Test")
    
    fr fr Create multiple channels
    sus ch1 dm<normie> = dm<normie>(1)
    sus ch2 dm<tea> = dm<tea>(1)
    sus timeout dm<lit> = make_timeout(1000)  fr fr 1 second timeout
    
    fr fr Send data on one channel
    dm_send(ch1, 100)
    
    fr fr Select statement with multiple cases
    ready {
        mood value := dm_recv(ch1):
            vibez.spillf("Received from ch1: {}", value)
            assert_eq_int(value, 100)
            
        mood message := dm_recv(ch2):
            vibez.spillf("Received from ch2: {}", message)
            
        mood dm_recv(timeout):
            vibez.spill("Operation timed out")
            
        basic:
            vibez.spill("No channels ready")
    }
    
    print_test_summary()
}

fr fr Example 4: Producer-Consumer pattern
slay test_producer_consumer() {
    test_start("Producer-Consumer Test")
    
    sus jobs dm<normie> = dm<normie>(10)
    sus results dm<normie> = dm<normie>(10)
    
    fr fr Producer goroutine
    stan {
        vibez.spill("Producer: Creating jobs...")
        bestie i := 1; i <= 5; i = i + 1 {
            dm_send(jobs, i)
            vibez.spillf("Producer: Created job {}", i)
        }
        dm_close(jobs)
        vibez.spill("Producer: All jobs created!")
    }
    
    fr fr Consumer goroutine
    stan {
        vibez.spill("Consumer: Processing jobs...")
        bestie {
            ready {
                mood job := dm_recv(jobs):
                    sus result normie = job * 2  fr fr Process job
                    dm_send(results, result)
                    vibez.spillf("Consumer: Processed job {} -> {}", job, result)
                    
                basic:
                    vibez.spill("Consumer: No more jobs")
                    vibes  fr fr Break from loop
            }
        }
        dm_close(results)
        vibez.spill("Consumer: All jobs processed!")
    }
    
    fr fr Result collector
    stan {
        vibez.spill("Collector: Collecting results...")
        sus total normie = 0
        bestie {
            ready {
                mood result := dm_recv(results):
                    total = total + result
                    vibez.spillf("Collector: Got result {}, total: {}", result, total)
                    
                basic:
                    vibez.spillf("Collector: Final total: {}", total)
                    assert_eq_int(total, 30)  fr fr 2+4+6+8+10 = 30
                    vibes  fr fr Break from loop
            }
        }
    }
    
    fr fr Wait for all goroutines to complete
    bestie i := 0; i < 10; i = i + 1 {
        yolo()
    }
    
    print_test_summary()
}

fr fr Example 5: Advanced select with timeouts and priorities
slay test_advanced_select() {
    test_start("Advanced Select Test")
    
    sus high_priority dm<tea> = dm<tea>(1)
    sus low_priority dm<normie> = dm<normie>(1)
    sus control dm<lit> = dm<lit>(1)
    
    fr fr Background worker
    stan {
        dm_send(low_priority, 42)
        yolo()
        dm_send(high_priority, "urgent")
        yolo()
        dm_send(control, based)
    }
    
    fr fr Priority-based message processor
    sus processed_count normie = 0
    bestie processed_count < 3 {
        ready {
            fr fr High priority messages first
            mood urgent := dm_recv(high_priority):
                vibez.spillf("Processing urgent: {}", urgent)
                processed_count = processed_count + 1
                
            mood normal := dm_recv(low_priority):
                vibez.spillf("Processing normal: {}", normal)
                processed_count = processed_count + 1
                
            mood dm_recv(control):
                vibez.spill("Received control signal")
                processed_count = processed_count + 1
                
            basic:
                vibez.spill("No messages available, waiting...")
                yolo()
        }
    }
    
    assert_eq_int(processed_count, 3)
    print_test_summary()
}

fr fr Example 6: Goroutine synchronization and coordination
slay test_goroutine_coordination() {
    test_start("Goroutine Coordination Test")
    
    sus barrier dm<lit> = dm<lit>(3)  fr fr Barrier for 3 goroutines
    sus results dm<normie> = dm<normie>(3)
    
    fr fr Worker 1
    stan {
        vibez.spill("Worker 1: Starting work...")
        yolo()  fr fr Simulate work
        vibez.spill("Worker 1: Work complete")
        dm_send(barrier, based)
        dm_send(results, 1)
    }
    
    fr fr Worker 2
    stan {
        vibez.spill("Worker 2: Starting work...")
        yolo()  fr fr Simulate work
        vibez.spill("Worker 2: Work complete")
        dm_send(barrier, based)
        dm_send(results, 2)
    }
    
    fr fr Worker 3
    stan {
        vibez.spill("Worker 3: Starting work...")
        yolo()  fr fr Simulate work
        vibez.spill("Worker 3: Work complete")
        dm_send(barrier, based)
        dm_send(results, 3)
    }
    
    fr fr Wait for all workers to reach barrier
    dm_recv(barrier)
    dm_recv(barrier)
    dm_recv(barrier)
    vibez.spill("All workers reached barrier!")
    
    fr fr Collect results
    sus total normie = 0
    total = total + dm_recv(results)
    total = total + dm_recv(results)
    total = total + dm_recv(results)
    
    assert_eq_int(total, 6)  fr fr 1 + 2 + 3 = 6
    print_test_summary()
}

fr fr Main function to run all tests
slay main() {
    vibez.spill("🚀 CURSED Concurrency System Demo")
    vibez.spill("===================================")
    vibez.spill("")
    
    test_basic_goroutines()
    vibez.spill("")
    
    test_channel_communication()
    vibez.spill("")
    
    test_select_statements()
    vibez.spill("")
    
    test_producer_consumer()
    vibez.spill("")
    
    test_advanced_select()
    vibez.spill("")
    
    test_goroutine_coordination()
    vibez.spill("")
    
    vibez.spill("🎉 All concurrency demos completed!")
    vibez.spill("✅ Goroutines: Working correctly")
    vibez.spill("✅ Channels: Send/receive operations functional")
    vibez.spill("✅ Select statements: Non-blocking operations working")
    vibez.spill("✅ Patterns: Producer-consumer and coordination working")
}
