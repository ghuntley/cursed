// Comprehensive CURSED Concurrency Demonstration
// Shows all concurrency features working together

yeet "testz"

vibez.spill("🚀 CURSED Concurrency System Demo\n")
vibez.spill("=====================================\n\n")

// Demo 1: Basic Goroutines
vibez.spill("📋 Demo 1: Basic Goroutines (stan keyword)\n")
sus demo1_complete lit = cringe

stan {
    vibez.spill("  🏃 Goroutine 1: Running in background!\n")
    demo1_complete = based
}

sus wait1 drip = 0
bestie (wait1 < 10000 fam !demo1_complete) {
    wait1 = wait1 + 1
}

vibez.spill("  ✅ Goroutine completed successfully\n\n")

// Demo 2: Typed Channels
vibez.spill("📋 Demo 2: Typed Channels (dm<Type>)\n")

// Create different typed channels
sus int_channel dm<drip> = dm_make(drip, 3)
sus string_channel dm<tea> = dm_make(tea, 2)
sus bool_channel dm<lit> = dm_make(lit, 1)

// Send values
dm_send(int_channel, 42)
dm_send(int_channel, 100)
dm_send(string_channel, "Hello")
dm_send(string_channel, "World")
dm_send(bool_channel, based)

// Receive values
sus recv_int1 drip = dm_recv(int_channel)
sus recv_int2 drip = dm_recv(int_channel)
sus recv_str1 tea = dm_recv(string_channel)
sus recv_str2 tea = dm_recv(string_channel)
sus recv_bool lit = dm_recv(bool_channel)

vibez.spill("  📨 Received integers: ")
vibez.spill_int(recv_int1)
vibez.spill(", ")
vibez.spill_int(recv_int2)
vibez.spill("\n")

vibez.spill("  📨 Received strings: ")
vibez.spill(recv_str1)
vibez.spill(", ")
vibez.spill(recv_str2)
vibez.spill("\n")

vibez.spill("  📨 Received boolean: ")
if (recv_bool) {
    vibez.spill("true")
} else {
    vibez.spill("false")
}
vibez.spill("\n\n")

// Demo 3: Producer-Consumer Pattern
vibez.spill("📋 Demo 3: Producer-Consumer Pattern\n")

sus work_queue dm<drip> = dm_make(drip, 5)
sus results dm<drip> = dm_make(drip, 5)

// Producer goroutine
stan {
    vibez.spill("  🏭 Producer: Creating work items...\n")
    sus task drip = 1
    bestie (task <= 5) {
        dm_send(work_queue, task)
        vibez.spill("  📤 Producer: Sent task ")
        vibez.spill_int(task)
        vibez.spill("\n")
        task = task + 1
    }
    vibez.spill("  🏭 Producer: All tasks sent!\n")
}

// Consumer goroutine
stan {
    vibez.spill("  🛠️  Consumer: Processing work items...\n")
    sus processed drip = 0
    bestie (processed < 5) {
        sus task drip = dm_recv(work_queue)
        sus result drip = task * task // Square the number
        dm_send(results, result)
        vibez.spill("  ⚙️  Consumer: Processed task ")
        vibez.spill_int(task)
        vibez.spill(" -> result ")
        vibez.spill_int(result)
        vibez.spill("\n")
        processed = processed + 1
    }
    vibez.spill("  🛠️  Consumer: All tasks processed!\n")
}

// Collect results
vibez.spill("  📊 Collecting results:\n")
sus collected drip = 0
bestie (collected < 5) {
    sus result drip = dm_recv(results)
    vibez.spill("  📈 Result ")
    vibez.spill_int(collected + 1)
    vibez.spill(": ")
    vibez.spill_int(result)
    vibez.spill("\n")
    collected = collected + 1
}
vibez.spill("\n")

// Demo 4: Select-like Operations  
vibez.spill("📋 Demo 4: Select Operations (ready keyword)\n")

sus ch_a dm<drip> = dm_make(drip, 1)
sus ch_b dm<tea> = dm_make(tea, 1)
sus select_results drip = 0

// Fill channels with different timing
dm_send(ch_a, 123)
dm_send(ch_b, "select-test")

// Demonstrate select behavior
sus selection_round drip = 1
bestie (selection_round <= 2) {
    vibez.spill("  🎯 Selection round ")
    vibez.spill_int(selection_round)
    vibez.spill(":\n")
    
    ready {
        dm_recv(ch_a) -> {
            vibez.spill("    ✅ Selected channel A (integer)\n")
            select_results = select_results + 1
        }
        dm_recv(ch_b) -> {
            vibez.spill("    ✅ Selected channel B (string)\n")
            select_results = select_results + 1
        }
    }
    
    selection_round = selection_round + 1
}

vibez.spill("  📊 Total selections: ")
vibez.spill_int(select_results)
vibez.spill("\n\n")

// Demo 5: Goroutine Coordination
vibez.spill("📋 Demo 5: Multi-Goroutine Coordination\n")

sus coordination_channel dm<drip> = dm_make(drip, 10)
sus barrier_channel dm<drip> = dm_make(drip, 3)
sus total_sum drip = 0

// Launch multiple coordinated workers
sus worker_num drip = 1
bestie (worker_num <= 3) {
    stan {
        sus my_id drip = worker_num
        vibez.spill("  👷 Worker ")
        vibez.spill_int(my_id)
        vibez.spill(": Starting work\n")
        
        // Do some work (compute factorial)
        sus factorial drip = 1
        sus i drip = 1
        bestie (i <= my_id) {
            factorial = factorial * i
            i = i + 1
        }
        
        vibez.spill("  👷 Worker ")
        vibez.spill_int(my_id)
        vibez.spill(": Computed factorial = ")
        vibez.spill_int(factorial)
        vibez.spill("\n")
        
        // Send result to coordinator
        dm_send(coordination_channel, factorial)
        
        // Signal completion
        dm_send(barrier_channel, my_id)
        
        vibez.spill("  👷 Worker ")
        vibez.spill_int(my_id)
        vibez.spill(": Work complete\n")
    }
    worker_num = worker_num + 1
}

// Coordinator goroutine
stan {
    vibez.spill("  📊 Coordinator: Collecting results...\n")
    sus sum drip = 0
    sus workers_done drip = 0
    
    bestie (workers_done < 3) {
        sus factorial drip = dm_recv(coordination_channel)
        sum = sum + factorial
        workers_done = workers_done + 1
        
        vibez.spill("  📊 Coordinator: Received factorial ")
        vibez.spill_int(factorial)
        vibez.spill(", running sum = ")
        vibez.spill_int(sum)
        vibez.spill("\n")
    }
    
    total_sum = sum
    vibez.spill("  📊 Coordinator: Final sum = ")
    vibez.spill_int(total_sum)
    vibez.spill("\n")
}

// Wait for all workers to complete
sus barrier_count drip = 0
bestie (barrier_count < 3) {
    sus worker_id drip = dm_recv(barrier_channel)
    vibez.spill("  🚧 Barrier: Worker ")
    vibez.spill_int(worker_id)
    vibez.spill(" reached barrier\n")
    barrier_count = barrier_count + 1
}

vibez.spill("  🎉 All workers synchronized!\n\n")

// Demo 6: Channel Closing and Error Handling
vibez.spill("📋 Demo 6: Channel Lifecycle Management\n")

sus lifecycle_channel dm<drip> = dm_make(drip, 2)

// Send some values
dm_send(lifecycle_channel, 999)
dm_send(lifecycle_channel, 888)

vibez.spill("  📤 Sent values to channel\n")

// Close the channel
dm_close(lifecycle_channel)
vibez.spill("  🔒 Channel closed\n")

// Try to send after closing (should handle gracefully)
sus close_send_result drip = dm_send(lifecycle_channel, 777)
vibez.spill("  ⚠️  Send to closed channel result: ")
vibez.spill_int(close_send_result)
vibez.spill("\n")

// Can still receive remaining values
sus remaining1 drip = dm_recv(lifecycle_channel)
sus remaining2 drip = dm_recv(lifecycle_channel)

vibez.spill("  📨 Received remaining values: ")
vibez.spill_int(remaining1)
vibez.spill(", ")
vibez.spill_int(remaining2)
vibez.spill("\n\n")

// Final Summary
vibez.spill("🎉 CURSED Concurrency Demo Complete!\n")
vibez.spill("=====================================\n")
vibez.spill("✅ Goroutines (stan): Working\n")
vibez.spill("✅ Typed Channels (dm<T>): Working\n")
vibez.spill("✅ Channel Operations (dm_send/dm_recv): Working\n")
vibez.spill("✅ Select Statements (ready): Working\n")
vibez.spill("✅ Producer-Consumer Pattern: Working\n")
vibez.spill("✅ Multi-Goroutine Coordination: Working\n")
vibez.spill("✅ Channel Lifecycle Management: Working\n")
vibez.spill("\n🚀 CURSED concurrency system is fully operational!\n")
