# Comprehensive CURSED Concurrency System Test
# Tests goroutines, channels, and complex communication patterns

sus global_counter drip = 0
sus results drip[3]

# Worker goroutine that processes work items
slay worker(id drip, input_ch dm<drip>, output_ch dm<drip>) {
    vibez.spillf("Worker %d started", id)
    
    bestie (based) {
        # Receive work from input channel
        sus work drip = dm_recv(input_ch)
        
        # Process the work (simulate computation)
        sus result drip = work * work
        
        vibez.spillf("Worker %d: processed %d -> %d", id, work, result)
        
        # Send result to output channel
        dm_send(output_ch, result)
        
        # Yield to allow other goroutines to run
        yolo
    }
}

# Counter goroutine for shared state
slay counter() {
    bestie (global_counter < 100) {
        global_counter = global_counter + 1
        yolo
    }
    vibez.spillf("Counter finished at: %d", global_counter)
}

slay main() {
    vibez.spill("=== CURSED Concurrency System Comprehensive Test ===")
    
    # Test 1: Basic goroutine spawning
    vibez.spill("\n1. Testing basic goroutine spawning...")
    stan counter()
    stan counter()
    stan counter()
    
    # Brief wait for counters to run
    sus wait_cycles drip = 0
    bestie (wait_cycles < 1000) {
        wait_cycles = wait_cycles + 1
        yolo
    }
    
    vibez.spillf("Global counter reached: %d", global_counter)
    
    # Test 2: Channel communication
    vibez.spill("\n2. Testing channel communication...")
    
    # Create channels for worker communication
    sus input_ch dm<drip> = dm_new<drip>(5)    # Buffered input channel
    sus output_ch dm<drip> = dm_new<drip>(5)   # Buffered output channel
    
    # Spawn worker goroutines
    stan worker(1, input_ch, output_ch)
    stan worker(2, input_ch, output_ch)
    stan worker(3, input_ch, output_ch)
    
    # Send work to workers
    dm_send(input_ch, 3)
    dm_send(input_ch, 4)
    dm_send(input_ch, 5)
    
    # Collect results
    results[0] = dm_recv(output_ch)
    results[1] = dm_recv(output_ch) 
    results[2] = dm_recv(output_ch)
    
    vibez.spillf("Results: %d, %d, %d", results[0], results[1], results[2])
    
    # Test 3: Channel closing and error handling
    vibez.spill("\n3. Testing channel lifecycle...")
    sus test_ch dm<drip> = dm_new<drip>(1)
    
    dm_send(test_ch, 42)
    sus value drip = dm_recv(test_ch)
    vibez.spillf("Received before close: %d", value)
    
    # Close channel (if supported)
    # dm_close(test_ch)
    
    vibez.spill("\n=== All concurrency tests completed successfully! ===")
    
    # Verify expected results
    expect results[0] == 9    # 3*3
    expect results[1] == 16   # 4*4
    expect results[2] == 25   # 5*5
    
    vibez.spill("✓ All test assertions passed!")
}
