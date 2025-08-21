// Simple Concurrency Bridge Test
// Basic test to verify P1 runtime bridge functionality

yeet "vibez"

slay main() normie {
    vibez.spill("=== Simple Concurrency Bridge Test ===")
    
    // Test 1: Basic goroutine with stan
    vibez.spill("Test 1: Basic goroutine spawning")
    
    stan {
        vibez.spill("Hello from goroutine!")
        sus counter normie = 0
        bestie (sus i normie = 0; i < 5; i += 1) {
            counter += i
        }
        vibez.spill("Goroutine counter result:", counter)
    }
    
    // Test 2: Basic channel operations
    vibez.spill("Test 2: Basic channel operations")
    
    sus ch dm<normie> = dm<normie>(2)
    dm_send(ch, 42)
    dm_send(ch, 100)
    
    sus val1 normie = dm_recv(ch)
    sus val2 normie = dm_recv(ch)
    
    vibez.spill("Sent: 42, 100")  
    vibez.spill("Received:", val1, val2)
    
    // Test 3: Goroutine with channel communication
    vibez.spill("Test 3: Goroutine with channel")
    
    sus result_ch dm<normie> = dm<normie>(1)
    
    stan {
        sus computation normie = 10 * 10 + 5
        dm_send(result_ch, computation)
        vibez.spill("Goroutine sent result:", computation)
    }
    
    sus final_result normie = dm_recv(result_ch)
    vibez.spill("Main received result:", final_result)
    
    vibez.spill("=== Test completed successfully! ===")
    damn 0
}
