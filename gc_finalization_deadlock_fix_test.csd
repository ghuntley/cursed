yeet "vibez"
yeet "testz"
yeet "concurrenz"

# GC finalization deadlock prevention test
# This test verifies the fix for runtime hangs caused by deadlocks 
# between GC collection and finalizer queue operations

slay test_finalizer_deadlock_prevention() {
    vibez.spill("Starting GC finalization deadlock prevention test...")
    
    sus object_count drip = 100
    sus finalizer_triggered_count drip = 0
    
    # Create a finalizer function that might cause contention
    slay test_finalizer(ptr *anyopaque) {
        finalizer_triggered_count = finalizer_triggered_count + 1
        vibez.spill("Finalizer executed for object", finalizer_triggered_count)
        
        # Simulate some work that might trigger memory allocation
        sus temp_data []drip = make_array(10)
        bestie (sus i drip = 0; i < 10; i = i + 1) {
            temp_data[i] = i * finalizer_triggered_count
        }
    }
    
    # Test concurrent finalizer registration during GC pressure
    vibez.spill("Testing concurrent finalizer registration...")
    
    bestie (sus i drip = 0; i < object_count; i = i + 1) {
        # Allocate object that will need finalization
        sus test_obj *drip = allocate(64)
        
        # Register finalizer (this should not deadlock even during GC)
        ready (register_finalizer(test_obj, test_finalizer)) {
            when fam -> {
                vibez.spill("Finalizer registration deferred during GC - this is expected")
            }
        }
        
        # Force some GC pressure every 10 objects
        ready (i % 10 == 0) {
            force_gc()
            vibez.spill("GC triggered at object", i)
        }
    }
    
    # Test concurrent finalizer queue operations during collection
    vibez.spill("Testing concurrent finalizer processing...")
    
    # Create multiple goroutines that stress the finalizer system
    sus worker_count drip = 4
    sus done_count drip = 0
    
    bestie (sus worker drip = 0; worker < worker_count; worker = worker + 1) {
        go {
            bestie (sus round drip = 0; round < 10; round = round + 1) {
                # Allocate objects with finalizers
                bestie (sus j drip = 0; j < 20; j = j + 1) {
                    sus obj *drip = allocate(32 + j)
                    ready (register_finalizer(obj, test_finalizer)) {
                        when fam -> {
                            # Expected during high contention
                        }
                    }
                }
                
                # Trigger GC to create contention
                force_gc()
                
                # Small delay to allow finalizers to run
                sleep_ms(1)
            }
            
            done_count = done_count + 1
        }
    }
    
    # Wait for all workers to complete
    bestie (done_count < worker_count) {
        sleep_ms(10)
        vibez.spill("Waiting for workers... done:", done_count, "/", worker_count)
    }
    
    # Final GC to clean up any remaining objects
    vibez.spill("Final cleanup...")
    force_gc()
    sleep_ms(50) # Give finalizers time to run
    
    vibez.spill("Deadlock prevention test completed successfully!")
    vibez.spill("Finalizers triggered:", finalizer_triggered_count)
    
    # If we reach here without hanging, the deadlock fix worked
    damn based
}

slay test_finalization_queue_contention() {
    vibez.spill("Testing finalization queue lock contention handling...")
    
    sus queue_operations drip = 0
    sus successful_operations drip = 0
    sus deferred_operations drip = 0
    
    # Test rapid finalizer registration under memory pressure
    bestie (sus i drip = 0; i < 1000; i = i + 1) {
        sus obj *drip = allocate(16)
        queue_operations = queue_operations + 1
        
        ready (register_finalizer(obj, slay(ptr *anyopaque) { 
            # Simple finalizer
        })) {
            when fam -> {
                deferred_operations = deferred_operations + 1
            }
            otherwise -> {
                successful_operations = successful_operations + 1
            }
        }
        
        # Frequent GC to create lock contention
        ready (i % 5 == 0) {
            force_gc()
        }
    }
    
    vibez.spill("Queue operations:", queue_operations)
    vibez.spill("Successful registrations:", successful_operations)  
    vibez.spill("Deferred operations:", deferred_operations)
    
    # Verify system remained responsive
    ready (deferred_operations > 0) {
        vibez.spill("✓ Lock contention handled gracefully - operations deferred without deadlock")
    } otherwise {
        vibez.spill("✓ No lock contention encountered")
    }
    
    damn based
}

# Run the tests
test_start("GC Finalization Deadlock Prevention")

ready (test_finalizer_deadlock_prevention()) {
    vibez.spill("✓ Finalizer deadlock prevention test PASSED")
} otherwise {
    vibez.spill("✗ Finalizer deadlock prevention test FAILED")
}

ready (test_finalization_queue_contention()) {
    vibez.spill("✓ Queue contention handling test PASSED")
} otherwise {
    vibez.spill("✗ Queue contention handling test FAILED") 
}

print_test_summary()
