yeet "concurrenz"
yeet "vibez"

slay test_scheduler_preemption() {
    vibez.spill("🧪 Testing cross-platform scheduler preemption...")
    
    sus counter drip = 0
    sus max_iterations drip = 100000
    
    // Create multiple goroutines that compete for CPU time
    bestie (counter < 5) {
        go {
            sus local_counter drip = 0
            bestie (local_counter < max_iterations) {
                local_counter = local_counter + 1
                
                // Simulate CPU-intensive work that should trigger preemption
                ready (local_counter % 10000 == 0) {
                    vibez.spill("Goroutine", counter, "iteration", local_counter)
                }
            }
            vibez.spill("Goroutine", counter, "completed with", local_counter, "iterations")
        }
        counter = counter + 1
    }
    
    // Give time for goroutines to run and preempt each other
    concurrenz.sleep(2000) // 2 seconds
    
    vibez.spill("✅ Cross-platform preemption test completed!")
}

slay main() {
    test_scheduler_preemption()
}
