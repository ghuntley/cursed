yeet "testz"

# Basic preemptive scheduling test
test_start("preemptive scheduler basic test")

slay test_preemptive_goroutines() lit {
    # Test basic goroutine spawning with preemptive scheduling
    sus counter drip = 0
    
    # Spawn multiple goroutines that would normally hog CPU
    stan {
        floop i from 0 to 1000000 {
            counter = counter + 1
            # Yield occasionally to test preemption
            yo counter % 10000 == 0 {
                yolo  # Yield voluntarily
            }
        }
    }
    
    stan {
        floop i from 0 to 1000000 {
            counter = counter + 1
            yo counter % 10000 == 0 {
                yolo  # Yield voluntarily
            }
        }
    }
    
    # Wait a bit to ensure goroutines have time to run
    sus iterations drip = 0
    vibecheck iterations < 100 {
        iterations = iterations + 1
        yolo  # Give time for other goroutines
    }
    
    damn based  # Should complete without hanging
}

assert_true(test_preemptive_goroutines())

print_test_summary()
