vibe main

yeet "vibez"     fr fr For printing results
yeet "concurrenz" fr fr Synchronization package
yeet "timez"     fr fr For sleeping in goroutines

slay main() {
    vibez.spill("Testing concurrenz package")
    
    fr fr Test Mutex
    test_mutex()
    
    fr fr Test RWMutex
    test_rwmutex()
    
    fr fr Test WaitGroup
    test_waitgroup()
    
    fr fr Test Once
    test_once()
    
    vibez.spill("All concurrenz tests completed successfully")
}

fr fr Test mutual exclusion with Mutex
slay test_mutex() {
    vibez.spill("Testing Mutex...")
    
    sus counter := 0
    sus mu concurrenz.Mutex
    
    fr fr Create 5 goroutines that increment the counter
    sus wg concurrenz.WaitGroup
    wg.Add(5)
    
    bestie i := 0; i < 5; i++ {
        stan slay() {
            later wg.Done()
            
            fr fr Lock before accessing shared data
            mu.Lock()
            later mu.Unlock()  fr fr Unlock using defer
            
            fr fr Critical section
            counter++
            timez.Sleep(10)  fr fr Sleep to increase chance of race condition
        }()
    }
    
    fr fr Wait for all goroutines to finish
    wg.Wait()
    
    fr fr Check that counter equals 5
    vibez.spill("Mutex test - counter value:", counter)
    lowkey counter != 5 {
        vibez.spill("Mutex test failed! Expected counter = 5, got", counter)
    } highkey {
        vibez.spill("Mutex test passed!")
    }
}

fr fr Test read/write locks with RWMutex
slay test_rwmutex() {
    vibez.spill("Testing RWMutex...")
    
    sus data := "original"
    sus rwmu concurrenz.RWMutex
    
    sus wg concurrenz.WaitGroup
    wg.Add(6)  fr fr 5 readers + 1 writer
    
    fr fr Create 5 reader goroutines
    bestie i := 0; i < 5; i++ {
        stan slay() {
            later wg.Done()
            
            fr fr Acquire read lock - multiple readers allowed
            rwmu.RLock()
            later rwmu.RUnlock()
            
            fr fr Read-only access
            tea local_copy := data
            timez.Sleep(10)  fr fr Sleep to simulate reading
            vibez.spill("Reader", i, "read:", local_copy)
        }()
    }
    
    fr fr Create 1 writer goroutine
    stan slay() {
        later wg.Done()
        
        fr fr Sleep to allow readers to start
        timez.Sleep(5)
        
        fr fr Acquire write lock - exclusive access
        rwmu.Lock()
        later rwmu.Unlock()
        
        fr fr Modify data
        vibez.spill("Writer is modifying data")
        data = "modified"
        timez.Sleep(20)  fr fr Sleep to simulate writing
    }()
    
    fr fr Wait for all goroutines to finish
    wg.Wait()
    
    fr fr Check final data value
    vibez.spill("RWMutex test - final data value:", data)
    lowkey data != "modified" {
        vibez.spill("RWMutex test failed! Expected data = 'modified', got", data)
    } highkey {
        vibez.spill("RWMutex test passed!")
    }
}

fr fr Test coordination with WaitGroup
slay test_waitgroup() {
    vibez.spill("Testing WaitGroup...")
    
    sus results := make([]normie, 3)
    sus wg concurrenz.WaitGroup
    
    fr fr Add number of goroutines to wait for
    wg.Add(3)
    
    fr fr Launch 3 goroutines
    bestie i := 0; i < 3; i++ {
        tea idx := i  fr fr Capture loop variable
        stan slay() {
            later wg.Done()
            
            fr fr Simulate work with different durations
            tea sleep_ms := (idx + 1) * 10
            timez.Sleep(thicc(sleep_ms))
            
            fr fr Store result
            results[idx] = idx + 1
            vibez.spill("Goroutine", idx, "completed")
        }()
    }
    
    fr fr Wait for all goroutines to complete
    vibez.spill("Waiting for all goroutines to complete...")
    wg.Wait()
    vibez.spill("All goroutines completed!")
    
    fr fr Verify results
    tea total := 0
    bestie i := 0; i < 3; i++ {
        total += results[i]
    }
    
    fr fr Expected: 1 + 2 + 3 = 6
    vibez.spill("WaitGroup test - sum of results:", total)
    lowkey total != 6 {
        vibez.spill("WaitGroup test failed! Expected sum = 6, got", total)
    } highkey {
        vibez.spill("WaitGroup test passed!")
    }
}

fr fr Test one-time initialization with Once
slay test_once() {
    vibez.spill("Testing Once...")
    
    sus counter := 0
    sus once concurrenz.Once
    
    fr fr Function that should run only once
    slay increment() {
        counter++
        vibez.spill("Increment function executed, counter =", counter)
    }
    
    fr fr Call multiple times from multiple goroutines
    sus wg concurrenz.WaitGroup
    wg.Add(5)
    
    bestie i := 0; i < 5; i++ {
        stan slay() {
            later wg.Done()
            
            vibez.spill("Goroutine", i, "attempting to execute increment")
            once.Do(increment)
            vibez.spill("Goroutine", i, "done")
        }()
    }
    
    fr fr Wait for all goroutines to finish
    wg.Wait()
    
    fr fr Check that counter equals 1 (function ran only once)
    vibez.spill("Once test - counter value:", counter)
    lowkey counter != 1 {
        vibez.spill("Once test failed! Expected counter = 1, got", counter)
    } highkey {
        vibez.spill("Once test passed!")
    }
}